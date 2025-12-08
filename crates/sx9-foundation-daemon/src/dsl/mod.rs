//! Domain Specific Language (DSL) for CTAS-7 Operations
//!
//! Provides Rust procedural macros for L2 hash-driven Kali tool orchestration
//! and daemon node spawning, replacing verbose XSD/XML playbooks.
//!
//! ## Core DSL Macros
//!
//! - `hash_trigger!` - Generic hash-driven operation trigger
//! - `intel_collection!` - Intelligence collection operations
//! - `pentest_spawn!` - Penetration testing cluster spawning
//! - `ephemeral_asset!` - Ephemeral asset lifecycle management
//! - `node_interview!` - CTAS node interview execution
//! - `kali_tool!` - Kali tool orchestration
//! - `workflow!` - Sequential workflow composition
//! - `parallel!` - Parallel task execution
//! - `conditional!` - Conditional execution based on hashes
//!
//! ## Architecture
//!
//! ```text
//! DSL Macros (compile-time)
//!     ↓
//! Hash Classification (MurmurHash3)
//!     ↓
//! Neural Mux Routing (<250ns)
//!     ↓
//! L2 Cognitive Decision Plane
//!     ↓
//! ATLAS Daemon Orchestration
//! ```

pub mod parser;
pub mod operations;
pub mod hash_classifier;
pub mod hash_extractor;

pub use parser::*;
pub use operations::*;
pub use hash_classifier::*;
pub use hash_extractor::*;

// New modules for Canonical Escalatory Action System
pub mod unicode_registry;
pub mod unicode_bridge;
pub mod playbook_unicode;
pub mod hash_unicode_bridge;
pub mod toml_unicode_compiler;
pub mod playbook_executor;

pub use unicode_registry::*;
pub use unicode_bridge::*;
pub use playbook_unicode::*;
pub use toml_unicode_compiler::*;
pub use playbook_executor::*;
pub use hash_unicode_bridge::*;

/// DSL version
pub const DSL_VERSION: &str = "7.3.1";

/// Maximum hash length (48 characters for SCH-CUID-UUID)
pub const MAX_HASH_LENGTH: usize = 48;

/// DSL operation result
pub type DSLResult<T> = Result<T, DSLError>;

/// DSL error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DSLError {
    /// Invalid hash format
    InvalidHash(String),
    /// Invalid operation parameters
    InvalidParameters(String),
    /// Hash extraction failed
    HashExtractionFailed(String),
    /// Neural Mux routing failed
    RoutingFailed(String),
    /// Execution failed
    ExecutionFailed(String),
    /// Unknown operation type
    UnknownOperation(String),
}

impl std::fmt::Display for DSLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DSLError::InvalidHash(msg) => write!(f, "Invalid hash: {}", msg),
            DSLError::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
            DSLError::HashExtractionFailed(msg) => write!(f, "Hash extraction failed: {}", msg),
            DSLError::RoutingFailed(msg) => write!(f, "Routing failed: {}", msg),
            DSLError::ExecutionFailed(msg) => write!(f, "Execution failed: {}", msg),
            DSLError::UnknownOperation(msg) => write!(f, "Unknown operation: {}", msg),
        }
    }
}

impl std::error::Error for DSLError {}

impl From<String> for DSLError {
    fn from(s: String) -> Self {
        DSLError::ExecutionFailed(s)
    }
}




