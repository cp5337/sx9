//! Reaction Cache
//!
//! Caches threat reactions for fast retrieval

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;
use uuid::Uuid;

use super::{ReactionSession, ReactionStatus};

/// Reaction cache
pub struct ReactionCache {
    sessions: Arc<RwLock<HashMap<Uuid, ReactionSession>>>,
}

impl ReactionCache {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn store(&mut self, session: &ReactionSession) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id, session.clone());
        debug!("Stored reaction session: {}", session.id);
        Ok(())
    }

    pub async fn get(&self, session_id: Uuid) -> Result<Option<ReactionSession>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.get(&session_id).cloned())
    }

    pub async fn update_status(&mut self, session_id: Uuid, status: ReactionStatus) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.status = status;
            debug!("Updated session {} status", session_id);
        }
        Ok(())
    }
}

impl Default for ReactionCache {
    fn default() -> Self {
        Self::new()
    }
}
