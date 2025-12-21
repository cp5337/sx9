#![allow(clippy::items_after_statements)]
#![allow(clippy::format_push_string)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::unused_async)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::similar_names)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::unused_self)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]
//! sx9-plasma-ecs - PLASMA-ECS Unified Architecture
//!
//! Integrates Legion (Layer 2) and apecs (Layer 1) ECS systems
//! for Plasma state management. Includes ANN observer layer for
//! neural network pattern recognition (RFC-9114 Rev 1.1).

pub mod ann_layer;
pub mod apecs_layer;
pub mod components;
pub mod legion_layer;
pub mod systems;
pub mod world;

pub use ann_layer::{AnnConfig, AnnObserverWorld};
pub use apecs_layer::ApecsPlasmaWorld;
pub use components::*;
pub use legion_layer::LegionPlasmaWorld;
pub use systems::*;
pub use world::PlasmaWorld;

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
