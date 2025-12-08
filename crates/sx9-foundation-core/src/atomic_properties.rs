//! Atomic Properties - Six Universal Properties from Universal Cognigraph
//! 
//! Implements Physical, Temporal, Energetic, Spatial, Relational, and Economic properties
//! that parameterize cognitive atoms according to Universal Cognigraph theory.

use bevy::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use super::UniversalNodeType;

/// Physical Properties (from Universal Cognigraph)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalProperties {
    pub mass: f32,              // m: Mass/resource requirement ∈ ℝ⁺
    pub resource_cost: f32,     // c_r: Resource consumption rate ∈ ℝ⁺  
    pub energy_footprint: f32,  // e_f: Energy footprint ∈ ℝ⁺
}

/// Temporal Properties (from Universal Cognigraph)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalProperties {
    pub activation_time: f32,   // t_a: Activation time ∈ ℝ⁺
    pub duration: f32,          // d: Duration ∈ ℝ⁺ ∪ {∞}
    pub decay: f32,            // τ: Decay/cooldown time ∈ ℝ⁺
}

/// Energetic Properties (from Universal Cognigraph) 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergeticProperties {
    pub consumption: f32,       // e_c: Energy consumption rate ∈ ℝ⁺
    pub generation: f32,        // e_g: Energy generation rate ∈ ℝ⁺
    pub threshold: f32,         // e_t: Activation threshold ∈ ℝ⁺
}

/// Spatial Properties (from Universal Cognigraph)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialProperties {
    pub interaction_radius: f32,    // r_i: Interaction radius ∈ ℝ⁺
    pub exclusion_radius: f32,      // r_e: Exclusion radius ∈ ℝ⁺
    pub volume: Vec3,               // V_o: Occupied volume ∈ ℝ³
}

/// Relational Properties (from Universal Cognigraph)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationalProperties {
    pub connectivity: u32,          // κ: Connectivity capacity ∈ ℕ
    pub dependencies: Vec<Uuid>,    // δ: Dependency vector
    pub interaction_matrix: HashMap<Uuid, f32>, // ι: Interaction strength matrix
}

/// Economic Properties (from Universal Cognigraph)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicProperties {
    pub setup_cost: f32,        // c_s: Setup cost ∈ ℝ⁺
    pub maintenance_cost: f32,  // c_m: Maintenance cost rate ∈ ℝ⁺
    pub opportunity_cost: f32,  // c_o: Opportunity cost ∈ ℝ⁺
    pub depreciation_rate: f32, // c_d: Depreciation rate ∈ [0,1]
}

// Default implementations optimized for each Universal Node Type

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
}

impl TemporalProperties {
    /// Create default temporal properties optimized for specific node type
    pub fn default_for_type(node_type: &UniversalNodeType) -> Self {
        match node_type {
            UniversalNodeType::Source => Self { 
                activation_time: 0.1, 
                duration: f32::INFINITY, 
                decay: 0.2 
            },
            UniversalNodeType::Sink => Self { 
                activation_time: 0.0, 
                duration: f32::INFINITY, 
                decay: 0.1 
            },
            UniversalNodeType::Transformer => Self { 
                activation_time: 0.3, 
                duration: 2.0, 
                decay: 1.0 
            },
            UniversalNodeType::Router => Self { 
                activation_time: 0.05, 
                duration: f32::INFINITY, 
                decay: 0.1 
            },
            UniversalNodeType::Buffer => Self { 
                activation_time: 0.1, 
                duration: f32::INFINITY, 
                decay: 0.5 
            },
            UniversalNodeType::Gate => Self { 
                activation_time: 0.0, 
                duration: 1.0, 
                decay: 0.1 
            },
            UniversalNodeType::Monitor => Self { 
                activation_time: 0.05, 
                duration: f32::INFINITY, 
                decay: 0.2 
            },
            UniversalNodeType::Catalyst => Self { 
                activation_time: 0.15, 
                duration: 3.0, 
                decay: 0.8 
            },
            UniversalNodeType::Inhibitor => Self { 
                activation_time: 0.2, 
                duration: 5.0, 
                decay: 1.5 
            },
            UniversalNodeType::Relay => Self { 
                activation_time: 0.1, 
                duration: f32::INFINITY, 
                decay: 0.3 
            },
        }
    }
    
    /// Check if node has persistent duration
    pub fn is_persistent(&self) -> bool {
        self.duration == f32::INFINITY
    }
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
}

impl SpatialProperties {
    /// Create spatial properties with specified position
    pub fn new(position: Vec3) -> Self {
        Self {
            interaction_radius: 10.0,
            exclusion_radius: 2.0,
            volume: position,
        }
    }
    
    /// Create optimized spatial properties for node type
    pub fn for_node_type(node_type: &UniversalNodeType, position: Vec3) -> Self {
        let (interaction_radius, exclusion_radius) = match node_type {
            UniversalNodeType::Source => (15.0, 1.0),      // Wide reach, low exclusion
            UniversalNodeType::Sink => (8.0, 3.0),        // Limited reach, moderate exclusion
            UniversalNodeType::Transformer => (10.0, 2.0), // Balanced
            UniversalNodeType::Router => (20.0, 1.5),     // Long reach for routing
            UniversalNodeType::Buffer => (5.0, 4.0),      // Short reach, high exclusion
            UniversalNodeType::Gate => (3.0, 2.0),        // Controlled access
            UniversalNodeType::Monitor => (25.0, 0.5),    // Very wide observation
            UniversalNodeType::Catalyst => (12.0, 1.0),   // Good reach for catalysis
            UniversalNodeType::Inhibitor => (8.0, 3.0),   // Limited but blocking
            UniversalNodeType::Relay => (30.0, 1.0),      // Maximum reach extension
        };
        
        Self {
            interaction_radius,
            exclusion_radius,
            volume: position,
        }
    }
    
    /// Check if two nodes can spatially interact
    pub fn can_interact_with(&self, other: &SpatialProperties) -> bool {
        let distance = self.volume.distance(other.volume);
        distance <= self.interaction_radius.max(other.interaction_radius) &&
        distance >= self.exclusion_radius + other.exclusion_radius
    }
}

impl Default for RelationalProperties {
    fn default() -> Self {
        Self {
            connectivity: 5,
            dependencies: Vec::new(),
            interaction_matrix: HashMap::new(),
        }
    }
}

impl RelationalProperties {
    /// Add dependency relationship
    pub fn add_dependency(&mut self, target_id: Uuid, strength: f32) {
        if !self.dependencies.contains(&target_id) {
            self.dependencies.push(target_id);
        }
        self.interaction_matrix.insert(target_id, strength);
    }
    
    /// Check if at connectivity limit
    pub fn is_at_capacity(&self) -> bool {
        self.dependencies.len() >= self.connectivity as usize
    }
}

impl Default for EconomicProperties {
    fn default() -> Self {
        Self {
            setup_cost: 1.0,
            maintenance_cost: 0.1,
            opportunity_cost: 0.2,
            depreciation_rate: 0.05,
        }
    }
}

impl EconomicProperties {
    /// Calculate total cost over time period
    pub fn total_cost_over_time(&self, time_period: f32) -> f32 {
        self.setup_cost + 
        (self.maintenance_cost * time_period) +
        (self.opportunity_cost * time_period) +
        (self.depreciation_rate * time_period)
    }
}
