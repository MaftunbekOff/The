//! PIE session resources.

use bevy::prelude::*;
use twelfth_visual_blueprint::ast::VisualScriptGraph;

/// Script o'zgaruvchilarni (variables) saqlash.
/// PIE sessiyasi davomida saqlanadi, Stop da tozalanadi.
#[derive(Resource, Default, Debug)]
pub struct BlackboardResource {
    pub(crate) vars: std::collections::HashMap<String, super::value::PieValue>,
}

impl BlackboardResource {
    pub fn clear(&mut self) {
        self.vars.clear();
    }
}

/// PIE ishlayotganda xatoli bo'lsa, bu resurda xato node_id va xabar saqlanadi.
/// UIni boshqa tizim o'qib, tegishli node ustiga qizil belgi chizadi.
#[derive(Resource, Default, Debug, Clone)]
pub struct PieRuntimeError {
    /// Xato chiqargan node ID (graph'dagi `NodeId.0`).
    pub failing_node_id: Option<u32>,
    /// Inson o'qishi uchun xato xabari.
    pub message: Option<String>,
}

impl PieRuntimeError {
    pub fn set(&mut self, node_id: Option<u32>, msg: impl Into<String>) {
        self.failing_node_id = node_id;
        self.message = Some(msg.into());
    }
    pub fn clear(&mut self) {
        self.failing_node_id = None;
        self.message = None;
    }
}
#[derive(Component, Debug)]
pub struct PieGoldHolder;

/// Har bir entity uchun alohida script holati (per-entity blackboard).
///
/// Bu komponent quyidagi muammoni hal qiladi:
/// `BlackboardResource` global — 1000 NPC bilan aralashadi.
/// `ScriptActor` esa har bir entity'ga O'Z o'zgaruvchilarini beradi.
///
/// ## Ishlatish (game developer tomonidan)
/// ```rust
/// commands.spawn((
///     ScriptActor {
///         vars: default(),
///         self_slot: entity_table.alloc_slot(),
///         active: true,
///     },
///     Transform::default(),
/// ));
/// ```
#[derive(Component, Default, Debug, Clone)]
pub struct ScriptActor {
    /// Per-entity o'zgaruvchilar — BlackboardResource bilan aralashmaydi.
    pub vars: std::collections::HashMap<String, super::value::PieValue>,
    /// Bu entity-ning `PieEntityTable` dagi slot ID.
    pub self_slot: u32,
    /// `false` bo'lsa tizim bu entity uchun scriptni o'tkazib yuboradi.
    pub active: bool,
}
#[derive(Component, Debug)]
pub struct PieEntityHolder {
    /// Slot ID — `PieEntityTable` da `entity` slotiga mos.
    pub slot: u32,
}

/// Slot ID → Bevy Entity mapping.
/// PIE davomida entity nomlari va IDlari shu yerda saqlanadi.
#[derive(Resource, Default, Debug)]
pub struct PieEntityTable {
    /// slot_id → Entity
    pub entities: std::collections::HashMap<u32, Entity>,
    /// entity nomi (masalan "player") → slot_id
    pub named: std::collections::HashMap<String, u32>,
    pub next_slot: u32,
}

impl PieEntityTable {
    #[allow(dead_code)]
    pub fn alloc_slot(&mut self) -> u32 {
        let s = self.next_slot;
        self.next_slot += 1;
        s
    }

    #[allow(dead_code)]
    pub fn register_named(&mut self, name: &str, slot: u32) {
        self.named.insert(name.to_string(), slot);
    }

    pub fn slot_for_name(&self, name: &str) -> Option<u32> {
        self.named.get(name).copied()
    }

    pub fn name_for_slot(&self, slot: u32) -> Option<&str> {
        self.named.iter().find_map(|(k, &v)| if v == slot { Some(k.as_str()) } else { None })
    }

    pub fn slot_by_index(&self, index: usize) -> Option<u32> {
        let mut slots: Vec<u32> = self.entities.keys().copied().collect();
        slots.sort_unstable();
        slots.get(index).copied()
    }

    #[allow(dead_code)]
    pub fn entity_for_slot(&self, slot: u32) -> Option<Entity> {
        self.entities.get(&slot).copied()
    }
}

/// PIE kontekstida entity uchun transform o'qish/yozish buyruqlari.
pub enum PieTransformOp {
    /// Translation ni yangi qiymatga o'rnatadi.
    SetTranslation { slot: u32, pos: [f32; 3] },
    /// Translation ga delta qo'shadi.
    Translate { slot: u32, delta: [f32; 3] },
    /// Scale ni o'rnatadi.
    SetScale { slot: u32, scale: [f32; 3] },
    /// Euler rotation ni o'rnatadi (radians).
    SetRotationEuler { slot: u32, euler: [f32; 3] },
}

/// Entity spawn so'rovi — exec.rs dan systems.rs ga o'tkaziladi.
pub struct PieEntitySpawnRequest {
    #[allow(dead_code)]
    /// Qaysi node ID (SpawnEntity.exec_out da slot data pin ni to'ldirish uchun).
    pub node_id: usize,
    /// Ajratilgan slot ID.
    pub slot: u32,
    /// Ixtiyoriy entity nomi — spawn dan keyin `named` da ro'yxatga olinadi.
    pub name: Option<String>,
}


#[derive(Resource, Default, Debug)]
pub struct PieSession {
    pub active: bool,
    pub tick_graph: Option<VisualScriptGraph>,
}

#[derive(Resource, Default, Debug)]
pub struct PiePendingGraph(pub Option<VisualScriptGraph>);

#[derive(Resource, Default, Debug)]
pub struct PieStartRequested(pub bool);

// ── Delay queue ───────────────────────────────────────────────────────────────

/// Qolgan vaqti bor exec zanjirlari (Delay tuguni tomonidan qo'shiladi).
pub struct PieDelayEntry {
    /// Necha sekund qolgani.
    pub remaining: f32,
    /// Delay dan keyingi exec node ID.
    pub resume_node_id: usize,
    /// Ushbu exec zanjiri uchun grafik — `Arc` bilan uchinchi klonlash yo'q.
    pub graph: std::sync::Arc<VisualScriptGraph>,
}

#[derive(Resource, Default)]
pub struct PieDelayQueue {
    pub entries: Vec<PieDelayEntry>,
}
