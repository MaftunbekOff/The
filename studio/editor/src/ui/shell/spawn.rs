//! Editor shell spawn — root layout, canvas, terminal.

use bevy::prelude::*;

use crate::graph::{GraphResource, NodeId};
use crate::registry::NodeKind;
use crate::state::{SelectedNode, TerminalState};
use crate::ui::canvas::grid::{grid_bundle, GridMaterial};
use crate::ui::canvas::interaction::{
    on_canvas_pan_drag, on_canvas_pan_drag_end, on_canvas_pan_drag_start, on_canvas_press,
    on_canvas_release, on_canvas_sel_drag, on_canvas_sel_drag_end, on_canvas_sel_drag_start,
};
use bevy::text::{EditableText, TextCursorStyle};
use crate::ui::components::{
    GraphCanvas, GraphCanvasViewport, GraphTabBar, PaletteSearchInput, SceneModeRoot,
    ScriptModeRoot, SelectionRect, TerminalLabel, TerminalScrollArea, ViewportPanel,
};
use crate::ui::nodes::{
    spawn_palette_button, spawn_vm_node, DeleteNodeButton, DisconnectButton,
};
use crate::ui::shell::toolbar::{
    palette_action_bundle, palette_action_label, spawn_toolbar,
};
use crate::ui::hierarchy::spawn_hierarchy_panel;
use crate::ui::inspector::spawn_inspector_panel;
const BG_WINDOW: Color = Color::srgb(0.09, 0.09, 0.11);
const BG_PANEL: Color = Color::srgb(0.12, 0.12, 0.15);
const BG_CANVAS: Color = Color::srgb(0.07, 0.07, 0.09);
const BG_TERMINAL: Color = Color::srgb(0.05, 0.06, 0.07);

pub fn spawn_editor_shell(
    mut commands: Commands,
    graph: Res<GraphResource>,
    selected: Res<SelectedNode>,
    mut terminal: ResMut<TerminalState>,
    mut saved_graphs: ResMut<crate::state::SavedGraphs>,
    mut grid_materials: ResMut<Assets<GridMaterial>>,
) {
    saved_graphs.active_name = "Main".to_string();
    terminal.log("[boot] Twelfth editor — blueprint compile pipeline");
    terminal.log("[hint] Toolbar: 'Scene / Script' → rejim almashish");
    terminal.log("[hint] Kanvas: g'ildirak=zoom | O'rta_sichqoncha=pan | O'ng_klik_port=uzish | Del=o'chir");

    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(BG_WINDOW),
            Pickable::IGNORE,
        ))
        .with_children(|root| {
            spawn_toolbar(root);
            // Script rejimi (default, ko'rinadi)
            spawn_script_mode_row(root, &graph, selected.primary, &mut grid_materials);
            // Scene rejimi (boshlang'ichda yashirin)
            spawn_scene_mode_row(root);
            spawn_terminal_panel(root);
        });
}

