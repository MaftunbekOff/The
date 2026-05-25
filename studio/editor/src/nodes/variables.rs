//! Script Variables — Set/Get typed variables in the blackboard.
//!
//! Variable nomi `string_a` da saqlanadi.
//! Blackboard PIE sessiyasi davomida saqlanadi, Stop da tozalanadi.

use twelfth_visual_blueprint::nodes::{NodeKind as K, VisualNode};

use crate::interpreter::context::PieExecContext;
use crate::interpreter::value::PieValue;
use crate::nodes::descriptor::{ExecBehavior, ExecFlow, NodeDescriptor, NodeRegistry};

// ── SetFloatVar ───────────────────────────────────────────────────────────────
struct ExecSetFloatVar;
impl ExecBehavior for ExecSetFloatVar {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).ok_or("SetFloatVar: node topilmadi")?.clone();
        let name = node.string_a.clone().unwrap_or_default();
        let value = ctx.resolve_float_in(node_id, "value", &node);
        ctx.blackboard.vars.insert(name, PieValue::Float(value));
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_SET_FLOAT_VAR: ExecSetFloatVar = ExecSetFloatVar;

// ── SetBoolVar ────────────────────────────────────────────────────────────────
struct ExecSetBoolVar;
impl ExecBehavior for ExecSetBoolVar {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).ok_or("SetBoolVar: node topilmadi")?.clone();
        let name = node.string_a.clone().unwrap_or_default();
        let value = ctx.resolve_bool_in(node_id, "value", &node);
        ctx.blackboard.vars.insert(name, PieValue::Bool(value));
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_SET_BOOL_VAR: ExecSetBoolVar = ExecSetBoolVar;

// ── SetIntVar ─────────────────────────────────────────────────────────────────
struct ExecSetIntVar;
impl ExecBehavior for ExecSetIntVar {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).ok_or("SetIntVar: node topilmadi")?.clone();
        let name = node.string_a.clone().unwrap_or_default();
        let value = ctx.resolve_int_in(node_id, "int_val", &node);
        ctx.blackboard.vars.insert(name, PieValue::Int(value));
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_SET_INT_VAR: ExecSetIntVar = ExecSetIntVar;

// ── SetStringVar ──────────────────────────────────────────────────────────────
struct ExecSetStringVar;
impl ExecBehavior for ExecSetStringVar {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).ok_or("SetStringVar: node topilmadi")?.clone();
        let name = node.string_a.clone().unwrap_or_default();
        let value = ctx.resolve_string_in(node_id, "text", &node);
        ctx.blackboard.vars.insert(name, PieValue::Str(value));
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_SET_STRING_VAR: ExecSetStringVar = ExecSetStringVar;

// ── register() ───────────────────────────────────────────────────────────────

pub(crate) fn register(r: &mut NodeRegistry) {
    r.register(NodeDescriptor {
        kind: K::SetFloatVar,
        label: "Set Float Var",
        description: "Float o'zgaruvchi qiymatini o'rnatadi",
        category: "Variables",
        width: 200.0, height: 80.0,
        default_node: || VisualNode::set_float_var("my_float"),
        exec: Some(&EXEC_SET_FLOAT_VAR),
    });
    r.register(NodeDescriptor {
        kind: K::GetFloatVar,
        label: "Get Float Var",
        description: "Float o'zgaruvchi qiymatini oladi (sof hisob)",
        category: "Variables",
        width: 180.0, height: 64.0,
        default_node: || VisualNode::get_float_var("my_float"),
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::SetBoolVar,
        label: "Set Bool Var",
        description: "Bool o'zgaruvchi qiymatini o'rnatadi",
        category: "Variables",
        width: 200.0, height: 80.0,
        default_node: || VisualNode::set_bool_var("my_bool"),
        exec: Some(&EXEC_SET_BOOL_VAR),
    });
    r.register(NodeDescriptor {
        kind: K::GetBoolVar,
        label: "Get Bool Var",
        description: "Bool o'zgaruvchi qiymatini oladi (sof hisob)",
        category: "Variables",
        width: 180.0, height: 64.0,
        default_node: || VisualNode::get_bool_var("my_bool"),
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::SetIntVar,
        label: "Set Int Var",
        description: "Int o'zgaruvchi qiymatini o'rnatadi",
        category: "Variables",
        width: 200.0, height: 80.0,
        default_node: || VisualNode::set_int_var("my_int"),
        exec: Some(&EXEC_SET_INT_VAR),
    });
    r.register(NodeDescriptor {
        kind: K::GetIntVar,
        label: "Get Int Var",
        description: "Int o'zgaruvchi qiymatini oladi (sof hisob)",
        category: "Variables",
        width: 180.0, height: 64.0,
        default_node: || VisualNode::get_int_var("my_int"),
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::SetStringVar,
        label: "Set String Var",
        description: "String o'zgaruvchi qiymatini o'rnatadi",
        category: "Variables",
        width: 210.0, height: 80.0,
        default_node: || VisualNode::set_string_var("my_string"),
        exec: Some(&EXEC_SET_STRING_VAR),
    });
    r.register(NodeDescriptor {
        kind: K::GetStringVar,
        label: "Get String Var",
        description: "String o'zgaruvchi qiymatini oladi (sof hisob)",
        category: "Variables",
        width: 190.0, height: 64.0,
        default_node: || VisualNode::get_string_var("my_string"),
        exec: None,
    });
}
