//! Styled blueprint node spawners.

mod branch;
mod delay;
mod event;
mod input;
mod print_log;
pub(crate) mod variables;

pub(crate) use branch::spawn_branch_node;
pub(crate) use delay::spawn_delay_node;
pub(crate) use event::{spawn_event_begin_play_node, spawn_event_tick_node};
pub(crate) use input::{spawn_is_key_just_pressed_node, spawn_is_key_pressed_node};
pub(crate) use print_log::spawn_print_log_node;
pub(crate) use variables::{
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
