//! CTAS-7 Core Provenance Structures
//! Core data structures for crate provenance tracking
//! Follows CTAS-7 standards: ≤200 LOC

use serde::{Deserialize, Serialize};

use super::source::SourceProvenance;
use super::build::BuildProvenance;
use super::quality::QualityProvenance;
use super::security::SecurityProvenance;
use super::deployment::DeploymentProvenance;
use super::blockchain::BlockchainAnchor;
use super::lifecycle::{LifecycleEvent, ProvenanceTransaction};

/// Complete crate provenance record with ACID guarantees
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateProvenance {
    /// Unique crate identifier
    pub crate_id: String,
    /// Crate name following ctas7- convention
    pub crate_name: String,
    /// Version information
    pub version: String,

    /// Source provenance
    pub source: SourceProvenance,
    /// Build provenance
    pub build: BuildProvenance,
    /// Quality provenance
    pub quality: QualityProvenance,
    /// Security provenance
    pub security: SecurityProvenance,
    /// Deployment provenance
    pub deployment: DeploymentProvenance,

    /// Blockchain anchoring
    pub blockchain_anchors: Vec<BlockchainAnchor>,
    /// Digital signatures
    pub signatures: Vec<ProvenanceSignature>,

    /// Lifecycle events with timestamps
    pub lifecycle_events: Vec<LifecycleEvent>,
    /// ACID transaction log
    pub transaction_log: Vec<ProvenanceTransaction>,

    /// Creation and modification timestamps
    pub created_at: u64,
    pub updated_at: u64,
    /// Provenance version for conflict resolution
    pub provenance_version: u32,
}

/// Digital signature for provenance integrity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceSignature {
    /// Signature algorithm
    pub algorithm: String,
    /// Public key fingerprint
    pub key_fingerprint: String,
    /// Digital signature
    pub signature: String,
    /// Timestamp when signed
    pub signed_at: u64,
    /// Signer identity
    pub signer: String,
    /// Tesla verification status
    pub tesla_verified: bool,
}

impl CrateProvenance {
    /// Create new provenance record
    pub fn new(crate_id: String, crate_name: String, version: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            crate_id,
            crate_name,
            version,
            source: SourceProvenance::default(),
            build: BuildProvenance::default(),
            quality: QualityProvenance::default(),
            security: SecurityProvenance::default(),
            deployment: DeploymentProvenance::default(),
            blockchain_anchors: Vec::new(),
            signatures: Vec::new(),
            lifecycle_events: Vec::new(),
            transaction_log: Vec::new(),
            created_at: now,
            updated_at: now,
            provenance_version: 1,
        }
    }

    /// Validate provenance completeness
    pub fn is_complete(&self) -> bool {
        !self.crate_id.is_empty()
            && !self.crate_name.is_empty()
            && !self.version.is_empty()
            && self.source.is_valid()
            && self.build.is_valid()
            && self.quality.is_valid()
    }

    /// Calculate provenance integrity hash
    pub fn integrity_hash(&self) -> String {
        use crate::hash_engine::Hasher;
        let mut hasher = Hasher::new();

        hasher.update(self.crate_id.as_bytes());
        hasher.update(self.crate_name.as_bytes());
        hasher.update(self.version.as_bytes());
        hasher.update(&self.created_at.to_le_bytes());
        hasher.update(&self.provenance_version.to_le_bytes());

        format!("{}", hasher.finalize().to_hex())
    }

    /// Add signature to provenance
    pub fn add_signature(&mut self, signature: ProvenanceSignature) {
        self.signatures.push(signature);
        self.update_timestamp();
    }

    /// Update modification timestamp
    pub fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.provenance_version += 1;
    }

    /// Tesla-grade validation
    pub fn tesla_grade_validation(&self) -> bool {
        self.signatures.iter().any(|s| s.tesla_verified)
            && self.source.repository.tesla_approved
            && self.quality.tesla_grading.overall_grade >= 90
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_provenance() {
        let prov = CrateProvenance::new(
            "test-id".to_string(),
            "test-crate".to_string(),
            "1.0.0".to_string()
        );

        assert_eq!(prov.crate_id, "test-id");
        assert_eq!(prov.crate_name, "test-crate");
        assert_eq!(prov.version, "1.0.0");
        assert_eq!(prov.provenance_version, 1);
    }

    #[test]
    fn test_integrity_hash() {
        let prov = CrateProvenance::new(
            "test".to_string(),
            "test".to_string(),
            "1.0.0".to_string()
        );

        let hash = prov.integrity_hash();
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // Blake3 hex output length
    }

    #[test]
    fn test_update_timestamp() {
        let mut prov = CrateProvenance::new(
            "test".to_string(),
            "test".to_string(),
            "1.0.0".to_string()
        );

        let original_version = prov.provenance_version;
        let original_updated = prov.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(10));
        prov.update_timestamp();

        assert!(prov.updated_at > original_updated);
        assert_eq!(prov.provenance_version, original_version + 1);
    }
}