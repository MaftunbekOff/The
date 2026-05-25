//! Control flow kategoriyasi: Branch, Delay, PrintLog.

use twelfth_visual_blueprint::nodes::{NodeKind as K, VisualNode};

use crate::interpreter::context::PieExecContext;
use crate::nodes::descriptor::{ExecBehavior, ExecFlow, NodeDescriptor, NodeRegistry};

// ── Branch ────────────────────────────────────────────────────────────────────
struct ExecBranch;
impl ExecBehavior for ExecBranch {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).unwrap().clone();
        let cond = ctx.resolve_bool(node_id, &node);
        ctx.log(format!("[PIE] branch → {cond}"));
        Ok(ExecFlow::Pin(if cond { "true" } else { "false" }))
    }
}
static EXEC_BRANCH: ExecBranch = ExecBranch;

// ── PrintLog ──────────────────────────────────────────────────────────────────
struct ExecPrintLog;
impl ExecBehavior for ExecPrintLog {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).unwrap().clone();
        let msg = ctx.resolve_string(node_id, &node);
        bevy::log::info!("{msg}");
        ctx.log(format!("[PIE] log: {msg}"));
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_PRINT_LOG: ExecPrintLog = ExecPrintLog;

// ── Delay ─────────────────────────────────────────────────────────────────────
struct ExecDelay;
impl ExecBehavior for ExecDelay {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).unwrap().clone();
        let duration = node.delay_seconds.unwrap_or(1.0);
        for &successor in ctx.graph.exec_successors(node_id, "exec_out").iter() {
            ctx.delay_requests.push((duration, successor));
        }
        ctx.log(format!("[PIE] delay {duration:.2}s → keyingi frame(lar)da"));
        Ok(ExecFlow::Deferred)
    }
}
static EXEC_DELAY: ExecDelay = ExecDelay;

// ── register() ───────────────────────────────────────────────────────────────

pub(crate) fn register(r: &mut NodeRegistry) {
    r.register(NodeDescriptor {
        kind: K::Branch,
        label: "Branch",
        description: "if/else exec shox",
        category: "Control",
        width: 220.0, height: 128.0,
        default_node: VisualNode::branch,
        exec: Some(&EXEC_BRANCH),
    });
    r.register(NodeDescriptor {
        kind: K::PrintLog,
        label: "Print Log",
        description: "info! matn",
        category: "Control",
        width: 228.0, height: 148.0,
        default_node: || VisualNode::print_log("Hello"),
        exec: Some(&EXEC_PRINT_LOG),
    });
    r.register(NodeDescriptor {
        kind: K::Delay,
        label: "Delay",
        description: "Kutish (sekund)",
        category: "Control",
        width: 228.0, height: 104.0,
        default_node: || VisualNode::delay(0.2),
        exec: Some(&EXEC_DELAY),
    });
}
