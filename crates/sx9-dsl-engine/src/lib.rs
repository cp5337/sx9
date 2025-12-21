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
//! sx9-dsl-engine - DSL Symbolic Control Engine
//!
//! Provides DSL execution engine with WASM runtime integration,
//! file watching, and hot reload capabilities.

pub mod dsl;
pub mod executor;
pub mod file_watcher;
pub mod wasm_runtime;

pub use dsl::{DSLEngine, DSLError, DSLOperation, DSLResult};
pub use executor::DSLExecutor;
pub use file_watcher::FileWatcher;
pub use wasm_runtime::WasmRuntime;

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
