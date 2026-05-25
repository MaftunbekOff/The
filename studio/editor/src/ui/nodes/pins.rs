//! Pin hitboxes and port row spawning.

use bevy::prelude::*;

use crate::graph::{GraphPort, NodeId};
use crate::registry::{PinRef, WireCategory};
use crate::ui::canvas::interaction::{
    on_port_drag_while_connecting, on_port_press, on_port_release, on_port_stop_drag,
    on_port_stop_drag_end, on_port_stop_drag_start, on_port_stop_pointer, on_port_stop_release,
};
use crate::ui::nodes::headers::pin_label;
use crate::ui::nodes::theme::{
    DATA_PIN_BOOL, DATA_PIN_RING, DATA_PIN_TEAL, DATA_PIN_VEC2, DATA_PIN_VEC3, EVENT_BODY_BG,
    EXEC_PIN_DOT, EXEC_PIN_GLOW, LABEL_CONDITION, LABEL_DATA_OUT, LABEL_INPUT_KEY, LABEL_MESSAGE,
    LABEL_MUTED, PORT_DATA, PORT_EXEC,
};
use crate::ui::param_fields::{
    spawn_bool_checkbox, spawn_float_param_field, spawn_string_param_field, NodeParamField,
};

const PIN_HITBOX: f32 = 40.0;

pub(crate) fn spawn_exec_out_row(parent: &mut ChildSpawnerCommands, node: NodeId, pin: &'static str, label: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            column_gap: px(8),
            width: percent(100),
            min_height: px(24.),
            ..default()
        })
        .with_children(|row| {
            row.spawn(pin_label(label, LABEL_MUTED));
            spawn_exec_pin_dot(row, node, pin);
        });
}

pub(crate) fn spawn_exec_in_row(parent: &mut ChildSpawnerCommands, node: NodeId, pin: &'static str, label: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(8),
            width: percent(100),
            min_height: px(24.),
            ..default()
        })
        .with_children(|row| {
            spawn_exec_pin_dot(row, node, pin);
            row.spawn(pin_label(label, LABEL_MUTED));
        });
}

pub(crate) fn spawn_condition_row(parent: &mut ChildSpawnerCommands, node: NodeId, initial: bool) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(8),
            width: percent(100),
            min_height: px(24.),
            ..default()
        })
        .with_children(|row| {
            spawn_data_pin_hollow(row, node, "condition", WireCategory::Data, DATA_PIN_BOOL);
            row.spawn(pin_label("Condition", LABEL_CONDITION));
            spawn_bool_checkbox(row, node, initial);
        });
}

pub(crate) fn spawn_data_out_row(parent: &mut ChildSpawnerCommands, node: NodeId, pin: &'static str, label: &str) {
    spawn_data_out_row_colored(parent, node, pin, label, DATA_PIN_RING);
}

/// Ixtiyoriy rang bilan data chiqish satri.
pub(crate) fn spawn_data_out_row_colored(
    parent: &mut ChildSpawnerCommands,
    node: NodeId,
    pin: &'static str,
    label: &str,
    color: Color,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            column_gap: px(8),
            width: percent(100),
            min_height: px(24.),
            ..default()
        })
        .with_children(|row| {
            row.spawn(pin_label(label, LABEL_DATA_OUT));
            spawn_data_pin_hollow(row, node, pin, WireCategory::Data, color);
        });
}

/// Ixtiyoriy rang bilan data kirish satri.
pub(crate) fn spawn_data_in_row_colored(
    parent: &mut ChildSpawnerCommands,
    node: NodeId,
    pin: &'static str,
    label: &str,
    color: Color,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(8),
            width: percent(100),
            min_height: px(24.),
            ..default()
        })
        .with_children(|row| {
            spawn_data_pin_hollow(row, node, pin, WireCategory::Data, color);
            row.spawn(pin_label(label, LABEL_DATA_OUT));
        });
}

pub(crate) fn spawn_message_row(parent: &mut ChildSpawnerCommands, node: NodeId, initial: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(8),
            width: percent(100),
            min_height: px(24.),
            ..default()
        })
        .with_children(|row| {
            spawn_data_pin_hollow(row, node, "message", WireCategory::Data, DATA_PIN_TEAL);
            row.spawn(pin_label("Message", LABEL_MESSAGE));
            spawn_string_param_field(row, node, NodeParamField::Message, initial);
        });
}

pub(crate) fn spawn_duration_row(parent: &mut ChildSpawnerCommands, node: NodeId, initial: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(8),
            width: percent(100),
            min_height: px(24.),
            ..default()
        })
        .with_children(|row| {
            spawn_data_pin_hollow(row, node, "duration", WireCategory::Data, DATA_PIN_RING);
            row.spawn(pin_label("Duration", LABEL_DATA_OUT));
            spawn_float_param_field(row, node, NodeParamField::Duration, initial);
        });
}

/// Input tugun satri: pin + "Key" yorlig'i + matn maydoni.
pub(crate) fn spawn_key_name_row(parent: &mut ChildSpawnerCommands, node: NodeId, initial: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            column_gap: px(8),
            width: percent(100),
            min_height: px(24.),
            ..default()
        })
        .with_children(|row| {
            row.spawn(pin_label("Key", LABEL_INPUT_KEY));
            spawn_string_param_field(row, node, NodeParamField::KeyName, initial);
        });
}

