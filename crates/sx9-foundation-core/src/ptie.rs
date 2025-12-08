//! Proactive Threat Intelligence Engine (PTIE) 2.0
//!
//! Autonomous OODA loop engine for continuous threat intelligence processing
//! Integrated from 7.1 improvements while maintaining 7.0 compatibility

use crate::usim::{UniversalSymbolicInformationMessage, Context};
use crate::context::{ContextualIntelligence, EnvironmentalMask};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc};
use tokio::sync::{mpsc, RwLock};
use tokio::time::{interval, Duration};
use std::sync::Arc;

/// PTIE 2.0 Engine - Autonomous OODA loop for threat intelligence
#[derive(Debug)]
pub struct ProactiveThreatIntelligenceEngine {
    /// Current operational state
    pub state: PTIEState,
    /// OODA loop processor
    pub ooda_processor: OODAProcessor,
    /// Pub/Sub message channels
    pub message_channels: MessageChannels,
    /// Contextual intelligence processor
    pub contextual_intelligence: Arc<RwLock<ContextualIntelligence>>,
    /// EEI (Essential Elements of Information) tracker
    pub eei_tracker: EEITracker,
    /// Configuration parameters
    pub config: PTIEConfig,
}

/// PTIE operational states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PTIEState {
    Initializing,
    Observing,
    Orienting,
    Deciding,
    Acting,
    Standby,
    Error { message: String },
}

/// OODA Loop processor implementation
#[derive(Debug)]
pub struct OODAProcessor {
    /// Current OODA phase
    pub current_phase: OODAPhase,
    /// Processing metrics
    pub metrics: OODAMetrics,
    /// Data fusion engine
    pub fusion_engine: DataFusionEngine,
    /// Decision engine
    pub decision_engine: DecisionEngine,
}

/// OODA Loop phases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OODAPhase {
    Observe,
    Orient,
    Decide,
    Act,
}

/// OODA processing metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OODAMetrics {
    pub cycle_count: u64,
    pub average_cycle_time_ms: f64,
    pub last_cycle_duration_ms: u64,
    pub observations_processed: u64,
    pub decisions_made: u64,
    pub actions_taken: u64,
}

/// Data fusion engine for Orient phase
#[derive(Debug)]
pub struct DataFusionEngine {
    /// Sliding window analysis buffer
    pub sliding_window: VecDeque<USIMObservation>,
    /// GIS context integration
    pub gis_context: HashMap<String, GISContext>,
    /// METOC (Meteorological and Oceanographic) integration
    pub metoc_context: HashMap<String, METOCContext>,
    /// Hash position tails processor
    pub position_tails: HashMap<String, String>,
}

/// Decision engine for Decide phase
#[derive(Debug)]
pub struct DecisionEngine {
    /// Prioritized EEI queue
    pub eei_priority_queue: VecDeque<EEIRequest>,
    /// Unsatisfied EEI tracker
    pub unsatisfied_eeis: HashMap<String, UnsatisfiedEEI>,
    /// Decision matrix
    pub decision_matrix: DecisionMatrix,
}

/// Message channels for Pub/Sub architecture
#[derive(Debug)]
pub struct MessageChannels {
    /// Inbound channels (Observe)
    pub intelligence_findings_rx: mpsc::Receiver<USIMObservation>,
    pub gis_updates_rx: mpsc::Receiver<GISUpdate>,
    pub metoc_alerts_rx: mpsc::Receiver<METOCAlert>,

    /// Outbound channels (Act)
    pub need_to_find_tx: mpsc::Sender<EEIRequest>,
    pub threat_alerts_tx: mpsc::Sender<ThreatAlert>,
    pub context_updates_tx: mpsc::Sender<ContextUpdate>,
}

/// EEI (Essential Elements of Information) tracker
#[derive(Debug)]
pub struct EEITracker {
    /// Active EEI requests
    pub active_eeis: HashMap<String, EEIRequest>,
    /// Satisfied EEI history
    pub satisfied_eeis: VecDeque<SatisfiedEEI>,
    /// EEI satisfaction rate
    pub satisfaction_rate: f64,
}

/// PTIE Configuration
#[derive(Debug, Clone)]
pub struct PTIEConfig {
    /// OODA loop cycle interval (milliseconds)
    pub cycle_interval_ms: u64,
    /// Sliding window size for observations
    pub sliding_window_size: usize,
    /// EEI timeout duration (minutes)
    pub eei_timeout_minutes: u64,
    /// Priority thresholds
    pub priority_thresholds: PriorityThresholds,
    /// Interface configuration
    pub interface_config: InterfaceConfig,
}

