//! PIE tick and delay-tick systems.

use std::collections::HashMap;

use bevy::input::ButtonInput;
use bevy::prelude::*;
use twelfth_visual_blueprint::Gold;

use crate::state::TerminalState;

use crate::interpreter::bevy_bridge::{
    apply_transform_ops_to_ecs, build_transform_snapshots, read_delta_time, snapshot_input,
};
use crate::interpreter::context::PieExecContext;
use crate::interpreter::exec::execute_exec_node;
use crate::interpreter::resources::{
    BlackboardResource, PieDelayEntry, PieDelayQueue, PieEntityHolder, PieEntityTable,
    PieGoldHolder, PieRuntimeError, PieSession,
};
use crate::interpreter::runtime::run_begin_play;

use super::lifecycle::{process_spawns, process_despawns};

pub fn pie_tick_system(
    session: Res<PieSession>,
    mut delay_q: ResMut<PieDelayQueue>,
    mut terminal: ResMut<TerminalState>,
    mut gold: Query<&mut Gold, With<PieGoldHolder>>,
    mut entity_table: ResMut<PieEntityTable>,
    entity_holders: Query<(Entity, &PieEntityHolder)>,
    mut transforms: Query<&mut Transform, With<PieEntityHolder>>,
    mut commands: Commands,
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut blackboard: ResMut<BlackboardResource>,
    mut pie_error: ResMut<PieRuntimeError>,
) {
    if !session.active {
        return;
    }
    let dt = read_delta_time(&time);
    let game_time = time.elapsed_secs();
    let input_snap = snapshot_input(&key_input);

    if let Some(graph) = session.tick_graph.as_ref() {
        if graph.tick_entry().is_ok() {
            let (t_snap, s_snap, r_snap) =
                build_transform_snapshots(&transforms, &entity_holders);

            let result = match gold.single_mut() {
                Ok(mut g) => run_begin_play(
                    graph, &mut terminal, Some(&mut *g),
                    Some(&input_snap), dt, game_time, "Tick",
                    &entity_table, &mut blackboard, t_snap, s_snap, r_snap,
                ),
                Err(_) => run_begin_play(
                    graph, &mut terminal, None,
                    Some(&input_snap), dt, game_time, "Tick",
                    &entity_table, &mut blackboard, HashMap::new(), HashMap::new(), HashMap::new(),
                ),
            };

            match result {
                Ok(r) => {
                    let next_slot = r.next_entity_slot;
                    for (dur, node_id) in r.delays {
                        delay_q.entries.push(PieDelayEntry {
                            remaining: dur,
                            resume_node_id: node_id,
                            graph: graph.clone().into(),
                        });
                    }
                    apply_transform_ops_to_ecs(
                        r.transform_ops,
                        &entity_table,
                        &mut transforms,
                        &entity_holders,
                    );
                    process_spawns(r.entity_spawns, &mut entity_table, &mut commands, next_slot);
                    process_despawns(r.entity_despawns, &mut entity_table, &mut commands);
                }
                Err(e) => {
                    terminal.log(format!("[PIE] Tick xato: {e}"));
                    // PieError formatidan node_id ni ajratib olamiz: "node#42: ..."
                    let node_id = extract_node_id_from_pie_error(&e);
                    pie_error.set(node_id, e.clone());
                }
            }
        }
    }

    for entry in &mut delay_q.entries {
        entry.remaining -= dt;
    }
}

