//! Agent Registry Module
//!
//! Defines AI agents as "team members" for @ mention routing in Linear and Slack.
//! Per RFC-9030: Agents appear in team member lists for assignment and mentions.
//!
//! ## Skill System
//!
//! Agents execute skills - atomic units of capability with:
//! - Input/output JSON schemas for validation
//! - SLO (Service Level Objective) definitions
//! - Prerequisite skill chains
//! - Runtime discovery and composition

mod registry;
mod skills;
mod types;

pub use registry::*;
pub use skills::*;
pub use types::*;
