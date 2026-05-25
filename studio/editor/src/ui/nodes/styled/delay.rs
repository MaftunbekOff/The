//! Delay styled node.

use bevy::prelude::*;

use crate::graph::GraphNodeData;
use crate::ui::canvas::interaction::{on_node_click, on_node_press};
use crate::ui::node_icons::HeaderIcon;
use crate::ui::nodes::headers::{body_column_bundle, spawn_colored_header};
use crate::ui::nodes::pins::{spawn_duration_row, spawn_exec_in_row, spawn_exec_out_row};
use crate::ui::nodes::shell::spawn_styled_shell;
use crate::ui::nodes::theme::{DELAY_HEADER_BG, FLOW_NODE_WIDTH, StyledNodeKind};
use crate::ui::param_fields::format_duration_display;

pub(crate) fn spawn_delay_node(
    parent: &mut ChildSpawnerCommands,
    node: &GraphNodeData,
    selected: bool,
) -> Entity {
    let duration = format_duration_display(node.params.delay_seconds.unwrap_or(0.2));
    let mut shell = spawn_styled_shell(
        parent,
        node,
        selected,
        StyledNodeKind::Delay,
        FLOW_NODE_WIDTH,
    );
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::DelayHourglass, "Delay", DELAY_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_duration_row(body, node.id, &duration);
            spawn_exec_out_row(body, node.id, "exec_out", "Completed");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}
