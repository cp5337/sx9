//! Thalamic Filter (RFC-9021)
//!
//! The "Gateway to Consciousness".
//! Filters incoming signals (USIMs, Sensory Data) based on Salience, Urgency, and Relevance.
//! Prevents cognitive overload by only passing high-value signals to the active OODA loop.

use crate::cognitive::{CognitiveMode, CognitiveState};
use crate::neural_mux::Priority;
use chrono::{DateTime, Utc};

/// Signal entering the cognitive system
#[derive(Debug, Clone)]
pub struct SensorySignal {
    pub id: String,
    pub source: String,
    pub signal_type: String,
    pub content_hash: String,
    pub priority: Priority,
    pub timestamp: DateTime<Utc>,
}

/// Filtered signal ready for processing
#[derive(Debug, Clone)]
pub struct ThalamicInput {
    pub signal: SensorySignal,
    pub salience_score: f32,
    pub urgency_score: f32,
    pub routing_target: String, // "Cortex", "Amygdala" (Reflex), etc.
}

pub struct ThalamicFilter {
    pub salience_threshold: f32,
    pub urgency_threshold: f32,
}

impl Default for ThalamicFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl ThalamicFilter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            salience_threshold: 0.4, // Default baseline
            urgency_threshold: 0.3,
        }
    }

    /// Adjust thresholds based on current cognitive load (Homeostasis)
    pub fn adjust_thresholds(&mut self, state: &CognitiveState) {
        // If overloaded, raise shields (thresholds)
        if state.cognitive_load > 0.8 {
            self.salience_threshold = 0.7;
            self.urgency_threshold = 0.6;
        } else if state.cognitive_load < 0.2 {
            self.salience_threshold = 0.2; // Lower thresholds to seek stimulation
            self.urgency_threshold = 0.1;
        } else {
            self.salience_threshold = 0.4;
            self.urgency_threshold = 0.3;
        }

        // Mode-specific adjustments
        match state.mode {
            CognitiveMode::Reactive => {
                self.urgency_threshold = 0.1; // React to everything urgent
            }
            CognitiveMode::Analytical => {
                self.salience_threshold = 0.6; // Only analyze important things
            }
            _ => {}
        }
    }

    /// Process a signal and determine if it should pass
    #[must_use]
    pub fn process_signal(&self, signal: SensorySignal) -> Option<ThalamicInput> {
        let urgency = self.calculate_urgency(&signal);
        let salience = self.calculate_salience(&signal);

        if urgency >= self.urgency_threshold || salience >= self.salience_threshold {
            let target = if urgency > 0.8 { "Reflex" } else { "Cortex" };

            Some(ThalamicInput {
                signal,
                salience_score: salience,
                urgency_score: urgency,
                routing_target: target.to_string(),
            })
        } else {
            // Signal inhibited
            None
        }
    }

    fn calculate_urgency(&self, signal: &SensorySignal) -> f32 {
        // Decay logic could go here
        match signal.priority {
            Priority::Critical => 1.0,
            Priority::High => 0.75,
            Priority::Medium => 0.5,
            Priority::Low => 0.25,
        }
    }

    fn calculate_salience(&self, signal: &SensorySignal) -> f32 {
        // Simplified salience: "Is this new or strictly related to current goals?"
        // In real impl, this checks Vector Memory for novelty
        0.5 // Placeholder
    }
}
