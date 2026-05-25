//! Canvas background press/release.

use bevy::prelude::*;

use crate::graph::{GraphPort, GraphResource};
use crate::state::{ConnectingState, SelectedNode, TerminalState};

use crate::ui::canvas::coords::{
    build_port_canvas_positions, pointer_to_graph_from_window, GraphCanvasView, UiCameraQuery,
    ViewportQuery, VmNodeLayoutQuery,
};
use crate::ui::canvas::interaction::wire_connect::try_finish_wire;
use crate::ui::components::{GraphCanvas, GraphCanvasViewport};

pub fn on_canvas_press(
    press: On<Pointer<Press>>,
    mut selected: ResMut<SelectedNode>,
    mut connecting: ResMut<ConnectingState>,
    canvas: Query<Entity, With<GraphCanvas>>,
    viewport: Query<Entity, With<GraphCanvasViewport>>,
) {
    if press.button != PointerButton::Primary {
        return;
    }
    let Ok(canvas_e) = canvas.single() else { return; };
    let Ok(vp_e) = viewport.single() else { return; };

    // Bo'shliqqa bosganda original target — kanvas yoki viewport (tugunlar bloklamagan joy).
    let original = press.original_event_target();
    if original != canvas_e && original != vp_e {
        return;
    }
    selected.clear();
    connecting.clear();
}

pub fn on_canvas_release(
    release: On<Pointer<Release>>,
    mut graph: ResMut<GraphResource>,
    mut connecting: ResMut<ConnectingState>,
    mut terminal: ResMut<TerminalState>,
    view: Res<GraphCanvasView>,
    windows: Query<&Window>,
    ports: Query<(&GraphPort, &UiGlobalTransform)>,
    nodes: VmNodeLayoutQuery,
    viewport: ViewportQuery,
    camera: UiCameraQuery,
) {
    if release.button != PointerButton::Primary || !connecting.is_active() {
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

    let Some(release_graph) =
        pointer_to_graph_from_window(window, cam, viewport_node, viewport_gt)
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

