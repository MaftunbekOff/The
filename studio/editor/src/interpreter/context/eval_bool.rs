//! Boolean evaluation for pure nodes.

use twelfth_visual_blueprint::nodes::NodeKind;

use super::{PieExecContext, PieValue};

impl PieExecContext<'_> {
    pub(super) fn eval_bool_out(&self, node_id: usize, pin: &'static str) -> bool {
        if let Some(PieValue::Bool(v)) = self.data.get(&(node_id, pin)) {
            return *v;
        }
        let Some(node) = self.graph.node(node_id) else { return false };
        let node = node.clone();
        match node.kind {
            NodeKind::FloatGreater =>
                self.resolve_float_in(node_id, "a", &node) > self.resolve_float_in(node_id, "b", &node),
            NodeKind::FloatLess =>
                self.resolve_float_in(node_id, "a", &node) < self.resolve_float_in(node_id, "b", &node),
            NodeKind::FloatGreaterEqual =>
                self.resolve_float_in(node_id, "a", &node) >= self.resolve_float_in(node_id, "b", &node),
            NodeKind::FloatLessEqual =>
                self.resolve_float_in(node_id, "a", &node) <= self.resolve_float_in(node_id, "b", &node),
            NodeKind::FloatEqual => {
                let a   = self.resolve_float_in(node_id, "a",   &node);
                let b   = self.resolve_float_in(node_id, "b",   &node);
                let eps = self.resolve_float_in(node_id, "eps", &node);
                (a - b).abs() <= eps
            }
            NodeKind::BoolAnd =>
                self.resolve_bool_in(node_id, "a", &node) && self.resolve_bool_in(node_id, "b", &node),
            NodeKind::BoolOr =>
                self.resolve_bool_in(node_id, "a", &node) || self.resolve_bool_in(node_id, "b", &node),
            NodeKind::BoolXor =>
                self.resolve_bool_in(node_id, "a", &node) ^ self.resolve_bool_in(node_id, "b", &node),
            NodeKind::BoolNot =>
                !self.resolve_bool_in(node_id, "a", &node),
            NodeKind::IntGreater =>
                self.resolve_int_in(node_id, "int_a", &node) > self.resolve_int_in(node_id, "int_b", &node),
            NodeKind::IntLess =>
                self.resolve_int_in(node_id, "int_a", &node) < self.resolve_int_in(node_id, "int_b", &node),
            NodeKind::IntEqual =>
                self.resolve_int_in(node_id, "int_a", &node) == self.resolve_int_in(node_id, "int_b", &node),
            NodeKind::IsKeyPressed => {
                let name = node.key_name.as_deref().unwrap_or("Space");
                self.key_input.map_or(false, |ki| ki.is_pressed(name))
            }
            NodeKind::IsKeyJustPressed => {
                let name = node.key_name.as_deref().unwrap_or("Space");
                self.key_input.map_or(false, |ki| ki.is_just_pressed(name))
            }

            // ── Variables ─────────────────────────────────────────────────────
            NodeKind::GetBoolVar => {
                let name = node.string_a.as_deref().unwrap_or("");
                if let Some(PieValue::Bool(v)) = self.blackboard.vars.get(name) {
                    *v
                } else {
                    false
                }
            }

            // ── Helpers ───────────────────────────────────────────────────────
            NodeKind::IsValidEntity => {
                if let Some(slot) = self.resolve_entity_in(node_id, "entity") {
                    self.entity_table.entity_for_slot(slot).is_some()
                } else {
                    false
                }
            }

            // ── Select ternary ─────────────────────────────────────────────────
            NodeKind::SelectBool => {
                let cond = self.resolve_bool_in(node_id, "condition", &node);
                if cond {
                    self.resolve_bool_in(node_id, "value_a", &node)
                } else {
                    self.resolve_bool_in(node_id, "value_b", &node)
                }
            }

            // ── ECS: HasTag ────────────────────────────────────────────────────
            NodeKind::HasTag => {
                let tag = node.string_a.as_deref().unwrap_or("tag").to_string();
                let key = format!("__tag__{tag}");
                matches!(self.blackboard.vars.get(&key), Some(PieValue::Bool(true)))
            }

            _ => false,
        }
    }
}
