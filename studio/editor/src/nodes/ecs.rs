//! ECS nodes: QueryAllEntities, ForEachEntity, AddTag, RemoveTag, HasTag,
//! GetEntityName, EntityArrayGet, EntityArrayLength, EventFixedTick, EventOnSpawn.
//!
//! PIE'da "tag" = ScriptActor.vars ichidagi bool qiymat.
//! AOT'da = marker komponent.

use twelfth_visual_blueprint::nodes::{NodeKind as K};

use crate::interpreter::context::PieExecContext;
use crate::interpreter::exec::execute_exec_node;
use crate::interpreter::value::PieValue;
use crate::nodes::descriptor::{ExecBehavior, ExecFlow, NodeDescriptor, NodeRegistry};

const MAX_LOOP_ITERATIONS: usize = 100_000;

// ── QueryAllEntities (pure, no exec) ─────────────────────────────────────────
// eval_int_out da hal qilinadi (entity_table.entities.len())

// ── ForEachEntity ─────────────────────────────────────────────────────────────
struct ExecForEachEntity;
impl ExecBehavior for ExecForEachEntity {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let _node = ctx.graph.node(node_id).ok_or("ForEachEntity: node topilmadi")?.clone();
        // Barcha entity slotlarini to'playmiz
        let slots: Vec<u32> = ctx.entity_table.entities.keys().copied().collect();

        let mut iterations = 0usize;
        for (i, &slot) in slots.iter().enumerate() {
            if iterations >= MAX_LOOP_ITERATIONS { break; }
            iterations += 1;

            ctx.data.insert((node_id, "entity"), PieValue::Entity(slot));
            ctx.data.insert((node_id, "index"),  PieValue::Int(i as i32));

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
static EXEC_FOR_EACH_ENTITY: ExecForEachEntity = ExecForEachEntity;

// ── AddTag ────────────────────────────────────────────────────────────────────
struct ExecAddTag;
impl ExecBehavior for ExecAddTag {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).ok_or("AddTag: node topilmadi")?.clone();
        let tag = node.string_a.as_deref().unwrap_or("tag").to_string();
        let tag_key = format!("__tag__{tag}");

        // Bu entity ning blackboard'iga yozamiz
        ctx.blackboard.vars.insert(tag_key, PieValue::Bool(true));
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_ADD_TAG: ExecAddTag = ExecAddTag;

// ── RemoveTag ─────────────────────────────────────────────────────────────────
struct ExecRemoveTag;
impl ExecBehavior for ExecRemoveTag {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        let node = ctx.graph.node(node_id).ok_or("RemoveTag: node topilmadi")?.clone();
        let tag = node.string_a.as_deref().unwrap_or("tag").to_string();
        let tag_key = format!("__tag__{tag}");
        ctx.blackboard.vars.remove(&tag_key);
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_REMOVE_TAG: ExecRemoveTag = ExecRemoveTag;

// ── EventFixedTick / EventOnSpawn (event entry — no exec logic, just ExecOut) ─
struct ExecEventPass;
impl ExecBehavior for ExecEventPass {
    fn run(&self, _ctx: &mut PieExecContext<'_>, _node_id: usize) -> Result<ExecFlow, String> {
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_EVENT_PASS: ExecEventPass = ExecEventPass;

// ── Registration ──────────────────────────────────────────────────────────────

pub fn register_ecs_nodes(r: &mut NodeRegistry) {
    use twelfth_visual_blueprint::nodes::VisualNode;

    r.register(NodeDescriptor {
        kind: K::QueryAllEntities,
        label: "Query All Entities",
        description: "Sahnadagi barcha entitylar sonini qaytaradi",
        category: "ECS/World",
        width: 190.0, height: 56.0,
        default_node: VisualNode::query_all_entities,
        exec: None,
    });

    r.register(NodeDescriptor {
        kind: K::QueryByTag,
        label: "Query By Tag",
        description: "Belgilangan tag ga ega entitylar sonini qaytaradi",
        category: "ECS/World",
        width: 200.0, height: 56.0,
        default_node: || VisualNode::query_by_tag("MyTag"),
        exec: None,
    });

    r.register(NodeDescriptor {
        kind: K::ForEachEntity,
        label: "For Each Entity",
        description: "Sahnadagi barcha entitylar ustida loop",
        category: "ECS/World",
        width: 200.0, height: 80.0,
        default_node: VisualNode::for_each_entity,
        exec: Some(&EXEC_FOR_EACH_ENTITY),
    });

    r.register(NodeDescriptor {
        kind: K::EntityArrayGet,
        label: "Entity Array Get",
        description: "Indeks bo'yicha entity olish",
        category: "ECS/World",
        width: 190.0, height: 56.0,
        default_node: VisualNode::entity_array_get,
        exec: None,
    });

    r.register(NodeDescriptor {
        kind: K::EntityArrayLength,
        label: "Entity Count",
        description: "Sahnadagi entity soni",
        category: "ECS/World",
        width: 170.0, height: 56.0,
        default_node: VisualNode::entity_array_length,
        exec: None,
    });

    r.register(NodeDescriptor {
        kind: K::AddTag,
        label: "Add Tag",
        description: "Entity ga tag qo'shish (marker komponent)",
        category: "ECS/Component",
        width: 180.0, height: 70.0,
        default_node: || VisualNode::add_tag("MyTag"),
        exec: Some(&EXEC_ADD_TAG),
    });

    r.register(NodeDescriptor {
        kind: K::RemoveTag,
        label: "Remove Tag",
        description: "Entity dan tag olib tashlash",
        category: "ECS/Component",
        width: 180.0, height: 70.0,
        default_node: || VisualNode::remove_tag("MyTag"),
        exec: Some(&EXEC_REMOVE_TAG),
    });

    r.register(NodeDescriptor {
        kind: K::HasTag,
        label: "Has Tag",
        description: "Entity da tag bor-yo'qligini tekshiradi",
        category: "ECS/Component",
        width: 180.0, height: 70.0,
        default_node: || VisualNode::has_tag("MyTag"),
        exec: None,
    });

    r.register(NodeDescriptor {
        kind: K::GetEntityName,
        label: "Get Entity Name",
        description: "Entity nomini qaytaradi",
        category: "ECS/Entity",
        width: 180.0, height: 56.0,
        default_node: VisualNode::get_entity_name,
        exec: None,
    });

    r.register(NodeDescriptor {
        kind: K::EventFixedTick,
        label: "Event Fixed Tick",
        description: "FixedUpdate schedule — fizika va deterministic logic",
        category: "ECS/Schedule",
        width: 200.0, height: 56.0,
        default_node: VisualNode::event_fixed_tick,
        exec: Some(&EXEC_EVENT_PASS),
    });

    r.register(NodeDescriptor {
        kind: K::EventOnSpawn,
        label: "Event On Spawn",
        description: "Entity spawn bo'lganda ishga tushadi",
        category: "ECS/Schedule",
        width: 200.0, height: 56.0,
        default_node: VisualNode::event_on_spawn,
        exec: Some(&EXEC_EVENT_PASS),
    });
}
