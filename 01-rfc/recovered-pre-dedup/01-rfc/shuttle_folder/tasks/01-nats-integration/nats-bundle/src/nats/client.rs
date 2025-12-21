//! NATS Client Connection Management
//!
//! Handles connection, reconnection, and JetStream context.

use anyhow::Result;
use async_nats::jetstream;
use std::time::Duration;
use tracing::{info, warn, error};

/// NATS connection configuration
#[derive(Debug, Clone)]
pub struct NatsConfig {
    /// NATS server URL(s)
    pub urls: Vec<String>,
    /// Connection name for identification
    pub name: Option<String>,
    /// Reconnect on disconnect
    pub reconnect: bool,
    /// Max reconnect attempts (0 = infinite)
    pub max_reconnects: usize,
    /// Reconnect delay
    pub reconnect_delay: Duration,
    /// Connection timeout
    pub connect_timeout: Duration,
    /// Ping interval for keepalive
    pub ping_interval: Duration,
}

impl Default for NatsConfig {
    fn default() -> Self {
        Self {
            urls: vec!["nats://localhost:4222".to_string()],
            name: None,
            reconnect: true,
            max_reconnects: 60,
            reconnect_delay: Duration::from_secs(2),
            connect_timeout: Duration::from_secs(10),
            ping_interval: Duration::from_secs(30),
        }
    }
}

impl NatsConfig {
    /// Create config from smart-crate.toml values
    pub fn from_toml(url: &str, name: Option<&str>) -> Self {
        Self {
            urls: vec![url.to_string()],
            name: name.map(String::from),
            ..Default::default()
        }
    }

    /// Add additional server URLs for clustering
    pub fn with_servers(mut self, servers: Vec<String>) -> Self {
        self.urls.extend(servers);
        self
    }

    /// Set connection name
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}

/// Connect to NATS with default configuration
///
/// Uses `nats://localhost:4222` by default.
pub async fn connect() -> Result<async_nats::Client> {
    connect_with_options(NatsConfig::default()).await
}

/// Connect to NATS with custom configuration
pub async fn connect_with_options(config: NatsConfig) -> Result<async_nats::Client> {
    let mut options = async_nats::ConnectOptions::new()
        .connection_timeout(config.connect_timeout)
        .ping_interval(config.ping_interval);

    if let Some(name) = &config.name {
        options = options.name(name);
    }

    if config.reconnect {
        options = options
            .retry_on_initial_connect()
            .max_reconnects(config.max_reconnects);
    }

    // Event callbacks for monitoring
    options = options
        .event_callback(|event| async move {
            match event {
                async_nats::Event::Connected => {
                    info!("🔗 NATS connected");
                }
                async_nats::Event::Disconnected => {
                    warn!("⚠️ NATS disconnected");
                }
                async_nats::Event::Reconnected => {
                    info!("🔄 NATS reconnected");
                }
                async_nats::Event::ServerError(err) => {
                    error!("❌ NATS server error: {}", err);
                }
                _ => {}
            }
        });

    let server_addr = config.urls.join(",");
    info!("Connecting to NATS: {}", server_addr);

    let client = options.connect(&server_addr).await?;
    
    info!("✅ NATS connection established");
    Ok(client)
}

/// Get JetStream context from client
pub fn jetstream(client: &async_nats::Client) -> jetstream::Context {
    jetstream::new(client.clone())
}

/// Health check - verify connection is alive
pub async fn health_check(client: &async_nats::Client) -> bool {
    // Flush ensures connection is alive
    client.flush().await.is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = NatsConfig::default();
        assert_eq!(config.urls, vec!["nats://localhost:4222"]);
        assert!(config.reconnect);
    }

    #[test]
    fn test_config_from_toml() {
        let config = NatsConfig::from_toml("nats://prod:4222", Some("kali-daemon"));
        assert_eq!(config.urls, vec!["nats://prod:4222"]);
        assert_eq!(config.name, Some("kali-daemon".to_string()));
    }
}
