//! Transform kategoriyasi: GetTranslation, SetTranslation, Translate,
//! GetScale, SetScale, GetRotationEuler, SetRotationEuler.

use twelfth_visual_blueprint::nodes::{NodeKind as K, VisualNode};

use crate::interpreter::context::PieExecContext;
use crate::interpreter::PieTransformOp;
use crate::nodes::descriptor::{ExecBehavior, ExecFlow, NodeDescriptor, NodeRegistry};

// ── SetTranslation ────────────────────────────────────────────────────────────
struct ExecSetTranslation;
impl ExecBehavior for ExecSetTranslation {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        if let Some(slot) = ctx.resolve_entity_in(node_id, "entity") {
            let pos = ctx.resolve_vec3_entity(node_id, "position");
            ctx.transform_ops.push(PieTransformOp::SetTranslation { slot, pos });
            ctx.log(format!("[PIE] set_translation slot={slot} → {pos:?}"));
        }
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_SET_TRANSLATION: ExecSetTranslation = ExecSetTranslation;

// ── Translate ─────────────────────────────────────────────────────────────────
struct ExecTranslate;
impl ExecBehavior for ExecTranslate {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        if let Some(slot) = ctx.resolve_entity_in(node_id, "entity") {
            let delta = ctx.resolve_vec3_entity(node_id, "delta");
            ctx.transform_ops.push(PieTransformOp::Translate { slot, delta });
            ctx.log(format!("[PIE] translate slot={slot} delta={delta:?}"));
        }
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_TRANSLATE: ExecTranslate = ExecTranslate;

// ── SetScale ──────────────────────────────────────────────────────────────────
struct ExecSetScale;
impl ExecBehavior for ExecSetScale {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        if let Some(slot) = ctx.resolve_entity_in(node_id, "entity") {
            let scale = ctx.resolve_vec3_entity(node_id, "scale");
            ctx.transform_ops.push(PieTransformOp::SetScale { slot, scale });
            ctx.log(format!("[PIE] set_scale slot={slot} → {scale:?}"));
        }
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_SET_SCALE: ExecSetScale = ExecSetScale;

// ── SetRotationEuler ──────────────────────────────────────────────────────────
struct ExecSetRotationEuler;
impl ExecBehavior for ExecSetRotationEuler {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        if let Some(slot) = ctx.resolve_entity_in(node_id, "entity") {
            let euler = ctx.resolve_vec3_entity(node_id, "rotation");
            ctx.transform_ops.push(PieTransformOp::SetRotationEuler { slot, euler });
            ctx.log(format!("[PIE] set_rotation_euler slot={slot} → {euler:?}"));
        }
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_SET_ROTATION_EULER: ExecSetRotationEuler = ExecSetRotationEuler;

// ── register() ───────────────────────────────────────────────────────────────

pub(crate) fn register(r: &mut NodeRegistry) {
    // Sof-hisob (read) tugunlar — exec = None
    r.register(NodeDescriptor {
        kind: K::GetTranslation,
        label: "Get Translation",
        description: "Entity.translation → Vec3",
        category: "Transform",
        width: 192.0, height: 88.0,
        default_node: VisualNode::get_translation,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::GetScale,
        label: "Get Scale",
        description: "Entity.scale → Vec3",
        category: "Transform",
        width: 192.0, height: 88.0,
        default_node: VisualNode::get_scale,
        exec: None,
    });
    r.register(NodeDescriptor {
        kind: K::GetRotationEuler,
        label: "Get Rotation",
        description: "Entity.rotation Euler → Vec3",
        category: "Transform",
        width: 192.0, height: 88.0,
        default_node: VisualNode::get_rotation_euler,
        exec: None,
    });

    // Exec (write) tugunlar
    r.register(NodeDescriptor {
        kind: K::SetTranslation,
        label: "Set Translation",
        description: "Entity.translation = Vec3 (exec)",
        category: "Transform",
        width: 212.0, height: 112.0,
        default_node: VisualNode::set_translation,
        exec: Some(&EXEC_SET_TRANSLATION),
    });
    r.register(NodeDescriptor {
        kind: K::Translate,
        label: "Translate",
        description: "Entity ni delta Vec3 ga siljitadi (exec)",
        category: "Transform",
        width: 212.0, height: 112.0,
        default_node: VisualNode::translate_entity,
        exec: Some(&EXEC_TRANSLATE),
    });
    r.register(NodeDescriptor {
        kind: K::SetScale,
        label: "Set Scale",
        description: "Entity.scale = Vec3 (exec)",
        category: "Transform",
        width: 212.0, height: 112.0,
        default_node: VisualNode::set_scale,
        exec: Some(&EXEC_SET_SCALE),
    });
    r.register(NodeDescriptor {
        kind: K::SetRotationEuler,
        label: "Set Rotation",
        description: "Entity.rotation = Euler Vec3 (exec)",
        category: "Transform",
        width: 212.0, height: 112.0,
        default_node: VisualNode::set_rotation_euler,
        exec: Some(&EXEC_SET_ROTATION_EULER),
    });
}
