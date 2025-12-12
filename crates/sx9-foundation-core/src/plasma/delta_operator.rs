//! PLASMA Delta Operator Module
//!
//! Implements Delta as a noise gating operator for toolchain escalation.
//! Computes delta angle, entropy drift, and semantic drift between contexts.

use crate::trivariate_hash_v731::{ContextFrame, TrivariateHash};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Delta measurement result
#[derive(Debug, Clone)]
pub struct DeltaMeasurement {
    /// Delta angle in degrees (0.0-180.0)
    pub delta_angle: f32,
    /// Entropy drift (0.0-1.0)
    pub entropy_drift: f32,
    /// Semantic drift (0.0-1.0)
    pub semantic_drift: f32,
    /// Combined noise score (0.0-1.0)
    pub noise_score: f32,
}

impl DeltaMeasurement {
    /// Create new delta measurement
    pub fn new(delta_angle: f32, entropy_drift: f32, semantic_drift: f32) -> Self {
        let noise_score = Self::compute_noise_score(delta_angle, entropy_drift, semantic_drift);
        Self {
            delta_angle,
            entropy_drift,
            semantic_drift,
            noise_score,
        }
    }

    /// Compute combined noise score from components
    fn compute_noise_score(delta_angle: f32, entropy_drift: f32, semantic_drift: f32) -> f32 {
        // Normalize delta angle to 0.0-1.0 (assuming max 180 degrees)
        let normalized_angle = (delta_angle.abs() / 180.0).min(1.0);

        // Weighted combination: 40% angle, 30% entropy, 30% semantic
        (normalized_angle * 0.4) + (entropy_drift * 0.3) + (semantic_drift * 0.3)
    }
}

/// PLASMA Delta Operator
pub struct DeltaOperator {
    /// Feature flag: enable/disable delta operator
    pub enabled: bool,
}

impl DeltaOperator {
    /// Create new delta operator
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Compute delta angle between two contexts
    /// Returns angle in degrees (0.0-180.0)
    pub fn compute_delta_angle(&self, ctx1: &ContextFrame, ctx2: &ContextFrame) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        // Compute vector difference
        let delta_timestamp = (ctx2.timestamp as f32) - (ctx1.timestamp as f32);
        let delta_agent = (ctx2.agent_id as f32) - (ctx1.agent_id as f32);
        let delta_lineage = (ctx2.lineage as f32) - (ctx1.lineage as f32);
        let delta_angle_rad = ctx2.delta_angle - ctx1.delta_angle;

        // Compute magnitude
        let magnitude = (delta_timestamp.powi(2)
            + delta_agent.powi(2)
            + delta_lineage.powi(2)
            + delta_angle_rad.powi(2))
        .sqrt();

        // Convert to degrees and normalize to 0-180
        let angle_degrees = magnitude.to_degrees();
        angle_degrees.min(180.0)
    }

    /// Compute entropy drift between two contexts
    /// Measures randomness/uncertainty change (0.0-1.0)
    pub fn compute_entropy_drift(&self, ctx1: &ContextFrame, ctx2: &ContextFrame) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        // Compute entropy for each context
        let entropy1 = Self::context_entropy(ctx1);
        let entropy2 = Self::context_entropy(ctx2);

        // Drift is absolute difference normalized
        (entropy2 - entropy1).abs().min(1.0)
    }

    /// Compute semantic drift between two trivariate hashes
    /// Measures semantic similarity change (0.0-1.0)
    pub fn compute_semantic_drift(&self, hash1: &TrivariateHash, hash2: &TrivariateHash) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        // Compare SCH components (semantic identity)
        let sch_diff = Self::hash_component_diff(&hash1.sch, &hash2.sch);

        // Compare CUID components (context similarity)
        let cuid_diff = Self::hash_component_diff(&hash1.cuid, &hash2.cuid);

        // Semantic drift is weighted: 70% SCH, 30% CUID
        (sch_diff * 0.7) + (cuid_diff * 0.3)
    }

    /// Compute complete delta measurement between two contexts and hashes
    pub fn measure_delta(
        &self,
        ctx1: &ContextFrame,
        ctx2: &ContextFrame,
        hash1: &TrivariateHash,
        hash2: &TrivariateHash,
    ) -> DeltaMeasurement {
        let delta_angle = self.compute_delta_angle(ctx1, ctx2);
        let entropy_drift = self.compute_entropy_drift(ctx1, ctx2);
        let semantic_drift = self.compute_semantic_drift(hash1, hash2);

        DeltaMeasurement::new(delta_angle, entropy_drift, semantic_drift)
    }

    // Helper: Compute context entropy
    fn context_entropy(ctx: &ContextFrame) -> f32 {
        // Use hash of context fields to measure entropy
        let mut hasher = DefaultHasher::new();
        ctx.timestamp.hash(&mut hasher);
        ctx.agent_id.hash(&mut hasher);
        ctx.delta_angle.to_bits().hash(&mut hasher);
        ctx.lineage.hash(&mut hasher);
        ctx.nonce.hash(&mut hasher);

        let hash = hasher.finish();
        // Normalize to 0.0-1.0
        (hash as f32) / (u64::MAX as f32)
    }

    // Helper: Compute difference between hash components
    fn hash_component_diff(comp1: &str, comp2: &str) -> f32 {
        if comp1.len() != comp2.len() {
            return 1.0; // Maximum drift if lengths differ
        }

        let mut diff_count = 0;
        for (c1, c2) in comp1.chars().zip(comp2.chars()) {
            if c1 != c2 {
                diff_count += 1;
            }
        }

        // Normalize to 0.0-1.0
        (diff_count as f32) / (comp1.len() as f32)
    }
}

