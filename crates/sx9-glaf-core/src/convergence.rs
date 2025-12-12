//! Graph Convergence Calculations (RFC-9021)
//!
//! Implements H1 (Operational) and H2 (Semantic) convergence scoring

use crate::glaf_core::GLAFCore;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Convergence event when both H1 and H2 exceed thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceEvent {
    pub h1_score: f64,
    pub h2_score: f64,
    pub timestamp: DateTime<Utc>,
    pub recommended_action: String,
}

/// Convergence monitor for real-time detection
pub struct ConvergenceMonitor {
    h1_threshold: f64,
    h2_threshold: f64,
    alert_callback: Box<dyn Fn(ConvergenceEvent) + Send + Sync>,
}

impl ConvergenceMonitor {
    pub fn new(
        h1_threshold: f64,
        h2_threshold: f64,
        alert_callback: Box<dyn Fn(ConvergenceEvent) + Send + Sync>,
    ) -> Self {
        Self {
            h1_threshold,
            h2_threshold,
            alert_callback,
        }
    }

    pub async fn monitor(&self, glaf_core: &crate::glaf_core::GLAFCore) {
        loop {
            let h1 = calculate_operational_convergence(glaf_core).await;
            let h2 = calculate_semantic_convergence(glaf_core).await;

            if h1 >= self.h1_threshold && h2 >= self.h2_threshold {
                (self.alert_callback)(ConvergenceEvent {
                    h1_score: h1,
                    h2_score: h2,
                    timestamp: Utc::now(),
                    recommended_action: recommend_action(h1, h2),
                });
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}

/// Calculate H1 (Operational) convergence score
///
/// RFC-9021: H1 measures operational scatter reduction
/// Uses TETH entropy: High entropy = scattered, Low entropy = converging
pub async fn calculate_operational_convergence(glaf_core: &crate::glaf_core::GLAFCore) -> f64 {
    // Get all nodes from graph
    let nodes = glaf_core.get_all_nodes().await;

    if nodes.is_empty() {
        return 0.0;
    }

    // Calculate TETH entropy for each node
    let mut entropy_sum = 0.0;
    for node in &nodes {
        let entropy = crate::teth::calculate_entropy(node).await;
        entropy_sum += entropy;
    }

    let avg_entropy = entropy_sum / nodes.len() as f64;

    // Convert entropy to convergence score (inverse relationship)
    // High entropy (5.0) = 0% convergence, Low entropy (0.0) = 100% convergence
    (1.0 - (avg_entropy / 5.0)).max(0.0).min(1.0)
}

/// Calculate H2 (Semantic) convergence score
///
/// RFC-9021: H2 measures pattern matching against corpus
/// Uses HMM phase detection: High transition probability = pattern match
pub async fn calculate_semantic_convergence(glaf_core: &crate::glaf_core::GLAFCore) -> f64 {
    // Get activity sequence from graph
    let activities = glaf_core.get_activity_sequence().await;

    if activities.is_empty() {
        return 0.0;
    }

    // Detect phase using HMM
    let phase_result = crate::hmm::detect_phase(&activities).await;

    // Use transition probability as H2 score
    phase_result.transition_probability
}

/// Recommend action based on convergence scores
fn recommend_action(h1: f64, h2: f64) -> String {
    let combined = (h1 + h2) / 2.0;

    if combined > 0.90 {
        "ACT_NOW".to_string()
    } else if combined > 0.75 {
        "PROCEED".to_string()
    } else if combined > 0.50 {
        "COLLECT_MORE".to_string()
    } else {
        "HUNT".to_string()
    }
}
