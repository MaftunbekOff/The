//! Variable (Set/Get *Var), Array (FloatArray*), CustomEvent, SpawnEntity styled node spawners.

use bevy::prelude::*;

use crate::graph::GraphNodeData;
use crate::ui::canvas::interaction::{on_node_click, on_node_press};
use crate::ui::node_icons::HeaderIcon;
use crate::ui::nodes::headers::{body_column_bundle, spawn_colored_header};
use crate::ui::nodes::pins::{
    spawn_data_in_row_colored, spawn_data_out_row, spawn_data_out_row_colored,
    spawn_entity_name_row, spawn_event_name_row, spawn_exec_in_row, spawn_exec_out_row,
    spawn_var_name_row,
};
use crate::ui::nodes::shell::spawn_styled_shell;
use crate::ui::nodes::theme::{
    COLLECTION_HEADER_BG, COMMENT_HEADER_BG, CUSTOM_EVENT_HEADER_BG, DATA_PIN_BOOL, DATA_PIN_RING,
    DATA_PIN_TEAL, DATA_PIN_VEC2, ENTITY_OP_HEADER_BG, EVENT_NODE_WIDTH, FLOW_CTRL_HEADER_BG,
    FLOW_NODE_WIDTH, LOOP_HEADER_BG, VAR_GET_HEADER_BG, VAR_SET_HEADER_BG, VEC2_HEADER_BG,
    StyledNodeKind,
};

// ── Variable Set nodes ────────────────────────────────────────────────────────

pub(crate) fn spawn_set_float_var_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    spawn_set_var(parent, node, selected, "Set Float Var", StyledNodeKind::VarSet, DATA_PIN_RING, "value")
}
pub(crate) fn spawn_set_bool_var_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    spawn_set_var(parent, node, selected, "Set Bool Var", StyledNodeKind::VarSet, DATA_PIN_BOOL, "value")
}
pub(crate) fn spawn_set_int_var_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    spawn_set_var(parent, node, selected, "Set Int Var", StyledNodeKind::VarSet, DATA_PIN_RING, "value")
}
pub(crate) fn spawn_set_string_var_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    spawn_set_var(parent, node, selected, "Set String Var", StyledNodeKind::VarSet, DATA_PIN_TEAL, "value")
}

fn spawn_set_var(
    parent: &mut ChildSpawnerCommands,
    node: &GraphNodeData,
    selected: bool,
    title: &str,
    kind: StyledNodeKind,
    pin_color: Color,
    value_pin: &'static str,
) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("");
    let mut shell = spawn_styled_shell(parent, node, selected, kind, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("$"), title, VAR_SET_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_var_name_row(body, node.id, "Name", name);
            spawn_data_in_row_colored(body, node.id, value_pin, "Value", pin_color);
            spawn_exec_out_row(body, node.id, "exec_out", "Out");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

// ── Variable Get nodes ────────────────────────────────────────────────────────

pub(crate) fn spawn_get_float_var_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    spawn_get_var(parent, node, selected, "Get Float Var", StyledNodeKind::VarGet, DATA_PIN_RING, "result")
}
pub(crate) fn spawn_get_bool_var_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    spawn_get_var(parent, node, selected, "Get Bool Var", StyledNodeKind::VarGet, DATA_PIN_BOOL, "result")
}
pub(crate) fn spawn_get_int_var_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    spawn_get_var(parent, node, selected, "Get Int Var", StyledNodeKind::VarGet, DATA_PIN_RING, "result")
}
pub(crate) fn spawn_get_string_var_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    spawn_get_var(parent, node, selected, "Get String Var", StyledNodeKind::VarGet, DATA_PIN_TEAL, "result")
}

