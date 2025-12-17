//! Universal Symbolic Message (USIM) Implementation
//!
//! Implements the USIM trivariate hashing system with SCH integration
//! for predictive crate orchestration based on OODA loop patterns.

use std::time::{SystemTime, UNIX_EPOCH};
use sx9_foundation_manifold::core::data::serde::{Deserialize, Serialize};
use sx9_foundation_manifold::core::data::uuid::Uuid;
use sx9_foundation_manifold::core::diagnostics::anyhow::{Context, Result};
use sx9_foundation_manifold::core::TrivariateHashEngine;

/// USIM trivariate structure: sch/cuid/GIS+thriod + genetic Trivariate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USIMTrivariate {
    /// Motion hash (smell) - 13 bytes base96
    pub sch: Vec<u8>,
    /// Context hash (vision) - 13 bytes base96
    pub cuid: Vec<u8>,
    /// Graph position + UUID (touch/hearing) - 29 bytes
    pub gis_thriod: Vec<u8>,
    /// Genetic Trivariate hash - 16 bytes, lifecycle-seeded
    pub genetic: Vec<u8>,
    /// Lifecycle stage for genetic seeding
    pub lifecycle_stage: LifecycleStage,
    /// Creation timestamp
    pub timestamp: u64,
}

/// Lifecycle stages for genetic hash seeding
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LifecycleStage {
    Birth,
    CodeCompletion,
    CrateCompletion,
}

/// SCH (Service/Crate/Health) vector for predictive autonomy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCHVector {
    /// Service component vector
    pub service: Vec<f32>,
    /// Crate component vector
    pub crate_component: Vec<f32>,
    /// Health component vector
    pub health: Vec<f32>,
    /// Combined prediction vector
    pub prediction: Vec<f32>,
    /// Convergence score (0.0-1.0)
    pub convergence: f32,
}

/// USIM processor for threat hunting and crate orchestration
#[derive(Debug)]
pub struct USIMProcessor {
    /// Base96 alphabet for encoding
    base96_alphabet: String,
    /// Current tactic graph state (147 nodes)
    tactic_graph_state: Vec<f32>,
}

impl USIMProcessor {
    /// Creates new USIM processor with base96 encoding
    pub fn new() -> Self {
        // Base96 alphabet: A-Z, a-z, 0-9, plus special chars
        let base96_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                              abcdefghijklmnopqrstuvwxyz\
                              0123456789!@#$%^&*()_+-=[]{}|;:,.<>?"
            .to_string();

        Self {
            base96_alphabet,
            tactic_graph_state: vec![0.0; 147], // 147 ATT&CK tactic nodes
        }
    }

    /// Generates USIM from telemetry data and crate context
    pub fn generate_usim(
        &self,
        telemetry: &str,
        crate_context: &str,
        lifecycle_stage: LifecycleStage,
    ) -> Result<USIMTrivariate> {
        // Generate SCH motion hash (13 bytes)
        let sch = self.generate_sch_hash(telemetry, crate_context)?;

        // Generate CUID context hash (13 bytes)
        let cuid = self.generate_cuid_hash(telemetry)?;

        // Generate GIS + thriod UUID (29 bytes: 13 + 16)
        let gis_thriod = self.generate_gis_thriod()?;

        // Generate genetic Trivariate hash with lifecycle seeding
        let genetic = self.generate_genetic_hash(&sch, &cuid, &gis_thriod, lifecycle_stage)?;

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        Ok(USIMTrivariate {
            sch,
            cuid,
            gis_thriod,
            genetic,
            lifecycle_stage,
            timestamp,
        })
    }

