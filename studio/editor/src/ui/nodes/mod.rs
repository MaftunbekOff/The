//! Blueprint node spawning and palette.

mod headers;
pub mod palette;
mod pins;
mod shell;
pub(crate) use shell::styled_border;
mod standard;
mod styled;
pub mod theme;

use bevy::prelude::*;
use twelfth_visual_blueprint::nodes::NodeKind as BpNodeKind;

use crate::graph::GraphNodeData;

pub use palette::{AddNodeButton, DeleteNodeButton, DisconnectButton, spawn_palette_button};

use standard::spawn_standard_node;
use styled::{
    spawn_branch_node, spawn_delay_node, spawn_event_begin_play_node, spawn_event_tick_node,
    spawn_is_key_just_pressed_node, spawn_is_key_pressed_node, spawn_print_log_node,
    // Variable Set
    spawn_set_float_var_node, spawn_set_bool_var_node,
    spawn_set_int_var_node,   spawn_set_string_var_node,
    // Variable Get
    spawn_get_float_var_node, spawn_get_bool_var_node,
    spawn_get_int_var_node,   spawn_get_string_var_node,
    // Float Arrays
    spawn_float_array_push_node, spawn_float_array_clear_node,
    spawn_float_array_get_node,  spawn_float_array_length_node,
    // Int Arrays
    spawn_int_array_push_node, spawn_int_array_get_node,
    spawn_int_array_length_node, spawn_int_array_clear_node,
    // String Arrays
    spawn_string_array_push_node, spawn_string_array_get_node,
    spawn_string_array_length_node, spawn_string_array_clear_node,
    // Loops
    spawn_for_each_float_node, spawn_while_loop_node,
    spawn_sequence_node,       spawn_do_once_node,
    // Loop control
    spawn_break_loop_node, spawn_reset_do_once_node,
    spawn_continue_loop_node,
    // Select ternary
    spawn_select_float_node, spawn_select_bool_node,
    spawn_select_int_node,   spawn_select_string_node,
    // Vec2
    spawn_vec2_make_node, spawn_vec2_add_node, spawn_vec2_sub_node,
    spawn_vec2_scale_node, spawn_vec2_length_node, spawn_vec2_normalize_node,
    spawn_vec2_dot_node, spawn_vec2_x_node, spawn_vec2_y_node,
    // Custom Events
    spawn_event_custom_begin_node, spawn_fire_custom_event_node,
    // Entity ops
    spawn_spawn_entity_node, spawn_get_named_entity_node,
    // Comment
    spawn_comment_node,
    // ScriptActor
    spawn_get_self_entity_node,
};

