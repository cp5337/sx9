//! RFC-9302 Nonagon Analytic Node Implementation
//!
//! 9-vertex graph structure with 3 trivariates:
//! - Alpha (α): Semantic - context, meaning, intent
//! - Beta (β): Operational - phase, intensity, duration
//! - Gamma (γ): Temporal - historical, current, predictive
//!
//! TETH Entropy calculation validates workflow quality

use serde::{Serialize, Deserialize};
use std::f64::consts::LN_2;

/// RFC-9302 precision constant (6 decimal places)
pub const DELTA_PRECISION: f64 = 1e-6;

/// Minimum TETH entropy threshold
pub const MIN_TETH_ENTROPY: f64 = 2.5;

/// Validated TETH entropy from RFC-9302
pub const VALIDATED_TETH_ENTROPY: f64 = 3.9232;

/// Trivariate coordinate in 3D space
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Trivariate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Trivariate {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Calculate magnitude of trivariate vector
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize to unit vector
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag < DELTA_PRECISION {
            return Self::default();
        }
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}

/// Alpha trivariate - Semantic dimensions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AlphaTrivariate {
    /// Context relevance [0.0, 1.0]
    pub context: f64,
    /// Semantic meaning [0.0, 1.0]
    pub meaning: f64,
    /// Intent classification [0.0, 1.0]
    pub intent: f64,
}

impl Default for AlphaTrivariate {
    fn default() -> Self {
        Self {
            context: 0.5,
            meaning: 0.6,
            intent: 0.5,
        }
    }
}

impl From<AlphaTrivariate> for Trivariate {
    fn from(alpha: AlphaTrivariate) -> Self {
        Trivariate::new(alpha.context, alpha.meaning, alpha.intent)
    }
}

/// Beta trivariate - Operational dimensions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BetaTrivariate {
    /// HD4 phase mapping [0.0, 1.0]
    pub phase: f64,
    /// Operation intensity [0.0, 1.0]
    pub intensity: f64,
    /// Duration factor [0.0, 1.0]
    pub duration: f64,
}

impl Default for BetaTrivariate {
    fn default() -> Self {
        Self {
            phase: 0.2,      // HUNT phase
            intensity: 0.01,
            duration: 0.5,
        }
    }
}

impl From<BetaTrivariate> for Trivariate {
    fn from(beta: BetaTrivariate) -> Self {
        Trivariate::new(beta.phase, beta.intensity, beta.duration)
    }
}

/// Gamma trivariate - Temporal dimensions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GammaTrivariate {
    /// Historical context [0.0, 1.0]
    pub historical: f64,
    /// Current state [0.0, 1.0]
    pub current: f64,
    /// Predictive projection [0.0, 1.0]
    pub predictive: f64,
}

impl Default for GammaTrivariate {
    fn default() -> Self {
        Self {
            historical: 0.5,
            current: 0.505,
            predictive: 0.6,
        }
    }
}

impl From<GammaTrivariate> for Trivariate {
    fn from(gamma: GammaTrivariate) -> Self {
        Trivariate::new(gamma.historical, gamma.current, gamma.predictive)
    }
}

/// RFC-9302 Nonagon Cell - 9 vertex workflow unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonagonCell {
    /// Cell identifier
    pub id: String,
    /// Alpha (semantic) trivariate
    pub alpha: AlphaTrivariate,
    /// Beta (operational) trivariate
    pub beta: BetaTrivariate,
    /// Gamma (temporal) trivariate
    pub gamma: GammaTrivariate,
    /// Computed TETH entropy
    pub teth_entropy: f64,
    /// Confidence score [0.0, 1.0]
    pub confidence: f64,
    /// Parent cell ID (if nested)
    pub parent_id: Option<String>,
    /// Child cell IDs
    pub children: Vec<String>,
}

impl NonagonCell {
    /// Create new nonagon cell with default trivariates
    pub fn new(id: impl Into<String>) -> Self {
        let mut cell = Self {
            id: id.into(),
            alpha: AlphaTrivariate::default(),
            beta: BetaTrivariate::default(),
            gamma: GammaTrivariate::default(),
            teth_entropy: 0.0,
            confidence: 0.0,
            parent_id: None,
            children: Vec::new(),
        };
        cell.recalculate();
        cell
    }

    /// Create with specific trivariates
    pub fn with_trivariates(
        id: impl Into<String>,
        alpha: AlphaTrivariate,
        beta: BetaTrivariate,
        gamma: GammaTrivariate,
    ) -> Self {
        let mut cell = Self {
            id: id.into(),
            alpha,
            beta,
            gamma,
            teth_entropy: 0.0,
            confidence: 0.0,
            parent_id: None,
            children: Vec::new(),
        };
        cell.recalculate();
        cell
    }

    /// Recalculate TETH entropy and confidence
    pub fn recalculate(&mut self) {
        self.teth_entropy = calculate_teth_entropy(self);
        self.confidence = self.calculate_confidence();
    }

