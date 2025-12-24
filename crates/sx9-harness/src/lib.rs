//! SX9 Harness - Unified Forge Execution \u0026 QA System
//!
//! This crate provides:
//! - Forge execution engine (from TypeScript harness)
//! - QA gates (static, architecture, pattern)
//! - State management and validation
//! - NATS integration for distributed execution

pub mod types;
pub mod executor;
pub mod actions;
pub mod middleware;
pub mod validators;
pub mod reducer;
pub mod selectors;
pub mod gates;

// Re-exports
pub use types::*;
pub use executor::Executor;
pub use gates::{StaticGate, ArchGate, PatternGate};
