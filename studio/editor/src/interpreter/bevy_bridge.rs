//! Bevy API ko'prisi — tizimning qolgan qismi bu fayl orqali Bevy ga murojaat qiladi.
//!
//! # Arxitektura kafolati
//! `context.rs`, `exec.rs`, `runtime.rs` — bu fayllar hech qachon Bevy-ga bevosita
//! bog'liq emas. Faqat `systems.rs` va `bevy_bridge.rs` Bevy tiplarini import qiladi.
//!
//! # Bevy yangilanganda
//! - `ButtonInput` API o'zgarse  → faqat `snapshot_input()` funksiyasi o'zgaradi
//! - `Transform` tuzilmasi o'zgarse → faqat `read_transform()` / `apply_transform_op()` o'zgaradi
//! - `Time` API o'zgarse          → faqat `read_delta_time()` o'zgaradi
//! - `KeyCode` variant nomlari o'zgarse → faqat `key_by_name()` o'zgaradi

use std::collections::HashSet;

use bevy::input::ButtonInput;
use bevy::prelude::*;

use super::resources::{PieEntityHolder, PieTransformOp};

// ── Input snapshot ─────────────────────────────────────────────────────────────

/// Bevy-dan mustaqil klaviatura holati.
/// `systems.rs` tomonidan har frame yaratiladi, `context.rs` ga o'tkaziladi.
#[derive(Default, Clone, Debug)]
pub struct InputSnapshot {
    /// Hozirda bosilgan tugmalar (Bevy KeyCode nom stringleri)
    pub pressed: HashSet<String>,
    /// Shu frameda birinchi marta bosilgan tugmalar
    pub just_pressed: HashSet<String>,
}

impl InputSnapshot {
    pub fn is_pressed(&self, name: &str) -> bool {
        self.pressed.contains(name)
    }

    pub fn is_just_pressed(&self, name: &str) -> bool {
        self.just_pressed.contains(name)
    }
}

/// `ButtonInput<KeyCode>` dan `InputSnapshot` yasaydi.
///
/// **Bevy yangilanishida o'zgaradigan yagona joy.**
/// Agar `ButtonInput<KeyCode>` → `ActionState` yoki boshqaga o'tsa,
/// faqat shu funksiya va `key_by_name` o'zgaradi.
pub fn snapshot_input(input: &ButtonInput<KeyCode>) -> InputSnapshot {
    let pressed = input
        .get_pressed()
        .map(keycode_to_name)
        .collect();
    let just_pressed = input
        .get_just_pressed()
        .map(keycode_to_name)
        .collect();
    InputSnapshot { pressed, just_pressed }
}

/// `KeyCode` → string nom (debug representatsiyasi ishlatiladi).
///
/// Bevy `KeyCode` variant nomlari o'zgarse — faqat shu funksiya va
/// `key_by_name` ni sinxronlash kerak.
fn keycode_to_name(key: &KeyCode) -> String {
    format!("{key:?}")
}

/// String nom → `KeyCode`. PIE kontekstida vizual tugunlarning `key_name` fieldidan
/// foydalaniladi.
///
/// **Bevy yangilanishida o'zgaradigan joy** (agar `KeyCode` qayta nomlanSa).
#[allow(dead_code)]
pub fn key_by_name(name: &str) -> KeyCode {
    match name {
        "Space"       => KeyCode::Space,
        "Enter"       => KeyCode::Enter,
        "Escape"      => KeyCode::Escape,
        "Tab"         => KeyCode::Tab,
        "Backspace"   => KeyCode::Backspace,
        "Delete"      => KeyCode::Delete,
        "Insert"      => KeyCode::Insert,
        "Home"        => KeyCode::Home,
        "End"         => KeyCode::End,
        "PageUp"      => KeyCode::PageUp,
        "PageDown"    => KeyCode::PageDown,
        "ArrowUp"     => KeyCode::ArrowUp,
        "ArrowDown"   => KeyCode::ArrowDown,
        "ArrowLeft"   => KeyCode::ArrowLeft,
        "ArrowRight"  => KeyCode::ArrowRight,
        "ShiftLeft"   => KeyCode::ShiftLeft,
        "ShiftRight"  => KeyCode::ShiftRight,
        "ControlLeft" => KeyCode::ControlLeft,
        "ControlRight"=> KeyCode::ControlRight,
        "AltLeft"     => KeyCode::AltLeft,
        "AltRight"    => KeyCode::AltRight,
        "KeyA"        => KeyCode::KeyA,
        "KeyB"        => KeyCode::KeyB,
        "KeyC"        => KeyCode::KeyC,
        "KeyD"        => KeyCode::KeyD,
        "KeyE"        => KeyCode::KeyE,
        "KeyF"        => KeyCode::KeyF,
        "KeyG"        => KeyCode::KeyG,
        "KeyH"        => KeyCode::KeyH,
        "KeyI"        => KeyCode::KeyI,
        "KeyJ"        => KeyCode::KeyJ,
        "KeyK"        => KeyCode::KeyK,
        "KeyL"        => KeyCode::KeyL,
        "KeyM"        => KeyCode::KeyM,
        "KeyN"        => KeyCode::KeyN,
        "KeyO"        => KeyCode::KeyO,
        "KeyP"        => KeyCode::KeyP,
        "KeyQ"        => KeyCode::KeyQ,
        "KeyR"        => KeyCode::KeyR,
        "KeyS"        => KeyCode::KeyS,
        "KeyT"        => KeyCode::KeyT,
        "KeyU"        => KeyCode::KeyU,
        "KeyV"        => KeyCode::KeyV,
        "KeyW"        => KeyCode::KeyW,
        "KeyX"        => KeyCode::KeyX,
        "KeyY"        => KeyCode::KeyY,
        "KeyZ"        => KeyCode::KeyZ,
        "Digit0"      => KeyCode::Digit0,
        "Digit1"      => KeyCode::Digit1,
        "Digit2"      => KeyCode::Digit2,
        "Digit3"      => KeyCode::Digit3,
        "Digit4"      => KeyCode::Digit4,
        "Digit5"      => KeyCode::Digit5,
        "Digit6"      => KeyCode::Digit6,
        "Digit7"      => KeyCode::Digit7,
        "Digit8"      => KeyCode::Digit8,
        "Digit9"      => KeyCode::Digit9,
        "F1"          => KeyCode::F1,
        "F2"          => KeyCode::F2,
        "F3"          => KeyCode::F3,
        "F4"          => KeyCode::F4,
        "F5"          => KeyCode::F5,
        "F6"          => KeyCode::F6,
        "F7"          => KeyCode::F7,
        "F8"          => KeyCode::F8,
        "F9"          => KeyCode::F9,
        "F10"         => KeyCode::F10,
        "F11"         => KeyCode::F11,
        "F12"         => KeyCode::F12,
        _ => {
            bevy::log::warn!("PIE: noma'lum KeyCode nomi '{name}', Space ishlatiladi");
            KeyCode::Space
        }
    }
}

