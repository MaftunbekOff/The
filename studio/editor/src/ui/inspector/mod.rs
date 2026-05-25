//! Inspector paneli — tanlangan entity Transform va script.

use bevy::prelude::*;

use crate::scene::{SceneTree};

// ── Komponentlar ──────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct InspectorPanel;

#[derive(Component)]
pub struct InspectorNameLabel;

#[derive(Component)]
pub struct InspectorPosX;
#[derive(Component)]
pub struct InspectorPosY;
#[derive(Component)]
pub struct InspectorPosZ;

#[derive(Component)]
pub struct InspectorScaleX;
#[derive(Component)]
pub struct InspectorScaleY;

#[derive(Component)]
pub struct InspectorScriptLabel;

#[derive(Component)]
pub struct InspectorNoSelection;

// ── Spawn ─────────────────────────────────────────────────────────────────────

const BG: Color = Color::srgb(0.10, 0.10, 0.13);
const BG_FIELD: Color = Color::srgb(0.07, 0.07, 0.10);
const LABEL_COL: Color = Color::srgb(0.65, 0.7, 0.78);
const VALUE_COL: Color = Color::srgb(0.92, 0.95, 1.0);

pub fn spawn_inspector_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            InspectorPanel,
            Node {
                width: Val::Px(220.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                border: UiRect::left(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(BG),
            BorderColor::all(Color::srgb(0.22, 0.22, 0.28)),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Inspector"),
                TextFont { font_size: FontSize::Px(13.0), ..default() },
                TextColor(Color::srgb(0.85, 0.88, 0.95)),
                Node {
                    padding: UiRect::bottom(Val::Px(6.0)),
                    border: UiRect::bottom(Val::Px(1.0)),
                    ..default()
                },
                BorderColor::all(Color::srgb(0.22, 0.22, 0.28)),
            ));

            // Hech narsa tanlanmagan xabari
            p.spawn((
                InspectorNoSelection,
                Text::new("Entity tanlanmagan"),
                TextFont { font_size: FontSize::Px(11.0), ..default() },
                TextColor(Color::srgb(0.5, 0.52, 0.58)),
            ));

            // Nom
            p.spawn((
                Text::new("Nom:"),
                TextFont { font_size: FontSize::Px(11.0), ..default() },
                TextColor(LABEL_COL),
            ));
            p.spawn((
                InspectorNameLabel,
                Text::new("—"),
                TextFont { font_size: FontSize::Px(12.0), ..default() },
                TextColor(VALUE_COL),
                Node {
                    padding: UiRect::axes(Val::Px(6.0), Val::Px(3.0)),
                    border_radius: BorderRadius::all(Val::Px(3.0)),
                    ..default()
                },
                BackgroundColor(BG_FIELD),
            ));

            // Position
            p.spawn((
                Text::new("Pozitsiya"),
                TextFont { font_size: FontSize::Px(11.0), ..default() },
                TextColor(LABEL_COL),
                Node { margin: UiRect::top(Val::Px(6.0)), ..default() },
            ));
            spawn_vec3_row(p, "X:", InspectorPosX, "0.0");
            spawn_vec3_row(p, "Y:", InspectorPosY, "0.0");
            spawn_vec3_row(p, "Z:", InspectorPosZ, "0.0");

            // Scale
            p.spawn((
                Text::new("Miqyos"),
                TextFont { font_size: FontSize::Px(11.0), ..default() },
                TextColor(LABEL_COL),
                Node { margin: UiRect::top(Val::Px(6.0)), ..default() },
            ));
            spawn_vec3_row(p, "X:", InspectorScaleX, "1.0");
            spawn_vec3_row(p, "Y:", InspectorScaleY, "1.0");

            // Script
            p.spawn((
                Text::new("Script"),
                TextFont { font_size: FontSize::Px(11.0), ..default() },
                TextColor(LABEL_COL),
                Node { margin: UiRect::top(Val::Px(6.0)), ..default() },
            ));
            p.spawn((
                InspectorScriptLabel,
                Text::new("—"),
                TextFont { font_size: FontSize::Px(11.0), ..default() },
                TextColor(Color::srgb(0.75, 0.88, 0.65)),
                Node {
                    padding: UiRect::axes(Val::Px(6.0), Val::Px(3.0)),
                    border_radius: BorderRadius::all(Val::Px(3.0)),
                    ..default()
                },
                BackgroundColor(BG_FIELD),
            ));
        });
}

