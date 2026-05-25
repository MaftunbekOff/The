//! BeginPlay / Tick entry execution.
//!
//! Bu fayl Bevy API ga BEVOSITA bog'liq emas.
//! Bevy yangilansa — faqat `bevy_bridge.rs` va `systems.rs` o'zgaradi.

use std::collections::HashMap;

use twelfth_visual_blueprint::{ast::VisualScriptGraph, Gold};

use crate::state::TerminalState;

use super::bevy_bridge::InputSnapshot;
use super::context::PieExecContext;
use super::exec::execute_exec_node;
use super::resources::{BlackboardResource, PieEntitySpawnRequest, PieEntityTable, PieTransformOp};

/// Execution result — delay so'rovlari, transform ops, spawn/despawn.
pub(crate) struct PieRunResult {
    pub delays: Vec<(f32, usize)>,
    pub transform_ops: Vec<PieTransformOp>,
    pub entity_spawns: Vec<PieEntitySpawnRequest>,
    pub entity_despawns: Vec<u32>,
    pub next_entity_slot: u32,
}

/// BeginPlay yoki Tick event zanjirini ishga tushiradi.
///
/// `key_input` — `InputSnapshot` (Bevy-dan mustaqil).
/// `entity_table` — readonly slot → Entity mapping.
/// `blackboard` — script variables.
pub(crate) fn run_begin_play(
    graph: &VisualScriptGraph,
    terminal: &mut TerminalState,
    gold: Option<&mut Gold>,
    key_input: Option<&InputSnapshot>,
    delta_time: f32,
    game_time: f32,
    event_label: &str,
    entity_table: &PieEntityTable,
    blackboard: &mut BlackboardResource,
    transform_snapshot: HashMap<u32, [f32; 3]>,
    scale_snapshot: HashMap<u32, [f32; 3]>,
    rotation_snapshot: HashMap<u32, [f32; 3]>,
) -> Result<PieRunResult, String> {
    let entry = if event_label == "Tick" || event_label == "FixedTick" {
        graph.tick_entry()?
    } else {
        graph.begin_play_entry()?
    };
    terminal.log(format!("[PIE] {event_label} → exec zanjir"));
    let mut ctx = PieExecContext {
        graph,
        terminal,
        gold,
        key_input,
        delta_time,
        game_time,
        data: HashMap::new(),
        delay_requests: Vec::new(),
        transform_snapshot,
        scale_snapshot,
        rotation_snapshot,
        transform_ops: Vec::new(),
        entity_spawns: Vec::new(),
        entity_despawns: Vec::new(),
        entity_table,
        next_entity_slot: entity_table.next_slot,
        blackboard,
        depth: 0,
    };
    if event_label == "Tick" || event_label == "FixedTick" {
        ctx.set_float(entry, "delta_time", delta_time);
    }
    execute_exec_node(&mut ctx, entry)?;
    if event_label == "BeginPlay" {
        ctx.log("[PIE] BeginPlay zanjir tugadi");
    }
    Ok(PieRunResult {
        delays: ctx.delay_requests,
        transform_ops: ctx.transform_ops,
        entity_spawns: ctx.entity_spawns,
        entity_despawns: ctx.entity_despawns,
        next_entity_slot: ctx.next_entity_slot,
    })
}

/// Custom event zanjirini nomlari bo'yicha ishga tushiradi.
#[allow(dead_code)]
pub(crate) fn run_custom_event(
    event_name: &str,
    graph: &VisualScriptGraph,
    terminal: &mut TerminalState,
    gold: Option<&mut Gold>,
    key_input: Option<&InputSnapshot>,
    delta_time: f32,
    game_time: f32,
    entity_table: &PieEntityTable,
    blackboard: &mut BlackboardResource,
    transform_snapshot: HashMap<u32, [f32; 3]>,
    scale_snapshot: HashMap<u32, [f32; 3]>,
    rotation_snapshot: HashMap<u32, [f32; 3]>,
) -> Result<PieRunResult, String> {
    use twelfth_visual_blueprint::nodes::NodeKind;

    let entries: Vec<usize> = graph
        .nodes()
        .filter(|(_, n)| {
            n.kind == NodeKind::EventCustomBegin
                && n.event_name.as_deref() == Some(event_name)
        })
        .map(|(id, _)| id)
        .collect();

    if entries.is_empty() {
        return Err(format!("PIE: '{event_name}' custom event topilmadi"));
    }

    let mut ctx = PieExecContext {
        graph,
        terminal,
        gold,
        key_input,
        delta_time,
        game_time,
        data: HashMap::new(),
        delay_requests: Vec::new(),
        transform_snapshot,
        scale_snapshot,
        rotation_snapshot,
        transform_ops: Vec::new(),
        entity_spawns: Vec::new(),
        entity_despawns: Vec::new(),
        entity_table,
        next_entity_slot: entity_table.next_slot,
        blackboard,
        depth: 0,
    };

    for entry in entries {
        execute_exec_node(&mut ctx, entry)?;
    }

    Ok(PieRunResult {
        delays: ctx.delay_requests,
        transform_ops: ctx.transform_ops,
        entity_spawns: ctx.entity_spawns,
        entity_despawns: ctx.entity_despawns,
        next_entity_slot: ctx.next_entity_slot,
    })
}
