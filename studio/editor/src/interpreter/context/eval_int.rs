//! Integer evaluation for pure nodes.

use twelfth_visual_blueprint::nodes::NodeKind;

use super::{PieExecContext, PieValue};

impl PieExecContext<'_> {
    pub(super) fn eval_int_out(&self, node_id: usize, pin: &'static str) -> i32 {
        if let Some(PieValue::Int(v)) = self.data.get(&(node_id, pin)) {
            return *v;
        }
        let Some(node) = self.graph.node(node_id) else { return 0 };
        let node = node.clone();
        match node.kind {
            NodeKind::IntAdd =>
                self.resolve_int_in(node_id, "int_a", &node) + self.resolve_int_in(node_id, "int_b", &node),
            NodeKind::IntSubtract =>
                self.resolve_int_in(node_id, "int_a", &node) - self.resolve_int_in(node_id, "int_b", &node),
            NodeKind::IntMultiply =>
                self.resolve_int_in(node_id, "int_a", &node) * self.resolve_int_in(node_id, "int_b", &node),
            NodeKind::IntDivide => {
                let b = self.resolve_int_in(node_id, "int_b", &node);
                if b == 0 { 0 } else { self.resolve_int_in(node_id, "int_a", &node) / b }
            }
            NodeKind::IntModulo => {
                let b = self.resolve_int_in(node_id, "int_b", &node);
                if b == 0 { 0 } else { self.resolve_int_in(node_id, "int_a", &node) % b }
            }
            NodeKind::FloatToInt =>
                self.resolve_float_in(node_id, "float_val", &node) as i32,

            // ── Variables ─────────────────────────────────────────────────────
            NodeKind::GetIntVar => {
                let name = node.string_a.as_deref().unwrap_or("");
                if let Some(PieValue::Int(v)) = self.blackboard.vars.get(name) {
                    *v
                } else {
                    0
                }
            }

            // ── Collections ───────────────────────────────────────────────────
            NodeKind::FloatArrayLength => {
                let name = node.string_a.as_deref().unwrap_or("");
                if let Some(PieValue::FloatArray(arr)) = self.blackboard.vars.get(name) {
                    arr.len() as i32
                } else {
                    0
                }
            }
            NodeKind::IntArrayGet => {
                let array_name = node.string_a.as_deref().unwrap_or("");
                let idx = self.resolve_int_in(node_id, "index", &node) as usize;
                if let Some(PieValue::IntArray(arr)) = self.blackboard.vars.get(array_name) {
                    arr.get(idx).copied().unwrap_or(0)
                } else {
                    0
                }
            }
            NodeKind::IntArrayLength => {
                let name = node.string_a.as_deref().unwrap_or("");
                if let Some(PieValue::IntArray(arr)) = self.blackboard.vars.get(name) {
                    arr.len() as i32
                } else {
                    0
                }
            }
            NodeKind::StringArrayLength => {
                let name = node.string_a.as_deref().unwrap_or("");
                if let Some(PieValue::StringArray(arr)) = self.blackboard.vars.get(name) {
                    arr.len() as i32
                } else {
                    0
                }
            }

            // ── Select ternary ─────────────────────────────────────────────────
            NodeKind::SelectInt => {
                let cond = self.resolve_bool_in(node_id, "condition", &node);
                if cond {
                    self.resolve_int_in(node_id, "value_a", &node)
                } else {
                    self.resolve_int_in(node_id, "value_b", &node)
                }
            }

            // ── ECS: entity count nodes ────────────────────────────────────────
            NodeKind::QueryAllEntities | NodeKind::EntityArrayLength => {
                self.entity_table.entities.len() as i32
            }
            NodeKind::QueryByTag => {
                let tag = node.string_a.as_deref().unwrap_or("tag").to_string();
                let key = format!("__tag__{tag}");
                self.entity_table.entities.keys()
                    .filter(|&&slot| {
                        matches!(self.blackboard.vars.get(&format!("__slot_{slot}_{key}")),
                            Some(PieValue::Bool(true)))
                    })
                    .count() as i32
            }

            _ => 0,
        }
    }
}
