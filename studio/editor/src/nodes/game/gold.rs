//! Gold kategoriyasi: AddGold, CheckGoldAmount.

use twelfth_visual_blueprint::nodes::{NodeKind as K, VisualNode};

use crate::interpreter::context::PieExecContext;
use crate::nodes::descriptor::{ExecBehavior, ExecFlow, NodeDescriptor, NodeRegistry};

// ── AddGold ───────────────────────────────────────────────────────────────────
struct ExecAddGold;
impl ExecBehavior for ExecAddGold {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).unwrap().clone();
        let amount = ctx.resolve_float_in(node_id, "amount", &node);
        ctx.add_gold(amount);
        ctx.log(format!("[PIE] add_gold {amount:.2} → jami={:.2}", ctx.gold_value()));
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_ADD_GOLD: ExecAddGold = ExecAddGold;

// ── CheckGoldAmount ───────────────────────────────────────────────────────────
struct ExecCheckGoldAmount;
impl ExecBehavior for ExecCheckGoldAmount {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).unwrap().clone();
        let threshold = node.gold_threshold.unwrap_or(0.0);
        let result = ctx.gold_value() > threshold;
        ctx.set_bool(node_id, "result", result);
        ctx.log(format!(
            "[PIE] check_gold {threshold} → gold={} → {result}",
            ctx.gold_value()
        ));
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_CHECK_GOLD: ExecCheckGoldAmount = ExecCheckGoldAmount;

// ── register() ───────────────────────────────────────────────────────────────

pub(crate) fn register(r: &mut NodeRegistry) {
    r.register(NodeDescriptor {
        kind: K::AddGold,
        label: "Add Gold",
        description: "Gold.value +=",
        category: "Game",
        width: 228.0, height: 104.0,
        default_node: || VisualNode::add_gold(100.0),
        exec: Some(&EXEC_ADD_GOLD),
    });
    r.register(NodeDescriptor {
        kind: K::CheckGoldAmount,
        label: "Check Gold",
        description: "Gold > threshold → bool",
        category: "Game",
        width: 168.0, height: 104.0,
        default_node: || VisualNode::check_gold_amount(1000.0),
        exec: Some(&EXEC_CHECK_GOLD),
    });
}
