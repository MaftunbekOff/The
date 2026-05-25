//! Float + Int Collections — FloatArray va IntArray operatsiyalari.
//!
//! Massiv nomi `string_a` da saqlanadi.
//! Massivlar blackboard da `PieValue::FloatArray` / `PieValue::IntArray` sifatida saqlanadi.

use twelfth_visual_blueprint::nodes::{NodeKind as K, VisualNode};

use crate::interpreter::context::PieExecContext;
use crate::interpreter::value::PieValue;
use crate::nodes::descriptor::{ExecBehavior, ExecFlow, NodeDescriptor, NodeRegistry};

// ── FloatArrayPush ────────────────────────────────────────────────────────────
struct ExecFloatArrayPush;
impl ExecBehavior for ExecFloatArrayPush {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).ok_or("FloatArrayPush: node topilmadi")?.clone();
        let array_name = node.string_a.clone().unwrap_or_default();
        let value = ctx.resolve_float_in(node_id, "value", &node);
        let arr = ctx.blackboard.vars
            .entry(array_name.clone())
            .or_insert_with(|| PieValue::FloatArray(Vec::new()));
        if let PieValue::FloatArray(v) = arr {
            v.push(value);
            let len = v.len();
            ctx.log(format!("[PIE] array '{array_name}' ← {value} (uzunlik: {len})"));
        }
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_FLOAT_ARRAY_PUSH: ExecFloatArrayPush = ExecFloatArrayPush;

// ── FloatArrayClear ───────────────────────────────────────────────────────────
struct ExecFloatArrayClear;
impl ExecBehavior for ExecFloatArrayClear {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).ok_or("FloatArrayClear: node topilmadi")?.clone();
        let array_name = node.string_a.clone().unwrap_or_default();
        ctx.blackboard.vars.insert(array_name.clone(), PieValue::FloatArray(Vec::new()));
        ctx.log(format!("[PIE] array '{array_name}' tozalandi"));
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_FLOAT_ARRAY_CLEAR: ExecFloatArrayClear = ExecFloatArrayClear;

// ── IntArrayPush ─────────────────────────────────────────────────────────────
struct ExecIntArrayPush;
impl ExecBehavior for ExecIntArrayPush {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).ok_or("IntArrayPush: node topilmadi")?.clone();
        let array_name = node.string_a.clone().unwrap_or_default();
        let value = ctx.resolve_int_in(node_id, "int_val", &node);
        let arr = ctx.blackboard.vars
            .entry(array_name.clone())
            .or_insert_with(|| PieValue::IntArray(Vec::new()));
        if let PieValue::IntArray(v) = arr {
            v.push(value);
            let len = v.len();
            ctx.log(format!("[PIE] int_array '{array_name}' ← {value} (uzunlik: {len})"));
        }
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_INT_ARRAY_PUSH: ExecIntArrayPush = ExecIntArrayPush;

// ── IntArrayClear ─────────────────────────────────────────────────────────────
struct ExecIntArrayClear;
impl ExecBehavior for ExecIntArrayClear {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).ok_or("IntArrayClear: node topilmadi")?.clone();
        let array_name = node.string_a.clone().unwrap_or_default();
        ctx.blackboard.vars.insert(array_name.clone(), PieValue::IntArray(Vec::new()));
        ctx.log(format!("[PIE] int_array '{array_name}' tozalandi"));
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_INT_ARRAY_CLEAR: ExecIntArrayClear = ExecIntArrayClear;

// ── StringArrayPush ───────────────────────────────────────────────────────────
struct ExecStringArrayPush;
impl ExecBehavior for ExecStringArrayPush {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).ok_or("StringArrayPush: node topilmadi")?.clone();
        let array_name = node.string_a.clone().unwrap_or_default();
        let value = ctx.resolve_string_in(node_id, "text", &node);
        let arr = ctx.blackboard.vars
            .entry(array_name.clone())
            .or_insert_with(|| PieValue::StringArray(Vec::new()));
        if let PieValue::StringArray(v) = arr {
            v.push(value.clone());
            let len = v.len();
            ctx.log(format!("[PIE] str_array '{array_name}' ← \"{value}\" (uzunlik: {len})"));
        }
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_STRING_ARRAY_PUSH: ExecStringArrayPush = ExecStringArrayPush;

