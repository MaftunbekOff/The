//! Math kategoriyasi: Float, Int, Vec3, Bool — barcha sof-hisob tugunlar.
//! Hisob `context.rs` da amalga oshiriladi. exec = None.

use twelfth_visual_blueprint::nodes::{NodeKind as K, VisualNode};

use crate::nodes::descriptor::{NodeDescriptor, NodeRegistry};

pub(crate) fn register(r: &mut NodeRegistry) {
    // ── Float binary ─────────────────────────────────────────────────────────
    r.register(NodeDescriptor {
        kind: K::FloatAdd,       label: "Float +",     description: "a + b → result",
        category: "Math",        width: 168.0,          height: 104.0,
        default_node: || VisualNode::float_add(0.0, 0.0),        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatSubtract,  label: "Float −",     description: "a - b → result",
        category: "Math",        width: 168.0,          height: 104.0,
        default_node: || VisualNode::float_subtract(0.0, 0.0),   exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatMultiply,  label: "Float ×",     description: "a * b → result",
        category: "Math",        width: 168.0,          height: 104.0,
        default_node: || VisualNode::float_multiply(1.0, 1.0),   exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatDivide,    label: "Float ÷",     description: "a / b → result",
        category: "Math",        width: 168.0,          height: 104.0,
        default_node: || VisualNode::float_divide(1.0, 1.0),     exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatMin,       label: "Float Min",   description: "min(a, b) → result",
        category: "Math",        width: 168.0,          height: 104.0,
        default_node: || VisualNode::float_min(0.0, 1.0),        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatMax,       label: "Float Max",   description: "max(a, b) → result",
        category: "Math",        width: 168.0,          height: 104.0,
        default_node: || VisualNode::float_max(0.0, 1.0),        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatPow,       label: "Float Pow",   description: "base ^ exp → result",
        category: "Math",        width: 168.0,          height: 104.0,
        default_node: || VisualNode::float_pow(2.0, 2.0),        exec: None,
    });

    // ── Float unary ──────────────────────────────────────────────────────────
    r.register(NodeDescriptor {
        kind: K::FloatNegate,    label: "Float Negate",  description: "-a → result",
        category: "Math",        width: 160.0,            height: 88.0,
        default_node: || VisualNode::float_negate(1.0),   exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatAbs,       label: "Float Abs",     description: "|a| → result",
        category: "Math",        width: 160.0,            height: 88.0,
        default_node: || VisualNode::float_abs(-1.0),     exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatSqrt,      label: "Float Sqrt",    description: "√a → result",
        category: "Math",        width: 160.0,            height: 88.0,
        default_node: || VisualNode::float_sqrt(4.0),     exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatSin,       label: "Float Sin",     description: "sin(a) → result",
        category: "Math",        width: 160.0,            height: 88.0,
        default_node: || VisualNode::float_sin(0.0),      exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatCos,       label: "Float Cos",     description: "cos(a) → result",
        category: "Math",        width: 160.0,            height: 88.0,
        default_node: || VisualNode::float_cos(0.0),      exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatFloor,     label: "Float Floor",   description: "⌊a⌋ → result",
        category: "Math",        width: 160.0,            height: 88.0,
        default_node: || VisualNode::float_floor(1.7),    exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatCeil,      label: "Float Ceil",    description: "⌈a⌉ → result",
        category: "Math",        width: 160.0,            height: 88.0,
        default_node: || VisualNode::float_ceil(1.2),     exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatRound,     label: "Float Round",   description: "round(a) → result",
        category: "Math",        width: 160.0,            height: 88.0,
        default_node: || VisualNode::float_round(1.5),    exec: None,
    });

    // ── Float ternary ────────────────────────────────────────────────────────
    r.register(NodeDescriptor {
        kind: K::FloatLerp,      label: "Float Lerp",    description: "lerp(a, b, t) → result",
        category: "Math",        width: 172.0,            height: 120.0,
        default_node: || VisualNode::float_lerp(0.0, 1.0, 0.5),  exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatClamp,     label: "Float Clamp",   description: "clamp(v, min, max) → result",
        category: "Math",        width: 172.0,            height: 120.0,
        default_node: || VisualNode::float_clamp(0.5, 0.0, 1.0), exec: None,
    });

    // ── Float comparison ─────────────────────────────────────────────────────
    r.register(NodeDescriptor {
        kind: K::FloatGreater,      label: "Float >",  description: "a > b → bool",
        category: "Math",           width: 168.0,       height: 104.0,
        default_node: || VisualNode::float_greater(1.0, 0.0),      exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatLess,         label: "Float <",  description: "a < b → bool",
        category: "Math",           width: 168.0,       height: 104.0,
        default_node: || VisualNode::float_less(0.0, 1.0),         exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatGreaterEqual, label: "Float ≥",  description: "a ≥ b → bool",
        category: "Math",           width: 168.0,       height: 104.0,
        default_node: || VisualNode::float_greater_equal(1.0, 0.0),exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatLessEqual,    label: "Float ≤",  description: "a ≤ b → bool",
        category: "Math",           width: 168.0,       height: 104.0,
        default_node: || VisualNode::float_less_equal(0.0, 1.0),   exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatEqual,        label: "Float ≈",  description: "|a−b| ≤ eps → bool",
        category: "Math",           width: 172.0,       height: 120.0,
        default_node: || VisualNode::float_equal(0.0, 0.0, 0.001), exec: None,
    });

    // ── Bool logic ───────────────────────────────────────────────────────────
    r.register(NodeDescriptor {
        kind: K::BoolAnd,  label: "Bool AND", description: "a && b → result",
        category: "Logic", width: 160.0,      height: 104.0,
        default_node: || VisualNode::bool_and(true, true),   exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::BoolOr,   label: "Bool OR",  description: "a || b → result",
        category: "Logic", width: 160.0,      height: 104.0,
        default_node: || VisualNode::bool_or(false, false),  exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::BoolNot,  label: "Bool NOT", description: "!a → result",
        category: "Logic", width: 152.0,      height: 88.0,
        default_node: || VisualNode::bool_not(false),        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::BoolXor,  label: "Bool XOR", description: "a ^ b → result",
        category: "Logic", width: 160.0,      height: 104.0,
        default_node: || VisualNode::bool_xor(false, false), exec: None,
    });

    // ── Vec3 ─────────────────────────────────────────────────────────────────
    r.register(NodeDescriptor {
        kind: K::Vec3Make,       label: "Vec3 Make",      description: "x,y,z → Vec3",
        category: "Vec3",        width: 172.0,             height: 120.0,
        default_node: || VisualNode::vec3_make(0.0, 0.0, 0.0),   exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec3Add,        label: "Vec3 +",          description: "a + b → Vec3",
        category: "Vec3",        width: 172.0,             height: 104.0,
        default_node: VisualNode::vec3_add,                       exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec3Sub,        label: "Vec3 −",          description: "a - b → Vec3",
        category: "Vec3",        width: 172.0,             height: 104.0,
        default_node: VisualNode::vec3_sub,                       exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec3Scale,      label: "Vec3 Scale",      description: "vec * scale → Vec3",
        category: "Vec3",        width: 180.0,             height: 104.0,
        default_node: VisualNode::vec3_scale,                     exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec3Length,     label: "Vec3 Length",     description: "|vec| → f32",
        category: "Vec3",        width: 168.0,             height: 88.0,
        default_node: VisualNode::vec3_length,                    exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec3Normalize,  label: "Vec3 Normalize",  description: "normalize(vec) → Vec3",
        category: "Vec3",        width: 168.0,             height: 88.0,
        default_node: VisualNode::vec3_normalize,                 exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec3Dot,        label: "Vec3 Dot",        description: "dot(a,b) → f32",
        category: "Vec3",        width: 172.0,             height: 104.0,
        default_node: VisualNode::vec3_dot,                       exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::Vec3Lerp,       label: "Vec3 Lerp",       description: "lerp(a, b, t) → Vec3",
        category: "Vec3",        width: 180.0,             height: 120.0,
        default_node: VisualNode::vec3_lerp,                      exec: None,
    });

    // ── Integer math ─────────────────────────────────────────────────────────
    r.register(NodeDescriptor {
        kind: K::IntAdd,      label: "Int +",   description: "a + b → i32",
        category: "Int",      width: 168.0,     height: 104.0,
        default_node: || VisualNode::int_add(0, 0),       exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::IntSubtract, label: "Int −",   description: "a - b → i32",
        category: "Int",      width: 168.0,     height: 104.0,
        default_node: || VisualNode::int_subtract(0, 0),  exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::IntMultiply, label: "Int ×",   description: "a * b → i32",
        category: "Int",      width: 168.0,     height: 104.0,
        default_node: || VisualNode::int_multiply(1, 1),  exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::IntDivide,   label: "Int ÷",   description: "a / b → i32",
        category: "Int",      width: 168.0,     height: 104.0,
        default_node: || VisualNode::int_divide(1, 1),    exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::IntModulo,   label: "Int %",   description: "a % b → i32",
        category: "Int",      width: 168.0,     height: 104.0,
        default_node: || VisualNode::int_modulo(10, 3),   exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatToInt,  label: "Float→Int", description: "f32 as i32",
        category: "Int",      width: 160.0,       height: 88.0,
        default_node: VisualNode::float_to_int,             exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::IntToFloat,  label: "Int→Float", description: "i32 as f32",
        category: "Int",      width: 160.0,       height: 88.0,
        default_node: VisualNode::int_to_float,             exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::IntGreater,  label: "Int >",   description: "a > b → bool",
        category: "Int",      width: 168.0,     height: 104.0,
        default_node: || VisualNode::int_greater(1, 0),   exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::IntLess,     label: "Int <",   description: "a < b → bool",
        category: "Int",      width: 168.0,     height: 104.0,
        default_node: || VisualNode::int_less(0, 1),      exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::IntEqual,    label: "Int =",   description: "a == b → bool",
        category: "Int",      width: 168.0,     height: 104.0,
        default_node: || VisualNode::int_equal(0, 0),     exec: None,
    });
}