    /// Calculate confidence based on trivariate alignment
    fn calculate_confidence(&self) -> f64 {
        let alpha_mag = Trivariate::from(self.alpha).magnitude();
        let beta_mag = Trivariate::from(self.beta).magnitude();
        let gamma_mag = Trivariate::from(self.gamma).magnitude();

        // Average magnitude normalized to [0,1]
        let avg_mag = (alpha_mag + beta_mag + gamma_mag) / 3.0;
        let max_mag = 3.0_f64.sqrt(); // sqrt(1^2 + 1^2 + 1^2)

        (avg_mag / max_mag).min(1.0)
    }

    /// Check if cell meets minimum entropy threshold
    pub fn is_valid(&self) -> bool {
        self.teth_entropy >= MIN_TETH_ENTROPY
    }

    /// Get all 9 vertex values as array
    pub fn vertices(&self) -> [f64; 9] {
        [
            self.alpha.context,
            self.alpha.meaning,
            self.alpha.intent,
            self.beta.phase,
            self.beta.intensity,
            self.beta.duration,
            self.gamma.historical,
            self.gamma.current,
            self.gamma.predictive,
        ]
    }

    /// Set HD4 phase (maps to beta.phase)
    pub fn set_hd4_phase(&mut self, phase: HD4Phase) {
        self.beta.phase = phase.to_beta_x();
        self.recalculate();
    }
}

/// HD4 Kill Chain phases
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HD4Phase {
    Hunt,
    Detect,
    Disrupt,
    Disable,
    Dominate,
}

impl HD4Phase {
    /// Map phase to beta.x value
    pub fn to_beta_x(self) -> f64 {
        match self {
            HD4Phase::Hunt => 0.2,
            HD4Phase::Detect => 0.4,
            HD4Phase::Disrupt => 0.6,
            HD4Phase::Disable => 0.8,
            HD4Phase::Dominate => 1.0,
        }
    }

    /// Create from beta.x value
    pub fn from_beta_x(x: f64) -> Self {
        if x <= 0.3 {
            HD4Phase::Hunt
        } else if x <= 0.5 {
            HD4Phase::Detect
        } else if x <= 0.7 {
            HD4Phase::Disrupt
        } else if x <= 0.9 {
            HD4Phase::Disable
        } else {
            HD4Phase::Dominate
        }
    }
}

/// Nonagon configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonagonConfig {
    /// Minimum TETH entropy threshold
    pub min_entropy: f64,
    /// Delta precision for calculations
    pub delta_precision: f64,
    /// Enable HMM phase detection
    pub hmm_enabled: bool,
    /// Enable convergence monitoring
    pub convergence_enabled: bool,
}

impl Default for NonagonConfig {
    fn default() -> Self {
        Self {
            min_entropy: MIN_TETH_ENTROPY,
            delta_precision: DELTA_PRECISION,
            hmm_enabled: true,
            convergence_enabled: true,
        }
    }
}

/// Calculate TETH entropy for a nonagon cell
/// Uses Shannon entropy over 9 vertex probability distribution
pub fn calculate_teth_entropy(cell: &NonagonCell) -> f64 {
    let vertices = cell.vertices();

    // Normalize to probability distribution
    let sum: f64 = vertices.iter().sum();
    if sum < DELTA_PRECISION {
        return 0.0;
    }

    let probabilities: Vec<f64> = vertices.iter().map(|v| v / sum).collect();

    // Shannon entropy: H = -Σ p(x) * log2(p(x))
    let entropy: f64 = probabilities
        .iter()
        .filter(|&&p| p > DELTA_PRECISION)
        .map(|&p| -p * (p.ln() / LN_2))
        .sum();

    // Round to 4 decimal places for consistency
    (entropy * 10000.0).round() / 10000.0
}

/// Calculate center of mass for nonagon vertices
pub fn calculate_center_of_mass(cell: &NonagonCell) -> Trivariate {
    Trivariate {
        x: (cell.alpha.context + cell.beta.phase + cell.gamma.historical) / 3.0,
        y: (cell.alpha.meaning + cell.beta.intensity + cell.gamma.current) / 3.0,
        z: (cell.alpha.intent + cell.beta.duration + cell.gamma.predictive) / 3.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_nonagon_entropy() {
        let cell = NonagonCell::new("test-cell");
        assert!(cell.teth_entropy > 0.0);
        println!("Default TETH entropy: {}", cell.teth_entropy);
    }

    #[test]
    fn test_hd4_phase_mapping() {
        assert_eq!(HD4Phase::Hunt.to_beta_x(), 0.2);
        assert_eq!(HD4Phase::Dominate.to_beta_x(), 1.0);
        assert_eq!(HD4Phase::from_beta_x(0.25), HD4Phase::Hunt);
        assert_eq!(HD4Phase::from_beta_x(0.95), HD4Phase::Dominate);
    }

    #[test]
    fn test_trivariate_normalize() {
        let t = Trivariate::new(3.0, 4.0, 0.0);
        let normalized = t.normalize();
        assert!((normalized.magnitude() - 1.0).abs() < DELTA_PRECISION);
    }
}
