//! Integration Bridges for Plasma Defender
//!
//! Connects Plasma-Defender to external intelligence systems:
//! - EEI Bridge: Essential Elements of Information queries via Leptose
//! - SlotGraph Bridge: Graph-based entity relationships
//!
//! Communication via NATS for loose coupling.

pub mod eei_bridge;
pub mod slot_bridge;

pub use eei_bridge::{EeiBridge, EeiQueryRequest, EeiQueryResponse};
pub use slot_bridge::{SlotBridge, SlotEntity, SlotQuery};
