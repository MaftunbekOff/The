//! Port pointer observers.

use bevy::prelude::*;

use crate::graph::{GraphPort, GraphResource};
use crate::registry::WireCategory;
use crate::state::{ActiveNodeDrag, ConnectingState, TerminalState};
use crate::ui::canvas::coords::{
    build_port_canvas_positions, pointer_to_graph_from_location, pointer_to_graph_from_window,
    UiCameraQuery, ViewportQuery, VmNodeLayoutQuery,
};
use crate::ui::canvas::interaction::pin_snap::apply_cursor_snap;
use crate::ui::canvas::interaction::wire_connect::{
    begin_connect_from_output, begin_connect_to_input, is_data_output_pin, is_exec_output_pin,
    try_finish_wire, try_finish_wire_at_port,
};

pub fn on_port_stop_pointer(mut event: On<Pointer<Press>>, ports: Query<(), With<GraphPort>>) {
    if ports.get(event.event_target()).is_ok() {
        event.propagate(false);
    }
}

pub fn on_port_stop_drag_start(
    mut event: On<Pointer<DragStart>>,
    ports: Query<(), With<GraphPort>>,
) {
    if ports.get(event.event_target()).is_ok() {
        event.propagate(false);
    }
}

pub fn on_port_drag_while_connecting(
    mut event: On<Pointer<Drag>>,
    mut connecting: ResMut<ConnectingState>,
    graph: Res<GraphResource>,
    view: Res<crate::ui::canvas::coords::GraphCanvasView>,
    ports: Query<(&GraphPort, &UiGlobalTransform)>,
    nodes: VmNodeLayoutQuery,
    viewport: ViewportQuery,
    camera: UiCameraQuery,
    port_marker: Query<(), With<GraphPort>>,
) {
    if !connecting.is_active() {
        return;
    }
    if port_marker.get(event.event_target()).is_ok() {
        event.propagate(false);
    }

    let Ok((viewport_node, viewport_gt)) = viewport.single() else {
        return;
    };
    let Ok(cam) = camera.single() else {
        return;
    };

    let raw = pointer_to_graph_from_location(
        &event.pointer_location,
        cam,
        viewport_node,
        viewport_gt,
    );

    let positions = build_port_canvas_positions(&graph, &ports, &nodes);
    apply_cursor_snap(&mut connecting, raw, &positions, &ports, &graph, view.zoom);
}

pub fn on_port_stop_drag(mut event: On<Pointer<Drag>>, ports: Query<(), With<GraphPort>>) {
    if ports.get(event.event_target()).is_ok() {
        event.propagate(false);
    }
}

pub fn on_port_stop_drag_end(mut event: On<Pointer<DragEnd>>, ports: Query<(), With<GraphPort>>) {
    if ports.get(event.event_target()).is_ok() {
        event.propagate(false);
    }
}

pub fn on_port_stop_release(mut event: On<Pointer<Release>>, ports: Query<(), With<GraphPort>>) {
    if ports.get(event.event_target()).is_ok() {
        event.propagate(false);
    }
}

pub fn on_port_press(
    mut press: On<Pointer<Press>>,
    mut graph: ResMut<GraphResource>,
    mut connecting: ResMut<ConnectingState>,
    mut terminal: ResMut<TerminalState>,
    mut drag: ResMut<ActiveNodeDrag>,
    ports: Query<(&GraphPort, &UiGlobalTransform)>,
    nodes: VmNodeLayoutQuery,
) {
    let Ok((port, _)) = ports.get(press.event_target()) else {
        return;
    };
    press.propagate(false);
    drag.node = None;

    match press.button {
        PointerButton::Primary => {
            if connecting.is_active() {
                if try_finish_wire_at_port(&mut connecting, &mut graph, &mut terminal, port) {
                    return;
                }
            }

            let is_out = match port.category {
                WireCategory::Exec => is_exec_output_pin(port.pin),
                WireCategory::Data => is_data_output_pin(port.pin),
            };
            if is_out {
                begin_connect_from_output(
                    &mut connecting,
                    port.node,
                    port.pin,
                    port.category,
                    &graph,
                    &ports,
                    &nodes,
                );
            } else {
                begin_connect_to_input(
                    &mut connecting,
                    port.node,
                    port.pin,
                    port.category,
                    &graph,
                    &ports,
                    &nodes,
                );
            }
        }
        PointerButton::Secondary => {
            let n = graph.disconnect_all_from_pin(port.node, port.pin);
            if n > 0 {
                terminal.log(format!(
                    "[wire] {}:{} — {n} sim uzildi",
                    graph.title(port.node), port.pin
                ));
            } else {
                terminal.log(format!(
                    "[wire] {}:{} — ulangan sim yo'q",
                    graph.title(port.node), port.pin
                ));
            }
        }
        _ => {}
    }
}

pub fn on_port_release(
    release: On<Pointer<Release>>,
    mut graph: ResMut<GraphResource>,
    mut connecting: ResMut<ConnectingState>,
    mut terminal: ResMut<TerminalState>,
    view: Res<crate::ui::canvas::coords::GraphCanvasView>,
    windows: Query<&Window>,
    ports: Query<(&GraphPort, &UiGlobalTransform)>,
    nodes: VmNodeLayoutQuery,
    viewport: ViewportQuery,
    camera: UiCameraQuery,
    port_marker: Query<(), With<GraphPort>>,
) {
    if release.button != PointerButton::Primary || !connecting.is_active() {
        return;
    }
    if port_marker.get(release.event_target()).is_err() {
        return;
    }

    let Ok((port, _)) = ports.get(release.event_target()) else {
        return;
    };

    if try_finish_wire_at_port(&mut connecting, &mut graph, &mut terminal, port) {
        return;
    }

    let Ok(window) = windows.single() else {
        connecting.clear();
        return;
    };
    let Ok((viewport_node, viewport_gt)) = viewport.single() else {
        connecting.clear();
        return;
    };
    let Ok(cam) = camera.single() else {
        connecting.clear();
        return;
    };

    let Some(release_graph) = pointer_to_graph_from_location(
        &release.pointer_location,
        cam,
        viewport_node,
        viewport_gt,
    )
    .or_else(|| pointer_to_graph_from_window(window, cam, viewport_node, viewport_gt))
    else {
        connecting.clear();
        return;
    };

    let positions = build_port_canvas_positions(&graph, &ports, &nodes);
    try_finish_wire(
        &mut connecting,
        &mut graph,
        &mut terminal,
        release_graph,
        &positions,
        &ports,
        view.zoom,
    );
}

