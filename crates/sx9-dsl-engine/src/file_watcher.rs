//! File Watcher for Hot Reload
//!
//! Watches DSL playbook files for changes and triggers reload

use anyhow::Result;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc;

/// File watcher for DSL hot reload
pub struct FileWatcher {
    watcher: RecommendedWatcher,
    event_rx: mpsc::Receiver<Result<Event, notify::Error>>,
}

impl FileWatcher {
    pub fn new() -> Result<Self> {
        let (tx, rx) = mpsc::channel(100);
        
        let watcher = notify::recommended_watcher(move |res| {
            let _ = tx.blocking_send(res);
        })?;

        Ok(Self {
            watcher,
            event_rx: rx,
        })
    }

    /// Watch a directory for changes
    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
        Ok(())
    }

    /// Get next file change event
    pub async fn next_event(&mut self) -> Option<Event> {
        if let Ok(Ok(event)) = self.event_rx.recv().await {
            Some(event)
        } else {
            None
        }
    }

    /// Check if event is a DSL file change
    pub fn is_dsl_change(&self, event: &Event) -> bool {
        matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_))
            && event.paths.iter().any(|p| {
                p.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == "toml" || ext == "dsl" || ext == "playbook")
                    .unwrap_or(false)
            })
    }
}

impl Default for FileWatcher {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

