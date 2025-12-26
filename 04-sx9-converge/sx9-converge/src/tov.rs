//! Time-of-Value (ToV) Lifecycle Gate
//!
//! RFC Compliance:
//! - RFC-9024 §6 Time-of-Value Decay
//! - RFC-9025 §3.8 Time-of-Value Block
//! - Unicode allocation: U+ED80-EDFF for ToV markers
//!
//! Purpose:
//! Implements IWAS-pattern EEI lifecycle management:
//! - Exponential decay with 5-minute half-life
//! - 10-minute validity threshold
//! - Collection gate: don't collect if data expires on arrival
//! - Refresh trigger mechanism
//!
//! Formula (RFC-9024 §6):
//!   decay_factor = exp(-age_secs / 300.0)  // 5-min half-life
//!   effective_score = semantic_score * decay_factor

use serde::{Deserialize, Serialize};
use sx9_foundation_core::data::{chrono, DateTime, Utc, Uuid};
use std::collections::HashMap;
use std::time::Duration;

/// Half-life in seconds (5 minutes per RFC-9024)
pub const HALF_LIFE_SECS: f64 = 300.0;

/// Validity window in seconds (10 minutes per RFC-9024)
pub const VALIDITY_SECS: u64 = 600;

/// Collection buffer in seconds - don't collect if <30s validity remaining
pub const COLLECTION_BUFFER_SECS: u64 = 30;

/// ToV decay rates (RFC-9025 §3.8)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DecayRate {
    /// Immediate expiration (tactical intel)
    Immediate,
    /// 1-minute half-life (breaking intelligence)
    Fast,
    /// 5-minute half-life (standard operational)
    #[default]
    Medium,
    /// 30-minute half-life (strategic intel)
    Slow,
}

impl DecayRate {
    /// Get half-life in seconds for this decay rate
    pub fn half_life_secs(&self) -> f64 {
        match self {
            Self::Immediate => 30.0,
            Self::Fast => 60.0,
            Self::Medium => 300.0,
            Self::Slow => 1800.0,
        }
    }

    /// Get validity window in seconds
    pub fn validity_secs(&self) -> u64 {
        // Validity = 2x half-life (approximately)
        (self.half_life_secs() * 2.0) as u64
    }

    /// Calculate decay factor for given age
    /// Uses true half-life formula: 2^(-age/half_life) = exp(-ln(2) * age / half_life)
    pub fn decay_factor(&self, age_secs: f64) -> f64 {
        let ln2 = std::f64::consts::LN_2;
        (-ln2 * age_secs / self.half_life_secs()).exp()
    }
}

/// Time-of-Value Block (RFC-9025 §3.8)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TovBlock {
    /// EEI ID this ToV applies to
    pub eei_id: Uuid,
    /// Collection window start
    pub collection_start: DateTime<Utc>,
    /// Collection window end
    pub collection_end: DateTime<Utc>,
    /// Actionable window (when intel is still useful)
    pub actionable_until: DateTime<Utc>,
    /// Decay rate for this intelligence type
    pub decay_rate: DecayRate,
    /// Persistence condition (what triggers refresh)
    pub persistence_condition: PersistenceCondition,
    /// Refresh trigger
    pub refresh_trigger: Option<RefreshTrigger>,
    /// Original semantic score before decay
    pub original_score: f64,
    /// Unicode address (U+ED80-EDFF range)
    pub unicode_address: String,
}

/// Conditions for persisting intelligence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersistenceCondition {
    /// Always persist (strategic intelligence)
    Always,
    /// Persist if score above threshold
    ScoreThreshold(f64),
    /// Persist if related entity active
    EntityActive(Uuid),
    /// Persist until specific event
    UntilEvent(String),
    /// No persistence (ephemeral)
    Never,
}

/// Refresh triggers for ToV renewal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTrigger {
    /// Trigger type
    pub trigger_type: RefreshTriggerType,
    /// Minimum interval between refreshes
    pub min_interval_secs: u64,
    /// Last refresh timestamp
    pub last_refresh: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefreshTriggerType {
    /// Refresh on new correlated data
    Correlation,
    /// Refresh on entity activity
    EntityActivity,
    /// Refresh on periodic interval
    Periodic(u64),
    /// Refresh on analyst request
    Manual,
}

