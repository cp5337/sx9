//! sx9-glaf-core - GLAF Neural Operations
//!
//! RFC-9021: Graph Convergence Theory implementation
//! Provides neural operations for graph analysis, convergence calculations,
//! and APOC++ procedure integration.

pub mod convergence;
pub mod glaf_core;
pub mod hawkes;
pub mod hmm;
pub mod matroid;
pub mod teth;
pub mod types;

pub use types::*;

pub use convergence::{
    calculate_operational_convergence, calculate_semantic_convergence, ConvergenceEvent,
    ConvergenceMonitor,
};
pub use glaf_core::GLAFCore;
pub use hawkes::{calculate_intensity, HawkesIntensity};
pub use hmm::{detect_phase, HmmPhaseDetector, Phase};
pub use matroid::{calculate_rank, calculate_rank_delta, MatroidRank};
pub use teth::{calculate_entropy, TethAnalyzer};

/// GLAF neural operations configuration
#[derive(Debug, Clone, Default)]
pub struct GlafConfig {
    pub h1_threshold: f64,
    pub h2_threshold: f64,
    pub convergence_window_ms: u64,
}

impl GlafConfig {
    pub fn default() -> Self {
        Self {
            h1_threshold: 0.75,
            h2_threshold: 0.75,
            convergence_window_ms: 100,
        }
    }
}
