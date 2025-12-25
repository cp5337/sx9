//! Heartbeat Emitter for Smart Crates (RFC-9141)
//!
//! Async NATS-based heartbeat publishing layer.
//! Uses foundation-core types, adds network transport.
//!
//! ## Non-Blocking Design
//! - Publishes to Core NATS (~50µs latency)
//! - Fire-and-forget pattern (no acks required)
//! - Eventual consistency for global state
//!
//! ## Usage
//! ```rust,ignore
//! let emitter = HeartbeatEmitter::connect("nats://localhost:4222").await?;
//! emitter.start_background_emission(config, Duration::from_secs(1)).await;
//! ```

use async_nats::Client;
use futures::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

// Re-export foundation-core heartbeat types
pub use sx9_foundation_core::heartbeat::{
    HeartbeatConfig, HealthStatus, LocalHeartbeat, SmartCrateHeartbeat, FOUNDATION_CORE_TOKEN,
};

use super::subjects::heartbeat as subjects;

/// NATS-based heartbeat emitter for smart crates
pub struct HeartbeatEmitter {
    client: Client,
    inner: SmartCrateHeartbeat,
    running: Arc<RwLock<bool>>,
}

impl HeartbeatEmitter {
    /// Connect to NATS and create emitter for a smart crate
    pub async fn connect(nats_url: &str, config: HeartbeatConfig) -> Result<Self, String> {
        let client = async_nats::connect(nats_url)
            .await
            .map_err(|e| format!("NATS connection failed: {}", e))?;

        let inner = SmartCrateHeartbeat::new(config);

        Ok(Self {
            client,
            inner,
            running: Arc::new(RwLock::new(false)),
        })
    }

    /// Create emitter for a foundation crate
    pub async fn foundation(
        nats_url: &str,
        crate_name: &str,
        version: &str,
        tier: &str,
        port: u16,
    ) -> Result<Self, String> {
        let config = HeartbeatConfig::foundation(crate_name, version, tier, port);
        Self::connect(nats_url, config).await
    }

    /// Set current health status
    pub fn set_health(&self, health: HealthStatus) {
        self.inner.set_health(health);
    }

    /// Emit a single heartbeat (non-blocking)
    pub async fn emit(&self, state_hash: String) -> Result<(), String> {
        let heartbeat = self.inner.emit(state_hash);
        let payload =
            serde_json::to_vec(&heartbeat).map_err(|e| format!("Serialization failed: {}", e))?;

        // Publish to crate-specific subject
        let subject = subjects::for_crate(self.inner.crate_name());
        self.client
            .publish(subject, payload.into())
            .await
            .map_err(|e| format!("NATS publish failed: {}", e))?;

        Ok(())
    }

    /// Start background heartbeat emission
    ///
    /// Spawns a tokio task that emits heartbeats at the specified interval.
    /// Returns immediately - heartbeats continue in background.
    pub async fn start_background_emission(
        self: Arc<Self>,
        interval: Duration,
        state_hash_fn: impl Fn() -> String + Send + Sync + 'static,
    ) {
        let running = self.running.clone();
        *running.write().await = true;

        let emitter = self.clone();
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                if !*emitter.running.read().await {
                    break;
                }

                let hash = state_hash_fn();
                if let Err(e) = emitter.emit(hash).await {
                    eprintln!("Heartbeat emission failed: {}", e);
                }
            }
        });
    }

    /// Stop background heartbeat emission
    pub async fn stop(&self) {
        *self.running.write().await = false;
    }

    /// Get the foundation-core token (proves dependency)
    pub fn foundation_token(&self) -> &'static str {
        self.inner.foundation_token()
    }

    /// Get the crate name
    pub fn crate_name(&self) -> &str {
        self.inner.crate_name()
    }
}

/// Heartbeat orchestrator for global state aggregation
///
/// Subscribes to all crate heartbeats and maintains global state.
/// Publishes alerts for unauthorized or missing heartbeats.
pub struct HeartbeatOrchestrator {
    client: Client,
    registry: sx9_foundation_core::heartbeat::HeartbeatRegistry,
}

impl HeartbeatOrchestrator {
    /// Connect to NATS and create orchestrator
    pub async fn connect(nats_url: &str) -> Result<Self, String> {
        let client = async_nats::connect(nats_url)
            .await
            .map_err(|e| format!("NATS connection failed: {}", e))?;

        let registry = sx9_foundation_core::heartbeat::HeartbeatRegistry::new();

        Ok(Self { client, registry })
    }

    /// Start the orchestrator (subscribes to heartbeats, publishes global state)
    pub async fn run(self: Arc<Self>) -> Result<(), String> {
        // Subscribe to all crate heartbeats
        let mut subscriber = self
            .client
            .subscribe(subjects::CRATE_WILDCARD.to_string())
            .await
            .map_err(|e| format!("NATS subscribe failed: {}", e))?;

        // Spawn validation loop
        let orchestrator = self.clone();
        tokio::spawn(async move {
            let mut validation_interval = tokio::time::interval(Duration::from_secs(5));

            loop {
                validation_interval.tick().await;

                let state = orchestrator.registry.validate_all();

                // Publish global state
                if let Ok(payload) = serde_json::to_vec(&state) {
                    let _ = orchestrator
                        .client
                        .publish(subjects::GLOBAL.to_string(), payload.into())
                        .await;
                }

                // Alert on unauthorized crates
                for crate_name in &state.unauthorized_crates {
                    let alert = serde_json::json!({
                        "crate_name": crate_name,
                        "violation": "missing_foundation_core",
                        "severity": "critical"
                    });
                    if let Ok(payload) = serde_json::to_vec(&alert) {
                        let _ = orchestrator
                            .client
                            .publish(subjects::ALERT_UNAUTHORIZED.to_string(), payload.into())
                            .await;
                    }
                }

                // Alert on missing heartbeats
                for crate_name in &state.missing_heartbeats {
                    let alert = serde_json::json!({
                        "crate_name": crate_name,
                        "violation": "missing_heartbeat",
                        "severity": "warning"
                    });
                    if let Ok(payload) = serde_json::to_vec(&alert) {
                        let _ = orchestrator
                            .client
                            .publish(subjects::ALERT_MISSING.to_string(), payload.into())
                            .await;
                    }
                }
            }
        });

        // Process incoming heartbeats
        while let Some(msg) = subscriber.next().await {
            if let Ok(heartbeat) = serde_json::from_slice::<LocalHeartbeat>(&msg.payload) {
                let validation = self.registry.record_heartbeat(heartbeat);

                // Immediately alert on unauthorized
                if validation.is_unauthorized() {
                    eprintln!("🚨 ZERO-TRUST: Unauthorized heartbeat detected");
                }
            }
        }

        Ok(())
    }

    /// Get current global state (for QA gate integration)
    pub fn get_state(&self) -> sx9_foundation_core::heartbeat::GlobalHeartbeatState {
        self.registry.validate_all()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foundation_token_available() {
        // This test proves sx9-harness has foundation-core
        assert_eq!(FOUNDATION_CORE_TOKEN, "sx9-foundation-core-7.3.1");
    }
}
