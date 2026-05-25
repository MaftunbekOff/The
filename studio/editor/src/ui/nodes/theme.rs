//! Node colors and styled node markers.

use bevy::prelude::*;

pub const BG_NODE: Color = Color::srgb(0.18, 0.2, 0.26);
pub const BG_NODE_SELECTED: Color = Color::srgb(0.22, 0.28, 0.38);
pub const PORT_EXEC: Color = Color::srgb(0.3, 0.85, 0.95);
pub const PORT_DATA: Color = Color::srgb(0.9, 0.75, 0.2);
pub const WIRE_COLOR: Color = Color::srgb(0.92, 0.94, 0.98);
pub const WIRE_DATA_COLOR: Color = Color::srgb(0.95, 0.82, 0.35);
pub const WIRE_PREVIEW: Color = Color::srgba(0.92, 0.94, 0.98, 0.75);

/// UE uslubida chizilgan tugun (tanlash vizuali uchun).
#[derive(Component, Debug, Clone, Copy)]
pub struct StyledBlueprintNode(pub StyledNodeKind);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyledNodeKind {
    EventBeginPlay,
    EventTick,
    Branch,
    PrintLog,
    Delay,
    IsKeyPressed,
    IsKeyJustPressed,
    /// Set*Var nodes — purple
    VarSet,
    /// Get*Var nodes — lighter purple
    VarGet,
    /// FloatArray* exec/pure nodes — teal
    Collection,
    /// ForEachFloat, WhileLoop loop nodes — amber
    LoopFor,
    /// Sequence, DoOnce — grey-blue
    FlowCtrl,
    /// EventCustomBegin / FireCustomEvent — magenta
    CustomEvent,
    /// SpawnEntity, GetNamedEntity, DestroyEntity + transforms — blue
    EntityOp,
    /// Vec2 math nodes — cyan-green
    Vec2Math,
    /// Comment annotation node — dark transparent
    Comment,
    /// ECS World/Query nodes — green
    #[allow(dead_code)]
    EcsWorld,
    /// ECS Component/Tag nodes — teal-green
    #[allow(dead_code)]
    EcsComponent,
    /// ECS Schedule/Event nodes — deep purple
    #[allow(dead_code)]
    EcsSchedule,
}

// ── Header background colors ──────────────────────────────────────────────────
pub const EVENT_HEADER_BG:     Color = Color::srgb(0.52, 0.12, 0.78);
pub const FLOW_HEADER_BG:      Color = Color::srgb(0.16, 0.19, 0.27);
pub const PRINT_HEADER_BG:     Color = Color::srgb(0.06, 0.52, 0.50);
pub const DELAY_HEADER_BG:     Color = Color::srgb(0.90, 0.42, 0.08);
pub const INPUT_HEADER_BG:     Color = Color::srgb(0.08, 0.62, 0.28);
pub const VAR_SET_HEADER_BG:   Color = Color::srgb(0.45, 0.18, 0.72);
pub const VAR_GET_HEADER_BG:   Color = Color::srgb(0.32, 0.14, 0.52);
pub const COLLECTION_HEADER_BG: Color = Color::srgb(0.06, 0.48, 0.55);
pub const LOOP_HEADER_BG:      Color = Color::srgb(0.72, 0.42, 0.06);
pub const FLOW_CTRL_HEADER_BG: Color = Color::srgb(0.18, 0.28, 0.45);
pub const CUSTOM_EVENT_HEADER_BG: Color = Color::srgb(0.62, 0.12, 0.48);
pub const ENTITY_OP_HEADER_BG: Color = Color::srgb(0.08, 0.32, 0.68);
/// Vec2 math — cyan-yeşil.
pub const VEC2_HEADER_BG:      Color = Color::srgb(0.08, 0.58, 0.42);
/// Comment node — koyu transparan.
pub const COMMENT_HEADER_BG:   Color = Color::srgb(0.22, 0.20, 0.18);

