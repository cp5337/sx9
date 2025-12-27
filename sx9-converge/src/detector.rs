//! CONVERGE Detector
//!
//! RFC Compliance:
//! - RFC-9021 Graph Convergence Theory
//! - RFC-9024 Dual Convergence Formula
//! - RFC-93X1 CONVERGE Core
//!
//! Deterministic detection of distributed action-set convergence.
//! Emits ConvergeSignal into GLAF.
//!
//! Dual-path architecture:
//! - Legion (H2): Hot path operational matching
//! - apecs (H1): Async semantic reasoning arbiter

use serde::{Deserialize, Serialize};
use sx9_foundation_core::data::{DateTime, Utc, Uuid};
use std::collections::HashMap;

use crate::model::{
    calculate_convergence, ActionEvent, BookSide, ConvergeEntity, ConvergeError,
    ConvergenceMethod, ConvergenceResult, ConvergeSignal, H1Semantic, H2Operational,
    HD4Phase, TacticalProfile, UnicodeClass,
};
use crate::window::{EventWindow, WindowSet};

/// Convergence threshold (RFC-9024)
pub const CONVERGENCE_THRESHOLD: f64 = 0.75;

/// Detector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectorConfig {
    /// Convergence threshold (0.0 - 1.0)
    pub threshold: f64,
    /// Enable dual-path (Legion + apecs)
    pub dual_path: bool,
    /// Enable cross-feed learning
    pub cross_feed: bool,
    /// Maximum latency budget in microseconds
    pub max_latency_us: u64,
}

impl Default for DetectorConfig {
    fn default() -> Self {
        Self {
            threshold: CONVERGENCE_THRESHOLD,
            dual_path: true,
            cross_feed: true,
            max_latency_us: 100, // <100μs per RFC-9109
        }
    }
}

/// Convergence detector state
pub struct ConvergeDetector {
    /// Configuration
    config: DetectorConfig,
    /// Event windows by source
    windows: WindowSet,
    /// Entity registry (by trivariate hash)
    entities: HashMap<String, ConvergeEntity>,
    /// Pending signals to emit
    pending_signals: Vec<ConvergeSignal>,
    /// Current HD4 phase
    hd4_phase: HD4Phase,
    /// Detection statistics
    stats: DetectorStats,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DetectorStats {
    pub events_processed: u64,
    pub convergences_detected: u64,
    pub signals_emitted: u64,
    pub cross_feed_updates: u64,
    pub threshold_misses: u64,
}

impl Default for ConvergeDetector {
    fn default() -> Self {
        Self::new(DetectorConfig::default())
    }
}

impl ConvergeDetector {
    pub fn new(config: DetectorConfig) -> Self {
        Self {
            config,
            windows: WindowSet::new(),
            entities: HashMap::new(),
            pending_signals: Vec::new(),
            hd4_phase: HD4Phase::Detect,
            stats: DetectorStats::default(),
        }
    }

    /// Process incoming action event
    pub fn process_event(&mut self, event: ActionEvent) -> Result<(), ConvergeError> {
        self.stats.events_processed += 1;

        // Route to appropriate window
        self.windows.route_event(event.clone())?;

        // Check for convergence opportunities
        if let Some(entity_id) = event.entity_id {
            self.check_entity_convergence(entity_id)?;
        }

        // Cross-feed if enabled
        if self.config.cross_feed {
            self.process_cross_feed(&event);
        }

        Ok(())
    }

    /// Check convergence for a specific entity
    fn check_entity_convergence(&mut self, entity_id: Uuid) -> Result<(), ConvergeError> {
        // Gather events from all windows for this entity
        let blue_events = self.windows.blue.drain_by_entity(entity_id);
        let red_events = self.windows.red.drain_by_entity(entity_id);

        if blue_events.is_empty() && red_events.is_empty() {
            return Ok(());
        }

        // Calculate aggregate tactical profiles
        let blue_tactical = Self::aggregate_events(&blue_events);
        let red_tactical = Self::aggregate_events(&red_events);

        // H1 = semantic (blue/defender perspective)
        // H2 = operational (red/attacker perspective)
        let h1 = blue_tactical.convergence_simple();
        let h2 = red_tactical.convergence_simple();

        let result = calculate_convergence(h1, h2, self.config.threshold);

        if result.is_converged {
            self.stats.convergences_detected += 1;
            self.emit_signal(entity_id, result, &blue_events, &red_events);
        } else {
            self.stats.threshold_misses += 1;
        }

        Ok(())
    }