impl Default for DeltaOperator {
    fn default() -> Self {
        Self::new(true) // Enabled by default
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trivariate_hash_v731::{ExecEnv, ExecState};

    #[test]
    fn test_delta_angle_computation() {
        let operator = DeltaOperator::new(true);

        let ctx1 = ContextFrame::new(ExecEnv::Wasm, 1, ExecState::Hot);
        let mut ctx2 = ContextFrame::new(ExecEnv::Wasm, 2, ExecState::Hot);
        ctx2.delta_angle = 45.0;

        let angle = operator.compute_delta_angle(&ctx1, &ctx2);
        assert!(angle >= 0.0 && angle <= 180.0);
    }

    #[test]
    fn test_entropy_drift() {
        let operator = DeltaOperator::new(true);

        let ctx1 = ContextFrame::new(ExecEnv::Wasm, 1, ExecState::Hot);
        let ctx2 = ContextFrame::new(ExecEnv::Container, 2, ExecState::Warm);

        let drift = operator.compute_entropy_drift(&ctx1, &ctx2);
        assert!(drift >= 0.0 && drift <= 1.0);
    }

    #[test]
    fn test_semantic_drift() {
        let operator = DeltaOperator::new(true);

        let hash1 = TrivariateHash::new(
            "aB7x9pQw2zRt4kMn".to_string(),
            "c5j8k3p2q7w1x9z".to_string(),
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        );

        let hash2 = TrivariateHash::new(
            "xY9mP4qR8sT2wN5k".to_string(),
            "d6k9l4p3q8w2x0z".to_string(),
            "660f9501-f3ac-52e5-b827-557766551111".to_string(),
        );

        let drift = operator.compute_semantic_drift(&hash1, &hash2);
        assert!(drift >= 0.0 && drift <= 1.0);
    }

    #[test]
    fn test_complete_measurement() {
        let operator = DeltaOperator::new(true);

        let ctx1 = ContextFrame::new(ExecEnv::Wasm, 1, ExecState::Hot);
        let ctx2 = ContextFrame::new(ExecEnv::Container, 2, ExecState::Warm);

        let hash1 = TrivariateHash::new(
            "aB7x9pQw2zRt4kMn".to_string(),
            "c5j8k3p2q7w1x9z".to_string(),
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        );

        let hash2 = TrivariateHash::new(
            "xY9mP4qR8sT2wN5k".to_string(),
            "d6k9l4p3q8w2x0z".to_string(),
            "660f9501-f3ac-52e5-b827-557766551111".to_string(),
        );

        let measurement = operator.measure_delta(&ctx1, &ctx2, &hash1, &hash2);

        assert!(measurement.delta_angle >= 0.0 && measurement.delta_angle <= 180.0);
        assert!(measurement.entropy_drift >= 0.0 && measurement.entropy_drift <= 1.0);
        assert!(measurement.semantic_drift >= 0.0 && measurement.semantic_drift <= 1.0);
        assert!(measurement.noise_score >= 0.0 && measurement.noise_score <= 1.0);
    }

    #[test]
    fn test_disabled_operator() {
        let operator = DeltaOperator::new(false);

        let ctx1 = ContextFrame::new(ExecEnv::Wasm, 1, ExecState::Hot);
        let ctx2 = ContextFrame::new(ExecEnv::Container, 2, ExecState::Warm);

        assert_eq!(operator.compute_delta_angle(&ctx1, &ctx2), 0.0);
        assert_eq!(operator.compute_entropy_drift(&ctx1, &ctx2), 0.0);
    }
}
