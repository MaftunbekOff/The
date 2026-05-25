//! Twelfth dev editor — blueprint kanvasi → compile-time eksport.

#![forbid(unsafe_code)]
#![allow(missing_docs, reason = "dev editor; docs later")]

mod export;
mod graph;
mod interpreter;
pub mod nodes;
mod persist;
mod registry;
pub mod scene;
mod state;
mod ui;

pub use export::{default_export_path, export_blueprint};
pub use graph::{GraphPort, GraphResource, GraphWire, NodeId, VmNode};
pub use interpreter::{
    BlackboardResource, PieDelayQueue, PiePendingGraph, PieSession, PieStartRequested,
};
pub use registry::{NodeKind, PinRef, WireCategory};
pub use scene::{SceneEditorEntity, SceneEntityMap, SceneEntityName, SceneTree};
pub use state::{
    ActiveNodeDrag, ConnectingState, EditorMode, LoadGraphRequest, PaletteFilter, PlayState,
    SavedGraphs, SelectedNode, SelectionBoxState, TerminalState, UndoStack, ViewportCameraMode,
};
pub use ui::TwelfthEditorPlugin;