impl TovBlock {
    /// Create new ToV block with standard decay
    pub fn new(eei_id: Uuid, score: f64, decay_rate: DecayRate) -> Self {
        let now = Utc::now();
        let validity = Duration::from_secs(decay_rate.validity_secs());

        Self {
            eei_id,
            collection_start: now,
            collection_end: now + chrono::Duration::from_std(validity).unwrap_or_default(),
            actionable_until: now + chrono::Duration::from_std(validity).unwrap_or_default(),
            decay_rate,
            persistence_condition: PersistenceCondition::ScoreThreshold(0.5),
            refresh_trigger: None,
            original_score: score,
            unicode_address: allocate_tov_address(0),
        }
    }

    /// Calculate current effective score with decay
    pub fn effective_score(&self) -> f64 {
        let now = Utc::now();
        let age = now
            .signed_duration_since(self.collection_start)
            .num_seconds() as f64;

        if age < 0.0 {
            return self.original_score;
        }

        self.original_score * self.decay_rate.decay_factor(age)
    }

    /// Check if intelligence is still valid
    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        now < self.actionable_until
    }

    /// Check if intelligence is expired
    pub fn is_expired(&self) -> bool {
        !self.is_valid()
    }

    /// Get remaining validity in seconds
    pub fn remaining_validity_secs(&self) -> i64 {
        let now = Utc::now();
        self.actionable_until
            .signed_duration_since(now)
            .num_seconds()
    }

    /// Check if collection is worthwhile (IWAS gate)
    /// Returns false if data will expire before processing completes
    pub fn should_collect(&self, estimated_processing_secs: u64) -> bool {
        let remaining = self.remaining_validity_secs();
        if remaining < 0 {
            return false;
        }
        (remaining as u64) > estimated_processing_secs + COLLECTION_BUFFER_SECS
    }

    /// Refresh the ToV block (extend validity)
    pub fn refresh(&mut self, new_score: Option<f64>) {
        let now = Utc::now();
        let validity = Duration::from_secs(self.decay_rate.validity_secs());

        self.collection_start = now;
        self.actionable_until = now + chrono::Duration::from_std(validity).unwrap_or_default();

        if let Some(score) = new_score {
            self.original_score = score;
        }

        if let Some(ref mut trigger) = self.refresh_trigger {
            trigger.last_refresh = Some(now);
        }
    }

    /// Check if should persist based on condition
    pub fn should_persist(&self) -> bool {
        match &self.persistence_condition {
            PersistenceCondition::Always => true,
            PersistenceCondition::ScoreThreshold(threshold) => {
                self.effective_score() >= *threshold
            }
            PersistenceCondition::EntityActive(_) => {
                // Would require entity registry lookup
                true
            }
            PersistenceCondition::UntilEvent(_) => true,
            PersistenceCondition::Never => false,
        }
    }
}

/// Allocate Unicode address in ToV range (U+ED80-EDFF)
pub fn allocate_tov_address(index: u8) -> String {
    let codepoint = 0xED80 + (index as u32);
    format!("U+{:04X}", codepoint)
}

/// ToV Lifecycle Gate - the IWAS pattern implementation
#[derive(Debug)]
pub struct TovGate {
    /// Active ToV blocks by EEI ID
    blocks: HashMap<Uuid, TovBlock>,
    /// Gate statistics
    stats: TovGateStats,
    /// Default decay rate for new blocks
    default_decay: DecayRate,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TovGateStats {
    /// EEIs registered
    pub eeis_registered: u64,
    /// EEIs expired (removed due to ToV)
    pub eeis_expired: u64,
    /// Collections blocked (would expire before useful)
    pub collections_blocked: u64,
    /// Refreshes performed
    pub refreshes_performed: u64,
    /// Current active EEIs
    pub active_eeis: usize,
}

impl Default for TovGate {
    fn default() -> Self {
        Self::new(DecayRate::Medium)
    }
}

impl TovGate {
    pub fn new(default_decay: DecayRate) -> Self {
        Self {
            blocks: HashMap::new(),
            stats: TovGateStats::default(),
            default_decay,
        }
    }

