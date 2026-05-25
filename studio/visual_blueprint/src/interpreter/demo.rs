//! Namuna grafiklari (test va makro codegen uchun).

use crate::ast::{ExecLink, VisualScriptGraph};
use crate::nodes::VisualNode;
use crate::pins::DataLink;

/// `EventBeginPlay` → `PrintLog` → `AddGold` (chiziqli, v2).
pub fn demo_treasury_graph() -> VisualScriptGraph {
    VisualScriptGraph::new(
        "TreasuryDemo",
        vec![
            (0, VisualNode::event_begin_play()),
            (1, VisualNode::print_log("Code Log: Tizim muvaffaqiyatli yuklandi!")),
            (2, VisualNode::add_gold(500.0)),
        ],
        vec![ExecLink::exec_out(0, 1), ExecLink::exec_out(1, 2)],
        vec![],
    )
}

/// v3 namuna: `Gold > 1000` → Rich / Need more gold.
pub fn demo_rich_branch_graph() -> VisualScriptGraph {
    VisualScriptGraph::new(
        "RichCheck",
        vec![
            (0, VisualNode::event_begin_play()),
            (1, VisualNode::check_gold_amount(1000.0)),
            (2, VisualNode::branch()),
            (3, VisualNode::print_log("Rich!")),
            (4, VisualNode::print_log("Need more gold!")),
        ],
        vec![
            ExecLink::exec_out(0, 1),
            ExecLink::exec_out(1, 2),
            ExecLink::branch_true(2, 3),
            ExecLink::branch_false(2, 4),
        ],
        vec![DataLink {
            from_node_id: 1,
            from_pin: "result",
            to_node_id: 2,
            to_pin: "condition",
        }],
    )
}
