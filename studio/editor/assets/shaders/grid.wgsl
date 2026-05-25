// Infinite Blueprint-style grid — shader-based, zero CPU cost per frame.
//
// Uniform layout (all vec4 for guaranteed 16-byte alignment):
//   pan_zoom_cell: (.xy = pan physical px, .z = zoom, .w = base cell physical px)
//   params:        (.x  = major_interval — minor cells per major line)
//   color_thin:    thin (minor) line RGBA
//   color_thick:   thick (major) line RGBA

#import bevy_ui::ui_vertex_output::UiVertexOutput

struct GridUniforms {
    pan_zoom_cell: vec4<f32>,
    params:        vec4<f32>,
    color_thin:    vec4<f32>,
    color_thick:   vec4<f32>,
};

@group(1) @binding(0)
var<uniform> g: GridUniforms;

// ── Anti-aliased 1-D grid line ────────────────────────────────────────────
//
// `coord`  : position in grid-cell units (integer = on a line).
// `lw`     : half line-width expressed as a multiple of fwidth(coord).
// Returns 0 (mid-cell) … 1 (on a line).
fn line_alpha(coord: f32, lw: f32) -> f32 {
    let c  = abs(fract(coord - 0.5) - 0.5);          // 0 on line, 0.5 mid-cell
    let fw = max(fwidth(coord), 1e-5);
    return 1.0 - smoothstep(lw * fw, (lw + 1.0) * fw, c);
}

// Returns the maximum of the H and V line alphas for the given cell size.
fn grid2d(gp: vec2<f32>, cell: f32, lw: f32) -> f32 {
    let uv = gp / cell;
    return max(line_alpha(uv.x, lw), line_alpha(uv.y, lw));
}

// ── Fragment entry point ──────────────────────────────────────────────────
@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let pan        = g.pan_zoom_cell.xy;
    let zoom       = g.pan_zoom_cell.z;
    let base_cell  = g.pan_zoom_cell.w;
    let major_n    = g.params.x;

    // Convert fragment UV → graph space (physical pixels).
    // Formula derived from Bevy UI's UiTransform (translate=pan, scale=zoom):
    //   screen_pos = center + pan + zoom * (graph_pos - center)
    // → graph_pos  = center + (screen_pos - center - pan) / zoom
    let screen = in.uv * in.size;
    let center = in.size * 0.5;
    let gp     = center + (screen - center - pan) / zoom;

    // ── LOD ──────────────────────────────────────────────────────────────
    // Choose two adjacent cell sizes so minor grid lines span ~40 px on screen.
    // target_px / zoom gives the ideal cell size in graph-physical units.
    // We express this as a power-of-two multiple of base_cell and use the
    // fractional part to cross-fade between adjacent LOD levels.
    let target_px = 40.0;
    let ideal     = target_px / zoom;
    let log_t    = log2(ideal / base_cell);
    let lod0     = floor(log_t);
    let blend    = fract(log_t);      // 0 → show lod0 fully, 1 → show lod1 fully

    let cell0 = base_cell * pow(2.0, lod0);   // finer level
    let cell1 = cell0 * 2.0;                   // coarser level

    let lw: f32 = 0.65;   // half-width in fwidth units → crisp 1-px lines

    // Minor grid: blend across the LOD transition to avoid pop.
    let minor = max(
        grid2d(gp, cell0, lw) * (1.0 - blend),
        grid2d(gp, cell1, lw) * blend,
    );

    // Major grid (every major_n minor cells).
    let major = max(
        grid2d(gp, cell0 * major_n, lw) * (1.0 - blend),
        grid2d(gp, cell1 * major_n, lw) * blend,
    );

    // ── Compose ──────────────────────────────────────────────────────────
    let a_thin  = minor * g.color_thin.a;
    let a_thick = major * g.color_thick.a;
    let a_final = max(a_thin, a_thick);

    // Transparent pixels — do nothing (no overdraw).
    if a_final < 0.004 {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }

    // Blend thin / thick colors based on their relative contributions.
    let t   = a_thick / (a_thin + a_thick + 1e-5);
    let rgb = mix(g.color_thin.rgb, g.color_thick.rgb, t);
    return vec4<f32>(rgb, a_final);
}
