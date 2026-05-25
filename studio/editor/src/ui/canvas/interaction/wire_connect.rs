//! Wire connect / finish helpers.

use bevy::prelude::*;
use std::collections::HashMap;

use crate::graph::{GraphPort, GraphResource, NodeId};
use crate::registry::WireCategory;
use crate::state::{ConnectingState, TerminalState};
use crate::ui::canvas::interaction::pin_snap::{
    find_snap_target, port_matches_connection, wire_finish_position,
};
use crate::ui::canvas::coords::VmNodeLayoutQuery;
use crate::ui::canvas::render::port_center_graph;

/// Port ustida qo‘yib chiqilganda — snap radiussiz to‘g‘ridan-to‘g‘ri ulanish.
pub(crate) fn try_finish_wire_at_port(
    connecting: &mut ConnectingState,
    graph: &mut GraphResource,
    terminal: &mut TerminalState,
    target: &GraphPort,
) -> bool {
    if !connecting.is_active() {
        return false;
    }

    let want_output = connecting.to.is_some();
    if !port_matches_connection(target, connecting, want_output, graph) {
        return false;
    }

    if let Some((from_node, from_pin, cat)) = connecting.from {
        if target.node == from_node {
            return false;
        }
        connecting.clear();
        let (result, to_label) = match cat {
            WireCategory::Exec => (
                graph.connect_exec(from_node, from_pin, target.node, target.pin),
                target.pin,
            ),
            WireCategory::Data => (
                graph.connect_data(from_node, from_pin, target.node, target.pin),
                target.pin,
            ),
        };
        log_wire_result(terminal, cat, result, from_node, from_pin, target.node, to_label, graph);
        return true;
    }

    if let Some((to_node, to_pin, cat)) = connecting.to {
        if target.node == to_node {
            return false;
        }
        connecting.clear();
        let result = match cat {
            WireCategory::Exec => {
                graph.connect_exec(target.node, target.pin, to_node, to_pin)
            }
            WireCategory::Data => {
                graph.connect_data(target.node, target.pin, to_node, to_pin)
            }
        };
        log_wire_result(terminal, cat, result, target.node, target.pin, to_node, to_pin, graph);
        return true;
    }

    false
}

fn try_finish_at_pin(
    connecting: &mut ConnectingState,
    graph: &mut GraphResource,
    terminal: &mut TerminalState,
    target_node: NodeId,
    target_pin: &'static str,
    category: WireCategory,
) -> bool {
    try_finish_wire_at_port(
        connecting,
        graph,
        terminal,
        &GraphPort {
            node: target_node,
            pin: target_pin,
            category,
        },
    )
}

pub(crate) fn try_finish_wire(
    connecting: &mut ConnectingState,
    graph: &mut GraphResource,
    terminal: &mut TerminalState,
    release_graph: Vec2,
    positions: &HashMap<(NodeId, &'static str), Vec2>,
    ports: &Query<(&GraphPort, &UiGlobalTransform)>,
    zoom: f32,
) {
    if !connecting.is_active() {
        return;
    }

    if let Some((node, pin)) = connecting.snap_target {
        if let Some(cat) = connecting
            .from
            .map(|(_, _, c)| c)
            .or_else(|| connecting.to.map(|(_, _, c)| c))
        {
            if try_finish_at_pin(connecting, graph, terminal, node, pin, cat) {
                return;
            }
        }
    }

    let finish = wire_finish_position(connecting, release_graph, positions, ports, graph, zoom);

    if let Some((from_node, from_pin, cat)) = connecting.from {
        let exclude = Some(from_node);
        let Some((to_node, to_pin, _)) =
            find_snap_target(finish, connecting, positions, ports, graph, zoom, exclude)
        else {
            connecting.clear();
            terminal.log("[wire] mos kirish porti topilmadi (In / data port)");
            return;
        };
        connecting.clear();
        let (result, to_label) = match cat {
            WireCategory::Exec => (
                graph.connect_exec(from_node, from_pin, to_node, to_pin),
                to_pin,
            ),
            WireCategory::Data => (
                graph.connect_data(from_node, from_pin, to_node, to_pin),
                to_pin,
            ),
        };
        log_wire_result(terminal, cat, result, from_node, from_pin, to_node, to_label, graph);
        return;
    }

    if let Some((to_node, to_pin, cat)) = connecting.to {
        let exclude = Some(to_node);
        let Some((from_node, from_pin, _)) =
            find_snap_target(finish, connecting, positions, ports, graph, zoom, exclude)
        else {
            connecting.clear();
            terminal.log("[wire] mos chiqish porti topilmadi");
            return;
        };
        connecting.clear();
        let result = match cat {
            WireCategory::Exec => graph.connect_exec(from_node, from_pin, to_node, to_pin),
            WireCategory::Data => graph.connect_data(from_node, from_pin, to_node, to_pin),
        };
        log_wire_result(terminal, cat, result, from_node, from_pin, to_node, to_pin, graph);
    }
}

fn log_wire_result(
    terminal: &mut TerminalState,
    cat: WireCategory,
    result: Result<(), String>,
    from_node: NodeId,
    from_pin: &'static str,
    to_node: NodeId,
    to_pin: &'static str,
    graph: &GraphResource,
) {
    match result {
        Ok(()) => terminal.log(format!(
            "[wire:{cat:?}] {}:{} → {}:{}",
            graph.title(from_node),
            from_pin,
            graph.title(to_node),
            to_pin
        )),
        Err(reason) => terminal.log(format!("[wire] {reason}")),
    }
}

pub(crate) fn begin_connect_from_output(
    connecting: &mut ConnectingState,
    node: NodeId,
    pin: &'static str,
    category: WireCategory,
    graph: &GraphResource,
    ports: &Query<(&GraphPort, &UiGlobalTransform)>,
    nodes: &VmNodeLayoutQuery,
) {
    connecting.clear();
    connecting.from = Some((node, pin, category));
    let anchor = port_center_graph(graph, node, pin, ports, nodes);
    connecting.anchor_canvas = anchor;
    connecting.cursor_canvas = anchor;
}

pub(crate) fn begin_connect_to_input(
    connecting: &mut ConnectingState,
    node: NodeId,
    pin: &'static str,
    category: WireCategory,
    graph: &GraphResource,
    ports: &Query<(&GraphPort, &UiGlobalTransform)>,
    nodes: &VmNodeLayoutQuery,
) {
    connecting.clear();
    connecting.to = Some((node, pin, category));
    let anchor = port_center_graph(graph, node, pin, ports, nodes);
    connecting.anchor_canvas = anchor;
    connecting.cursor_canvas = anchor;
}

pub(crate) fn is_exec_output_pin(pin: &str) -> bool {
    matches!(pin, "exec_out" | "true" | "false")
}

pub(crate) fn is_data_output_pin(pin: &str) -> bool {
    matches!(pin, "result" | "delta_time")
}
