//! Viewport: 2D sahna kamerasi tizimi.
//!
//! Render-to-texture o'rniga sahna kamerasi to'g'ridan-to'g'ri
//! alohida render layer'da ishlaydi. Scene Mode da UI overlay ko'rinadi.

use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::state::{EditorMode, ViewportCameraMode};
use crate::scene::{SceneEditorEntity, SceneEntityName, SceneTree, SceneMesh, SceneEntityMap};
use crate::scene::types::SceneEntityData;

// ── Komponentlar / Resurslar ──────────────────────────────────────────────────

/// Sahna kamerasi markeri.
#[derive(Component)]
pub struct ViewportCamera;

/// Render texture handle (viewport panelida ko'rsatish uchun).
#[derive(Resource, Default)]
#[allow(dead_code)]
pub struct ViewportRenderImage(pub Handle<Image>);

/// Kamera orbit holati.
#[derive(Resource, Default)]
pub struct OrbitState {
    pub dragging: bool,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}

/// Yangi entity joylashtirishni so'rash.
#[derive(Resource, Default)]
pub struct SpawnEntityRequest {
    /// Viewport CSS koordinatasida klik qilingan nuqta (kelajakda ishlatiladi).
    #[allow(dead_code)]
    pub pending_position: Option<Vec2>,
}

// ── Setup ─────────────────────────────────────────────────────────────────────

pub fn setup_viewport_camera(
    mut commands: Commands,
    mut orbit: ResMut<OrbitState>,
) {
    orbit.yaw = 0.0;
    orbit.pitch = -0.4;
    orbit.distance = 500.0;

    // 2D sahna kamerasi — UI kamera (order 0) dan keyin ishlaydi
    // Scene entitylari faqat shu kamera render layerida ko'rinadi
    commands.spawn((
        ViewportCamera,
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.05, 0.06, 0.08)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 999.0),
    ));
}

// ── Kamera boshqaruvi ─────────────────────────────────────────────────────────

pub fn viewport_camera_pan_2d(
    mode: Res<EditorMode>,
    cam_mode: Res<ViewportCameraMode>,
    mut cam_q: Query<&mut Transform, With<ViewportCamera>>,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    mut motion: MessageReader<MouseMotion>,
    mut scroll: MessageReader<MouseWheel>,
    mut orbit: ResMut<OrbitState>,
) {
    if *mode != EditorMode::Scene { return; }
    if *cam_mode != ViewportCameraMode::TwoD { return; }
    let Ok(mut tf) = cam_q.single_mut() else { return };

    for ev in scroll.read() {
        let y = match ev.unit { MouseScrollUnit::Line => ev.y * 20.0, MouseScrollUnit::Pixel => ev.y };
        let factor: f32 = 1.0 - y * 0.05;
        tf.scale *= Vec3::splat(factor.clamp(0.1, 10.0));
    }

    if mouse_btn.pressed(MouseButton::Middle) {
        if !orbit.dragging { orbit.dragging = true; }
        for ev in motion.read() {
            let scale = tf.scale.x;
            tf.translation.x -= ev.delta.x * scale;
            tf.translation.y += ev.delta.y * scale;
        }
    } else {
        orbit.dragging = false;
    }
}

pub fn viewport_camera_orbit_3d(
    mode: Res<EditorMode>,
    cam_mode: Res<ViewportCameraMode>,
    mut cam_q: Query<&mut Transform, With<ViewportCamera>>,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    mut motion: MessageReader<MouseMotion>,
    mut scroll: MessageReader<MouseWheel>,
    mut orbit: ResMut<OrbitState>,
) {
    if *mode != EditorMode::Scene { return; }
    if *cam_mode != ViewportCameraMode::ThreeD { return; }

    for ev in scroll.read() {
        let y = match ev.unit { MouseScrollUnit::Line => ev.y * 20.0, MouseScrollUnit::Pixel => ev.y };
        orbit.distance = (orbit.distance - y * 5.0).clamp(50.0, 5000.0);
    }

    if mouse_btn.pressed(MouseButton::Middle) {
        for ev in motion.read() {
            orbit.yaw   -= ev.delta.x * 0.005;
            orbit.pitch -= ev.delta.y * 0.005;
            orbit.pitch = orbit.pitch.clamp(-1.5, 1.5);
        }
    }

    let Ok(mut tf) = cam_q.single_mut() else { return };
    let rot = Quat::from_rotation_y(orbit.yaw) * Quat::from_rotation_x(orbit.pitch);
    let offset = rot * Vec3::new(0.0, 0.0, orbit.distance);
    tf.translation = offset;
    tf.look_at(Vec3::ZERO, Vec3::Y);
}

