//! Editor holati — VM o'rniga eksport / terminal.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use crate::graph::{GraphResource, NodeId};

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayState {
    #[default]
    Idle,
    /// Play-in-Editor ishlayapti.
    Playing,
    Exported,
}

/// Editor rejimi — Script (node graf) yoki Scene (viewport + hierarchy).
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    #[default]
    Script,
    Scene,
}

/// Viewport kamera rejimi — 2D yoki 3D.
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewportCameraMode {
    #[default]
    TwoD,
    ThreeD,
}


/// bitta tugunli amallar (disconnect tugmasi va h.k.) uchun ishlatiladi.
#[derive(Resource, Default, Debug, Clone)]
pub struct SelectedNode {
    pub nodes: HashSet<NodeId>,
    pub primary: Option<NodeId>,
}

impl SelectedNode {
    pub fn is_selected(&self, id: NodeId) -> bool {
        self.nodes.contains(&id)
    }
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.primary = None;
    }
    /// Faqat bitta tugunni tanlash (oldingi tanlovni o'chiradi).
    pub fn select_only(&mut self, id: NodeId) {
        self.nodes.clear();
        self.nodes.insert(id);
        self.primary = Some(id);
    }
    /// Shift+klik uchun: borsa o'chir, yo'qsa qo'sh.
    pub fn toggle(&mut self, id: NodeId) {
        if !self.nodes.remove(&id) {
            self.nodes.insert(id);
            self.primary = Some(id);
        } else if self.primary == Some(id) {
            self.primary = self.nodes.iter().next().copied();
        }
    }
    /// Rubber-band yoki Alt-duplicate keyin ko'p qo'shish.
    pub fn add_to_selection(&mut self, id: NodeId) {
        self.nodes.insert(id);
        self.primary = Some(id);
    }
    pub fn iter(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.nodes.iter().copied()
    }
}

/// Kauchuk-lenta (rubber-band) tanlov holati.
#[derive(Resource, Default, Debug)]
pub struct SelectionBoxState {
    pub active: bool,
    /// Sichqoncha bosilgan nuqta — graph CSS fazosida.
    pub start: Vec2,
    /// Joriy kursor pozitsiyasi — graph CSS fazosida.
    pub current: Vec2,
}

impl SelectionBoxState {
    pub fn rect_min(&self) -> Vec2 {
        self.start.min(self.current)
    }
    pub fn rect_max(&self) -> Vec2 {
        self.start.max(self.current)
    }
}

#[derive(Resource, Default, Debug)]
pub struct TerminalState {
    pub lines: Vec<String>,
}

impl TerminalState {
    pub fn log(&mut self, line: impl Into<String>) {
        let line = line.into();
        info!("{line}");
        self.lines.push(line);
        if self.lines.len() > 300 {
            let overflow = self.lines.len() - 300;
            self.lines.drain(0..overflow);
        }
    }

    pub fn text(&self) -> String {
        self.lines.join("\n")
    }
}

