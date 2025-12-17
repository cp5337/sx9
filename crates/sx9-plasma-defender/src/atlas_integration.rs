//! ATLAS Daemon Integration for Plasma Defender
//!
//! RFC-9022: ATLAS Daemon Specification
//! RFC-9024: Dual Neurotransmitter Systems
//!
//! Provides:
//! - 1ms cognitive tick loop (Zone B compliance)
//! - OODA cycle for threat assessment
//! - HD4 phase transitions
//! - Ring Bus integration for distributed execution

use crate::ecs::components::Hd4Phase;
use crate::ring_bus::{RingBusNode, RingMessage, RingMessageType, ThreatEvent, DEFENDER_NODE_ID};
use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;
use sx9_atlas_bus::{AtlasBus, Command, CommandKind, PlasmaState};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

// =============================================================================
// CONFIGURATION
// =============================================================================

/// ATLAS Configuration for Plasma Defender
/// Tuned for threat response (different from general sx9-atlas-daemon)
#[derive(Debug, Clone)]
pub struct DefenderAtlasConfig {
    /// Cognitive tick rate in milliseconds (1ms per RFC-9022)
    pub tick_rate_ms: u64,
    /// Maximum tick duration before zone violation
    pub max_tick_duration_ms: u64,
    /// Starting HD4 phase
    pub initial_phase: Hd4Phase,
    /// Crystal family for threat evaluation
    pub crystal_family: sx9_atlas_bus::CrystalFamily,
    /// Enable NATS bridge for distributed ATLAS
    pub nats_enabled: bool,
    /// NATS URL
    pub nats_url: String,
}

impl Default for DefenderAtlasConfig {
    fn default() -> Self {
        Self {
            tick_rate_ms: 1,
            max_tick_duration_ms: 1,
            initial_phase: Hd4Phase::Hunt,
            crystal_family: sx9_atlas_bus::CrystalFamily::TarPit, // Defensive for threats
            nats_enabled: true,
            nats_url: "nats://localhost:4222".to_string(),
        }
    }
}

// =============================================================================
// OODA STATE
// =============================================================================

/// OODA cycle phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OodaPhase {
    Observe,
    Orient,
    Decide,
    Act,
}

/// OODA state for threat processing
#[derive(Debug, Clone)]
pub struct OodaState {
    /// Current OODA phase
    pub phase: OodaPhase,
    /// Current HD4 phase
    pub hd4_phase: Hd4Phase,
    /// Tick count
    pub tick_count: u64,
    /// Last observation
    pub last_observation: Option<ThreatObservation>,
    /// Last decision
    pub last_decision: Option<ThreatDecision>,
    /// Zone violations count
    pub zone_violations: u64,
}

impl Default for OodaState {
    fn default() -> Self {
        Self {
            phase: OodaPhase::Observe,
            hd4_phase: Hd4Phase::Hunt,
            tick_count: 0,
            last_observation: None,
            last_decision: None,
            zone_violations: 0,
        }
    }
}

// =============================================================================
// OBSERVATIONS AND DECISIONS
// =============================================================================

/// Threat observation from agents
#[derive(Debug, Clone)]
pub struct ThreatObservation {
    /// Source agent/component that observed
    pub source: String,
    /// Trivariate hash of threat
    pub threat_hash: u64,
    /// Entity ID in ECS
    pub entity_id: u64,
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
    /// MITRE technique if known
    pub mitre_technique: Option<String>,
    /// Observation timestamp
    pub timestamp: Instant,
}

/// Decision from OODA cycle
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThreatDecision {
    /// Action to take
    pub action: ThreatAction,
    /// Target threat hash
    pub target_hash: u64,
    /// Target entity ID
    pub entity_id: u64,
    /// Priority (0-255, higher = more urgent)
    pub priority: u8,
    /// HD4 phase for this decision
    pub hd4_phase: Hd4Phase,
}

/// Threat response actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ThreatAction {
    /// Continue observation (Hunt phase)
    Monitor,
    /// Generate alert (Detect phase)
    Alert,
    /// Block threat (Disrupt phase)
    Block,
    /// Network isolation (Disable phase)
    Isolate,
    /// Escalate to higher level
    Escalate,
    /// Full neutralization (Dominate phase)
    Neutralize,
}

// =============================================================================
// DEFENDER ATLAS
// =============================================================================

/// Defender's ATLAS Integration
///
/// Provides cognitive processing for threat assessment with:
/// - 1ms OODA loop
/// - HD4 phase transitions
/// - Ring Bus integration
pub struct DefenderAtlas {
    config: DefenderAtlasConfig,
    bus: Arc<AtlasBus>,
    plasma: Arc<PlasmaState>,
    state: OodaState,

    // Channels for threat processing
    observation_tx: mpsc::Sender<ThreatObservation>,
    observation_rx: mpsc::Receiver<ThreatObservation>,
    decision_tx: mpsc::Sender<ThreatDecision>,

    // Ring Bus integration
    ring_bus: Option<Arc<RingBusNode>>,
}

