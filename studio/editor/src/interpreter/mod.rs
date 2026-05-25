//! Play-in-Editor (PIE) — blueprint grafi shu oynada bajariladi.
//!
//! # Bevy yangilanishi izolyatsiyasi
//! Faqat `bevy_bridge` va `systems` fayllar Bevy API ga bevosita bog'liq.
//! Qolgan modullar (`context`, `exec`, `runtime`) Bevy ga bog'liq emas.

pub mod bevy_bridge;
pub(crate) mod context;
pub(crate) mod error;
pub(crate) mod exec;
mod gold;
pub(crate) mod resources;
mod runtime;
mod subgraph;
pub mod systems;
pub(crate) mod value;

#[allow(unused_imports)]
pub(crate) use error::{PieError, PieErrorKind};
#[allow(unused_imports)]
pub use resources::{
    BlackboardResource, PieDelayQueue, PieEntityHolder, PieEntitySpawnRequest, PieEntityTable,
    PieGoldHolder, PiePendingGraph, PieRuntimeError, PieSession, PieStartRequested,
    PieTransformOp, ScriptActor,
};
pub use systems::pie_stop;
