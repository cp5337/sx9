//! sx9-dsl-engine - DSL Symbolic Control Engine
//!
//! Provides DSL execution engine with WASM runtime integration,
//! file watching, and hot reload capabilities.

pub mod dsl;
pub mod wasm_runtime;
pub mod file_watcher;
pub mod executor;

pub use dsl::{DSLEngine, DSLOperation, DSLResult, DSLError};
pub use executor::DSLExecutor;
pub use wasm_runtime::WasmRuntime;
pub use file_watcher::FileWatcher;

use anyhow::Result;

/// DSL Engine configuration
#[derive(Debug, Clone, Default)]
pub struct DSLConfig {
    pub wasm_enabled: bool,
    pub hot_reload: bool,
    pub watch_paths: Vec<String>,
}

impl DSLConfig {
    pub fn default() -> Self {
        Self {
            wasm_enabled: true,
            hot_reload: true,
            watch_paths: vec!["playbooks/".to_string()],
        }
    }
}
