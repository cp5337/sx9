//! Convergence Score Calculator
//!
//! RFC-9021: GLAF Matroid Convergence
//! H1 (Operational) and H2 (Semantic) score calculation

use serde::{Deserialize, Serialize};

/// Convergence scores (H1 operational, H2 semantic)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConvergenceScore {
    /// H1 - Operational convergence (0.0 - 1.0)
    pub h1: f64,
    /// H2 - Semantic convergence (0.0 - 1.0)
    pub h2: f64,
    /// Combined convergence
    pub combined: f64,
}

impl ConvergenceScore {
    /// Create new convergence score
    pub fn new(h1: f64, h2: f64) -> Self {
        Self {
            h1: h1.clamp(0.0, 1.0),
            h2: h2.clamp(0.0, 1.0),
            combined: ((h1 + h2) / 2.0).clamp(0.0, 1.0),
        }
    }

    /// Check if convergence exceeds threshold
    pub fn exceeds_threshold(&self, threshold: f64) -> bool {
        self.combined > threshold
    }
}

/// Convergence calculator with exponential moving average
pub struct ConvergenceCalculator {
    h1_ema: f64,
    h2_ema: f64,
    alpha: f64, // EMA smoothing factor
    sample_count: u64,
}

impl ConvergenceCalculator {
    /// Create new calculator with default alpha (0.1)
    pub fn new() -> Self {
        Self::with_alpha(0.1)
    }

    /// Create calculator with custom smoothing factor
    pub fn with_alpha(alpha: f64) -> Self {
        Self {
            h1_ema: 0.5,
            h2_ema: 0.5,
            alpha: alpha.clamp(0.01, 1.0),
            sample_count: 0,
        }
    }

    /// Update from raw signal values
    pub fn update_from_signals(&mut self, signals: &[f64]) {
        if signals.is_empty() {
            return;
        }

        // Calculate H1 (operational) from signal strength
        let h1_raw: f64 = signals.iter().sum::<f64>() / signals.len() as f64;

        // Calculate H2 (semantic) from signal variance
        let mean = h1_raw;
        let variance: f64 =
            signals.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / signals.len() as f64;
        let h2_raw = 1.0 - variance.sqrt().min(1.0); // Lower variance = higher semantic coherence

        self.update(h1_raw, h2_raw);
    }

    /// Update with direct H1/H2 values
    pub fn update(&mut self, h1: f64, h2: f64) {
        self.h1_ema = self.alpha * h1 + (1.0 - self.alpha) * self.h1_ema;
        self.h2_ema = self.alpha * h2 + (1.0 - self.alpha) * self.h2_ema;
        self.sample_count += 1;
    }

    /// Get current H1 score
    pub fn h1_score(&self) -> f64 {
        self.h1_ema
    }

    /// Get current H2 score
    pub fn h2_score(&self) -> f64 {
        self.h2_ema
    }

    /// Get combined convergence score
    pub fn score(&self) -> ConvergenceScore {
        ConvergenceScore::new(self.h1_ema, self.h2_ema)
    }

    /// Reset calculator state
    pub fn reset(&mut self) {
        self.h1_ema = 0.5;
        self.h2_ema = 0.5;
        self.sample_count = 0;
    }
}

impl Default for ConvergenceCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convergence_score() {
        let score = ConvergenceScore::new(0.8, 0.6);
        assert_eq!(score.combined, 0.7);
        assert!(score.exceeds_threshold(0.5));
        assert!(!score.exceeds_threshold(0.9));
    }

    #[test]
    fn test_convergence_clamp() {
        let score = ConvergenceScore::new(1.5, -0.5);
        assert_eq!(score.h1, 1.0);
        assert_eq!(score.h2, 0.0);
    }

    #[test]
    fn test_calculator_update() {
        let mut calc = ConvergenceCalculator::new();
        calc.update(0.9, 0.9);
        assert!(calc.h1_score() > 0.5);
        assert!(calc.h2_score() > 0.5);
    }

    #[test]
    fn test_calculator_from_signals() {
        let mut calc = ConvergenceCalculator::new();
        calc.update_from_signals(&[0.8, 0.85, 0.9]);
        assert!(calc.h1_score() > 0.5);
    }
}
