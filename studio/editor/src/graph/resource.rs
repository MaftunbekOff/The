//! Canvas graph resource — CRUD, wires, snapshots.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::registry::{NodeKind, WireCategory};

use super::types::{GraphNodeData, GraphWire, NodeId};

#[derive(Resource, Debug, Clone)]
pub struct GraphResource {
    pub nodes: Vec<GraphNodeData>,
    pub exec_wires: Vec<GraphWire>,
    pub data_wires: Vec<GraphWire>,
    pub next_id: u32,
    /// O(1) NodeId → Vec indeks — barcha mutatsiyalarda yangilanadi.
    pub(crate) by_id: HashMap<u32, usize>,
}

impl Default for GraphResource {
    fn default() -> Self {
        Self::with_starter_graph()
    }
}

impl GraphResource {
    fn alloc_id(&mut self) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;
        id
    }

    /// `by_id` indeksini to'liq qayta quradi (restore keyin chaqiriladi).
    pub(crate) fn rebuild_index(&mut self) {
        self.by_id.clear();
        for (i, n) in self.nodes.iter().enumerate() {
            self.by_id.insert(n.id.0, i);
        }
    }

    /// Undo/Redo uchun to'liq snapshot (klon).
    pub fn snapshot(&self) -> Self {
        self.clone()
    }

    /// Snapshot dan tiklash (Undo/Redo).
    pub fn restore(&mut self, snap: Self) {
        *self = snap;
        self.rebuild_index();
    }

    pub fn push_node(&mut self, kind: NodeKind, position: Vec2) -> NodeId {
        let id = self.alloc_id();
        let idx = self.nodes.len();
        self.nodes.push(GraphNodeData {
            id,
            kind,
            position,
            params: kind.default_node(),
        });
        self.by_id.insert(id.0, idx);
        id
    }

    pub fn add_node_at(&mut self, kind: NodeKind, position: Vec2) -> NodeId {
        self.push_node(kind, position)
    }

    pub fn node(&self, id: NodeId) -> Option<&GraphNodeData> {
        if let Some(&idx) = self.by_id.get(&id.0) {
            return self.nodes.get(idx);
        }
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut GraphNodeData> {
        if let Some(&idx) = self.by_id.get(&id.0) {
            return self.nodes.get_mut(idx);
        }
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    pub fn remove_node(&mut self, id: NodeId) -> bool {
        let before = self.nodes.len();
        self.nodes.retain(|n| n.id != id);
        self.exec_wires
            .retain(|w| w.from_node != id && w.to_node != id);
        self.data_wires
            .retain(|w| w.from_node != id && w.to_node != id);
        let removed = self.nodes.len() < before;
        if removed {
            self.rebuild_index();
        }
        removed
    }

    pub fn set_position(&mut self, id: NodeId, position: Vec2) {
        if let Some(n) = self.node_mut(id) {
            n.position = position;
        }
    }

    pub fn connect_exec(
        &mut self,
        from: NodeId,
        from_pin: &'static str,
        to: NodeId,
        to_pin: &'static str,
    ) -> Result<(), String> {
        self.connect_wire(from, from_pin, to, to_pin, WireCategory::Exec)
    }

    pub fn connect_data(
        &mut self,
        from: NodeId,
        from_pin: &'static str,
        to: NodeId,
        to_pin: &'static str,
    ) -> Result<(), String> {
        self.connect_wire(from, from_pin, to, to_pin, WireCategory::Data)
    }

    fn connect_wire(
        &mut self,
        from: NodeId,
        from_pin: &'static str,
        to: NodeId,
        to_pin: &'static str,
        category: WireCategory,
    ) -> Result<(), String> {
        if from == to {
            return Err("o'ziga bog'lab bo'lmaydi".into());
        }
        let Some(from_n) = self.node(from) else {
            return Err("chiqish tuguni topilmadi".into());
        };
        let Some(to_n) = self.node(to) else {
            return Err("kirish tuguni topilmadi".into());
        };
        match category {
            WireCategory::Exec => {
                if !from_n.kind.is_exec_output(from_pin) {
                    return Err("exec chiqish porti yo'q".into());
                }
                if !to_n.kind.is_exec_input(to_pin) {
                    return Err("exec kirish porti yo'q".into());
                }
                if self
                    .exec_wires
                    .iter()
                    .any(|w| w.to_node == to && w.to_pin == to_pin)
                {
                    return Err("exec kirish band".into());
                }
                // Har bir exec chiqish pini faqat bitta simga ega bo'lishi mumkin.
                if self
                    .exec_wires
                    .iter()
                    .any(|w| w.from_node == from && w.from_pin == from_pin)
                {
                    return Err("exec chiqish band".into());
                }
                self.exec_wires.push(GraphWire {
                    from_node: from,
                    from_pin,
                    to_node: to,
                    to_pin,
                    category,
                });
            }
            WireCategory::Data => {
                if !from_n.kind.is_data_output(from_pin) {
                    return Err("data chiqish porti yo'q".into());
                }
                if !to_n.kind.is_data_input(to_pin) {
                    return Err("data kirish porti yo'q".into());
                }
                let from_ty = from_n
                    .params
                    .data_output_type(from_pin)
                    .ok_or_else(|| "data chiqish tipi yo'q".to_string())?;
                let to_ty = to_n
                    .params
                    .data_input_type(to_pin)
                    .ok_or_else(|| "data kirish tipi yo'q".to_string())?;
                twelfth_visual_blueprint::validate_data_link(from_ty, to_ty, "kanvas data simi")
                    .map_err(|e| e.message)?;
                if self
                    .data_wires
                    .iter()
                    .any(|w| w.to_node == to && w.to_pin == to_pin)
                {
                    return Err("data kirish band".into());
                }
                self.data_wires.push(GraphWire {
                    from_node: from,
                    from_pin,
                    to_node: to,
                    to_pin,
                    category,
                });
            }
        }
        Ok(())
    }

    /// Tugunni nusxalaydi: bir xil `kind` va `params`, `offset` bilan siljitilgan.
    /// Yangi tugunning `NodeId` sini qaytaradi.
    pub fn duplicate_node(&mut self, id: NodeId, offset: Vec2) -> Option<NodeId> {
        let data = self.node(id)?.clone();
        let new_id = self.push_node(data.kind, data.position + offset);
        if let Some(n) = self.node_mut(new_id) {
            n.params = data.params;
        }
        Some(new_id)
    }

    pub fn disconnect_exec_from(&mut self, from: NodeId, from_pin: &'static str) -> bool {
        let len = self.exec_wires.len();
        self.exec_wires
            .retain(|w| !(w.from_node == from && w.from_pin == from_pin));
        self.exec_wires.len() < len
    }

    /// Faqat `node` ning `pin` portiga ulangan barcha simlarni uzadi
    /// (exec va data, chiqish yoki kirish — ikkalasini ham tekshiradi).
    pub fn disconnect_all_from_pin(&mut self, node: NodeId, pin: &str) -> usize {
        let e = self.exec_wires.len();
        let d = self.data_wires.len();
        self.exec_wires.retain(|w| {
            !((w.from_node == node && w.from_pin == pin)
                || (w.to_node == node && w.to_pin == pin))
        });
        self.data_wires.retain(|w| {
            !((w.from_node == node && w.from_pin == pin)
                || (w.to_node == node && w.to_pin == pin))
        });
        (e - self.exec_wires.len()) + (d - self.data_wires.len())
    }

    pub fn disconnect_all_from_node(&mut self, id: NodeId) -> usize {
        let e = self.exec_wires.len();
        let d = self.data_wires.len();
        self.exec_wires
            .retain(|w| w.from_node != id && w.to_node != id);
        self.data_wires
            .retain(|w| w.from_node != id && w.to_node != id);
        (e - self.exec_wires.len()) + (d - self.data_wires.len())
    }

    pub fn title(&self, id: NodeId) -> &str {
        self.node(id).map(GraphNodeData::title).unwrap_or("?")
    }
}
