//! Kanvas fazo — viewport ichidagi graph koordinatalar (tugun `left`/`top` bilan bir xil).

use bevy::prelude::*;
use bevy::picking::pointer::Location;
use std::collections::HashMap;

use crate::graph::{GraphPort, GraphResource, NodeId, VmNode};
use crate::ui::components::GraphCanvasViewport;

pub const MIN_CANVAS_ZOOM: f32 = 0.25;
pub const MAX_CANVAS_ZOOM: f32 = 2.5;

#[derive(Resource, Debug, Clone, Copy)]
pub struct GraphCanvasView {
    pub pan: Vec2,
    pub zoom: f32,
}

impl Default for GraphCanvasView {
    fn default() -> Self {
        Self {
            pan: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl GraphCanvasView {
    pub fn clamp_zoom(zoom: f32) -> f32 {
        zoom.clamp(MIN_CANVAS_ZOOM, MAX_CANVAS_ZOOM)
    }

    pub fn zoom_about_graph_point(&mut self, graph_point: Vec2, factor: f32) {
        let new_zoom = Self::clamp_zoom(self.zoom * factor);
        self.pan += graph_point * (self.zoom - new_zoom);
        self.zoom = new_zoom;
    }
}

/// Kamera viewport ichidagi UI global (physical px).
pub(crate) fn ui_global_from_physical(physical: Vec2, camera: &Camera) -> Vec2 {
    let mut pos = physical;
    if let Some(viewport) = camera.physical_viewport_rect() {
        pos -= viewport.min.as_vec2();
    }
    pos
}

/// Bevy UI picking bilan bir xil: `Location` → UI global (physical).
pub(crate) fn ui_global_from_pointer(location: &Location, camera: &Camera) -> Vec2 {
    let mut pos = location.position * camera.target_scaling_factor().unwrap_or(1.);
    if let Some(viewport) = camera.physical_viewport_rect() {
        pos -= viewport.min.as_vec2();
    }
    pos
}

/// UI global (physical) → viewport graph (mantiqiy px, yuqori-chap asos).
pub(crate) fn global_ui_to_graph(
    ui_global: Vec2,
    viewport: &ComputedNode,
    viewport_gt: &UiGlobalTransform,
) -> Option<Vec2> {
    let inv = viewport_gt.try_inverse()?;
    let local_physical = inv.transform_point2(ui_global);
    let scale = viewport.inverse_scale_factor();
    Some((local_physical + viewport.size() * 0.5) * scale)
}

/// Port markazi — tugun `position` + layout offset (viewport zoom bilan mos).
pub(crate) fn port_center_from_node_layout(
    port_gt: &UiGlobalTransform,
    node_gt: &UiGlobalTransform,
    node_computed: &ComputedNode,
    node_top_left: Vec2,
) -> Option<Vec2> {
    let inv = node_gt.try_inverse()?;
    let port_global = port_gt.affine().transform_point2(Vec2::ZERO);
    let local_physical = inv.transform_point2(port_global);
    Some(node_top_left + (local_physical + node_computed.size() * 0.5) * node_computed.inverse_scale_factor())
}

pub(crate) fn build_port_canvas_positions(
    graph: &GraphResource,
    ports: &Query<(&GraphPort, &UiGlobalTransform)>,
    nodes: &Query<(&VmNode, &UiGlobalTransform, &ComputedNode)>,
) -> HashMap<(NodeId, &'static str), Vec2> {
    let mut node_layout: HashMap<NodeId, (&UiGlobalTransform, &ComputedNode, Vec2)> =
        HashMap::new();
    for (vm, gt, computed) in nodes.iter() {
        if let Some(data) = graph.node(vm.id) {
            node_layout.insert(vm.id, (gt, computed, data.position));
        }
    }

    let mut map = HashMap::new();
    for (port, port_gt) in ports.iter() {
        let Some((node_gt, node_computed, node_pos)) = node_layout.get(&port.node) else {
            continue;
        };
        if let Some(center) =
            port_center_from_node_layout(port_gt, node_gt, node_computed, *node_pos)
        {
            map.insert((port.node, port.pin), center);
        }
    }
    map
}

pub(crate) fn pointer_to_graph(
    ui_global: Vec2,
    viewport: &ComputedNode,
    viewport_gt: &UiGlobalTransform,
) -> Option<Vec2> {
    global_ui_to_graph(ui_global, viewport, viewport_gt)
}

pub(crate) fn pointer_to_graph_from_window(
    window: &Window,
    camera: &Camera,
    viewport: &ComputedNode,
    viewport_gt: &UiGlobalTransform,
) -> Option<Vec2> {
    let physical = window.physical_cursor_position()?;
    pointer_to_graph(
        ui_global_from_physical(physical, camera),
        viewport,
        viewport_gt,
    )
}

pub(crate) fn pointer_to_graph_from_location(
    location: &Location,
    camera: &Camera,
    viewport: &ComputedNode,
    viewport_gt: &UiGlobalTransform,
) -> Option<Vec2> {
    pointer_to_graph(
        ui_global_from_pointer(location, camera),
        viewport,
        viewport_gt,
    )
}

pub(crate) type ViewportQuery<'w, 's> = Query<
    'w,
    's,
    (&'static ComputedNode, &'static UiGlobalTransform),
    With<GraphCanvasViewport>,
>;

pub(crate) type UiCameraQuery<'w, 's> = Query<'w, 's, &'static Camera, With<IsDefaultUiCamera>>;

pub(crate) type VmNodeLayoutQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static VmNode,
        &'static UiGlobalTransform,
        &'static ComputedNode,
    ),
>;
