//! Connecting wire cursor snap (PostUpdate).



use bevy::prelude::*;



use crate::graph::{GraphPort, GraphResource};

use crate::state::ConnectingState;

use crate::ui::canvas::coords::{
    build_port_canvas_positions, pointer_to_graph_from_window, GraphCanvasView, UiCameraQuery,
    ViewportQuery, VmNodeLayoutQuery,
};

use crate::ui::canvas::interaction::pin_snap::apply_cursor_snap;



pub fn update_connecting_cursor(

    mut connecting: ResMut<ConnectingState>,

    graph: Res<GraphResource>,
    view: Res<GraphCanvasView>,
    windows: Query<&Window>,

    ports: Query<(&GraphPort, &UiGlobalTransform)>,

    nodes: VmNodeLayoutQuery,

    viewport: ViewportQuery,

    camera: UiCameraQuery,

) {

    if !connecting.is_active() {

        connecting.cursor_canvas = None;

        connecting.snap_target = None;

        return;

    }

    let Ok(window) = windows.single() else {

        return;

    };

    let Ok((viewport_node, viewport_gt)) = viewport.single() else {

        return;

    };

    let Ok(cam) = camera.single() else {

        return;

    };



    let raw = pointer_to_graph_from_window(window, cam, viewport_node, viewport_gt);

    let positions = build_port_canvas_positions(&graph, &ports, &nodes);

    apply_cursor_snap(&mut connecting, raw, &positions, &ports, &graph, view.zoom);

}


