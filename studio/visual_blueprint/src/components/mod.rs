//! ECS komponentlari — tizimsiz, faqat struct maydonlar.

#[cfg(feature = "bevy")]
mod gold;

#[cfg(feature = "bevy")]
pub use gold::Gold;
