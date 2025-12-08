/// Persistent EEI Integration Module
///
/// Integrates existing EEI structures from the unified data structure with
/// distributed EEI systems to support persistent intelligence requirements
/// for metsoc, hash mask global systems, and other long-term intelligence needs.

use crate::temporal_intelligence::{EEI, EEIType, LTOV};
use crate::distributed_eei::{EEISpecialization, DistributedEEINode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;
use tracing::{info, warn};

/// Enhanced EEI requirement for persistent intelligence
/// Bridges the gap between interview structures and operational intelligence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentEEI {
    pub id: Uuid,
    pub eei_id: String,
    pub question: String,
    pub persistence_type: PersistenceType,
    pub specialization: EEISpecialization,
    pub priority: PersistentPriority,
    pub collection_method: String,
    pub timeline: PersistentTimeline,
    pub global_scope: bool,
    pub retention_policy: RetentionPolicy,
}

/// Types of persistent intelligence requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PersistenceType {
    /// Meteorological and environmental intelligence (persistent)
    MetSocIntelligence,
    /// Global hash mask and signature analysis (persistent)
    HashMaskGlobal,
    /// Continuous threat hunting operations (persistent)
    ContinuousHunter,
    /// Child protection measures and monitoring (persistent)
    ChildProtection,
    /// National security implications tracking (persistent)
    NationalSecurity,
    /// Strategic threat intelligence (long-term)
    StrategicThreat,
    /// Infrastructure monitoring (persistent)
    InfrastructureWatch,
    /// Regulatory compliance tracking (persistent)
    ComplianceWatch,
}

/// Priority levels for persistent EEIs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PersistentPriority {
    /// Critical national security or child protection
    Critical,
    /// High operational importance
    High,
    /// Standard persistent intelligence
    Medium,
    /// Background monitoring
    Low,
    /// Research and development
    Research,
}

/// Timeline specifications for persistent EEIs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentTimeline {
    /// Minimum retention period
    pub min_retention: Duration,
    /// Maximum retention period (None = indefinite)
    pub max_retention: Option<Duration>,
    /// Update frequency
    pub update_frequency: Duration,
    /// Review cycle
    pub review_cycle: Duration,
}

/// Retention policy for persistent intelligence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Base retention period
    pub base_retention: Duration,
    /// Decay function type
    pub decay_type: DecayType,
    /// Classification level affects retention
    pub classification_multiplier: f32,
    /// Archive after this period
    pub archive_threshold: Option<Duration>,
}

/// Decay types for persistent intelligence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecayType {
    /// No decay - permanent retention
    None,
    /// Very slow linear decay
    SlowLinear,
    /// Exponential decay with long half-life
    LongExponential,
    /// Step decay at specific intervals
    StepDecay,
}

/// Persistent EEI manager for long-term intelligence operations
pub struct PersistentEEIManager {
    persistent_eeis: HashMap<Uuid, PersistentEEI>,
    specialization_map: HashMap<EEISpecialization, Vec<Uuid>>,
    global_retention_policies: HashMap<PersistenceType, RetentionPolicy>,
}

impl PersistentEEIManager {
    /// Create new persistent EEI manager with default policies
    pub fn new() -> Self {
        let mut manager = Self {
            persistent_eeis: HashMap::new(),
            specialization_map: HashMap::new(),
            global_retention_policies: HashMap::new(),
        };

        manager.initialize_default_policies();
        manager
    }

    /// Initialize default retention policies for different persistence types
    fn initialize_default_policies(&mut self) {
        // MetSoc intelligence - meteorological data with seasonal patterns
        self.global_retention_policies.insert(
            PersistenceType::MetSocIntelligence,
            RetentionPolicy {
                base_retention: Duration::from_secs(365 * 24 * 60 * 60 * 5), // 5 years
                decay_type: DecayType::SlowLinear,
                classification_multiplier: 1.0,
                archive_threshold: Some(Duration::from_secs(365 * 24 * 60 * 60 * 2)), // Archive after 2 years
            }
        );

        // Hash mask global - permanent signature intelligence
        self.global_retention_policies.insert(
            PersistenceType::HashMaskGlobal,
            RetentionPolicy {
                base_retention: Duration::from_secs(365 * 24 * 60 * 60 * 10), // 10 years
                decay_type: DecayType::None, // No decay for hash signatures
                classification_multiplier: 1.0,
                archive_threshold: Some(Duration::from_secs(365 * 24 * 60 * 60 * 3)), // Archive after 3 years
            }
        );

        // Child protection - permanent retention with high priority
        self.global_retention_policies.insert(
            PersistenceType::ChildProtection,
            RetentionPolicy {
                base_retention: Duration::from_secs(u64::MAX), // Permanent
                decay_type: DecayType::None,
                classification_multiplier: 2.0, // Enhanced retention
                archive_threshold: None, // Never archive
            }
        );

        // National security - extended retention with classification scaling
        self.global_retention_policies.insert(
            PersistenceType::NationalSecurity,
            RetentionPolicy {
                base_retention: Duration::from_secs(365 * 24 * 60 * 60 * 25), // 25 years
                decay_type: DecayType::LongExponential,
                classification_multiplier: 3.0, // Classification affects retention significantly
                archive_threshold: Some(Duration::from_secs(365 * 24 * 60 * 60 * 7)), // Archive after 7 years
            }
        );

        // Continuous hunter operations
        self.global_retention_policies.insert(
            PersistenceType::ContinuousHunter,
            RetentionPolicy {
                base_retention: Duration::from_secs(365 * 24 * 60 * 60 * 3), // 3 years
                decay_type: DecayType::SlowLinear,
                classification_multiplier: 1.5,
                archive_threshold: Some(Duration::from_secs(365 * 24 * 60 * 60)), // Archive after 1 year
            }
        );
    }