/// Priority thresholds for decision making
#[derive(Debug, Clone)]
pub struct PriorityThresholds {
    pub critical: f64,   // 0.9+
    pub high: f64,       // 0.7-0.9
    pub medium: f64,     // 0.4-0.7
    pub low: f64,        // 0.0-0.4
}

/// Interface configuration (GUI + ASCII CLI)
#[derive(Debug, Clone)]
pub struct InterfaceConfig {
    pub enable_gui: bool,
    pub enable_ascii_cli: bool,
    pub contextual_bridge_enabled: bool,
    pub universal_hover_enabled: bool,
}

/// Observation from intelligence sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USIMObservation {
    pub usim: UniversalSymbolicInformationMessage,
    pub source_id: String,
    pub confidence: f64,
    pub timestamp: DateTime<Utc>,
}

/// GIS context data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GISContext {
    pub location_id: String,
    pub coordinates: (f64, f64),
    pub elevation_m: f64,
    pub region_code: String,
    pub terrain_type: String,
}

/// METOC context data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct METOCContext {
    pub location_id: String,
    pub weather_conditions: String,
    pub visibility_km: f64,
    pub wind_speed_kph: f64,
    pub temperature_celsius: f64,
    pub sea_state: Option<u8>, // 0-9 scale
}

/// EEI request for collection missions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EEIRequest {
    pub eei_id: String,
    pub description: String,
    pub priority: f64,
    pub target_context: Context,
    pub collection_method: CollectionMethod,
    pub deadline: DateTime<Utc>,
    pub requestor: String,
}

/// Unsatisfied EEI tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsatisfiedEEI {
    pub eei_request: EEIRequest,
    pub time_unsatisfied_minutes: u64,
    pub escalation_level: u8,
}

/// Satisfied EEI record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatisfiedEEI {
    pub eei_request: EEIRequest,
    pub satisfaction_usim: UniversalSymbolicInformationMessage,
    pub satisfaction_timestamp: DateTime<Utc>,
    pub response_time_minutes: u64,
}

/// Collection methods for EEI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollectionMethod {
    SIGINT,     // Signals Intelligence
    HUMINT,     // Human Intelligence
    GEOINT,     // Geospatial Intelligence
    OSINT,      // Open Source Intelligence
    MASINT,     // Measurement and Signature Intelligence
    CYBINT,     // Cyber Intelligence
}

/// Decision matrix for prioritization
#[derive(Debug, Clone)]
pub struct DecisionMatrix {
    pub threat_weight: f64,
    pub urgency_weight: f64,
    pub context_weight: f64,
    pub confidence_weight: f64,
}

