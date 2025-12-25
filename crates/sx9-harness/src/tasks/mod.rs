//! FORGE Mission Tasks
//!
//! Agnostic core tasks following V-N-N pattern:
//! Verb - Object - Context (subject Forge implicit)
//!
//! No dual tasks. Single action per task definition.

mod core;
mod engineering;
mod graph;

pub use core::*;
pub use engineering::*;
pub use graph::*;
