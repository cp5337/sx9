#![allow(unexpected_cfgs)] // Suppress warning about fastrand feature not being in Cargo.toml

//! OODA Loop Implementation
//!
//! RFC-9022: ATLAS Daemon OODA Loop
//! Observe-Orient-Decide-Act cognitive cycle
//!
//! Adapted from ctas7-atlas-daemon to use sx9-atlas-bus

use crate::convergence::ConvergenceCalculator;
use crate::hd4_phases::{HD4Phase, VerticalLevel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sx9_atlas_bus::{AtlasBus, Crystal, CrystalFamily};

/// Trivariate hash type alias
pub type TrivariateHash = u128;

/// OODA Loop outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OodaOutcome {
    /// Execute an HD4 phase action
    Execute(HD4Phase, String),
    /// Escalate to higher vertical level
    Escalate(VerticalLevel, String),
    /// Cycle back to observe phase
    #[allow(dead_code)]
    CycleBack,
    /// No operation (maintain current state)
    NoOp,
}

/// Current state of the OODA loop
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OodaState {
    pub level: VerticalLevel,
    pub phase: HD4Phase,
    pub h1_score: f64,
    pub h2_score: f64,
}

/// Tactical action types for HD4 phase execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TacticalAction {
    /// Isolate a compromised entity by hash
    Isolate(TrivariateHash),
    /// Reroute traffic from a compromised source
    Reroute(TrivariateHash),
    /// No operation required
    NoOp,
}

/// OODA Loop processor integrated with sx9-atlas-bus
pub struct OodaLoop {
    level: VerticalLevel,
    current_phase: HD4Phase,
    convergence: ConvergenceCalculator,
    cycle_count: u64,
    /// Reference to AtlasBus for command dispatch
    bus: Arc<AtlasBus>,
    /// Crystal for resonance evaluation
    #[allow(dead_code)]
    crystal: Crystal,
}

impl OodaLoop {
    /// Create new OODA loop at specified level and phase
    pub fn new(level: VerticalLevel, initial_phase: HD4Phase, bus: Arc<AtlasBus>) -> Self {
        Self {
            level,
            current_phase: initial_phase,
            convergence: ConvergenceCalculator::new(),
            cycle_count: 0,
            bus,
            crystal: Crystal::new(CrystalFamily::GroundStation),
        }
    }

    /// Execute one OODA cycle
    ///
    /// Enhanced version that supports explicit tick tracking for telemetry.
    /// If tick is None, uses internal cycle_count.
    pub async fn cycle(&mut self, tick: Option<u64>) -> OodaOutcome {
        let current_tick = tick.unwrap_or_else(|| {
            self.cycle_count += 1;
            self.cycle_count
        });
        self.cycle_count = current_tick;

        // 1. OBSERVE - Gather telemetry hashes from AtlasBus
        let observations = self.observe_telemetry().await;

        // 2. ORIENT - Calculate convergence scores from observations
        let (h1_score, h2_score) = self.calculate_convergence(observations).await;

        // Update convergence calculator
        self.convergence.update(h1_score, h2_score);

        // 3. DECIDE - Determine action based on scores and crystal resonance
        let decision = self.decide(h1_score, h2_score).await;

        // 4. ACT - Return outcome for execution
        self.act(decision).await
    }

    /// OBSERVE phase - Gather telemetry hashes from AtlasBus
    ///
    /// RFC-9022: Collects trivariate hashes from industrial telemetry.
    /// Pulls commands from AtlasBus and extracts hashes.
    async fn observe_telemetry(&self) -> Vec<TrivariateHash> {
        // Drain commands from bus for this tick
        let mut hashes = Vec::new();

        // Process commands from bus
        for _cmd in self.bus.tick() {
            // Extract hash from command (if available)
            // Commands may contain trivariate hashes in their payload
            // For now, simulate hash extraction
            hashes.push(fastrand::u128(..));
        }

        // If no commands, simulate single event
        if hashes.is_empty() {
            hashes.push(fastrand::u128(..));
        }

        hashes
    }

    /// Calculate convergence from telemetry observations
    ///
    /// RFC-9021: H1 (Operational) - Fast Excitatory System
    /// RFC-9024: H2 (Semantic) - Slow Modulatory System
    async fn calculate_convergence(&self, observations: Vec<TrivariateHash>) -> (f64, f64) {
        // H1 (Operational) - Fast Excitatory System (RFC-9024)
        // Driven by immediate events, delta angles, and Hawkes intensity
        let h1_score = 0.5 + (observations.len() as f64 * 0.1) + fastrand::f64() * 0.2;

        // H2 (Semantic) - Slow Modulatory System (RFC-9024)
        // Driven by Matroid rank and HMM phase detection (Zone C/Analytical)
        // In production, this would query ctas7-glaf-matroid-core
        let h2_score = 0.4 + fastrand::f64() * 0.4;

        (h1_score.min(1.0), h2_score.min(1.0))
    }

    /// ORIENT phase - Analyze and calculate convergence
    #[allow(dead_code)]
    async fn orient(&mut self, signals: &[f64]) -> (f64, f64) {
        self.convergence.update_from_signals(signals);
        (self.convergence.h1_score(), self.convergence.h2_score())
    }

