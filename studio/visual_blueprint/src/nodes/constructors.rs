//! Constructor helpers for `VisualNode`.

use super::{NodeKind, VisualNode};

impl VisualNode {
    // ── Event constructors ────────────────────────────────────────────────────

    pub fn event_begin_play() -> Self { Self::empty(NodeKind::EventBeginPlay) }
    pub fn event_tick()       -> Self { Self::empty(NodeKind::EventTick) }

    // ── Exec-flow constructors ────────────────────────────────────────────────

    pub fn print_log(message: impl Into<String>) -> Self {
        Self { kind: NodeKind::PrintLog, log_message: Some(message.into()), ..Self::empty(NodeKind::PrintLog) }
    }

    pub fn delay(seconds: f32) -> Self {
        Self { kind: NodeKind::Delay, delay_seconds: Some(seconds), ..Self::empty(NodeKind::Delay) }
    }

    pub fn add_gold(amount: f32) -> Self {
        Self { kind: NodeKind::AddGold, gold_amount: Some(amount), ..Self::empty(NodeKind::AddGold) }
    }

    pub fn branch() -> Self {
        Self { kind: NodeKind::Branch, condition_value: Some(false), ..Self::empty(NodeKind::Branch) }
    }

    pub fn check_gold_amount(threshold: f32) -> Self {
        Self { kind: NodeKind::CheckGoldAmount, gold_threshold: Some(threshold), ..Self::empty(NodeKind::CheckGoldAmount) }
    }

    // ── Float math — binary ───────────────────────────────────────────────────

    pub fn float_add(a: f32, b: f32) -> Self {
        Self { kind: NodeKind::FloatAdd, float_a: Some(a), float_b: Some(b), ..Self::empty(NodeKind::FloatAdd) }
    }
    pub fn float_subtract(a: f32, b: f32) -> Self {
        Self { kind: NodeKind::FloatSubtract, float_a: Some(a), float_b: Some(b), ..Self::empty(NodeKind::FloatSubtract) }
    }
    pub fn float_multiply(a: f32, b: f32) -> Self {
        Self { kind: NodeKind::FloatMultiply, float_a: Some(a), float_b: Some(b), ..Self::empty(NodeKind::FloatMultiply) }
    }
    pub fn float_divide(a: f32, b: f32) -> Self {
        Self { kind: NodeKind::FloatDivide, float_a: Some(a), float_b: Some(b), ..Self::empty(NodeKind::FloatDivide) }
    }
    pub fn float_min(a: f32, b: f32) -> Self {
        Self { kind: NodeKind::FloatMin, float_a: Some(a), float_b: Some(b), ..Self::empty(NodeKind::FloatMin) }
    }
    pub fn float_max(a: f32, b: f32) -> Self {
        Self { kind: NodeKind::FloatMax, float_a: Some(a), float_b: Some(b), ..Self::empty(NodeKind::FloatMax) }
    }
    pub fn float_pow(base: f32, exp: f32) -> Self {
        Self { kind: NodeKind::FloatPow, float_a: Some(base), float_b: Some(exp), ..Self::empty(NodeKind::FloatPow) }
    }

    // ── Float math — unary ────────────────────────────────────────────────────

    pub fn float_negate(a: f32) -> Self {
        Self { kind: NodeKind::FloatNegate, float_a: Some(a), ..Self::empty(NodeKind::FloatNegate) }
    }
    pub fn float_abs(a: f32) -> Self {
        Self { kind: NodeKind::FloatAbs, float_a: Some(a), ..Self::empty(NodeKind::FloatAbs) }
    }
    pub fn float_sqrt(a: f32) -> Self {
        Self { kind: NodeKind::FloatSqrt, float_a: Some(a), ..Self::empty(NodeKind::FloatSqrt) }
    }
    pub fn float_sin(a: f32) -> Self {
        Self { kind: NodeKind::FloatSin, float_a: Some(a), ..Self::empty(NodeKind::FloatSin) }
    }
    pub fn float_cos(a: f32) -> Self {
        Self { kind: NodeKind::FloatCos, float_a: Some(a), ..Self::empty(NodeKind::FloatCos) }
    }
    pub fn float_floor(a: f32) -> Self {
        Self { kind: NodeKind::FloatFloor, float_a: Some(a), ..Self::empty(NodeKind::FloatFloor) }
    }
    pub fn float_ceil(a: f32) -> Self {
        Self { kind: NodeKind::FloatCeil, float_a: Some(a), ..Self::empty(NodeKind::FloatCeil) }
    }
    pub fn float_round(a: f32) -> Self {
        Self { kind: NodeKind::FloatRound, float_a: Some(a), ..Self::empty(NodeKind::FloatRound) }
    }

