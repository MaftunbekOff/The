//! Loop and flow control nodes — ForEachFloat, WhileLoop, Sequence, DoOnce.

use twelfth_visual_blueprint::nodes::{NodeKind as K, VisualNode};

use crate::interpreter::context::PieExecContext;
use crate::interpreter::exec::execute_exec_node;
use crate::interpreter::value::PieValue;
use crate::nodes::descriptor::{ExecBehavior, ExecFlow, NodeDescriptor, NodeRegistry};

/// Maksimal iteratsiya soni — cheksiz loopni oldini oladi.
const MAX_LOOP_ITERATIONS: usize = 100_000;

// ── ForEachFloat ──────────────────────────────────────────────────────────────
struct ExecForEachFloat;
impl ExecBehavior for ExecForEachFloat {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).ok_or("ForEachFloat: node topilmadi")?.clone();
        let array_name = node.string_a.as_deref().unwrap_or("").to_string();

        // Massivni oldindan clone qilamiz — loop ichida ctx ni mut qilish uchun
        let items: Vec<f32> = match ctx.blackboard.vars.get(&array_name) {
            Some(PieValue::FloatArray(arr)) => arr.clone(),
            _ => Vec::new(),
        };

        for (i, &item) in items.iter().enumerate() {
            ctx.data.insert((node_id, "item"),  PieValue::Float(item));
            ctx.data.insert((node_id, "index"), PieValue::Int(i as i32));
            let children: Vec<usize> = ctx.graph.exec_successors(node_id, "loop_body");
            let mut broke = false;
            for child in children {
                match execute_exec_node(ctx, child) {
                    Err(e) if e == "__break__"    => { broke = true; break; }
                    Err(e) if e == "__continue__" => break,  // skip rest of body, next iteration
                    other => other?,
                }
            }
            if broke { break; }
        }
        // Completed pinni davom ettirish
        Ok(ExecFlow::Pin("completed"))
    }
}
static EXEC_FOR_EACH_FLOAT: ExecForEachFloat = ExecForEachFloat;

// ── WhileLoop ─────────────────────────────────────────────────────────────────
struct ExecWhileLoop;
impl ExecBehavior for ExecWhileLoop {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let mut iterations = 0usize;
        loop {
            if iterations >= MAX_LOOP_ITERATIONS {
                ctx.log(format!(
                    "[PIE] WhileLoop #{node_id}: maksimal iteratsiya ({MAX_LOOP_ITERATIONS}) ga yetdi"
                ));
                break;
            }
            let node = ctx.graph.node(node_id)
                .ok_or("WhileLoop: node topilmadi")?.clone();
            let cond = ctx.resolve_bool_in(node_id, "condition", &node);
            if !cond { break; }
            iterations += 1;
            let children: Vec<usize> = ctx.graph.exec_successors(node_id, "loop_body");
            let mut broke = false;
            for child in children {
                match execute_exec_node(ctx, child) {
                    Err(e) if e == "__break__"    => { broke = true; break; }
                    Err(e) if e == "__continue__" => break,
                    other => other?,
                }
            }
            if broke { break; }
        }
        Ok(ExecFlow::Pin("completed"))
    }
}
static EXEC_WHILE_LOOP: ExecWhileLoop = ExecWhileLoop;

// ── Sequence ──────────────────────────────────────────────────────────────────
struct ExecSequence;
impl ExecBehavior for ExecSequence {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        // then_0, then_1 ketma-ket bajariladi, keyin exec_out
        let mut targets = ctx.graph.exec_successors(node_id, "then_0");
        targets.extend(ctx.graph.exec_successors(node_id, "then_1"));
        Ok(ExecFlow::ThenExecOut(targets))
    }
}
static EXEC_SEQUENCE: ExecSequence = ExecSequence;

