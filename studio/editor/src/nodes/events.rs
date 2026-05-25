//! Events kategoriyasi: BeginPlay, Tick, CustomEvent, FireEvent.

use twelfth_visual_blueprint::nodes::{NodeKind as K, VisualNode};

use crate::interpreter::context::PieExecContext;
use crate::nodes::descriptor::{ExecBehavior, ExecFlow, NodeDescriptor, NodeRegistry};

// ── EventBeginPlay ────────────────────────────────────────────────────────────
struct ExecEventBeginPlay;
impl ExecBehavior for ExecEventBeginPlay {
    fn run(&self, _ctx: &mut PieExecContext<'_>, _node_id: usize) -> Result<ExecFlow, String> {
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_EVENT_BEGIN_PLAY: ExecEventBeginPlay = ExecEventBeginPlay;

// ── EventTick ─────────────────────────────────────────────────────────────────
struct ExecEventTick;
impl ExecBehavior for ExecEventTick {
    fn run(&self, _ctx: &mut PieExecContext<'_>, _node_id: usize) -> Result<ExecFlow, String> {
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_EVENT_TICK: ExecEventTick = ExecEventTick;

// ── EventCustomBegin ──────────────────────────────────────────────────────────
struct ExecEventCustomBegin;
impl ExecBehavior for ExecEventCustomBegin {
    fn run(&self, _ctx: &mut PieExecContext<'_>, _node_id: usize) -> Result<ExecFlow, String> {
        Ok(ExecFlow::ExecOut)
    }
}
static EXEC_EVENT_CUSTOM_BEGIN: ExecEventCustomBegin = ExecEventCustomBegin;

// ── FireCustomEvent ───────────────────────────────────────────────────────────
struct ExecFireCustomEvent;
impl ExecBehavior for ExecFireCustomEvent {
    fn run(&self, ctx: &mut PieExecContext<'_>, node_id: usize) -> Result<ExecFlow, String> {
        use twelfth_visual_blueprint::nodes::NodeKind;

        let node = ctx.graph.node(node_id).unwrap().clone();
        let event_name = node.event_name.clone().unwrap_or_default();
        ctx.log(format!("[PIE] fire_event '{event_name}'"));

        let targets: Vec<usize> = ctx
            .graph
            .nodes()
            .filter(|(_, n)| {
                n.kind == NodeKind::EventCustomBegin
                    && n.event_name.as_deref() == Some(&event_name)
            })
            .map(|(id, _)| id)
            .collect();

        Ok(ExecFlow::ThenExecOut(targets))
    }
}
static EXEC_FIRE_CUSTOM_EVENT: ExecFireCustomEvent = ExecFireCustomEvent;

// ── register() ───────────────────────────────────────────────────────────────

pub(crate) fn register(r: &mut NodeRegistry) {
    r.register(NodeDescriptor {
        kind: K::EventBeginPlay,
        label: "Event BeginPlay",
        description: "Startup — bir marta",
        category: "Events",
        width: 204.0, height: 80.0,
        default_node: VisualNode::event_begin_play,
        exec: Some(&EXEC_EVENT_BEGIN_PLAY),
    });
    r.register(NodeDescriptor {
        kind: K::EventTick,
        label: "Event Tick",
        description: "Har frame (Update)",
        category: "Events",
        width: 204.0, height: 96.0,
        default_node: || VisualNode::event_tick(),
        exec: Some(&EXEC_EVENT_TICK),
    });
    r.register(NodeDescriptor {
        kind: K::EventCustomBegin,
        label: "Custom Event",
        description: "Nomli event entry — FireEvent bilan ishga tushiriladi",
        category: "Events",
        width: 220.0, height: 96.0,
        default_node: || VisualNode::event_custom_begin("on_my_event"),
        exec: Some(&EXEC_EVENT_CUSTOM_BEGIN),
    });
    r.register(NodeDescriptor {
        kind: K::FireCustomEvent,
        label: "Fire Event",
        description: "Nomli event entry ni ishga tushiradi (exec)",
        category: "Events",
        width: 220.0, height: 80.0,
        default_node: || VisualNode::fire_custom_event("on_my_event"),
        exec: Some(&EXEC_FIRE_CUSTOM_EVENT),
    });
}