// ── StringArrayClear ──────────────────────────────────────────────────────────
struct ExecStringArrayClear;
impl ExecBehavior for ExecStringArrayClear {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).ok_or("StringArrayClear: node topilmadi")?.clone();
        let array_name = node.string_a.clone().unwrap_or_default();
        ctx.blackboard.vars.insert(array_name.clone(), PieValue::StringArray(Vec::new()));
        ctx.log(format!("[PIE] str_array '{array_name}' tozalandi"));
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_STRING_ARRAY_CLEAR: ExecStringArrayClear = ExecStringArrayClear;

// ── register() ───────────────────────────────────────────────────────────────

pub(crate) fn register(r: &mut NodeRegistry) {
    r.register(NodeDescriptor {
        kind: K::FloatArrayPush,
        label: "Float Array Push",
        description: "Float massiviga element qo'shadi",
        category: "Collections",
        width: 210.0, height: 80.0,
        default_node: || VisualNode::float_array_push("my_array"),
        exec: Some(&EXEC_FLOAT_ARRAY_PUSH),
    });
    r.register(NodeDescriptor {
        kind: K::FloatArrayGet,
        label: "Float Array Get",
        description: "Float massividan element oladi (sof hisob)",
        category: "Collections",
        width: 200.0, height: 72.0,
        default_node: || VisualNode::float_array_get("my_array"),
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatArrayLength,
        label: "Float Array Length",
        description: "Float massivi uzunligini qaytaradi (sof hisob)",
        category: "Collections",
        width: 200.0, height: 64.0,
        default_node: || VisualNode::float_array_length("my_array"),
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::FloatArrayClear,
        label: "Float Array Clear",
        description: "Float massivini tozalaydi",
        category: "Collections",
        width: 200.0, height: 64.0,
        default_node: || VisualNode::float_array_clear("my_array"),
        exec: Some(&EXEC_FLOAT_ARRAY_CLEAR),
    });

    // ── Int Arrays ───────────────────────────────────────────────────────────
    r.register(NodeDescriptor {
        kind: K::IntArrayPush,
        label: "Int Array Push",
        description: "Int massiviga element qo'shadi",
        category: "Collections",
        width: 210.0, height: 80.0,
        default_node: || VisualNode::int_array_push("my_int_array"),
        exec: Some(&EXEC_INT_ARRAY_PUSH),
    });
    r.register(NodeDescriptor {
        kind: K::IntArrayGet,
        label: "Int Array Get",
        description: "Int massividan element oladi (sof hisob)",
        category: "Collections",
        width: 200.0, height: 72.0,
        default_node: || VisualNode::int_array_get("my_int_array"),
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::IntArrayLength,
        label: "Int Array Length",
        description: "Int massivi uzunligini qaytaradi (sof hisob)",
        category: "Collections",
        width: 200.0, height: 64.0,
        default_node: || VisualNode::int_array_length("my_int_array"),
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::IntArrayClear,
        label: "Int Array Clear",
        description: "Int massivini tozalaydi",
        category: "Collections",
        width: 200.0, height: 64.0,
        default_node: || VisualNode::int_array_clear("my_int_array"),
        exec: Some(&EXEC_INT_ARRAY_CLEAR),
    });

    // ── String Arrays ────────────────────────────────────────────────────────
    r.register(NodeDescriptor {
        kind: K::StringArrayPush,
        label: "String Array Push",
        description: "String massiviga element qo'shadi",
        category: "Collections",
        width: 220.0, height: 80.0,
        default_node: || VisualNode::string_array_push("my_str_array"),
        exec: Some(&EXEC_STRING_ARRAY_PUSH),
    });
    r.register(NodeDescriptor {
        kind: K::StringArrayGet,
        label: "String Array Get",
        description: "String massividan element oladi (sof hisob)",
        category: "Collections",
        width: 210.0, height: 72.0,
        default_node: || VisualNode::string_array_get("my_str_array"),
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::StringArrayLength,
        label: "String Array Length",
        description: "String massivi uzunligini qaytaradi (sof hisob)",
        category: "Collections",
        width: 210.0, height: 64.0,
        default_node: || VisualNode::string_array_length("my_str_array"),
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::StringArrayClear,
        label: "String Array Clear",
        description: "String massivini tozalaydi",
        category: "Collections",
        width: 210.0, height: 64.0,
        default_node: || VisualNode::string_array_clear("my_str_array"),
        exec: Some(&EXEC_STRING_ARRAY_CLEAR),
    });
}
