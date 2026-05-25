//! Branch styled node.

use bevy::prelude::*;

use crate::graph::GraphNodeData;
use crate::ui::canvas::interaction::{on_node_click, on_node_press};
use crate::ui::node_icons::HeaderIcon;
use crate::ui::nodes::headers::{pin_column_bundle, spawn_flow_header};
use crate::ui::nodes::pins::{spawn_condition_row, spawn_exec_in_row, spawn_exec_out_row};
use crate::ui::nodes::shell::spawn_styled_shell;
use crate::ui::nodes::theme::{BRANCH_NODE_WIDTH, StyledNodeKind};

pub(crate) fn spawn_branch_node(
    parent: &mut ChildSpawnerCommands,
    node: &GraphNodeData,
    selected: bool,
) -> Entity {
    let condition = node.params.condition_value.unwrap_or(false);
    let mut shell = spawn_styled_shell(
        parent,
        node,
        selected,
        StyledNodeKind::Branch,
        BRANCH_NODE_WIDTH,
    );
    shell.with_children(|root| {
        spawn_flow_header(root, HeaderIcon::BranchFork, "Branch");
        root.spawn(Node {
            width: percent(100),
            flex_direction: FlexDirection::Row,
            padding: UiRect::axes(px(8), px(8)),
            column_gap: px(4),
            ..default()
        })
        .with_children(|body| {
            body.spawn(pin_column_bundle(AlignItems::FlexStart))
                .with_children(|left| {
                    spawn_exec_in_row(left, node.id, "exec_in", "In");
                    spawn_condition_row(left, node.id, condition);
                });
            body.spawn(pin_column_bundle(AlignItems::FlexEnd))
                .with_children(|right| {
                    spawn_exec_out_row(right, node.id, "true", "True");
                    spawn_exec_out_row(right, node.id, "false", "False");
                });
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}
