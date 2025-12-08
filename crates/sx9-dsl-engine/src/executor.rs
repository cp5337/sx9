//! DSL Executor
//!
//! Orchestrates DSL execution with WASM runtime and file watching

use crate::dsl::{DSLEngine, DSLOperation, DSLResult};
use crate::wasm_runtime::WasmRuntime;
use crate::file_watcher::FileWatcher;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// DSL executor with hot reload support
pub struct DSLExecutor {
    engine: Arc<RwLock<DSLEngine>>,
    wasm_runtime: Arc<RwLock<Option<WasmRuntime>>>,
    file_watcher: Arc<RwLock<Option<FileWatcher>>>,
}

impl DSLExecutor {
    pub fn new(wasm_enabled: bool) -> Self {
        Self {
            engine: Arc::new(RwLock::new(DSLEngine::new(wasm_enabled))),
            wasm_runtime: Arc::new(RwLock::new(if wasm_enabled {
                Some(WasmRuntime::new().unwrap())
            } else {
                None
            })),
            file_watcher: Arc::new(RwLock::new(None)),
        }
    }

    /// Enable file watching for hot reload
    pub async fn enable_file_watching(&self, paths: Vec<String>) -> Result<()> {
        let mut watcher = FileWatcher::new()?;
        for path in paths {
            watcher.watch(&path)?;
        }
        *self.file_watcher.write().await = Some(watcher);
        Ok(())
    }

    /// Execute DSL operation
    pub async fn execute(&self, operation: &DSLOperation) -> DSLResult<String> {
        let engine = self.engine.read().await;
        engine.execute(operation).await
    }

    /// Parse and execute DSL string
    pub async fn parse_and_execute(&self, input: &str) -> DSLResult<String> {
        let engine = self.engine.read().await;
        let operation = engine.parse(input).await?;
        drop(engine);
        self.execute(&operation).await
    }

    /// Start hot reload watcher
    pub async fn start_hot_reload(&self) -> Result<()> {
        let watcher = self.file_watcher.clone();
        tokio::spawn(async move {
            let mut watcher = watcher.write().await;
            if let Some(ref mut w) = *watcher {
                while let Some(event) = w.next_event().await {
                    if w.is_dsl_change(&event) {
                        tracing::info!("DSL file changed: {:?}", event.paths);
                        // Trigger reload
                    }
                }
            }
        });
        Ok(())
    }
}

