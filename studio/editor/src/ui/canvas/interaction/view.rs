//! Graph kanvas pan / zoom — g'ildirak = zoom, Ctrl+g'ildirak = pan, o'rta tugma = pan.

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
    prelude::*,
};

use crate::ui::canvas::coords::{
    pointer_to_graph_from_window, GraphCanvasView, UiCameraQuery, ViewportQuery,
};
use crate::ui::components::{GraphCanvas, GraphCanvasViewport};

const WHEEL_LINE: f32 = 18.0;
/// Zoom change per one scroll line (12 % per tick).
const ZOOM_STEP: f32 = 0.12;

#[derive(Resource, Default, Debug)]
pub struct CanvasPanDrag {
    pub active: bool,
}

pub fn sync_graph_viewport_transform(
    view: Res<GraphCanvasView>,
    mut transforms: Query<&mut UiTransform, With<GraphCanvasViewport>>,
) {
    let Ok(mut ui) = transforms.single_mut() else {
        return;
    };
    *ui = UiTransform {
        translation: Val2::new(Val::Px(view.pan.x), Val::Px(view.pan.y)),
        scale: Vec2::splat(view.zoom),
        ..default()
    };
}

fn entity_under_canvas(entity: Entity, canvas: Entity, child_of: &Query<&ChildOf>) -> bool {
    let mut current = entity;
    loop {
        if current == canvas {
            return true;
        }
        let Ok(parent) = child_of.get(current) else {
            return false;
        };
        current = parent.parent();
    }
}

fn hover_over_canvas(hover_map: &HoverMap, canvas: Entity, child_of: &Query<&ChildOf>) -> bool {
    for pointer_map in hover_map.values() {
        for &entity in pointer_map.keys() {
            if entity_under_canvas(entity, canvas, child_of) {
                return true;
            }
        }
    }
    false
}

pub fn canvas_wheel_pan_zoom(
    mut wheel: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    child_of: Query<&ChildOf>,
    canvas_root: Query<Entity, With<GraphCanvas>>,
    windows: Query<&Window>,
    viewport: ViewportQuery,
    camera: UiCameraQuery,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut view: ResMut<GraphCanvasView>,
) {
    let Ok(canvas_entity) = canvas_root.single() else {
        return;
    };
    if !hover_over_canvas(&hover_map, canvas_entity, &child_of) {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((viewport_node, viewport_gt)) = viewport.single() else {
        return;
    };
    let Ok(camera) = camera.single() else {
        return;
    };

    let ctrl = keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    let cursor_graph =
        pointer_to_graph_from_window(window, camera, viewport_node, viewport_gt)
            .unwrap_or(Vec2::ZERO);

    for event in wheel.read() {
        // delta used for panning (negated so scroll-up moves content up)
        let mut delta = -Vec2::new(event.x, event.y);
        if event.unit == MouseScrollUnit::Line {
            delta *= WHEEL_LINE;
        }

        if ctrl {
            // Ctrl + scroll → pan
            view.pan += delta;
        } else {
            // Plain scroll → zoom in / out around cursor
            // Normalise to "lines" so pixel-based devices behave the same.
            let lines = match event.unit {
                MouseScrollUnit::Line  => event.y,
                MouseScrollUnit::Pixel => event.y / WHEEL_LINE,
            };
            let factor = 1.0 + lines * ZOOM_STEP;
            view.zoom_about_graph_point(cursor_graph, factor);
        }
    }
}

pub fn on_canvas_pan_drag_start(
    start: On<Pointer<DragStart>>,
    mut pan: ResMut<CanvasPanDrag>,
    canvas_root: Query<Entity, With<GraphCanvas>>,
    child_of: Query<&ChildOf>,
) {
    if start.button != PointerButton::Middle {
        return;
    }
    let Ok(canvas) = canvas_root.single() else {
        return;
    };
    if !entity_under_canvas(start.event_target(), canvas, &child_of) {
        return;
    }
    pan.active = true;
}

pub fn on_canvas_pan_drag(
    drag: On<Pointer<Drag>>,
    pan: Res<CanvasPanDrag>,
    mut view: ResMut<GraphCanvasView>,
) {
    if !pan.active {
        return;
    }
    view.pan += drag.delta;
}

pub fn on_canvas_pan_drag_end(_end: On<Pointer<DragEnd>>, mut pan: ResMut<CanvasPanDrag>) {
    pan.active = false;
}
