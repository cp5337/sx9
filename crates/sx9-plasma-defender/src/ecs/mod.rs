//! ECS Integration for Plasma Defender
//!
//! RFC-9116: Three-layer ECS stack for threat entity management
//! - Layer 1: apecs (Async I/O, cold-path)
//! - Layer 2: Legion (Hot-path, deterministic batch processing)
//! - Layer 3: ATLAS (Cognitive, 1ms OODA loop) - separate module
//!
//! This is Plasma-Defender's isolated ECS - not shared with sx9-plasma-ecs

pub mod apecs_layer;
pub mod components;
pub mod legion_layer;
pub mod systems;
pub mod world;

pub use apecs_layer::DefenderApecsWorld;
pub use components::*;
pub use legion_layer::DefenderLegionWorld;
pub use systems::*;
pub use world::DefenderWorld;
