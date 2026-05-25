//! Infinite shader-based grid — UE Blueprint style.
//!
//! Architecture:
//!  • [`GridMaterial`] — custom [`UiMaterial`] with a single packed uniform.
//!  • [`GridQuad`]     — marker component on the full-canvas background quad.
//!  • [`GridPlugin`]   — registers the material + sync system.
//!  • [`sync_grid_uniforms`] — runs every PostUpdate; zero CPU mesh work.
//!  • [`snap_to_grid`] — CPU helper matching the shader's LOD formula.

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
};

use crate::ui::canvas::coords::GraphCanvasView;

// ── Constants ─────────────────────────────────────────────────────────────

/// Grid cell size in CSS logical pixels at zoom = 1, DPI = 1.
pub const BASE_CELL_CSS: f32 = 16.0;
/// How many minor cells make one major (thick) line interval.
const MAJOR_INTERVAL: f32 = 8.0;
/// Shader asset path (relative to the workspace `assets/` folder).
const SHADER_PATH: &str = "shaders/grid.wgsl";

// ── GPU uniform struct ────────────────────────────────────────────────────

/// Mirror of the WGSL `GridUniforms` struct.
/// Every field is a [`Vec4`] — guarantees 16-byte alignment without padding.
#[derive(Clone, Debug, Default, ShaderType)]
pub struct GridUniforms {
    /// `x,y` = pan (physical px) · `z` = zoom · `w` = base cell (physical px).
    pub pan_zoom_cell: Vec4,
    /// `x` = major interval count; `y,z,w` unused.
    pub params: Vec4,
    /// Thin (minor) line color — RGBA linear.
    pub color_thin: Vec4,
    /// Thick (major) line color — RGBA linear.
    pub color_thick: Vec4,
}

// ── Material ──────────────────────────────────────────────────────────────

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct GridMaterial {
    #[uniform(0)]
    pub uniforms: GridUniforms,
}

impl Default for GridMaterial {
    fn default() -> Self {
        Self {
            uniforms: GridUniforms {
                // pan=0, zoom=1, base_cell=BASE_CELL (overwritten every frame)
                pan_zoom_cell: Vec4::new(0.0, 0.0, 1.0, BASE_CELL_CSS),
                params:        Vec4::new(MAJOR_INTERVAL, 0.0, 0.0, 0.0),
                // Subtle dark-blue-grey tones matching the editor palette
                color_thin:  Vec4::new(0.155, 0.170, 0.215, 0.85),
                color_thick: Vec4::new(0.215, 0.245, 0.320, 1.00),
            },
        }
    }
}

impl UiMaterial for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
}

// ── Marker component ──────────────────────────────────────────────────────

/// Placed on the full-canvas background [`Node`] that renders the grid.
#[derive(Component)]
pub struct GridQuad;

// ── Plugin ────────────────────────────────────────────────────────────────

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiMaterialPlugin::<GridMaterial>::default())
            .add_systems(
                PostUpdate,
                sync_grid_uniforms
                    .after(bevy::ui::UiSystems::Layout)
                    .before(bevy::ui::UiSystems::Stack),
            );
    }
}

// ── Spawn helper ──────────────────────────────────────────────────────────

/// Returns the bundle for the infinite-grid background quad.
///
/// Spawn this as the **first child** of [`GraphCanvas`] so it renders
/// behind the viewport and all nodes.
pub fn grid_bundle(materials: &mut Assets<GridMaterial>) -> impl Bundle {
    (
        GridQuad,
        MaterialNode(materials.add(GridMaterial::default())),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        // No ZIndex override — spawn order ensures grid is behind the viewport.
        Pickable::IGNORE,
    )
}

// ── Update system ─────────────────────────────────────────────────────────

/// Syncs [`GraphCanvasView`] pan/zoom → [`GridMaterial`] uniforms.
///
/// Runs every frame in `PostUpdate` after layout so [`ComputedNode`]
/// (which carries the DPI scale factor) is already fresh.
///
/// **Optimisation:** returns immediately when pan/zoom hasn't changed,
/// so there is truly 0 CPU work and 0 GPU uniform upload while the
/// canvas is idle.
pub fn sync_grid_uniforms(
    view: Res<GraphCanvasView>,
    query: Query<(&MaterialNode<GridMaterial>, &ComputedNode), With<GridQuad>>,
    mut materials: ResMut<Assets<GridMaterial>>,
) {
    // Early-out: no change → nothing to upload to the GPU this frame.
    if !view.is_changed() {
        return;
    }

    for (handle, computed) in &query {
        let Some(mut mat) = materials.get_mut(handle) else {
            continue;
        };
        // Convert CSS logical pixels → physical pixels for the shader.
        let dpi = 1.0 / computed.inverse_scale_factor().max(0.001);
        mat.uniforms.pan_zoom_cell = Vec4::new(
            view.pan.x * dpi,
            view.pan.y * dpi,
            view.zoom,
            BASE_CELL_CSS * dpi,
        );
        // params.x (major_interval) is constant — set at spawn, never needs update.
    }
}

// ── Snap-to-grid helper ───────────────────────────────────────────────────

/// Snap `pos` (graph CSS coordinates) to the nearest grid vertex
/// using the **same LOD formula** as the WGSL shader.
///
/// ```
/// let snapped = snap_to_grid(dragged_pos, &view);
/// ```
#[allow(dead_code)]
pub fn snap_to_grid(pos: Vec2, view: &GraphCanvasView) -> Vec2 {
    let ideal_cell = 40.0 / view.zoom;
    let lod        = (ideal_cell / BASE_CELL_CSS).log2().floor();
    let cell       = BASE_CELL_CSS * 2.0_f32.powf(lod);
    Vec2::new(
        (pos.x / cell).round() * cell,
        (pos.y / cell).round() * cell,
    )
}
