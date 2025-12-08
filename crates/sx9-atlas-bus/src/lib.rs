//! # SX9 ATLAS Bus
//!
//! Zero-allocation, lock-free ring buffer for ATLAS daemon ↔ apecs IPC.
//!
//! ## Design Goals
//! - Sub-10ns push/pop latency
//! - Cache-line aligned to prevent false sharing
//! - Priority lanes (critical/urgent/normal)
//! - Plasma state integration
//! - SDT gate control
//! - WASM compatible (no-std optional)
//!
//! ## Usage
//! ```rust,ignore
//! use sx9_atlas_bus::{AtlasBus, Command, CommandKind};
//!
//! // Create bus (large, typically static or boxed)
//! static BUS: AtlasBus = AtlasBus::new();
//!
//! // Enable SDT gate
//! BUS.plasma().prime();
//! BUS.plasma().trigger(0);
//!
//! // Dispatch command (auto-routes by priority)
//! BUS.dispatch(Command::new(CommandKind::Dijkstra { src: 0, dst: 42, max_hops: 10 }));
//!
//! // Legion tick - drain all pending
//! for cmd in BUS.tick() {
//!     match cmd.kind {
//!         CommandKind::Dijkstra { src, dst, .. } => { /* SIMD path */ }
//!         _ => {}
//!     }
//! }
//! ```

#![cfg_attr(all(not(feature = "std"), not(feature = "nats"), not(test)), no_std)]

#[cfg(any(test, feature = "nats"))]
extern crate std;

#[cfg(all(not(feature = "std"), not(feature = "nats"), not(test)))]
extern crate alloc;

mod ring;
mod command;
mod result;
mod bus;
mod plasma;
pub mod crystal;

pub use ring::Ring;
pub use command::{Command, CommandKind};
pub use result::{AtlasResult, ResultKind};
pub use bus::{AtlasBus, DispatchResult, BusStats};
pub use plasma::{PlasmaState, PlasmaSnapshot, SdtState, ThyristorConfig};
pub use crystal::{
    Crystal, CrystalFamily, ResonanceProfile, DeltaClass,
    VotingPolicy, Polycrystal, PolycrystalResult, MAX_CRYSTALS,
};

#[cfg(feature = "nats")]
pub mod bridge;

