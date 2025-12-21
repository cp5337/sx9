#![allow(clippy::items_after_statements)]
#![allow(clippy::format_push_string)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::unused_async)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::similar_names)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::unused_self)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]
//! SX9 ATLAS Daemon - Cognitive Engine
//!
//! RFC-9022: ATLAS Daemon Specification
//! RFC-9024: Dual Neurotransmitter Systems
//!
//! Core Element: Cognitive Engine (OODA)
//! Implementation: sx9-atlas-daemon::ooda_loop
//!
//! Zone B - 1ms cognitive tick for OODA loop processing
//! Integrated with sx9-atlas-bus for command dispatch

pub mod convergence;
pub mod hd4_phases;
pub mod ooda_loop;

pub use convergence::{ConvergenceCalculator, ConvergenceScore};
pub use hd4_phases::{HD4Phase, VerticalLevel};
pub use ooda_loop::{OodaLoop, OodaOutcome, OodaState, TacticalAction};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sx9_atlas_bus::{AtlasBus, PlasmaState, ThyristorConfig};
use sx9_foundation_core::async_runtime::tokio::time::{interval, Duration};

/// Trivariate hash type alias
pub type TrivariateHash = u128;

/// ATLAS Daemon configuration
#[derive(Debug, Clone)]
pub struct AtlasConfig {
    /// Port for ATLAS daemon (default: 18500)
    pub port: u16,
    /// Cognitive tick rate in milliseconds (default: 1ms per RFC-9022)
    pub tick_rate_ms: u64,
    /// Maximum allowed tick duration before zone violation
    pub max_tick_duration_ms: u64,
    /// Enable telemetry
    pub telemetry_enabled: bool,
    /// Crystal family for resonance evaluation
    pub crystal_family: sx9_atlas_bus::CrystalFamily,
    /// SDT gate configuration
    pub sdt_config: ThyristorConfig,
}

impl Default for AtlasConfig {
    fn default() -> Self {
        Self {
            port: 18500,
            tick_rate_ms: 1,
            max_tick_duration_ms: 1,
            telemetry_enabled: true,
            crystal_family: sx9_atlas_bus::CrystalFamily::GroundStation,
            sdt_config: ThyristorConfig::default(),
        }
    }
}

/// ATLAS Daemon - Main cognitive engine
pub struct AtlasDaemon {
    config: AtlasConfig,
    ooda: OodaLoop,
    /// AtlasBus for command dispatch (shared)
    bus: Arc<AtlasBus>,
    /// PlasmaState for delta angle tracking
    plasma: Arc<PlasmaState>,
    /// Tick counter
    tick_count: u64,
}

impl AtlasDaemon {
    /// Create new ATLAS daemon
    pub fn new(config: AtlasConfig) -> Self {
        let bus = Arc::new(AtlasBus::new());
        let plasma = Arc::new(PlasmaState::new());

        // Prime SDT gate
        plasma.prime();

        let ooda = OodaLoop::new(VerticalLevel::Tactical, HD4Phase::Hunt, bus.clone());

        Self {
            config,
            ooda,
            bus,
            plasma,
            tick_count: 0,
        }
    }

    /// Start the cognitive tick loop (Zone B - 1ms)
    pub async fn start_cognitive_tick(&mut self) {
        let mut ticker = interval(Duration::from_millis(self.config.tick_rate_ms));

        tracing::info!(
            "SX9 ATLAS Daemon starting cognitive tick loop ({}ms interval)",
            self.config.tick_rate_ms
        );

        loop {
            ticker.tick().await;
            let start = std::time::Instant::now();

            // Execute OODA cycle (with tick parameter)
            let outcome = self.ooda.cycle(Some(self.tick_count)).await;

            // Update plasma state based on outcome
            self.update_plasma_state(&outcome);

            // Check Bernoulli Zone B compliance
            let elapsed = start.elapsed();
            if elapsed > Duration::from_millis(self.config.max_tick_duration_ms) {
                tracing::error!(
                    "BERNOULLI ZONE B VIOLATION: Tick {} took {:?}",
                    self.tick_count,
                    elapsed
                );
            }

            self.tick_count += 1;
        }
    }

    /// Update plasma state based on OODA outcome
    fn update_plasma_state(&self, outcome: &OodaOutcome) {
        match outcome {
            OodaOutcome::Execute(phase, _) => {
                // Update delta angle based on phase transition
                let delta_degrees = match phase {
                    HD4Phase::Hunt => 5.0f32,
                    HD4Phase::Detect => 15.0f32,
                    HD4Phase::Disable => 30.0f32,
                    HD4Phase::Disrupt => 60.0f32,
                    HD4Phase::Dominate => 90.0f32,
                };
                self.plasma.set_delta_angle(delta_degrees);
            }
            OodaOutcome::Escalate(_, _) => {
                // Escalation increases delta angle
                self.plasma.set_delta_angle(90.0f32);
            }
            _ => {}
        }
    }

    /// Get current tick count
    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    /// Get current OODA state
    pub fn ooda_state(&self) -> OodaState {
        self.ooda.state()
    }

    /// Get plasma snapshot
    pub fn plasma_snapshot(&self) -> sx9_atlas_bus::PlasmaSnapshot {
        self.plasma.snapshot()
    }

    /// Get AtlasBus reference
    pub fn bus(&self) -> &Arc<AtlasBus> {
        &self.bus
    }
}

/// ATLAS daemon status for API endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtlasStatus {
    pub version: &'static str,
    pub tick_count: u64,
    pub current_phase: HD4Phase,
    pub vertical_level: VerticalLevel,
    pub zone: &'static str,
    pub tick_rate_ms: u64,
    pub plasma: PlasmaStatus,
}

/// Plasma status (serializable version of PlasmaSnapshot)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlasmaStatus {
    pub delta_angle: u16,
    pub entropy: u32,
    pub excited: bool,
    pub sdt_state: String,
    pub last_trigger_tick: u64,
    pub trigger_count: u32,
    pub last_ring_strength: f32,
    pub supersession_count: u32,
}

impl AtlasStatus {
    pub fn from_daemon(daemon: &AtlasDaemon) -> Self {
        let state = daemon.ooda_state();
        let plasma_snap = daemon.plasma_snapshot();
        Self {
            version: "0.1.0",
            tick_count: daemon.tick_count(),
            current_phase: state.phase,
            vertical_level: state.level,
            zone: "B",
            tick_rate_ms: daemon.config.tick_rate_ms,
            plasma: PlasmaStatus {
                delta_angle: plasma_snap.delta_angle,
                entropy: plasma_snap.entropy,
                excited: plasma_snap.excited,
                sdt_state: format!("{:?}", plasma_snap.sdt_state),
                last_trigger_tick: plasma_snap.last_trigger_tick,
                trigger_count: plasma_snap.trigger_count,
                last_ring_strength: plasma_snap.last_ring_strength,
                supersession_count: plasma_snap.supersession_count,
            },
        }
    }
}
