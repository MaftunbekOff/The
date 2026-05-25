//! PIE lifecycle systems — start and stop.

use std::collections::HashMap;

use bevy::prelude::*;
use twelfth_visual_blueprint::Gold;

use crate::graph::GraphResource;
use crate::state::TerminalState;

use crate::interpreter::bevy_bridge::{apply_transform_ops_to_ecs, build_transform_snapshots};
use crate::interpreter::resources::{
    BlackboardResource, PieDelayEntry, PieDelayQueue, PieEntityHolder,
    PieEntitySpawnRequest, PieEntityTable, PieGoldHolder, PiePendingGraph, PieRuntimeError,
    PieSession, PieStartRequested, ScriptActor,
};
use crate::interpreter::gold::ensure_pie_gold;
use crate::interpreter::runtime::run_begin_play;
use crate::interpreter::subgraph::tick_subgraph;
use crate::scene::SceneEditorEntity;

// ── Entity spawn/despawn helpers ──────────────────────────────────────────────

pub(super) fn process_spawns(
    spawns: Vec<PieEntitySpawnRequest>,
    entity_table: &mut PieEntityTable,
    commands: &mut Commands,
    next_slot_after_exec: u32,
) {
    // Context da hisoblangan keyingi slotni saqlaymiz
    entity_table.next_slot = entity_table.next_slot.max(next_slot_after_exec);
    for req in spawns {
        let entity = commands
            .spawn((
                PieEntityHolder { slot: req.slot },
                Transform::default(),
                GlobalTransform::default(),
            ))
            .id();
        entity_table.entities.insert(req.slot, entity);
        // Entity nomi register qilish — GetNamedEntity ishlashi uchun
        if let Some(name) = req.name {
            entity_table.register_named(&name, req.slot);
        }
    }
}

pub(super) fn process_despawns(
    despawns: Vec<u32>,
    entity_table: &mut PieEntityTable,
    commands: &mut Commands,
) {
    for slot in despawns {
        if let Some(entity) = entity_table.entities.remove(&slot) {
            commands.entity(entity).despawn();
        }
    }
}

// ── pie_start_system ──────────────────────────────────────────────────────────

pub fn pie_start_system(
    mut requested: ResMut<PieStartRequested>,
    mut pending: ResMut<PiePendingGraph>,
    mut session: ResMut<PieSession>,
    mut terminal: ResMut<TerminalState>,
    mut delay_q: ResMut<PieDelayQueue>,
    mut entity_table: ResMut<PieEntityTable>,
    mut blackboard: ResMut<BlackboardResource>,
    mut pie_error: ResMut<PieRuntimeError>,
    mut commands: Commands,
    holders: Query<Entity, With<PieGoldHolder>>,
    mut gold: Query<&mut Gold, With<PieGoldHolder>>,
    entity_holders: Query<(Entity, &PieEntityHolder)>,
    mut transforms: Query<&mut Transform, With<PieEntityHolder>>,
    graph: Res<GraphResource>,
    time: Res<Time>,
    // Scene entitylari uchun
    scene_entities: Query<(Entity, &SceneEditorEntity)>,
) {
    let ast = if let Some(g) = pending.0.take() {
        g
    } else {
        if !requested.0 {
            return;
        }
        requested.0 = false;
        // Yangi PIE boshlaganda xatolarni tozalaymiz
        pie_error.clear();
        match graph.to_blueprint_graph() {
            Ok(g) => g,
            Err(err) => {
                terminal.log(format!("[PIE] Graf xato: {err}"));
                return;
            }
        }
    };

    if ast.needs_gold_query() && holders.is_empty() {
        ensure_pie_gold(&mut commands, &ast, &holders);
        pending.0 = Some(ast);
        return;
    }

    session.tick_graph = tick_subgraph(&ast);
    session.active = true;
    delay_q.entries.clear();
    blackboard.clear();  // Sessiya boshida tozalaymiz
    terminal.log("[PIE] Play-in-Editor boshlandi ▶");

    // Scene entitylariga ScriptActor biriktirish
    let mut actor_count = 0u32;
    for (entity, scene_ent) in &scene_entities {
        commands.entity(entity).insert(ScriptActor {
            vars: Default::default(),
            self_slot: scene_ent.scene_id as u32,
            active: true,
        });
        actor_count += 1;
    }
    if actor_count > 0 {
        terminal.log(format!("[PIE] {actor_count} scene entity ScriptActor oldi"));
    }

    let game_time = time.elapsed_secs();
    let (t_snap, s_snap, r_snap) = build_transform_snapshots(&transforms, &entity_holders);
    let arc_ast = std::sync::Arc::new(ast);

    let run_result = match gold.single_mut() {
        Ok(mut g) => run_begin_play(
            &arc_ast, &mut terminal, Some(&mut *g), None, 0.0, game_time, "BeginPlay",
            &entity_table, &mut blackboard, t_snap, s_snap, r_snap,
        ),
        Err(_) if arc_ast.needs_gold_query() => {
            pending.0 = Some((*arc_ast).clone());
            session.active = false;
            session.tick_graph = None;
            return;
        }
        Err(_) => run_begin_play(
            &arc_ast, &mut terminal, None, None, 0.0, game_time, "BeginPlay",
            &entity_table, &mut blackboard, HashMap::new(), HashMap::new(), HashMap::new(),
        ),
    };

    match run_result {
        Ok(result) => {
            let next_slot = result.next_entity_slot;
            for (dur, node_id) in result.delays {
                delay_q.entries.push(PieDelayEntry {
                    remaining: dur,
                    resume_node_id: node_id,
                    graph: arc_ast.clone(),
                });
            }
            apply_transform_ops_to_ecs(
                result.transform_ops,
                &entity_table,
                &mut transforms,
                &entity_holders,
            );
            process_spawns(result.entity_spawns, &mut entity_table, &mut commands, next_slot);
            process_despawns(result.entity_despawns, &mut entity_table, &mut commands);
        }
        Err(err) => {
            terminal.log(format!("[PIE] Xato: {err}"));
            session.active = false;
            session.tick_graph = None;
        }
    }
}

// ── pie_stop ──────────────────────────────────────────────────────────────────

pub fn pie_stop(
    session: &mut PieSession,
    commands: &mut Commands,
    holders: Query<Entity, With<PieGoldHolder>>,
    entity_holders: Query<Entity, With<PieEntityHolder>>,
    entity_table: &mut PieEntityTable,
    blackboard: &mut BlackboardResource,
    terminal: &mut TerminalState,
    scene_actors: Query<Entity, With<ScriptActor>>,
) {
    if session.active {
        terminal.log("[PIE] To'xtatildi");
    }
    session.active = false;
    session.tick_graph = None;
    // PIE tomonidan yaratilgan entitylarni o'chirish
    for entity in &holders {
        commands.entity(entity).despawn();
    }
    for entity in &entity_holders {
        commands.entity(entity).despawn();
    }
    // Scene entitylardan ScriptActor olib tashlash (entity o'chirilmaydi)
    for entity in &scene_actors {
        commands.entity(entity).remove::<ScriptActor>();
    }
    entity_table.entities.clear();
    entity_table.named.clear();
    entity_table.next_slot = 0;
    blackboard.clear();
}
