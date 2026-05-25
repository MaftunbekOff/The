//! Rubber-band ko'p-tanlov va Delete tugma bilan tugun o'chirish.

use bevy::prelude::*;

use crate::graph::{GraphResource, NodeId, VmNode};
use crate::state::{ConnectingState, SelectionBoxState, SelectedNode, TerminalState};
use crate::ui::canvas::coords::{
    pointer_to_graph_from_location, UiCameraQuery, ViewportQuery,
};
use crate::ui::components::{GraphCanvas, GraphCanvasViewport, SelectionRect};

// ---------------------------------------------------------------------------
// Rubber-band observers (GraphCanvas entityga biriktiriladi)
// ---------------------------------------------------------------------------

/// Chap tugma bilan kanvas foni sudraganida rubber-band boshlash.
pub fn on_canvas_sel_drag_start(
    start: On<Pointer<DragStart>>,
    canvas: Query<Entity, With<GraphCanvas>>,
    vp_q: Query<Entity, With<GraphCanvasViewport>>,
    connecting: Res<ConnectingState>,
    mut sel_box: ResMut<SelectionBoxState>,
    viewport: ViewportQuery,
    camera: UiCameraQuery,
) {
    if start.button != PointerButton::Primary {
        return;
    }
    let Ok(canvas_e) = canvas.single() else { return; };
    let Ok(vp_e) = vp_q.single() else { return; };

    // Bo'shliqqa bosganda original target kanvas yoki viewport bo'ladi.
    // Tugun ustida bosishda node entity bo'ladi — u holda rubber-band boshlanmaydi.
    let original = start.original_event_target();
    if original != canvas_e && original != vp_e {
        return;
    }
    // Sim tortish davomida rubber-band ishlamaydi
    if connecting.is_active() {
        return;
    }
    let Ok((vp_node, vp_gt)) = viewport.single() else { return; };
    let Ok(cam) = camera.single() else { return; };

    let Some(graph_pos) =
        pointer_to_graph_from_location(&start.pointer_location, cam, vp_node, vp_gt)
    else {
        return;
    };

    sel_box.active = true;
    sel_box.start = graph_pos;
    sel_box.current = graph_pos;
}

/// Rubber-band joriy o'lchamini yangilash.
pub fn on_canvas_sel_drag(
    drag: On<Pointer<Drag>>,
    mut sel_box: ResMut<SelectionBoxState>,
    viewport: ViewportQuery,
    camera: UiCameraQuery,
) {
    // sel_box.active faqat to'g'ri drag_start dan keyin true bo'ladi
    if drag.button != PointerButton::Primary || !sel_box.active {
        return;
    }
    let Ok((vp_node, vp_gt)) = viewport.single() else { return; };
    let Ok(cam) = camera.single() else { return; };

    if let Some(pos) =
        pointer_to_graph_from_location(&drag.pointer_location, cam, vp_node, vp_gt)
    {
        sel_box.current = pos;
    }
}

/// Rubber-band tugaganda to'rtburchak ichidagi tugunlarni tanlash.
pub fn on_canvas_sel_drag_end(
    end: On<Pointer<DragEnd>>,
    mut sel_box: ResMut<SelectionBoxState>,
    graph: Res<GraphResource>,
    mut selected: ResMut<SelectedNode>,
) {
    if end.button != PointerButton::Primary || !sel_box.active {
        return;
    }
    sel_box.active = false;

    let min = sel_box.rect_min();
    let max = sel_box.rect_max();

    // Juda kichik drag — oddiy klik, tanlovni o'zgartirmaymiz
    let size = max - min;
    if size.x * size.y < 64.0 {
        return;
    }

    // To'rtburchak bilan kesishuvchi tugunlarni tanlash
    selected.clear();
    for node in &graph.nodes {
        let nmin = node.position;
        let nmax = node.position
            + Vec2::new(node.kind.visual_width(), node.kind.visual_height());
        if nmin.x < max.x && nmax.x > min.x && nmin.y < max.y && nmax.y > min.y {
            selected.add_to_selection(node.id);
        }
    }
}

// ---------------------------------------------------------------------------
// System: Delete / X tugma bilan tanlangan tugunlarni o'chirish
// ---------------------------------------------------------------------------

pub fn handle_delete_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedNode>,
    mut graph: ResMut<GraphResource>,
    mut terminal: ResMut<TerminalState>,
    mut commands: Commands,
    nodes: Query<(Entity, &VmNode)>,
) {
    let pressed =
        keyboard.just_pressed(KeyCode::Delete) || keyboard.just_pressed(KeyCode::KeyX);
    if !pressed || selected.is_empty() {
        return;
    }

    let ids: Vec<NodeId> = selected.iter().collect();
    let count = ids.len();

    for &id in &ids {
        graph.remove_node(id);
    }

    for (entity, vm) in &nodes {
        if ids.contains(&vm.id) {
            commands.entity(entity).despawn();
        }
    }

    selected.clear();
    terminal.log(format!("[node] {count} ta tugun o'chirildi"));
}

// ---------------------------------------------------------------------------
// System: Rubber-band to'rtburchak UI ni sinxronlashtirish
// ---------------------------------------------------------------------------

pub fn sync_selection_rect(
    sel_box: Res<SelectionBoxState>,
    mut rects: Query<(&mut Node, &mut Visibility), With<SelectionRect>>,
) {
    if !sel_box.is_changed() {
        return;
    }
    let Ok((mut node, mut vis)) = rects.single_mut() else {
        return;
    };
    if !sel_box.active {
        *vis = Visibility::Hidden;
        return;
    }
    *vis = Visibility::Visible;
    let min = sel_box.rect_min();
    let max = sel_box.rect_max();
    node.left = Val::Px(min.x);
    node.top = Val::Px(min.y);
    node.width = Val::Px((max.x - min.x).max(0.0));
    node.height = Val::Px((max.y - min.y).max(0.0));
}