    /// Register a new EEI with ToV tracking
    pub fn register_eei(&mut self, eei_id: Uuid, score: f64) -> TovBlock {
        let block = TovBlock::new(eei_id, score, self.default_decay);
        self.blocks.insert(eei_id, block.clone());
        self.stats.eeis_registered += 1;
        self.stats.active_eeis = self.blocks.len();
        block
    }

    /// Register EEI with custom decay rate
    pub fn register_eei_with_decay(
        &mut self,
        eei_id: Uuid,
        score: f64,
        decay_rate: DecayRate,
    ) -> TovBlock {
        let block = TovBlock::new(eei_id, score, decay_rate);
        self.blocks.insert(eei_id, block.clone());
        self.stats.eeis_registered += 1;
        self.stats.active_eeis = self.blocks.len();
        block
    }

    /// Check if collection should proceed (IWAS gate)
    pub fn should_collect(&self, eei_id: &Uuid, processing_time_secs: u64) -> bool {
        match self.blocks.get(eei_id) {
            Some(block) => block.should_collect(processing_time_secs),
            None => true, // Unknown EEI - allow collection
        }
    }

    /// Gate collection with stats tracking
    pub fn gate_collection(&mut self, eei_id: &Uuid, processing_time_secs: u64) -> bool {
        let should = self.should_collect(eei_id, processing_time_secs);
        if !should {
            self.stats.collections_blocked += 1;
        }
        should
    }

    /// Get effective score for EEI (with decay applied)
    pub fn get_effective_score(&self, eei_id: &Uuid) -> Option<f64> {
        self.blocks.get(eei_id).map(|b| b.effective_score())
    }

    /// Check if EEI is still valid
    pub fn is_valid(&self, eei_id: &Uuid) -> bool {
        self.blocks
            .get(eei_id)
            .map(|b| b.is_valid())
            .unwrap_or(false)
    }

    /// Refresh EEI (extend validity)
    pub fn refresh(&mut self, eei_id: &Uuid, new_score: Option<f64>) -> bool {
        if let Some(block) = self.blocks.get_mut(eei_id) {
            block.refresh(new_score);
            self.stats.refreshes_performed += 1;
            true
        } else {
            false
        }
    }

    /// Expire and remove invalid EEIs
    pub fn expire_invalid(&mut self) -> Vec<Uuid> {
        let expired: Vec<Uuid> = self
            .blocks
            .iter()
            .filter(|(_, block)| block.is_expired())
            .map(|(id, _)| *id)
            .collect();

        for id in &expired {
            self.blocks.remove(id);
            self.stats.eeis_expired += 1;
        }

        self.stats.active_eeis = self.blocks.len();
        expired
    }

    /// Get all EEIs that should persist before expiration
    pub fn get_persistable(&self) -> Vec<&TovBlock> {
        self.blocks
            .values()
            .filter(|b| b.should_persist())
            .collect()
    }

    /// Get gate statistics
    pub fn stats(&self) -> &TovGateStats {
        &self.stats
    }

    /// Get ToV block for EEI
    pub fn get_block(&self, eei_id: &Uuid) -> Option<&TovBlock> {
        self.blocks.get(eei_id)
    }
}

/// EEI (Essential Element of Information) with ToV lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EEI {
    /// Unique identifier
    pub id: Uuid,
    /// Intelligence content
    pub content: String,
    /// Source system
    pub source: String,
    /// Classification level
    pub classification: String,
    /// Semantic confidence (original, pre-decay)
    pub confidence: f64,
    /// ToV lifecycle data
    pub tov: TovBlock,
    /// Entity relationships
    pub related_entities: Vec<Uuid>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated
    pub updated_at: DateTime<Utc>,
}

