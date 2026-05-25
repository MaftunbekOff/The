//! Entity kategoriyasi: SpawnEntity, DestroyEntity, GetNamedEntity.

use twelfth_visual_blueprint::nodes::{NodeKind as K, VisualNode};

use crate::interpreter::context::PieExecContext;
use crate::interpreter::{PieEntitySpawnRequest};
use crate::nodes::descriptor::{ExecBehavior, ExecFlow, NodeDescriptor, NodeRegistry};

// ── SpawnEntity ───────────────────────────────────────────────────────────────
struct ExecSpawnEntity;
impl ExecBehavior for ExecSpawnEntity {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        // Slotni mahalliy counter dan olamiz — entity_table immutable bo'lsa ham ishlaydi
        let slot = ctx.next_entity_slot;
        ctx.next_entity_slot += 1;
        // Entity nomi — bo'lsa ro'yxatga olish uchun request ga kiritamiz
        let name = ctx.graph.node(node_id).and_then(|n| n.entity_name.clone());
        ctx.entity_spawns.push(PieEntitySpawnRequest { node_id, slot, name });
        ctx.set_entity(node_id, "entity", slot);
        ctx.log(format!("[PIE] spawn_entity → slot={slot}"));
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_SPAWN_ENTITY: ExecSpawnEntity = ExecSpawnEntity;

// ── DestroyEntity ─────────────────────────────────────────────────────────────
struct ExecDestroyEntity;
impl ExecBehavior for ExecDestroyEntity {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        if let Some(slot) = ctx.resolve_entity_in(node_id, "entity") {
            ctx.entity_despawns.push(slot);
            ctx.log(format!("[PIE] destroy_entity slot={slot}"));
        }
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_DESTROY_ENTITY: ExecDestroyEntity = ExecDestroyEntity;

// ── GetNamedEntity — sof-hisob (exec = None) ─────────────────────────────────
// Hisob `context.rs::eval_entity_out` da amalga oshiriladi.

// ── register() ───────────────────────────────────────────────────────────────

pub(crate) fn register(r: &mut NodeRegistry) {
    r.register(NodeDescriptor {
        kind: K::SpawnEntity,
        label: "Spawn Entity",
        description: "Yangi entity yaratadi (exec)",
        category: "Entity",
        width: 200.0, height: 80.0,
        default_node: VisualNode::spawn_entity,
        exec: Some(&EXEC_SPAWN_ENTITY),
    });
    r.register(NodeDescriptor {
        kind: K::DestroyEntity,
        label: "Destroy Entity",
        description: "Entity ni o'chiradi (exec)",
        category: "Entity",
        width: 200.0, height: 88.0,
        default_node: VisualNode::destroy_entity,
        exec: Some(&EXEC_DESTROY_ENTITY),
    });
    r.register(NodeDescriptor {
        kind: K::GetNamedEntity,
        label: "Get Named Entity",
        description: "Nom bo'yicha entity → slot",
        category: "Entity",
        width: 212.0, height: 88.0,
        default_node: || VisualNode::get_named_entity("player"),
        exec: None,
    });
}
