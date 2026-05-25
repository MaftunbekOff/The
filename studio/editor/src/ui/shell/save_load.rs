//! Toolbar save / load / export button actions.

use bevy::prelude::*;

use crate::export::export_blueprint;
use crate::graph::GraphResource;
use crate::persist::{read_graph_ron, write_graph_ron};
use crate::scene::persist::{default_project_dir, load_scene_ron, save_scene_ron};
use crate::scene::{SceneTree, types::SceneFile};
use crate::state::{
    EditorMode, LoadGraphRequest, PlayState, TerminalState, UndoStack, ViewportCameraMode,
};
use crate::ui::components::{
    LoadButton, LoadSceneButton, SaveButton, SaveRonButton, SaveSceneButton,
    SceneModeRoot, ScriptModeRoot, ToggleCameraModeButton, ToggleEditorModeButton,
};

const GRAPH_PATH: &str = "graph.ron";

pub(crate) fn do_export(graph: &GraphResource, terminal: &mut TerminalState, play: &mut PlayState) {
    match export_blueprint(graph) {
        Ok(path) => {
            terminal.log(format!("[export] Yozildi: {}", path.display()));
            terminal.log("[export] Keyin: cargo run -p twelfth_editor_play");
            *play = PlayState::Exported;
        }
        Err(err) => {
            terminal.log(format!("[export] Xato: {err}"));
            *play = PlayState::Idle;
        }
    }
}

// ── Graph save / load / export ────────────────────────────────────────────────

pub fn toolbar_buttons(
    graph: Res<GraphResource>,
    mut terminal: ResMut<TerminalState>,
    mut play: ResMut<PlayState>,
    mut undo: ResMut<UndoStack>,
    mut load_req: ResMut<LoadGraphRequest>,
    save_ron_q:   Query<&Interaction, (Changed<Interaction>, With<SaveRonButton>)>,
    load_q:       Query<&Interaction, (Changed<Interaction>, With<LoadButton>)>,
    save_q:       Query<&Interaction, (Changed<Interaction>, With<SaveButton>)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);

    let save_ron_pressed = (ctrl && keys.just_pressed(KeyCode::KeyS))
        || save_ron_q.iter().any(|i| *i == Interaction::Pressed);
    if save_ron_pressed {
        undo.push(graph.snapshot());
        match write_graph_ron(&graph, std::path::Path::new(GRAPH_PATH)) {
            Ok(()) => terminal.log(format!("[save] {GRAPH_PATH} ga yozildi (RON)")),
            Err(e) => terminal.log(format!("[save] Xato: {e}")),
        }
    }

    let load_pressed = (ctrl && keys.just_pressed(KeyCode::KeyO))
        || load_q.iter().any(|i| *i == Interaction::Pressed);
    if load_pressed {
        match read_graph_ron(std::path::Path::new(GRAPH_PATH)) {
            Ok(new_graph) => {
                terminal.log(format!("[load] {GRAPH_PATH} yuklandi — {} tugun", new_graph.nodes.len()));
                load_req.0 = Some(new_graph);
            }
            Err(e) => terminal.log(format!("[load] Xato: {e}")),
        }
    }

    for interaction in &save_q {
        if *interaction == Interaction::Pressed {
            terminal.log("[save] Graf eksport qilinmoqda…");
            do_export(&graph, &mut terminal, &mut play);
        }
    }
}

// ── Scene save / load ─────────────────────────────────────────────────────────

pub fn scene_toolbar_buttons(
    mut tree: ResMut<SceneTree>,
    mut terminal: ResMut<TerminalState>,
    save_scene_q: Query<&Interaction, (Changed<Interaction>, With<SaveSceneButton>)>,
    load_scene_q: Query<&Interaction, (Changed<Interaction>, With<LoadSceneButton>)>,
) {
    for interaction in &save_scene_q {
        if *interaction == Interaction::Pressed {
            let scene = SceneFile {
                version: 1,
                name: "main_scene".into(),
                entities: tree.entities.clone(),
            };
            let dir = default_project_dir();
            match save_scene_ron(&scene, &dir) {
                Ok(()) => terminal.log(format!(
                    "[scene] scenes/main_scene.ron saqlandi ({} entity)",
                    scene.entities.len()
                )),
                Err(e) => terminal.log(format!("[scene] Saqlash xatosi: {e}")),
            }
        }
    }

    for interaction in &load_scene_q {
        if *interaction == Interaction::Pressed {
            let dir = default_project_dir();
            match load_scene_ron("main_scene", &dir) {
                Ok(scene) => {
                    terminal.log(format!(
                        "[scene] main_scene.ron yuklandi ({} entity)",
                        scene.entities.len()
                    ));
                    tree.entities = scene.entities;
                    tree.selected_id = None;
                }
                Err(e) => terminal.log(format!("[scene] Yuklash xatosi: {e}")),
            }
        }
    }
}

// ── Editor mode toggle ────────────────────────────────────────────────────────

pub fn editor_mode_toggle(
    mut editor_mode: ResMut<EditorMode>,
    mut cam_mode: ResMut<ViewportCameraMode>,
    mut terminal: ResMut<TerminalState>,
    toggle_mode_q: Query<&Interaction, (Changed<Interaction>, With<ToggleEditorModeButton>)>,
    toggle_cam_q:  Query<&Interaction, (Changed<Interaction>, With<ToggleCameraModeButton>)>,
    mut script_root: Query<&mut Visibility, (With<ScriptModeRoot>, Without<SceneModeRoot>)>,
    mut scene_root: Query<&mut Visibility, (With<SceneModeRoot>, Without<ScriptModeRoot>)>,
) {
    for interaction in &toggle_mode_q {
        if *interaction == Interaction::Pressed {
            *editor_mode = match *editor_mode {
                EditorMode::Script => EditorMode::Scene,
                EditorMode::Scene  => EditorMode::Script,
            };
            let is_scene = *editor_mode == EditorMode::Scene;
            terminal.log(if is_scene { "[mode] Scene rejimi" } else { "[mode] Script rejimi" });
        }
    }

    if editor_mode.is_changed() {
        let is_scene = *editor_mode == EditorMode::Scene;
        for mut vis in &mut script_root {
            *vis = if is_scene { Visibility::Hidden } else { Visibility::Visible };
        }
        for mut vis in &mut scene_root {
            *vis = if is_scene { Visibility::Visible } else { Visibility::Hidden };
        }
    }

    for interaction in &toggle_cam_q {
        if *interaction == Interaction::Pressed {
            *cam_mode = match *cam_mode {
                ViewportCameraMode::TwoD   => ViewportCameraMode::ThreeD,
                ViewportCameraMode::ThreeD => ViewportCameraMode::TwoD,
            };
            terminal.log(match *cam_mode {
                ViewportCameraMode::TwoD   => "[viewport] 2D kamera",
                ViewportCameraMode::ThreeD => "[viewport] 3D kamera",
            });
        }
    }
}
