//! Entity resolution for PIE context.

use twelfth_visual_blueprint::nodes::NodeKind;

use super::{PieExecContext, PieValue};

impl PieExecContext<'_> {
    pub fn set_entity(&mut self, node: usize, pin: &'static str, slot: u32) {
        self.data.insert((node, pin), PieValue::Entity(slot));
    }

    pub fn resolve_entity_in(&self, node_id: usize, pin: &str) -> Option<u32> {
        if let Some(src) = self.graph.data_source(node_id, pin) {
            return self.eval_entity_out(src.node_id, src.pin);
        }
        if let Some(PieValue::Entity(s)) = self.data.get(&(node_id, pin)) {
            return Some(*s);
        }
        None
    }

    fn eval_entity_out(&self, node_id: usize, pin: &str) -> Option<u32> {
        if let Some(PieValue::Entity(s)) = self.data.get(&(node_id, pin)) {
            return Some(*s);
        }
        let node = self.graph.node(node_id)?;
        match node.kind {
            NodeKind::GetNamedEntity => {
                let name = node.entity_name.as_deref().unwrap_or("");
                self.entity_table.slot_for_name(name)
            }
            NodeKind::SpawnEntity => None,
            // ScriptActor o'zini entity sifatida qaytaradi
            // self_slot BlackboardResource dagi maxsus kalitda saqlanadi
            NodeKind::GetSelfEntity => {
                use crate::interpreter::value::PieValue;
                if let Some(PieValue::Int(slot)) = self.blackboard.vars.get("__self_slot__") {
                    Some(*slot as u32)
                } else {
                    None
                }
            }
            // ECS: entity ni indeks orqali olish
            NodeKind::EntityArrayGet => {
                let idx = self.resolve_int_in(node_id, "index", node) as usize;
                self.entity_table.slot_by_index(idx)
            }
            _ => None,
        }
    }

    pub fn get_translation_for_slot(&self, slot: u32) -> [f32; 3] {
        self.transform_snapshot.get(&slot).copied().unwrap_or([0.0; 3])
    }

    pub fn get_scale_for_slot(&self, slot: u32) -> [f32; 3] {
        self.scale_snapshot.get(&slot).copied().unwrap_or([1.0, 1.0, 1.0])
    }

    pub fn get_rotation_for_slot(&self, slot: u32) -> [f32; 3] {
        self.rotation_snapshot.get(&slot).copied().unwrap_or([0.0; 3])
    }

    pub fn resolve_vec3_entity(&self, node_id: usize, pin: &str) -> [f32; 3] {
        if let Some(src) = self.graph.data_source(node_id, pin) {
            return self.eval_vec3_out_inner(src.node_id, src.pin);
        }
        [0.0; 3]
    }

    fn eval_vec3_out_inner(&self, node_id: usize, pin: &str) -> [f32; 3] {
        if let Some(PieValue::Vec3(v)) = self.data.get(&(node_id, pin)) {
            return *v;
        }
        let node = match self.graph.node(node_id) {
            Some(n) => n,
            None    => return [0.0; 3],
        };
        match node.kind {
            NodeKind::GetTranslation => {
                if let Some(slot) = self.resolve_entity_in(node_id, "entity") {
                    return self.get_translation_for_slot(slot);
                }
                [0.0; 3]
            }
            NodeKind::GetScale => {
                if let Some(slot) = self.resolve_entity_in(node_id, "entity") {
                    return self.get_scale_for_slot(slot);
                }
                [1.0, 1.0, 1.0]
            }
            NodeKind::GetRotationEuler => {
                if let Some(slot) = self.resolve_entity_in(node_id, "entity") {
                    return self.get_rotation_for_slot(slot);
                }
                [0.0; 3]
            }
            _ => [0.0; 3],
        }
    }
}