pub fn spawn_vm_node(
    parent: &mut ChildSpawnerCommands,
    node: &GraphNodeData,
    selected: bool,
) -> Entity {
    match node.kind.0 {
        BpNodeKind::EventBeginPlay    => spawn_event_begin_play_node(parent, node, selected),
        BpNodeKind::EventTick         => spawn_event_tick_node(parent, node, selected),
        BpNodeKind::Branch            => spawn_branch_node(parent, node, selected),
        BpNodeKind::PrintLog          => spawn_print_log_node(parent, node, selected),
        BpNodeKind::Delay             => spawn_delay_node(parent, node, selected),
        BpNodeKind::IsKeyPressed      => spawn_is_key_pressed_node(parent, node, selected),
        BpNodeKind::IsKeyJustPressed  => spawn_is_key_just_pressed_node(parent, node, selected),
        // Variables
        BpNodeKind::SetFloatVar       => spawn_set_float_var_node(parent, node, selected),
        BpNodeKind::SetBoolVar        => spawn_set_bool_var_node(parent, node, selected),
        BpNodeKind::SetIntVar         => spawn_set_int_var_node(parent, node, selected),
        BpNodeKind::SetStringVar      => spawn_set_string_var_node(parent, node, selected),
        BpNodeKind::GetFloatVar       => spawn_get_float_var_node(parent, node, selected),
        BpNodeKind::GetBoolVar        => spawn_get_bool_var_node(parent, node, selected),
        BpNodeKind::GetIntVar         => spawn_get_int_var_node(parent, node, selected),
        BpNodeKind::GetStringVar      => spawn_get_string_var_node(parent, node, selected),
        // Arrays
        BpNodeKind::FloatArrayPush    => spawn_float_array_push_node(parent, node, selected),
        BpNodeKind::FloatArrayClear   => spawn_float_array_clear_node(parent, node, selected),
        BpNodeKind::FloatArrayGet     => spawn_float_array_get_node(parent, node, selected),
        BpNodeKind::FloatArrayLength  => spawn_float_array_length_node(parent, node, selected),
        // Loops & flow
        BpNodeKind::ForEachFloat      => spawn_for_each_float_node(parent, node, selected),
        BpNodeKind::WhileLoop         => spawn_while_loop_node(parent, node, selected),
        BpNodeKind::Sequence          => spawn_sequence_node(parent, node, selected),
        BpNodeKind::DoOnce            => spawn_do_once_node(parent, node, selected),
        BpNodeKind::BreakLoop         => spawn_break_loop_node(parent, node, selected),
        BpNodeKind::ResetDoOnce       => spawn_reset_do_once_node(parent, node, selected),
        BpNodeKind::ContinueLoop      => spawn_continue_loop_node(parent, node, selected),
        // Select ternary
        BpNodeKind::SelectFloat       => spawn_select_float_node(parent, node, selected),
        BpNodeKind::SelectBool        => spawn_select_bool_node(parent, node, selected),
        BpNodeKind::SelectInt         => spawn_select_int_node(parent, node, selected),
        BpNodeKind::SelectString      => spawn_select_string_node(parent, node, selected),
        // Int Arrays
        BpNodeKind::IntArrayPush      => spawn_int_array_push_node(parent, node, selected),
        BpNodeKind::IntArrayGet       => spawn_int_array_get_node(parent, node, selected),
        BpNodeKind::IntArrayLength    => spawn_int_array_length_node(parent, node, selected),
        BpNodeKind::IntArrayClear     => spawn_int_array_clear_node(parent, node, selected),
        // String Arrays
        BpNodeKind::StringArrayPush   => spawn_string_array_push_node(parent, node, selected),
        BpNodeKind::StringArrayGet    => spawn_string_array_get_node(parent, node, selected),
        BpNodeKind::StringArrayLength => spawn_string_array_length_node(parent, node, selected),
        BpNodeKind::StringArrayClear  => spawn_string_array_clear_node(parent, node, selected),
        // Vec2
        BpNodeKind::Vec2Make          => spawn_vec2_make_node(parent, node, selected),
        BpNodeKind::Vec2Add           => spawn_vec2_add_node(parent, node, selected),
        BpNodeKind::Vec2Sub           => spawn_vec2_sub_node(parent, node, selected),
        BpNodeKind::Vec2Scale         => spawn_vec2_scale_node(parent, node, selected),
        BpNodeKind::Vec2Length        => spawn_vec2_length_node(parent, node, selected),
        BpNodeKind::Vec2Normalize     => spawn_vec2_normalize_node(parent, node, selected),
        BpNodeKind::Vec2Dot           => spawn_vec2_dot_node(parent, node, selected),
        BpNodeKind::Vec2X             => spawn_vec2_x_node(parent, node, selected),
        BpNodeKind::Vec2Y             => spawn_vec2_y_node(parent, node, selected),
        // Comment
        BpNodeKind::Comment           => spawn_comment_node(parent, node, selected),
        // ScriptActor
        BpNodeKind::GetSelfEntity     => spawn_get_self_entity_node(parent, node, selected),
        // Custom Events
        BpNodeKind::EventCustomBegin  => spawn_event_custom_begin_node(parent, node, selected),
        BpNodeKind::FireCustomEvent   => spawn_fire_custom_event_node(parent, node, selected),
        // Entity ops with name fields
        BpNodeKind::SpawnEntity       => spawn_spawn_entity_node(parent, node, selected),
        BpNodeKind::GetNamedEntity    => spawn_get_named_entity_node(parent, node, selected),
        // ECS nodes — standard node with category color applied in spawn_standard_node
        BpNodeKind::QueryAllEntities | BpNodeKind::QueryByTag
        | BpNodeKind::ForEachEntity   | BpNodeKind::EntityArrayGet | BpNodeKind::EntityArrayLength
        | BpNodeKind::AddTag          | BpNodeKind::RemoveTag | BpNodeKind::HasTag | BpNodeKind::GetEntityName
        | BpNodeKind::EventFixedTick  | BpNodeKind::EventOnSpawn
            => spawn_standard_node(parent, node, selected),
        _                             => spawn_standard_node(parent, node, selected),
    }
}

/// `Commands` + viewport Entity orqali tugun spawn qiladi (tab switch uchun).
pub fn spawn_vm_node_commands(
    commands: &mut Commands,
    node: &GraphNodeData,
    viewport: Entity,
) {
    commands.entity(viewport).with_children(|parent| {
        spawn_vm_node(parent, node, false);
    });
}
