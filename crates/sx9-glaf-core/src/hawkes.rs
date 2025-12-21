//! Hawkes Process Intensity Calculation
//!
//! RFC-9021: Temporal event clustering for self-exciting patterns
//! λ(t) = μ + Σ α × e^(-β(t-tᵢ))

use chrono::{DateTime, Utc};

/// Hawkes process intensity calculator
pub struct HawkesIntensity {
    background_rate: f64,     // μ
    excitation_strength: f64, // α
    decay_rate: f64,          // β
}

impl HawkesIntensity {
    pub fn new(background_rate: f64, excitation_strength: f64, decay_rate: f64) -> Self {
        Self {
            background_rate,
            excitation_strength,
            decay_rate,
        }
    }

    /// Calculate intensity at time t
    ///
    /// λ(t) = μ + Σ α × e^(-β(t-tᵢ))
    pub fn calculate_intensity(
        &self,
        current_time: DateTime<Utc>,
        event_times: &[DateTime<Utc>],
    ) -> f64 {
        let t = current_time.timestamp_millis() as f64 / 1000.0;

        let mut excitation_sum = 0.0;
        for event_time in event_times {
            let t_i = event_time.timestamp_millis() as f64 / 1000.0;
            let delta_t = t - t_i;
            if delta_t > 0.0 {
                excitation_sum += self.excitation_strength * (-self.decay_rate * delta_t).exp();
            }
        }

        self.background_rate + excitation_sum
    }
}

/// Calculate Hawkes intensity for event sequence
pub async fn calculate_intensity(
    current_time: DateTime<Utc>,
    event_times: &[DateTime<Utc>],
) -> f64 {
    // Default parameters: μ=0.1, α=0.5, β=1.0
    let calculator = HawkesIntensity::new(0.1, 0.5, 1.0);
    calculator.calculate_intensity(current_time, event_times)
}
