//! Graf tab bar — yangi tab qo'shish, o'tish, ko'rsatish.

use bevy::prelude::*;

use crate::graph::{GraphResource, VmNode};
use crate::state::{SavedGraphs, TerminalState, UndoStack};
use crate::ui::components::{GraphCanvasViewport, GraphTabBar, GraphTabButton, NewGraphButton};
use crate::ui::nodes::spawn_vm_node_commands;

const TAB_BG:        Color = Color::srgb(0.12, 0.12, 0.14);
const TAB_ACTIVE_BG: Color = Color::srgb(0.22, 0.44, 0.82);
const TAB_TEXT:      Color = Color::WHITE;
const NEW_BTN_BG:    Color = Color::srgb(0.14, 0.48, 0.28);

fn build_tab(commands: &mut Commands, name: &str, active: bool) -> Entity {
    let bg = if active { TAB_ACTIVE_BG } else { TAB_BG };
    let tab = commands.spawn((
        GraphTabButton { graph_name: name.to_string() },
        Button,
        Node {
            padding: UiRect::axes(Val::Px(12.0), Val::Px(4.0)),
            height: Val::Px(28.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(bg),
    )).id();
    let lbl = commands.spawn((
        Text::new(name.to_string()),
        TextFont { font_size: FontSize::Px(12.0), ..default() },
        TextColor(TAB_TEXT),
    )).id();
    commands.entity(tab).add_child(lbl);
    tab
}

fn build_new_btn(commands: &mut Commands) -> Entity {
    let btn = commands.spawn((
        NewGraphButton,
        Button,
        Node {
            width: Val::Px(28.0),
            height: Val::Px(24.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::left(Val::Px(4.0)),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(NEW_BTN_BG),
    )).id();
    let lbl = commands.spawn((
        Text::new("+"),
        TextFont { font_size: FontSize::Px(14.0), ..default() },
        TextColor(TAB_TEXT),
    )).id();
    commands.entity(btn).add_child(lbl);
    btn
}

/// Tab bar-ni boshlang'ich holda spawn qiladi (birinchi tab "Main").
#[allow(dead_code)]
pub fn spawn_tab_bar(commands: &mut Commands, parent: Entity) {
    let bar = commands.spawn((
        GraphTabBar,
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Px(36.0),
            padding: UiRect::horizontal(Val::Px(8.0)),
            column_gap: Val::Px(4.0),
            ..default()
        },
        BackgroundColor(TAB_BG),
    )).id();

    let main_tab = build_tab(commands, "Main", true);
    let new_btn  = build_new_btn(commands);
    commands.entity(bar).add_child(main_tab);
    commands.entity(bar).add_child(new_btn);
    commands.entity(parent).add_child(bar);
}

/// Tab bar-ni qayta render qiladi (graflar o'zgarganda).
pub fn rebuild_tab_bar(
    commands: &mut Commands,
    tab_bar_q: &Query<Entity, With<GraphTabBar>>,
    saved: &SavedGraphs,
) {
    let Ok(bar_entity) = tab_bar_q.single() else { return };
    commands.entity(bar_entity).despawn_related::<Children>();

    let active = saved.active_name.clone();
    let all_names = saved.all_tab_names(&active);

    for name in &all_names {
        let tab = build_tab(commands, name, *name == active.as_str());
        commands.entity(bar_entity).add_child(tab);
    }
    let new_btn = build_new_btn(commands);
    commands.entity(bar_entity).add_child(new_btn);
}

/// Tab bar-ni boshlang'ich holda to'ldiradi ("Main" tab + «+» tugmasi).
pub fn init_tab_bar_system(
    mut commands: Commands,
    tab_bar_q: Query<Entity, With<GraphTabBar>>,
    saved: Res<SavedGraphs>,
) {
    rebuild_tab_bar(&mut commands, &tab_bar_q, &saved);
}

/// Tab tugmalari: bosilganda grafni almashtiradi yoki yangi tab yaratadi.
pub fn tab_bar_system(
    mut commands: Commands,
    mut graph: ResMut<GraphResource>,
    mut saved: ResMut<SavedGraphs>,
    mut undo: ResMut<UndoStack>,
    mut terminal: ResMut<TerminalState>,
    canvas_q: Query<Entity, With<GraphCanvasViewport>>,
    tab_bar_q: Query<Entity, With<GraphTabBar>>,
    node_q: Query<(Entity, &VmNode)>,
    tab_buttons: Query<(&Interaction, &GraphTabButton), Changed<Interaction>>,
    new_buttons: Query<&Interaction, (Changed<Interaction>, With<NewGraphButton>)>,
) {
    let mut switched = false;

    // Mavjud tab tugmalariga bosish
    for (interaction, tab) in &tab_buttons {
        if *interaction != Interaction::Pressed { continue; }
        if tab.graph_name == saved.active_name { continue; }

        undo.push(graph.snapshot());
        let current_name = saved.active_name.clone();
        if let Some((new_graph, new_name)) = saved.switch_to(&current_name, graph.clone(), &tab.graph_name) {
            for (entity, _) in &node_q {
                commands.entity(entity).despawn();
            }
            *graph = new_graph;
            saved.active_name = new_name.clone();
            terminal.log(format!("[tab] '{}' grafiga o'tildi", new_name));

            if let Ok(viewport) = canvas_q.single() {
                for node_data in graph.nodes.clone() {
                    spawn_vm_node_commands(&mut commands, &node_data, viewport);
                }
            }
            switched = true;
        }
        break;
    }

    // «+» tugmasi — yangi bo'sh tab
    for interaction in &new_buttons {
        if *interaction != Interaction::Pressed { continue; }

        undo.push(graph.snapshot());
        let new_name = format!("Graph {}", saved.graphs.len() + 2);
        let current_name = saved.active_name.clone();
        let new_graph = saved.new_tab(&current_name, graph.clone(), &new_name);

        for (entity, _) in &node_q {
            commands.entity(entity).despawn();
        }
        *graph = new_graph;
        terminal.log(format!("[tab] Yangi graf '{}' yaratildi", new_name));
        switched = true;
        break;
    }

    if switched {
        rebuild_tab_bar(&mut commands, &tab_bar_q, &saved);
    }
}
