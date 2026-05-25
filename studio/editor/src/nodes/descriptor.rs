//! Node Descriptor — har bir node uchun bitta manba haqiqati.
//!
//! # Yangi node qo'shish
//! 1. `visual_blueprint/src/nodes.rs` — `NodeKind` enum varianti + pin specklari
//! 2. `editor/src/nodes/[kategoriya].rs` — `NodeDescriptor` + `exec_fn` (ixtiyoriy)
//!
//! `registry.rs`, `exec.rs` va boshqa fayllar TEGILMAYDI.

use std::collections::HashMap;
use std::sync::OnceLock;

use twelfth_visual_blueprint::nodes::{NodeKind as BpNodeKind, VisualNode};

use crate::interpreter::context::PieExecContext;

// ── ExecFlow ──────────────────────────────────────────────────────────────────

/// `exec_fn` bajarilgandan keyin qaysi yo'lni davom ettirish kerakligini bildiradi.
/// Rekursiv `execute_exec_node` chaqiruvlari `exec.rs` da markazlashtirilgan.
#[derive(Debug)]
pub(crate) enum ExecFlow {
    /// `exec_out` pinini davom ettir (aksariyat tugunlar)
    ExecOut,
    /// Belgilangan pinni davom ettir (Branch: "true"/"false")
    Pin(&'static str),
    /// Avval bu node IDlarni ishga tushir, keyin `exec_out` davom ettir
    /// (FireCustomEvent: avval target events, so'ng exec_out)
    ThenExecOut(Vec<usize>),
    /// Delay ro'yxatga qo'shildi, zanjir shu yerda to'xtaydi
    Deferred,
    /// Loop ichidan chiqish — ForEachFloat / WhileLoop ushlab oladi
    Break,
    /// Loop joriy iteratsiyasini o'tkazib yuborish — ForEachFloat / WhileLoop ushlab oladi
    Continue,
    /// DoOnce ni reset qilindi — faqat exec_out davom ettirish
    ResetDone,
}

// ── ExecBehavior trait ────────────────────────────────────────────────────────

/// Exec tugunning bajarilish mantig'i.
/// Har bir exec node uchun zero-sized struct impl qiladi.
///
/// Lifetime parametri `PieExecContext<'_>` da ishlatiladi — trait method da
/// HRTB muammosi yo'q.
pub(crate) trait ExecBehavior: Send + Sync {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String>;
}

// ── NodeDescriptor ────────────────────────────────────────────────────────────

/// Bitta node turining to'liq tavsifi.
/// Yangi node = yangi `NodeDescriptor` (bitta fayl).
pub(crate) struct NodeDescriptor {
    pub kind: BpNodeKind,
    pub label: &'static str,
    pub description: &'static str,
    /// Palette kategoriyasi (UI da guruhlash uchun) — kelajakda ishlatiladi
    #[allow(dead_code)]
    pub category: &'static str,
    /// UI kengligi (px)
    pub width: f32,
    /// UI balandligi (px)
    pub height: f32,
    /// Boshlang'ich `VisualNode` yasaydigan funksiya
    pub default_node: fn() -> VisualNode,
    /// `None` = sof-hisob tugun (exec zanjirida qatnashmaydi)
    pub exec: Option<&'static (dyn ExecBehavior + Sync)>,
}

// ── NodeRegistry ──────────────────────────────────────────────────────────────

pub(crate) struct NodeRegistry {
    pub(crate) descriptors: Vec<NodeDescriptor>,
    /// O(1) kind → Vec indeks — `register()` da quriladi.
    index: HashMap<BpNodeKind, usize>,
}

impl NodeRegistry {
    pub(crate) fn new() -> Self {
        Self {
            descriptors: Vec::new(),
            index: HashMap::new(),
        }
    }

    pub(crate) fn register(&mut self, desc: NodeDescriptor) {
        let idx = self.descriptors.len();
        self.index.insert(desc.kind, idx);
        self.descriptors.push(desc);
    }

    /// Global (lazy) registry — birinchi chaqiruvda quriladi.
    pub(crate) fn global() -> &'static NodeRegistry {
        static REGISTRY: OnceLock<NodeRegistry> = OnceLock::new();
        REGISTRY.get_or_init(crate::nodes::build_registry)
    }

    /// `kind` uchun descriptor qaytaradi — O(1) HashMap lookup.
    pub(crate) fn get(&self, kind: BpNodeKind) -> Option<&NodeDescriptor> {
        self.index.get(&kind).and_then(|&i| self.descriptors.get(i))
    }

    /// Palette uchun barcha descriptorlar ro'yxati.
    pub(crate) fn palette(&self) -> &[NodeDescriptor] {
        &self.descriptors
    }
}
