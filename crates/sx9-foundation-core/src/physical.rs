//! Physical Properties - Mass, resource cost, energy footprint
//! 
//! Core physical constraints for cognitive atoms in Universal Cognigraph.

use serde::{Deserialize, Serialize};
use super::super::UniversalNodeType;

/// Physical Properties (from Universal Cognigraph)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalProperties {
    pub mass: f32,              // m: Mass/resource requirement ∈ ℝ⁺
    pub resource_cost: f32,     // c_r: Resource consumption rate ∈ ℝ⁺  
    pub energy_footprint: f32,  // e_f: Energy footprint ∈ ℝ⁺
}

impl PhysicalProperties {
    /// Create default physical properties optimized for specific node type
    pub fn default_for_type(node_type: &UniversalNodeType) -> Self {
        match node_type {
            UniversalNodeType::Source => Self { 
                mass: 1.0, 
                resource_cost: 0.1, 
                energy_footprint: 0.5 
            },
            UniversalNodeType::Sink => Self { 
                mass: 2.0, 
                resource_cost: 0.0, 
                energy_footprint: 0.2 
            },
            UniversalNodeType::Transformer => Self { 
                mass: 1.5, 
                resource_cost: 0.3, 
                energy_footprint: 0.8 
            },
            UniversalNodeType::Router => Self { 
                mass: 0.8, 
                resource_cost: 0.15, 
                energy_footprint: 0.3 
            },
            UniversalNodeType::Buffer => Self { 
                mass: 3.0, 
                resource_cost: 0.05, 
                energy_footprint: 0.1 
            },
            UniversalNodeType::Gate => Self { 
                mass: 0.5, 
                resource_cost: 0.02, 
                energy_footprint: 0.1 
            },
            UniversalNodeType::Monitor => Self { 
                mass: 0.3, 
                resource_cost: 0.08, 
                energy_footprint: 0.2 
            },
            UniversalNodeType::Catalyst => Self { 
                mass: 0.6, 
                resource_cost: 0.25, 
                energy_footprint: 0.6 
            },
            UniversalNodeType::Inhibitor => Self { 
                mass: 1.2, 
                resource_cost: 0.2, 
                energy_footprint: 0.4 
            },
            UniversalNodeType::Relay => Self { 
                mass: 0.7, 
                resource_cost: 0.12, 
                energy_footprint: 0.3 
            },
        }
    }
    
    /// Calculate total resource requirement
    pub fn total_resource_requirement(&self) -> f32 {
        self.mass + self.resource_cost + self.energy_footprint
    }
    
    /// Create custom physical properties
    pub fn new(mass: f32, resource_cost: f32, energy_footprint: f32) -> Self {
        Self {
            mass,
            resource_cost,
            energy_footprint,
        }
    }
    
    /// Check if properties are within acceptable limits
    pub fn is_valid(&self) -> bool {
        self.mass > 0.0 && 
        self.resource_cost >= 0.0 && 
        self.energy_footprint > 0.0
    }
    
    /// Scale properties by factor (for different operational contexts)
    pub fn scaled(&self, factor: f32) -> Self {
        Self {
            mass: self.mass * factor,
            resource_cost: self.resource_cost * factor,
            energy_footprint: self.energy_footprint * factor,
        }
    }
}
