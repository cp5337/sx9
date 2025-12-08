//! Temporal Properties - Activation time, duration, decay
//! 
//! Core temporal constraints for cognitive atoms in Universal Cognigraph.

use serde::{Deserialize, Serialize};
use super::super::UniversalNodeType;

/// Temporal Properties (from Universal Cognigraph)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalProperties {
    pub activation_time: f32,   // t_a: Activation time ∈ ℝ⁺
    pub duration: f32,          // d: Duration ∈ ℝ⁺ ∪ {∞}
    pub decay: f32,            // τ: Decay/cooldown time ∈ ℝ⁺
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