// ── Transform helpers ─────────────────────────────────────────────────────────

/// Bitta entity Transform ma'lumotlarini o'qiydi.
///
/// **Bevy yangilanishida o'zgaradigan joy.**
/// `Transform::translation` → boshqa nom bo'lsa, faqat shu funksiya o'zgaradi.
pub fn read_transform_data(tf: &Transform) -> ([f32; 3], [f32; 3], [f32; 3]) {
    let t = tf.translation;
    let s = tf.scale;
    let (ex, ey, ez) = tf.rotation.to_euler(EulerRot::XYZ);
    ([t.x, t.y, t.z], [s.x, s.y, s.z], [ex, ey, ez])
}

/// `PieTransformOp` ni `Transform` ga qo'llaydi.
///
/// **Bevy yangilanishida o'zgaradigan joy.**
pub fn apply_transform_op(tf: &mut Transform, op: &PieTransformOp) {
    match op {
        PieTransformOp::SetTranslation { pos, .. } =>
            tf.translation = Vec3::from(*pos),
        PieTransformOp::Translate { delta, .. } =>
            tf.translation += Vec3::from(*delta),
        PieTransformOp::SetScale { scale, .. } =>
            tf.scale = Vec3::from(*scale),
        PieTransformOp::SetRotationEuler { euler, .. } =>
            tf.rotation = Quat::from_euler(EulerRot::XYZ, euler[0], euler[1], euler[2]),
    }
}

/// Barcha PIE entitylardan transform snapshot oladi.
///
/// **Bevy yangilanishida o'zgaradigan joy.**
pub fn build_transform_snapshots(
    transforms: &Query<&mut Transform, With<PieEntityHolder>>,
    entity_q: &Query<(Entity, &PieEntityHolder)>,
) -> (
    std::collections::HashMap<u32, [f32; 3]>,
    std::collections::HashMap<u32, [f32; 3]>,
    std::collections::HashMap<u32, [f32; 3]>,
) {
    let mut trans_map = std::collections::HashMap::new();
    let mut scale_map = std::collections::HashMap::new();
    let mut rot_map   = std::collections::HashMap::new();

    for (entity, holder) in entity_q.iter() {
        if let Ok(tf) = transforms.get(entity) {
            let (t, s, r) = read_transform_data(tf);
            trans_map.insert(holder.slot, t);
            scale_map.insert(holder.slot, s);
            rot_map.insert(holder.slot, r);
        }
    }
    (trans_map, scale_map, rot_map)
}

/// Transform ops ni ECS ga qo'llaydi.
///
/// **Bevy yangilanishida o'zgaradigan joy.**
pub fn apply_transform_ops_to_ecs(
    ops: Vec<PieTransformOp>,
    entity_table: &super::resources::PieEntityTable,
    transforms: &mut Query<&mut Transform, With<PieEntityHolder>>,
    entity_q: &Query<(Entity, &PieEntityHolder)>,
) {
    for op in &ops {
        let slot = match op {
            PieTransformOp::SetTranslation { slot, .. }    => *slot,
            PieTransformOp::Translate      { slot, .. }    => *slot,
            PieTransformOp::SetScale       { slot, .. }    => *slot,
            PieTransformOp::SetRotationEuler { slot, .. }  => *slot,
        };
        if let Some(&entity) = entity_table.entities.get(&slot) {
            for (q_entity, _) in entity_q.iter() {
                if q_entity == entity {
                    if let Ok(mut tf) = transforms.get_mut(q_entity) {
                        apply_transform_op(&mut tf, op);
                    }
                    break;
                }
            }
        }
    }
}

// ── Time helper ───────────────────────────────────────────────────────────────

/// Delta time soniyalarda.
///
/// **Bevy yangilanishida o'zgaradigan joy.**
/// `time.delta_secs()` → boshqa nom bo'lsa, faqat shu funksiya o'zgaradi.
pub fn read_delta_time(time: &Time) -> f32 {
    time.delta_secs()
}