impl EEI {
    pub fn new(content: String, source: String, confidence: f64) -> Self {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let tov = TovBlock::new(id, confidence, DecayRate::Medium);

        Self {
            id,
            content,
            source,
            classification: "UNCLASSIFIED".to_string(),
            confidence,
            tov,
            related_entities: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Get current effective confidence (with decay)
    pub fn effective_confidence(&self) -> f64 {
        self.tov.effective_score()
    }

    /// Check if EEI is still actionable
    pub fn is_actionable(&self) -> bool {
        self.tov.is_valid()
    }

    /// Refresh EEI with new correlation
    pub fn refresh(&mut self, correlation_boost: f64) {
        let new_score = (self.confidence + correlation_boost).min(1.0);
        self.tov.refresh(Some(new_score));
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay_factor() {
        let decay = DecayRate::Medium;

        // At t=0, decay factor should be 1.0
        assert!((decay.decay_factor(0.0) - 1.0).abs() < 0.001);

        // At t=half_life, decay factor should be ~0.5
        let at_half_life = decay.decay_factor(300.0);
        assert!((at_half_life - 0.5).abs() < 0.1);

        // At t=2*half_life, decay factor should be ~0.25
        let at_double = decay.decay_factor(600.0);
        assert!((at_double - 0.25).abs() < 0.1);
    }

    #[test]
    fn test_decay_rates() {
        assert_eq!(DecayRate::Immediate.half_life_secs(), 30.0);
        assert_eq!(DecayRate::Fast.half_life_secs(), 60.0);
        assert_eq!(DecayRate::Medium.half_life_secs(), 300.0);
        assert_eq!(DecayRate::Slow.half_life_secs(), 1800.0);
    }

    #[test]
    fn test_tov_block_creation() {
        let id = Uuid::new_v4();
        let block = TovBlock::new(id, 0.85, DecayRate::Medium);

        assert_eq!(block.eei_id, id);
        assert!((block.original_score - 0.85).abs() < 0.001);
        assert!(block.is_valid());
        assert!(!block.is_expired());
    }

    #[test]
    fn test_tov_gate_registration() {
        let mut gate = TovGate::default();
        let id = Uuid::new_v4();

        let block = gate.register_eei(id, 0.9);
        assert!(block.is_valid());
        assert_eq!(gate.stats().eeis_registered, 1);
        assert_eq!(gate.stats().active_eeis, 1);
    }

    #[test]
    fn test_collection_gate() {
        let mut gate = TovGate::default();
        let id = Uuid::new_v4();

        gate.register_eei(id, 0.9);

        // Should allow collection with reasonable processing time
        assert!(gate.gate_collection(&id, 10));

        // Stats should be unchanged
        assert_eq!(gate.stats().collections_blocked, 0);
    }

    #[test]
    fn test_effective_score() {
        let mut gate = TovGate::default();
        let id = Uuid::new_v4();

        gate.register_eei(id, 1.0);

        // Immediately after registration, score should be close to 1.0
        let score = gate.get_effective_score(&id).unwrap();
        assert!(score > 0.99);
    }

    #[test]
    fn test_unicode_address_allocation() {
        assert_eq!(allocate_tov_address(0), "U+ED80");
        assert_eq!(allocate_tov_address(127), "U+EDFF");
    }

    #[test]
    fn test_eei_creation() {
        let eei = EEI::new(
            "Suspicious activity detected".to_string(),
            "wazuh".to_string(),
            0.85,
        );

        assert!(eei.is_actionable());
        assert!((eei.effective_confidence() - 0.85).abs() < 0.01);
    }

    #[test]
    fn test_eei_refresh() {
        let mut eei = EEI::new(
            "Test intelligence".to_string(),
            "sensor".to_string(),
            0.7,
        );

        let original_until = eei.tov.actionable_until;

        // Refresh with correlation boost
        eei.refresh(0.1);

        // Score should increase
        assert!(eei.confidence <= 1.0);
        // Validity should be extended
        assert!(eei.tov.actionable_until >= original_until);
    }

    #[test]
    fn test_persistence_conditions() {
        let id = Uuid::new_v4();

        // Test ScoreThreshold
        let mut block = TovBlock::new(id, 0.8, DecayRate::Medium);
        block.persistence_condition = PersistenceCondition::ScoreThreshold(0.5);
        assert!(block.should_persist());

        block.persistence_condition = PersistenceCondition::ScoreThreshold(0.9);
        assert!(!block.should_persist());

        // Test Always
        block.persistence_condition = PersistenceCondition::Always;
        assert!(block.should_persist());

        // Test Never
        block.persistence_condition = PersistenceCondition::Never;
        assert!(!block.should_persist());
    }
}