fn spawn_get_var(
    parent: &mut ChildSpawnerCommands,
    node: &GraphNodeData,
    selected: bool,
    title: &str,
    kind: StyledNodeKind,
    pin_color: Color,
    result_pin: &'static str,
) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("");
    let mut shell = spawn_styled_shell(parent, node, selected, kind, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("$"), title, VAR_GET_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_var_name_row(body, node.id, "Name", name);
            spawn_data_out_row_colored(body, node.id, result_pin, "Result", pin_color);
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

// ── Float Array exec nodes ────────────────────────────────────────────────────

pub(crate) fn spawn_float_array_push_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Collection, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("[]"), "Array Push", COLLECTION_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_var_name_row(body, node.id, "Array", name);
            spawn_data_in_row_colored(body, node.id, "value", "Value", DATA_PIN_RING);
            spawn_exec_out_row(body, node.id, "exec_out", "Out");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_float_array_clear_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Collection, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("[]"), "Array Clear", COLLECTION_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_var_name_row(body, node.id, "Array", name);
            spawn_exec_out_row(body, node.id, "exec_out", "Out");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

// ── Float Array pure nodes ────────────────────────────────────────────────────

pub(crate) fn spawn_float_array_get_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Collection, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("[]"), "Array Get", COLLECTION_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_var_name_row(body, node.id, "Array", name);
            spawn_data_in_row_colored(body, node.id, "index", "Index", DATA_PIN_RING);
            spawn_data_out_row(body, node.id, "result", "Result");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_float_array_length_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Collection, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("[]"), "Array Length", COLLECTION_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_var_name_row(body, node.id, "Array", name);
            spawn_data_out_row(body, node.id, "result", "Length");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

// ── ForEachFloat ──────────────────────────────────────────────────────────────

pub(crate) fn spawn_for_each_float_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::LoopFor, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("↺"), "For Each Float", LOOP_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_var_name_row(body, node.id, "Array", name);
            spawn_exec_out_row(body, node.id, "loop_body", "Loop Body");
            spawn_exec_out_row(body, node.id, "completed", "Completed");
            spawn_data_out_row(body, node.id, "item", "Item");
            spawn_data_out_row(body, node.id, "index", "Index");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

// ── WhileLoop ─────────────────────────────────────────────────────────────────

pub(crate) fn spawn_while_loop_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::LoopFor, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("↺"), "While Loop", LOOP_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_data_in_row_colored(body, node.id, "condition", "Condition", DATA_PIN_BOOL);
            spawn_exec_out_row(body, node.id, "loop_body", "Loop Body");
            spawn_exec_out_row(body, node.id, "completed", "Completed");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

// ── Sequence / DoOnce ─────────────────────────────────────────────────────────

pub(crate) fn spawn_sequence_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::FlowCtrl, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("→"), "Sequence", FLOW_CTRL_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_exec_out_row(body, node.id, "then_0", "Then 0");
            spawn_exec_out_row(body, node.id, "then_1", "Then 1");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_do_once_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::FlowCtrl, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("1"), "Do Once", FLOW_CTRL_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_exec_out_row(body, node.id, "exec_out", "Out");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

// ── Custom Event nodes ────────────────────────────────────────────────────────

pub(crate) fn spawn_event_custom_begin_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.event_name.as_deref().unwrap_or("MyEvent");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::CustomEvent, EVENT_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("E"), "Custom Event", CUSTOM_EVENT_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_event_name_row(body, node.id, name);
            spawn_exec_out_row(body, node.id, "exec_out", "Out");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_fire_custom_event_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.event_name.as_deref().unwrap_or("MyEvent");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::CustomEvent, EVENT_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("E"), "Fire Event", CUSTOM_EVENT_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_event_name_row(body, node.id, name);
            spawn_exec_out_row(body, node.id, "exec_out", "Out");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

// ── Entity operation nodes ────────────────────────────────────────────────────

pub(crate) fn spawn_spawn_entity_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.entity_name.as_deref().unwrap_or("");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::EntityOp, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("E+"), "Spawn Entity", ENTITY_OP_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_entity_name_row(body, node.id, name);
            spawn_exec_out_row(body, node.id, "exec_out", "Out");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_get_named_entity_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.entity_name.as_deref().unwrap_or("");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::EntityOp, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("E?"), "Get Named Entity", ENTITY_OP_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_entity_name_row(body, node.id, name);
            spawn_data_out_row_colored(body, node.id, "entity", "Entity", DATA_PIN_RING);
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

// ── Loop control ──────────────────────────────────────────────────────────────

pub(crate) fn spawn_break_loop_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::FlowCtrl, 160.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("↩"), "Break Loop", FLOW_CTRL_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_continue_loop_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::FlowCtrl, 170.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("↷"), "Continue Loop", FLOW_CTRL_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_reset_do_once_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::FlowCtrl, 190.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("↺"), "Reset DoOnce", FLOW_CTRL_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_exec_out_row(body, node.id, "exec_out", "Out");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

// ── Select ternary nodes ──────────────────────────────────────────────────────

pub(crate) fn spawn_select_float_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    spawn_select_node(parent, node, selected, "Select Float", DATA_PIN_RING, "value_a", "value_b", "result")
}
pub(crate) fn spawn_select_bool_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    spawn_select_node(parent, node, selected, "Select Bool", DATA_PIN_BOOL, "value_a", "value_b", "result")
}
pub(crate) fn spawn_select_int_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    spawn_select_node(parent, node, selected, "Select Int", DATA_PIN_RING, "value_a", "value_b", "int_result")
}
pub(crate) fn spawn_select_string_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    spawn_select_node(parent, node, selected, "Select String", DATA_PIN_TEAL, "value_a", "value_b", "text")
}

