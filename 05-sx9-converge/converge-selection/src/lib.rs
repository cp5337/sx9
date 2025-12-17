//! CONVERGE Selection Core
//!
//! RFC Compliance:
//! - RFC-93X3 Matroid-Constrained Selection Framework
//!
//! Purpose:
//! Deterministic greedy selection of action sets under matroid constraints.
//!
//! Invariants:
//! - Explicit ordering
//! - Deterministic rank computation
//! - No probabilistic shortcuts

pub mod partition;
pub mod laminar;
pub mod greedy;