pub fn pie_delay_tick_system(
    session: Res<PieSession>,
    mut delay_q: ResMut<PieDelayQueue>,
    mut terminal: ResMut<TerminalState>,
    mut gold: Query<&mut Gold, With<PieGoldHolder>>,
    mut entity_table: ResMut<PieEntityTable>,
    entity_holders: Query<(Entity, &PieEntityHolder)>,
    mut transforms: Query<&mut Transform, With<PieEntityHolder>>,
    mut commands: Commands,
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut blackboard: ResMut<BlackboardResource>,
) {
    if !session.active || delay_q.entries.is_empty() {
        return;
    }
    let input_snap = snapshot_input(&key_input);
    let game_time = time.elapsed_secs();

    let mut expired = Vec::new();
    let mut remaining = Vec::new();
    for entry in delay_q.entries.drain(..) {
        if entry.remaining <= 0.0 {
            expired.push(entry);
        } else {
            remaining.push(entry);
        }
    }
    delay_q.entries = remaining;

    for entry in expired {
        terminal.log(format!(
            "[PIE] delay tugadi → node #{} bajarilmoqda",
            entry.resume_node_id
        ));

        let (t_snap, s_snap, r_snap) =
            build_transform_snapshots(&transforms, &entity_holders);

        let mut ctx_t_ops    = Vec::new();
        let mut ctx_spawns   = Vec::new();
        let mut ctx_despawns = Vec::new();
        let mut ctx_next_slot = entity_table.next_slot;

        let new_delays = match gold.single_mut() {
            Ok(mut g) => {
                let mut ctx = PieExecContext {
                    graph: &entry.graph,
                    terminal: &mut terminal,
                    gold: Some(&mut *g),
                    key_input: Some(&input_snap),
                    delta_time: 0.0,
                    game_time,
                    data: HashMap::new(),
                    delay_requests: Vec::new(),
                    transform_snapshot: t_snap,
                    scale_snapshot: s_snap,
                    rotation_snapshot: r_snap,
                    transform_ops: Vec::new(),
                    entity_spawns: Vec::new(),
                    entity_despawns: Vec::new(),
                    entity_table: &entity_table,
                    next_entity_slot: entity_table.next_slot,
                    blackboard: &mut *blackboard,
                    depth: 0,
                };
                if let Err(e) = execute_exec_node(&mut ctx, entry.resume_node_id) {
                    ctx.terminal.log(format!("[PIE] Delay xato: {e}"));
                }
                ctx_t_ops    = ctx.transform_ops;
                ctx_spawns   = ctx.entity_spawns;
                ctx_despawns = ctx.entity_despawns;
                ctx_next_slot = ctx.next_entity_slot;
                ctx.delay_requests
            }
            Err(_) => {
                let mut ctx = PieExecContext {
                    graph: &entry.graph,
                    terminal: &mut terminal,
                    gold: None,
                    key_input: Some(&input_snap),
                    delta_time: 0.0,
                    game_time,
                    data: HashMap::new(),
                    delay_requests: Vec::new(),
                    transform_snapshot: HashMap::new(),
                    scale_snapshot: HashMap::new(),
                    rotation_snapshot: HashMap::new(),
                    transform_ops: Vec::new(),
                    entity_spawns: Vec::new(),
                    entity_despawns: Vec::new(),
                    entity_table: &entity_table,
                    next_entity_slot: entity_table.next_slot,
                    blackboard: &mut *blackboard,
                    depth: 0,
                };
                if let Err(e) = execute_exec_node(&mut ctx, entry.resume_node_id) {
                    ctx.terminal.log(format!("[PIE] Delay xato: {e}"));
                }
                ctx.delay_requests
            }
        };

        apply_transform_ops_to_ecs(ctx_t_ops, &entity_table, &mut transforms, &entity_holders);
        process_spawns(ctx_spawns, &mut entity_table, &mut commands, ctx_next_slot);
        process_despawns(ctx_despawns, &mut entity_table, &mut commands);

        for (dur, node_id) in new_delays {
            delay_q.entries.push(PieDelayEntry {
                remaining: dur,
                resume_node_id: node_id,
                graph: entry.graph.clone(),
            });
        }
    }
}

/// PieError string formatidan node ID ni ajratib oladi.
/// Format: `[NodeNotFound@node#42] message` yoki `tugun #42`
fn extract_node_id_from_pie_error(err: &str) -> Option<u32> {
    // PieError::Display format: "[Kind@node#ID] ..."
    if let Some(pos) = err.find("node#") {
        let after = &err[pos + 5..];
        let end = after.find(|c: char| !c.is_ascii_digit()).unwrap_or(after.len());
        if end > 0 {
            return after[..end].parse::<u32>().ok();
        }
    }
    // Fallback: "node #ID" (bo'sh bilan)
    if let Some(pos) = err.find("node #") {
        let after = &err[pos + 6..];
        let end = after.find(|c: char| !c.is_ascii_digit()).unwrap_or(after.len());
        if end > 0 {
            return after[..end].parse::<u32>().ok();
        }
    }
    None
}