    // ── Float math — ternary ──────────────────────────────────────────────────

    pub fn float_lerp(a: f32, b: f32, t: f32) -> Self {
        Self { kind: NodeKind::FloatLerp, float_a: Some(a), float_b: Some(b), float_c: Some(t), ..Self::empty(NodeKind::FloatLerp) }
    }
    pub fn float_clamp(value: f32, min: f32, max: f32) -> Self {
        Self { kind: NodeKind::FloatClamp, float_a: Some(value), float_b: Some(min), float_c: Some(max), ..Self::empty(NodeKind::FloatClamp) }
    }

    // ── Comparison ────────────────────────────────────────────────────────────

    pub fn float_greater(a: f32, b: f32) -> Self {
        Self { kind: NodeKind::FloatGreater, float_a: Some(a), float_b: Some(b), ..Self::empty(NodeKind::FloatGreater) }
    }
    pub fn float_less(a: f32, b: f32) -> Self {
        Self { kind: NodeKind::FloatLess, float_a: Some(a), float_b: Some(b), ..Self::empty(NodeKind::FloatLess) }
    }
    pub fn float_greater_equal(a: f32, b: f32) -> Self {
        Self { kind: NodeKind::FloatGreaterEqual, float_a: Some(a), float_b: Some(b), ..Self::empty(NodeKind::FloatGreaterEqual) }
    }
    pub fn float_less_equal(a: f32, b: f32) -> Self {
        Self { kind: NodeKind::FloatLessEqual, float_a: Some(a), float_b: Some(b), ..Self::empty(NodeKind::FloatLessEqual) }
    }
    pub fn float_equal(a: f32, b: f32, eps: f32) -> Self {
        Self { kind: NodeKind::FloatEqual, float_a: Some(a), float_b: Some(b), float_c: Some(eps), ..Self::empty(NodeKind::FloatEqual) }
    }

    // ── Boolean logic ──────────────────────────────────────────────────────────

    pub fn bool_and(a: bool, b: bool) -> Self {
        Self { kind: NodeKind::BoolAnd, bool_a: Some(a), bool_b: Some(b), ..Self::empty(NodeKind::BoolAnd) }
    }
    pub fn bool_or(a: bool, b: bool) -> Self {
        Self { kind: NodeKind::BoolOr, bool_a: Some(a), bool_b: Some(b), ..Self::empty(NodeKind::BoolOr) }
    }
    pub fn bool_not(a: bool) -> Self {
        Self { kind: NodeKind::BoolNot, bool_a: Some(a), ..Self::empty(NodeKind::BoolNot) }
    }
    pub fn bool_xor(a: bool, b: bool) -> Self {
        Self { kind: NodeKind::BoolXor, bool_a: Some(a), bool_b: Some(b), ..Self::empty(NodeKind::BoolXor) }
    }

    // ── Input constructors ────────────────────────────────────────────────────

    pub fn is_key_pressed(key: impl Into<String>) -> Self {
        Self { kind: NodeKind::IsKeyPressed, key_name: Some(key.into()), ..Self::empty(NodeKind::IsKeyPressed) }
    }
    pub fn is_key_just_pressed(key: impl Into<String>) -> Self {
        Self { kind: NodeKind::IsKeyJustPressed, key_name: Some(key.into()), ..Self::empty(NodeKind::IsKeyJustPressed) }
    }

    // ── Vec3 constructors ─────────────────────────────────────────────────────

    pub fn vec3_make(x: f32, y: f32, z: f32) -> Self {
        Self { kind: NodeKind::Vec3Make, float_a: Some(x), float_b: Some(y), float_c: Some(z), ..Self::empty(NodeKind::Vec3Make) }
    }
    pub fn vec3_add()       -> Self { Self::empty(NodeKind::Vec3Add) }
    pub fn vec3_sub()       -> Self { Self::empty(NodeKind::Vec3Sub) }
    pub fn vec3_scale()     -> Self { Self::empty(NodeKind::Vec3Scale) }
    pub fn vec3_length()    -> Self { Self::empty(NodeKind::Vec3Length) }
    pub fn vec3_normalize() -> Self { Self::empty(NodeKind::Vec3Normalize) }
    pub fn vec3_dot()       -> Self { Self::empty(NodeKind::Vec3Dot) }
    pub fn vec3_lerp()      -> Self { Self::empty(NodeKind::Vec3Lerp) }

