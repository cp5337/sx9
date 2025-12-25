//! CONVERGE Sensor Crate
//!
//! RFC Compliance:
//! - RFC-9101 Smart Crate System
//! - RFC-9021 Graph Convergence Theory
//! - RFC-9304 GLAF Graph Engine
//! - RFC-93X1 CONVERGE Core
//! - RFC-93X2 Geometry Boundary
//! - RFC-93X3 Matroid Selection
//! - RFC-93X4 Integration
//!
//! Purpose:
//! Deterministic, storage-free detection of distributed action-set convergence.
//! Emits ConvergeSignal into GLAF.
//!
//! Invariants:
//! - Deterministic execution
//! - No raw event persistence
//! - Set-based reasoning only

pub mod model;
pub mod window;
pub mod detector;
