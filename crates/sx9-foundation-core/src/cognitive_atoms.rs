//! Cognitive Atoms - Core implementation of Universal Cognigraph theory
//! 
//! Maps cognitive atoms to Bevy ECS entities with full atomic properties.
//! Implements the foundational CognitiveAtom component and its core behaviors.

use bevy::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use super::{UniversalNodeType, CTASNodeType, *};

/// Universal Cognitive Atom - Core implementation of Universal Cognigraph theory
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveAtom {
    pub atom_id: Uuid,
    pub atomic_number: u8, // 1-10 for universal node types
    pub node_type: UniversalNodeType,
    pub ctas_node_type: Option<CTASNodeType>,
    
    // Six Universal Properties from Cognigraph Paper
    pub physical: PhysicalProperties,
    pub temporal: TemporalProperties, 
    pub energetic: EnergeticProperties,
    pub spatial: SpatialProperties,
    pub relational: RelationalProperties,
    pub economic: EconomicProperties,
    
    // Operational state
    pub activation_state: ActivationState,
    pub interaction_history: Vec<InteractionEvent>,
}

/// Activation state for cognitive atoms
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActivationState {
    Dormant,        // Below threshold
    Primed,         // At threshold, ready for activation
    Active,         // Currently executing function
    Exhausted,      // In decay/cooldown period
    Blocked,        // Prevented from activation
}

/// Interaction event between cognitive atoms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionEvent {
    pub event_id: Uuid,
    pub timestamp: f64,
    pub source_atom: Uuid,
    pub target_atom: Uuid,
    pub interaction_type: InteractionType,
    pub energy_transfer: f32,
    pub information_payload: HashMap<String, String>,
}

/// Types of atomic interactions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InteractionType {
    EnergyTransfer,     // Direct energy flow
    InformationFlow,    // Data/signal transmission
    ResourceExchange,   // Material/resource transfer
    ControlSignal,      // Command/control interaction
    FeedbackLoop,       // Status/feedback information
    Synchronization,    // Timing coordination
    Inhibition,         // Blocking/throttling
    Catalysis,          // Acceleration/enhancement
}

impl CognitiveAtom {
    /// Create a new cognitive atom with Universal Cognigraph properties
    pub fn new(
        node_type: UniversalNodeType,
        ctas_type: Option<CTASNodeType>,
        position: Vec3,
    ) -> Self {
        let atomic_number = match node_type {
            UniversalNodeType::Source => 1,
            UniversalNodeType::Sink => 2,
            UniversalNodeType::Transformer => 3,
            UniversalNodeType::Router => 4,
            UniversalNodeType::Buffer => 5,
            UniversalNodeType::Gate => 6,
            UniversalNodeType::Monitor => 7,
            UniversalNodeType::Catalyst => 8,
            UniversalNodeType::Inhibitor => 9,
            UniversalNodeType::Relay => 10,
        };

        Self {
            atom_id: Uuid::new_v4(),
            atomic_number,
            node_type: node_type.clone(),
            ctas_node_type: ctas_type,
            physical: PhysicalProperties::default_for_type(&node_type),
            temporal: TemporalProperties::default_for_type(&node_type),
            energetic: EnergeticProperties::default_for_type(&node_type),
            spatial: SpatialProperties::new(position),
            relational: RelationalProperties::default(),
            economic: EconomicProperties::default(),
            activation_state: ActivationState::Dormant,
            interaction_history: Vec::new(),
        }
    }
    
    /// Check if atom can interact with another atom based on Universal Cognigraph rules
    pub fn can_interact_with(&self, other: &CognitiveAtom, distance: f32) -> bool {
        // Check spatial constraints
        if distance > self.spatial.interaction_radius.max(other.spatial.interaction_radius) {
            return false;
        }
        
        // Check energetic constraints  
        if self.energetic.consumption > self.energetic.generation && 
           other.energetic.generation < self.energetic.consumption {
            return false;
        }
        
        // Check activation state
        matches!(self.activation_state, ActivationState::Active | ActivationState::Primed) &&
        matches!(other.activation_state, ActivationState::Active | ActivationState::Primed)
    }
    
    /// Update atom state based on time and interactions
    pub fn update_state(&mut self, delta_time: f32) {
        match self.activation_state {
            ActivationState::Active => {
                // Check if duration exceeded
                if delta_time > self.temporal.duration {
                    self.activation_state = ActivationState::Exhausted;
                }
            }
            ActivationState::Exhausted => {
                // Check if decay period completed
                if delta_time > self.temporal.decay {
                    self.activation_state = ActivationState::Dormant;
                }
            }
            ActivationState::Primed => {
                // Check activation threshold
                if self.energetic.generation >= self.energetic.threshold {
                    self.activation_state = ActivationState::Active;
                }
            }
            _ => {}
        }
    }
    
    /// Record interaction with another atom
    pub fn record_interaction(&mut self, event: InteractionEvent) {
        self.interaction_history.push(event);
        
        // Keep history manageable (last 50 interactions)
        if self.interaction_history.len() > 50 {
            self.interaction_history.remove(0);
        }
    }
}

/// Bundle for spawning complete cognitive atoms in Bevy ECS
/// Note: Transform components should be handled separately via PbrBundle or other bundles
#[derive(Bundle)]
pub struct CognitiveAtomBundle {
    pub atom: CognitiveAtom,
}

impl CognitiveAtomBundle {
    pub fn new(atom: CognitiveAtom) -> Self {
        Self {
            atom,
        }
    }
    
    /// Helper to get the spatial position for transform
    pub fn get_position(&self) -> Vec3 {
        self.atom.spatial.volume
    }
    
    /// Create a cognitive atom bundle from universal node type
    pub fn from_universal_type(
        node_type: UniversalNodeType,
        position: Vec3,
    ) -> Self {
        let atom = CognitiveAtom::new(node_type, None, position);
        Self::new(atom)
    }
    
    /// Create a cognitive atom bundle with CTAS domain type
    pub fn from_ctas_type(
        universal_type: UniversalNodeType,
        ctas_type: CTASNodeType,
        position: Vec3,
    ) -> Self {
        let atom = CognitiveAtom::new(universal_type, Some(ctas_type), position);
        Self::new(atom)
    }
}
