//! Tugun drag (ko'p-tanlov + Alt-nusxalash) va bosish kuzatuvchilari.

use bevy::prelude::*;

use crate::graph::{GraphPort, GraphResource, NodeId, VmNode, VmNodeTitle};
use crate::state::{ActiveNodeDrag, SelectedNode, TerminalState};
use crate::ui::canvas::coords::GraphCanvasView;
use crate::ui::components::GraphCanvasViewport;
use crate::ui::nodes::spawn_vm_node;

pub(crate) fn vm_node_entity(
    target: Entity,
    nodes: &Query<&VmNode>,
    child_of: &Query<&ChildOf>,
    titles: &Query<(), With<VmNodeTitle>>,
) -> Option<Entity> {
    if nodes.get(target).is_ok() {
        return Some(target);
    }
    if titles.get(target).is_ok() {
        return child_of.get(target).ok().map(|c| c.parent());
    }
    None
}

pub fn on_node_drag_start(
    start: On<Pointer<DragStart>>,
    mut drag: ResMut<ActiveNodeDrag>,
    mut graph: ResMut<GraphResource>,
    mut selected: ResMut<SelectedNode>,
    mut terminal: ResMut<TerminalState>,
    nodes: Query<&VmNode>,
    child_of: Query<&ChildOf>,
    titles: Query<(), With<VmNodeTitle>>,
    ports: Query<(), With<GraphPort>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    viewport: Query<Entity, With<GraphCanvasViewport>>,
    mut commands: Commands,
) {
    if start.button != PointerButton::Primary {
        return;
    }
    if ports.get(start.original_event_target()).is_ok() {
        return;
    }
    let Some(node_entity) =
        vm_node_entity(start.event_target(), &nodes, &child_of, &titles)
    else {
        return;
    };
    let Ok(vm) = nodes.get(node_entity) else {
        return;
    };

    // Agar tugun tanlovda bo'lmasa — uni yakka tanlash
    if !selected.is_selected(vm.id) {
        selected.select_only(vm.id);
    }

    let alt = keyboard.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]);

    let drag_ids: Vec<NodeId> = if alt {
        // Alt bosilgan: barcha tanlangan tugunlarni nusxalash
        let offset = Vec2::new(24.0, 24.0);
        let Ok(vp_entity) = viewport.single() else {
            return;
        };

        let base_ids: Vec<NodeId> = selected.iter().collect();
        let mut new_ids: Vec<NodeId> = Vec::new();
        for &id in &base_ids {
            if let Some(new_id) = graph.duplicate_node(id, offset) {
                new_ids.push(new_id);
            }
        }

        // Yangi tugunlar uchun UI yaratamiyz
        let new_nodes: Vec<_> = new_ids
            .iter()
            .filter_map(|&id| graph.node(id).cloned())
            .collect();
        commands.entity(vp_entity).with_children(|vp| {
            for node_data in &new_nodes {
                spawn_vm_node(vp, node_data, true);
            }
        });

        // Faqat nusxalarni tanlash
        selected.clear();
        for &id in &new_ids {
            selected.add_to_selection(id);
        }

        terminal.log(format!("[node] {} ta tugun nusxalandi", new_ids.len()));
        new_ids
    } else {
        // Oddiy drag: barcha tanlangan tugunlarni sudralash
        selected.iter().collect()
    };

    // Drag ma'lumotlarini to'ldirish
    // vm.id — haqiqatan siljitilayotgan tugun (event target), primary emas
    drag.node = Some(vm.id);
    drag.origins.clear();
    for id in drag_ids {
        if let Some(data) = graph.node(id) {
            drag.origins.insert(id, data.position);
        }
    }
}

pub fn on_node_drag(
    event: On<Pointer<Drag>>,
    drag: Res<ActiveNodeDrag>,
    view: Res<GraphCanvasView>,
    mut graph: ResMut<GraphResource>,
    child_of: Query<&ChildOf>,
    titles: Query<(), With<VmNodeTitle>>,
    mut node_q: Query<(Entity, &VmNode, &mut Node)>,
) {
    let Some(primary_id) = drag.node else {
        return;
    };

    // Faqat drag boshlagan tugunning eventida ishlov ber (dublikat eventlarni e'tiborsiz qoldirish)
    let event_target = event.event_target();
    let is_primary = node_q
        .iter()
        .any(|(e, vm, _)| vm.id == primary_id && (e == event_target || {
            // Sarlavha child orqali tekshirish
            titles.get(event_target).is_ok()
                && child_of.get(event_target).ok().map(|c| c.parent()) == Some(e)
        }));

    if !is_primary {
        return;
    }

    let delta_graph = event.distance / view.zoom;

    // Barcha sudralayotgan tugunlarni yangilash
    for (_, vm, mut node) in &mut node_q {
        if let Some(&origin) = drag.origins.get(&vm.id) {
            let new_pos = origin + delta_graph;
            graph.set_position(vm.id, new_pos);
            node.left = Val::Px(new_pos.x);
            node.top = Val::Px(new_pos.y);
        }
    }
}

pub fn on_node_drag_end(_end: On<Pointer<DragEnd>>, mut drag: ResMut<ActiveNodeDrag>) {
    drag.node = None;
    drag.origins.clear();
}

pub fn on_node_press(
    press: On<Pointer<Press>>,
    mut selected: ResMut<SelectedNode>,
    nodes: Query<&VmNode>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if press.button != PointerButton::Primary {
        return;
    }
    let Ok(vm) = nodes.get(press.event_target()) else {
        return;
    };
    let shift = keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    if shift {
        selected.toggle(vm.id);
    } else if !selected.is_selected(vm.id) {
        // Yangi tugunga bosildi → darhol yagona tanlash
        selected.select_only(vm.id);
    }
    // Allaqachon tanlangan tugunga bosildi — hech narsa qilma.
    // Ko'p-tanlov drag uchun saqlanib qoladi.
    // on_node_click drag bo'lmagan click da yagona tanlovga o'tkazadi.
}

pub fn on_node_click(
    click: On<Pointer<Click>>,
    mut selected: ResMut<SelectedNode>,
    nodes: Query<&VmNode>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if click.button != PointerButton::Primary {
        return;
    }
    let Ok(vm) = nodes.get(click.event_target()) else {
        return;
    };
    let shift = keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    // Shift holida press allaqachon toggle qildi → skip
    if shift {
        return;
    }
    // Drag bo'lmagan oddiy click: ko'p-tanlovni yagona tanlovga kamaytirish
    if selected.len() > 1 {
        selected.select_only(vm.id);
    }
}