/// Updates from various sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GISUpdate {
    pub location_id: String,
    pub gis_context: GISContext,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct METOCAlert {
    pub alert_id: String,
    pub metoc_context: METOCContext,
    pub alert_type: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAlert {
    pub alert_id: String,
    pub threat_description: String,
    pub threat_level: String,
    pub affected_context: Context,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextUpdate {
    pub update_id: String,
    pub context: Context,
    pub environmental_mask: EnvironmentalMask,
    pub timestamp: DateTime<Utc>,
}

impl ProactiveThreatIntelligenceEngine {
    /// Create new PTIE 2.0 engine
    pub fn new(config: PTIEConfig) -> Self {
        let (intelligence_tx, intelligence_rx) = mpsc::channel(1000);
        let (gis_tx, gis_rx) = mpsc::channel(1000);
        let (metoc_tx, metoc_rx) = mpsc::channel(1000);
        let (need_to_find_tx, _need_to_find_rx) = mpsc::channel(1000);
        let (threat_alerts_tx, _threat_alerts_rx) = mpsc::channel(1000);
        let (context_updates_tx, _context_updates_rx) = mpsc::channel(1000);

        Self {
            state: PTIEState::Initializing,
            ooda_processor: OODAProcessor::new(),
            message_channels: MessageChannels {
                intelligence_findings_rx: intelligence_rx,
                gis_updates_rx: gis_rx,
                metoc_alerts_rx: metoc_rx,
                need_to_find_tx,
                threat_alerts_tx,
                context_updates_tx,
            },
            contextual_intelligence: Arc::new(RwLock::new(ContextualIntelligence::new())),
            eei_tracker: EEITracker::new(),
            config,
        }
    }

    /// Start the autonomous OODA loop
    pub async fn start_ooda_loop(&mut self) {
        let mut cycle_interval = interval(Duration::from_millis(self.config.cycle_interval_ms));

        loop {
            tokio::select! {
                _ = cycle_interval.tick() => {
                    let start_time = std::time::Instant::now();

                    // Execute OODA cycle
                    self.execute_ooda_cycle().await;

                    // Update metrics
                    let cycle_duration = start_time.elapsed().as_millis() as u64;
                    self.ooda_processor.metrics.update_cycle_metrics(cycle_duration);
                }
            }
        }
    }

    /// Execute complete OODA cycle
    async fn execute_ooda_cycle(&mut self) {
        // OBSERVE: Ingest data streams
        self.observe_phase().await;

        // ORIENT: Fuse data with context
        self.orient_phase().await;

        // DECIDE: Identify priority EEIs
        self.decide_phase().await;

        // ACT: Publish collection missions
        self.act_phase().await;
    }

    /// OBSERVE: Ingest intelligence findings, GIS updates, METOC alerts
    async fn observe_phase(&mut self) {
        self.state = PTIEState::Observing;
        self.ooda_processor.current_phase = OODAPhase::Observe;

        // Process intelligence findings
        while let Ok(observation) = self.message_channels.intelligence_findings_rx.try_recv() {
            self.ooda_processor.fusion_engine.add_observation(observation);
            self.ooda_processor.metrics.observations_processed += 1;
        }

        // Process GIS updates
        while let Ok(gis_update) = self.message_channels.gis_updates_rx.try_recv() {
            self.ooda_processor.fusion_engine.update_gis_context(gis_update);
        }

        // Process METOC alerts
        while let Ok(metoc_alert) = self.message_channels.metoc_alerts_rx.try_recv() {
            self.ooda_processor.fusion_engine.update_metoc_context(metoc_alert);
        }
    }

    /// ORIENT: Fuse data with graph, apply GIS/METOC context, run sliding window analysis
    async fn orient_phase(&mut self) {
        self.state = PTIEState::Orienting;
        self.ooda_processor.current_phase = OODAPhase::Orient;

        // Apply contextual intelligence processing
        let mut ci = self.contextual_intelligence.write().await;

        // Process observations through sliding window
        self.ooda_processor.fusion_engine.process_sliding_window(&mut ci).await;

        // Generate hash position tails with environmental context
        self.ooda_processor.fusion_engine.generate_position_tails(&ci);
    }

    /// DECIDE: Identify highest-priority unsatisfied EEIs
    async fn decide_phase(&mut self) {
        self.state = PTIEState::Deciding;
        self.ooda_processor.current_phase = OODAPhase::Decide;

        // Update EEI satisfaction status
        self.eei_tracker.update_satisfaction_status();

        // Prioritize unsatisfied EEIs using decision matrix
        let priority_eeis = self.ooda_processor.decision_engine.prioritize_eeis(&self.eei_tracker);

        // Generate new EEI requests based on current context
        let new_eeis = self.generate_contextual_eeis().await;

        for eei in new_eeis {
            self.eei_tracker.add_eei_request(eei);
        }

        self.ooda_processor.metrics.decisions_made += 1;
    }

    /// ACT: Publish prioritized EEIs to need-to-find topic
    async fn act_phase(&mut self) {
        self.state = PTIEState::Acting;
        self.ooda_processor.current_phase = OODAPhase::Act;

        // Get top priority unsatisfied EEIs
        let priority_eeis = self.eei_tracker.get_priority_eeis(5); // Top 5

        for eei in priority_eeis {
            // Publish to need-to-find topic
            if let Err(e) = self.message_channels.need_to_find_tx.send(eei.clone()).await {
                eprintln!("Failed to publish EEI request: {}", e);
            }
        }

        self.ooda_processor.metrics.actions_taken += 1;
    }

    /// Generate contextual EEIs based on current operating picture
    async fn generate_contextual_eeis(&self) -> Vec<EEIRequest> {
        let mut eeis = Vec::new();

        // Analyze current context for intelligence gaps
        let ci = self.contextual_intelligence.read().await;
        let analysis = ci.generate_analysis_report();

        // Generate EEIs based on context analysis
        // This would be enhanced with machine learning in production
        if analysis.contains("threat_level = \"High\"") {
            eeis.push(EEIRequest {
                eei_id: format!("EEI-THREAT-{}", Utc::now().timestamp()),
                description: "High threat level detected - require additional SIGINT coverage".to_string(),
                priority: 0.9,
                target_context: Context::Logical {
                    system_id: "THREAT_ANALYSIS".to_string(),
                    relative_position: "HIGH_PRIORITY".to_string(),
                },
                collection_method: CollectionMethod::SIGINT,
                deadline: Utc::now() + chrono::Duration::hours(2),
                requestor: "PTIE-2.0".to_string(),
            });
        }

        eeis
    }

    /// Get engine status report
    pub fn get_status_report(&self) -> String {
        format!(
            r#"[ptie_engine]
state = "{:?}"
current_ooda_phase = "{:?}"
cycle_count = {}
average_cycle_time_ms = {:.2}
observations_processed = {}
decisions_made = {}
actions_taken = {}

[eei_tracker]
active_eeis = {}
satisfied_eeis = {}
satisfaction_rate = {:.2}%

[sliding_window]
window_size = {}
current_observations = {}

[contextual_intelligence]
environmental_masks = "integrated"
position_tails = "active"
"#,
            self.state,
            self.ooda_processor.current_phase,
            self.ooda_processor.metrics.cycle_count,
            self.ooda_processor.metrics.average_cycle_time_ms,
            self.ooda_processor.metrics.observations_processed,
            self.ooda_processor.metrics.decisions_made,
            self.ooda_processor.metrics.actions_taken,
            self.eei_tracker.active_eeis.len(),
            self.eei_tracker.satisfied_eeis.len(),
            self.eei_tracker.satisfaction_rate * 100.0,
            self.config.sliding_window_size,
            self.ooda_processor.fusion_engine.sliding_window.len()
        )
    }
}

// Implementation blocks for supporting structures
impl OODAProcessor {
    fn new() -> Self {
        Self {
            current_phase: OODAPhase::Observe,
            metrics: OODAMetrics::new(),
            fusion_engine: DataFusionEngine::new(),
            decision_engine: DecisionEngine::new(),
        }
    }
}

impl OODAMetrics {
    fn new() -> Self {
        Self {
            cycle_count: 0,
            average_cycle_time_ms: 0.0,
            last_cycle_duration_ms: 0,
            observations_processed: 0,
            decisions_made: 0,
            actions_taken: 0,
        }
    }

    fn update_cycle_metrics(&mut self, cycle_duration_ms: u64) {
        self.cycle_count += 1;
        self.last_cycle_duration_ms = cycle_duration_ms;

        // Update rolling average
        self.average_cycle_time_ms =
            (self.average_cycle_time_ms * (self.cycle_count - 1) as f64 + cycle_duration_ms as f64)
            / self.cycle_count as f64;
    }
}

impl DataFusionEngine {
    fn new() -> Self {
        Self {
            sliding_window: VecDeque::new(),
            gis_context: HashMap::new(),
            metoc_context: HashMap::new(),
            position_tails: HashMap::new(),
        }
    }

    fn add_observation(&mut self, observation: USIMObservation) {
        self.sliding_window.push_back(observation);

        // Maintain window size
        if self.sliding_window.len() > 1000 { // Default window size
            self.sliding_window.pop_front();
        }
    }

    fn update_gis_context(&mut self, update: GISUpdate) {
        self.gis_context.insert(update.location_id, update.gis_context);
    }

    fn update_metoc_context(&mut self, alert: METOCAlert) {
        self.metoc_context.insert(alert.alert_id, alert.metoc_context);
    }

    async fn process_sliding_window(&mut self, ci: &mut ContextualIntelligence) {
        // Process observations in sliding window through contextual intelligence
        for observation in &self.sliding_window {
            let position_tails = ci.process_context(&observation.usim.context);
            self.position_tails.extend(position_tails);
        }
    }

    fn generate_position_tails(&mut self, ci: &ContextualIntelligence) {
        // Generate environmental position tails
        let environmental_tails = ci.generate_environmental_tails();
        self.position_tails.extend(environmental_tails);
    }
}

impl DecisionEngine {
    fn new() -> Self {
        Self {
            eei_priority_queue: VecDeque::new(),
            unsatisfied_eeis: HashMap::new(),
            decision_matrix: DecisionMatrix::default(),
        }
    }

    fn prioritize_eeis(&mut self, eei_tracker: &EEITracker) -> Vec<EEIRequest> {
        let mut priority_eeis = Vec::new();

        for (_id, unsatisfied) in &eei_tracker.active_eeis {
            // Apply decision matrix scoring
            let score = self.calculate_priority_score(unsatisfied);

            if score > 0.7 { // High priority threshold
                priority_eeis.push(unsatisfied.clone());
            }
        }

        // Sort by priority score
        priority_eeis.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
        priority_eeis
    }

    fn calculate_priority_score(&self, eei: &EEIRequest) -> f64 {
        // Simplified priority calculation
        // In production, this would use the full decision matrix
        eei.priority
    }
}

impl EEITracker {
    fn new() -> Self {
        Self {
            active_eeis: HashMap::new(),
            satisfied_eeis: VecDeque::new(),
            satisfaction_rate: 0.0,
        }
    }

    fn add_eei_request(&mut self, eei: EEIRequest) {
        self.active_eeis.insert(eei.eei_id.clone(), eei);
    }

    fn update_satisfaction_status(&mut self) {
        // Check for expired EEIs and update satisfaction rate
        let total_eeis = self.active_eeis.len() + self.satisfied_eeis.len();
        if total_eeis > 0 {
            self.satisfaction_rate = self.satisfied_eeis.len() as f64 / total_eeis as f64;
        }
    }

    fn get_priority_eeis(&self, count: usize) -> Vec<EEIRequest> {
        let mut eeis: Vec<_> = self.active_eeis.values().cloned().collect();
        eeis.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
        eeis.into_iter().take(count).collect()
    }
}

impl Default for PTIEConfig {
    fn default() -> Self {
        Self {
            cycle_interval_ms: 5000, // 5 second OODA cycles
            sliding_window_size: 1000,
            eei_timeout_minutes: 60,
            priority_thresholds: PriorityThresholds::default(),
            interface_config: InterfaceConfig::default(),
        }
    }
}

impl Default for PriorityThresholds {
    fn default() -> Self {
        Self {
            critical: 0.9,
            high: 0.7,
            medium: 0.4,
            low: 0.0,
        }
    }
}

impl Default for InterfaceConfig {
    fn default() -> Self {
        Self {
            enable_gui: true,
            enable_ascii_cli: true,
            contextual_bridge_enabled: true,
            universal_hover_enabled: true,
        }
    }
}

impl Default for DecisionMatrix {
    fn default() -> Self {
        Self {
            threat_weight: 0.4,
            urgency_weight: 0.3,
            context_weight: 0.2,
            confidence_weight: 0.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usim::USIMBuilder;

    #[test]
    fn test_ptie_engine_creation() {
        let config = PTIEConfig::default();
        let ptie = ProactiveThreatIntelligenceEngine::new(config);

        assert!(matches!(ptie.state, PTIEState::Initializing));
        assert!(matches!(ptie.ooda_processor.current_phase, OODAPhase::Observe));
    }

    #[test]
    fn test_ooda_metrics_update() {
        let mut metrics = OODAMetrics::new();

        metrics.update_cycle_metrics(1000);
        assert_eq!(metrics.cycle_count, 1);
        assert_eq!(metrics.last_cycle_duration_ms, 1000);
        assert_eq!(metrics.average_cycle_time_ms, 1000.0);

        metrics.update_cycle_metrics(2000);
        assert_eq!(metrics.cycle_count, 2);
        assert_eq!(metrics.average_cycle_time_ms, 1500.0);
    }

    #[test]
    fn test_eei_tracker() {
        let mut tracker = EEITracker::new();

        let eei = EEIRequest {
            eei_id: "TEST-001".to_string(),
            description: "Test EEI".to_string(),
            priority: 0.8,
            target_context: Context::Logical {
                system_id: "TEST".to_string(),
                relative_position: "TEST".to_string(),
            },
            collection_method: CollectionMethod::OSINT,
            deadline: Utc::now() + chrono::Duration::hours(1),
            requestor: "TEST".to_string(),
        };

        tracker.add_eei_request(eei);
        assert_eq!(tracker.active_eeis.len(), 1);

        let priority_eeis = tracker.get_priority_eeis(1);
        assert_eq!(priority_eeis.len(), 1);
        assert_eq!(priority_eeis[0].priority, 0.8);
    }
}