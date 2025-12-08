//! SDT Integration with Load-Based Gating
//!
//! Software-Defined Thyristor controller with threshold-based gating

use sx9_atlas_bus::{PlasmaState, SdtState, ThyristorConfig};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::plasma_bus::PlasmaBus;
use tracing::{info, warn};

/// SDT Controller with load-based threshold gating
pub struct SdtIntegration {
    plasma: Arc<PlasmaState>,
    config: ThyristorConfig,
    threshold: f64,
    gated: Arc<RwLock<bool>>,
}

impl SdtIntegration {
    pub fn new(plasma: Arc<PlasmaState>, config: ThyristorConfig) -> Self {
        let threshold = config.gate_thresh as f64;
        Self {
            plasma,
            config,
            threshold,
            gated: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Get current SDT gate state
    pub fn get_state(&self) -> SdtState {
        self.plasma.sdt_state()
    }
    
    /// Evaluate load and update SDT gate state
    pub async fn evaluate_load(&self, load: f64, bus: &PlasmaBus) -> anyhow::Result<()> {
        let mut gated = self.gated.write().await;
        
        if load > self.threshold && !*gated {
            *gated = true;
            warn!("⚠️ SDT engaged: load {:.2} > threshold {:.2}", load, self.threshold);
            bus.emit("sdt.engaged", load).await?;
        } else if *gated && load < self.threshold * 0.8 {
            *gated = false;
            info!("✅ SDT released: load {:.2} < threshold {:.2}", load, self.threshold * 0.8);
            bus.emit("sdt.released", load).await?;
        }
        
        Ok(())
    }
    
    /// Check if SDT gate is currently gated
    pub async fn is_gated(&self) -> bool {
        *self.gated.read().await
    }
    
    /// Evaluate if command should proceed (crystal + SDT gate)
    pub fn should_proceed(&self, _ring_strength: f32) -> bool {
        // Check if SDT gate allows flow
        let state = self.get_state();
        matches!(state, SdtState::Conducting | SdtState::Latched)
    }
    
    /// Resonate payload through crystal and update SDT gate
    pub fn resonate(&self, payload: &[u8], crystal: &sx9_atlas_bus::Crystal, tick: u64) -> bool {
        self.plasma.resonate(crystal, payload, tick, &self.config)
    }
    
    /// Resonate through polycrystal and update SDT gate
    pub fn resonate_poly(
        &self,
        payload: &[u8],
        polycrystal: &sx9_atlas_bus::Polycrystal,
        tick: u64,
    ) -> (bool, sx9_atlas_bus::PolycrystalResult) {
        self.plasma.resonate_poly(polycrystal, payload, tick, &self.config)
    }
    
    /// Get SDT configuration
    pub fn config(&self) -> &ThyristorConfig {
        &self.config
    }
}

