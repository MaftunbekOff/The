//! Tugun sarlavha ikonkalari — shrift/emojiga bog‘liq emas (UI shakllar).

use bevy::prelude::*;

const ICON: f32 = 16.0;

#[derive(Clone, Copy, Debug)]
pub enum HeaderIcon {
    EventLightning,
    EventClock,
    BranchFork,
    DelayHourglass,
    /// Keyboard — input tugmasi uchun.
    InputKeyboard,
    /// ASCII (masalan `>_`) — default shriftda ishlaydi.
    Ascii(&'static str),
}

pub fn spawn_header_icon(parent: &mut ChildSpawnerCommands, icon: HeaderIcon) {
    match icon {
        HeaderIcon::EventLightning  => spawn_lightning_icon(parent),
        HeaderIcon::EventClock      => spawn_clock_icon(parent),
        HeaderIcon::BranchFork      => spawn_branch_icon(parent),
        HeaderIcon::DelayHourglass  => spawn_hourglass_icon(parent),
        HeaderIcon::InputKeyboard   => spawn_keyboard_icon(parent),
        HeaderIcon::Ascii(text) => {
            parent.spawn((
                Text::new(text),
                Pickable::IGNORE,
                TextFont {
                    font_size: FontSize::Px(14.0),
                    weight: FontWeight::BOLD,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        }
    }
}

fn icon_slot(parent: &mut ChildSpawnerCommands, f: impl FnOnce(&mut ChildSpawnerCommands)) {
    parent
        .spawn((
            Node {
                width: px(ICON),
                height: px(ICON),
                flex_shrink: 0.,
                position_type: PositionType::Relative,
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(f);
}

/// Event BeginPlay — chaqmoq.
fn spawn_lightning_icon(parent: &mut ChildSpawnerCommands) {
    icon_slot(parent, |icon| {
        let bolt = Color::srgb(1.0, 0.98, 0.75);
        icon.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(8.),
                top: px(1.),
                width: px(3.),
                height: px(5.),
                ..default()
            },
            BackgroundColor(bolt),
        ));
        icon.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(5.),
                top: px(5.),
                width: px(6.),
                height: px(2.5),
                ..default()
            },
            BackgroundColor(bolt),
        ));
        icon.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(6.),
                top: px(7.),
                width: px(3.),
                height: px(8.),
                ..default()
            },
            BackgroundColor(Color::WHITE),
        ));
    });
}

/// Event Tick — soat.
fn spawn_clock_icon(parent: &mut ChildSpawnerCommands) {
    icon_slot(parent, |icon| {
        icon.spawn((
            Node {
                width: px(14.),
                height: px(14.),
                margin: UiRect::all(px(1.)),
                border: UiRect::all(px(2.)),
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::WHITE),
        ));
        icon.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(7.5),
                top: px(4.),
                width: px(1.5),
                height: px(4.5),
                ..default()
            },
            BackgroundColor(Color::WHITE),
        ));
        icon.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(7.5),
                top: px(7.5),
                width: px(4.),
                height: px(1.5),
                ..default()
            },
            BackgroundColor(Color::WHITE),
        ));
    });
}

/// Branch — tarmoq (vertikal + gorizontal).
fn spawn_branch_icon(parent: &mut ChildSpawnerCommands) {
    icon_slot(parent, |icon| {
        let line = Color::srgb(0.78, 0.58, 0.95);
        icon.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(7.),
                top: px(2.),
                width: px(2.),
                height: px(12.),
                ..default()
            },
            BackgroundColor(line),
        ));
        icon.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(3.),
                top: px(6.),
                width: px(10.),
                height: px(2.),
                ..default()
            },
            BackgroundColor(line),
        ));
        for x in [3., 11.] {
            icon.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: px(x),
                    top: px(5.),
                    width: px(4.),
                    height: px(4.),
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                BackgroundColor(line),
            ));
        }
    });
}

/// Delay — qumsoat (ikki uchburchakdan yaqin).
fn spawn_hourglass_icon(parent: &mut ChildSpawnerCommands) {
    icon_slot(parent, |icon| {
        let sand = Color::srgb(1.0, 0.92, 0.7);
        icon.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(4.),
                top: px(2.),
                width: px(8.),
                height: px(2.),
                ..default()
            },
            BackgroundColor(sand),
        ));
        icon.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(5.5),
                top: px(4.),
                width: px(5.),
                height: px(4.),
                ..default()
            },
            BackgroundColor(sand),
        ));
        icon.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(5.5),
                top: px(8.),
                width: px(5.),
                height: px(4.),
                ..default()
            },
            BackgroundColor(sand),
        ));
        icon.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(4.),
                top: px(12.),
                width: px(8.),
                height: px(2.),
                ..default()
            },
            BackgroundColor(sand),
        ));
    });
}

/// Klaviatura — input tugunlari uchun (ikkita katak + spacebar).
fn spawn_keyboard_icon(parent: &mut ChildSpawnerCommands) {
    icon_slot(parent, |icon| {
        let key = Color::srgb(0.45, 0.92, 0.62);
        // Ustki qator: uchta kichik katak
        for (x, y) in [(2., 2.), (6., 2.), (10., 2.)] {
            icon.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: px(x),
                    top: px(y),
                    width: px(3.),
                    height: px(3.),
                    border_radius: BorderRadius::all(px(1.)),
                    ..default()
                },
                BackgroundColor(key),
            ));
        }
        // O'rta qator: ikkita katak
        for (x, y) in [(3., 7.), (9., 7.)] {
            icon.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: px(x),
                    top: px(y),
                    width: px(3.),
                    height: px(3.),
                    border_radius: BorderRadius::all(px(1.)),
                    ..default()
                },
                BackgroundColor(key),
            ));
        }
        // Pastki: spacebar
        icon.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(3.),
                top: px(12.),
                width: px(10.),
                height: px(2.),
                border_radius: BorderRadius::all(px(1.)),
                ..default()
            },
            BackgroundColor(key),
        ));
    });
}