    /// Aggregate tactical profile from events
    fn aggregate_events(events: &[ActionEvent]) -> TacticalProfile {
        if events.is_empty() {
            return TacticalProfile::default();
        }

        let count = events.len() as f64;
        let (sum_p, sum_t, sum_h) = events.iter().fold(
            (0.0, 0.0, 0.0),
            |(p, t, h), e| (p + e.tactical.p, t + e.tactical.t, h + e.tactical.h),
        );

        TacticalProfile {
            p: sum_p / count,
            t: sum_t / count,
            h: sum_h / count,
        }
    }

    /// Emit convergence signal to GLAF
    fn emit_signal(
        &mut self,
        entity_id: Uuid,
        result: ConvergenceResult,
        blue_events: &[ActionEvent],
        red_events: &[ActionEvent],
    ) {
        let trigger_events: Vec<Uuid> = blue_events
            .iter()
            .chain(red_events.iter())
            .map(|e| e.id)
            .collect();

        let signal = ConvergeSignal {
            id: Uuid::new_v4(),
            entity_ids: vec![entity_id],
            score: match result.recommendation {
                ConvergenceMethod::Weighted => result.weighted,
                _ => result.simple,
            },
            method: result.recommendation,
            timestamp: Utc::now(),
            hd4_phase: self.hd4_phase,
            trigger_events,
        };

        self.pending_signals.push(signal);
        self.stats.signals_emitted += 1;
    }

    /// Process cross-feed intelligence (RFC-9109 Section 6)
    fn process_cross_feed(&mut self, event: &ActionEvent) {
        // Blue fills → Red intel (blocked attacks inform red team)
        // Red fills → Blue learning (attacks inform defense)
        match event.source.as_str() {
            "wazuh" | "ossec" | "blue" => {
                // Defense action - could inform red team about what's blocked
                self.stats.cross_feed_updates += 1;
            }
            "kali" | "teth" | "red" => {
                // Attack action - informs defense about new techniques
                self.stats.cross_feed_updates += 1;
            }
            _ => {}
        }
    }

    /// Set HD4 phase (market state)
    pub fn set_hd4_phase(&mut self, phase: HD4Phase) {
        self.hd4_phase = phase;
        self.windows.set_hd4_phase(phase);
    }

    /// Get pending signals and clear buffer
    pub fn drain_signals(&mut self) -> Vec<ConvergeSignal> {
        std::mem::take(&mut self.pending_signals)
    }

    /// Get statistics
    pub fn stats(&self) -> &DetectorStats {
        &self.stats
    }

    /// Register entity for tracking
    pub fn register_entity(&mut self, entity: ConvergeEntity) {
        self.entities.insert(entity.h2.trivariate_hash.clone(), entity);
    }

    /// Get entity by trivariate hash
    pub fn get_entity(&self, hash: &str) -> Option<&ConvergeEntity> {
        self.entities.get(hash)
    }

    /// Create entity from H1 semantic data
    pub fn create_entity_from_h1(&self, h1: H1Semantic) -> ConvergeEntity {
        let unicode_class = UnicodeClass::from_entity_type(&h1.entity_type);
        let now = Utc::now();
        let timestamp_ns = now.timestamp_nanos_opt().unwrap_or(0) as u64;

        // Generate placeholder trivariate hash (should use foundation-core engine)
        let trivariate_hash = format!(
            "{:016X}{:016X}{:016X}",
            timestamp_ns,
            h1.id.len() as u64,
            h1.confidence as u64 * 1000
        );

        let h2 = H2Operational {
            trivariate_hash,
            unicode_address: unicode_class.allocate_address(0),
            unicode_class,
            hd4_phase: self.hd4_phase,
            tactical: TacticalProfile {
                p: h1.confidence,
                t: 0.5,
                h: h1.confidence,
            },
            confidence: h1.confidence,
            latency_budget_us: self.config.max_latency_us,
            priority: 2, // Default HIGH
            ttl_us: 1_000_000, // 1 second default
            timestamp_ns,
            tool_id: None,
            book_side: Some(BookSide::Blue), // Default to defender
        };

        ConvergeEntity::new(h1, h2)
    }
}

/// HFT Order Book integration (RFC-9109)
pub mod order_book {
    use super::*;

