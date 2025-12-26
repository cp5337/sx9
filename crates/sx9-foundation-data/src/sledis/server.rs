//! Sledis TCP Server
//!
//! Redis-compatible TCP server running on port 18401.

use super::{RespParser, RespValue, SledisCommand, SledisStore, SLEDIS_PORT};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Sledis server configuration
#[derive(Debug, Clone)]
pub struct SledisServerConfig {
    pub host: String,
    pub port: u16,
    pub db_path: String,
}

impl Default for SledisServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: SLEDIS_PORT,
            db_path: shellexpand::tilde("~/.sx9/sledis").to_string(),
        }
    }
}

/// Sledis server
pub struct SledisServer {
    config: SledisServerConfig,
    store: Arc<SledisStore>,
}

impl SledisServer {
    /// Create new server with config
    pub fn new(config: SledisServerConfig) -> Result<Self, super::SledisError> {
        let store = SledisStore::open(&config.db_path)?;
        Ok(Self {
            config,
            store: Arc::new(store),
        })
    }

    /// Create server with default config
    pub fn with_defaults() -> Result<Self, super::SledisError> {
        Self::new(SledisServerConfig::default())
    }

    /// Run the server (blocking)
    pub async fn run(&self) -> Result<(), super::SledisError> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = TcpListener::bind(&addr)
            .await
            .map_err(|e| super::SledisError::Connection(e.to_string()))?;

        tracing::info!("Sledis server listening on {}", addr);

        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    tracing::debug!("New connection from {}", addr);
                    let store = Arc::clone(&self.store);
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(socket, store).await {
                            tracing::error!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    tracing::error!("Accept error: {}", e);
                }
            }
        }
    }

    /// Get reference to store for direct access
    pub fn store(&self) -> &SledisStore {
        &self.store
    }
}

async fn handle_connection(
    mut socket: TcpStream,
    store: Arc<SledisStore>,
) -> Result<(), super::SledisError> {
    let mut buffer = vec![0u8; 4096];

    loop {
        let n = socket
            .read(&mut buffer)
            .await
            .map_err(|e| super::SledisError::Connection(e.to_string()))?;

        if n == 0 {
            // Connection closed
            return Ok(());
        }

        let data = buffer[..n].to_vec();

        // Try RESP parsing first
        let response = if data.starts_with(&[b'*']) || data.starts_with(&[b'$']) {
            parse_and_execute_resp(&data, &store)
        } else {
            // Inline command
            let line = String::from_utf8_lossy(&data);
            let line = line.trim();
            parse_and_execute_inline(line, &store)
        };

        let response_bytes = response.encode();
        socket
            .write_all(&response_bytes)
            .await
            .map_err(|e| super::SledisError::Connection(e.to_string()))?;
    }
}

fn parse_and_execute_resp(data: &[u8], store: &SledisStore) -> RespValue {
    let mut parser = RespParser::new(data.to_vec());

    match parser.parse() {
        Ok(RespValue::Array(Some(items))) => {
            let args: Vec<String> = items
                .into_iter()
                .filter_map(|item| match item {
                    RespValue::BulkString(Some(s)) => Some(s),
                    RespValue::SimpleString(s) => Some(s),
                    _ => None,
                })
                .collect();

            execute_command(&args, store)
        }
        Ok(_) => RespValue::err("ERR invalid request"),
        Err(e) => RespValue::err(format!("ERR {}", e)),
    }
}

fn parse_and_execute_inline(line: &str, store: &SledisStore) -> RespValue {
    let args: Vec<String> = line.split_whitespace().map(String::from).collect();
    if args.is_empty() {
        return RespValue::err("ERR empty command");
    }
    execute_command(&args, store)
}

fn execute_command(args: &[String], store: &SledisStore) -> RespValue {
    match SledisCommand::parse(args) {
        Ok(cmd) => match cmd.execute(store) {
            Ok(response) => response,
            Err(e) => RespValue::err(format!("ERR {}", e)),
        },
        Err(e) => RespValue::err(format!("ERR {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_default() {
        let config = SledisServerConfig::default();
        assert_eq!(config.port, SLEDIS_PORT);
        assert_eq!(config.host, "127.0.0.1");
    }
}