    /// Register a new persistent EEI with the system
    pub fn register_persistent_eei(&mut self, persistent_eei: PersistentEEI) -> Result<(), String> {
        let id = persistent_eei.id;
        let specialization = persistent_eei.specialization.clone();

        // Validate retention policy
        if !self.global_retention_policies.contains_key(&persistent_eei.persistence_type) {
            return Err(format!("No retention policy defined for {:?}", persistent_eei.persistence_type));
        }

        // Store the persistent EEI
        self.persistent_eeis.insert(id, persistent_eei);

        // Update specialization mapping
        self.specialization_map
            .entry(specialization)
            .or_insert_with(Vec::new)
            .push(id);

        info!("Registered persistent EEI {} with specialization {:?}", id, specialization);
        Ok(())
    }

    /// Convert persistent EEI to operational EEI with extended LTOV
    pub fn to_operational_eei(&self, persistent_id: Uuid) -> Option<EEI> {
        let persistent_eei = self.persistent_eeis.get(&persistent_id)?;
        let retention_policy = self.global_retention_policies.get(&persistent_eei.persistence_type)?;

        // Create extended LTOV based on retention policy
        let ltov = match retention_policy.decay_type {
            DecayType::None => LTOV::new(
                Duration::from_secs(u64::MAX), // Permanent
                0.0, // No decay
                1.0  // Full relevance maintained
            ),
            DecayType::SlowLinear => LTOV::new(
                retention_policy.base_retention,
                0.01, // Very slow decay
                0.95  // High base relevance
            ),
            DecayType::LongExponential => LTOV::new(
                retention_policy.base_retention,
                0.05, // Slow exponential decay
                0.9   // High base relevance
            ),
            DecayType::StepDecay => LTOV::new(
                retention_policy.base_retention,
                0.02, // Step-wise decay
                0.85  // Good base relevance
            ),
        };

        // Determine EEI type based on persistence type
        let eei_type = match persistent_eei.persistence_type {
            PersistenceType::MetSocIntelligence => EEIType::Environmental,
            PersistenceType::HashMaskGlobal => EEIType::TechnicalSignature,
            PersistenceType::ContinuousHunter => EEIType::ActiveThreat,
            PersistenceType::ChildProtection => EEIType::LegalProtection,
            PersistenceType::NationalSecurity => EEIType::StrategicIntel,
            PersistenceType::StrategicThreat => EEIType::StrategicIntel,
            PersistenceType::InfrastructureWatch => EEIType::Infrastructure,
            PersistenceType::ComplianceWatch => EEIType::Compliance,
        };

        Some(EEI {
            id: persistent_eei.id,
            eei_type,
            description: persistent_eei.question.clone(),
            priority: match persistent_eei.priority {
                PersistentPriority::Critical => 1.0,
                PersistentPriority::High => 0.8,
                PersistentPriority::Medium => 0.6,
                PersistentPriority::Low => 0.4,
                PersistentPriority::Research => 0.2,
            },
            ltov,
            collection_requirements: vec![persistent_eei.collection_method.clone()],
            intelligence_gaps: vec![],
            created_at: std::time::Instant::now(),
            last_updated: std::time::Instant::now(),
        })
    }