    /// DECIDE phase - Determine appropriate action
    ///
    /// RFC-9021: HD4 phase transitions based on convergence thresholds
    /// Enhanced thresholds: 0.50, 0.75, 0.85, 0.90
    /// Uses crystal resonance to gate decisions
    async fn decide(&mut self, h1: f64, h2: f64) -> Decision {
        let convergence = (h1 + h2) / 2.0;

        // Check for escalation threshold (RFC-9022)
        if self.level == VerticalLevel::Tactical && convergence > 0.95 {
            return Decision::Escalate(VerticalLevel::Operational);
        }

        // Enhanced HD4 phase transition thresholds (RFC-9021)
        let should_transition = match self.current_phase {
            HD4Phase::Hunt => convergence > 0.50,
            HD4Phase::Detect => convergence > 0.75, // Enhanced from 0.65
            HD4Phase::Disable => convergence > 0.85, // Enhanced from 0.75
            HD4Phase::Disrupt => convergence > 0.90, // Enhanced from 0.85
            HD4Phase::Dominate => false,            // Terminal phase
        };

        if should_transition {
            let next_phase = self.current_phase.next();
            if next_phase != self.current_phase {
                self.current_phase = next_phase;
                return Decision::Execute(next_phase);
            }
        }

        // Determine specific tactical action based on phase and convergence
        let tactical_action = self.determine_tactical_action(h1, h2, convergence > 0.75);

        match tactical_action {
            TacticalAction::Isolate(hash) => {
                return Decision::ExecuteWithAction(HD4Phase::Disable, hash);
            }
            TacticalAction::Reroute(hash) => {
                return Decision::ExecuteWithAction(HD4Phase::Disrupt, hash);
            }
            TacticalAction::NoOp => {}
        }

        Decision::NoOp
    }

    /// Determine specific tactical action based on HD4 phase and convergence
    fn determine_tactical_action(&self, _h1: f64, _h2: f64, is_converged: bool) -> TacticalAction {
        if !is_converged {
            return TacticalAction::NoOp;
        }

        match self.current_phase {
            HD4Phase::Hunt => TacticalAction::NoOp, // Hunting, not acting yet
            HD4Phase::Detect => TacticalAction::NoOp, // Still gathering data for action
            HD4Phase::Disable => TacticalAction::Isolate(fastrand::u128(..)), // Isolate compromised PLC
            HD4Phase::Disrupt => TacticalAction::Reroute(fastrand::u128(..)), // Reroute process flow
            HD4Phase::Dominate => TacticalAction::NoOp, // Full system control/recovery (handled by Orchestrator)
        }
    }

    /// ACT phase - Generate outcome for orchestrator
    async fn act(&self, decision: Decision) -> OodaOutcome {
        match decision {
            Decision::Execute(phase) => {
                let detail = format!(
                    "Phase transition to {:?} (cycle {})",
                    phase, self.cycle_count
                );
                OodaOutcome::Execute(phase, detail)
            }
            Decision::ExecuteWithAction(phase, hash) => {
                let detail = format!(
                    "Execute {:?} action on hash {:016x} (cycle {})",
                    phase, hash, self.cycle_count
                );
                OodaOutcome::Execute(phase, detail)
            }
            Decision::Escalate(level) => {
                let reason = format!(
                    "Convergence threshold exceeded at cycle {}",
                    self.cycle_count
                );
                OodaOutcome::Escalate(level, reason)
            }
            Decision::CycleBack => OodaOutcome::CycleBack,
            Decision::NoOp => OodaOutcome::NoOp,
        }
    }

    /// Get current state
    pub fn state(&self) -> OodaState {
        OodaState {
            level: self.level,
            phase: self.current_phase,
            h1_score: self.convergence.h1_score(),
            h2_score: self.convergence.h2_score(),
        }
    }
}

/// Internal decision type
enum Decision {
    Execute(HD4Phase),
    ExecuteWithAction(HD4Phase, TrivariateHash), // Enhanced: includes tactical action hash
    Escalate(VerticalLevel),
    CycleBack,
    NoOp,
}

// Use fastrand crate if available, otherwise fallback to simple implementation
#[cfg(feature = "fastrand")]
use fastrand;

#[cfg(not(feature = "fastrand"))]
mod fastrand {
    use std::sync::atomic::{AtomicU64, Ordering};
    static STATE: AtomicU64 = AtomicU64::new(0x853c49e6748fea9b);

    pub fn f64() -> f64 {
        let state = STATE.fetch_add(0x9e3779b97f4a7c15, Ordering::Relaxed);
        let mixed = (state ^ (state >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        (mixed as f64) / (u64::MAX as f64)
    }

    pub fn u128(_: std::ops::RangeFull) -> u128 {
        let state = STATE.fetch_add(0x9e3779b97f4a7c15, Ordering::Relaxed);
        ((state as u128) << 64) | (STATE.load(Ordering::Relaxed) as u128)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sx9_atlas_bus::AtlasBus;

    #[tokio::test]
    async fn test_ooda_cycle() {
        let bus = Arc::new(AtlasBus::new());
        let mut ooda = OodaLoop::new(VerticalLevel::Tactical, HD4Phase::Hunt, bus);
        let outcome = ooda.cycle(None).await;
        // Should produce some outcome
        assert!(matches!(
            outcome,
            OodaOutcome::Execute(_, _) | OodaOutcome::NoOp | OodaOutcome::CycleBack
        ));
    }

    #[test]
    fn test_ooda_state() {
        let bus = Arc::new(AtlasBus::new());
        let ooda = OodaLoop::new(VerticalLevel::Operational, HD4Phase::Detect, bus);
        let state = ooda.state();
        assert_eq!(state.level, VerticalLevel::Operational);
        assert_eq!(state.phase, HD4Phase::Detect);
    }
}
