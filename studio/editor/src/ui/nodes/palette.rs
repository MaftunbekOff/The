//! Node palette buttons.

use bevy::prelude::*;

use crate::registry::NodeKind;

#[derive(Component, Debug, Clone, Copy)]
pub struct AddNodeButton {
    pub kind: NodeKind,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct DeleteNodeButton;

#[derive(Component, Debug, Clone, Copy)]
pub struct DisconnectButton;

pub fn spawn_palette_button(parent: &mut ChildSpawnerCommands, kind: NodeKind) {
    parent
        .spawn((
            AddNodeButton { kind },
            Button,
            Node {
                width: percent(100),
                padding: UiRect::axes(px(10.), px(6.)),
                border_radius: BorderRadius::all(px(4.)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.16, 0.18, 0.24)),
            BorderColor::all(Color::srgb(0.28, 0.32, 0.4)),
        ))
        .with_children(|b| {
            b.spawn((
                Text::new(format!("+ {}", kind.label())),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::srgb(0.88, 0.9, 0.94)),
            ));
            b.spawn((
                Text::new(kind.description()),
                TextFont {
                    font_size: FontSize::Px(10.0),
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.54, 0.58)),
            ));
        });
}
