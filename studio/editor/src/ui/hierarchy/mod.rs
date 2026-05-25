//! Entity iyerarxiya paneli — Hierarchy.
//!
//! Scene rejimida chapda ko'rsatiladi.
//! SceneTree'dagi entitylarni daraxt ko'rinishida ro'yxatlaydi.

use bevy::prelude::*;

use crate::scene::SceneTree;

// ── Komponentlar ──────────────────────────────────────────────────────────────

/// Hierarchy panel root konteyneri.
#[derive(Component)]
pub struct HierarchyPanel;

/// Hierarchy panelida bitta entity qatori.
#[derive(Component, Clone)]
pub struct HierarchyRow {
    pub scene_id: u64,
}

/// "Yangi Entity" tugmasi.
#[derive(Component)]
pub struct AddEntityButton;

/// "Entity o'chirish" tugmasi.
#[derive(Component)]
pub struct DeleteEntityButton;

// ── Spawn ─────────────────────────────────────────────────────────────────────

const BG_HIER: Color = Color::srgb(0.10, 0.10, 0.13);
const BG_ROW: Color = Color::srgb(0.13, 0.13, 0.17);
const BG_ROW_SEL: Color = Color::srgb(0.18, 0.32, 0.52);
const BG_BTN: Color = Color::srgb(0.16, 0.42, 0.28);
const BG_DEL: Color = Color::srgb(0.45, 0.16, 0.16);

pub fn spawn_hierarchy_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            HierarchyPanel,
            Node {
                width: Val::Px(200.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                border: UiRect::right(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(BG_HIER),
            BorderColor::all(Color::srgb(0.2, 0.2, 0.26)),
        ))
        .with_children(|panel| {
            // Header
            panel.spawn((
                Text::new("Hierarchy"),
                TextFont { font_size: FontSize::Px(13.0), ..default() },
                TextColor(Color::srgb(0.85, 0.88, 0.95)),
                Node {
                    padding: UiRect::new(Val::Px(10.0), Val::Px(10.0), Val::Px(8.0), Val::Px(6.0)),
                    border: UiRect::bottom(Val::Px(1.0)),
                    ..default()
                },
                BorderColor::all(Color::srgb(0.22, 0.22, 0.28)),
            ));

            // "+ Entity" tugmasi
            panel.spawn((
                AddEntityButton,
                Button,
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                    border: UiRect::bottom(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(BG_BTN),
                BorderColor::all(Color::srgb(0.1, 0.3, 0.2)),
                children![(
                    Text::new("+ Entity qo'shish"),
                    TextFont { font_size: FontSize::Px(12.0), ..default() },
                    TextColor(Color::WHITE),
                )],
            ));

            // Entity ro'yxati
            panel.spawn((
                HierarchyList,
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    overflow: Overflow::scroll_y(),
                    ..default()
                },
            ));

            // Entity o'chirish
            panel.spawn((
                DeleteEntityButton,
                Button,
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                    border: UiRect::top(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(BG_DEL),
                BorderColor::all(Color::srgb(0.3, 0.1, 0.1)),
                children![(
                    Text::new("O'chirish"),
                    TextFont { font_size: FontSize::Px(12.0), ..default() },
                    TextColor(Color::WHITE),
                )],
            ));
        });
}

/// Hierarchy entity ro'yxati konteyneri.
#[derive(Component)]
pub struct HierarchyList;

// ── Sync tizimi ───────────────────────────────────────────────────────────────

/// `SceneTree` o'zgarganda Hierarchy panelini qayta quradi.
pub fn sync_hierarchy_panel(
    mut commands: Commands,
    tree: Res<SceneTree>,
    list_q: Query<Entity, With<HierarchyList>>,
    rows_q: Query<Entity, With<HierarchyRow>>,
) {
    if !tree.is_changed() { return; }

    let Ok(list_entity) = list_q.single() else { return };

    // Eski qatorlarni tozalash
    for row_entity in &rows_q {
        commands.entity(row_entity).despawn();
    }

    // Yangi qatorlar
    commands.entity(list_entity).with_children(|list| {
        for data in tree.root_entities() {
            spawn_hierarchy_row(list, data, 0, tree.selected_id);
        }
    });
}

fn spawn_hierarchy_row(
    parent: &mut ChildSpawnerCommands,
    data: &crate::scene::types::SceneEntityData,
    depth: u32,
    selected_id: Option<u64>,
) {
    let is_sel = selected_id == Some(data.id);
    let indent = depth as f32 * 14.0 + 8.0;
    let bg = if is_sel { BG_ROW_SEL } else { BG_ROW };
    let label = format!("{}", data.name);

    parent.spawn((
        HierarchyRow { scene_id: data.id },
        Button,
        Node {
            width: Val::Percent(100.0),
            padding: UiRect::new(Val::Px(indent), Val::Px(8.0), Val::Px(4.0), Val::Px(4.0)),
            border: UiRect::bottom(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(bg),
        BorderColor::all(Color::srgb(0.18, 0.18, 0.22)),
        children![(
            Text::new(label),
            TextFont { font_size: FontSize::Px(12.0), ..default() },
            TextColor(if is_sel { Color::WHITE } else { Color::srgb(0.82, 0.85, 0.9) }),
        )],
    ));
}

// ── Tugmalar ──────────────────────────────────────────────────────────────────

/// Hierarchy qator bosilganda entity ni tanlaydi.
pub fn hierarchy_row_clicks(
    rows_q: Query<(&HierarchyRow, &Interaction), (Changed<Interaction>, With<Button>)>,
    mut tree: ResMut<SceneTree>,
) {
    for (row, interaction) in &rows_q {
        if *interaction == Interaction::Pressed {
            tree.selected_id = Some(row.scene_id);
        }
    }
}

/// "Entity qo'shish" tugmasi.
pub fn add_entity_button(
    btn_q: Query<&Interaction, (Changed<Interaction>, With<AddEntityButton>)>,
    mut tree: ResMut<SceneTree>,
) {
    for interaction in &btn_q {
        if *interaction == Interaction::Pressed {
            let id = tree.alloc_id();
            let pos = [
                (id % 5) as f32 * 80.0 - 160.0,
                (id / 5) as f32 * 80.0 - 120.0,
                0.0,
            ];
            let data = crate::scene::types::SceneEntityData::new(
                id,
                format!("Entity {id}"),
                pos,
            );
            tree.add(data);
            tree.selected_id = Some(id);
        }
    }
}

/// "Entity o'chirish" tugmasi.
pub fn delete_entity_button(
    btn_q: Query<&Interaction, (Changed<Interaction>, With<DeleteEntityButton>)>,
    mut tree: ResMut<SceneTree>,
) {
    for interaction in &btn_q {
        if *interaction == Interaction::Pressed {
            if let Some(id) = tree.selected_id {
                tree.remove(id);
            }
        }
    }
}
