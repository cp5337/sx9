//! Threat Monitoring System
//!
//! Orchestrates threat agents → crystal → SDT gate flow

use crate::agents::{ThreatAgent, ThreatEvent};
use crate::crystal::CrystalIntegration;
use crate::plasma_bus::PlasmaBus;
use crate::sdt::SdtIntegration;
use std::sync::Arc;
use sx9_atlas_bus::PlasmaState;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

pub struct ThreatMonitor {
    agents: Arc<RwLock<Vec<ThreatAgent>>>,
    crystal: Arc<CrystalIntegration>,
    sdt: Arc<SdtIntegration>,
    plasma: Arc<PlasmaState>,
    plasma_bus: Arc<PlasmaBus>,
    monitor_interval: Duration,
    tick: u64,
}

impl ThreatMonitor {
    pub fn new(
        agents: Arc<RwLock<Vec<ThreatAgent>>>,
        crystal: Arc<CrystalIntegration>,
        sdt: Arc<SdtIntegration>,
        plasma: Arc<PlasmaState>,
        plasma_bus: Arc<PlasmaBus>,
    ) -> Self {
        Self {
            agents,
            crystal,
            sdt,
            plasma,
            plasma_bus,
            monitor_interval: Duration::from_millis(100), // 100ms tick
            tick: 0,
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let mut ticker = interval(self.monitor_interval);

        loop {
            ticker.tick().await;
            self.tick += 1;

            // Monitor threats from agents
            let mut agents = self.agents.write().await;
            for agent in agents.iter_mut() {
                if let Ok(Some(event)) = agent.monitor().await {
                    // Evaluate through crystal
                    let ring_strength = self
                        .crystal
                        .get_ring_strength(&event.payload, self.plasma.delta_angle_raw());

                    // Resonate through SDT gate
                    let crystal = self.crystal.crystal();
                    let allowed = self.sdt.resonate(&event.payload, &crystal, self.tick);

                    // Evaluate load (simulated from ring strength)
                    let load = ring_strength as f64;
                    self.sdt.evaluate_load(load, &self.plasma_bus).await?;

                    if allowed {
                        // Threat detected and allowed by SDT
                        self.handle_threat(event).await?;
                    } else {
                        // SDT gate blocked - threat killed
                        self.block_threat(event).await?;
                    }
                }
            }
        }
    }

    async fn handle_threat(&self, event: ThreatEvent) -> anyhow::Result<()> {
        tracing::warn!("Threat detected and allowed: {:?}", event);
        self.plasma_bus
            .emit(
                "plasma.defender.threat.allowed",
                event.severity as u8 as f64,
            )
            .await?;
        // Handle threat (alert, log, etc.)
        Ok(())
    }

    async fn block_threat(&self, event: ThreatEvent) -> anyhow::Result<()> {
        tracing::info!("Threat blocked by SDT gate: {:?}", event);
        self.plasma_bus
            .emit(
                "plasma.defender.threat.blocked",
                event.severity as u8 as f64,
            )
            .await?;
        // Block threat (log, metrics, etc.)
        Ok(())
    }
}
