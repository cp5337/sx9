//! Crystal Resonance Integration
//!
//! Integrates with sx9-atlas-bus crystal system for resonance evaluation

use sx9_atlas_bus::{Crystal, CrystalFamily, Polycrystal, PolycrystalResult, VotingPolicy};
use std::sync::Arc;

/// Crystal Integration - Wrapper for sx9-atlas-bus crystal system
pub struct CrystalIntegration {
    polycrystal: Arc<Polycrystal>,
    crystal_family: CrystalFamily,
}

impl CrystalIntegration {
    pub fn new(family: CrystalFamily) -> Self {
        let mut polycrystal = Polycrystal::new(VotingPolicy::WeightedAverage);
        let _ = polycrystal.add(Crystal::new(family));
        Self {
            polycrystal: Arc::new(polycrystal),
            crystal_family: family,
        }
    }
    
    /// Evaluate payload through crystal resonance
    pub fn evaluate(&self, payload: &[u8], delta_angle: u16) -> PolycrystalResult {
        self.polycrystal.resonate_payload(payload, delta_angle)
    }
    
    /// Get ring strength for threat evaluation
    pub fn get_ring_strength(&self, payload: &[u8], delta_angle: u16) -> f32 {
        let result = self.evaluate(payload, delta_angle);
        result.ring_strength
    }
    
    /// Check if crystal passed (vote passed)
    pub fn passed(&self, payload: &[u8], delta_angle: u16) -> bool {
        let result = self.evaluate(payload, delta_angle);
        result.passed
    }
    
    /// Get crystal family
    pub fn family(&self) -> CrystalFamily {
        self.crystal_family
    }
    
    /// Get crystal instance for direct use
    pub fn crystal(&self) -> Crystal {
        Crystal::new(self.crystal_family)
    }
}

