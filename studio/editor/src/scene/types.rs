//! Scene entity uchun ma'lumot turlari.
//!
//! Bevy ECS dan mustaqil — sof Rust struct lar.
//! Serialize / Deserialize → RON fayl uchun.

use serde::{Deserialize, Serialize};

/// Sahnada joylashgan bitta entity ning ma'lumoti.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneEntityData {
    /// Editor tomonidan berilgan unikal ID.
    pub id: u64,
    /// Ko'rsatish nomi (Hierarchy panelida).
    pub name: String,
    /// Pozitsiya (x, y, z).
    pub position: [f32; 3],
    /// Miqyos (x, y, z).
    pub scale: [f32; 3],
    /// Burish — Euler burchaklari gradusda (x, y, z).
    pub rotation_deg: [f32; 3],
    /// Ota-entity ID (iyerarxiya uchun).
    pub parent: Option<u64>,
    /// Tayinlangan script faylining nomi (`scripts/` papkasidagi .ron).
    pub script: Option<String>,
    /// Vizual shakl turi.
    pub mesh: SceneMesh,
}

impl SceneEntityData {
    pub fn new(id: u64, name: impl Into<String>, position: [f32; 3]) -> Self {
        Self {
            id,
            name: name.into(),
            position,
            scale: [1.0, 1.0, 1.0],
            rotation_deg: [0.0, 0.0, 0.0],
            parent: None,
            script: None,
            mesh: SceneMesh::Quad { width: 50.0, height: 50.0 },
        }
    }
}

/// Entity ning vizual shakli.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SceneMesh {
    /// 2D to'rtburchak (default).
    #[serde(rename = "quad")]
    Quad { width: f32, height: f32 },
    /// 3D kub.
    #[serde(rename = "cube")]
    Cube { size: f32 },
    /// 3D sfera.
    #[serde(rename = "sphere")]
    Sphere { radius: f32 },
    /// Ko'rinmas (faqat transform holder).
    #[serde(rename = "empty")]
    Empty,
}

impl Default for SceneMesh {
    fn default() -> Self {
        SceneMesh::Quad { width: 50.0, height: 50.0 }
    }
}

/// Sahnaning to'liq ma'lumotlari (RON faylga yoziladi).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SceneFile {
    /// Fayl versiyasi — migratsiya uchun.
    pub version: u32,
    /// Sahna nomi.
    pub name: String,
    /// Barcha entitylar.
    pub entities: Vec<SceneEntityData>,
}

/// Loyiha metama'lumotlari (`project.ron`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMeta {
    /// Loyiha nomi.
    pub name: String,
    /// Boshlang'ich sahna (scenes/ papkasidagi .ron fayl nomi).
    pub startup_scene: String,
    /// Loyiha versiyasi.
    pub version: u32,
}

impl Default for ProjectMeta {
    fn default() -> Self {
        Self {
            name: "Yangi Loyiha".into(),
            startup_scene: "main_scene".into(),
            version: 1,
        }
    }
}