// ── DoOnce ────────────────────────────────────────────────────────────────────
struct ExecDoOnce;
impl ExecBehavior for ExecDoOnce {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let flag_key = format!("__doonce_{node_id}__");
        if let Some(PieValue::Bool(true)) = ctx.blackboard.vars.get(&flag_key) {
            // Allaqachon bajarilgan — hech narsa qilmaymiz
            return Ok(ExecFlow::Deferred);
        }
        ctx.blackboard.vars.insert(flag_key, PieValue::Bool(true));
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_DO_ONCE: ExecDoOnce = ExecDoOnce;

// ── BreakLoop ─────────────────────────────────────────────────────────────────
struct ExecBreakLoop;
impl ExecBehavior for ExecBreakLoop {
    fn run(&self, _ctx: &mut PieExecContext<'_>, _node_id: usize) -> Result<ExecFlow, String> {
        Ok(ExecFlow::Break)
    }
}
static EXEC_BREAK_LOOP: ExecBreakLoop = ExecBreakLoop;

// ── ContinueLoop ──────────────────────────────────────────────────────────────
struct ExecContinueLoop;
impl ExecBehavior for ExecContinueLoop {
    fn run(&self, _ctx: &mut PieExecContext<'_>, _node_id: usize) -> Result<ExecFlow, String> {
        Ok(ExecFlow::Continue)
    }
}
static EXEC_CONTINUE_LOOP: ExecContinueLoop = ExecContinueLoop;

// ── ResetDoOnce ───────────────────────────────────────────────────────────────
struct ExecResetDoOnce;
impl ExecBehavior for ExecResetDoOnce {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        // Reset the do-once flag for the targeted DoOnce node.
        // Convention: string_a holds the DoOnce node_id as string, or we reset all if empty.
        let node = ctx.graph.node(node_id).ok_or("ResetDoOnce: node topilmadi")?.clone();
        if let Some(target_id_str) = &node.string_a {
            if let Ok(tid) = target_id_str.parse::<usize>() {
                let flag_key = format!("__doonce_{tid}__");
                ctx.blackboard.vars.remove(&flag_key);
            }
        }
        Ok(ExecFlow::ResetDone)
    }
}
static EXEC_RESET_DO_ONCE: ExecResetDoOnce = ExecResetDoOnce;

// ── register() ───────────────────────────────────────────────────────────────

pub(crate) fn register(r: &mut NodeRegistry) {
    r.register(NodeDescriptor {
        kind: K::ForEachFloat,
        label: "For Each Float",
        description: "Float massivi bo'yicha iteratsiya",
        category: "Loops",
        width: 220.0, height: 96.0,
        default_node: || VisualNode::for_each_float("my_array"),
        exec: Some(&EXEC_FOR_EACH_FLOAT),
    });
    r.register(NodeDescriptor {
        kind: K::WhileLoop,
        label: "While Loop",
        description: "Shart true bo'lguncha takrorlanadi",
        category: "Loops",
        width: 210.0, height: 80.0,
        default_node: VisualNode::while_loop,
        exec: Some(&EXEC_WHILE_LOOP),
    });
    r.register(NodeDescriptor {
        kind: K::Sequence,
        label: "Sequence",
        description: "then_0 → then_1 → exec_out ketma-ket bajaradi",
        category: "Flow",
        width: 180.0, height: 80.0,
        default_node: VisualNode::sequence,
        exec: Some(&EXEC_SEQUENCE),
    });
    r.register(NodeDescriptor {
        kind: K::DoOnce,
        label: "Do Once",
        description: "Faqat birinchi marta exec_out ni ishga tushiradi",
        category: "Flow",
        width: 180.0, height: 64.0,
        default_node: VisualNode::do_once,
        exec: Some(&EXEC_DO_ONCE),
    });
    r.register(NodeDescriptor {
        kind: K::BreakLoop,
        label: "Break Loop",
        description: "Joriy loopdan chiqadi (ForEachFloat/WhileLoop)",
        category: "Loops",
        width: 160.0, height: 56.0,
        default_node: VisualNode::break_loop,
        exec: Some(&EXEC_BREAK_LOOP),
    });
    r.register(NodeDescriptor {
        kind: K::ContinueLoop,
        label: "Continue Loop",
        description: "Joriy iteratsiyani o'tkazib keyingisiga o'tadi",
        category: "Loops",
        width: 170.0, height: 56.0,
        default_node: VisualNode::continue_loop,
        exec: Some(&EXEC_CONTINUE_LOOP),
    });
    r.register(NodeDescriptor {
        kind: K::ResetDoOnce,
        label: "Reset DoOnce",
        description: "DoOnce tugunini reset qiladi — qayta ishga tushirish uchun",
        category: "Flow",
        width: 190.0, height: 64.0,
        default_node: VisualNode::reset_do_once,
        exec: Some(&EXEC_RESET_DO_ONCE),
    });
}
