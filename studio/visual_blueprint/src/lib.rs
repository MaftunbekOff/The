//! Pure Rust vizual skript **ma'lumot** qatlami (v3: shox + dataflow).

#![forbid(unsafe_code)]
#![allow(missing_docs, reason = "blueprint crate; docs expand with API")]

pub mod ast;
#[cfg(feature = "bevy")]
pub mod components;
pub mod export;
pub mod interpreter;
pub mod nodes;
pub mod pins;
pub mod validate;

pub use ast::{ExecLink, VisualScriptGraph};
pub use export::graph_to_dsl;
pub use interpreter::demo::{demo_rich_branch_graph, demo_treasury_graph};
pub use interpreter::flow::{validate_exec_dag, DataPort};
pub use nodes::{NodeKind, VisualNode};
pub use pins::{DataLink, PinSpec, PinType, TypedLiteral, VisualEntityId};
pub use validate::{validate_data_link, validate_graph, ValidationError, ValidationResult};

#[cfg(feature = "bevy")]
pub use components::Gold;
