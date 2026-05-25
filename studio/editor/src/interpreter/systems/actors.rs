//! Per-entity ScriptActor tick tizimi.
//!
//! `BlackboardResource` (global) PIE session uchun saqlanadi.
//! `ScriptActor` (per-entity component) esa har bir NPC/citizen uchun
//! alohida o'zgaruvchilar (vars) saqlaydi — hech qanday aralashuv yo'q.
//!
//! ## Ishlash prinsipi
//! 1. PIE sessiyasi aktiv bo'lsa, Query<&mut ScriptActor> iteratsiya qilinadi.
//! 2. Har bir aktiv actor uchun global tick_graph BITTA ishga tushiriladi.
//! 3. Actor'ning vars'i vaqtincha swap qilinadi (BlackboardResource sifatida) —
//!    klon qilinmaydi, faqat pointer almashtiriladi: O(1).
//! 4. Natijalar (transform ops, spawn/despawn) yig'iladi va birdan qo'llaniladi.

use bevy::prelude::*;

use crate::state::TerminalState;

use crate::interpreter::bevy_bridge::{
    apply_transform_ops_to_ecs, build_transform_snapshots, read_delta_time, snapshot_input,
};
use crate::interpreter::resources::{
    BlackboardResource, PieDelayEntry, PieDelayQueue, PieEntityHolder, PieEntityTable,
    PieRuntimeError, PieSession, ScriptActor,
};
use crate::interpreter::runtime::run_begin_play;

use super::lifecycle::{process_spawns, process_despawns};

/// PIE rejimida har bir `ScriptActor`'ni mustaqil ijro etadi.
///
/// **Ishlash ko'rsatkichi:**
/// - O'zgaruvchi almashish: O(1) (HashMap pointer swap)
/// - Iteratsiya: O(n_actors)
/// - Bevy parallel par_iter_mut ga tayyor (terminal/entity_table lock kerak emas)
pub fn script_actor_tick_system(
    session: Res<PieSession>,
    mut delay_q: ResMut<PieDelayQueue>,
    mut terminal: ResMut<TerminalState>,
    mut entity_table: ResMut<PieEntityTable>,
    entity_holders: Query<(Entity, &PieEntityHolder)>,
    mut transforms: Query<&mut Transform, With<PieEntityHolder>>,
    mut actors: Query<&mut ScriptActor>,
    mut commands: Commands,
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut pie_error: ResMut<PieRuntimeError>,
) {
    if !session.active {
        return;
    }
    let Some(graph) = session.tick_graph.as_ref() else { return };
    if graph.tick_entry().is_err() {
        return;
    }

    let dt        = read_delta_time(&time);
    let game_time = time.elapsed_secs();
    let input_snap = snapshot_input(&key_input);

    let (t_snap, s_snap, r_snap) =
        build_transform_snapshots(&transforms, &entity_holders);

    for mut actor in actors.iter_mut() {
        if !actor.active {
            continue;
        }

        // O(1): actor.vars ↔ temp_bb.vars — klon yo'q, pointer almashadi
        let mut temp_bb = BlackboardResource::default();
        std::mem::swap(&mut temp_bb.vars, &mut actor.vars);

        // GetSelfEntity uchun self_slot ni blackboard'ga yozamiz
        temp_bb.vars.insert(
            "__self_slot__".to_string(),
            crate::interpreter::value::PieValue::Int(actor.self_slot as i32),
        );

        let result = run_begin_play(
            graph,
            &mut terminal,
            None,                     // gold per-entity emas (hali)
            Some(&input_snap),
            dt,
            game_time,
            "Tick",
            &entity_table,
            &mut temp_bb,
            t_snap.clone(),
            s_snap.clone(),
            r_snap.clone(),
        );

        // vars ni qaytaramiz
        std::mem::swap(&mut temp_bb.vars, &mut actor.vars);

        match result {
            Ok(r) => {
                let next_slot = r.next_entity_slot;
                apply_transform_ops_to_ecs(
                    r.transform_ops,
                    &entity_table,
                    &mut transforms,
                    &entity_holders,
                );
                process_spawns(r.entity_spawns, &mut entity_table, &mut commands, next_slot);
                process_despawns(r.entity_despawns, &mut entity_table, &mut commands);
                for (dur, node_id) in r.delays {
                    delay_q.entries.push(PieDelayEntry {
                        remaining: dur,
                        resume_node_id: node_id,
                        graph: graph.clone().into(),
                    });
                }
            }
            Err(e) => {
                terminal.log(format!(
                    "[PIE:Actor slot={}] Xato: {e}",
                    actor.self_slot
                ));
                let node_id = extract_node_id(&e);
                pie_error.set(node_id, e);
            }
        }
    }
}

fn extract_node_id(err: &str) -> Option<u32> {
    if let Some(pos) = err.find("node#") {
        let after = &err[pos + 5..];
        let end = after.find(|c: char| !c.is_ascii_digit()).unwrap_or(after.len());
        if end > 0 { return after[..end].parse().ok(); }
    }
    None
}
