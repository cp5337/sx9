//! SX9 Language Server Protocol Implementation
//!
//! Provides IDE integration for:
//! - SX9 workflow files
//! - RFC documents
//! - Agent and skill definitions
//! - CLSGS N-V-N-N annotations
//!
//! ## Features
//!
//! - **Completion**: Agent names, skill IDs, RFC references
//! - **Hover**: Documentation for agents, skills, and types
//! - **Go to Definition**: Navigate to skill definitions
//! - **Diagnostics**: Validate N-V-N-N patterns, skill schemas
//! - **Code Actions**: Quick fixes and refactorings

pub mod backend;
pub mod capabilities;
pub mod completion;
pub mod diagnostics;
pub mod hover;
pub mod document;
pub mod rfc;

pub use backend::Sx9Backend;
pub use capabilities::server_capabilities;
