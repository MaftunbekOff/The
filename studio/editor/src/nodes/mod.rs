//! Node registry — barcha node kategoriyalari shu yerda birlashtiriladi.
//!
//! # Yangi node kategoriyasi qo'shish
//! 1. Yangi `[kategoriya].rs` fayl yarating
//! 2. `pub(crate) mod` va `build_registry` ga `register()` chaqiruvini qo'shing
//! 3. Bo'ldi. `registry.rs`, `exec.rs` tegilmaydi.

pub mod descriptor;
pub(crate) mod control;
pub(crate) mod events;
pub(crate) mod input;
pub(crate) mod math;
pub(crate) mod string;
pub(crate) mod game;
pub(crate) mod variables;
pub(crate) mod collections;
pub(crate) mod loops;
pub(crate) mod helpers;
pub(crate) mod vec2;
pub(crate) mod ecs;

pub(crate) use descriptor::NodeRegistry;

/// Global registry ni quradi — `NodeRegistry::global()` tomonidan chaqiriladi.
pub(crate) fn build_registry() -> NodeRegistry {
    let mut r = NodeRegistry::new();

    events::register(&mut r);
    control::register(&mut r);
    input::register(&mut r);
    math::register(&mut r);
    string::register(&mut r);
    game::gold::register(&mut r);
    game::entity::register(&mut r);
    game::transform::register(&mut r);
    variables::register(&mut r);
    collections::register(&mut r);
    loops::register(&mut r);
    helpers::register(&mut r);
    vec2::register(&mut r);
    ecs::register_ecs_nodes(&mut r);

    r
}
