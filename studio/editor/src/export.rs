//! Kanvas → `generated/blueprint.rs` (compile-time makro).

use std::fs;
use std::path::PathBuf;

use twelfth_visual_blueprint::export::graph_to_dsl;

use crate::graph::GraphResource;

/// `studio/editor/generated/blueprint.rs`
pub fn default_export_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("generated/blueprint.rs")
}

pub fn export_blueprint(graph: &GraphResource) -> Result<PathBuf, String> {
    let ast = graph.to_blueprint_graph()?;
    let dsl = graph_to_dsl(&ast)?;
    let path = default_export_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(&path, dsl).map_err(|e| e.to_string())?;
    Ok(path)
}
