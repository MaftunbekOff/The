//! Toolbar layout and button bundles.

use bevy::prelude::*;

use crate::ui::components::{
    LoadButton, LoadSceneButton, RunButton, SaveButton, SaveRonButton, SaveSceneButton,
    StatusLabel, StopButton, ToggleCameraModeButton, ToggleEditorModeButton,
};

const BG_PANEL: Color = Color::srgb(0.12, 0.12, 0.15);
const ACCENT_RUN: Color = Color::srgb(0.2, 0.55, 0.32);
const ACCENT_SAVE: Color = Color::srgb(0.25, 0.4, 0.6);
const ACCENT_SAVE_RON: Color = Color::srgb(0.18, 0.45, 0.55);
const ACCENT_LOAD: Color = Color::srgb(0.35, 0.28, 0.52);
const ACCENT_STOP: Color = Color::srgb(0.55, 0.22, 0.2);
const ACCENT_SCENE: Color = Color::srgb(0.42, 0.28, 0.58);
const ACCENT_CAM: Color = Color::srgb(0.28, 0.45, 0.52);
const ACCENT_SAVE_SCENE: Color = Color::srgb(0.22, 0.48, 0.38);

pub(crate) fn spawn_toolbar(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: percent(100),
                height: px(44),
                padding: UiRect::horizontal(px(12)),
                column_gap: px(8),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                border: UiRect::bottom(px(1)),
                ..default()
            },
            BackgroundColor(BG_PANEL),
            BorderColor::all(Color::srgb(0.22, 0.22, 0.26)),
        ))
        .with_children(|bar| {
            bar.spawn((
                Text::new("Twelfth Editor"),
                TextFont { font_size: FontSize::Px(16.0), ..default() },
                TextColor(Color::srgb(0.92, 0.92, 0.95)),
            ));

            // Rejim almashtirish
            bar.spawn((
                ToggleEditorModeButton,
                Button,
                tool_button_bundle(ACCENT_SCENE),
                children![tool_label("Scene / Script")],
            ));

            // Script rejimi tugmalari
            bar.spawn((
                SaveRonButton,
                Button,
                tool_button_bundle(ACCENT_SAVE_RON),
                children![tool_label("Save RON")],
            ));
            bar.spawn((
                LoadButton,
                Button,
                tool_button_bundle(ACCENT_LOAD),
                children![tool_label("Load RON")],
            ));
            bar.spawn((
                SaveButton,
                Button,
                tool_button_bundle(ACCENT_SAVE),
                children![tool_label("Export DSL")],
            ));

            // Scene rejimi tugmalari
            bar.spawn((
                SaveSceneButton,
                Button,
                tool_button_bundle(ACCENT_SAVE_SCENE),
                children![tool_label("Sahnani saqlash")],
            ));
            bar.spawn((
                LoadSceneButton,
                Button,
                tool_button_bundle(ACCENT_LOAD),
                children![tool_label("Sahnani yuklash")],
            ));
            bar.spawn((
                ToggleCameraModeButton,
                Button,
                tool_button_bundle(ACCENT_CAM),
                children![tool_label("2D / 3D")],
            ));

            // PIE
            bar.spawn((
                RunButton,
                Button,
                tool_button_bundle(ACCENT_RUN),
                children![tool_label("Start")],
            ));
            bar.spawn((
                StopButton,
                Button,
                tool_button_bundle(ACCENT_STOP),
                children![tool_label("Stop")],
            ));
            bar.spawn((
                StatusLabel,
                Text::new("Holat: Tayyor"),
                TextFont { font_size: FontSize::Px(13.0), ..default() },
                TextColor(Color::srgb(0.65, 0.7, 0.75)),
            ));
        });
}

pub(crate) fn tool_button_bundle(color: Color) -> impl Bundle {
    (
        Node {
            padding: UiRect::axes(px(14), px(6)),
            border_radius: BorderRadius::all(px(4)),
            ..default()
        },
        BackgroundColor(color),
    )
}

pub(crate) fn tool_label(text: &str) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: FontSize::Px(14.0),
            ..default()
        },
        TextColor(Color::WHITE),
    )
}

pub(crate) fn palette_action_bundle() -> impl Bundle {
    (
        Node {
            width: percent(100),
            padding: UiRect::axes(px(10), px(6)),
            border_radius: BorderRadius::all(px(4)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.14, 0.16, 0.22)),
    )
}

pub(crate) fn palette_action_label(text: &str) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: FontSize::Px(12.0),
            ..default()
        },
        TextColor(Color::srgb(0.88, 0.9, 0.94)),
    )
}
