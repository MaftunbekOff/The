//! Execution context for PIE node evaluation.
//!
//! Bevy API ga BEVOSITA bog'liq emas — Bevy yangilansa faqat
//! `bevy_bridge.rs` va `systems/` o'zgaradi.

mod eval_float;
mod eval_bool;
mod eval_string;
mod eval_int;
mod eval_entity;
mod eval_vec2;

use std::collections::HashMap;

use twelfth_visual_blueprint::{
    ast::VisualScriptGraph,
    nodes::VisualNode,
    Gold,
};

use crate::state::TerminalState;

use crate::interpreter::bevy_bridge::InputSnapshot;
use crate::interpreter::resources::{BlackboardResource, PieEntityTable};

pub(crate) use crate::interpreter::resources::{PieTransformOp, PieEntitySpawnRequest};
pub(super) use crate::interpreter::value::PieValue;

pub(crate) struct PieExecContext<'a> {
    pub graph: &'a VisualScriptGraph,
    pub terminal: &'a mut TerminalState,
    pub gold: Option<&'a mut Gold>,
    pub key_input: Option<&'a InputSnapshot>,
    pub delta_time: f32,
    /// Jami o'yin vaqti (sekundda) — GetGameTime tugunida ishlatiladi.
    pub game_time: f32,
    pub data: HashMap<(usize, &'static str), PieValue>,
    pub delay_requests: Vec<(f32, usize)>,
    pub transform_snapshot: HashMap<u32, [f32; 3]>,
    pub scale_snapshot: HashMap<u32, [f32; 3]>,
    pub rotation_snapshot: HashMap<u32, [f32; 3]>,
    pub transform_ops: Vec<PieTransformOp>,
    pub entity_spawns: Vec<PieEntitySpawnRequest>,
    pub entity_despawns: Vec<u32>,
    pub entity_table: &'a PieEntityTable,
    /// Keyingi entity slot ID — SpawnEntity tomidin mahalliy ravishda o'sadi.
    pub next_entity_slot: u32,
    /// Script o'zgaruvchilari (Variables va Arrays).
    pub blackboard: &'a mut BlackboardResource,
    /// Exec stack chuqurligi — cheksiz rekursiyani oldini olish uchun.
    pub depth: usize,
}

impl PieExecContext<'_> {
    // ── Gold helpers ──────────────────────────────────────────────────────────

    pub fn gold_value(&self) -> f32 {
        self.gold.as_ref().map(|g| g.value).unwrap_or(0.0)
    }

    pub fn add_gold(&mut self, amount: f32) {
        if let Some(gold) = self.gold.as_deref_mut() {
            gold.value += amount;
        }
    }

    // ── Cache setters ─────────────────────────────────────────────────────────

    pub fn set_bool(&mut self, node: usize, pin: &'static str, v: bool) {
        self.data.insert((node, pin), PieValue::Bool(v));
    }

    pub fn set_float(&mut self, node: usize, pin: &'static str, v: f32) {
        self.data.insert((node, pin), PieValue::Float(v));
    }

    #[allow(dead_code)]
    pub fn set_int(&mut self, node: usize, pin: &'static str, v: i32) {
        self.data.insert((node, pin), PieValue::Int(v));
    }

    #[allow(dead_code)]
    pub fn set_str(&mut self, node: usize, pin: &'static str, v: String) {
        self.data.insert((node, pin), PieValue::Str(v));
    }

    // ── Float resolution ──────────────────────────────────────────────────────

    pub fn resolve_float_in(&self, node_id: usize, pin: &str, node: &VisualNode) -> f32 {
        if let Some(src) = self.graph.data_source(node_id, pin) {
            return self.eval_float_out(src.node_id, src.pin);
        }
        node.float_literal_for(pin)
    }

    // ── Vec3 resolution ───────────────────────────────────────────────────────

    pub fn resolve_vec3_in(&self, node_id: usize, pin: &str, _node: &VisualNode) -> [f32; 3] {
        if let Some(src) = self.graph.data_source(node_id, pin) {
            return self.eval_vec3_out(src.node_id, src.pin);
        }
        [0.0; 3]
    }

    // ── Vec2 resolution ───────────────────────────────────────────────────────

    pub fn resolve_vec2_in(&self, node_id: usize, pin: &str, _node: &VisualNode) -> [f32; 2] {
        if let Some(src) = self.graph.data_source(node_id, pin) {
            return self.eval_vec2_out(src.node_id, src.pin);
        }
        // Vec2Make literal: float_a = x, float_b = y (stored on consuming node, not yet resolved)
        [0.0; 2]
    }

    // ── Bool resolution ───────────────────────────────────────────────────────

    pub fn resolve_bool_in(&self, node_id: usize, pin: &str, node: &VisualNode) -> bool {
        if let Some(src) = self.graph.data_source(node_id, pin) {
            return self.eval_bool_out(src.node_id, src.pin);
        }
        node.bool_literal_for(pin)
    }

    pub fn resolve_bool(&self, node_id: usize, node: &VisualNode) -> bool {
        self.resolve_bool_in(node_id, "condition", node)
    }

    // ── String resolution ─────────────────────────────────────────────────────

    pub fn resolve_string(&self, node_id: usize, node: &VisualNode) -> String {
        if let Some(src) = self.graph.data_source(node_id, "message") {
            return self.eval_string_out(src.node_id, src.pin);
        }
        node.log_message.clone().unwrap_or_else(|| "<bo'sh>".into())
    }

    pub fn resolve_string_in(&self, node_id: usize, pin: &str, node: &VisualNode) -> String {
        if let Some(src) = self.graph.data_source(node_id, pin) {
            return self.eval_string_out(src.node_id, src.pin);
        }
        node.string_literal_for(pin)
    }

    // ── Integer resolution ────────────────────────────────────────────────────

    pub fn resolve_int_in(&self, node_id: usize, pin: &str, node: &VisualNode) -> i32 {
        if let Some(src) = self.graph.data_source(node_id, pin) {
            return self.eval_int_out(src.node_id, src.pin);
        }
        node.int_literal_for(pin)
    }

    // ── Logging ───────────────────────────────────────────────────────────────

    pub fn log(&mut self, line: impl Into<String>) {
        self.terminal.log(line);
    }
}
