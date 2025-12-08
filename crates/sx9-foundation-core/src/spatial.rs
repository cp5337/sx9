//! Spatial Properties - Interaction radius, exclusion radius, volume
//! 
//! Core spatial constraints for cognitive atoms in Universal Cognigraph.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use super::super::UniversalNodeType;

/// Spatial Properties (from Universal Cognigraph)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialProperties {
    pub interaction_radius: f32,    // r_i: Interaction radius ∈ ℝ⁺
    pub exclusion_radius: f32,      // r_e: Exclusion radius ∈ ℝ⁺
    pub volume: Vec3,               // V_o: Occupied volume ∈ ℝ³
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
