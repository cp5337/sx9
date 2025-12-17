//! ECS Systems for Plasma Defender
//!
//! Systems that operate on threat entities for detection and response.

use crate::ecs::components::*;
use anyhow::Result;

// =============================================================================
// THREAT PROCESSING SYSTEMS
// =============================================================================

/// Threat evaluation system - evaluates threat entities and updates HD4 phase
pub struct ThreatEvalSystem;

impl ThreatEvalSystem {
    /// Evaluate threats and determine HD4 phase transitions
    pub fn evaluate(
        threats: &[ThreatEntityComponent],
        crystals: &[CrystalEvalComponent],
        hd4s: &mut [Hd4PhaseComponent],
    ) -> Result<Vec<u64>> {
        let mut transitioned = Vec::new();

        for ((threat, crystal), hd4) in threats.iter().zip(crystals.iter()).zip(hd4s.iter_mut()) {
            // Calculate threat score from confidence and ring strength
            let threat_score = threat.confidence * crystal.ring_strength;

            // Determine target phase based on threat score
            let target_phase = match threat_score {
                s if s >= 0.9 => Hd4Phase::Dominate,
                s if s >= 0.7 => Hd4Phase::Disable,
                s if s >= 0.5 => Hd4Phase::Disrupt,
                s if s >= 0.3 => Hd4Phase::Detect,
                _ => Hd4Phase::Hunt,
            };

            // Check for phase transition
            if target_phase != hd4.phase {
                hd4.previous_phase = Some(hd4.phase);
                hd4.phase = target_phase;
                hd4.ticks_in_phase = 0;
                hd4.actions_taken = 0;
                transitioned.push(threat.entity_id);
            } else {
                hd4.ticks_in_phase += 1;
            }
        }

        Ok(transitioned)
    }
}

/// Crystal resonance system - calculates ring strength for threat evaluation
pub struct CrystalResonanceSystem;

impl CrystalResonanceSystem {
    /// Calculate crystal resonance for threats
    pub fn calculate(
        threats: &[ThreatEntityComponent],
        crystals: &mut [CrystalEvalComponent],
    ) -> Result<()> {
        for (threat, crystal) in threats.iter().zip(crystals.iter_mut()) {
            // Ring strength based on threat confidence and speed class
            let speed_factor = match threat.speed_class {
                0 => 1.0,  // Hot - full strength
                1 => 0.75, // Warm - 75%
                _ => 0.5,  // Cold - 50%
            };

            crystal.ring_strength = threat.confidence * speed_factor;
            crystal.last_eval_ns = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64;
            crystal.eval_count += 1;

            // Resonance frequency based on entity hash
            crystal.resonance_frequency = (threat.threat_hash as f64 % 1000.0) / 1000.0;
        }

        Ok(())
    }
}

/// SDT gate system - evaluates gate thresholds and triggers latching
pub struct SdtGateSystem;

impl SdtGateSystem {
    /// Evaluate SDT gates and latch if threshold exceeded
    pub fn evaluate(
        threats: &[ThreatEntityComponent],
        hd4s: &[Hd4PhaseComponent],
        gates: &mut [SdtGateComponent],
        current_tick: u64,
    ) -> Result<Vec<u64>> {
        let mut latched = Vec::new();

        for ((threat, hd4), gate) in threats.iter().zip(hd4s.iter()).zip(gates.iter_mut()) {
            // Gate value based on HD4 phase and confidence
            let value = hd4.phase.y_axis() as f32 * threat.confidence;
            gate.current_value = value;

            // Check for latching
            if value >= gate.threshold && gate.state != SdtState::Latched {
                gate.state = SdtState::Latched;
                gate.last_trigger_tick = current_tick;
                gate.trigger_count += 1;
                latched.push(threat.entity_id);
            } else if value >= gate.threshold * 0.5 && gate.state == SdtState::Off {
                gate.state = SdtState::Primed;
            } else if value >= gate.threshold * 0.75 && gate.state == SdtState::Primed {
                gate.state = SdtState::Conducting;
            }
        }

        Ok(latched)
    }
}

// =============================================================================
// THREAT OBSERVER SYSTEMS (ANN)
// =============================================================================

/// Threat observer system - watches entity patterns for anomalies
pub struct ThreatObserverSystem;