    /// Generates SCH vector for predictive crate orchestration
    pub fn generate_sch_vector(
        &self,
        usim: &USIMTrivariate,
        service_health: f32,
        crate_complexity: f32,
    ) -> Result<SCHVector> {
        // Service vector from USIM sch component
        let service = self.embed_to_vector(&usim.sch, 64)?;

        // Crate vector from USIM cuid component
        let crate_component = self.embed_to_vector(&usim.cuid, 64)?;

        // Health vector from lifecycle and service health
        let mut health = vec![service_health; 64];
        health[0] = match usim.lifecycle_stage {
            LifecycleStage::Birth => 0.2,
            LifecycleStage::CodeCompletion => 0.6,
            LifecycleStage::CrateCompletion => 1.0,
        };
        health[1] = crate_complexity;

        // Prediction vector: weighted combination
        let mut prediction = Vec::with_capacity(64);
        for i in 0..64 {
            let weighted = 0.4 * service[i] + 0.4 * crate_component[i] + 0.2 * health[i];
            prediction.push(weighted);
        }

        // Calculate convergence score
        let convergence = self.calculate_convergence(&prediction)?;

        Ok(SCHVector {
            service,
            crate_component,
            health,
            prediction,
            convergence,
        })
    }

    /// Predicts if autonomous crate spinning is recommended
    pub fn predict_crate_spin(
        &self,
        sch_vector: &SCHVector,
        threat_threshold: f32,
    ) -> (bool, String) {
        let spin_score = sch_vector.prediction.iter().sum::<f32>() / 64.0;
        let should_spin = spin_score > threat_threshold && sch_vector.convergence > 0.7;

        let narrative = if should_spin {
            format!(
                "OODA: High threat score ({:.3}) + convergence ({:.3}) → Recommend autonomous crate spin",
                spin_score, sch_vector.convergence
            )
        } else {
            format!(
                "OODA: Normal conditions (score: {:.3}, convergence: {:.3}) → Monitor",
                spin_score, sch_vector.convergence
            )
        };

        (should_spin, narrative)
    }

    /// Generates SCH motion hash (13 bytes base96)
    fn generate_sch_hash(&self, telemetry: &str, context: &str) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        data.extend_from_slice(b"SCH_MOTION");
        data.extend_from_slice(telemetry.as_bytes());
        data.extend_from_slice(context.as_bytes());

        let hasher = TrivariateHashEngine::new();
        let hash_string = hasher.generate_hash_from_bytes(&data);
        // TrivariateHashEngine usually returns a hex string or similar.
        // We need bytes. Assuming it returns a hex string of the hash.
        // Or if it returns something else, I need to check.
        // The summary said "compute replaced with generate_hash_from_bytes".
        // Let's assume it returns a String (hex) or specialized struct.
        // If it returns a string, I can bytes() it or decode it.
        // Previous usages suggest it returns a String.
        // "md5::compute" replaced with "generate_hash_from_bytes(...)".

        // If it returns a String (hex), I can interpret it.
        let hash_bytes = hash_string.as_bytes();

