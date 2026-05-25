//! Node shell spawn and border styling.

use bevy::prelude::*;

use crate::graph::{GraphNodeData, VmNode};
use crate::ui::nodes::theme::{StyledBlueprintNode, StyledNodeKind, EVENT_BODY_BG};

pub(crate) fn spawn_styled_shell<'a>(
    parent: &'a mut ChildSpawnerCommands,
    node: &GraphNodeData,
    selected: bool,
    kind: StyledNodeKind,
    width: f32,
) -> EntityCommands<'a> {
    let pos = node.position;
    let border = styled_border(selected, kind);
    parent.spawn((
        VmNode { id: node.id },
        StyledBlueprintNode(kind),
        Pickable::default(),
        Node {
            position_type: PositionType::Absolute,
            left: px(pos.x),
            top: px(pos.y),
            width: px(width),
            flex_direction: FlexDirection::Column,
            overflow: Overflow::clip(),
            border: UiRect::all(px(if selected { 2 } else { 1 })),
            border_radius: BorderRadius::all(px(8)),
            ..default()
        },
        BackgroundColor(EVENT_BODY_BG),
        BorderColor::all(border),
    ))
}

pub(crate) fn styled_border(selected: bool, kind: StyledNodeKind) -> Color {
    use super::theme::{
        COLLECTION_BORDER, COMMENT_BORDER, CUSTOM_EVENT_BORDER, DELAY_BORDER, ENTITY_OP_BORDER,
        EVENT_BORDER, FLOW_BORDER, FLOW_CTRL_BORDER, INPUT_BORDER, LOOP_BORDER, PRINT_BORDER,
        VAR_GET_BORDER, VAR_SET_BORDER, VEC2_BORDER,
        ECS_WORLD_BORDER, ECS_COMP_BORDER, ECS_SCHED_BORDER,
    };
    match kind {
        StyledNodeKind::EventBeginPlay | StyledNodeKind::EventTick => {
            if selected { Color::srgb(0.85, 0.55, 1.0) } else { EVENT_BORDER }
        }
        StyledNodeKind::Branch => {
            if selected { Color::srgb(0.5, 0.62, 0.88) } else { FLOW_BORDER }
        }
        StyledNodeKind::PrintLog => {
            if selected { Color::srgb(0.35, 0.85, 0.82) } else { PRINT_BORDER }
        }
        StyledNodeKind::Delay => {
            if selected { Color::srgb(1.0, 0.62, 0.25) } else { DELAY_BORDER }
        }
        StyledNodeKind::IsKeyPressed | StyledNodeKind::IsKeyJustPressed => {
            if selected { Color::srgb(0.28, 0.98, 0.58) } else { INPUT_BORDER }
        }
        StyledNodeKind::VarSet => {
            if selected { Color::srgb(0.72, 0.48, 1.0) } else { VAR_SET_BORDER }
        }
        StyledNodeKind::VarGet => {
            if selected { Color::srgb(0.58, 0.38, 0.82) } else { VAR_GET_BORDER }
        }
        StyledNodeKind::Collection => {
            if selected { Color::srgb(0.28, 0.85, 0.95) } else { COLLECTION_BORDER }
        }
        StyledNodeKind::LoopFor => {
            if selected { Color::srgb(1.0, 0.72, 0.28) } else { LOOP_BORDER }
        }
        StyledNodeKind::FlowCtrl => {
            if selected { Color::srgb(0.45, 0.58, 0.82) } else { FLOW_CTRL_BORDER }
        }
        StyledNodeKind::CustomEvent => {
            if selected { Color::srgb(0.95, 0.42, 0.88) } else { CUSTOM_EVENT_BORDER }
        }
        StyledNodeKind::EntityOp => {
            if selected { Color::srgb(0.35, 0.65, 1.0) } else { ENTITY_OP_BORDER }
        }
        StyledNodeKind::Vec2Math => {
            if selected { Color::srgb(0.28, 0.98, 0.72) } else { VEC2_BORDER }
        }
        StyledNodeKind::Comment => {
            if selected { Color::srgb(0.70, 0.66, 0.55) } else { COMMENT_BORDER }
        }
        StyledNodeKind::EcsWorld => {
            if selected { Color::srgb(0.38, 0.85, 0.48) } else { ECS_WORLD_BORDER }
        }
        StyledNodeKind::EcsComponent => {
            if selected { Color::srgb(0.28, 0.95, 0.72) } else { ECS_COMP_BORDER }
        }
        StyledNodeKind::EcsSchedule => {
            if selected { Color::srgb(0.65, 0.38, 1.0) } else { ECS_SCHED_BORDER }
        }
    }
}
