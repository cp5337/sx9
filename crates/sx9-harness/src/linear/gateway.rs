//! Linear Gateway Service
//!
//! Central coordination point for all Linear operations (RFC-9030).

use std::sync::Arc;
use std::time::Instant;

use super::{
    AgentDispatcher, GatewayHealth, LinearAgentTask, LinearGatewayConfig, LinearIssue,
    LinearWebhook, WebhookEventType, WebhookHandler,
};
use crate::agents::AgentRegistry;
use crate::nats::subjects;

/// Linear Gateway Service
///
/// Provides unified Linear integration:
/// - Webhook handling
/// - GraphQL client (stub - requires async-graphql)
/// - Agent dispatch via NATS
pub struct LinearGateway {
    /// Configuration
    config: LinearGatewayConfig,

    /// Agent registry for dispatch
    agents: Arc<AgentRegistry>,

    /// Webhook handler
    webhook_handler: WebhookHandler,

    /// Agent dispatcher
    dispatcher: AgentDispatcher,

    /// Start time for uptime tracking
    started_at: Instant,

    /// Connection states
    linear_connected: bool,
    nats_connected: bool,
}

impl LinearGateway {
    /// Create new gateway instance
    pub fn new(config: LinearGatewayConfig, agents: AgentRegistry) -> Self {
        let agents = Arc::new(agents);

        Self {
            webhook_handler: WebhookHandler::new(config.webhook_secret.clone()),
            dispatcher: AgentDispatcher::new(Arc::clone(&agents)),
            config,
            agents,
            started_at: Instant::now(),
            linear_connected: false,
            nats_connected: false,
        }
    }

    /// Handle incoming webhook from Linear
    pub fn handle_webhook(&self, payload: &LinearWebhook) -> Result<Option<LinearAgentTask>, String> {
        // Verify webhook signature would happen at HTTP layer
        let event_type = WebhookEventType::from_webhook(payload);

        match event_type {
            WebhookEventType::IssueCreated => self.handle_issue_created(payload),
            WebhookEventType::IssueUpdated => self.handle_issue_updated(payload),
            WebhookEventType::CommentCreated => self.handle_comment_created(payload),
            _ => Ok(None),
        }
    }

    /// Handle issue creation - dispatch to appropriate agent
    fn handle_issue_created(&self, webhook: &LinearWebhook) -> Result<Option<LinearAgentTask>, String> {
        let issue = webhook
            .to_issue()
            .ok_or_else(|| "Failed to parse issue from webhook".to_string())?;

        // Dispatch to agent
        let task = self.dispatcher.dispatch_issue(&issue)?;

        // Would publish to NATS here
        // self.nats.publish(subjects::linear::CREATE, task).await?;

        Ok(Some(task))
    }

    /// Handle issue update - sync status
    fn handle_issue_updated(&self, webhook: &LinearWebhook) -> Result<Option<LinearAgentTask>, String> {
        let issue = webhook
            .to_issue()
            .ok_or_else(|| "Failed to parse issue from webhook".to_string())?;

        // Check if state changed to in_progress - agent picked it up
        // or if state changed to done - agent completed

        // Would publish status update to NATS
        // self.nats.publish(subjects::linear::UPDATE, status).await?;

        Ok(None)
    }

    /// Handle comment creation - may trigger AI response
    fn handle_comment_created(&self, webhook: &LinearWebhook) -> Result<Option<LinearAgentTask>, String> {
        let comment = webhook
            .to_comment()
            .ok_or_else(|| "Failed to parse comment from webhook".to_string())?;

        // Check if comment mentions an agent (@forge, @claude, etc.)
        if let Some(agent_handle) = self.extract_agent_mention(&comment.body) {
            // Create task for mentioned agent
            let _agent = self
                .agents
                .get_by_handle(&agent_handle)
                .ok_or_else(|| format!("Agent not found: {}", agent_handle))?;

            // Would dispatch to agent
            // Return task for the agent to handle
        }

        Ok(None)
    }

    /// Extract agent @ mention from text
    fn extract_agent_mention(&self, text: &str) -> Option<String> {
        // Look for @agent patterns
        for word in text.split_whitespace() {
            if word.starts_with('@') {
                let handle = word.trim_start_matches('@').to_lowercase();
                if self.agents.get_by_handle(&handle).is_some() {
                    return Some(handle);
                }
            }
        }
        None
    }

    /// Get gateway health status
    pub fn health(&self) -> GatewayHealth {
        GatewayHealth {
            status: if self.linear_connected && self.nats_connected {
                "healthy".to_string()
            } else {
                "degraded".to_string()
            },
            linear_connected: self.linear_connected,
            nats_connected: self.nats_connected,
            agents_active: self.agents.len(),
            uptime_seconds: self.started_at.elapsed().as_secs(),
        }
    }

    /// Get NATS subject for issue events
    pub fn issue_subject(issue_id: &str) -> String {
        subjects::linear::issue(issue_id)
    }

    /// Get gateway port
    pub fn port(&self) -> u16 {
        self.config.port
    }

    /// Set connection states (for testing/monitoring)
    pub fn set_linear_connected(&mut self, connected: bool) {
        self.linear_connected = connected;
    }

    pub fn set_nats_connected(&mut self, connected: bool) {
        self.nats_connected = connected;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> LinearGatewayConfig {
        LinearGatewayConfig {
            api_key: "test_key".to_string(),
            team_id: "team_123".to_string(),
            workspace_id: "ws_123".to_string(),
            webhook_secret: "whsec_test".to_string(),
            port: 18120,
            nats_url: "nats://localhost:4222".to_string(),
        }
    }

    #[test]
    fn test_gateway_creation() {
        let config = test_config();
        let agents = AgentRegistry::with_defaults();
        let gateway = LinearGateway::new(config, agents);

        assert_eq!(gateway.port(), 18120);
    }

    #[test]
    fn test_gateway_health() {
        let config = test_config();
        let agents = AgentRegistry::with_defaults();
        let gateway = LinearGateway::new(config, agents);

        let health = gateway.health();
        assert_eq!(health.status, "degraded"); // Not connected yet
        assert!(!health.linear_connected);
        assert!(!health.nats_connected);
        assert!(health.agents_active > 0);
    }

    #[test]
    fn test_extract_agent_mention() {
        let config = test_config();
        let agents = AgentRegistry::with_defaults();
        let gateway = LinearGateway::new(config, agents);

        // Test with known agent
        let mention = gateway.extract_agent_mention("Hey @forge can you help?");
        assert_eq!(mention, Some("forge".to_string()));

        // Test with unknown agent
        let mention = gateway.extract_agent_mention("Hey @unknown can you help?");
        assert_eq!(mention, None);

        // Test with no mention
        let mention = gateway.extract_agent_mention("No mentions here");
        assert_eq!(mention, None);
    }

    #[test]
    fn test_issue_subject() {
        let subject = LinearGateway::issue_subject("SX9-123");
        assert_eq!(subject, "sx9.linear.issue.SX9-123");
    }
}