// ── Border colors ─────────────────────────────────────────────────────────────
pub(crate) const EVENT_BORDER: Color = Color::srgb(0.28, 0.1, 0.42);
pub(crate) const FLOW_BORDER: Color = Color::srgb(0.22, 0.26, 0.36);
pub(crate) const PRINT_BORDER: Color = Color::srgb(0.08, 0.38, 0.36);
pub(crate) const DELAY_BORDER: Color = Color::srgb(0.55, 0.28, 0.06);
pub(crate) const INPUT_BORDER: Color = Color::srgb(0.06, 0.38, 0.18);
pub(crate) const VAR_SET_BORDER: Color = Color::srgb(0.28, 0.12, 0.45);
pub(crate) const VAR_GET_BORDER: Color = Color::srgb(0.22, 0.10, 0.34);
pub(crate) const COLLECTION_BORDER: Color = Color::srgb(0.05, 0.30, 0.35);
pub(crate) const LOOP_BORDER: Color = Color::srgb(0.45, 0.26, 0.05);
pub(crate) const FLOW_CTRL_BORDER: Color = Color::srgb(0.14, 0.20, 0.32);
pub(crate) const CUSTOM_EVENT_BORDER: Color = Color::srgb(0.40, 0.08, 0.30);
pub(crate) const ENTITY_OP_BORDER: Color = Color::srgb(0.06, 0.22, 0.45);
pub(crate) const VEC2_BORDER:      Color = Color::srgb(0.06, 0.36, 0.28);
pub(crate) const COMMENT_BORDER:   Color = Color::srgb(0.30, 0.28, 0.24);
/// ECS World nodes — zeleno-tamno
#[allow(dead_code)]
pub const ECS_WORLD_HEADER_BG:     Color = Color::srgb(0.10, 0.42, 0.18);
pub(crate) const ECS_WORLD_BORDER: Color = Color::srgb(0.07, 0.28, 0.12);
/// ECS Component/Tag nodes
#[allow(dead_code)]
pub const ECS_COMP_HEADER_BG:      Color = Color::srgb(0.08, 0.48, 0.38);
pub(crate) const ECS_COMP_BORDER:  Color = Color::srgb(0.05, 0.32, 0.26);
/// ECS Schedule/Event nodes
#[allow(dead_code)]
pub const ECS_SCHED_HEADER_BG:     Color = Color::srgb(0.28, 0.08, 0.55);
pub(crate) const ECS_SCHED_BORDER: Color = Color::srgb(0.18, 0.06, 0.36);

// ── Pin and label colors ──────────────────────────────────────────────────────
pub const EVENT_BODY_BG: Color = Color::srgb(0.07, 0.07, 0.09);
pub(crate) const EXEC_PIN_DOT: Color = Color::srgb(0.96, 0.97, 1.0);
pub(crate) const EXEC_PIN_GLOW: Color = Color::srgba(0.92, 0.88, 1.0, 0.55);
pub(crate) const DATA_PIN_RING: Color = Color::srgb(0.95, 0.82, 0.28);
pub(crate) const DATA_PIN_TEAL: Color = Color::srgb(0.35, 0.82, 0.78);
pub(crate) const DATA_PIN_BOOL: Color = Color::srgb(0.92, 0.42, 0.38);
/// Vec3 pin rengi — moviy-ko'k.
pub(crate) const DATA_PIN_VEC3: Color = Color::srgb(0.28, 0.72, 0.95);
/// Vec2 pin rengi — cyan-yashil.
pub(crate) const DATA_PIN_VEC2: Color = Color::srgb(0.18, 0.88, 0.62);
pub(crate) const LABEL_MUTED: Color = Color::srgb(0.62, 0.64, 0.7);
pub(crate) const LABEL_MESSAGE: Color = Color::srgb(0.55, 0.88, 0.90);
pub(crate) const LABEL_CONDITION: Color = Color::srgb(0.95, 0.58, 0.52);
pub(crate) const LABEL_DATA_OUT: Color = Color::srgb(0.95, 0.82, 0.35);
pub(crate) const LABEL_INPUT_KEY: Color = Color::srgb(0.45, 0.92, 0.62);

// ── Node size constants ───────────────────────────────────────────────────────
pub(crate) const NODE_WIDTH: f32 = 168.0;
pub(crate) const EVENT_NODE_WIDTH: f32 = 204.0;
pub(crate) const BRANCH_NODE_WIDTH: f32 = 220.0;
pub(crate) const FLOW_NODE_WIDTH: f32 = 228.0;
pub(crate) const INPUT_NODE_WIDTH: f32 = 220.0;
