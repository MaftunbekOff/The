//! String evaluation for pure nodes.

use twelfth_visual_blueprint::nodes::NodeKind;

use super::{PieExecContext, PieValue};

impl PieExecContext<'_> {
    pub(super) fn eval_string_out(&self, node_id: usize, pin: &'static str) -> String {
        if let Some(PieValue::Str(s)) = self.data.get(&(node_id, pin)) {
            return s.clone();
        }
        let Some(node) = self.graph.node(node_id) else { return String::new() };
        let node = node.clone();
        match node.kind {
            NodeKind::StringConcat => {
                let a = self.resolve_string_in(node_id, "string_a", &node);
                let b = self.resolve_string_in(node_id, "string_b", &node);
                format!("{a}{b}")
            }
            NodeKind::FloatToString => {
                let v = self.resolve_float_in(node_id, "float_val", &node);
                format!("{v:.4}")
            }
            NodeKind::BoolToString => {
                self.resolve_bool_in(node_id, "a", &node).to_string()
            }
            NodeKind::IntToString => {
                self.resolve_int_in(node_id, "int_val", &node).to_string()
            }

            // ── Variables ─────────────────────────────────────────────────────
            NodeKind::GetStringVar => {
                let name = node.string_a.as_deref().unwrap_or("");
                if let Some(PieValue::Str(s)) = self.blackboard.vars.get(name) {
                    s.clone()
                } else {
                    String::new()
                }
            }

            // ── Select ternary ─────────────────────────────────────────────────
            NodeKind::SelectString => {
                let cond = self.resolve_bool_in(node_id, "condition", &node);
                if cond {
                    self.resolve_string_in(node_id, "value_a", &node)
                } else {
                    self.resolve_string_in(node_id, "value_b", &node)
                }
            }

            // ── String Arrays ──────────────────────────────────────────────────
            NodeKind::StringArrayGet => {
                let array_name = node.string_a.as_deref().unwrap_or("");
                let idx = self.resolve_int_in(node_id, "index", &node) as usize;
                if let Some(PieValue::StringArray(arr)) = self.blackboard.vars.get(array_name) {
                    arr.get(idx).cloned().unwrap_or_default()
                } else {
                    String::new()
                }
            }

            // ── ECS: GetEntityName ─────────────────────────────────────────────
            NodeKind::GetEntityName => {
                if let Some(slot) = self.resolve_entity_in(node_id, "entity") {
                    self.entity_table.name_for_slot(slot)
                        .unwrap_or_default()
                        .to_string()
                } else {
                    String::new()
                }
            }

            _ => String::new(),
        }
    }
}
