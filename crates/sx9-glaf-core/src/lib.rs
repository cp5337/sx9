//! sx9-glaf-core - GLAF Neural Operations
//!
//! RFC-9021: Graph Convergence Theory implementation
//! Provides neural operations for graph analysis, convergence calculations,
//! and APOC++ procedure integration.

pub mod convergence;
pub mod teth;
pub mod hmm;
pub mod matroid;
pub mod hawkes;
pub mod glaf_core;

pub use convergence::{ConvergenceMonitor, ConvergenceEvent, calculate_operational_convergence, calculate_semantic_convergence};
pub use teth::{TethAnalyzer, calculate_entropy};
pub use hmm::{HmmPhaseDetector, detect_phase, Phase};
pub use matroid::{MatroidRank, calculate_rank, calculate_rank_delta};
pub use hawkes::{HawkesIntensity, calculate_intensity};
pub use glaf_core::{GLAFCore, GlafNode, GlafRelationship};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

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