fn spawn_vec3_row(parent: &mut ChildSpawnerCommands, label: &str, marker: impl Component, default_val: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(4.0),
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont { font_size: FontSize::Px(11.0), ..default() },
                TextColor(LABEL_COL),
                Node { width: Val::Px(18.0), ..default() },
            ));
            row.spawn((
                marker,
                Text::new(default_val),
                TextFont { font_size: FontSize::Px(11.0), ..default() },
                TextColor(VALUE_COL),
                Node {
                    flex_grow: 1.0,
                    padding: UiRect::axes(Val::Px(5.0), Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(3.0)),
                    ..default()
                },
                BackgroundColor(BG_FIELD),
            ));
        });
}

// ── Sync ─────────────────────────────────────────────────────────────────────

/// Tanlangan entity ma'lumotlarini Inspector'da ko'rsatadi.
pub fn sync_inspector(
    tree: Res<SceneTree>,
    mut name_q: Query<&mut Text, With<InspectorNameLabel>>,
    mut pos_x: Query<&mut Text, (With<InspectorPosX>, Without<InspectorNameLabel>, Without<InspectorPosY>, Without<InspectorPosZ>, Without<InspectorScaleX>, Without<InspectorScaleY>, Without<InspectorScriptLabel>)>,
    mut pos_y: Query<&mut Text, (With<InspectorPosY>, Without<InspectorNameLabel>, Without<InspectorPosX>, Without<InspectorPosZ>, Without<InspectorScaleX>, Without<InspectorScaleY>, Without<InspectorScriptLabel>)>,
    mut pos_z: Query<&mut Text, (With<InspectorPosZ>, Without<InspectorNameLabel>, Without<InspectorPosX>, Without<InspectorPosY>, Without<InspectorScaleX>, Without<InspectorScaleY>, Without<InspectorScriptLabel>)>,
    mut scl_x: Query<&mut Text, (With<InspectorScaleX>, Without<InspectorNameLabel>, Without<InspectorPosX>, Without<InspectorPosY>, Without<InspectorPosZ>, Without<InspectorScaleY>, Without<InspectorScriptLabel>)>,
    mut scl_y: Query<&mut Text, (With<InspectorScaleY>, Without<InspectorNameLabel>, Without<InspectorPosX>, Without<InspectorPosY>, Without<InspectorPosZ>, Without<InspectorScaleX>, Without<InspectorScriptLabel>)>,
    mut script_q: Query<&mut Text, (With<InspectorScriptLabel>, Without<InspectorNameLabel>, Without<InspectorPosX>, Without<InspectorPosY>, Without<InspectorPosZ>, Without<InspectorScaleX>, Without<InspectorScaleY>)>,
    mut no_sel_q: Query<&mut Visibility, With<InspectorNoSelection>>,
) {
    if !tree.is_changed() { return; }

    let Some(id) = tree.selected_id else {
        if let Ok(mut vis) = no_sel_q.single_mut() { *vis = Visibility::Visible; }
        return;
    };
    if let Ok(mut vis) = no_sel_q.single_mut() { *vis = Visibility::Hidden; }

    let Some(data) = tree.get(id) else { return };

    if let Ok(mut t) = name_q.single_mut() { **t = data.name.clone(); }
    if let Ok(mut t) = pos_x.single_mut() { **t = format!("{:.2}", data.position[0]); }
    if let Ok(mut t) = pos_y.single_mut() { **t = format!("{:.2}", data.position[1]); }
    if let Ok(mut t) = pos_z.single_mut() { **t = format!("{:.2}", data.position[2]); }
    if let Ok(mut t) = scl_x.single_mut() { **t = format!("{:.2}", data.scale[0]); }
    if let Ok(mut t) = scl_y.single_mut() { **t = format!("{:.2}", data.scale[1]); }
    if let Ok(mut t) = script_q.single_mut() {
        **t = data.script.as_deref().unwrap_or("—").to_string();
    }
}
