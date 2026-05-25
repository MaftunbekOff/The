//! Print Log styled node.

use bevy::prelude::*;

use crate::graph::GraphNodeData;
use crate::ui::canvas::interaction::{on_node_click, on_node_press};
use crate::ui::node_icons::HeaderIcon;
use crate::ui::nodes::headers::{body_column_bundle, spawn_colored_header};
use crate::ui::nodes::pins::{spawn_exec_in_row, spawn_exec_out_row, spawn_message_row};
use crate::ui::nodes::shell::spawn_styled_shell;
use crate::ui::nodes::theme::{FLOW_NODE_WIDTH, PRINT_HEADER_BG, StyledNodeKind};

pub(crate) fn spawn_print_log_node(
    parent: &mut ChildSpawnerCommands,
    node: &GraphNodeData,
    selected: bool,
) -> Entity {
    let message = node
        .params
        .log_message
        .as_deref()
        .unwrap_or("Hello");
    let mut shell = spawn_styled_shell(
        parent,
        node,
        selected,
        StyledNodeKind::PrintLog,
        FLOW_NODE_WIDTH,
    );
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii(">_"), "Print Log", PRINT_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_message_row(body, node.id, message);
            spawn_exec_out_row(body, node.id, "exec_out", "Out");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}