impl ThreatObserverSystem {
    /// Observe threat patterns and update observers
    pub fn observe(
        threats: &[ThreatEntityComponent],
        hd4s: &[Hd4PhaseComponent],
        crystals: &[CrystalEvalComponent],
        observers: &mut [ThreatObserverComponent],
    ) -> Result<()> {
        for (((threat, hd4), crystal), observer) in threats
            .iter()
            .zip(hd4s.iter())
            .zip(crystals.iter())
            .zip(observers.iter_mut())
        {
            // Only observe in active modes
            if observer.mode == ThreatObserverMode::Passive {
                continue;
            }

            // Build pattern buffer from threat state
            observer.pattern_buffer.clear();
            observer.pattern_buffer.push(threat.confidence);
            observer
                .pattern_buffer
                .push(threat.speed_class as f32 / 2.0);
            observer.pattern_buffer.push(hd4.phase.y_axis() as f32);
            observer.pattern_buffer.push(crystal.ring_strength);
            observer
                .pattern_buffer
                .push(crystal.resonance_frequency as f32);

            // Calculate observation hash
            let hash = observer
                .pattern_buffer
                .iter()
                .map(|f| f.to_bits())
                .fold(0u64, |acc, bits| {
                    acc.wrapping_mul(31).wrapping_add(bits as u64)
                });
            observer.last_observation_hash = hash;
            observer.observation_count += 1;

            // Calculate anomaly score (simple deviation from expected)
            let expected_confidence = hd4.phase.y_axis() as f32;
            observer.anomaly_score = (threat.confidence - expected_confidence).abs();
        }

        Ok(())
    }

    /// Set observer mode for all observers
    pub fn set_mode(observers: &mut [ThreatObserverComponent], mode: ThreatObserverMode) {
        for observer in observers {
            observer.mode = mode;
        }
    }
}

/// Anomaly detection system - detects anomalous patterns
pub struct AnomalyDetectionSystem;

impl AnomalyDetectionSystem {
    /// Detect anomalies in observer patterns
    pub fn detect(
        observers: &[ThreatObserverComponent],
        threshold: f32,
    ) -> Result<Vec<(String, f32)>> {
        let mut anomalies = Vec::new();

        for observer in observers {
            if observer.anomaly_score >= threshold {
                anomalies.push((observer.observer_id.clone(), observer.anomaly_score));
            }
        }

        Ok(anomalies)
    }
}

// =============================================================================
// OSSEC PROCESSING SYSTEMS
// =============================================================================

/// OSSEC alert processing system
pub struct OssecAlertSystem;

impl OssecAlertSystem {
    /// Process OSSEC alerts and update threat entities
    pub fn process(
        alerts: &[OssecAlertComponent],
        threats: &mut [ThreatEntityComponent],
    ) -> Result<Vec<u64>> {
        let mut updated = Vec::new();

        for (alert, threat) in alerts.iter().zip(threats.iter_mut()) {
            // Update threat confidence based on alert level
            let alert_confidence = alert.level as f32 / 15.0;
            if alert_confidence > threat.confidence {
                threat.confidence = alert_confidence;
                updated.push(threat.entity_id);
            }

            // Update speed class based on alert level
            threat.speed_class = if alert.level >= 10 {
                0 // Hot
            } else if alert.level >= 5 {
                1 // Warm
            } else {
                2 // Cold
            };

            // Update last seen
            if alert.timestamp > threat.last_seen_ns {
                threat.last_seen_ns = alert.timestamp;
            }
        }

        Ok(updated)
    }
}

// =============================================================================
// EEI CORRELATION SYSTEMS
// =============================================================================

/// EEI correlation system - correlates threats with intelligence requirements
pub struct EeiCorrelationSystem;

impl EeiCorrelationSystem {
    /// Update EEI correlations for threats
    pub fn correlate(
        threats: &[ThreatEntityComponent],
        ossec_alerts: &[OssecAlertComponent],
        correlations: &mut [EeiCorrelationComponent],
    ) -> Result<Vec<u64>> {
        let mut correlated = Vec::new();

        for ((threat, alert), correlation) in threats
            .iter()
            .zip(ossec_alerts.iter())
            .zip(correlations.iter_mut())
        {
            // Extract keywords from alert for EEI matching
            let mut keywords = Vec::new();

            if let Some(ref technique) = alert.mitre_technique {
                keywords.push(technique.clone());
            }
            if let Some(ref tactic) = alert.mitre_tactic {
                keywords.push(tactic.clone());
            }
            if let Some(ref src_ip) = alert.src_ip {
                keywords.push(src_ip.clone());
            }

            if !keywords.is_empty() {
                correlation.matched_keywords = keywords;
                correlation.correlation_score = threat.confidence;
                correlation.last_correlation_ns = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64;
                correlated.push(threat.entity_id);
            }
        }

        Ok(correlated)
    }
}

// =============================================================================
// AGENT SYSTEMS
// =============================================================================

/// Agent tick system - updates agent state each tick
pub struct AgentTickSystem;

impl AgentTickSystem {
    /// Tick all agents
    pub fn tick(agents: &mut [AgentComponent], current_tick: u64) -> Result<()> {
        for agent in agents {
            if agent.active {
                agent.last_tick = current_tick;
            }
        }
        Ok(())
    }
}