fn spawn_select_node(
    parent: &mut ChildSpawnerCommands,
    node: &GraphNodeData,
    selected: bool,
    title: &str,
    pin_color: Color,
    pin_a: &'static str,
    pin_b: &'static str,
    out_pin: &'static str,
) -> Entity {
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::FlowCtrl, 200.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("?:"), title, FLOW_CTRL_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_data_in_row_colored(body, node.id, "condition", "Cond", DATA_PIN_BOOL);
            spawn_data_in_row_colored(body, node.id, pin_a, "A", pin_color);
            spawn_data_in_row_colored(body, node.id, pin_b, "B", pin_color);
            spawn_data_out_row_colored(body, node.id, out_pin, "Result", pin_color);
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

// ── Int Array nodes ───────────────────────────────────────────────────────────

pub(crate) fn spawn_int_array_push_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("my_int_array");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Collection, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("[i]+"), "Int Array Push", COLLECTION_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_var_name_row(body, node.id, "Array", name);
            spawn_data_in_row_colored(body, node.id, "int_val", "Value", DATA_PIN_RING);
            spawn_exec_out_row(body, node.id, "exec_out", "Out");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_int_array_get_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("my_int_array");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Collection, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("[i]?"), "Int Array Get", COLLECTION_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_var_name_row(body, node.id, "Array", name);
            spawn_data_in_row_colored(body, node.id, "index", "Index", DATA_PIN_RING);
            spawn_data_out_row(body, node.id, "int_result", "Value");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_int_array_length_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("my_int_array");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Collection, 200.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("[i]#"), "Int Array Length", COLLECTION_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_var_name_row(body, node.id, "Array", name);
            spawn_data_out_row(body, node.id, "int_result", "Length");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_int_array_clear_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("my_int_array");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Collection, 200.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("[i]✕"), "Int Array Clear", COLLECTION_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_var_name_row(body, node.id, "Array", name);
            spawn_exec_out_row(body, node.id, "exec_out", "Out");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

// ── String Array nodes ────────────────────────────────────────────────────────

pub(crate) fn spawn_string_array_push_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("my_str_array");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Collection, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("[s]+"), "String Array Push", COLLECTION_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_var_name_row(body, node.id, "Array", name);
            spawn_data_in_row_colored(body, node.id, "text", "Text", DATA_PIN_TEAL);
            spawn_exec_out_row(body, node.id, "exec_out", "Out");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_string_array_get_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("my_str_array");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Collection, FLOW_NODE_WIDTH);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("[s]?"), "String Array Get", COLLECTION_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_var_name_row(body, node.id, "Array", name);
            spawn_data_in_row_colored(body, node.id, "index", "Index", DATA_PIN_RING);
            spawn_data_out_row(body, node.id, "text", "Text");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_string_array_length_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("my_str_array");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Collection, 210.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("[s]#"), "String Array Length", COLLECTION_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_var_name_row(body, node.id, "Array", name);
            spawn_data_out_row(body, node.id, "int_result", "Length");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_string_array_clear_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let name = node.params.string_a.as_deref().unwrap_or("my_str_array");
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Collection, 210.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("[s]✕"), "String Array Clear", COLLECTION_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_exec_in_row(body, node.id, "exec_in", "In");
            spawn_var_name_row(body, node.id, "Array", name);
            spawn_exec_out_row(body, node.id, "exec_out", "Out");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

// ── Vec2 math nodes ───────────────────────────────────────────────────────────

pub(crate) fn spawn_vec2_make_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Vec2Math, 180.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("v2"), "Vec2 Make", VEC2_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_data_in_row_colored(body, node.id, "x", "X", DATA_PIN_RING);
            spawn_data_in_row_colored(body, node.id, "y", "Y", DATA_PIN_RING);
            spawn_data_out_row_colored(body, node.id, "result", "Vec2", DATA_PIN_VEC2);
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