    // ── String constructors ───────────────────────────────────────────────────

    pub fn string_concat(a: impl Into<String>, b: impl Into<String>) -> Self {
        Self {
            kind: NodeKind::StringConcat,
            string_a: Some(a.into()),
            string_b: Some(b.into()),
            ..Self::empty(NodeKind::StringConcat)
        }
    }
    pub fn float_to_string() -> Self { Self::empty(NodeKind::FloatToString) }
    pub fn bool_to_string()  -> Self { Self::empty(NodeKind::BoolToString) }
    pub fn int_to_string()   -> Self { Self::empty(NodeKind::IntToString) }

    // ── Integer constructors ──────────────────────────────────────────────────

    pub fn int_add(a: i32, b: i32) -> Self {
        Self { kind: NodeKind::IntAdd, int_a: Some(a), int_b: Some(b), ..Self::empty(NodeKind::IntAdd) }
    }
    pub fn int_subtract(a: i32, b: i32) -> Self {
        Self { kind: NodeKind::IntSubtract, int_a: Some(a), int_b: Some(b), ..Self::empty(NodeKind::IntSubtract) }
    }
    pub fn int_multiply(a: i32, b: i32) -> Self {
        Self { kind: NodeKind::IntMultiply, int_a: Some(a), int_b: Some(b), ..Self::empty(NodeKind::IntMultiply) }
    }
    pub fn int_divide(a: i32, b: i32) -> Self {
        Self { kind: NodeKind::IntDivide, int_a: Some(a), int_b: Some(b), ..Self::empty(NodeKind::IntDivide) }
    }
    pub fn int_modulo(a: i32, b: i32) -> Self {
        Self { kind: NodeKind::IntModulo, int_a: Some(a), int_b: Some(b), ..Self::empty(NodeKind::IntModulo) }
    }
    pub fn float_to_int() -> Self { Self::empty(NodeKind::FloatToInt) }
    pub fn int_to_float() -> Self { Self::empty(NodeKind::IntToFloat) }
    pub fn int_greater(a: i32, b: i32) -> Self {
        Self { kind: NodeKind::IntGreater, int_a: Some(a), int_b: Some(b), ..Self::empty(NodeKind::IntGreater) }
    }
    pub fn int_less(a: i32, b: i32) -> Self {
        Self { kind: NodeKind::IntLess, int_a: Some(a), int_b: Some(b), ..Self::empty(NodeKind::IntLess) }
    }
    pub fn int_equal(a: i32, b: i32) -> Self {
        Self { kind: NodeKind::IntEqual, int_a: Some(a), int_b: Some(b), ..Self::empty(NodeKind::IntEqual) }
    }

    // ── Entity constructors ───────────────────────────────────────────────────

    pub fn spawn_entity()   -> Self { Self::empty(NodeKind::SpawnEntity) }
    pub fn destroy_entity() -> Self { Self::empty(NodeKind::DestroyEntity) }

