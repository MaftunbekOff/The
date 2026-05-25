//! Editor shell layout and interaction systems.

mod canvas;
mod components;
mod hierarchy;
mod inspector;
mod node_icons;
mod nodes;
mod param_fields;
mod scroll;
mod shell;
pub mod viewport;

use bevy::prelude::*;

use crate::interpreter::{
    PieDelayQueue, PieEntityTable, PiePendingGraph, PieRuntimeError, PieSession, PieStartRequested,
};
use crate::interpreter::resources::BlackboardResource;
use crate::graph::GraphResource;
use crate::scene::{SceneTree, SceneEntityMap};
use crate::state::{
    ActiveNodeDrag, ConnectingState, EditorMode, LoadGraphRequest, PaletteFilter, PlayState,
    SavedGraphs, SelectedNode, SelectionBoxState, TerminalState, UndoStack, ViewportCameraMode,
};
use crate::ui::canvas::coords::GraphCanvasView;
use crate::ui::canvas::grid::GridPlugin;
use crate::ui::canvas::interaction::{
    canvas_wheel_pan_zoom, handle_delete_key, sync_graph_viewport_transform,
    sync_selection_rect, update_connecting_cursor, CanvasPanDrag,
};
use crate::ui::canvas::render::{sync_graph_wires, WireSegmentPool};
use crate::ui::shell::{
    editor_mode_toggle, init_tab_bar_system, load_graph_system, node_delete_buttons,
    node_disconnect_buttons, node_palette_buttons, pie_control_buttons, scene_toolbar_buttons,
    spawn_editor_shell, sync_node_selection_visual, sync_palette_search, sync_pie_error_badge,
    sync_status_text, sync_terminal_text, tab_bar_system, toolbar_buttons, undo_redo_system,
};
use crate::ui::viewport::{
    setup_viewport_camera, sync_scene_tree_to_ecs, sync_scene_entity_transforms,
    viewport_camera_pan_2d, viewport_camera_orbit_3d, sync_viewport_camera_mode,
    OrbitState, SpawnEntityRequest, ViewportRenderImage,
};
use crate::ui::hierarchy::{
    sync_hierarchy_panel, hierarchy_row_clicks, add_entity_button, delete_entity_button,
};
use crate::ui::inspector::sync_inspector;

pub struct TwelfthEditorPlugin;

impl Plugin for TwelfthEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GridPlugin)
            // Graph
            .init_resource::<GraphResource>()
            // Editor state
            .init_resource::<TerminalState>()
            .init_resource::<ConnectingState>()
            .init_resource::<SelectedNode>()
            .init_resource::<SelectionBoxState>()
            .init_resource::<ActiveNodeDrag>()
            .init_resource::<WireSegmentPool>()
            .init_resource::<GraphCanvasView>()
            .init_resource::<CanvasPanDrag>()
            // PIE
            .init_resource::<PieSession>()
            .init_resource::<PieStartRequested>()
            .init_resource::<PiePendingGraph>()
            .init_resource::<PieDelayQueue>()
            .init_resource::<PieEntityTable>()
            .init_resource::<BlackboardResource>()
            .init_resource::<PieRuntimeError>()
            // Undo / tabs
            .init_resource::<UndoStack>()
            .init_resource::<PaletteFilter>()
            .init_resource::<LoadGraphRequest>()
            .init_resource::<SavedGraphs>()
            .insert_resource(PlayState::default())
            // Scene editor
            .init_resource::<EditorMode>()
            .init_resource::<ViewportCameraMode>()
            .init_resource::<SceneTree>()
            .init_resource::<SceneEntityMap>()
            .init_resource::<OrbitState>()
            .init_resource::<SpawnEntityRequest>()
            .init_resource::<ViewportRenderImage>()
            // Startup: viewport camera FIRST, then shell (uses ViewportRenderImage)
            .add_systems(
                Startup,
                (setup_viewport_camera, spawn_editor_shell, init_tab_bar_system).chain(),
            )
            // Scroll
            .add_systems(Update, (scroll::send_scroll_events, canvas_wheel_pan_zoom))
            .add_observer(scroll::on_scroll_handler)
            // Toolbar + graph actions
            .add_systems(Update, toolbar_buttons)
            .add_systems(Update, scene_toolbar_buttons)
            .add_systems(Update, editor_mode_toggle)
            .add_systems(
                Update,
                (
                    pie_control_buttons,
                    node_palette_buttons,
                    node_delete_buttons,
                    node_disconnect_buttons,
                    handle_delete_key,
                    undo_redo_system,
                    load_graph_system,
                    tab_bar_system,
                ).chain(),
            )
            // UI sinxronizatsiya
            .add_systems(
                Update,
                (
                    sync_node_selection_visual,
                    sync_selection_rect,
                    sync_terminal_text,
                    sync_status_text,
                    sync_palette_search,
                    sync_pie_error_badge,
                    param_fields::sync_node_param_inputs,
                ).chain(),
            )
            // Scene editor tizimlar (tuple limit: max 20 params per system, max ~9 per tuple)
            .add_systems(
                Update,
                (
                    sync_scene_tree_to_ecs,
                    sync_scene_entity_transforms,
                    sync_hierarchy_panel,
                    hierarchy_row_clicks,
                    add_entity_button,
                ),
            )
            .add_systems(
                Update,
                (
                    delete_entity_button,
                    sync_inspector,
                    viewport_camera_pan_2d,
                    viewport_camera_orbit_3d,
                    sync_viewport_camera_mode,
                ),
            )
            // PIE tizimi
            .add_systems(
                Update,
                (
                    crate::interpreter::systems::pie_start_system,
                    crate::interpreter::systems::pie_tick_system,
                    crate::interpreter::systems::pie_delay_tick_system,
                    crate::interpreter::systems::script_actor_tick_system,
                ).chain(),
            )
            .add_systems(
                PostUpdate,
                sync_graph_viewport_transform.before(bevy::ui::UiSystems::Layout),
            )
            .add_systems(
                PostUpdate,
                (update_connecting_cursor, sync_graph_wires)
                    .chain()
                    .after(bevy::ui::UiSystems::Stack),
            );
    }
}