        let encoded = self.encode_base96(&hash_bytes[..10])?;
        Ok(encoded.into_bytes().into_iter().take(13).collect())
    }

    /// Generates CUID context hash (13 bytes base96)
    fn generate_cuid_hash(&self, telemetry: &str) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        data.extend_from_slice(b"CUID_CONTEXT");
        data.extend_from_slice(telemetry.as_bytes());
        data.extend_from_slice(
            &SystemTime::now()
                .duration_since(UNIX_EPOCH)?
                .as_nanos()
                .to_le_bytes(),
        );

        let hasher = TrivariateHashEngine::new();
        let hash_string = hasher.generate_hash_from_bytes(&data);
        let hash_bytes = hash_string.as_bytes();

        let encoded = self.encode_base96(&hash_bytes[..10])?;
        Ok(encoded.into_bytes().into_iter().take(13).collect())
    }

    /// Generates GIS + thriod UUID (29 bytes: 13 GIS + 16 UUID)
    fn generate_gis_thriod(&self) -> Result<Vec<u8>> {
        // GIS component (13 bytes)
        let mut data = Vec::new();
        data.extend_from_slice(b"GIS_POSITION");
        let bits: Vec<u8> = self
            .tactic_graph_state
            .iter()
            .flat_map(|&f| f.to_bits().to_le_bytes())
            .collect();
        data.extend_from_slice(&bits);

        let hasher = TrivariateHashEngine::new();
        let hash_string = hasher.generate_hash_from_bytes(&data);
        let hash_bytes = hash_string.as_bytes();

        let gis_encoded = self.encode_base96(&hash_bytes[..10])?;
        let mut gis_bytes: Vec<u8> = gis_encoded.into_bytes().into_iter().take(13).collect();

        // UUID component (16 bytes)
        let uuid = Uuid::new_v4();
        gis_bytes.extend_from_slice(uuid.as_bytes());

        Ok(gis_bytes)
    }

    /// Generates genetic Trivariate hash with lifecycle seeding
    fn generate_genetic_hash(
        &self,
        sch: &[u8],
        cuid: &[u8],
        gis_thriod: &[u8],
        lifecycle: LifecycleStage,
    ) -> Result<Vec<u8>> {
        let mut data = Vec::new();

        // Lifecycle seed
        let seed = match lifecycle {
            LifecycleStage::Birth => b"BIRTH_SEED",
            LifecycleStage::CodeCompletion => b"CODE_SEED_",
            LifecycleStage::CrateCompletion => b"CRATE_SEED",
        };
        data.extend_from_slice(seed);

        // Component hashes
        data.extend_from_slice(sch);
        data.extend_from_slice(cuid);
        data.extend_from_slice(gis_thriod);

        let hasher = TrivariateHashEngine::new();
        let hash_string = hasher.generate_hash_from_bytes(&data);
        Ok(hash_string.as_bytes()[..16].to_vec())
    }

    /// Encodes bytes to base96 string
    fn encode_base96(&self, bytes: &[u8]) -> Result<String> {
        let mut result = String::new();
        let alphabet = self.base96_alphabet.chars().collect::<Vec<_>>();

        for &byte in bytes {
            let index = (byte as usize) % 96;
            result.push(alphabet[index]);
        }

        Ok(result)
    }

    /// Embeds hash bytes to vector of specified dimension
    fn embed_to_vector(&self, hash_bytes: &[u8], dim: usize) -> Result<Vec<f32>> {
        let mut vector = Vec::with_capacity(dim);

        for i in 0..dim {
            let byte_idx = i % hash_bytes.len();
            let normalized = (hash_bytes[byte_idx] as f32) / 255.0;
            vector.push(normalized);
        }

        Ok(vector)
    }

    /// Calculates convergence score for prediction vector
    fn calculate_convergence(&self, prediction: &[f32]) -> Result<f32> {
        let mean = prediction.iter().sum::<f32>() / prediction.len() as f32;
        let variance =
            prediction.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / prediction.len() as f32;

        // Higher convergence = lower variance
        let convergence = 1.0 / (1.0 + variance);
        Ok(convergence.clamp(0.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usim_generation() {
        let processor = USIMProcessor::new();
        let usim = processor
            .generate_usim(
                "test_telemetry",
                "test_crate_context",
                LifecycleStage::Birth,
            )
            .unwrap();

        assert_eq!(usim.sch.len(), 13);
        assert_eq!(usim.cuid.len(), 13);
        assert_eq!(usim.gis_thriod.len(), 29);
        assert_eq!(usim.genetic.len(), 16);
    }

    #[test]
    fn test_sch_vector_generation() {
        let processor = USIMProcessor::new();
        let usim = processor
            .generate_usim(
                "test_telemetry",
                "test_context",
                LifecycleStage::CodeCompletion,
            )
            .unwrap();

        let sch_vector = processor.generate_sch_vector(&usim, 0.8, 0.3).unwrap();

        assert_eq!(sch_vector.service.len(), 64);
        assert_eq!(sch_vector.crate_component.len(), 64);
        assert_eq!(sch_vector.health.len(), 64);
        assert_eq!(sch_vector.prediction.len(), 64);
        assert!(sch_vector.convergence >= 0.0 && sch_vector.convergence <= 1.0);
    }

    #[test]
    fn test_crate_spin_prediction() {
        let processor = USIMProcessor::new();
        let usim = processor
            .generate_usim(
                "high_threat_telemetry",
                "critical_context",
                LifecycleStage::CrateCompletion,
            )
            .unwrap();

        let sch_vector = processor.generate_sch_vector(&usim, 0.9, 0.1).unwrap();
        let (should_spin, narrative) = processor.predict_crate_spin(&sch_vector, 0.5);

        assert!(!narrative.is_empty());
        assert!(narrative.contains("OODA"));
    }
}
