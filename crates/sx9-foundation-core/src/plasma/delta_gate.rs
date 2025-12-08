//! PLASMA Delta Gate Module
//!
//! Implements signal suppression/amplification/equalization based on delta measurements.
//! Configurable thresholds for noise gating in toolchain escalation.

use super::delta_operator::DeltaMeasurement;

/// Gate operation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateMode {
    /// Suppress signals above threshold
    Suppress,
    /// Amplify signals above threshold
    Amplify,
    /// Equalize signals (normalize to threshold)
    Equalize,
}

/// Delta gate configuration
#[derive(Debug, Clone)]
pub struct DeltaGateConfig {
    /// Noise threshold (0.0-1.0)
    pub threshold: f32,
    /// Gate mode
    pub mode: GateMode,
    /// Amplification factor (for Amplify mode)
    pub amplification_factor: f32,
    /// Suppression factor (for Suppress mode)
    pub suppression_factor: f32,
}

impl Default for DeltaGateConfig {
    fn default() -> Self {
        Self {
            threshold: 0.5,
            mode: GateMode::Suppress,
            amplification_factor: 2.0,
            suppression_factor: 0.1,
        }
    }
}

/// Gated payload result
#[derive(Debug, Clone)]
pub struct GatedPayload<T> {
    /// Original payload
    pub payload: T,
    /// Gated weight (0.0-1.0)
    pub gated_weight: f32,
    /// Whether signal passed the gate
    pub passed: bool,
    /// Delta measurement used for gating
    pub delta_measurement: DeltaMeasurement,
}

/// PLASMA Delta Gate
pub struct DeltaGate {
    config: DeltaGateConfig,
    enabled: bool,
}

impl DeltaGate {
    /// Create new delta gate
    pub fn new(config: DeltaGateConfig, enabled: bool) -> Self {
        Self { config, enabled }
    }

    /// Create gate with default config
    pub fn with_default_config(enabled: bool) -> Self {
        Self::new(DeltaGateConfig::default(), enabled)
    }

    /// Gate a payload based on delta measurement
    pub fn gate<T: Clone>(
        &self,
        payload: T,
        delta: &DeltaMeasurement,
    ) -> GatedPayload<T> {
        if !self.enabled {
            // Pass through unchanged if disabled
            return GatedPayload {
                payload,
                gated_weight: 1.0,
                passed: true,
                delta_measurement: delta.clone(),
            };
        }

        let gated_weight = self.compute_gated_weight(delta);
        let passed = gated_weight > 0.0;

        GatedPayload {
            payload,
            gated_weight,
            passed,
            delta_measurement: delta.clone(),
        }
    }

    /// Compute gated weight based on delta measurement and config
    fn compute_gated_weight(&self, delta: &DeltaMeasurement) -> f32 {
        let noise = delta.noise_score;

        match self.config.mode {
            GateMode::Suppress => {
                if noise > self.config.threshold {
                    // Suppress: reduce weight based on how much over threshold
                    let excess = noise - self.config.threshold;
                    let suppression = excess * self.config.suppression_factor;
                    (1.0 - suppression).max(0.0)
                } else {
                    // Below threshold: pass through
                    1.0
                }
            }
            GateMode::Amplify => {
                if noise > self.config.threshold {
                    // Amplify: increase weight based on noise
                    let excess = noise - self.config.threshold;
                    let amplification = 1.0 + (excess * self.config.amplification_factor);
                    amplification.min(2.0) // Cap at 2x
                } else {
                    // Below threshold: pass through
                    1.0
                }
            }
            GateMode::Equalize => {
                // Equalize: normalize to threshold
                if noise > 0.0 {
                    self.config.threshold / noise.max(0.01)
                } else {
                    1.0
                }
            }
        }
    }

    /// Update gate configuration
    pub fn update_config(&mut self, config: DeltaGateConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn config(&self) -> &DeltaGateConfig {
        &self.config
    }

    /// Enable/disable gate
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if gate is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suppress_mode() {
        let config = DeltaGateConfig {
            threshold: 0.5,
            mode: GateMode::Suppress,
            amplification_factor: 2.0,
            suppression_factor: 0.5,
        };
        let gate = DeltaGate::new(config, true);

        // High noise: should be suppressed
        let high_noise = DeltaMeasurement::new(90.0, 0.8, 0.9);
        let payload = "test".to_string();
        let gated = gate.gate(payload.clone(), &high_noise);
        
        assert!(gated.gated_weight < 1.0);
        assert!(gated.passed); // Still passes but with reduced weight

        // Low noise: should pass through
        let low_noise = DeltaMeasurement::new(5.0, 0.1, 0.2);
        let gated = gate.gate(payload, &low_noise);
        
        assert_eq!(gated.gated_weight, 1.0);
        assert!(gated.passed);
    }

    #[test]
    fn test_amplify_mode() {
        let config = DeltaGateConfig {
            threshold: 0.5,
            mode: GateMode::Amplify,
            amplification_factor: 2.0,
            suppression_factor: 0.1,
        };
        let gate = DeltaGate::new(config, true);

        // High noise: should be amplified
        let high_noise = DeltaMeasurement::new(90.0, 0.8, 0.9);
        let payload = "test".to_string();
        let gated = gate.gate(payload, &high_noise);
        
        assert!(gated.gated_weight > 1.0);
        assert!(gated.gated_weight <= 2.0); // Capped at 2x
    }

    #[test]
    fn test_equalize_mode() {
        let config = DeltaGateConfig {
            threshold: 0.5,
            mode: GateMode::Equalize,
            amplification_factor: 2.0,
            suppression_factor: 0.1,
        };
        let gate = DeltaGate::new(config, true);

        let noise = DeltaMeasurement::new(45.0, 0.6, 0.7);
        let payload = "test".to_string();
        let gated = gate.gate(payload, &noise);
        
        assert!(gated.gated_weight > 0.0 && gated.gated_weight <= 1.0);
    }

    #[test]
    fn test_disabled_gate() {
        let gate = DeltaGate::with_default_config(false);
        
        let noise = DeltaMeasurement::new(90.0, 0.8, 0.9);
        let payload = "test".to_string();
        let gated = gate.gate(payload, &noise);
        
        assert_eq!(gated.gated_weight, 1.0);
        assert!(gated.passed);
    }

    #[test]
    fn test_tunable_levels() {
        // Test suppression at different levels
        let levels = vec![0.1, 0.3, 0.7, 0.9];
        
        for level in levels {
            let config = DeltaGateConfig {
                threshold: level,
                mode: GateMode::Suppress,
                amplification_factor: 2.0,
                suppression_factor: 0.5,
            };
            let gate = DeltaGate::new(config, true);
            
            let noise = DeltaMeasurement::new(90.0, 0.8, 0.9);
            let payload = "test".to_string();
            let gated = gate.gate(payload, &noise);
            
            // Higher threshold = less suppression (more passes through)
            assert!(gated.gated_weight >= 0.0 && gated.gated_weight <= 1.0);
        }
    }
}


