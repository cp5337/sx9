//! Linear Gateway Service
//!
//! Unified Linear integration following RFC-9030.
//! Port: 18120
//!
//! ## BNE Workflow (RFC-9141)
//!
//! Bar Napkin Engineering workflow transforms ideation into structured
//! Linear issues with scholarly references and PoC test specifications.

mod types;
mod gateway;
mod webhook;
mod dispatch;
mod bne;
mod zotero;
mod scholarly;

pub use types::*;
pub use gateway::LinearGateway;
pub use webhook::WebhookHandler;
pub use dispatch::AgentDispatcher;
pub use bne::*;
pub use zotero::ZoteroClient;
pub use scholarly::ScholarlyClient;