impl DefenderAtlas {
    /// Create new Defender ATLAS
    pub fn new(config: DefenderAtlasConfig) -> Self {
        let bus = Arc::new(AtlasBus::new());
        let plasma = Arc::new(PlasmaState::new());

        // Prime SDT gate for threat response
        plasma.prime();

        let (observation_tx, observation_rx) = mpsc::channel(1024);
        let (decision_tx, _) = mpsc::channel(1024);

        let state = OodaState {
            hd4_phase: config.initial_phase,
            ..Default::default()
        };

        Self {
            config,
            bus,
            plasma,
            state,
            observation_tx,
            observation_rx,
            decision_tx,
            ring_bus: None,
        }
    }

    /// Connect to Ring Bus
    pub fn with_ring_bus(mut self, ring_bus: Arc<RingBusNode>) -> Self {
        self.ring_bus = Some(ring_bus);
        self
    }

    /// Get observation sender for agents to submit observations
    pub fn observation_sender(&self) -> mpsc::Sender<ThreatObservation> {
        self.observation_tx.clone()
    }

    /// Get decision sender for external decision injection
    pub fn decision_sender(&self) -> mpsc::Sender<ThreatDecision> {
        self.decision_tx.clone()
    }

    /// Start the 1ms cognitive tick loop (Zone B)
    pub async fn start_cognitive_tick(&mut self) {
        let mut ticker = interval(Duration::from_millis(self.config.tick_rate_ms));

        tracing::info!(
            "🧠 Defender ATLAS starting cognitive tick loop ({}ms interval)",
            self.config.tick_rate_ms
        );

        loop {
            ticker.tick().await;
            let start = Instant::now();

            // Execute OODA cycle
            self.ooda_cycle().await;

            // Check Zone B compliance
            let elapsed = start.elapsed();
            if elapsed > Duration::from_millis(self.config.max_tick_duration_ms) {
                self.state.zone_violations += 1;
                tracing::error!(
                    "⚠️ BERNOULLI ZONE B VIOLATION #{}: Tick {} took {:?}",
                    self.state.zone_violations,
                    self.state.tick_count,
                    elapsed
                );
            }

            self.state.tick_count += 1;
        }
    }

    /// Execute single OODA cycle
    async fn ooda_cycle(&mut self) {
        match self.state.phase {
            OodaPhase::Observe => {
                self.observe().await;
            }
            OodaPhase::Orient => {
                self.orient().await;
            }
            OodaPhase::Decide => {
                self.decide().await;
            }
            OodaPhase::Act => {
                self.act().await;
            }
        }
    }

    /// OBSERVE: Check for new threat observations
    async fn observe(&mut self) {
        // Try to receive observation (non-blocking)
        if let Ok(obs) = self.observation_rx.try_recv() {
            tracing::debug!(
                "Observed threat {} from {} (confidence: {:.2})",
                obs.threat_hash,
                obs.source,
                obs.confidence
            );
            self.state.last_observation = Some(obs);
            self.state.phase = OodaPhase::Orient;
        }
        // Stay in Observe if no observations
    }

    /// ORIENT: Evaluate threat through crystal resonance
    async fn orient(&mut self) {
        if let Some(ref obs) = self.state.last_observation {
            // Evaluate crystal resonance
            let ring_strength = self.evaluate_crystal_resonance(obs);

            // Determine HD4 phase based on threat level
            let new_phase = self.determine_hd4_phase(obs.confidence, ring_strength);

            if new_phase != self.state.hd4_phase {
                tracing::info!(
                    "HD4 transition: {:?} → {:?} (threat: {})",
                    self.state.hd4_phase,
                    new_phase,
                    obs.threat_hash
                );
                self.state.hd4_phase = new_phase;

                // Update plasma delta angle
                self.plasma.set_delta_angle(new_phase.delta_angle());
            }
        }
        self.state.phase = OodaPhase::Decide;
    }

    /// DECIDE: Make threat response decision
    async fn decide(&mut self) {
        if let Some(ref obs) = self.state.last_observation {
            let decision = self.make_decision(obs);

            tracing::debug!(
                "Decision: {:?} for threat {} (priority: {})",
                decision.action,
                decision.target_hash,
                decision.priority
            );

            self.state.last_decision = Some(decision);
        }
        self.state.phase = OodaPhase::Act;
    }

    /// ACT: Execute decision via Ring Bus
    async fn act(&mut self) {
        if let Some(ref decision) = self.state.last_decision {
            // Dispatch to AtlasBus (local)
            self.dispatch_to_bus(decision);

            // Forward to Ring Bus if connected (distributed)
            if let Some(ref ring_bus) = self.ring_bus {
                if let Err(e) = self.forward_to_ring(ring_bus, decision).await {
                    tracing::warn!("Failed to forward to Ring Bus: {}", e);
                }
            }
        }

        // Reset for next cycle
        self.state.last_observation = None;
        self.state.last_decision = None;
        self.state.phase = OodaPhase::Observe;
    }

    // =========================================================================
    // HELPER METHODS
    // =========================================================================

