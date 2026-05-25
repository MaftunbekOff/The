//! Tugun ichidagi inline qiymat maydonlari (`EditableText` / checkbox → graf).

use bevy::prelude::*;
use bevy::text::{EditableText, EditableTextFilter, TextCursorStyle};

use crate::graph::{GraphResource, NodeId};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeParamField {
    Message,
    Duration,
    /// IsKeyPressed / IsKeyJustPressed uchun `key_name` maydoni.
    KeyName,
    /// Set/Get *Var, Array nodes uchun `string_a` (o'zgaruvchi/massiv nomi).
    VarName,
    /// SpawnEntity / GetNamedEntity uchun `entity_name`.
    EntityName,
    /// EventCustomBegin / FireCustomEvent uchun `event_name`.
    EventName,
}

/// Matn/raqam maydoni.
#[derive(Component, Debug, Clone, Copy)]
pub struct NodeParamInput {
    pub node: NodeId,
    pub field: NodeParamField,
}

/// Branch `condition` checkbox holati.
#[derive(Component, Debug, Clone, Copy)]
pub struct NodeParamCheckbox {
    pub node: NodeId,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct CheckboxState(pub bool);

const CHECKBOX_UNCHECKED_BG: Color = Color::srgb(0.06, 0.06, 0.08);
const CHECKBOX_CHECKED_BG: Color = Color::srgb(0.12, 0.14, 0.18);
const CHECKBOX_BORDER: Color = Color::srgb(0.88, 0.90, 0.94);
const CHECKBOX_MARK: Color = Color::srgb(0.92, 0.94, 0.98);

pub fn sync_node_param_inputs(
    mut graph: ResMut<GraphResource>,
    inputs: Query<(&NodeParamInput, &EditableText), Changed<EditableText>>,
) {
    for (input, editable) in &inputs {
        let text = editable_text_string(editable);
        let Some(data) = graph.node_mut(input.node) else {
            continue;
        };
        match input.field {
            NodeParamField::Message => {
                data.params.log_message = Some(text);
            }
            NodeParamField::Duration => {
                let normalized = text.replace(',', ".");
                if let Ok(v) = normalized.parse::<f32>() {
                    data.params.delay_seconds = Some(v);
                }
            }
            NodeParamField::KeyName => {
                data.params.key_name = Some(text);
            }
            NodeParamField::VarName => {
                data.params.string_a = Some(text);
            }
            NodeParamField::EntityName => {
                data.params.entity_name = Some(text);
            }
            NodeParamField::EventName => {
                data.params.event_name = Some(text);
            }
        }
    }
}

pub fn on_condition_checkbox_press(
    mut press: On<Pointer<Press>>,
    mut graph: ResMut<GraphResource>,
    mut q: Query<(
        &NodeParamCheckbox,
        &mut CheckboxState,
        &Children,
        &mut BackgroundColor,
    )>,
    mut marks: Query<&mut Node, Without<NodeParamCheckbox>>,
) {
    if press.button != PointerButton::Primary {
        return;
    }
    let Ok((input, mut state, children, mut bg)) = q.get_mut(press.event_target()) else {
        return;
    };
    press.propagate(false);

    state.0 = !state.0;
    *bg = BackgroundColor(if state.0 {
        CHECKBOX_CHECKED_BG
    } else {
        CHECKBOX_UNCHECKED_BG
    });

    if let Some(data) = graph.node_mut(input.node) {
        data.params.condition_value = Some(state.0);
    }

  // Belgini yangilash: bitta bolak — checkmark to‘rtburchagi.
    let mark_entity = children.first().copied();
    if let Some(mark) = mark_entity {
        if let Ok(mut node) = marks.get_mut(mark) {
            node.display = if state.0 {
                Display::DEFAULT
            } else {
                Display::None
            };
        }
    }
}

pub fn on_param_stop_pointer(
    mut event: On<Pointer<Press>>,
    q: Query<(), (With<NodeParamInput>, Without<NodeParamCheckbox>)>,
    checkboxes: Query<(), With<NodeParamCheckbox>>,
) {
    if q.get(event.event_target()).is_ok() || checkboxes.get(event.event_target()).is_ok() {
        event.propagate(false);
    }
}

pub fn on_param_stop_drag_start(
    mut event: On<Pointer<DragStart>>,
    q: Query<(), (With<NodeParamInput>, Without<NodeParamCheckbox>)>,
    checkboxes: Query<(), With<NodeParamCheckbox>>,
) {
    if q.get(event.event_target()).is_ok() || checkboxes.get(event.event_target()).is_ok() {
        event.propagate(false);
    }
}

pub fn on_param_stop_drag(
    mut event: On<Pointer<Drag>>,
    q: Query<(), (With<NodeParamInput>, Without<NodeParamCheckbox>)>,
    checkboxes: Query<(), With<NodeParamCheckbox>>,
) {
    if q.get(event.event_target()).is_ok() || checkboxes.get(event.event_target()).is_ok() {
        event.propagate(false);
    }
}

fn editable_text_string(editable: &EditableText) -> String {
    editable
        .value()
        .into_iter()
        .fold(String::new(), |mut s, part| {
            s.push_str(part);
            s
        })
}

pub fn spawn_bool_checkbox(
    parent: &mut ChildSpawnerCommands,
    node: NodeId,
    initial: bool,
) {
    parent
        .spawn((
            NodeParamCheckbox { node },
            CheckboxState(initial),
            Pickable::default(),
            Node {
                width: px(18.),
                height: px(18.),
                flex_shrink: 0.,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(px(1.5)),
                border_radius: BorderRadius::all(px(4.)),
                ..default()
            },
            BackgroundColor(if initial {
                CHECKBOX_CHECKED_BG
            } else {
                CHECKBOX_UNCHECKED_BG
            }),
            BorderColor::all(CHECKBOX_BORDER),
        ))
        .with_children(|c| {
            c.spawn((
                Node {
                    width: px(10.),
                    height: px(10.),
                    border_radius: BorderRadius::all(px(2.)),
                    display: if initial {
                        Display::DEFAULT
                    } else {
                        Display::None
                    },
                    ..default()
                },
                BackgroundColor(CHECKBOX_MARK),
            ));
        })
        .observe(on_condition_checkbox_press)
        .observe(on_param_stop_pointer)
        .observe(on_param_stop_drag_start)
        .observe(on_param_stop_drag);
}

pub fn spawn_string_param_field(
    parent: &mut ChildSpawnerCommands,
    node: NodeId,
    field: NodeParamField,
    initial: &str,
) {
    parent
        .spawn((
            NodeParamInput { node, field },
            Pickable::default(),
            Node {
                flex_grow: 1.0,
                flex_shrink: 1.0,
                min_width: px(52.),
                max_width: px(120.),
                height: px(22.),
                padding: UiRect::axes(px(6.), px(2.)),
                border: UiRect::all(px(1.)),
                border_radius: BorderRadius::all(px(3.)),
                overflow: Overflow::clip(),
                ..default()
            },
            EditableText::new(initial),
            TextCursorStyle::default(),
            TextFont {
                font_size: FontSize::Px(11.0),
                ..default()
            },
            TextColor(Color::srgb(0.92, 0.94, 0.98)),
            BackgroundColor(Color::srgb(0.04, 0.04, 0.06)),
            BorderColor::all(Color::srgb(0.42, 0.45, 0.52)),
        ))
        .observe(on_param_stop_pointer)
        .observe(on_param_stop_drag_start)
        .observe(on_param_stop_drag);
}

pub fn spawn_float_param_field(
    parent: &mut ChildSpawnerCommands,
    node: NodeId,
    field: NodeParamField,
    initial: &str,
) {
    parent
        .spawn((
            NodeParamInput { node, field },
            Pickable::default(),
            Node {
                flex_grow: 1.0,
                flex_shrink: 1.0,
                min_width: px(40.),
                max_width: px(72.),
                height: px(22.),
                padding: UiRect::axes(px(6.), px(2.)),
                border: UiRect::all(px(1.)),
                border_radius: BorderRadius::all(px(3.)),
                overflow: Overflow::clip(),
                ..default()
            },
            EditableText::new(initial),
            TextCursorStyle::default(),
            EditableTextFilter::new(|c| c.is_ascii_digit() || c == '.' || c == ','),
            TextFont {
                font_size: FontSize::Px(11.0),
                ..default()
            },
            TextColor(Color::srgb(0.92, 0.94, 0.98)),
            BackgroundColor(Color::srgb(0.04, 0.04, 0.06)),
            BorderColor::all(Color::srgb(0.42, 0.45, 0.52)),
        ))
        .observe(on_param_stop_pointer)
        .observe(on_param_stop_drag_start)
        .observe(on_param_stop_drag);
}

pub fn format_duration_display(secs: f32) -> String {
    format!("{:.2}", secs).replace('.', ",")
}
