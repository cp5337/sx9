//! Persona lifecycle management

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};

use crate::{Persona, PersonaId, PersonaRegistry, TrustLevel};
use crate::types::{AgentId, Result, AgentError, AgentStatus, AgentMetadata};

/// Persona lifecycle states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersonaState {
    Created,
    Initializing,
    Active,
    Suspended,
    Expiring,
    Terminated,
}

/// Persona lifecycle manager
#[derive(Debug)]
pub struct PersonaLifecycleManager {
    registry: Arc<RwLock<PersonaRegistry>>,
    states: Arc<RwLock<HashMap<PersonaId, PersonaState>>>,
    last_cleanup: Arc<RwLock<DateTime<Utc>>>,
}

impl PersonaLifecycleManager {
    /// Create new lifecycle manager
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(PersonaRegistry::new())),
            states: Arc::new(RwLock::new(HashMap::new())),
            last_cleanup: Arc::new(RwLock::new(Utc::now())),
        }
    }

    /// Create persona and start lifecycle
    pub async fn create_persona(
        &self,
        cuid: String,
        name: String,
        capabilities: Vec<String>,
        trust_level: TrustLevel,
    ) -> Result<PersonaId> {
        let mut persona = Persona::new(cuid, name);
        persona.trust_level = trust_level;

        for capability in capabilities {
            persona.add_capability(capability);
        }

        let persona_id = persona.id.clone();

        // Register persona
        {
            let mut registry = self.registry.write().await;
            registry.register(persona)?;
        }

        // Set initial state
        {
            let mut states = self.states.write().await;
            states.insert(persona_id.clone(), PersonaState::Created);
        }

        info!("Created persona '{}' with ID '{}'",
              self.get_persona_name(&persona_id).await.unwrap_or_default(),
              persona_id.0);

        Ok(persona_id)
    }

    /// Initialize persona (transition to Active state)
    pub async fn initialize_persona(&self, persona_id: &PersonaId) -> Result<()> {
        {
            let mut states = self.states.write().await;
            if let Some(state) = states.get_mut(persona_id) {
                match state {
                    PersonaState::Created => {
                        *state = PersonaState::Initializing;
                        debug!("Initializing persona '{}'", persona_id.0);
                    }
                    _ => {
                        return Err(AgentError::InvalidInput {
                            field: "persona_state".to_string(),
                            reason: format!("Cannot initialize persona in state {:?}", state),
                        });
                    }
                }
            } else {
                return Err(AgentError::AgentNotFound {
                    id: persona_id.0.clone(),
                });
            }
        }

        // Perform initialization tasks
        self.perform_initialization(persona_id).await?;

        // Transition to Active
        {
            let mut states = self.states.write().await;
            if let Some(state) = states.get_mut(persona_id) {
                *state = PersonaState::Active;
                info!("Persona '{}' is now active", persona_id.0);
            }
        }

        Ok(())
    }

    /// Suspend persona
    pub async fn suspend_persona(&self, persona_id: &PersonaId) -> Result<()> {
        let mut states = self.states.write().await;
        if let Some(state) = states.get_mut(persona_id) {
            match state {
                PersonaState::Active => {
                    *state = PersonaState::Suspended;
                    warn!("Suspended persona '{}'", persona_id.0);
                    Ok(())
                }
                _ => Err(AgentError::InvalidInput {
                    field: "persona_state".to_string(),
                    reason: format!("Cannot suspend persona in state {:?}", state),
                })
            }
        } else {
            Err(AgentError::AgentNotFound {
                id: persona_id.0.clone(),
            })
        }
    }

    /// Resume suspended persona
    pub async fn resume_persona(&self, persona_id: &PersonaId) -> Result<()> {
        let mut states = self.states.write().await;
        if let Some(state) = states.get_mut(persona_id) {
            match state {
                PersonaState::Suspended => {
                    *state = PersonaState::Active;
                    info!("Resumed persona '{}'", persona_id.0);
                    Ok(())
                }
                _ => Err(AgentError::InvalidInput {
                    field: "persona_state".to_string(),
                    reason: format!("Cannot resume persona in state {:?}", state),
                })
            }
        } else {
            Err(AgentError::AgentNotFound {
                id: persona_id.0.clone(),
            })
        }
    }

    /// Terminate persona
    pub async fn terminate_persona(&self, persona_id: &PersonaId) -> Result<()> {
        // Remove from registry
        {
            let mut registry = self.registry.write().await;
            registry.unregister(persona_id)?;
        }

        // Update state
        {
            let mut states = self.states.write().await;
            states.insert(persona_id.clone(), PersonaState::Terminated);
        }

        info!("Terminated persona '{}'", persona_id.0);
        Ok(())
    }

    /// Get persona state
    pub async fn get_state(&self, persona_id: &PersonaId) -> Option<PersonaState> {
        let states = self.states.read().await;
        states.get(persona_id).copied()
    }

    /// Get all active personas
    pub async fn active_personas(&self) -> Vec<PersonaId> {
        let states = self.states.read().await;
        states
            .iter()
            .filter(|(_, state)| **state == PersonaState::Active)
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Periodic cleanup of expired personas
    pub async fn cleanup_expired(&self) -> Result<usize> {
        let mut last_cleanup = self.last_cleanup.write().await;
        let now = Utc::now();

        // Only run cleanup every hour
        if (now - *last_cleanup).num_hours() < 1 {
            return Ok(0);
        }

        *last_cleanup = now;
        drop(last_cleanup);

        let expired_ids = {
            let registry = self.registry.read().await;
            registry
                .personas
                .iter()
                .filter(|p| p.is_expired())
                .map(|p| p.id.clone())
                .collect::<Vec<_>>()
        };

        let mut cleaned = 0;
        for persona_id in expired_ids {
            if let Err(e) = self.terminate_persona(&persona_id).await {
                error!("Failed to terminate expired persona '{}': {}", persona_id.0, e);
            } else {
                cleaned += 1;
            }
        }

        if cleaned > 0 {
            info!("Cleaned up {} expired personas", cleaned);
        }

        Ok(cleaned)
    }

    /// Get persona metadata for agent system integration
    pub async fn get_agent_metadata(&self, persona_id: &PersonaId) -> Result<AgentMetadata> {
        let registry = self.registry.read().await;
        let persona = registry.find_by_id(persona_id)
            .ok_or_else(|| AgentError::AgentNotFound {
                id: persona_id.0.clone(),
            })?;

        let state = self.get_state(persona_id).await
            .unwrap_or(PersonaState::Terminated);

        let agent_status = match state {
            PersonaState::Active => AgentStatus::Online,
            PersonaState::Suspended => AgentStatus::Offline,
            PersonaState::Initializing => AgentStatus::Busy,
            _ => AgentStatus::Error,
        };

        Ok(AgentMetadata {
            id: persona.agent_id,
            name: persona.name.clone(),
            version: "1.0".to_string(),
            capabilities: persona.capabilities.clone(),
            status: agent_status,
            last_seen: Utc::now(),
        })
    }

    /// Get persona name by ID
    async fn get_persona_name(&self, persona_id: &PersonaId) -> Option<String> {
        let registry = self.registry.read().await;
        registry.find_by_id(persona_id).map(|p| p.name.clone())
    }

    /// Perform initialization tasks
    async fn perform_initialization(&self, _persona_id: &PersonaId) -> Result<()> {
        // Placeholder for initialization logic
        // Could include setting up gRPC endpoints, registering with discovery, etc.
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        Ok(())
    }
}

impl Default for PersonaLifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}