    /// Get all persistent EEIs for a given specialization
    pub fn get_persistent_eeis_for_specialization(&self, specialization: &EEISpecialization) -> Vec<&PersistentEEI> {
        if let Some(eei_ids) = self.specialization_map.get(specialization) {
            eei_ids.iter()
                .filter_map(|id| self.persistent_eeis.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Create default persistent EEIs for specialized nodes
    pub fn create_default_persistent_eeis() -> Vec<PersistentEEI> {
        vec![
            // MetSoc persistent intelligence
            PersistentEEI {
                id: Uuid::new_v4(),
                eei_id: "METSOC-001".to_string(),
                question: "What environmental and meteorological conditions affect threat operations?".to_string(),
                persistence_type: PersistenceType::MetSocIntelligence,
                specialization: EEISpecialization::MetSoc,
                priority: PersistentPriority::Medium,
                collection_method: "Environmental monitoring and weather intelligence".to_string(),
                timeline: PersistentTimeline {
                    min_retention: Duration::from_secs(365 * 24 * 60 * 60), // 1 year
                    max_retention: Some(Duration::from_secs(365 * 24 * 60 * 60 * 5)), // 5 years
                    update_frequency: Duration::from_secs(6 * 60 * 60), // Every 6 hours
                    review_cycle: Duration::from_secs(30 * 24 * 60 * 60), // Monthly
                },
                global_scope: true,
                retention_policy: RetentionPolicy {
                    base_retention: Duration::from_secs(365 * 24 * 60 * 60 * 5),
                    decay_type: DecayType::SlowLinear,
                    classification_multiplier: 1.0,
                    archive_threshold: Some(Duration::from_secs(365 * 24 * 60 * 60 * 2)),
                },
            },

            // Hash mask global intelligence
            PersistentEEI {
                id: Uuid::new_v4(),
                eei_id: "HASHMASK-GLOBAL-001".to_string(),
                question: "What are the global hash signatures and patterns for known threats?".to_string(),
                persistence_type: PersistenceType::HashMaskGlobal,
                specialization: EEISpecialization::HashMaskGlobal,
                priority: PersistentPriority::High,
                collection_method: "Global hash signature analysis and pattern recognition".to_string(),
                timeline: PersistentTimeline {
                    min_retention: Duration::from_secs(365 * 24 * 60 * 60 * 5), // 5 years
                    max_retention: None, // Indefinite
                    update_frequency: Duration::from_secs(60 * 60), // Every hour
                    review_cycle: Duration::from_secs(7 * 24 * 60 * 60), // Weekly
                },
                global_scope: true,
                retention_policy: RetentionPolicy {
                    base_retention: Duration::from_secs(365 * 24 * 60 * 60 * 10),
                    decay_type: DecayType::None,
                    classification_multiplier: 1.0,
                    archive_threshold: Some(Duration::from_secs(365 * 24 * 60 * 60 * 3)),
                },
            },

            // Child protection persistent monitoring
            PersistentEEI {
                id: Uuid::new_v4(),
                eei_id: "CHILDPROT-001".to_string(),
                question: "What indicators suggest child exploitation or abuse activities?".to_string(),
                persistence_type: PersistenceType::ChildProtection,
                specialization: EEISpecialization::ChildProtection,
                priority: PersistentPriority::Critical,
                collection_method: "Multi-source intelligence fusion with privacy protection".to_string(),
                timeline: PersistentTimeline {
                    min_retention: Duration::from_secs(u64::MAX), // Permanent
                    max_retention: None, // Indefinite
                    update_frequency: Duration::from_secs(15 * 60), // Every 15 minutes
                    review_cycle: Duration::from_secs(24 * 60 * 60), // Daily
                },
                global_scope: true,
                retention_policy: RetentionPolicy {
                    base_retention: Duration::from_secs(u64::MAX),
                    decay_type: DecayType::None,
                    classification_multiplier: 2.0,
                    archive_threshold: None,
                },
            },

            // National security persistent tracking
            PersistentEEI {
                id: Uuid::new_v4(),
                eei_id: "NATSEC-001".to_string(),
                question: "What activities pose national security implications or risks?".to_string(),
                persistence_type: PersistenceType::NationalSecurity,
                specialization: EEISpecialization::NationalSecurity,
                priority: PersistentPriority::Critical,
                collection_method: "Strategic intelligence collection and analysis".to_string(),
                timeline: PersistentTimeline {
                    min_retention: Duration::from_secs(365 * 24 * 60 * 60 * 10), // 10 years
                    max_retention: Some(Duration::from_secs(365 * 24 * 60 * 60 * 25)), // 25 years
                    update_frequency: Duration::from_secs(30 * 60), // Every 30 minutes
                    review_cycle: Duration::from_secs(7 * 24 * 60 * 60), // Weekly
                },
                global_scope: true,
                retention_policy: RetentionPolicy {
                    base_retention: Duration::from_secs(365 * 24 * 60 * 60 * 25),
                    decay_type: DecayType::LongExponential,
                    classification_multiplier: 3.0,
                    archive_threshold: Some(Duration::from_secs(365 * 24 * 60 * 60 * 7)),
                },
            },

            // Continuous hunter operations
            PersistentEEI {
                id: Uuid::new_v4(),
                eei_id: "CONTHUNT-001".to_string(),
                question: "What persistent threats require continuous monitoring and hunting?".to_string(),
                persistence_type: PersistenceType::ContinuousHunter,
                specialization: EEISpecialization::ContinuousHunter,
                priority: PersistentPriority::High,
                collection_method: "Continuous threat hunting and behavioral analysis".to_string(),
                timeline: PersistentTimeline {
                    min_retention: Duration::from_secs(365 * 24 * 60 * 60), // 1 year
                    max_retention: Some(Duration::from_secs(365 * 24 * 60 * 60 * 3)), // 3 years
                    update_frequency: Duration::from_secs(5 * 60), // Every 5 minutes
                    review_cycle: Duration::from_secs(24 * 60 * 60), // Daily
                },
                global_scope: true,
                retention_policy: RetentionPolicy {
                    base_retention: Duration::from_secs(365 * 24 * 60 * 60 * 3),
                    decay_type: DecayType::SlowLinear,
                    classification_multiplier: 1.5,
                    archive_threshold: Some(Duration::from_secs(365 * 24 * 60 * 60)),
                },
            },
        ]
    }
}

impl Default for PersistentEEIManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait for DistributedEEINode to support persistent EEIs
pub trait PersistentEEIExtension {
    /// Check if an EEI should be treated as persistent based on node specialization
    fn is_persistent_eei(&self, eei: &EEI) -> impl std::future::Future<Output = bool> + Send;

    /// Initialize persistent EEIs for this node's specializations
    fn initialize_persistent_eeis(&self) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
}

impl PersistentEEIExtension for DistributedEEINode {
    async fn is_persistent_eei(&self, eei: &EEI) -> bool {
        // Check if this node handles persistent EEI types
        for specialization in &self.specializations {
            match specialization {
                EEISpecialization::MetSoc |
                EEISpecialization::HashMaskGlobal |
                EEISpecialization::ChildProtection |
                EEISpecialization::NationalSecurity |
                EEISpecialization::ContinuousHunter => {
                    // These specializations handle persistent EEIs
                    return true;
                },
                _ => continue,
            }
        }

        // Check EEI type for persistence indicators
        matches!(eei.eei_type,
            EEIType::StrategicIntel |
            EEIType::LegalProtection |
            EEIType::Infrastructure |
            EEIType::Environmental |
            EEIType::TechnicalSignature
        )
    }

    async fn initialize_persistent_eeis(&self) -> Result<(), Box<dyn std::error::Error>> {
        let persistent_manager = PersistentEEIManager::new();
        let default_persistent_eeis = PersistentEEIManager::create_default_persistent_eeis();

        // Filter EEIs based on this node's specializations
        for persistent_eei in default_persistent_eeis {
            if self.specializations.contains(&persistent_eei.specialization) {
                if let Some(operational_eei) = persistent_manager.to_operational_eei(persistent_eei.id) {
                    self.add_eei(operational_eei).await?;
                    info!("Initialized persistent EEI {} for specialization {:?}",
                          persistent_eei.eei_id, persistent_eei.specialization);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persistent_eei_creation() {
        let manager = PersistentEEIManager::new();
        assert!(manager.global_retention_policies.contains_key(&PersistenceType::MetSocIntelligence));
        assert!(manager.global_retention_policies.contains_key(&PersistenceType::HashMaskGlobal));
        assert!(manager.global_retention_policies.contains_key(&PersistenceType::ChildProtection));
    }

    #[test]
    fn test_default_persistent_eeis() {
        let default_eeis = PersistentEEIManager::create_default_persistent_eeis();
        assert!(!default_eeis.is_empty());

        // Verify we have the major persistent types
        let types: Vec<_> = default_eeis.iter().map(|e| &e.persistence_type).collect();
        assert!(types.contains(&&PersistenceType::MetSocIntelligence));
        assert!(types.contains(&&PersistenceType::HashMaskGlobal));
        assert!(types.contains(&&PersistenceType::ChildProtection));
        assert!(types.contains(&&PersistenceType::NationalSecurity));
    }

    #[test]
    fn test_retention_policy_differences() {
        let manager = PersistentEEIManager::new();

        let child_protection = manager.global_retention_policies.get(&PersistenceType::ChildProtection).unwrap();
        let hash_mask = manager.global_retention_policies.get(&PersistenceType::HashMaskGlobal).unwrap();

        // Child protection should have no decay and permanent retention
        assert_eq!(child_protection.decay_type, DecayType::None);
        assert_eq!(child_protection.base_retention, Duration::from_secs(u64::MAX));

        // Hash mask should also have no decay but with archiving
        assert_eq!(hash_mask.decay_type, DecayType::None);
        assert!(hash_mask.archive_threshold.is_some());
    }
}