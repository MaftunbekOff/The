//! Vec2 evaluation for pure nodes.

use twelfth_visual_blueprint::nodes::NodeKind;

use super::{PieExecContext, PieValue};

impl PieExecContext<'_> {
    pub(super) fn eval_vec2_out(&self, node_id: usize, _pin: &'static str) -> [f32; 2] {
        if let Some(PieValue::Vec2(v)) = self.data.get(&(node_id, _pin)) {
            return *v;
        }
        let Some(node) = self.graph.node(node_id) else { return [0.0; 2] };
        let node = node.clone();
        match node.kind {
            NodeKind::Vec2Make => [
                self.resolve_float_in(node_id, "x", &node),
                self.resolve_float_in(node_id, "y", &node),
            ],
            NodeKind::Vec2Add => {
                let a = self.resolve_vec2_in(node_id, "a", &node);
                let b = self.resolve_vec2_in(node_id, "b", &node);
                [a[0]+b[0], a[1]+b[1]]
            }
            NodeKind::Vec2Sub => {
                let a = self.resolve_vec2_in(node_id, "a", &node);
                let b = self.resolve_vec2_in(node_id, "b", &node);
                [a[0]-b[0], a[1]-b[1]]
            }
            NodeKind::Vec2Scale => {
                let v = self.resolve_vec2_in(node_id, "vec", &node);
                let s = self.resolve_float_in(node_id, "scale", &node);
                [v[0]*s, v[1]*s]
            }
            NodeKind::Vec2Normalize => {
                let v = self.resolve_vec2_in(node_id, "vec", &node);
                let len = (v[0]*v[0] + v[1]*v[1]).sqrt();
                if len < 1e-9 { [0.0; 2] } else { [v[0]/len, v[1]/len] }
            }
            _ => [0.0; 2],
        }
    }
}
