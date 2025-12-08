//! Message validation for CTAS-7 protocols

use ctas7_agentic_core::{Result, AgentError, Priority};
use crate::protocols::{MessageEnvelope, MessagePayload, RoutingInfo};

/// Message validation rules
pub struct MessageValidator {
    max_payload_size: usize,
    max_ttl_seconds: u64,
    required_capabilities: Vec<String>,
}

impl MessageValidator {
    /// Create new validator with default rules
    pub fn new() -> Self {
        Self {
            max_payload_size: 1_048_576, // 1MB
            max_ttl_seconds: 86400,      // 24 hours
            required_capabilities: Vec::new(),
        }
    }

    /// Create validator with custom limits
    pub fn with_limits(max_payload_size: usize, max_ttl_seconds: u64) -> Self {
        Self {
            max_payload_size,
            max_ttl_seconds,
            required_capabilities: Vec::new(),
        }
    }

    /// Validate message envelope
    pub fn validate(&self, envelope: &MessageEnvelope) -> Result<()> {
        self.validate_routing(&envelope.routing)?;
        self.validate_payload(&envelope.payload)?;
        self.validate_metadata(envelope)?;
        Ok(())
    }

    /// Validate routing information
    fn validate_routing(&self, routing: &RoutingInfo) -> Result<()> {
        // Ensure sender is specified
        if routing.from.0.is_nil() {
            return Err(AgentError::InvalidInput {
                field: "routing.from".to_string(),
                reason: "Sender agent ID cannot be nil".to_string(),
            });
        }

        // Validate capability hint if present
        if let Some(capability) = &routing.capability_hint {
            if capability.is_empty() {
                return Err(AgentError::InvalidInput {
                    field: "routing.capability_hint".to_string(),
                    reason: "Capability hint cannot be empty".to_string(),
                });
            }

            if !self.required_capabilities.is_empty() &&
               !self.required_capabilities.contains(capability) {
                return Err(AgentError::InvalidInput {
                    field: "routing.capability_hint".to_string(),
                    reason: format!("Unknown capability: {}", capability),
                });
            }
        }

        Ok(())
    }

    /// Validate message payload
    fn validate_payload(&self, payload: &MessagePayload) -> Result<()> {
        let size = self.calculate_payload_size(payload);

        if size > self.max_payload_size {
            return Err(AgentError::InvalidInput {
                field: "payload".to_string(),
                reason: format!("Payload size {} exceeds limit {}", size, self.max_payload_size),
            });
        }

        match payload {
            MessagePayload::Text { content } => {
                if content.is_empty() {
                    return Err(AgentError::InvalidInput {
                        field: "payload.content".to_string(),
                        reason: "Text content cannot be empty".to_string(),
                    });
                }
            }
            MessagePayload::Binary { data } => {
                if data.is_empty() {
                    return Err(AgentError::InvalidInput {
                        field: "payload.data".to_string(),
                        reason: "Binary data cannot be empty".to_string(),
                    });
                }
            }
            MessagePayload::Command { action, .. } => {
                if action.is_empty() {
                    return Err(AgentError::InvalidInput {
                        field: "payload.action".to_string(),
                        reason: "Command action cannot be empty".to_string(),
                    });
                }
            }
            _ => {} // Json and Result payloads are valid by construction
        }

        Ok(())
    }

    /// Validate message metadata
    fn validate_metadata(&self, envelope: &MessageEnvelope) -> Result<()> {
        // Validate TTL
        if let Some(ttl) = envelope.metadata.ttl_seconds {
            if ttl > self.max_ttl_seconds {
                return Err(AgentError::InvalidInput {
                    field: "metadata.ttl_seconds".to_string(),
                    reason: format!("TTL {} exceeds limit {}", ttl, self.max_ttl_seconds),
                });
            }
        }

        // Validate retry count
        if envelope.metadata.retry_count > envelope.metadata.max_retries {
            return Err(AgentError::InvalidInput {
                field: "metadata.retry_count".to_string(),
                reason: "Retry count exceeds maximum retries".to_string(),
            });
        }

        Ok(())
    }

    /// Calculate payload size for validation
    fn calculate_payload_size(&self, payload: &MessagePayload) -> usize {
        match payload {
            MessagePayload::Text { content } => content.len(),
            MessagePayload::Binary { data } => data.len(),
            MessagePayload::Json { value } => {
                serde_json::to_string(value).map(|s| s.len()).unwrap_or(0)
            }
            MessagePayload::Command { action, params } => {
                action.len() + serde_json::to_string(params).map(|s| s.len()).unwrap_or(0)
            }
            MessagePayload::Result { data, .. } => {
                serde_json::to_string(data).map(|s| s.len()).unwrap_or(0)
            }
        }
    }

    /// Set required capabilities for validation
    pub fn with_required_capabilities(mut self, capabilities: Vec<String>) -> Self {
        self.required_capabilities = capabilities;
        self
    }
}

impl Default for MessageValidator {
    fn default() -> Self {
        Self::new()
    }
}