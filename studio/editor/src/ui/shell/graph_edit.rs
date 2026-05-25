//! Graph editing actions — load, undo/redo, palette, delete, disconnect.

use bevy::prelude::*;

use crate::graph::{GraphResource, VmNode};
use crate::state::{LoadGraphRequest, SelectedNode, TerminalState, UndoStack};
use crate::ui::components::GraphCanvasViewport;
use crate::ui::nodes::{spawn_vm_node, AddNodeButton, DeleteNodeButton, DisconnectButton};

/// `LoadGraphRequest` resource'dagi yangi grafni ECS ga o'tkazadi.
pub fn load_graph_system(
    mut commands: Commands,
    mut load_req: ResMut<LoadGraphRequest>,
    mut graph: ResMut<GraphResource>,
    mut undo: ResMut<UndoStack>,
    mut terminal: ResMut<TerminalState>,
    old_nodes: Query<Entity, With<VmNode>>,
    canvas: Query<Entity, With<GraphCanvasViewport>>,
) {
    let Some(new_graph) = load_req.0.take() else {
        return;
    };

    undo.push(graph.snapshot());
    for entity in &old_nodes {
        commands.entity(entity).despawn();
    }
    *graph = new_graph;

    if let Ok(canvas_entity) = canvas.single() {
        commands.entity(canvas_entity).with_children(|c| {
            for node in &graph.nodes {
                spawn_vm_node(c, node, false);
            }
        });
    }
    terminal.log(format!("[load] {} tugun spawn qilindi", graph.nodes.len()));
}

pub fn undo_redo_system(
    mut commands: Commands,
    mut graph: ResMut<GraphResource>,
    mut undo: ResMut<UndoStack>,
    mut terminal: ResMut<TerminalState>,
    keys: Res<ButtonInput<KeyCode>>,
    old_nodes: Query<Entity, With<VmNode>>,
    canvas: Query<Entity, With<GraphCanvasViewport>>,
) {
    let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);
    if !ctrl {
        return;
    }

    let shift   = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    let do_undo = keys.just_pressed(KeyCode::KeyZ) && !shift;
    let do_redo = keys.just_pressed(KeyCode::KeyY)
        || (keys.just_pressed(KeyCode::KeyZ) && shift);

    let restored = if do_undo {
        undo.undo(graph.snapshot())
    } else if do_redo {
        undo.redo(graph.snapshot())
    } else {
        return;
    };

    let Some(snap) = restored else {
        terminal.log(if do_undo { "[undo] Undo tarixi bo'sh" } else { "[redo] Redo tarixi bo'sh" });
        return;
    };

    for entity in &old_nodes {
        commands.entity(entity).despawn();
    }
    *graph = snap;

    if let Ok(canvas_entity) = canvas.single() {
        commands.entity(canvas_entity).with_children(|c| {
            for node in &graph.nodes {
                spawn_vm_node(c, node, false);
            }
        });
    }
    terminal.log(if do_undo { "[undo] Qaytarildi" } else { "[redo] Qayta qo'llanildi" });
}

pub fn node_palette_buttons(
    mut commands: Commands,
    mut graph: ResMut<GraphResource>,
    mut terminal: ResMut<TerminalState>,
    mut undo: ResMut<UndoStack>,
    selected: Res<SelectedNode>,
    canvas: Query<Entity, With<GraphCanvasViewport>>,
    add_q: Query<(&Interaction, &AddNodeButton), Changed<Interaction>>,
) {
    let Ok(canvas_entity) = canvas.single() else {
        return;
    };

    for (interaction, btn) in &add_q {
        if *interaction != Interaction::Pressed {
            continue;
        }
        undo.push(graph.snapshot());
        let n = graph.nodes.len() as f32;
        let position = Vec2::new(64.0 + n * 28.0, 100.0 + n * 36.0);
        let id = graph.add_node_at(btn.kind, position);
        let Some(node) = graph.node(id).cloned() else { continue };
        let sel = selected.is_selected(id);
        commands.entity(canvas_entity).with_children(|c| {
            spawn_vm_node(c, &node, sel);
        });
        terminal.log(format!("[node] {} (#{})", btn.kind.label(), id.0));
    }
}

pub fn node_delete_buttons(
    mut commands: Commands,
    mut graph: ResMut<GraphResource>,
    mut selected: ResMut<SelectedNode>,
    mut terminal: ResMut<TerminalState>,
    mut undo: ResMut<UndoStack>,
    q: Query<&Interaction, (Changed<Interaction>, With<DeleteNodeButton>)>,
    nodes: Query<(Entity, &VmNode)>,
) {
    for interaction in &q {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let Some(id) = selected.primary else {
            terminal.log("[node] Avval tugunni tanlang");
            continue;
        };
        undo.push(graph.snapshot());
        if !graph.remove_node(id) {
            continue;
        }
        selected.nodes.remove(&id);
        if selected.primary == Some(id) {
            selected.primary = selected.nodes.iter().next().copied();
        }
        for (entity, vm) in &nodes {
            if vm.id == id {
                commands.entity(entity).despawn();
                break;
            }
        }
        terminal.log(format!("[node] O'chirildi: #{}", id.0));
    }
}

pub fn node_disconnect_buttons(
    mut graph: ResMut<GraphResource>,
    selected: Res<SelectedNode>,
    mut terminal: ResMut<TerminalState>,
    mut undo: ResMut<UndoStack>,
    q: Query<&Interaction, (Changed<Interaction>, With<DisconnectButton>)>,
) {
    for interaction in &q {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let Some(id) = selected.primary else {
            terminal.log("[wire] Tugun tanlanmagan");
            continue;
        };
        undo.push(graph.snapshot());
        let n = graph.disconnect_all_from_node(id);
        terminal.log(format!("[wire] {} — {n} sim uzildi", graph.title(id)));
    }
}