    pub fn get_named_entity(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::GetNamedEntity, entity_name: Some(name.into()), ..Self::empty(NodeKind::GetNamedEntity) }
    }

    pub fn get_translation()    -> Self { Self::empty(NodeKind::GetTranslation) }
    pub fn set_translation()    -> Self { Self::empty(NodeKind::SetTranslation) }
    pub fn translate_entity()   -> Self { Self::empty(NodeKind::Translate) }
    pub fn get_scale()          -> Self { Self::empty(NodeKind::GetScale) }
    pub fn set_scale()          -> Self { Self::empty(NodeKind::SetScale) }
    pub fn get_rotation_euler() -> Self { Self::empty(NodeKind::GetRotationEuler) }
    pub fn set_rotation_euler() -> Self { Self::empty(NodeKind::SetRotationEuler) }

    // ── Custom event constructors ─────────────────────────────────────────────

    pub fn event_custom_begin(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::EventCustomBegin, event_name: Some(name.into()), ..Self::empty(NodeKind::EventCustomBegin) }
    }
    pub fn fire_custom_event(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::FireCustomEvent, event_name: Some(name.into()), ..Self::empty(NodeKind::FireCustomEvent) }
    }

    // ── Variable constructors — name stored in string_a ───────────────────────

    pub fn set_float_var(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::SetFloatVar, string_a: Some(name.into()), ..Self::empty(NodeKind::SetFloatVar) }
    }
    pub fn get_float_var(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::GetFloatVar, string_a: Some(name.into()), ..Self::empty(NodeKind::GetFloatVar) }
    }
    pub fn set_bool_var(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::SetBoolVar, string_a: Some(name.into()), ..Self::empty(NodeKind::SetBoolVar) }
    }
    pub fn get_bool_var(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::GetBoolVar, string_a: Some(name.into()), ..Self::empty(NodeKind::GetBoolVar) }
    }
    pub fn set_int_var(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::SetIntVar, string_a: Some(name.into()), ..Self::empty(NodeKind::SetIntVar) }
    }
    pub fn get_int_var(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::GetIntVar, string_a: Some(name.into()), ..Self::empty(NodeKind::GetIntVar) }
    }
    pub fn set_string_var(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::SetStringVar, string_a: Some(name.into()), ..Self::empty(NodeKind::SetStringVar) }
    }
    pub fn get_string_var(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::GetStringVar, string_a: Some(name.into()), ..Self::empty(NodeKind::GetStringVar) }
    }

    // ── Float Collection constructors — array name in string_a ────────────────

    pub fn float_array_push(array_name: impl Into<String>) -> Self {
        Self { kind: NodeKind::FloatArrayPush, string_a: Some(array_name.into()), ..Self::empty(NodeKind::FloatArrayPush) }
    }
    pub fn float_array_get(array_name: impl Into<String>) -> Self {
        Self { kind: NodeKind::FloatArrayGet, string_a: Some(array_name.into()), ..Self::empty(NodeKind::FloatArrayGet) }
    }
    pub fn float_array_length(array_name: impl Into<String>) -> Self {
        Self { kind: NodeKind::FloatArrayLength, string_a: Some(array_name.into()), ..Self::empty(NodeKind::FloatArrayLength) }
    }
    pub fn float_array_clear(array_name: impl Into<String>) -> Self {
        Self { kind: NodeKind::FloatArrayClear, string_a: Some(array_name.into()), ..Self::empty(NodeKind::FloatArrayClear) }
    }

    // ── Control flow constructors ─────────────────────────────────────────────

    pub fn sequence() -> Self { Self::empty(NodeKind::Sequence) }
    pub fn do_once()  -> Self { Self::empty(NodeKind::DoOnce) }

    // ── Loop constructors ─────────────────────────────────────────────────────

    pub fn for_each_float(array_name: impl Into<String>) -> Self {
        Self { kind: NodeKind::ForEachFloat, string_a: Some(array_name.into()), ..Self::empty(NodeKind::ForEachFloat) }
    }
    pub fn while_loop() -> Self { Self::empty(NodeKind::WhileLoop) }

    // ── Helper constructors ───────────────────────────────────────────────────

    pub fn random_float(min: f32, max: f32) -> Self {
        Self { kind: NodeKind::RandomFloat, float_a: Some(min), float_b: Some(max), ..Self::empty(NodeKind::RandomFloat) }
    }
    pub fn get_game_time()    -> Self { Self::empty(NodeKind::GetGameTime) }
    pub fn is_valid_entity()  -> Self { Self::empty(NodeKind::IsValidEntity) }

    // ── Loop control constructors ─────────────────────────────────────────────

    pub fn break_loop()    -> Self { Self::empty(NodeKind::BreakLoop) }
    pub fn reset_do_once() -> Self { Self::empty(NodeKind::ResetDoOnce) }

    // ── Select ternary constructors ───────────────────────────────────────────

    pub fn select_float()  -> Self { Self::empty(NodeKind::SelectFloat) }
    pub fn select_bool()   -> Self { Self::empty(NodeKind::SelectBool) }
    pub fn select_int()    -> Self { Self::empty(NodeKind::SelectInt) }
    pub fn select_string() -> Self { Self::empty(NodeKind::SelectString) }

    // ── Int Array constructors ────────────────────────────────────────────────

    pub fn int_array_push(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::IntArrayPush, string_a: Some(name.into()), ..Self::empty(NodeKind::IntArrayPush) }
    }
    pub fn int_array_get(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::IntArrayGet, string_a: Some(name.into()), ..Self::empty(NodeKind::IntArrayGet) }
    }
    pub fn int_array_length(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::IntArrayLength, string_a: Some(name.into()), ..Self::empty(NodeKind::IntArrayLength) }
    }
    pub fn int_array_clear(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::IntArrayClear, string_a: Some(name.into()), ..Self::empty(NodeKind::IntArrayClear) }
    }

    // ── String Array constructors ─────────────────────────────────────────────

    pub fn string_array_push(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::StringArrayPush, string_a: Some(name.into()), ..Self::empty(NodeKind::StringArrayPush) }
    }
    pub fn string_array_get(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::StringArrayGet, string_a: Some(name.into()), ..Self::empty(NodeKind::StringArrayGet) }
    }
    pub fn string_array_length(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::StringArrayLength, string_a: Some(name.into()), ..Self::empty(NodeKind::StringArrayLength) }
    }
    pub fn string_array_clear(name: impl Into<String>) -> Self {
        Self { kind: NodeKind::StringArrayClear, string_a: Some(name.into()), ..Self::empty(NodeKind::StringArrayClear) }
    }

    // ── Vec2 constructors ─────────────────────────────────────────────────────

    pub fn vec2_make(x: f32, y: f32) -> Self {
        Self { kind: NodeKind::Vec2Make, float_a: Some(x), float_b: Some(y), ..Self::empty(NodeKind::Vec2Make) }
    }
    pub fn vec2_add()       -> Self { Self::empty(NodeKind::Vec2Add) }
    pub fn vec2_sub()       -> Self { Self::empty(NodeKind::Vec2Sub) }
    pub fn vec2_scale()     -> Self { Self::empty(NodeKind::Vec2Scale) }
    pub fn vec2_length()    -> Self { Self::empty(NodeKind::Vec2Length) }
    pub fn vec2_normalize() -> Self { Self::empty(NodeKind::Vec2Normalize) }
    pub fn vec2_dot()       -> Self { Self::empty(NodeKind::Vec2Dot) }
    pub fn vec2_x()         -> Self { Self::empty(NodeKind::Vec2X) }
    pub fn vec2_y()         -> Self { Self::empty(NodeKind::Vec2Y) }

    // ── Loop control (extended) ───────────────────────────────────────────────

    pub fn continue_loop() -> Self { Self::empty(NodeKind::ContinueLoop) }

    // ── Comment ───────────────────────────────────────────────────────────────

    pub fn comment(text: impl Into<String>) -> Self {
        let mut n = Self::empty(NodeKind::Comment);
        n.log_message = Some(text.into());
        n
    }

    // ── ScriptActor ───────────────────────────────────────────────────────────

    pub fn get_self_entity() -> Self { Self::empty(NodeKind::GetSelfEntity) }

    // ── ECS Query ─────────────────────────────────────────────────────────────

    pub fn query_all_entities() -> Self  { Self::empty(NodeKind::QueryAllEntities) }
    pub fn for_each_entity() -> Self     { Self::empty(NodeKind::ForEachEntity) }
    pub fn entity_array_get() -> Self    { Self::empty(NodeKind::EntityArrayGet) }
    pub fn entity_array_length() -> Self { Self::empty(NodeKind::EntityArrayLength) }
    pub fn query_by_tag(tag: impl Into<String>) -> Self {
        let mut n = Self::empty(NodeKind::QueryByTag);
        n.string_a = Some(tag.into());
        n
    }

    // ── ECS Component / Tag ───────────────────────────────────────────────────

    pub fn add_tag(tag: impl Into<String>) -> Self {
        let mut n = Self::empty(NodeKind::AddTag);
        n.string_a = Some(tag.into());
        n
    }
    pub fn remove_tag(tag: impl Into<String>) -> Self {
        let mut n = Self::empty(NodeKind::RemoveTag);
        n.string_a = Some(tag.into());
        n
    }
    pub fn has_tag(tag: impl Into<String>) -> Self {
        let mut n = Self::empty(NodeKind::HasTag);
        n.string_a = Some(tag.into());
        n
    }
    pub fn get_entity_name() -> Self { Self::empty(NodeKind::GetEntityName) }

    // ── ECS Schedule ─────────────────────────────────────────────────────────

    pub fn event_fixed_tick() -> Self  { Self::empty(NodeKind::EventFixedTick) }
    pub fn event_on_spawn() -> Self    { Self::empty(NodeKind::EventOnSpawn) }
}
