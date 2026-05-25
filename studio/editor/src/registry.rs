//! Blueprint tugunlar ro'yxati — palette va portlar (newtype).
//!
//! Barcha node metadata (`label`, `description`, `width`, `height`, `default_node`)
//! endi `editor/src/nodes/` kategoriya fayllarida.
//! Bu fayl faqat UI qatlamlari uchun newtype va pin yordam funksiyalarini o'z ichiga oladi.

use twelfth_visual_blueprint::nodes::{NodeKind as BpNodeKind, VisualNode};
use twelfth_visual_blueprint::pins::{PinSpec, PinType};

use crate::nodes::NodeRegistry;

// ── Newtype wrapper ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeKind(pub BpNodeKind);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum WireCategory {
    Exec,
    Data,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PinRef {
    pub name: &'static str,
    pub category: WireCategory,
    pub ty: PinType,
}

// ── NodeKind: all metadata delegated to NodeRegistry ─────────────────────────

impl NodeKind {
    /// Palette uchun barcha node turlari — NodeRegistry dan olinadi.
    pub fn palette() -> impl Iterator<Item = NodeKind> {
        NodeRegistry::global()
            .palette()
            .iter()
            .map(|d| NodeKind(d.kind))
    }

    /// UI da ko'rsatiladigan nom.
    pub fn label(self) -> &'static str {
        NodeRegistry::global()
            .get(self.0)
            .map(|d| d.label)
            .unwrap_or("???")
    }

    /// Palette tooltipi.
    pub fn description(self) -> &'static str {
        NodeRegistry::global()
            .get(self.0)
            .map(|d| d.description)
            .unwrap_or("")
    }

    /// Boshlang'ich `VisualNode`.
    pub fn default_node(self) -> VisualNode {
        NodeRegistry::global()
            .get(self.0)
            .map(|d| (d.default_node)())
            .unwrap_or_else(|| VisualNode::print_log("???"))
    }

    /// UI kengligi (px).
    pub fn visual_width(self) -> f32 {
        NodeRegistry::global()
            .get(self.0)
            .map(|d| d.width)
            .unwrap_or(180.0)
    }

    /// UI balandligi (px).
    pub fn visual_height(self) -> f32 {
        NodeRegistry::global()
            .get(self.0)
            .map(|d| d.height)
            .unwrap_or(100.0)
    }

    // ── Pin helpers ───────────────────────────────────────────────────────────

    pub fn pins(self) -> Vec<PinRef> {
        let n = self.default_node();
        let mut out = Vec::new();
        for p in n.exec_inputs()  { out.push(pin(p, WireCategory::Exec)); }
        for p in n.exec_outputs() { out.push(pin(p, WireCategory::Exec)); }
        for p in n.data_inputs()  { out.push(pin(p, WireCategory::Data)); }
        for p in n.data_outputs() { out.push(pin(p, WireCategory::Data)); }
        out
    }

    pub fn is_exec_output(self, pin: &str) -> bool {
        self.default_node().exec_outputs().iter().any(|p| p.name == pin)
    }

    pub fn is_exec_input(self, pin: &str) -> bool {
        self.default_node().exec_inputs().iter().any(|p| p.name == pin)
    }

    pub fn is_data_output(self, pin: &str) -> bool {
        self.default_node().data_outputs().iter().any(|p| p.name == pin)
    }

    pub fn is_data_input(self, pin: &str) -> bool {
        self.default_node().data_inputs().iter().any(|p| p.name == pin)
    }
}

fn pin(p: &PinSpec, category: WireCategory) -> PinRef {
    PinRef { name: p.name, category, ty: p.ty }
}
