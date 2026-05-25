//! Node headers and layout row bundles.

use bevy::prelude::*;

use crate::graph::VmNodeTitle;
use crate::ui::canvas::interaction::{on_node_drag, on_node_drag_end, on_node_drag_start};
use crate::ui::node_icons::{spawn_header_icon, HeaderIcon};
use crate::ui::nodes::theme::{EVENT_HEADER_BG, FLOW_HEADER_BG};

pub(crate) fn spawn_colored_header(
    parent: &mut ChildSpawnerCommands,
    icon: HeaderIcon,
    title: &str,
    bg: Color,
) {
    parent
        .spawn((
            VmNodeTitle,
            Pickable::default(),
            header_row_bundle(bg),
        ))
        .with_children(|row| {
            spawn_header_icon(row, icon);
            row.spawn((
                Text::new(title),
                Pickable::IGNORE,
                TextFont {
                    font_size: FontSize::Px(13.0),
                    weight: FontWeight::BOLD,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        })
        .observe(on_node_drag_start)
        .observe(on_node_drag)
        .observe(on_node_drag_end);
}

pub(crate) fn spawn_event_header(parent: &mut ChildSpawnerCommands, icon: HeaderIcon, title: &str) {
    spawn_colored_header(parent, icon, title, EVENT_HEADER_BG);
}

pub(crate) fn spawn_flow_header(parent: &mut ChildSpawnerCommands, icon: HeaderIcon, title: &str) {
    spawn_colored_header(parent, icon, title, FLOW_HEADER_BG);
}

pub(crate) fn header_row_bundle(bg: Color) -> impl Bundle {
    (
        Node {
            width: percent(100),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(8),
            padding: UiRect::axes(px(10), px(8)),
            ..default()
        },
        BackgroundColor(bg),
    )
}

pub(crate) fn body_column_bundle() -> Node {
    Node {
        width: percent(100),
        flex_direction: FlexDirection::Column,
        row_gap: px(8),
        padding: UiRect::axes(px(10), px(10)),
        ..default()
    }
}

pub(crate) fn pin_row_bundle() -> Node {
    Node {
        width: percent(100),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::FlexEnd,
        align_items: AlignItems::Center,
        column_gap: px(8),
        padding: UiRect::axes(px(12), px(10)),
        min_height: px(38.),
        ..default()
    }
}

pub(crate) fn pin_column_bundle(align: AlignItems) -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        row_gap: px(8),
        flex_grow: 1.0,
        align_items: align,
        ..default()
    }
}

pub(crate) fn pin_label(text: &str, color: Color) -> impl Bundle {
    (
        Text::new(text),
        Pickable::IGNORE,
        TextFont {
            font_size: FontSize::Px(11.0),
            ..default()
        },
        TextColor(color),
    )
}