pub fn sync_viewport_camera_mode(
    cam_mode: Res<ViewportCameraMode>,
    mut cam_q: Query<&mut Transform, With<ViewportCamera>>,
    mut orbit: ResMut<OrbitState>,
) {
    if !cam_mode.is_changed() { return; }
    let Ok(mut tf) = cam_q.single_mut() else { return };
    match *cam_mode {
        ViewportCameraMode::TwoD => {
            tf.translation = Vec3::new(0.0, 0.0, 999.0);
            tf.rotation = Quat::IDENTITY;
            tf.scale = Vec3::ONE;
        }
        ViewportCameraMode::ThreeD => {
            orbit.yaw = 0.3;
            orbit.pitch = -0.4;
            orbit.distance = 500.0;
        }
    }
}

// ── Scene entity spawning ─────────────────────────────────────────────────────

pub fn sync_scene_tree_to_ecs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tree: Res<SceneTree>,
    mut entity_map: ResMut<SceneEntityMap>,
) {
    if !tree.is_changed() { return; }

    for data in &tree.entities {
        if entity_map.get_entity(data.id).is_some() { continue; }
        let pos = Vec3::from(data.position);
        let scale = Vec3::from(data.scale);
        let entity = spawn_scene_entity(&mut commands, &mut meshes, &mut materials, data, pos, scale);
        entity_map.insert(data.id, entity);
    }

    let scene_ids: std::collections::HashSet<u64> = tree.entities.iter().map(|e| e.id).collect();
    let to_remove: Vec<u64> = entity_map.bevy.keys().copied().filter(|id| !scene_ids.contains(id)).collect();
    for id in to_remove {
        if let Some(entity) = entity_map.bevy.get(&id).copied() {
            commands.entity(entity).despawn();
        }
        entity_map.remove_by_scene_id(id);
    }
}

fn spawn_scene_entity(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    data: &SceneEntityData,
    pos: Vec3,
    scale: Vec3,
) -> Entity {
    let color = entity_color(data.id);
    let mesh_handle = match &data.mesh {
        SceneMesh::Quad { width, height } => meshes.add(Rectangle::new(*width, *height)),
        SceneMesh::Cube { size } => meshes.add(Rectangle::new(*size, *size)),
        SceneMesh::Sphere { radius } => meshes.add(Circle::new(*radius)),
        SceneMesh::Empty => meshes.add(Rectangle::new(1.0, 1.0)),
    };
    let mat = materials.add(ColorMaterial::from_color(color));

    commands.spawn((
        SceneEditorEntity { scene_id: data.id },
        SceneEntityName(data.name.clone()),
        Mesh2d(mesh_handle),
        MeshMaterial2d(mat),
        Transform::from_translation(pos).with_scale(scale),
    )).id()
}

fn entity_color(id: u64) -> Color {
    let h = (id * 137 % 360) as f32;
    Color::hsl(h, 0.65, 0.55)
}

pub fn sync_scene_entity_transforms(
    mut tree: ResMut<SceneTree>,
    map_by_bevy: Query<(&Transform, &SceneEditorEntity)>,
    changed: Query<&SceneEditorEntity, Changed<Transform>>,
) {
    if changed.is_empty() { return; }
    for (tf, se) in &map_by_bevy {
        if let Some(data) = tree.get_mut(se.scene_id) {
            data.position = tf.translation.into();
            data.scale = tf.scale.into();
        }
    }
}
