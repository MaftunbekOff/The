//! Scene va loyiha fayllarini RON formatida saqlash / yuklash.

use std::path::{Path, PathBuf};

use super::types::{ProjectMeta, SceneFile};

const SCENE_VERSION: u32 = 1;

// ── Project ───────────────────────────────────────────────────────────────────

/// `project.ron` ni yozadi.
pub fn save_project_ron(meta: &ProjectMeta, dir: &Path) -> std::io::Result<()> {
    let path = dir.join("project.ron");
    let text = ron::ser::to_string_pretty(
        meta,
        ron::ser::PrettyConfig::default(),
    )
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    std::fs::write(path, text)
}

/// `project.ron` ni o'qiydi.
pub fn load_project_ron(dir: &Path) -> Result<ProjectMeta, String> {
    let path = dir.join("project.ron");
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("project.ron o'qib bo'lmadi: {e}"))?;
    ron::de::from_str(&text)
        .map_err(|e| format!("project.ron parse xatosi: {e}"))
}

// ── Scene ─────────────────────────────────────────────────────────────────────

/// `scenes/<name>.ron` ni yozadi. `dir` — loyiha katalogi.
pub fn save_scene_ron(scene: &SceneFile, dir: &Path) -> std::io::Result<()> {
    let scenes_dir = dir.join("scenes");
    std::fs::create_dir_all(&scenes_dir)?;
    let path = scenes_dir.join(format!("{}.ron", scene.name));
    let mut versioned = scene.clone();
    versioned.version = SCENE_VERSION;
    let text = ron::ser::to_string_pretty(
        &versioned,
        ron::ser::PrettyConfig::default(),
    )
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    std::fs::write(path, text)
}

/// `scenes/<name>.ron` ni o'qiydi.
pub fn load_scene_ron(name: &str, dir: &Path) -> Result<SceneFile, String> {
    let path = dir.join("scenes").join(format!("{name}.ron"));
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("Sahna '{name}' o'qib bo'lmadi: {e}"))?;
    ron::de::from_str(&text)
        .map_err(|e| format!("Sahna '{name}' parse xatosi: {e}"))
}

/// Joriy ishchi katalog — loyiha papkasi sifatida.
pub fn default_project_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}
