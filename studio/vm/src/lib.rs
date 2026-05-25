//! Visual-script virtual machine: graph snapshot → compile → step runtime.
//!
//! Bevy-dan mustaqil; editor va o‘yin `GraphSnapshot` orqali ulanaadi.

#![forbid(unsafe_code)]
#![allow(missing_docs, reason = "VM crate; public API docs later")]

mod compile;
mod graph;
mod runtime;

pub use compile::{compile, CompileError};
pub use graph::{GraphSnapshot, NodeId, NodeKind};
pub use runtime::{VmEvent, VmRuntime};
