//! Linear Gateway Service
//!
//! Unified Linear integration following RFC-9030.
//! Port: 18120

mod types;
mod gateway;
mod webhook;
mod dispatch;

pub use types::*;
pub use gateway::LinearGateway;
pub use webhook::WebhookHandler;
pub use dispatch::AgentDispatcher;
