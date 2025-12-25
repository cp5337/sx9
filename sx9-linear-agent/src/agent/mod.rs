//! Agent Module
//!
//! Autonomous agent system for Linear issue processing.
//! Implements the agent loop pattern from RFC-9141.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                         AGENT LOOP                                       │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │                                                                          │
//! │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                 │
//! │  │ Initializer │───▶│   Coder     │───▶│  Handoff    │                 │
//! │  │   Agent     │    │   Agent     │    │   Agent     │                 │
//! │  └─────────────┘    └─────────────┘    └─────────────┘                 │
//! │        │                  │                  │                          │
//! │        │ Creates          │ Implements       │ Saves State             │
//! │        ▼                  ▼                  ▼                          │
//! │  ┌───────────────────────────────────────────────────────────────────┐ │
//! │  │                         LINEAR                                     │ │
//! │  │    Initiative → Project → Issue (Atomic Prompt Unit)              │ │
//! │  └───────────────────────────────────────────────────────────────────┘ │
//! │                              │                                          │
//! │                              ▼                                          │
//! │  ┌───────────────────────────────────────────────────────────────────┐ │
//! │  │                      QA GATES                                      │ │
//! │  │   Static → Arch → Pattern → Semantic → Certification              │ │
//! │  └───────────────────────────────────────────────────────────────────┘ │
//! │                                                                          │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```

pub mod agent_loop;
pub mod coder;
pub mod handoff;
pub mod initializer;
pub mod types;

pub use agent_loop::AgentLoop;
pub use coder::CoderAgent;
pub use handoff::HandoffAgent;
pub use initializer::InitializerAgent;
pub use types::*;

use anyhow::Result;

use crate::linear::Client as LinearClient;
use crate::mcp::{SerenaClient, SlackMCP};

/// Run the agent system
pub async fn run(
    linear: LinearClient,
    slack: SlackMCP,
    serena: SerenaClient,
) -> Result<()> {
    let agent_loop = AgentLoop::new(linear, slack, serena);
    agent_loop.run().await
}
