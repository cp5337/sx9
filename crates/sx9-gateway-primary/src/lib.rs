//! # SX9 Gateway
//!
//! The unified WebSocket gateway for the SYNAPTIX9 cognitive engine.
//!
//! This crate provides the **only new code needed** to expose the existing
//! SX9 infrastructure (sx9-atlas-bus, databases, PlasmaState) to the UI.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    WORKBENCH UI (React)                          │
//! └───────────────────────────┬─────────────────────────────────────┘
//!                             │ WebSocket
//!                             ▼
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    sx9-gateway (THIS CRATE)                      │
//! │                                                                  │
//! │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐           │
//! │  │  Query   │ │  Graph   │ │ Workflow │ │  Health  │           │
//! │  │ Handler  │ │ Handler  │ │ Handler  │ │ Handler  │           │
//! │  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘           │
//! └───────┼────────────┼────────────┼────────────┼──────────────────┘
//!         │            │            │            │
//!         ▼            ▼            ▼            ▼
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                 EXISTING SX9 INFRASTRUCTURE                      │
//! │                                                                  │
//! │  Supabase │ Neon │ Sled │ Sledis │ NATS │ sx9-atlas-bus      │
//! └─────────────────────────────────────────────────────────────────┘
//! ```

pub mod glaf_client;
pub mod handlers;
pub mod licensing;
pub mod protocol;
pub mod server;
pub mod state;

pub use licensing::{LicenseTier, Subscription, ComponentMeta, LicenseValidation};
pub use protocol::{Database, WsMessage, WsResponse};
pub use server::{run_gateway, DEFAULT_PORT};
pub use state::GatewayState;