/// Sim tortish: exec yoki data portidan; kategoriya mos kelishi shart.
#[derive(Resource, Default, Debug)]
pub struct ConnectingState {
    pub from: Option<(NodeId, &'static str, crate::registry::WireCategory)>,
    pub to: Option<(NodeId, &'static str, crate::registry::WireCategory)>,
    /// Kanvas graph fazosidagi kursor (snap qilingan bo'lishi mumkin).
    pub cursor_canvas: Option<Vec2>,
    pub anchor_canvas: Option<Vec2>,
    /// Sim boshlangan paytdagi ekran pozitsiyasi (graph delta uchun).
    pub press_screen: Option<Vec2>,
    /// Magnetic snap — ulanish tugaganda ishlatiladi.
    pub snap_target: Option<(NodeId, &'static str)>,
}

impl ConnectingState {
    pub fn is_active(&self) -> bool {
        self.from.is_some() || self.to.is_some()
    }

    pub fn clear(&mut self) {
        self.from = None;
        self.to = None;
        self.cursor_canvas = None;
        self.anchor_canvas = None;
        self.press_screen = None;
        self.snap_target = None;
    }
}

/// Ko'p-tugun drag holati. `origins` — drag boshlanganidagi barcha
/// tanlangan tugunlar pozitsiyasi.
#[derive(Resource, Default, Debug)]
pub struct ActiveNodeDrag {
    /// Asosiy sudralayotgan tugun (drag event qaysi tugundan boshlandi).
    pub node: Option<NodeId>,
    /// Barcha sudranadigan tugunlar uchun boshlang'ich pozitsiyalar.
    pub origins: HashMap<NodeId, Vec2>,
}

// ── Undo / Redo ───────────────────────────────────────────────────────────────

/// Graf holatini qaytarish / qaytamaslik uchun stek.
/// Har bir snapshot `GraphResource` ning to'liq kloni.
#[derive(Resource, Default)]
pub struct UndoStack {
    undo: Vec<GraphResource>,
    redo: Vec<GraphResource>,
}

impl UndoStack {
    /// Mutatsiyadan OLDIN chaqiriladi: joriy holatni undo stekiga qo'shadi.
    pub fn push(&mut self, snapshot: GraphResource) {
        self.undo.push(snapshot);
        self.redo.clear();
        if self.undo.len() > 128 {
            self.undo.remove(0);
        }
    }

    /// Ctrl+Z — oxirgi holatni qaytaradi. `current` joriy holatni qaytaradi.
    pub fn undo(&mut self, current: GraphResource) -> Option<GraphResource> {
        let prev = self.undo.pop()?;
        self.redo.push(current);
        Some(prev)
    }

    /// Ctrl+Y — qaytarilgan holatni redo qiladi.
    pub fn redo(&mut self, current: GraphResource) -> Option<GraphResource> {
        let next = self.redo.pop()?;
        self.undo.push(current);
        Some(next)
    }
}

// ── Palette search ────────────────────────────────────────────────────────────

/// Paletda qidiruv filtri — kichik-katta harf farq qilmaydi.
#[derive(Resource, Default, Debug, Clone)]
pub struct PaletteFilter {
    pub query: String,
}

// ── Load graph request ────────────────────────────────────────────────────────

/// Load button bosilganda RON dan o'qilgan yangi graf.
/// `load_graph_system` uni ko'rib, ECS ni yangilaydi.
#[derive(Resource, Default)]
pub struct LoadGraphRequest(pub Option<GraphResource>);

// ── Multiple graphs (tabs) ────────────────────────────────────────────────────

/// Nofaol (background) graflar ombori.
/// Faol graf har doim `GraphResource`-da saqlanadi.
#[derive(Resource, Default, Debug, Clone)]
pub struct SavedGraphs {
    /// (nom, grafik) juftlari — faol grafdagi emas!
    pub graphs: Vec<(String, GraphResource)>,
    /// Faol grafning nomi.
    pub active_name: String,
}

impl SavedGraphs {
    /// Yangi bo'sh graf tab qo'shadi, faolni eski nomda saqlaydi.
    pub fn new_tab(
        &mut self,
        current_name: &str,
        current_graph: GraphResource,
        new_name: impl Into<String>,
    ) -> GraphResource {
        self.push_current(current_name, current_graph);
        self.active_name = new_name.into();
        GraphResource::default()
    }

    /// Joriy grafni saqlaydi va boshqa tabga o'tadi.
    /// Eski faol grafni qaytaradi (eski nom bilan).
    pub fn switch_to(
        &mut self,
        current_name: &str,
        current_graph: GraphResource,
        target_name: &str,
    ) -> Option<(GraphResource, String)> {
        // Faqat boshqa tab bo'lsa o'tamiz
        if current_name == target_name { return None; }

        // Joriyni saqlash
        self.push_current(current_name, current_graph);

        // Maqsadni chiqarish
        if let Some(idx) = self.graphs.iter().position(|(n, _)| n == target_name) {
            let (name, graph) = self.graphs.remove(idx);
            self.active_name = name.clone();
            Some((graph, name))
        } else {
            None
        }
    }

    fn push_current(&mut self, name: &str, graph: GraphResource) {
        // Avvalgi yozuvni yangilash yoki yangi qo'shish
        if let Some(slot) = self.graphs.iter().position(|(n, _)| n == name) {
            self.graphs[slot].1 = graph;
        } else {
            self.graphs.push((name.to_string(), graph));
        }
    }

    /// Barcha tab nomlarini qaytaradi (faol + nofaol).
    pub fn all_tab_names<'a>(&'a self, active_name: &'a str) -> Vec<&'a str> {
        let mut names: Vec<&str> = self.graphs.iter().map(|(n, _)| n.as_str()).collect();
        names.push(active_name);
        names.sort();
        names
    }
}
