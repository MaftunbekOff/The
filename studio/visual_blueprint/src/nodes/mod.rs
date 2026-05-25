//! Visual scripting node types — data model only, no runtime logic.

mod constructors;
mod pins;

use crate::pins::PinType;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NodeKind {
    // ── Events ──────────────────────────────────────────────────────────────
    EventBeginPlay,
    EventTick,

    // ── Exec-flow ────────────────────────────────────────────────────────────
    PrintLog,
    Delay,
    AddGold,
    Branch,
    CheckGoldAmount,

    // ── Float math — binary (a, b → result: f32) ────────────────────────────
    FloatAdd,
    FloatSubtract,
    FloatMultiply,
    FloatDivide,
    FloatMin,
    FloatMax,
    FloatPow,

    // ── Float math — unary (a → result: f32) ────────────────────────────────
    FloatNegate,
    FloatAbs,
    FloatSqrt,
    FloatSin,
    FloatCos,
    FloatFloor,
    FloatCeil,
    FloatRound,

    // ── Float math — ternary ─────────────────────────────────────────────────
    FloatLerp,
    FloatClamp,

    // ── Comparison (a, b → result: bool) ────────────────────────────────────
    FloatGreater,
    FloatLess,
    FloatGreaterEqual,
    FloatLessEqual,
    FloatEqual,

    // ── Boolean logic ─────────────────────────────────────────────────────────
    BoolAnd,
    BoolOr,
    BoolNot,
    BoolXor,

    // ── Input ────────────────────────────────────────────────────────────────
    IsKeyPressed,
    IsKeyJustPressed,

    // ── Vec3 math ─────────────────────────────────────────────────────────────
    Vec3Make,
    Vec3Add,
    Vec3Sub,
    Vec3Scale,
    Vec3Length,
    Vec3Normalize,
    Vec3Dot,
    Vec3Lerp,

    // ── String operations ─────────────────────────────────────────────────────
    StringConcat,
    FloatToString,
    BoolToString,
    IntToString,

    // ── Integer math ──────────────────────────────────────────────────────────
    IntAdd,
    IntSubtract,
    IntMultiply,
    IntDivide,
    IntModulo,
    FloatToInt,
    IntToFloat,
    IntGreater,
    IntLess,
    IntEqual,

    // ── Entity tizimi ─────────────────────────────────────────────────────────
    SpawnEntity,
    DestroyEntity,
    GetNamedEntity,
    GetTranslation,
    SetTranslation,
    Translate,
    GetScale,
    SetScale,
    GetRotationEuler,
    SetRotationEuler,

    // ── Custom Events ─────────────────────────────────────────────────────────
    EventCustomBegin,
    FireCustomEvent,

    // ── Script Variables ──────────────────────────────────────────────────────
    SetFloatVar,
    GetFloatVar,
    SetBoolVar,
    GetBoolVar,
    SetIntVar,
    GetIntVar,
    SetStringVar,
    GetStringVar,

    // ── Float Collections ─────────────────────────────────────────────────────
    FloatArrayPush,
    FloatArrayGet,
    FloatArrayLength,
    FloatArrayClear,

    // ── Control Flow (extended) ───────────────────────────────────────────────
    Sequence,
    DoOnce,

    // ── Loops ─────────────────────────────────────────────────────────────────
    ForEachFloat,
    WhileLoop,

    // ── Helper Nodes ──────────────────────────────────────────────────────────
    RandomFloat,
    GetGameTime,
    IsValidEntity,

    // ── Loop control ──────────────────────────────────────────────────────────
    BreakLoop,
    ResetDoOnce,

    // ── Select / ternary helpers ──────────────────────────────────────────────
    SelectFloat,
    SelectBool,
    SelectInt,
    SelectString,

    // ── Int arrays ───────────────────────────────────────────────────────────
    IntArrayPush,
    IntArrayGet,
    IntArrayLength,
    IntArrayClear,

    // ── String arrays ─────────────────────────────────────────────────────────
    StringArrayPush,
    StringArrayGet,
    StringArrayLength,
    StringArrayClear,

    // ── Vec2 math ─────────────────────────────────────────────────────────────
    Vec2Make,
    Vec2Add,
    Vec2Sub,
    Vec2Scale,
    Vec2Length,
    Vec2Normalize,
    Vec2Dot,
    Vec2X,
    Vec2Y,

    // ── Loop control (extended) ───────────────────────────────────────────────
    ContinueLoop,

    // ── Visual annotations ────────────────────────────────────────────────────
    /// Faqat UI uchun — izoh qutisi. Exec/data pinlari yo'q.
    Comment,

    // ── ScriptActor self-reference ────────────────────────────────────────────
    /// Joriy ScriptActor entity'sini qaytaradi (per-entity scripting uchun).
    GetSelfEntity,

    // ── ECS Query (World darajasi) ────────────────────────────────────────────
    /// SceneTree dagi barcha entitylarni qaytaradi.
    QueryAllEntities,
    /// Tag bo'yicha entitylarni filtrlaydi (HasTag=true bo'lganlar).
    QueryByTag,
    /// Entity massivini iteratsiya qilish (ForEach loop).
    ForEachEntity,
    /// Entity massividan indeks bo'yicha bitta entity olish.
    EntityArrayGet,
    /// Entity massividagi elementlar soni.
    EntityArrayLength,

    // ── ECS Component / Tag ───────────────────────────────────────────────────
    /// Entityga tag qo'shish (PIE: ScriptActor.vars; AOT: marker component).
    AddTag,
    /// Entitydan tag olib tashlash.
    RemoveTag,
    /// Entityda tag borligini tekshirish → Bool.
    HasTag,
    /// Entityning nomini qaytaradi (SceneEntityName komponentidan).
    GetEntityName,

    // ── ECS Schedule / Events ─────────────────────────────────────────────────
    /// FixedUpdate schedule — fizika va deterministic logic uchun.
    EventFixedTick,
    /// Entity spawn bo'lganda ishga tushadigan event.
    EventOnSpawn,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VisualNode {
    pub kind: NodeKind,

    // ── Exec-node params ─────────────────────────────────────────────────────
    pub log_message:    Option<String>,
    pub delay_seconds:  Option<f32>,
    pub gold_amount:    Option<f32>,
    pub gold_threshold: Option<f32>,
    pub condition_value: Option<bool>,

    // ── Input node params ─────────────────────────────────────────────────────
    pub key_name:    Option<String>,
    pub entity_name: Option<String>,
    pub event_name:  Option<String>,

    // ── Pure-node literals ────────────────────────────────────────────────────
    pub float_a:  Option<f32>,
    pub float_b:  Option<f32>,
    pub float_c:  Option<f32>,
    pub bool_a:   Option<bool>,
    pub bool_b:   Option<bool>,
    pub string_a: Option<String>,
    pub string_b: Option<String>,
    pub int_a:    Option<i32>,
    pub int_b:    Option<i32>,
}

impl VisualNode {
    pub(crate) fn empty(kind: NodeKind) -> Self {
        Self {
            kind,
            log_message:    None,
            delay_seconds:  None,
            gold_amount:    None,
            gold_threshold: None,
            condition_value: None,
            key_name:    None,
            entity_name: None,
            event_name:  None,
            float_a:  None,
            float_b:  None,
            float_c:  None,
            bool_a:   None,
            bool_b:   None,
            string_a: None,
            string_b: None,
            int_a:    None,
            int_b:    None,
        }
    }

    // ── Queries ───────────────────────────────────────────────────────────────

    pub fn is_event(&self) -> bool {
        matches!(self.kind,
            NodeKind::EventBeginPlay
            | NodeKind::EventTick
            | NodeKind::EventCustomBegin
            | NodeKind::EventFixedTick
            | NodeKind::EventOnSpawn)
    }

    pub fn is_begin_play(&self) -> bool {
        matches!(self.kind, NodeKind::EventBeginPlay | NodeKind::EventOnSpawn)
    }

    pub fn is_tick(&self) -> bool {
        matches!(self.kind, NodeKind::EventTick | NodeKind::EventFixedTick)
    }

    pub fn is_custom_event(&self) -> bool {
        matches!(self.kind, NodeKind::EventCustomBegin)
    }

    pub fn needs_gold(&self) -> bool {
        matches!(self.kind, NodeKind::AddGold | NodeKind::CheckGoldAmount)
    }

    /// Exec pinlari yo'q, faqat data in/out — "sof hisob" tuguni.
    pub fn is_pure(&self) -> bool {
        self.exec_inputs().is_empty() && self.exec_outputs().is_empty()
    }

    // ── Literal helpers ───────────────────────────────────────────────────────

    pub fn float_literal_for(&self, pin: &str) -> f32 {
        match pin {
            "a" | "value" | "base" | "x" | "float_val" => self.float_a.unwrap_or(0.0),
            "b" | "min" | "exp" | "y"                  => self.float_b.unwrap_or(0.0),
            "t" | "max" | "eps" | "z"                  => self.float_c.unwrap_or(0.0),
            _ => 0.0,
        }
    }

    pub fn bool_literal_for(&self, pin: &str) -> bool {
        match pin {
            "a"         => self.bool_a.unwrap_or(false),
            "b"         => self.bool_b.unwrap_or(false),
            "condition" => self.condition_value.unwrap_or(false),
            _ => false,
        }
    }

    pub fn string_literal_for(&self, pin: &str) -> String {
        match pin {
            "string_a" | "text" => self.string_a.clone().unwrap_or_default(),
            "string_b"          => self.string_b.clone().unwrap_or_default(),
            _ => String::new(),
        }
    }

    pub fn int_literal_for(&self, pin: &str) -> i32 {
        match pin {
            "int_a" | "int_val" => self.int_a.unwrap_or(0),
            "int_b"             => self.int_b.unwrap_or(0),
            _ => 0,
        }
    }

    pub fn data_input_type(&self, pin: &str) -> Option<PinType> {
        self.data_inputs().iter().find(|p| p.name == pin).map(|p| p.ty)
    }

    pub fn data_output_type(&self, pin: &str) -> Option<PinType> {
        self.data_outputs().iter().find(|p| p.name == pin).map(|p| p.ty)
    }
}
