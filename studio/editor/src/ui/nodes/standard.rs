//! Standard (non-styled) blueprint node.

use bevy::prelude::*;

use crate::graph::{GraphNodeData, VmNode, VmNodeTitle};
use crate::registry::{PinRef, WireCategory};
use crate::ui::canvas::interaction::{on_node_click, on_node_drag, on_node_drag_end, on_node_drag_start, on_node_press};
use crate::ui::nodes::pins::spawn_pin_row;
use crate::ui::nodes::theme::{BG_NODE, BG_NODE_SELECTED, NODE_WIDTH};

pub(crate) fn spawn_standard_node(
    parent: &mut ChildSpawnerCommands,
    node: &GraphNodeData,
    selected: bool,
) -> Entity {
    let pos = node.position;
    let bg = if selected {
        BG_NODE_SELECTED
    } else {
        BG_NODE
    };
    let pins: Vec<PinRef> = node.kind.pins();
    let exec_pins: Vec<_> = pins
        .iter()
        .filter(|p| p.category == WireCategory::Exec)
        .copied()
        .collect();
    let data_pins: Vec<_> = pins
        .iter()
        .filter(|p| p.category == WireCategory::Data)
        .copied()
        .collect();

    parent
        .spawn((
            VmNode { id: node.id },
            Pickable::default(),
            Node {
                position_type: PositionType::Absolute,
                left: px(pos.x),
                top: px(pos.y),
                width: px(NODE_WIDTH),
                min_height: px(if data_pins.is_empty() { 76.0 } else { 104.0 }),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(px(selected as u32 + 1)),
                border_radius: BorderRadius::all(px(6)),
                ..default()
            },
            BackgroundColor(bg),
            BorderColor::all(if selected {
                Color::srgb(0.45, 0.65, 0.95)
            } else {
                Color::srgb(0.32, 0.36, 0.45)
            }),
        ))
        .with_children(|body| {
            body.spawn((
                VmNodeTitle,
                Pickable::default(),
                Node {
                    width: percent(100),
                    padding: UiRect::all(px(8)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.14, 0.16, 0.22)),
                children![(
                    Text::new(node.title()),
                    Pickable::IGNORE,
                    TextFont {
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.92, 0.96)),
                )],
            ))
            .observe(on_node_drag_start)
            .observe(on_node_drag)
            .observe(on_node_drag_end);

            spawn_pin_row(body, node.id, &exec_pins);
            if !data_pins.is_empty() {
                spawn_pin_row(body, node.id, &data_pins);
            }
        })
        .observe(on_node_press)
        .observe(on_node_click)
        .id()
}
