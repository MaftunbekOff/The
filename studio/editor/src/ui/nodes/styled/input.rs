//! IsKeyPressed / IsKeyJustPressed styled node spawners.

use bevy::prelude::*;

use crate::graph::GraphNodeData;
use crate::ui::canvas::interaction::{on_node_click, on_node_press};
use crate::ui::node_icons::HeaderIcon;
use crate::ui::nodes::headers::{body_column_bundle, spawn_colored_header};
use crate::ui::nodes::pins::{spawn_key_name_row};
use crate::ui::nodes::shell::spawn_styled_shell;
use crate::ui::nodes::theme::{DATA_PIN_BOOL, INPUT_HEADER_BG, INPUT_NODE_WIDTH, StyledNodeKind};

pub(crate) fn spawn_is_key_pressed_node(
    parent: &mut ChildSpawnerCommands,
    node: &GraphNodeData,
    selected: bool,
) -> Entity {
    let key = node.params.key_name.clone().unwrap_or_else(|| "Space".into());
    let mut shell = spawn_styled_shell(
        parent,
        node,
        selected,
        StyledNodeKind::IsKeyPressed,
        INPUT_NODE_WIDTH,
    );
    shell.with_children(|root| {
        spawn_colored_header(root, HeaderIcon::InputKeyboard, "Is Key Pressed", INPUT_HEADER_BG);
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_key_name_row(body, node.id, &key);
            // Bool chiqish — "pressed"
            spawn_data_out_row_bool(body, node.id, "pressed", "Pressed");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

pub(crate) fn spawn_is_key_just_pressed_node(
    parent: &mut ChildSpawnerCommands,
    node: &GraphNodeData,
    selected: bool,
) -> Entity {
    let key = node.params.key_name.clone().unwrap_or_else(|| "Space".into());
    let mut shell = spawn_styled_shell(
        parent,
        node,
        selected,
        StyledNodeKind::IsKeyJustPressed,
        INPUT_NODE_WIDTH,
    );
    shell.with_children(|root| {
        spawn_colored_header(
            root,
            HeaderIcon::InputKeyboard,
            "Is Key Just Pressed",
            INPUT_HEADER_BG,
        );
        root.spawn(body_column_bundle()).with_children(|body| {
            spawn_key_name_row(body, node.id, &key);
            spawn_data_out_row_bool(body, node.id, "pressed", "Pressed");
        });
    })
    .observe(on_node_press)
    .observe(on_node_click)
    .id()
}

/// Bool tipdagi chiqish satri (qizil halqa).
fn spawn_data_out_row_bool(
    parent: &mut ChildSpawnerCommands,
    node: crate::graph::NodeId,
    pin: &'static str,
    label: &str,
) {
    use crate::ui::nodes::headers::pin_label;
    use crate::ui::nodes::theme::LABEL_DATA_OUT;
    use crate::ui::canvas::interaction::{
        on_port_drag_while_connecting, on_port_press, on_port_release, on_port_stop_drag,
        on_port_stop_drag_end, on_port_stop_drag_start, on_port_stop_pointer, on_port_stop_release,
    };
    use crate::graph::GraphPort;
    use crate::registry::WireCategory;

    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.),
            width: Val::Percent(100.),
            min_height: Val::Px(24.),
            ..default()
        })
        .with_children(|row| {
            row.spawn(pin_label(label, LABEL_DATA_OUT));
            // Hollow bool ring hitbox
            row.spawn((
                GraphPort { node, pin, category: WireCategory::Data },
                Pickable::default(),
                Node {
                    width: Val::Px(40.),
                    height: Val::Px(40.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_shrink: 0.,
                    ..default()
                },
            ))
            .with_children(|hit| {
                use crate::ui::nodes::theme::EVENT_BODY_BG;
                hit.spawn((
                    Node {
                        width: Val::Px(14.),
                        height: Val::Px(14.),
                        flex_shrink: 0.,
                        border: UiRect::all(Val::Px(2.)),
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(EVENT_BODY_BG),
                    BorderColor::all(DATA_PIN_BOOL),
                    Pickable::IGNORE,
                ));
            })
            .observe(on_port_stop_pointer)
            .observe(on_port_stop_release)
            .observe(on_port_stop_drag_start)
            .observe(on_port_drag_while_connecting)
            .observe(on_port_stop_drag)
            .observe(on_port_stop_drag_end)
            .observe(on_port_press)
            .observe(on_port_release);
        });
}
