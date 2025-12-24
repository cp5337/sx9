//! QA Gates module
//!
//! Implements the 4-stage QA pipeline:
//! 1. Static Gate - Code structure and complexity
//! 2. Architecture Gate - ECS compliance and patterns
//! 3. Pattern Gate - Canonical pattern matching

pub mod static_gate;
pub mod arch_gate;
pub mod pattern_gate;

pub use static_gate::StaticGate;
pub use arch_gate::ArchGate;
pub use pattern_gate::PatternGate;