    /// Order entry for HFT matching
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Order {
        pub order_id: Uuid,
        pub side: BookSide,
        pub price: u8,           // 1-4, lower = higher priority
        pub time_ns: u64,
        pub size: u32,
        pub hd4_phase: HD4Phase,
        pub tool_id: String,
        pub ttl_us: u64,
    }

    /// Fill result from matching
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Fill {
        pub order_id: Uuid,
        pub side: BookSide,
        pub fill_size: u32,
        pub spread_us: i64,      // Negative = winning
        pub timestamp_ns: u64,
    }

    /// Priority levels (price in order book terms)
    pub const PRIORITY_CRITICAL: u8 = 1; // Nation-state, APT
    pub const PRIORITY_HIGH: u8 = 2;     // Sophisticated attack
    pub const PRIORITY_MEDIUM: u8 = 3;   // Standard threat
    pub const PRIORITY_LOW: u8 = 4;      // Script kiddie

    impl Order {
        /// Create order from action event
        pub fn from_event(event: &ActionEvent, side: BookSide, tool_id: String) -> Self {
            // Map tactical profile to priority
            let priority = if event.tactical.p >= 0.9 {
                PRIORITY_CRITICAL
            } else if event.tactical.p >= 0.7 {
                PRIORITY_HIGH
            } else if event.tactical.p >= 0.4 {
                PRIORITY_MEDIUM
            } else {
                PRIORITY_LOW
            };

            Self {
                order_id: event.id,
                side,
                price: priority,
                time_ns: event.timestamp_ns,
                size: 1, // Single event = size 1
                hd4_phase: HD4Phase::Detect,
                tool_id,
                ttl_us: 1_000_000, // 1 second default
            }
        }

        /// Check if order is expired
        pub fn is_expired(&self, current_ns: u64) -> bool {
            let elapsed_us = (current_ns.saturating_sub(self.time_ns)) / 1000;
            elapsed_us > self.ttl_us
        }
    }

    /// Calculate spread for order
    pub fn calculate_spread(response_time_us: u64, urgency_us: u64) -> i64 {
        (response_time_us as i64) - (urgency_us as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sx9_foundation_core::data::serde_json::json;

    fn make_event(source: &str, entity_id: Option<Uuid>) -> ActionEvent {
        ActionEvent {
            id: Uuid::new_v4(),
            source: source.to_string(),
            event_type: "test".to_string(),
            entity_id,
            timestamp_ns: Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            payload: json!({}),
            tactical: TacticalProfile { p: 0.8, t: 0.7, h: 0.75 },
        }
    }

    #[test]
    fn test_detector_process_event() {
        let mut detector = ConvergeDetector::default();
        let event = make_event("wazuh", None);
        assert!(detector.process_event(event).is_ok());
        assert_eq!(detector.stats.events_processed, 1);
    }

    #[test]
    fn test_detector_cross_feed() {
        let mut detector = ConvergeDetector::default();
        detector.process_event(make_event("wazuh", None)).unwrap();
        detector.process_event(make_event("kali", None)).unwrap();
        assert_eq!(detector.stats.cross_feed_updates, 2);
    }

    #[test]
    fn test_order_priority_mapping() {
        use order_book::*;

        let mut event = make_event("test", None);
        event.tactical.p = 0.95;
        let order = Order::from_event(&event, BookSide::Blue, "daemon-1".to_string());
        assert_eq!(order.price, PRIORITY_CRITICAL);

        event.tactical.p = 0.3;
        let order = Order::from_event(&event, BookSide::Red, "kali-tool".to_string());
        assert_eq!(order.price, PRIORITY_LOW);
    }

    #[test]
    fn test_spread_calculation() {
        use order_book::calculate_spread;

        // Response faster than needed = winning (negative spread)
        assert!(calculate_spread(50, 100) < 0);

        // Response lagging = slipping (positive spread)
        assert!(calculate_spread(150, 100) > 0);

        // Exactly meeting SLA
        assert_eq!(calculate_spread(100, 100), 0);
    }
}
