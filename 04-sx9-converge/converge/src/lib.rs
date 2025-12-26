//! CONVERGE Sensor Crate
//!
//! RFC Compliance:
//! - RFC-9101 Smart Crate System
//! - RFC-9021 Graph Convergence Theory
//! - RFC-9024 Dual Convergence Formula
//! - RFC-9109 HFT Order Book (Plasma Defender)
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
//! Architecture:
//! - H1 (Semantic/TOML): Human-readable, planning-oriented (apecs async path)
//! - H2 (Operational/JSON): Machine-optimized, runtime-oriented (Legion hot path)
//!
//! Invariants:
//! - Deterministic execution
//! - No raw event persistence
//! - Set-based reasoning only

pub mod model;
pub mod window;
pub mod detector;

// Re-exports for convenience
pub use model::{
    ActionEvent, BookSide, ConvergeEntity, ConvergeError, ConvergenceMethod,
    ConvergenceResult, ConvergeSignal, H1Semantic, H2Operational, HD4Phase,
    TacticalProfile, UnicodeClass, calculate_convergence,
};
pub use window::{EventWindow, WindowSet, MAX_EVENTS_PER_WINDOW, MAX_WINDOW_MS};
pub use detector::{ConvergeDetector, DetectorConfig, DetectorStats, CONVERGENCE_THRESHOLD};
