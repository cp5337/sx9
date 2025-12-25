//! NATS Integration for SX9 Harness
//!
//! Message bus communication layer for FORGE Agent System.
//! Subjects mirror Redux action types for frontend consistency.

pub mod heartbeat_emitter;
pub mod subjects;

pub use heartbeat_emitter::{HeartbeatEmitter, HeartbeatOrchestrator};
pub use subjects::*;
