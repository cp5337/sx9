//! Agent Registry Module
//!
//! Defines AI agents as "team members" for @ mention routing in Linear and Slack.
//! Per RFC-9030: Agents appear in team member lists for assignment and mentions.

mod registry;
mod types;

pub use registry::*;
pub use types::*;
