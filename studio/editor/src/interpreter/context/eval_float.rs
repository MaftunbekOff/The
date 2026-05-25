//! Float and Vec3 evaluation for pure nodes.

use twelfth_visual_blueprint::nodes::NodeKind;

use super::{PieExecContext, PieValue};

impl PieExecContext<'_> {
    pub(super) fn eval_float_out(&self, node_id: usize, pin: &'static str) -> f32 {
        if let Some(PieValue::Float(v)) = self.data.get(&(node_id, pin)) {
            return *v;
        }
        let Some(node) = self.graph.node(node_id) else { return 0.0 };
        let node = node.clone();
        match node.kind {
            NodeKind::FloatAdd =>
                self.resolve_float_in(node_id, "a", &node)
                    + self.resolve_float_in(node_id, "b", &node),
            NodeKind::FloatSubtract =>
                self.resolve_float_in(node_id, "a", &node)
                    - self.resolve_float_in(node_id, "b", &node),
            NodeKind::FloatMultiply =>
                self.resolve_float_in(node_id, "a", &node)
                    * self.resolve_float_in(node_id, "b", &node),
            NodeKind::FloatDivide => {
                let b = self.resolve_float_in(node_id, "b", &node);
                if b == 0.0 { 0.0 } else { self.resolve_float_in(node_id, "a", &node) / b }
            }
            NodeKind::FloatMin =>
                self.resolve_float_in(node_id, "a", &node)
                    .min(self.resolve_float_in(node_id, "b", &node)),
            NodeKind::FloatMax =>
                self.resolve_float_in(node_id, "a", &node)
                    .max(self.resolve_float_in(node_id, "b", &node)),
            NodeKind::FloatPow =>
                self.resolve_float_in(node_id, "base", &node)
                    .powf(self.resolve_float_in(node_id, "exp", &node)),
            NodeKind::FloatNegate => -self.resolve_float_in(node_id, "a", &node),
            NodeKind::FloatAbs    =>  self.resolve_float_in(node_id, "a", &node).abs(),
            NodeKind::FloatSqrt   =>  self.resolve_float_in(node_id, "a", &node).sqrt(),
            NodeKind::FloatSin    =>  self.resolve_float_in(node_id, "a", &node).sin(),
            NodeKind::FloatCos    =>  self.resolve_float_in(node_id, "a", &node).cos(),
            NodeKind::FloatFloor  =>  self.resolve_float_in(node_id, "a", &node).floor(),
            NodeKind::FloatCeil   =>  self.resolve_float_in(node_id, "a", &node).ceil(),
            NodeKind::FloatRound  =>  self.resolve_float_in(node_id, "a", &node).round(),
            NodeKind::FloatLerp => {
                let a = self.resolve_float_in(node_id, "a", &node);
                let b = self.resolve_float_in(node_id, "b", &node);
                let t = self.resolve_float_in(node_id, "t", &node);
                a + (b - a) * t
            }
            NodeKind::FloatClamp => {
                let v   = self.resolve_float_in(node_id, "value", &node);
                let min = self.resolve_float_in(node_id, "min",   &node);
                let max = self.resolve_float_in(node_id, "max",   &node);
                v.clamp(min, max)
            }
            NodeKind::Vec3Length => {
                let v = self.resolve_vec3_in(node_id, "vec", &node);
                (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt()
            }
            NodeKind::Vec3Dot => {
                let a = self.resolve_vec3_in(node_id, "a", &node);
                let b = self.resolve_vec3_in(node_id, "b", &node);
                a[0]*b[0] + a[1]*b[1] + a[2]*b[2]
            }
            NodeKind::EventTick | NodeKind::EventFixedTick if pin == "delta_time" => self.delta_time,
            NodeKind::IntToFloat =>
                self.resolve_int_in(node_id, "int_val", &node) as f32,

            // ── Variables ─────────────────────────────────────────────────────
            NodeKind::GetFloatVar => {
                let name = node.string_a.as_deref().unwrap_or("");
                if let Some(PieValue::Float(v)) = self.blackboard.vars.get(name) {
                    *v
                } else {
                    0.0
                }
            }

            // ── Float Collections ─────────────────────────────────────────────
            NodeKind::FloatArrayGet => {
                let array_name = node.string_a.as_deref().unwrap_or("");
                let idx = self.resolve_int_in(node_id, "index", &node) as usize;
                if let Some(PieValue::FloatArray(arr)) = self.blackboard.vars.get(array_name) {
                    arr.get(idx).copied().unwrap_or(0.0)
                } else {
                    0.0
                }
            }

            // ── Helpers ───────────────────────────────────────────────────────
            NodeKind::RandomFloat => {
                let min_v = self.resolve_float_in(node_id, "min", &node);
                let max_v = self.resolve_float_in(node_id, "max", &node);
                if min_v >= max_v { return min_v; }
                let seed = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.subsec_nanos())
                    .unwrap_or(42)
                    .wrapping_add(node_id as u32 * 2654435761);
                let normalized = (seed as f32) / (u32::MAX as f32);
                min_v + (max_v - min_v) * normalized
            }
            NodeKind::GetGameTime => self.game_time,

            // ── Select ternary ─────────────────────────────────────────────────
            NodeKind::SelectFloat => {
                let cond = self.resolve_bool_in(node_id, "condition", &node);
                if cond {
                    self.resolve_float_in(node_id, "value_a", &node)
                } else {
                    self.resolve_float_in(node_id, "value_b", &node)
                }
            }

            // ── Vec2 derived floats ────────────────────────────────────────────
            NodeKind::Vec2Length => {
                let v = self.resolve_vec2_in(node_id, "vec", &node);
                (v[0]*v[0] + v[1]*v[1]).sqrt()
            }
            NodeKind::Vec2Dot => {
                let a = self.resolve_vec2_in(node_id, "a", &node);
                let b = self.resolve_vec2_in(node_id, "b", &node);
                a[0]*b[0] + a[1]*b[1]
            }
            NodeKind::Vec2X => {
                self.resolve_vec2_in(node_id, "vec", &node)[0]
            }
            NodeKind::Vec2Y => {
                self.resolve_vec2_in(node_id, "vec", &node)[1]
            }

            _ => 0.0,
        }
    }

    pub(super) fn eval_vec3_out(&self, node_id: usize, pin: &'static str) -> [f32; 3] {
        if let Some(PieValue::Vec3(v)) = self.data.get(&(node_id, pin)) {
            return *v;
        }
        let Some(node) = self.graph.node(node_id) else { return [0.0; 3] };
        let node = node.clone();
        match node.kind {
            NodeKind::Vec3Make => [
                self.resolve_float_in(node_id, "x", &node),
                self.resolve_float_in(node_id, "y", &node),
                self.resolve_float_in(node_id, "z", &node),
            ],
            NodeKind::Vec3Add => {
                let a = self.resolve_vec3_in(node_id, "a", &node);
                let b = self.resolve_vec3_in(node_id, "b", &node);
                [a[0]+b[0], a[1]+b[1], a[2]+b[2]]
            }
            NodeKind::Vec3Sub => {
                let a = self.resolve_vec3_in(node_id, "a", &node);
                let b = self.resolve_vec3_in(node_id, "b", &node);
                [a[0]-b[0], a[1]-b[1], a[2]-b[2]]
            }
            NodeKind::Vec3Scale => {
                let v = self.resolve_vec3_in(node_id, "vec", &node);
                let s = self.resolve_float_in(node_id, "scale", &node);
                [v[0]*s, v[1]*s, v[2]*s]
            }
            NodeKind::Vec3Normalize => {
                let v = self.resolve_vec3_in(node_id, "vec", &node);
                let len = (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt();
                if len < 1e-9 { [0.0; 3] } else { [v[0]/len, v[1]/len, v[2]/len] }
            }
            NodeKind::Vec3Lerp => {
                let a = self.resolve_vec3_in(node_id, "a", &node);
                let b = self.resolve_vec3_in(node_id, "b", &node);
                let t = self.resolve_float_in(node_id, "t", &node);
                [
                    a[0] + (b[0]-a[0]) * t,
                    a[1] + (b[1]-a[1]) * t,
                    a[2] + (b[2]-a[2]) * t,
                ]
            }
            _ => [0.0; 3],
        }
    }
}
