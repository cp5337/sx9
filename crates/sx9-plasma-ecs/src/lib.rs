//! sx9-plasma-ecs - PLASMA-ECS Unified Architecture
//!
//! Integrates Legion (Layer 2) and apecs (Layer 1) ECS systems
//! for Plasma state management. Includes ANN observer layer for
//! neural network pattern recognition (RFC-9114 Rev 1.1).

pub mod components;
pub mod systems;
pub mod world;
pub mod legion_layer;
pub mod apecs_layer;
pub mod ann_layer;

pub use components::*;
pub use systems::*;
pub use world::PlasmaWorld;
pub use legion_layer::LegionPlasmaWorld;
pub use apecs_layer::ApecsPlasmaWorld;
pub use ann_layer::{AnnObserverWorld, AnnConfig};

use anyhow::Result;

/// PLASMA-ECS configuration
#[derive(Debug, Clone, Default)]
pub struct PlasmaEcsConfig {
    pub use_legion: bool,
    pub use_apecs: bool,
}

impl PlasmaEcsConfig {
    pub fn default() -> Self {
        Self {
            use_legion: true,
            use_apecs: true,
        }
    }
}
