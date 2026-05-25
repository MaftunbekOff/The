//! Input kategoriyasi: IsKeyPressed, IsKeyJustPressed.

use twelfth_visual_blueprint::nodes::{NodeKind as K, VisualNode};

use crate::interpreter::context::PieExecContext;
use crate::nodes::descriptor::{ExecBehavior, ExecFlow, NodeDescriptor, NodeRegistry};

// Input nodes — pure data (exec_fn = None, exec = None).
// Hisob `context.rs::eval_bool_out` da amalga oshiriladi.

pub(crate) fn register(r: &mut NodeRegistry) {
    r.register(NodeDescriptor {
        kind: K::IsKeyPressed,
        label: "Is Key Pressed",
        description: "Tugma bosib turilganmi → bool",
        category: "Input",
        width: 220.0, height: 104.0,
        default_node: || VisualNode::is_key_pressed("Space"),
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::IsKeyJustPressed,
        label: "Is Key Just Pressed",
        description: "Tugma shu frameda bosildimi → bool",
        category: "Input",
        width: 220.0, height: 104.0,
        default_node: || VisualNode::is_key_just_pressed("Space"),
        exec: None,
    });
}

// Unused struct to satisfy trait bound in descriptor if needed later
#[allow(dead_code)]
struct ExecInputPlaceholder;
impl ExecBehavior for ExecInputPlaceholder {
    fn run(&self, _ctx: &mut PieExecContext<'_>, _node_id: usize) -> Result<ExecFlow, String> {
        unreachable!()
    }
}
