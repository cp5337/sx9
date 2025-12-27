//! Voice Session Management
//!
//! RFC-9107 §5: Manages voice conversation sessions with state tracking.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
use uuid::Uuid;

/// Voice session state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionState {
    /// Session is active
    Active,
    /// Session is paused
    Paused,
    /// Session has ended
    Ended,
    /// Session encountered an error
    Error,
}

/// A voice interaction within a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    /// Interaction ID
    pub id: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Speaker (user or agent ID)
    pub speaker: String,
    /// Text content
    pub text: String,
    /// Audio file path (if saved)
    pub audio_path: Option<String>,
    /// Processing duration in ms
    pub duration_ms: u64,
}

/// Voice session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session ID
    pub id: String,
    /// Agent ID handling this session
    pub agent_id: String,
    /// Voice ID being used
    pub voice_id: String,
    /// Session state
    pub state: SessionState,
    /// Session start time
    pub started_at: DateTime<Utc>,
    /// Session end time (if ended)
    pub ended_at: Option<DateTime<Utc>>,
    /// Interactions in this session
    pub interactions: Vec<Interaction>,
    /// Session metadata
    pub metadata: HashMap<String, String>,
}

impl Session {
    /// Create a new session
    pub fn new(agent_id: impl Into<String>, voice_id: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            agent_id: agent_id.into(),
            voice_id: voice_id.into(),
            state: SessionState::Active,
            started_at: Utc::now(),
            ended_at: None,
            interactions: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add an interaction to the session
    pub fn add_interaction(&mut self, speaker: impl Into<String>, text: impl Into<String>) -> String {
        let interaction = Interaction {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            speaker: speaker.into(),
            text: text.into(),
            audio_path: None,
            duration_ms: 0,
        };
        let id = interaction.id.clone();
        self.interactions.push(interaction);
        id
    }

    /// End the session
    pub fn end(&mut self) {
        self.state = SessionState::Ended;
        self.ended_at = Some(Utc::now());
    }

    /// Pause the session
    pub fn pause(&mut self) {
        self.state = SessionState::Paused;
    }

    /// Resume the session
    pub fn resume(&mut self) {
        if self.state == SessionState::Paused {
            self.state = SessionState::Active;
        }
    }

    /// Get session duration in seconds
    pub fn duration_secs(&self) -> i64 {
        let end = self.ended_at.unwrap_or_else(Utc::now);
        (end - self.started_at).num_seconds()
    }

    /// Get interaction count
    pub fn interaction_count(&self) -> usize {
        self.interactions.len()
    }

    /// Get last interaction
    pub fn last_interaction(&self) -> Option<&Interaction> {
        self.interactions.last()
    }
}

/// Session manager for handling multiple voice sessions
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    max_sessions: usize,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(max_sessions: usize) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            max_sessions,
        }
    }

    /// Create a new session
    pub async fn create_session(
        &self,
        agent_id: impl Into<String>,
        voice_id: impl Into<String>,
    ) -> String {
        let session = Session::new(agent_id, voice_id);
        let session_id = session.id.clone();

        let mut sessions = self.sessions.write().await;

        // Clean up old sessions if at capacity
        if sessions.len() >= self.max_sessions {
            self.cleanup_old_sessions(&mut sessions);
        }

        info!("Created voice session: {}", session_id);
        sessions.insert(session_id.clone(), session);

        session_id
    }

    /// Get a session by ID
    pub async fn get_session(&self, session_id: &str) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    /// Update a session
    pub async fn update_session<F>(&self, session_id: &str, f: F) -> bool
    where
        F: FnOnce(&mut Session),
    {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            f(session);
            true
        } else {
            false
        }
    }

    /// Add interaction to session
    pub async fn add_interaction(
        &self,
        session_id: &str,
        speaker: impl Into<String>,
        text: impl Into<String>,
    ) -> Option<String> {
        let speaker = speaker.into();
        let text = text.into();

        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            let interaction_id = session.add_interaction(speaker, text);
            debug!("Added interaction {} to session {}", interaction_id, session_id);
            Some(interaction_id)
        } else {
            None
        }
    }

    /// End a session
    pub async fn end_session(&self, session_id: &str) -> bool {
        self.update_session(session_id, |s| s.end()).await
    }

    /// Get all active sessions
    pub async fn get_active_sessions(&self) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        sessions
            .values()
            .filter(|s| s.state == SessionState::Active)
            .cloned()
            .collect()
    }

    /// Get session count
    pub async fn session_count(&self) -> usize {
        self.sessions.read().await.len()
    }

    /// Cleanup old ended sessions
    fn cleanup_old_sessions(&self, sessions: &mut HashMap<String, Session>) {
        let cutoff = Utc::now() - chrono::Duration::hours(1);

        sessions.retain(|_, session| {
            if session.state == SessionState::Ended {
                if let Some(ended_at) = session.ended_at {
                    return ended_at > cutoff;
                }
            }
            true
        });
    }

    /// Get sessions by agent
    pub async fn get_sessions_by_agent(&self, agent_id: &str) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        sessions
            .values()
            .filter(|s| s.agent_id == agent_id)
            .cloned()
            .collect()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = Session::new("natasha", "voice-123");
        assert_eq!(session.agent_id, "natasha");
        assert_eq!(session.voice_id, "voice-123");
        assert_eq!(session.state, SessionState::Active);
    }

    #[test]
    fn test_session_interaction() {
        let mut session = Session::new("natasha", "voice-123");
        session.add_interaction("user", "Hello");
        session.add_interaction("natasha", "Hi there!");

        assert_eq!(session.interaction_count(), 2);
        assert_eq!(session.last_interaction().unwrap().speaker, "natasha");
    }

    #[test]
    fn test_session_lifecycle() {
        let mut session = Session::new("natasha", "voice-123");
        assert_eq!(session.state, SessionState::Active);

        session.pause();
        assert_eq!(session.state, SessionState::Paused);

        session.resume();
        assert_eq!(session.state, SessionState::Active);

        session.end();
        assert_eq!(session.state, SessionState::Ended);
        assert!(session.ended_at.is_some());
    }

    #[tokio::test]
    async fn test_session_manager() {
        let manager = SessionManager::new(10);

        let session_id = manager.create_session("natasha", "voice-123").await;
        assert!(!session_id.is_empty());

        let session = manager.get_session(&session_id).await;
        assert!(session.is_some());

        manager
            .add_interaction(&session_id, "user", "Hello")
            .await;

        let session = manager.get_session(&session_id).await.unwrap();
        assert_eq!(session.interaction_count(), 1);
    }
}