fn spawn_script_mode_row(
    parent: &mut ChildSpawnerCommands,
    graph: &GraphResource,
    selected: Option<NodeId>,
    grid_materials: &mut Assets<GridMaterial>,
) {
    parent
        .spawn((
            ScriptModeRoot,
            Node {
                width: percent(100),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                ..default()
            },
        ))
        .with_children(|row| {
            // Palette
            row.spawn((
                Node {
                    width: px(200),
                    height: percent(100),
                    padding: UiRect::all(px(10)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(6),
                    border: UiRect::right(px(1)),
                    ..default()
                },
                BackgroundColor(BG_PANEL),
                BorderColor::all(Color::srgb(0.2, 0.2, 0.24)),
            ))
            .with_children(|palette| {
                palette.spawn((
                    Text::new("Blueprint tugunlar"),
                    TextFont { font_size: FontSize::Px(14.0), ..default() },
                    TextColor(Color::srgb(0.85, 0.88, 0.95)),
                ));
                palette.spawn((
                    PaletteSearchInput,
                    Node {
                        width: percent(100),
                        height: px(26.),
                        padding: UiRect::axes(px(7.), px(3.)),
                        border: UiRect::all(px(1.)),
                        border_radius: BorderRadius::all(px(4.)),
                        ..default()
                    },
                    EditableText::new(""),
                    TextCursorStyle::default(),
                    TextFont { font_size: FontSize::Px(12.0), ..default() },
                    TextColor(Color::srgb(0.8, 0.82, 0.88)),
                    BackgroundColor(Color::srgb(0.07, 0.07, 0.1)),
                    BorderColor::all(Color::srgb(0.3, 0.32, 0.42)),
                ));
                for kind in NodeKind::palette() {
                    spawn_palette_button(palette, kind);
                }
                palette.spawn(Node { height: px(8), ..default() });
                palette.spawn((
                    DeleteNodeButton,
                    Button,
                    palette_action_bundle(),
                    children![palette_action_label("O'chirish")],
                ));
                palette.spawn((
                    DisconnectButton,
                    Button,
                    palette_action_bundle(),
                    children![palette_action_label("Simlarni uzish")],
                ));
                palette.spawn((
                    Text::new("Exec (ko'k) va data (sariq) portlar.\nUzish: portda o'ng tugma."),
                    TextFont { font_size: FontSize::Px(10.0), ..default() },
                    TextColor(Color::srgb(0.5, 0.54, 0.58)),
                ));
            });

            // Canvas
            row.spawn((
                GraphCanvas,
                Pickable::default(),
                Node {
                    width: percent(100),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Relative,
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(BG_CANVAS),
            ))
            .observe(on_canvas_press)
            .observe(on_canvas_release)
            .observe(on_canvas_pan_drag_start)
            .observe(on_canvas_pan_drag)
            .observe(on_canvas_pan_drag_end)
            .observe(on_canvas_sel_drag_start)
            .observe(on_canvas_sel_drag)
            .observe(on_canvas_sel_drag_end)
            .with_children(|canvas| {
                let bar_entity = canvas.spawn((
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
                    BackgroundColor(Color::srgb(0.12, 0.12, 0.14)),
                )).id();
                let _ = bar_entity;

                canvas.spawn(grid_bundle(grid_materials));

                canvas
                    .spawn((
                        GraphCanvasViewport,
                        Node {
                            width: percent(100),
                            height: percent(100),
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        UiTransform::default(),
                    ))
                    .with_children(|viewport| {
                        viewport.spawn((
                            SelectionRect,
                            Pickable::IGNORE,
                            Visibility::Hidden,
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                top: Val::Px(0.0),
                                width: Val::Px(0.0),
                                height: Val::Px(0.0),
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.25, 0.45, 0.9, 0.12)),
                            BorderColor::all(Color::srgba(0.4, 0.65, 1.0, 0.75)),
                        ));

                        for node in &graph.nodes {
                            let sel = selected == Some(node.id);
                            spawn_vm_node(viewport, node, sel);
                        }
                    });
            });
        });
}

fn spawn_scene_mode_row(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            SceneModeRoot,
            Visibility::Hidden,
            Node {
                width: percent(100),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                ..default()
            },
        ))
        .with_children(|row| {
            // Sol: Hierarchy
            spawn_hierarchy_panel(row);

            // Markaz: Viewport
            row.spawn((
                ViewportPanel,
                Node {
                    flex_grow: 1.0,
                    height: percent(100),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.04, 0.05, 0.07)),
            ))
            .with_children(|vp| {
                vp.spawn((
                    Text::new("Viewport  [O'rta sichqoncha = pan | G'ildirak = zoom | 2D/3D tugmasi toolbarda]"),
                    TextFont { font_size: FontSize::Px(11.0), ..default() },
                    TextColor(Color::srgb(0.45, 0.5, 0.58)),
                    Node {
                        padding: UiRect::axes(px(8.0), px(4.0)),
                        border: UiRect::bottom(px(1.0)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.18, 0.2, 0.26)),
                ));
                // Viewport sahna ko'rinishi uchun joy (kamera bevosita oynaga render qiladi)
                vp.spawn((
                    ViewportPanel,
                    Node {
                        flex_grow: 1.0,
                        width: percent(100),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                ));
            });

            // O'ng: Inspector
            spawn_inspector_panel(row);
        });
}

fn spawn_terminal_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: percent(100),
                height: px(160),
                flex_direction: FlexDirection::Column,
                border: UiRect::top(px(1)),
                ..default()
            },
            BackgroundColor(BG_TERMINAL),
            BorderColor::all(Color::srgb(0.2, 0.2, 0.24)),
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new("Terminal / eksport"),
                TextFont { font_size: FontSize::Px(12.0), ..default() },
                TextColor(Color::srgb(0.55, 0.58, 0.62)),
                Node { padding: UiRect::new(px(8), px(8), px(4), px(4)), ..default() },
            ));
            panel
                .spawn((
                    TerminalScrollArea,
                    Node {
                        width: percent(100),
                        flex_grow: 1.0,
                        overflow: Overflow::scroll_y(),
                        padding: UiRect::all(px(8)),
                        ..default()
                    },
                ))
                .with_children(|scroll| {
                    scroll.spawn((
                        TerminalLabel,
                        Text::new(""),
                        TextFont { font_size: FontSize::Px(11.0), ..default() },
                        TextColor(Color::srgb(0.75, 0.78, 0.8)),
                    ));
                });
        });
}
