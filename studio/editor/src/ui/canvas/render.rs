//! Exec/data simlar — UE uslubida gorizontal/vertikal, viewport graph fozasida.

use bevy::prelude::*;
use std::collections::HashMap;

use crate::graph::{GraphPort, GraphResource, NodeId};
use crate::state::ConnectingState;
use crate::ui::canvas::coords::{build_port_canvas_positions, VmNodeLayoutQuery};
use crate::ui::components::{GraphCanvasViewport, GraphWire};
use crate::ui::nodes::theme::{WIRE_COLOR, WIRE_DATA_COLOR, WIRE_PREVIEW};

const WIRE_THICKNESS: f32 = 2.5;
const MIN_TANGENT: f32 = 48.0;
const MAX_TANGENT: f32 = 220.0;
/// Orqaga yo'nalgan simlar uchun bypass kengligi (node chetidan chiqib ketish).
const BACK_BYPASS: f32 = 56.0;

#[derive(Resource, Default)]
pub struct WireSegmentPool {
    pub segments: Vec<Entity>,
}

#[derive(Clone, Copy)]
struct WireBar {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
}

fn port_at(
    positions: &HashMap<(NodeId, &'static str), Vec2>,
    node: NodeId,
    pin: &str,
) -> Option<Vec2> {
    positions.get(&(node, pin)).copied()
}

fn is_output_pin(pin: &str) -> bool {
    matches!(pin, "exec_out" | "true" | "false" | "result" | "delta_time")
}

fn wire_tangent(start: Vec2, end: Vec2) -> f32 {
    (start.distance(end) * 0.45 + MIN_TANGENT).clamp(MIN_TANGENT, MAX_TANGENT)
}

fn push_h(bars: &mut Vec<WireBar>, x0: f32, x1: f32, y: f32, color: Color) {
    let w = (x1 - x0).abs();
    if w < 0.5 {
        return;
    }
    let half = WIRE_THICKNESS * 0.5;
    bars.push(WireBar {
        x: x0.min(x1),
        y: y - half,
        width: w,
        height: WIRE_THICKNESS,
        color,
    });
}

fn push_v(bars: &mut Vec<WireBar>, x: f32, y0: f32, y1: f32, color: Color) {
    let h = (y1 - y0).abs();
    if h < 0.5 {
        return;
    }
    let half = WIRE_THICKNESS * 0.5;
    bars.push(WireBar {
        x: x - half,
        y: y0.min(y1),
        width: WIRE_THICKNESS,
        height: h,
        color,
    });
}

/// Chiqish → kirish routing: ikki holat —
///  • Oldinga (end.x > start.x): tangent dx/2 ga qirqiladi, hech qachon orqaga ketmaydi.
///  • Orqaga  (end.x ≤ start.x): bypass yo'nalishi: o'ng → pastga/tepaga → chap → end.
fn add_flow_wire(
    bars: &mut Vec<WireBar>,
    start: Vec2,
    end: Vec2,
    from_pin: &str,
    to_pin: &str,
    color: Color,
) {
    if start.distance_squared(end) < 1.0 {
        return;
    }

    // leave / arrive yo'nalishlari pin tomoniga qarab
    let sign_leave: f32 = if is_output_pin(from_pin) { 1.0 } else { -1.0 };
    let sign_arrive: f32 = if is_output_pin(to_pin) { 1.0 } else { -1.0 };

    let t_raw = wire_tangent(start, end);
    let dx = (end.x - start.x) * sign_leave; // musbat = oldinga, manfiy = orqaga

    if dx > 0.0 {
        // ── Oldinga: tangent dx/2 ga cheklangan — orqa segment hech qachon yo'q ──
        let t = t_raw.min(dx * 0.5);
        let leave_x = start.x + sign_leave * t;
        let arrive_x = end.x + sign_arrive * t;

        push_h(bars, start.x, leave_x, start.y, color);
        push_v(bars, leave_x, start.y, end.y, color);
        if (arrive_x - leave_x).abs() >= 1.0 {
            push_h(bars, leave_x, arrive_x, end.y, color);
        }
        push_h(bars, arrive_x, end.x, end.y, color);
    } else {
        // ── Orqaga / yaqin: bypass — U shakli ──
        // Chiqish tomoniga BACK_BYPASS qo'shib, o'rta Y dan o'tib, kirish tomoniga keladi.
        let bypass_r = start.x + sign_leave * BACK_BYPASS;
        let bypass_l = end.x + sign_arrive * BACK_BYPASS;
        let mid_y = (start.y + end.y) * 0.5;

        push_h(bars, start.x, bypass_r, start.y, color);
        push_v(bars, bypass_r, start.y, mid_y, color);
        push_h(bars, bypass_r, bypass_l, mid_y, color);
        push_v(bars, bypass_l, mid_y, end.y, color);
        push_h(bars, bypass_l, end.x, end.y, color);
    }
}

/// Drag preview chiqishdan — adaptiv: oldinda Z-shakl, orqada bypass.
fn add_drag_preview_from_output(bars: &mut Vec<WireBar>, start: Vec2, end: Vec2, color: Color) {
    if start.distance_squared(end) < 1.0 {
        return;
    }
    let t_raw = wire_tangent(start, end);
    let dx = end.x - start.x;

    if dx >= 0.0 {
        let t = t_raw.min(dx * 0.5 + 1.0);
        let leave_x = start.x + t;
        push_h(bars, start.x, leave_x, start.y, color);
        push_v(bars, leave_x, start.y, end.y, color);
        if (end.x - leave_x).abs() >= 1.0 {
            push_h(bars, leave_x, end.x, end.y, color);
        }
    } else {
        let bypass_r = start.x + BACK_BYPASS;
        let mid_y = (start.y + end.y) * 0.5;
        push_h(bars, start.x, bypass_r, start.y, color);
        push_v(bars, bypass_r, start.y, mid_y, color);
        push_h(bars, bypass_r, end.x, mid_y, color);
        push_v(bars, end.x, mid_y, end.y, color);
    }
}

/// Drag preview kirishdan — adaptiv: oldinda Z-shakl, orqada bypass.
fn add_drag_preview_to_input(bars: &mut Vec<WireBar>, start: Vec2, end: Vec2, color: Color) {
    if start.distance_squared(end) < 1.0 {
        return;
    }
    let t_raw = wire_tangent(start, end);
    let dx = end.x - start.x;

    if dx <= 0.0 {
        let t = t_raw.min((-dx) * 0.5 + 1.0);
        let arrive_x = end.x - t;
        push_h(bars, start.x, arrive_x, start.y, color);
        push_v(bars, arrive_x, start.y, end.y, color);
        if (arrive_x - end.x).abs() >= 1.0 {
            push_h(bars, arrive_x, end.x, end.y, color);
        }
    } else {
        let bypass_l = end.x - BACK_BYPASS;
        let mid_y = (start.y + end.y) * 0.5;
        push_h(bars, start.x, bypass_l, start.y, color);
        push_v(bars, bypass_l, start.y, mid_y, color);
        push_h(bars, bypass_l, end.x, mid_y, color);
        push_v(bars, end.x, mid_y, end.y, color);
    }
}

fn collect_wire_bars(
    graph: &GraphResource,
    connecting: &ConnectingState,
    positions: &HashMap<(NodeId, &'static str), Vec2>,
) -> Vec<WireBar> {
    let mut bars = Vec::new();

    for w in &graph.exec_wires {
        if connecting
            .from
            .is_some_and(|(n, p, _)| n == w.from_node && p == w.from_pin)
            || connecting
                .to
                .is_some_and(|(n, p, _)| n == w.to_node && p == w.to_pin)
        {
            continue;
        }
        if let (Some(start), Some(end)) = (
            port_at(positions, w.from_node, w.from_pin),
            port_at(positions, w.to_node, w.to_pin),
        ) {
            add_flow_wire(&mut bars, start, end, w.from_pin, w.to_pin, WIRE_COLOR);
        }
    }
    for w in &graph.data_wires {
        if connecting
            .from
            .is_some_and(|(n, p, _)| n == w.from_node && p == w.from_pin)
            || connecting
                .to
                .is_some_and(|(n, p, _)| n == w.to_node && p == w.to_pin)
        {
            continue;
        }
        if let (Some(start), Some(end)) = (
            port_at(positions, w.from_node, w.from_pin),
            port_at(positions, w.to_node, w.to_pin),
        ) {
            add_flow_wire(&mut bars, start, end, w.from_pin, w.to_pin, WIRE_DATA_COLOR);
        }
    }

    if let Some((from, from_pin, _)) = connecting.from {
        if let (Some(start), Some(end)) = (
            port_at(positions, from, from_pin),
            connecting.cursor_canvas,
        ) {
            add_drag_preview_from_output(&mut bars, start, end, WIRE_PREVIEW);
        }
    }
    if let Some((to, to_pin, _)) = connecting.to {
        if let (Some(end), Some(start)) = (
            port_at(positions, to, to_pin),
            connecting.cursor_canvas,
        ) {
            add_drag_preview_to_input(&mut bars, start, end, WIRE_PREVIEW);
        }
    }

    bars
}

pub fn sync_graph_wires(
    mut commands: Commands,
    graph: Res<GraphResource>,
    connecting: Res<ConnectingState>,
    mut pool: ResMut<WireSegmentPool>,
    ports: Query<(&GraphPort, &UiGlobalTransform)>,
    nodes: VmNodeLayoutQuery,
    viewport_entity: Query<Entity, With<GraphCanvasViewport>>,
) {
    let Ok(viewport_entity) = viewport_entity.single() else {
        return;
    };
    let positions = build_port_canvas_positions(&graph, &ports, &nodes);
    let bars = collect_wire_bars(&graph, &connecting, &positions);
    let needed = bars.len();

    while pool.segments.len() < needed {
        let e = commands
            .spawn((
                GraphWire,
                Node {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                GlobalZIndex(50),
                BackgroundColor(WIRE_COLOR),
                Pickable::IGNORE,
            ))
            .id();
        commands.entity(viewport_entity).add_child(e);
        pool.segments.push(e);
    }

    for (i, bar) in bars.iter().enumerate() {
        let entity = pool.segments[i];
        commands.entity(entity).insert((
            Node {
                position_type: PositionType::Absolute,
                left: px(bar.x),
                top: px(bar.y),
                width: px(bar.width.max(1.0)),
                height: px(bar.height.max(1.0)),
                ..default()
            },
            BackgroundColor(bar.color),
            Visibility::Visible,
        ));
    }

    for &entity in pool.segments.iter().skip(needed) {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}

pub fn port_center_graph(
    graph: &GraphResource,
    node: NodeId,
    pin: &str,
    ports: &Query<(&GraphPort, &UiGlobalTransform)>,
    nodes: &VmNodeLayoutQuery,
) -> Option<Vec2> {
    build_port_canvas_positions(graph, ports, nodes)
        .get(&(node, pin))
        .copied()
}
