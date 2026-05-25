//! Graf topologiyasi — exec (shoxlanishli) va data simlari.

use std::collections::HashMap;

use crate::nodes::VisualNode;
use crate::pins::DataLink;

/// Exec oqimi (nomli portlar: `exec_out`, `true`, `false`, …).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecLink {
    pub from_node_id: usize,
    pub from_pin: &'static str,
    pub to_node_id: usize,
    pub to_pin: &'static str,
}

impl ExecLink {
    pub fn exec_out(from: usize, to: usize) -> Self {
        Self { from_node_id: from, from_pin: "exec_out", to_node_id: to, to_pin: "exec_in" }
    }
    pub fn branch_true(from: usize, to: usize) -> Self {
        Self { from_node_id: from, from_pin: "true", to_node_id: to, to_pin: "exec_in" }
    }
    pub fn branch_false(from: usize, to: usize) -> Self {
        Self { from_node_id: from, from_pin: "false", to_node_id: to, to_pin: "exec_in" }
    }
}

/// To'liq vizual skript grafi.
#[derive(Clone, Debug, Default)]
pub struct VisualScriptGraph {
    pub name: String,
    pub nodes: Vec<(usize, VisualNode)>,
    pub exec_links: Vec<ExecLink>,
    pub data_links: Vec<DataLink>,

    // ── Tezlik indekslari — build_indices() da quriladi ──────────────────────
    /// node_id → `nodes` Vec indeksi — O(1) node qidirish.
    node_index: HashMap<usize, usize>,
    /// from_node_id → [(from_pin, to_node_id)] — exec adjacency list.
    exec_adj: HashMap<usize, Vec<(&'static str, usize)>>,
}

impl VisualScriptGraph {
    /// Yangi graf yaratadi (indekslar opsiyonal — `build_indices()` kerak).
    pub fn new(
        name: impl Into<String>,
        nodes: Vec<(usize, VisualNode)>,
        exec_links: Vec<ExecLink>,
        data_links: Vec<DataLink>,
    ) -> Self {
        Self {
            name: name.into(),
            nodes,
            exec_links,
            data_links,
            node_index: HashMap::new(),
            exec_adj: HashMap::new(),
        }
    }

    /// Tezlik indekslarini qurib chiqadi.
    /// `to_blueprint_graph()` va demo graflar oxirida chaqirilishi kerak.
    pub fn build_indices(&mut self) {
        self.node_index.clear();
        for (i, (id, _)) in self.nodes.iter().enumerate() {
            self.node_index.insert(*id, i);
        }
        self.exec_adj.clear();
        for link in &self.exec_links {
            self.exec_adj
                .entry(link.from_node_id)
                .or_default()
                .push((link.from_pin, link.to_node_id));
        }
    }

    /// O(1) node qidirish (indeks qurilgan bo'lsa), fallback O(n).
    pub fn node(&self, id: usize) -> Option<&VisualNode> {
        if let Some(&idx) = self.node_index.get(&id) {
            return self.nodes.get(idx).map(|(_, n)| n);
        }
        self.nodes.iter().find(|(nid, _)| *nid == id).map(|(_, n)| n)
    }

    /// Exec chiqishlari — indeks bo'lsa O(out_degree), aks holda O(n).
    pub fn exec_successors(&self, from: usize, from_pin: &str) -> Vec<usize> {
        if let Some(edges) = self.exec_adj.get(&from) {
            return edges.iter()
                .filter(|(pin, _)| *pin == from_pin)
                .map(|(_, to)| *to)
                .collect();
        }
        self.exec_links
            .iter()
            .filter(|l| l.from_node_id == from && l.from_pin == from_pin)
            .map(|l| l.to_node_id)
            .collect()
    }

    pub fn exec_outgoing(&self, from: usize) -> Option<usize> {
        self.exec_successors(from, "exec_out").into_iter().next()
    }

    pub fn begin_play_entry(&self) -> Result<usize, String> {
        self.nodes
            .iter()
            .find(|(_, n)| n.is_begin_play())
            .map(|(id, _)| *id)
            .ok_or_else(|| "EventBeginPlay yo'q".to_string())
    }

    pub fn tick_entry(&self) -> Result<usize, String> {
        self.nodes
            .iter()
            .find(|(_, n)| n.is_tick())
            .map(|(id, _)| *id)
            .ok_or_else(|| "EventTick yo'q".to_string())
    }

    pub fn needs_gold_query(&self) -> bool {
        self.nodes.iter().any(|(_, n)| n.needs_gold())
    }

    pub fn nodes(&self) -> impl Iterator<Item = (usize, &VisualNode)> {
        self.nodes.iter().map(|(id, n)| (*id, n))
    }
}
