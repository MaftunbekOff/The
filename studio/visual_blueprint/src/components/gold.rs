//! Faqat ma’lumot — hisob-kitob tizimlarda (makro generatsiyasi).

#[cfg(feature = "bevy")]
use bevy::prelude::*;

/// Oltin balansi (`AddGold` tuguni `Gold::value` ni o‘zgartiradi).
#[cfg(feature = "bevy")]
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Gold {
    pub value: f32,
}
