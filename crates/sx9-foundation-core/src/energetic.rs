//! Energetic Properties - Energy consumption, generation, and thresholds
//! 
//! Core energetic constraints for cognitive atoms in Universal Cognigraph.

use serde::{Deserialize, Serialize};
use super::super::UniversalNodeType;

/// Energetic Properties (from Universal Cognigraph) 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergeticProperties {
    pub consumption: f32,       // e_c: Energy consumption rate ∈ ℝ⁺
    pub generation: f32,        // e_g: Energy generation rate ∈ ℝ⁺
    pub threshold: f32,         // e_t: Activation threshold ∈ ℝ⁺
}

impl EnergeticProperties {
    /// Create default energetic properties optimized for specific node type
    pub fn default_for_type(node_type: &UniversalNodeType) -> Self {
        match node_type {
            UniversalNodeType::Source => Self { 
                consumption: 0.1, 
                generation: 1.0, 
                threshold: 0.2 
            },
            UniversalNodeType::Sink => Self { 
                consumption: 0.5, 
                generation: 0.0, 
                threshold: 0.1 
            },
            UniversalNodeType::Transformer => Self { 
                consumption: 0.4, 
                generation: 0.3, 
                threshold: 0.3 
            },
            UniversalNodeType::Router => Self { 
                consumption: 0.2, 
                generation: 0.1, 
                threshold: 0.15 
            },
            UniversalNodeType::Buffer => Self { 
                consumption: 0.1, 
                generation: 0.05, 
                threshold: 0.1 
            },
            UniversalNodeType::Gate => Self { 
                consumption: 0.05, 
                generation: 0.0, 
                threshold: 0.5 
            },
            UniversalNodeType::Monitor => Self { 
                consumption: 0.15, 
                generation: 0.0, 
                threshold: 0.1 
            },
            UniversalNodeType::Catalyst => Self { 
                consumption: 0.2, 
                generation: 0.8, 
                threshold: 0.3 
            },
            UniversalNodeType::Inhibitor => Self { 
                consumption: 0.3, 
                generation: 0.1, 
                threshold: 0.4 
            },
            UniversalNodeType::Relay => Self { 
                consumption: 0.2, 
                generation: 0.2, 
                threshold: 0.2 
            },
        }
    }
    
    /// Calculate net energy balance
    pub fn net_energy_balance(&self) -> f32 {
        self.generation - self.consumption
    }
    
    /// Check if node is energy positive
    pub fn is_energy_positive(&self) -> bool {
        self.net_energy_balance() > 0.0
    }
    
    /// Check if node can activate given available energy
    pub fn can_activate(&self, available_energy: f32) -> bool {
        available_energy >= self.threshold
    }
    
    /// Create custom energetic properties
    pub fn new(consumption: f32, generation: f32, threshold: f32) -> Self {
        Self {
            consumption,
            generation,
            threshold,
        }
    }
    
    /// Get energy efficiency ratio (generation / consumption)
    pub fn efficiency_ratio(&self) -> f32 {
        if self.consumption > 0.0 {
            self.generation / self.consumption
        } else {
            f32::INFINITY // Perfect efficiency for zero consumption
        }
    }
    
    /// Calculate energy cost for operation duration
    pub fn energy_cost_for_duration(&self, duration: f32) -> f32 {
        self.consumption * duration
    }
    
    /// Check if properties are balanced for sustainable operation
    pub fn is_sustainable(&self) -> bool {
        self.is_energy_positive() || self.consumption == 0.0
    }
}
