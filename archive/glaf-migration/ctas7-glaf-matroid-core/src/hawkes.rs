//! Hawkes Process for event intensity modeling
//!
//! RFC-9021: Self-exciting point process for threat event clustering

use serde::{Deserialize, Serialize};

/// Hawkes process parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HawkesParams {
    /// Background intensity (baseline rate)
    pub mu: f64,
    /// Jump size (impact of each event)
    pub alpha: f64,
    /// Decay rate
    pub beta: f64,
}

impl Default for HawkesParams {
    fn default() -> Self {
        Self {
            mu: 0.1,    // Low baseline rate
            alpha: 0.5, // Moderate excitation
            beta: 1.0,  // Unit decay rate
        }
    }
}

/// Hawkes process for modeling self-exciting events
///
/// Used for threat event clustering and intensity prediction
pub struct HawkesProcess {
    params: HawkesParams,
    /// Event timestamps (relative to start)
    events: Vec<f64>,
    /// Current time
    current_time: f64,
}

impl HawkesProcess {
    /// Create new Hawkes process
    pub fn new(params: HawkesParams) -> Self {
        Self {
            params,
            events: Vec::new(),
            current_time: 0.0,
        }
    }

    /// Record an event at current time
    pub fn record_event(&mut self) {
        self.events.push(self.current_time);
    }

    /// Advance time
    pub fn advance_time(&mut self, delta: f64) {
        self.current_time += delta;
    }

    /// Calculate intensity at current time
    ///
    /// λ(t) = μ + Σ α * exp(-β * (t - ti))
    pub fn intensity(&self) -> f64 {
        let mut intensity = self.params.mu;

        for &event_time in &self.events {
            let time_since = self.current_time - event_time;
            if time_since > 0.0 {
                intensity += self.params.alpha * (-self.params.beta * time_since).exp();
            }
        }

        intensity
    }

    /// Calculate expected number of events in next interval
    pub fn expected_events(&self, interval: f64) -> f64 {
        let current = self.intensity();
        // Approximate: average of start and end intensity * interval
        // More accurate would require integration
        current * interval
    }

    /// Check if current intensity exceeds threshold
    pub fn is_elevated(&self, threshold: f64) -> bool {
        self.intensity() > threshold
    }

    /// Get branching ratio (α/β) - stability indicator
    ///
    /// < 1.0: Stable (subcritical)
    /// = 1.0: Critical
    /// > 1.0: Unstable (supercritical)
    pub fn branching_ratio(&self) -> f64 {
        self.params.alpha / self.params.beta
    }

    /// Check if process is stable
    pub fn is_stable(&self) -> bool {
        self.branching_ratio() < 1.0
    }

    /// Get event count
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Clear event history
    pub fn reset(&mut self) {
        self.events.clear();
        self.current_time = 0.0;
    }

    /// Get events for analysis
    pub fn events(&self) -> &[f64] {
        &self.events
    }
}

impl Default for HawkesProcess {
    fn default() -> Self {
        Self::new(HawkesParams::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baseline_intensity() {
        let process = HawkesProcess::default();
        // With no events, intensity should equal mu
        assert!((process.intensity() - 0.1).abs() < 0.001);
    }

    #[test]
    fn test_intensity_after_event() {
        let mut process = HawkesProcess::default();
        process.record_event();
        process.advance_time(0.001); // Small time step

        // Intensity should be elevated right after event
        assert!(process.intensity() > process.params.mu);
    }

    #[test]
    fn test_intensity_decay() {
        let mut process = HawkesProcess::default();
        process.record_event();

        let intensity_near = {
            process.advance_time(0.1);
            process.intensity()
        };

        process.current_time = 0.0;
        let intensity_far = {
            process.advance_time(10.0);
            process.intensity()
        };

        // Intensity should decay over time
        assert!(intensity_near > intensity_far);
    }

    #[test]
    fn test_stability() {
        let stable_params = HawkesParams { mu: 0.1, alpha: 0.5, beta: 1.0 };
        let unstable_params = HawkesParams { mu: 0.1, alpha: 1.5, beta: 1.0 };

        let stable = HawkesProcess::new(stable_params);
        let unstable = HawkesProcess::new(unstable_params);

        assert!(stable.is_stable());
        assert!(!unstable.is_stable());
    }
}
