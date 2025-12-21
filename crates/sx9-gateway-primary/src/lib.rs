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
//! │  Supabase │ SurrealDB │ Sled │ Sledis │ NATS │ sx9-atlas-bus   │
//! └─────────────────────────────────────────────────────────────────┘
//! ```

pub mod glaf_client;
pub mod handlers;
pub mod protocol;
pub mod server;
pub mod state;

pub use protocol::{Database, WsMessage, WsResponse};
pub use server::{run_gateway, DEFAULT_PORT};
pub use state::GatewayState;
