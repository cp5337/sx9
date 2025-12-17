//! Threat Monitoring Agents
//!
//! Network, Threat, Canary, and Anomaly detection agents integrated with
//! Plasma state, ECS, and ATLAS observation channels.

use crate::atlas_integration::ThreatObservation;
use crate::ecs::components::Hd4Phase;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Instant;
use sx9_atlas_bus::PlasmaState;
use tokio::sync::mpsc;

// =============================================================================
// AGENT TYPES AND EVENTS
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentType {
    NetworkMonitor,  // Monitor network traffic
    ThreatHunter,    // Hunt for threats
    CanaryWatcher,   // Watch canary triggers
    AnomalyDetector, // Detect anomalies
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEvent {
    pub agent_id: String,
    pub event_type: String,
    pub payload: Vec<u8>,
    pub timestamp: u64,
    pub severity: ThreatSeverity,
    pub hd4_phase: Hd4Phase,
    pub confidence: f32,
    pub mitre_technique: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatSeverity {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

impl ThreatSeverity {
    pub fn to_hd4(self) -> Hd4Phase {
        match self {
            Self::Low => Hd4Phase::Hunt,
            Self::Medium => Hd4Phase::Detect,
            Self::High => Hd4Phase::Disrupt,
            Self::Critical => Hd4Phase::Disable,
        }
    }

    pub fn to_level(self) -> u8 {
        match self {
            Self::Low => 3,
            Self::Medium => 7,
            Self::High => 11,
            Self::Critical => 14,
        }
    }
}

// =============================================================================
// THREAT AGENT
// =============================================================================

pub struct ThreatAgent {
    agent_id: String,
    agent_type: AgentType,
    plasma: Arc<PlasmaState>,
    enabled: bool,
    /// ATLAS observation channel
    observation_tx: Option<mpsc::Sender<ThreatObservation>>,
    /// Historical delta angles for anomaly detection
    delta_history: VecDeque<u16>,
    /// Historical entropy values
    entropy_history: VecDeque<u32>,
    /// Threshold for anomaly detection
    anomaly_threshold: f32,
    /// Canary trigger count
    canary_triggers: u32,
    /// Last tick processed
    last_tick: u64,
    /// Events generated
    events_generated: u64,
}

impl ThreatAgent {
    pub fn new(agent_id: String, agent_type: AgentType, plasma: Arc<PlasmaState>) -> Self {
        Self {
            agent_id,
            agent_type,
            plasma,
            enabled: true,
            observation_tx: None,
            delta_history: VecDeque::with_capacity(100),
            entropy_history: VecDeque::with_capacity(100),
            anomaly_threshold: 2.0, // 2 standard deviations
            canary_triggers: 0,
            last_tick: 0,
            events_generated: 0,
        }
    }

    /// Connect to ATLAS observation channel
    pub fn with_atlas(mut self, tx: mpsc::Sender<ThreatObservation>) -> Self {
        self.observation_tx = Some(tx);
        self
    }

    /// Set anomaly detection threshold
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.anomaly_threshold = threshold;
        self
    }

    pub async fn monitor(&mut self) -> anyhow::Result<Option<ThreatEvent>> {
        if !self.enabled {
            return Ok(None);
        }

        self.last_tick += 1;

        let event = match self.agent_type {
            AgentType::NetworkMonitor => self.monitor_network().await?,
            AgentType::ThreatHunter => self.hunt_threats().await?,
            AgentType::CanaryWatcher => self.watch_canaries().await?,
            AgentType::AnomalyDetector => self.detect_anomalies().await?,
        };

        // If event generated, send observation to ATLAS
        if let Some(ref event) = event {
            self.events_generated += 1;
            self.send_observation(event).await;
        }

        Ok(event)
    }

    /// Send observation to ATLAS
    async fn send_observation(&self, event: &ThreatEvent) {
        if let Some(ref tx) = self.observation_tx {
            let observation = ThreatObservation {
                source: format!("agent:{}", self.agent_id),
                threat_hash: self.calculate_event_hash(event),
                entity_id: 0, // Will be set when entity is created
                confidence: event.confidence,
                mitre_technique: event.mitre_technique.clone(),
                timestamp: Instant::now(),
            };

            if let Err(e) = tx.try_send(observation) {
                tracing::warn!("Failed to send observation to ATLAS: {}", e);
            }
        }
    }

    /// Calculate hash for event
    fn calculate_event_hash(&self, event: &ThreatEvent) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        event.agent_id.hash(&mut hasher);
        event.event_type.hash(&mut hasher);
        event.timestamp.hash(&mut hasher);
        hasher.finish()
    }

    // =========================================================================
    // NETWORK MONITOR
    // =========================================================================

    async fn monitor_network(&mut self) -> anyhow::Result<Option<ThreatEvent>> {
        // Get current plasma state
        let snapshot = self.plasma.snapshot();

        // Record delta angle history
        self.delta_history.push_back(snapshot.delta_angle);
        if self.delta_history.len() > 100 {
            self.delta_history.pop_front();
        }

        // Check for rapid delta angle changes (potential network attack)
        if self.delta_history.len() >= 10 {
            let recent: Vec<u16> = self.delta_history.iter().rev().take(10).copied().collect();
            let avg: f64 = recent.iter().map(|&x| x as f64).sum::<f64>() / recent.len() as f64;
            let variance: f64 = recent
                .iter()
                .map(|&x| {
                    let diff = x as f64 - avg;
                    diff * diff
                })
                .sum::<f64>()
                / recent.len() as f64;
            let std_dev = variance.sqrt();

            // High variance indicates potential attack
            if std_dev > 1000.0 {
                let severity = if std_dev > 5000.0 {
                    ThreatSeverity::Critical
                } else if std_dev > 3000.0 {
                    ThreatSeverity::High
                } else if std_dev > 2000.0 {
                    ThreatSeverity::Medium
                } else {
                    ThreatSeverity::Low
                };

                return Ok(Some(ThreatEvent {
                    agent_id: self.agent_id.clone(),
                    event_type: "network_anomaly".to_string(),
                    payload: vec![],
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as u64,
                    severity,
                    hd4_phase: severity.to_hd4(),
                    confidence: (std_dev / 10000.0).min(1.0) as f32,
                    mitre_technique: Some("T1071".to_string()), // Application Layer Protocol
                }));
            }
        }

        Ok(None)
    }

    // =========================================================================
    // THREAT HUNTER
    // =========================================================================

    async fn hunt_threats(&mut self) -> anyhow::Result<Option<ThreatEvent>> {
        let snapshot = self.plasma.snapshot();

        // Hunt based on SDT state - if latched, there's a confirmed threat
        if snapshot.sdt_state == sx9_atlas_bus::SdtState::Latched {
            // SDT latched indicates confirmed threat
            return Ok(Some(ThreatEvent {
                agent_id: self.agent_id.clone(),
                event_type: "sdt_latched".to_string(),
                payload: snapshot.trigger_count.to_le_bytes().to_vec(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64,
                severity: ThreatSeverity::High,
                hd4_phase: Hd4Phase::Disrupt,
                confidence: 0.95,
                mitre_technique: None,
            }));
        }

        // Hunt based on high entropy + high delta angle
        if snapshot.entropy > 800_000 && snapshot.delta_angle > 45000 {
            return Ok(Some(ThreatEvent {
                agent_id: self.agent_id.clone(),
                event_type: "high_entropy_threat".to_string(),
                payload: vec![],
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64,
                severity: ThreatSeverity::Medium,
                hd4_phase: Hd4Phase::Detect,
                confidence: 0.7,
                mitre_technique: Some("T1027".to_string()), // Obfuscated Files
            }));
        }

        // Hunt based on supersession count (many rapid changes)
        if snapshot.supersession_count > 10 {
            return Ok(Some(ThreatEvent {
                agent_id: self.agent_id.clone(),
                event_type: "rapid_supersession".to_string(),
                payload: vec![],
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64,
                severity: ThreatSeverity::Medium,
                hd4_phase: Hd4Phase::Detect,
                confidence: 0.6,
                mitre_technique: Some("T1070".to_string()), // Indicator Removal
            }));
        }

        Ok(None)
    }

    // =========================================================================
    // CANARY WATCHER
    // =========================================================================

    async fn watch_canaries(&mut self) -> anyhow::Result<Option<ThreatEvent>> {
        let snapshot = self.plasma.snapshot();

        // Check for canary triggers (SDT trigger count changes)
        let current_triggers = snapshot.trigger_count;

        if current_triggers > self.canary_triggers {
            let new_triggers = current_triggers - self.canary_triggers;
            self.canary_triggers = current_triggers;

            // Canary was triggered!
            return Ok(Some(ThreatEvent {
                agent_id: self.agent_id.clone(),
                event_type: "canary_triggered".to_string(),
                payload: new_triggers.to_le_bytes().to_vec(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64,
                severity: ThreatSeverity::Critical,
                hd4_phase: Hd4Phase::Disable,
                confidence: 0.99, // Canaries are high confidence
                mitre_technique: Some("T1083".to_string()), // File Discovery (accessing canary)
            }));
        }

        // Check for excited state (plasma excitation indicates intrusion)
        if snapshot.excited {
            return Ok(Some(ThreatEvent {
                agent_id: self.agent_id.clone(),
                event_type: "plasma_excited".to_string(),
                payload: vec![],
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64,
                severity: ThreatSeverity::High,
                hd4_phase: Hd4Phase::Disrupt,
                confidence: 0.85,
                mitre_technique: None,
            }));
        }

        Ok(None)
    }

    // =========================================================================
    // ANOMALY DETECTOR
    // =========================================================================

    async fn detect_anomalies(&mut self) -> anyhow::Result<Option<ThreatEvent>> {
        let snapshot = self.plasma.snapshot();

        // Record entropy history
        self.entropy_history.push_back(snapshot.entropy);
        if self.entropy_history.len() > 100 {
            self.entropy_history.pop_front();
        }

        // Need enough history for anomaly detection
        if self.entropy_history.len() < 20 {
            return Ok(None);
        }

        // Calculate entropy statistics
        let values: Vec<f64> = self.entropy_history.iter().map(|&x| x as f64).collect();
        let mean: f64 = values.iter().sum::<f64>() / values.len() as f64;
        let variance: f64 =
            values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
        let std_dev = variance.sqrt();

        // Check if current entropy is anomalous
        let current = snapshot.entropy as f64;
        let z_score = if std_dev > 0.0 {
            (current - mean).abs() / std_dev
        } else {
            0.0
        };

        if z_score > self.anomaly_threshold as f64 {
            let severity = if z_score > 4.0 {
                ThreatSeverity::Critical
            } else if z_score > 3.0 {
                ThreatSeverity::High
            } else {
                ThreatSeverity::Medium
            };

            return Ok(Some(ThreatEvent {
                agent_id: self.agent_id.clone(),
                event_type: "entropy_anomaly".to_string(),
                payload: vec![],
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64,
                severity,
                hd4_phase: severity.to_hd4(),
                confidence: ((z_score - self.anomaly_threshold as f64) / 4.0).min(1.0) as f32,
                mitre_technique: Some("T1082".to_string()), // System Information Discovery
            }));
        }

        // Check for ring strength anomalies
        if snapshot.last_ring_strength > 0.9 {
            return Ok(Some(ThreatEvent {
                agent_id: self.agent_id.clone(),
                event_type: "high_ring_strength".to_string(),
                payload: vec![],
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64,
                severity: ThreatSeverity::High,
                hd4_phase: Hd4Phase::Disrupt,
                confidence: snapshot.last_ring_strength,
                mitre_technique: None,
            }));
        }

        Ok(None)
    }

    // =========================================================================
    // AGENT CONTROL
    // =========================================================================

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn agent_id(&self) -> &str {
        &self.agent_id
    }

    pub fn agent_type(&self) -> AgentType {
        self.agent_type
    }

    pub fn events_generated(&self) -> u64 {
        self.events_generated
    }

    pub fn last_tick(&self) -> u64 {
        self.last_tick
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_to_hd4() {
        assert_eq!(ThreatSeverity::Low.to_hd4(), Hd4Phase::Hunt);
        assert_eq!(ThreatSeverity::Critical.to_hd4(), Hd4Phase::Disable);
    }
}
