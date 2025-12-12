//! Message routing implementation for CTAS-7

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, warn};

use crate::types::{
    AgentId, Message, MessageId, AgentMetadata, MessageRouter as MessageRouterTrait,
    Result, AgentError, Priority
};

/// Core message router implementation
#[derive(Debug)]
pub struct MessageRouter {
    agents: Arc<RwLock<HashMap<AgentId, AgentMetadata>>>,
    capabilities: Arc<RwLock<HashMap<String, Vec<AgentId>>>>,
}

impl MessageRouter {
    /// Create new message router
    pub fn new() -> Self {
        Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            capabilities: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Route message by capability instead of direct agent ID
    pub async fn route_by_capability(&self, message: Message, capability: &str) -> Result<()> {
        let capabilities = self.capabilities.read().await;

        if let Some(agent_ids) = capabilities.get(capability) {
            if agent_ids.is_empty() {
                return Err(AgentError::MessageRoutingFailed {
                    message_id: message.id.0.to_string(),
                    reason: format!("No agents available for capability: {}", capability),
                });
            }

            // Route to first available agent (can be enhanced with load balancing)
            let target_agent = agent_ids[0];
            debug!("Routing message {} to agent {} for capability {}",
                   message.id.0, target_agent.0, capability);

            // Here you would implement actual message delivery
            // For now, just log success
            debug!("Message routed successfully");
            Ok(())
        } else {
            Err(AgentError::MessageRoutingFailed {
                message_id: message.id.0.to_string(),
                reason: format!("No agents registered for capability: {}", capability),
            })
        }
    }

    /// Get agents by capability
    pub async fn get_agents_by_capability(&self, capability: &str) -> Result<Vec<AgentMetadata>> {
        let capabilities = self.capabilities.read().await;
        let agents = self.agents.read().await;

        if let Some(agent_ids) = capabilities.get(capability) {
            let mut result = Vec::new();
            for agent_id in agent_ids {
                if let Some(metadata) = agents.get(agent_id) {
                    result.push(metadata.clone());
                }
            }
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }
}

impl Default for MessageRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MessageRouterTrait for MessageRouter {
    async fn route_message(&self, message: Message) -> Result<()> {
        let agents = self.agents.read().await;

        if let Some(target_agent) = message.to {
            if !agents.contains_key(&target_agent) {
                return Err(AgentError::AgentNotFound {
                    id: target_agent.0.to_string(),
                });
            }

            debug!("Routing message {} from {} to {}",
                   message.id.0, message.from.0, target_agent.0);

            // Implementation would send message to target agent
            Ok(())
        } else {
            // Broadcast message - route to all agents except sender
            debug!("Broadcasting message {} from {}", message.id.0, message.from.0);
            Ok(())
        }
    }

    async fn register_agent(&mut self, metadata: AgentMetadata) -> Result<()> {
        let agent_id = metadata.id;

        // Register agent
        {
            let mut agents = self.agents.write().await;
            agents.insert(agent_id, metadata.clone());
        }

        // Register capabilities
        {
            let mut capabilities = self.capabilities.write().await;
            for capability in &metadata.capabilities {
                capabilities
                    .entry(capability.clone())
                    .or_insert_with(Vec::new)
                    .push(agent_id);
            }
        }

        debug!("Registered agent {} with capabilities: {:?}",
               agent_id.0, metadata.capabilities);
        Ok(())
    }

    async fn unregister_agent(&mut self, agent_id: AgentId) -> Result<()> {
        // Get agent metadata before removal
        let metadata = {
            let mut agents = self.agents.write().await;
            agents.remove(&agent_id)
        };

        if let Some(metadata) = metadata {
            // Remove from capabilities
            let mut capabilities = self.capabilities.write().await;
            for capability in &metadata.capabilities {
                if let Some(agent_list) = capabilities.get_mut(capability) {
                    agent_list.retain(|id| *id != agent_id);
                    if agent_list.is_empty() {
                        capabilities.remove(capability);
                    }
                }
            }

            debug!("Unregistered agent {}", agent_id.0);
            Ok(())
        } else {
            warn!("Attempted to unregister unknown agent {}", agent_id.0);
            Err(AgentError::AgentNotFound {
                id: agent_id.0.to_string(),
            })
        }
    }
}