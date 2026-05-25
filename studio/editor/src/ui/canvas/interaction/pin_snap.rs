//! Port magnetic snap — ekranda barqaror radius, registry orqali pin mosligi.

use bevy::prelude::*;
use std::collections::HashMap;

use crate::graph::{GraphPort, GraphResource, NodeId};
use crate::registry::WireCategory;
use crate::state::ConnectingState;
use crate::ui::canvas::coords::{MAX_CANVAS_ZOOM, MIN_CANVAS_ZOOM};

/// Port atrofidagi sim tortish / ulanish radiusi (ekran pikseli).
pub const PIN_SNAP_RADIUS: f32 = 28.0;

fn snap_radius_sq(zoom: f32) -> f32 {
    let z = zoom.clamp(MIN_CANVAS_ZOOM, MAX_CANVAS_ZOOM);
    let r = PIN_SNAP_RADIUS / z;
    r * r
}

fn connection_category(connecting: &ConnectingState) -> Option<WireCategory> {
    match (connecting.from, connecting.to) {
        (Some((_, _, c)), _) | (_, Some((_, _, c))) => Some(c),
        _ => None,
    }
}

/// Port ushbu ulanish yo‘nalishiga mos keladimi (chiqish/kirish).
pub fn port_matches_connection(
    port: &GraphPort,
    connecting: &ConnectingState,
    want_output: bool,
    graph: &GraphResource,
) -> bool {
    let Some(cat) = connection_category(connecting) else {
        return false;
    };
    if port.category != cat {
        return false;
    }
    let Some(node) = graph.node(port.node) else {
        return false;
    };
    let kind = node.kind;
    match cat {
        WireCategory::Exec if want_output => kind.is_exec_output(port.pin),
        WireCategory::Exec => kind.is_exec_input(port.pin),
        WireCategory::Data if want_output => kind.is_data_output(port.pin),
        WireCategory::Data => kind.is_data_input(port.pin),
    }
}

/// Eng yaqin mos port (markaz, node, pin).
pub fn find_snap_target(
    cursor_graph: Vec2,
    connecting: &ConnectingState,
    positions: &HashMap<(NodeId, &'static str), Vec2>,
    ports: &Query<(&GraphPort, &UiGlobalTransform)>,
    graph: &GraphResource,
    zoom: f32,
    exclude_node: Option<NodeId>,
) -> Option<(NodeId, &'static str, Vec2)> {
    if !connecting.is_active() {
        return None;
    }

    let want_output = connecting.to.is_some();
    let radius_sq = snap_radius_sq(zoom);
    let mut best: Option<(NodeId, &'static str, Vec2, f32)> = None;

    for (port, _) in ports.iter() {
        if exclude_node == Some(port.node) {
            continue;
        }
        if !port_matches_connection(port, connecting, want_output, graph) {
            continue;
        }
        let center = *positions.get(&(port.node, port.pin))?;
        let d2 = center.distance_squared(cursor_graph);
        if d2 > radius_sq {
            continue;
        }
        if best.is_none() || d2 < best.unwrap().3 {
            best = Some((port.node, port.pin, center, d2));
        }
    }

    best.map(|(n, p, c, _)| (n, p, c))
}

/// Kursorni snap qiladi; `snap_target` yangilanadi.
pub fn apply_cursor_snap(
    connecting: &mut ConnectingState,
    raw_cursor: Option<Vec2>,
    positions: &HashMap<(NodeId, &'static str), Vec2>,
    ports: &Query<(&GraphPort, &UiGlobalTransform)>,
    graph: &GraphResource,
    zoom: f32,
) {
    let Some(raw) = raw_cursor else {
        connecting.cursor_canvas = connecting.anchor_canvas;
        connecting.snap_target = None;
        return;
    };

    let exclude = match connecting.from {
        Some((n, _, _)) => Some(n),
        None => connecting.to.map(|(n, _, _)| n),
    };

    if let Some((node, pin, center)) =
        find_snap_target(raw, connecting, positions, ports, graph, zoom, exclude)
    {
        connecting.cursor_canvas = Some(center);
        connecting.snap_target = Some((node, pin));
    } else {
        connecting.cursor_canvas = Some(raw);
        connecting.snap_target = None;
    }
}

/// Ulanishni tugatish nuqtasi — snap ustuvor.
pub fn wire_finish_position(
    connecting: &ConnectingState,
    release_graph: Vec2,
    positions: &HashMap<(NodeId, &'static str), Vec2>,
    ports: &Query<(&GraphPort, &UiGlobalTransform)>,
    graph: &GraphResource,
    zoom: f32,
) -> Vec2 {
    let radius_sq = snap_radius_sq(zoom);

    if let Some((node, pin)) = connecting.snap_target {
        if let Some(center) = positions.get(&(node, pin)) {
            if center.distance_squared(release_graph) <= radius_sq {
                return *center;
            }
        }
    }

    let exclude = match connecting.from {
        Some((n, _, _)) => Some(n),
        None => connecting.to.map(|(n, _, _)| n),
    };
    if let Some((_, _, center)) =
        find_snap_target(release_graph, connecting, positions, ports, graph, zoom, exclude)
    {
        return center;
    }

    release_graph
}
