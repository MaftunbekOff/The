//! Event BeginPlay and Event Tick styled nodes.

use bevy::prelude::*;

use crate::graph::GraphNodeData;
use crate::ui::canvas::interaction::{on_node_click, on_node_press};
use crate::ui::node_icons::HeaderIcon;
use crate::ui::nodes::headers::{pin_column_bundle, pin_row_bundle, spawn_event_header};
use crate::ui::nodes::pins::{spawn_data_out_row, spawn_exec_out_row};
use crate::ui::nodes::shell::spawn_styled_shell;
use crate::ui::nodes::theme::{EVENT_NODE_WIDTH, StyledNodeKind};

pub(crate) fn spawn_event_begin_play_node(
    parent: &mut ChildSpawnerCommands,
    node: &GraphNodeData,
    selected: bool,
) -> Entity {
    let mut shell = spawn_styled_shell(
        parent,
        node,
        selected,
        StyledNodeKind::EventBeginPlay,
        EVENT_NODE_WIDTH,
    );
    shell.with_children(|root| {
        spawn_event_header(root, HeaderIcon::EventLightning, "Event BeginPlay");
        root.spawn(pin_row_bundle()).with_children(|row| {
            spawn_exec_out_row(row, node.id, "exec_out", "Exec");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_event_tick_node(
    parent: &mut ChildSpawnerCommands,
    node: &GraphNodeData,
    selected: bool,
) -> Entity {
    let mut shell = spawn_styled_shell(
        parent,
        node,
        selected,
        StyledNodeKind::EventTick,
        EVENT_NODE_WIDTH,
    );
    shell.with_children(|root| {
        spawn_event_header(root, HeaderIcon::EventClock, "Event Tick");
        root.spawn(pin_column_bundle(AlignItems::FlexEnd)).with_children(|col| {
            spawn_exec_out_row(col, node.id, "exec_out", "Exec");
            spawn_data_out_row(col, node.id, "delta_time", "Delta Time");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}
