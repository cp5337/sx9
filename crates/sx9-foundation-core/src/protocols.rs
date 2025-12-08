//! Message protocols for CTAS-7 communication

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use ctas7_agentic_core::{AgentId, MessageId, SessionId, Priority};

/// Message envelope for protocol wrapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEnvelope {
    pub protocol_version: String,
    pub message_type: MessageType,
    pub payload: MessagePayload,
    pub routing: RoutingInfo,
    pub metadata: ProtocolMetadata,
}

/// Message type classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Request,
    Response,
    Notification,
    Broadcast,
    Heartbeat,
}

/// Message payload variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePayload {
    Text { content: String },
    Binary { data: Vec<u8> },
    Json { value: serde_json::Value },
    Command { action: String, params: serde_json::Value },
    Result { success: bool, data: serde_json::Value },
}

/// Routing information for message delivery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingInfo {
    pub from: AgentId,
    pub to: Option<AgentId>,
    pub reply_to: Option<AgentId>,
    pub correlation_id: Option<MessageId>,
    pub capability_hint: Option<String>,
}

/// Protocol metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolMetadata {
    pub id: MessageId,
    pub timestamp: DateTime<Utc>,
    pub priority: Priority,
    pub session_id: Option<SessionId>,
    pub ttl_seconds: Option<u64>,
    pub retry_count: u32,
    pub max_retries: u32,
}

impl MessageEnvelope {
    /// Create new message envelope
    pub fn new(
        message_type: MessageType,
        payload: MessagePayload,
        from: AgentId,
        to: Option<AgentId>,
    ) -> Self {
        Self {
            protocol_version: "1.0".to_string(),
            message_type,
            payload,
            routing: RoutingInfo {
                from,
                to,
                reply_to: None,
                correlation_id: None,
                capability_hint: None,
            },
            metadata: ProtocolMetadata {
                id: MessageId::new(),
                timestamp: Utc::now(),
                priority: Priority::Medium,
                session_id: None,
                ttl_seconds: None,
                retry_count: 0,
                max_retries: 3,
            },
        }
    }

    /// Create request message
    pub fn request(from: AgentId, to: AgentId, payload: MessagePayload) -> Self {
        Self::new(MessageType::Request, payload, from, Some(to))
    }

    /// Create response message
    pub fn response(from: AgentId, to: AgentId, payload: MessagePayload,
                   correlation_id: MessageId) -> Self {
        let mut envelope = Self::new(MessageType::Response, payload, from, Some(to));
        envelope.routing.correlation_id = Some(correlation_id);
        envelope
    }

    /// Create broadcast message
    pub fn broadcast(from: AgentId, payload: MessagePayload) -> Self {
        Self::new(MessageType::Broadcast, payload, from, None)
    }

    /// Create notification message
    pub fn notification(from: AgentId, to: Option<AgentId>, payload: MessagePayload) -> Self {
        Self::new(MessageType::Notification, payload, from, to)
    }

    /// Set priority
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.metadata.priority = priority;
        self
    }

    /// Set session ID
    pub fn with_session(mut self, session_id: SessionId) -> Self {
        self.metadata.session_id = Some(session_id);
        self
    }

    /// Set capability hint for routing
    pub fn with_capability_hint(mut self, capability: String) -> Self {
        self.routing.capability_hint = Some(capability);
        self
    }

    /// Set TTL
    pub fn with_ttl(mut self, ttl_seconds: u64) -> Self {
        self.metadata.ttl_seconds = Some(ttl_seconds);
        self
    }

    /// Check if message has expired
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.metadata.ttl_seconds {
            let elapsed = Utc::now()
                .signed_duration_since(self.metadata.timestamp)
                .num_seconds() as u64;
            elapsed > ttl
        } else {
            false
        }
    }

    /// Check if retries are exhausted
    pub fn retries_exhausted(&self) -> bool {
        self.metadata.retry_count >= self.metadata.max_retries
    }

    /// Increment retry count
    pub fn increment_retry(&mut self) {
        self.metadata.retry_count += 1;
    }
}