/// Variable/Array node nomi satri — o'zgaruvchi yoki massiv nomini tahrirlash.
pub(crate) fn spawn_var_name_row(parent: &mut ChildSpawnerCommands, node: NodeId, label: &str, initial: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(8),
            width: percent(100),
            padding: UiRect::horizontal(px(8.)),
            min_height: px(24.),
            ..default()
        })
        .with_children(|row| {
            row.spawn(pin_label(label, LABEL_MUTED));
            spawn_string_param_field(row, node, NodeParamField::VarName, initial);
        });
}

/// Entity nom satri — SpawnEntity / GetNamedEntity uchun.
pub(crate) fn spawn_entity_name_row(parent: &mut ChildSpawnerCommands, node: NodeId, initial: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(8),
            width: percent(100),
            padding: UiRect::horizontal(px(8.)),
            min_height: px(24.),
            ..default()
        })
        .with_children(|row| {
            row.spawn(pin_label("Name", Color::srgb(0.45, 0.72, 1.0)));
            spawn_string_param_field(row, node, NodeParamField::EntityName, initial);
        });
}

/// Custom event nom satri — EventCustomBegin / FireCustomEvent uchun.
pub(crate) fn spawn_event_name_row(parent: &mut ChildSpawnerCommands, node: NodeId, initial: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(8),
            width: percent(100),
            padding: UiRect::horizontal(px(8.)),
            min_height: px(24.),
            ..default()
        })
        .with_children(|row| {
            row.spawn(pin_label("Event", Color::srgb(0.95, 0.55, 0.88)));
            spawn_string_param_field(row, node, NodeParamField::EventName, initial);
        });
}

/// Vec3 chiqish satri.
#[allow(dead_code)]
pub(crate) fn spawn_vec3_out_row(parent: &mut ChildSpawnerCommands, node: NodeId, pin: &'static str, label: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            column_gap: px(8),
            width: percent(100),
            min_height: px(24.),
            ..default()
        })
        .with_children(|row| {
            row.spawn(pin_label(label, DATA_PIN_VEC3));
            spawn_data_pin_hollow(row, node, pin, WireCategory::Data, DATA_PIN_VEC3);
        });
}

fn spawn_exec_pin_dot(parent: &mut ChildSpawnerCommands, node: NodeId, pin: &'static str) {
    spawn_pin_hitbox(parent, node, pin, WireCategory::Exec, |hit| {
        hit.spawn((
            Node {
                width: px(14.),
                height: px(14.),
                flex_shrink: 0.,
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BackgroundColor(EXEC_PIN_DOT),
            BoxShadow::new(EXEC_PIN_GLOW, px(0.), px(0.), px(2.), px(6.)),
            Pickable::IGNORE,
        ));
    });
}

fn spawn_pin_hitbox(
    parent: &mut ChildSpawnerCommands,
    node: NodeId,
    pin: &'static str,
    category: WireCategory,
    visual: impl FnOnce(&mut ChildSpawnerCommands),
) {
    parent
        .spawn((
            GraphPort {
                node,
                pin,
                category,
            },
            Pickable::default(),
            Node {
                width: px(PIN_HITBOX),
                height: px(PIN_HITBOX),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_shrink: 0.,
                ..default()
            },
        ))
        .with_children(|hit| {
            visual(hit);
        })
        .observe(on_port_stop_pointer)
        .observe(on_port_stop_release)
        .observe(on_port_stop_drag_start)
        .observe(on_port_drag_while_connecting)
        .observe(on_port_stop_drag)
        .observe(on_port_stop_drag_end)
        .observe(on_port_press)
        .observe(on_port_release);
}

fn spawn_data_pin_hollow(
    parent: &mut ChildSpawnerCommands,
    node: NodeId,
    pin: &'static str,
    category: WireCategory,
    ring: Color,
) {
    spawn_pin_hitbox(parent, node, pin, category, |hit| {
        hit.spawn((
            Node {
                width: px(14.),
                height: px(14.),
                flex_shrink: 0.,
                border: UiRect::all(px(2.)),
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BackgroundColor(EVENT_BODY_BG),
            BorderColor::all(ring),
            Pickable::IGNORE,
        ));
    });
}

pub(crate) fn spawn_pin_row(parent: &mut ChildSpawnerCommands, node: NodeId, pins: &[PinRef]) {
    parent
        .spawn(Node {
            width: percent(100),
            padding: UiRect::all(px(6)),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            min_height: px(28.),
            ..default()
        })
        .with_children(|row| {
            for pin in pins {
                spawn_port(row, node, *pin);
            }
        });
}

fn spawn_port(parent: &mut ChildSpawnerCommands, node: NodeId, pin: PinRef) {
    use twelfth_visual_blueprint::pins::PinType;
    let color = match pin.category {
        WireCategory::Exec => PORT_EXEC,
        WireCategory::Data => match pin.ty {
            PinType::Bool   => DATA_PIN_BOOL,
            PinType::Vec3   => DATA_PIN_VEC3,
            PinType::Vec2   => DATA_PIN_VEC2,
            PinType::String => DATA_PIN_TEAL,
            _               => PORT_DATA,
        },
    };
    spawn_pin_hitbox(parent, node, pin.name, pin.category, |hit| {
        hit.spawn((
            Node {
                min_width: px(36.),
                height: px(22.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(px(4)),
                border_radius: BorderRadius::all(px(4)),
                ..default()
            },
            BackgroundColor(color),
            Pickable::IGNORE,
            children![(
                Text::new(pin.name),
                TextFont {
                    font_size: FontSize::Px(9.0),
                    ..default()
                },
                TextColor(Color::srgb(0.05, 0.05, 0.08)),
            )],
        ));
    });
}