fn spawn_vec2_binary_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool, title: &str) -> Entity {
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Vec2Math, 180.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("v2"), title, VEC2_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_data_in_row_colored(body, node.id, "a", "A", DATA_PIN_VEC2);
            spawn_data_in_row_colored(body, node.id, "b", "B", DATA_PIN_VEC2);
            spawn_data_out_row_colored(body, node.id, "result", "Result", DATA_PIN_VEC2);
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}
pub(crate) fn spawn_vec2_add_node(p: &mut ChildSpawnerCommands, n: &GraphNodeData, s: bool) -> Entity { spawn_vec2_binary_node(p, n, s, "Vec2 Add") }
pub(crate) fn spawn_vec2_sub_node(p: &mut ChildSpawnerCommands, n: &GraphNodeData, s: bool) -> Entity { spawn_vec2_binary_node(p, n, s, "Vec2 Sub") }

pub(crate) fn spawn_vec2_scale_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Vec2Math, 180.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("v2"), "Vec2 Scale", VEC2_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_data_in_row_colored(body, node.id, "vec",   "Vec",   DATA_PIN_VEC2);
            spawn_data_in_row_colored(body, node.id, "scale", "Scale", DATA_PIN_RING);
            spawn_data_out_row_colored(body, node.id, "result", "Result", DATA_PIN_VEC2);
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_vec2_length_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Vec2Math, 180.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("v2"), "Vec2 Length", VEC2_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_data_in_row_colored(body, node.id, "vec", "Vec", DATA_PIN_VEC2);
            spawn_data_out_row_colored(body, node.id, "result", "Length", DATA_PIN_RING);
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_vec2_normalize_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Vec2Math, 190.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("v2"), "Vec2 Normalize", VEC2_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_data_in_row_colored(body, node.id, "vec", "Vec", DATA_PIN_VEC2);
            spawn_data_out_row_colored(body, node.id, "result", "Unit", DATA_PIN_VEC2);
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_vec2_dot_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Vec2Math, 180.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("v2"), "Vec2 Dot", VEC2_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_data_in_row_colored(body, node.id, "a", "A", DATA_PIN_VEC2);
            spawn_data_in_row_colored(body, node.id, "b", "B", DATA_PIN_VEC2);
            spawn_data_out_row_colored(body, node.id, "result", "Dot", DATA_PIN_RING);
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

fn spawn_vec2_component_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool, component: &str) -> Entity {
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::Vec2Math, 160.0);
    shell.with_children(|root| {
        let title = format!("Vec2 {}", component);
        spawn_colored_header(root, HeaderIcon::Ascii("v2"), &title, VEC2_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_data_in_row_colored(body, node.id, "vec", "Vec", DATA_PIN_VEC2);
            spawn_data_out_row_colored(body, node.id, "result", component, DATA_PIN_RING);
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}
pub(crate) fn spawn_vec2_x_node(p: &mut ChildSpawnerCommands, n: &GraphNodeData, s: bool) -> Entity { spawn_vec2_component_node(p, n, s, "X") }
pub(crate) fn spawn_vec2_y_node(p: &mut ChildSpawnerCommands, n: &GraphNodeData, s: bool) -> Entity { spawn_vec2_component_node(p, n, s, "Y") }

// ── Comment node ──────────────────────────────────────────────────────────────

pub(crate) fn spawn_comment_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    use crate::ui::nodes::theme::LABEL_MUTED;
    let text = node.params.log_message.as_deref().unwrap_or("// Izoh...");
    let pos = node.position;
    let border_color = crate::ui::nodes::shell::styled_border(selected, StyledNodeKind::Comment);

    parent.spawn((
        crate::graph::VmNode { id: node.id },
        crate::ui::nodes::theme::StyledBlueprintNode(StyledNodeKind::Comment),
        Pickable::default(),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(pos.x),
            top: Val::Px(pos.y),
            min_width: Val::Px(200.0),
            padding: UiRect::all(Val::Px(10.0)),
            border: UiRect::all(Val::Px(if selected { 2.0 } else { 1.0 })),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(COMMENT_HEADER_BG.with_alpha(0.72)),
        BorderColor::all(border_color),
    ))
    .with_children(|root| {
        root.spawn((
            Text::new(text),
            TextFont { font_size: FontSize::Px(13.0), ..default() },
            TextColor(LABEL_MUTED),
        ));
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_get_self_entity_node(parent: &mut ChildSpawnerCommands, node: &GraphNodeData, selected: bool) -> Entity {
    use crate::ui::nodes::theme::{DATA_PIN_VEC3, ENTITY_OP_HEADER_BG};
    let mut shell = spawn_styled_shell(parent, node, selected, StyledNodeKind::EntityOp, 190.0);
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::Ascii("@"), "Get Self Entity", ENTITY_OP_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_data_out_row_colored(body, node.id, "entity", "Self", DATA_PIN_VEC3);
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}
