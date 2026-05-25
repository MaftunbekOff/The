//! SceneTree resursi — editor sahnasidagi entitylar holati.

use bevy::prelude::*;

use super::types::SceneEntityData;

/// Editor sahnasining holati: entitylar ro'yxati + tanlov.
#[derive(Resource, Default, Debug)]
pub struct SceneTree {
    /// Sahnada joylashgan entitylar.
    pub entities: Vec<SceneEntityData>,
    /// Tanlangan entity ID.
    pub selected_id: Option<u64>,
    /// Keyingi entity uchun ID hisoblagich.
    pub next_id: u64,
}

impl SceneTree {
    /// Yangi entity qo'shadi va uning ID sini qaytaradi.
    pub fn add(&mut self, data: SceneEntityData) -> u64 {
        let id = data.id;
        self.entities.push(data);
        id
    }

    /// ID bo'yicha entity topadi.
    pub fn get(&self, id: u64) -> Option<&SceneEntityData> {
        self.entities.iter().find(|e| e.id == id)
    }

    /// ID bo'yicha entity topadi (o'zgartirish uchun).
    pub fn get_mut(&mut self, id: u64) -> Option<&mut SceneEntityData> {
        self.entities.iter_mut().find(|e| e.id == id)
    }

    /// Yangi unikal ID generatsiya qiladi.
    pub fn alloc_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// ID bo'yicha entity o'chiradi.
    pub fn remove(&mut self, id: u64) {
        self.entities.retain(|e| e.id != id);
        if self.selected_id == Some(id) {
            self.selected_id = None;
        }
    }

    /// Farzand entitylarni qaytaradi.
    pub fn children_of(&self, parent_id: u64) -> Vec<&SceneEntityData> {
        self.entities
            .iter()
            .filter(|e| e.parent == Some(parent_id))
            .collect()
    }

    /// Yuqori darajali (ota yo'q) entitylarni qaytaradi.
    pub fn root_entities(&self) -> Vec<&SceneEntityData> {
        self.entities.iter().filter(|e| e.parent.is_none()).collect()
    }
}

/// Bevy `Entity` → `SceneEntityData.id` mapping.
/// Viewport da joylashtirilgan Bevy entitylarini scene ID bilan bog'laydi.
#[derive(Resource, Default, Debug)]
pub struct SceneEntityMap {
    /// scene_id → Bevy Entity
    pub bevy: std::collections::HashMap<u64, Entity>,
    /// Bevy Entity → scene_id
    pub scene: std::collections::HashMap<Entity, u64>,
}

impl SceneEntityMap {
    pub fn insert(&mut self, scene_id: u64, entity: Entity) {
        self.bevy.insert(scene_id, entity);
        self.scene.insert(entity, scene_id);
    }

    pub fn remove_by_scene_id(&mut self, scene_id: u64) {
        if let Some(entity) = self.bevy.remove(&scene_id) {
            self.scene.remove(&entity);
        }
    }

    pub fn get_entity(&self, scene_id: u64) -> Option<Entity> {
        self.bevy.get(&scene_id).copied()
    }
}

/// Marker component — editor tomonidan joylashtirilgan entity.
#[derive(Component, Debug, Clone)]
pub struct SceneEditorEntity {
    pub scene_id: u64,
}

/// Entity uchun ko'rsatish nomi.
#[derive(Component, Debug, Clone)]
pub struct SceneEntityName(pub String);
