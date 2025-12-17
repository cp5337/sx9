//! NATS Bridge - Message bus integration
//!
//! Subjects:
//! - `osint.intel` - Inbound OSINT intelligence from Python pipeline
//! - `eei.query` - EEI query requests
//! - `eei.answer` - EEI answer responses
//! - `leptose.status` - Engine status updates
//! - `leptose.graph.*` - Graph operations

use crate::{LeptoseError, Result};
use async_nats::{Client, Message, Subscriber};
use bytes::Bytes;

/// NATS subject constants
pub mod subjects {
    pub const OSINT_INTEL: &str = "osint.intel";
    pub const EEI_QUERY: &str = "eei.query";
    pub const EEI_ANSWER: &str = "eei.answer";
    pub const LEPTOSE_STATUS: &str = "leptose.status";
    pub const LEPTOSE_GRAPH: &str = "leptose.graph";
}

/// NATS bridge for message bus communication
pub struct NatsBridge {
    client: Client,
}

impl NatsBridge {
    /// Connect to NATS server
    pub async fn connect(url: &str) -> Result<Self> {
        let client = async_nats::connect(url)
            .await
            .map_err(|e| LeptoseError::NatsError(format!("Failed to connect to NATS: {}", e)))?;

        tracing::info!("Connected to NATS at {}", url);

        Ok(Self { client })
    }

    /// Subscribe to a subject
    pub async fn subscribe(&self, subject: &str) -> Result<NatsSubscriber> {
        let subscriber = self
            .client
            .subscribe(subject.to_string())
            .await
            .map_err(|e| LeptoseError::NatsError(format!("Failed to subscribe: {}", e)))?;

        Ok(NatsSubscriber { inner: subscriber })
    }

    /// Publish a message
    pub async fn publish(&self, subject: &str, payload: Bytes) -> Result<()> {
        self.client
            .publish(subject.to_string(), payload)
            .await
            .map_err(|e| LeptoseError::NatsError(format!("Failed to publish: {}", e)))?;

        Ok(())
    }

    /// Publish a message and wait for reply
    pub async fn request(&self, subject: &str, payload: Bytes) -> Result<Message> {
        let response = self
            .client
            .request(subject.to_string(), payload)
            .await
            .map_err(|e| LeptoseError::NatsError(format!("Request failed: {}", e)))?;

        Ok(response)
    }

    /// Publish status update
    pub async fn publish_status(&self, status: &str) -> Result<()> {
        self.publish(subjects::LEPTOSE_STATUS, Bytes::from(status.to_string()))
            .await
    }

    /// Get the underlying client for advanced operations
    pub fn client(&self) -> &Client {
        &self.client
    }
}

/// NATS subscriber wrapper
pub struct NatsSubscriber {
    inner: Subscriber,
}

impl NatsSubscriber {
    /// Get next message
    pub async fn next(&mut self) -> Option<Message> {
        use futures_util::StreamExt;
        self.inner.next().await
    }
}
