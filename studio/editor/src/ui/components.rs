//! Editor shell marker components.

use bevy::prelude::*;

/// Marker for the status line next to Run / Stop.
#[derive(Component)]
pub struct StatusLabel;

/// Scrollable terminal body.
#[derive(Component)]
pub struct TerminalScrollArea;

/// Terminal text entity (inside [`TerminalScrollArea`]).
#[derive(Component)]
pub struct TerminalLabel;

/// Graph drawing area root (clip + pan/zoom kiritish).
#[derive(Component)]
pub struct GraphCanvas;

/// Tugunlar va simlar — `GraphCanvasView` transform qo'llanadi.
#[derive(Component)]
pub struct GraphCanvasViewport;

/// One visual edge segment (child of [`GraphCanvasViewport`]).
#[derive(Component)]
pub struct GraphWire;

/// Toolbar «Start» (PIE) tugmasi.
#[derive(Component)]
pub struct RunButton;

#[derive(Component)]
pub struct StopButton;

/// Graf DSL eksport (eski Save → generated/blueprint.rs).
#[derive(Component)]
pub struct SaveButton;

/// Graf RON formatida saqlash (graph.ron).
#[derive(Component)]
pub struct SaveRonButton;

/// Graf RON fayldan yuklash (graph.ron).
#[derive(Component)]
pub struct LoadButton;

/// Palette ustidagi qidiruv matn maydoni.
#[derive(Component)]
pub struct PaletteSearchInput;

/// Tab bar tugmasi — Graf nomini saqlaydi.
#[derive(Component, Clone)]
pub struct GraphTabButton {
    pub graph_name: String,
}

/// Tab bar «+ Yangi Graf» tugmasi.
#[derive(Component)]
pub struct NewGraphButton;

/// Tab bar konteyner (respawn uchun marker).
#[derive(Component)]
pub struct GraphTabBar;

/// Rubber-band tanlov to'rtburchagi — viewport ichida, tugunlar bilan bir fazoda.
#[derive(Component)]
pub struct SelectionRect;

// ── Scene editor komponentlari ────────────────────────────────────────────────

/// Scene rejimi: kamera 2D/3D almashtirish tugmasi.
#[derive(Component)]
pub struct ToggleCameraModeButton;

/// Scene rejimi: sahna saqlash tugmasi.
#[derive(Component)]
pub struct SaveSceneButton;

/// Scene rejimi: sahna yuklash tugmasi.
#[derive(Component)]
pub struct LoadSceneButton;

/// Scene / Script rejim almashtirish tugmasi.
#[derive(Component)]
pub struct ToggleEditorModeButton;

/// Viewport UI panel — `UiImage` uchun marker.
#[derive(Component)]
pub struct ViewportPanel;

/// Scene rejimi asosiy qator konteyneri.
#[derive(Component)]
pub struct SceneModeRoot;

/// Script rejimi asosiy qator konteyneri.
#[derive(Component)]
pub struct ScriptModeRoot;
