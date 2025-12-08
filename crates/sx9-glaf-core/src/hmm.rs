//! Hidden Markov Model (HMM) Phase Detection
//!
//! RFC-9021: Detects adversary behavior phases
//! Hidden States: [Recon] → [Staging] → [Execution] → [Exfil]

use serde::{Serialize, Deserialize};
use anyhow::Result;

/// Adversary phase states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Recon,
    Staging,
    Execution,
    Exfil,
    Unknown,
}

/// HMM phase detection result
#[derive(Debug, Clone)]
pub struct PhaseResult {
    pub current_phase: Phase,
    pub transition_probability: f64,
    pub next_phase: Option<Phase>,
}

/// HMM phase detector
pub struct HmmPhaseDetector {
    transition_matrix: [[f64; 4]; 4], // 4 phases
}

impl HmmPhaseDetector {
    pub fn new() -> Self {
        // Default transition probabilities
        // [Recon, Staging, Execution, Exfil]
        let transition_matrix = [
            [0.6, 0.3, 0.05, 0.05], // From Recon
            [0.1, 0.5, 0.3, 0.1],   // From Staging
            [0.05, 0.1, 0.6, 0.25],  // From Execution
            [0.0, 0.0, 0.1, 0.9],    // From Exfil (terminal)
        ];
        
        Self { transition_matrix }
    }

    pub fn detect(&self, activities: &[String]) -> PhaseResult {
        if activities.is_empty() {
            return PhaseResult {
                current_phase: Phase::Unknown,
                transition_probability: 0.0,
                next_phase: None,
            };
        }

        // Simple heuristic: classify based on activity keywords
        let last_activity = activities.last().unwrap().to_lowercase();
        
        let (phase, prob) = if last_activity.contains("scan") || last_activity.contains("recon") {
            (Phase::Recon, 0.8)
        } else if last_activity.contains("download") || last_activity.contains("stage") {
            (Phase::Staging, 0.7)
        } else if last_activity.contains("execute") || last_activity.contains("lateral") {
            (Phase::Execution, 0.75)
        } else if last_activity.contains("exfil") || last_activity.contains("outbound") {
            (Phase::Exfil, 0.9)
        } else {
            (Phase::Unknown, 0.5)
        };

        // Calculate next phase probability
        let phase_idx = phase as usize;
        let next_phase_idx = self.transition_matrix[phase_idx]
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(0);

        let next_phase = match next_phase_idx {
            0 => Phase::Recon,
            1 => Phase::Staging,
            2 => Phase::Execution,
            3 => Phase::Exfil,
            _ => Phase::Unknown,
        };

        PhaseResult {
            current_phase: phase,
            transition_probability: prob,
            next_phase: Some(next_phase),
        }
    }
}

/// Detect phase from activity sequence
pub async fn detect_phase(activities: &[String]) -> PhaseResult {
    let detector = HmmPhaseDetector::new();
    detector.detect(activities)
}

