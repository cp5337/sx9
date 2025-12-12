//! sx9-plasma-defender - PLASMA Defender with Crystal & SDT Integration
//!
//! Purpose: PLASMA defender for health monitoring and threat detection
//! - Health endpoint (/health)
//! - Metrics endpoint (/metrics)
//! - SDT state endpoint (/sdt/state)
//! - Crystal resonance endpoint (/crystal/resonance)
//! - Latency enforcement
//! - PLASMA state monitoring
//! - Crystal resonance evaluation
//! - SDT gate control
//! - Threat monitoring agents
//! - Optimized Axum server with compression, timeout, body limit

pub mod advisory;
pub mod agents;
pub mod ann_daemon;
pub mod config;
pub mod crystal;
pub mod health;
pub mod metrics;
pub mod monitor;
pub mod plasma_bus;
pub mod sdt;
pub mod server;
pub mod tool_handler;

pub use advisory::{AnnAdvisory, AnnContext};
pub use agents::{AgentType, ThreatAgent};
pub use ann_daemon::{AnnConfig, AnnDaemon, AnnObservation};
pub use config::DefenderConfig;
pub use crystal::CrystalIntegration;
pub use health::HealthMonitor;
pub use metrics::MetricsCollector;
pub use monitor::ThreatMonitor;
pub use plasma_bus::{PlasmaBus, PlasmaEvent};
pub use sdt::SdtIntegration;
pub use server::PlasmaDefenderServer;
pub use tool_handler::subscribe_tool_results;

use std::sync::Arc;
use sx9_atlas_bus::{CrystalFamily, PlasmaState, ThyristorConfig};
use tokio::sync::RwLock;

pub struct PlasmaDefender {
    server: PlasmaDefenderServer,
    health: HealthMonitor,
    metrics: MetricsCollector,
    agents: Arc<RwLock<Vec<ThreatAgent>>>,
    crystal: Arc<CrystalIntegration>,
    sdt: Arc<SdtIntegration>,
    monitor: Arc<RwLock<ThreatMonitor>>,
    plasma: Arc<PlasmaState>,
    plasma_bus: Arc<PlasmaBus>,
    ann_daemon: Arc<AnnDaemon>,
    config: DefenderConfig,
}

impl PlasmaDefender {
    pub async fn new(config: DefenderConfig) -> anyhow::Result<Self> {
        // Initialize PlasmaState from sx9-atlas-bus
        let plasma = Arc::new(PlasmaState::new());

        // Initialize PlasmaBus (NATS telemetry)
        let plasma_bus = Arc::new(PlasmaBus::new(&config.nats_url).await?);

        // Initialize crystal integration
        let crystal_family = config
            .crystal_family
            .unwrap_or(CrystalFamily::GroundStation);
        let crystal = Arc::new(CrystalIntegration::new(crystal_family));

        // Initialize SDT integration
        let sdt_config = config
            .sdt_config
            .as_ref()
            .map(|w| sx9_atlas_bus::ThyristorConfig::from(*w))
            .unwrap_or_else(|| sx9_atlas_bus::ThyristorConfig::default());
        let sdt = Arc::new(SdtIntegration::new(plasma.clone(), sdt_config));

        // Initialize threat agents
        let agents = Arc::new(RwLock::new(Vec::new()));
        for agent_config in &config.agents {
            let agent = ThreatAgent::new(
                agent_config.id.clone(),
                agent_config.agent_type,
                plasma.clone(),
            );
            agents.write().await.push(agent);
        }

        // Initialize ANN daemon
        let ann_config = crate::ann_daemon::AnnConfig {
            enabled: config.ann_enabled,
        };
        let ann_daemon = Arc::new(crate::ann_daemon::AnnDaemon::new(ann_config).await?);

        // Initialize threat monitor
        let monitor = Arc::new(RwLock::new(ThreatMonitor::new(
            agents.clone(),
            crystal.clone(),
            sdt.clone(),
            plasma.clone(),
            plasma_bus.clone(),
        )));

        // Initialize server
        let server = PlasmaDefenderServer::new(&config, plasma.clone(), plasma_bus.clone()).await?;

        // Initialize health and metrics
        let health = HealthMonitor::new(plasma.clone());
        let metrics = MetricsCollector::new(plasma.clone());

        Ok(Self {
            server,
            health,
            metrics,
            agents,
            crystal,
            sdt,
            monitor,
            plasma,
            plasma_bus,
            ann_daemon,
            config,
        })
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        // Start threat monitor
        let monitor = self.monitor.clone();
        tokio::spawn(async move {
            let mut monitor = monitor.write().await;
            if let Err(e) = monitor.run().await {
                tracing::error!("Threat monitor error: {}", e);
            }
        });

        // Subscribe to tool results from Kali Plasma
        // Spawn subscription in a separate task
        let plasma_bus = self.plasma_bus.clone();
        let ann_daemon_clone = self.ann_daemon.clone();
        tokio::spawn(async move {
            if let Err(e) =
                crate::tool_handler::subscribe_tool_results_internal(plasma_bus, ann_daemon_clone)
                    .await
            {
                tracing::error!("Tool result subscription error: {}", e);
            }
        });

        // Start Axum server
        self.server.start(&self.config.bind_addr).await
    }

    pub fn get_health(&self) -> health::HealthStatus {
        self.health.get_status()
    }

    pub fn get_metrics(&self) -> metrics::Metrics {
        self.metrics.collect()
    }

    /// Evaluate threat through crystal resonance and SDT gate
    pub async fn evaluate_threat(&self, payload: &[u8]) -> anyhow::Result<ThreatResult> {
        let delta_angle = self.plasma.delta_angle_raw();

        // Evaluate through crystal
        let ring_strength = self.crystal.get_ring_strength(payload, delta_angle);

        // Check SDT gate
        let sdt_state = self.sdt.get_state();
        let allowed = self.sdt.should_proceed(ring_strength);

        // Emit telemetry
        self.plasma_bus
            .emit("plasma.defender.threat.evaluated", ring_strength as f64)
            .await?;

        Ok(ThreatResult {
            ring_strength,
            sdt_state,
            allowed,
            delta_angle,
            entropy: self.plasma.entropy(),
        })
    }

    /// Get current SDT gate state
    pub fn get_sdt_state(&self) -> sx9_atlas_bus::SdtState {
        self.sdt.get_state()
    }

    /// Get crystal ring strength
    pub fn get_ring_strength(&self, payload: &[u8]) -> f32 {
        let delta_angle = self.plasma.delta_angle_raw();
        self.crystal.get_ring_strength(payload, delta_angle)
    }
}

#[derive(Debug, Clone)]
pub struct ThreatResult {
    pub ring_strength: f32,
    pub sdt_state: sx9_atlas_bus::SdtState,
    pub allowed: bool,
    pub delta_angle: u16,
    pub entropy: u32,
}