    /// Evaluate crystal resonance for threat
    fn evaluate_crystal_resonance(&self, obs: &ThreatObservation) -> f32 {
        // Use crystal family to determine resonance
        let base_resonance = match self.config.crystal_family {
            sx9_atlas_bus::CrystalFamily::TarPit => 0.9, // High resonance for trapping
            sx9_atlas_bus::CrystalFamily::Silent => 0.3, // Low resonance, stealth
            sx9_atlas_bus::CrystalFamily::GroundStation => 0.8, // Stable, strict
            sx9_atlas_bus::CrystalFamily::Orbital => 0.6, // High entropy tolerance
            sx9_atlas_bus::CrystalFamily::Adaptive => 0.7, // Learns from patterns
        };

        // Combine with observation confidence
        obs.confidence * base_resonance
    }

    /// Determine HD4 phase based on threat assessment
    fn determine_hd4_phase(&self, confidence: f32, ring_strength: f32) -> Hd4Phase {
        let threat_score = confidence * ring_strength;

        match threat_score {
            s if s >= 0.9 => Hd4Phase::Dominate,
            s if s >= 0.7 => Hd4Phase::Disable,
            s if s >= 0.5 => Hd4Phase::Disrupt,
            s if s >= 0.3 => Hd4Phase::Detect,
            _ => Hd4Phase::Hunt,
        }
    }

    /// Make threat response decision
    fn make_decision(&self, obs: &ThreatObservation) -> ThreatDecision {
        let action = match self.state.hd4_phase {
            Hd4Phase::Hunt => ThreatAction::Monitor,
            Hd4Phase::Detect => ThreatAction::Alert,
            Hd4Phase::Disrupt => ThreatAction::Block,
            Hd4Phase::Disable => ThreatAction::Isolate,
            Hd4Phase::Dominate => ThreatAction::Neutralize,
        };

        ThreatDecision {
            action,
            target_hash: obs.threat_hash,
            entity_id: obs.entity_id,
            priority: (obs.confidence * 255.0) as u8,
            hd4_phase: self.state.hd4_phase,
        }
    }

    /// Dispatch decision to AtlasBus
    fn dispatch_to_bus(&self, decision: &ThreatDecision) {
        let cmd = Command::new(CommandKind::SdtTrigger {
            gate_id: decision.target_hash as u32,
            reason: decision.hd4_phase as u16,
        });

        self.bus.dispatch(cmd);
    }

    /// Forward decision to Ring Bus
    async fn forward_to_ring(
        &self,
        ring_bus: &RingBusNode,
        decision: &ThreatDecision,
    ) -> Result<()> {
        let event = ThreatEvent {
            threat_hash: decision.target_hash,
            entity_id: decision.entity_id,
            confidence: decision.priority as f32 / 255.0,
            hd4_phase: decision.hd4_phase as u8,
            mitre_technique: None,
            source: "atlas".to_string(),
            timestamp_ns: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
            source_node: DEFENDER_NODE_ID,
        };

        // Publish threat event
        ring_bus.publish_threat_detected(&event).await?;

        // Also broadcast on Ring Bus for other nodes
        let msg = RingMessage::new(
            RingMessageType::Hd4Transition,
            serde_json::to_vec(decision).unwrap_or_default(),
        );
        ring_bus.ring_broadcast(msg).await?;

        Ok(())
    }

    // =========================================================================
    // PUBLIC ACCESSORS
    // =========================================================================

    /// Get current OODA state
    pub fn state(&self) -> &OodaState {
        &self.state
    }

    /// Get current HD4 phase
    pub fn hd4_phase(&self) -> Hd4Phase {
        self.state.hd4_phase
    }

    /// Get tick count
    pub fn tick_count(&self) -> u64 {
        self.state.tick_count
    }

    /// Get zone violations count
    pub fn zone_violations(&self) -> u64 {
        self.state.zone_violations
    }

    /// Get plasma snapshot
    pub fn plasma_snapshot(&self) -> sx9_atlas_bus::PlasmaSnapshot {
        self.plasma.snapshot()
    }

    /// Get AtlasBus reference
    pub fn bus(&self) -> &Arc<AtlasBus> {
        &self.bus
    }

    /// Get plasma state reference
    pub fn plasma(&self) -> &Arc<PlasmaState> {
        &self.plasma
    }
}

/// Status for API endpoints
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AtlasStatus {
    pub tick_count: u64,
    pub ooda_phase: String,
    pub hd4_phase: String,
    pub zone_violations: u64,
    pub delta_angle: u16,
    pub entropy: u32,
}

impl From<&DefenderAtlas> for AtlasStatus {
    fn from(atlas: &DefenderAtlas) -> Self {
        let snap = atlas.plasma_snapshot();
        Self {
            tick_count: atlas.tick_count(),
            ooda_phase: format!("{:?}", atlas.state.phase),
            hd4_phase: format!("{:?}", atlas.hd4_phase()),
            zone_violations: atlas.zone_violations(),
            delta_angle: snap.delta_angle,
            entropy: snap.entropy,
        }
    }
}
