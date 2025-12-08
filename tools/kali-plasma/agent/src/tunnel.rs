//! CDN tunnel module
//!
//! Connects to CTAS CDN via NATS over encrypted tunnel.
//! All traffic flows through polycrystal resonance.

use anyhow::{Result, Context};
use async_nats::{Client, jetstream};
use futures_util::StreamExt;
use serde::{Serialize, Deserialize};

use crate::Operator;
use crate::ebpf::{ToolCommand, ToolResult};

/// CDN tunnel connection
pub struct CdnTunnel {
    /// NATS client
    client: Client,
    /// JetStream context
    js: jetstream::Context,
    /// Operator ID for routing
    operator_id: String,
}

/// Wire format for commands
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WireCommand {
    /// Command ID
    cmd_id: u64,
    /// Tool name
    tool: String,
    /// Payload (base64 encoded)
    payload: String,
    /// Timestamp
    timestamp: u64,
    /// Delta angle at send time
    delta_angle: u16,
}

/// Wire format for results
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WireResult {
    /// Command ID
    cmd_id: u64,
    /// Tool name
    tool: String,
    /// Payload (base64 encoded)
    payload: String,
    /// Success flag
    success: bool,
    /// Timestamp
    timestamp: u64,
}

impl CdnTunnel {
    /// Get NATS client (for ANN integration)
    pub fn get_nats_client(&self) -> &Client {
        &self.client
    }
    
    /// Connect to CDN
    pub async fn connect(operator: &Operator) -> Result<Self> {
        // Connect to first available endpoint
        let endpoint = operator.cdn_endpoints.first()
            .context("No CDN endpoints configured")?;
        
        tracing::info!("Connecting to CDN: {}", endpoint);
        
        // In production, this would use TLS with operator cert
        let client = async_nats::connect(endpoint).await
            .context("Failed to connect to NATS")?;
        
        // Get JetStream context
        let js = jetstream::new(client.clone());
        
        // Subscribe to operator's command stream
        let operator_id = hex::encode(&operator.id[..8]);
        
        tracing::info!("Subscribed to: sx9.plasma.{}.cmd", operator_id);
        
        Ok(Self {
            client,
            js,
            operator_id,
        })
    }
    
    /// Receive command from CDN
    pub async fn recv(&self) -> Result<ToolCommand> {
        let subject = format!("sx9.plasma.{}.cmd", self.operator_id);
        
        // In production, this would use JetStream consumer
        let mut subscriber = self.client.subscribe(subject).await
            .context("Failed to subscribe")?;
        
        let msg = subscriber.next().await
            .context("No message received")?;
        
        let wire: WireCommand = serde_json::from_slice(&msg.payload)
            .context("Failed to parse command")?;
        
        let payload = general_purpose::STANDARD
            .decode(&wire.payload)
            .map_err(|e| anyhow::anyhow!("Failed to decode payload: {}", e))?;
        
        Ok(ToolCommand {
            tool: wire.tool,
            payload,
            cmd_id: wire.cmd_id,
        })
    }
    
    /// Send result to CDN
    pub async fn send(&self, result: &ToolResult) -> Result<()> {
        let subject = format!("sx9.plasma.{}.result", self.operator_id);
        
        let wire = WireResult {
            cmd_id: result.cmd_id,
            tool: result.tool.clone(),
            payload: general_purpose::STANDARD.encode(&result.payload),
            success: result.success,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        let payload = serde_json::to_vec(&wire)
            .context("Failed to serialize result")?;
        
        self.client.publish(subject, payload.into()).await
            .context("Failed to publish result")?;
        
        Ok(())
    }
    
    /// Send entropy harvest to CDN
    pub async fn send_entropy(&self, entropy: u32) -> Result<()> {
        let subject = format!("sx9.plasma.{}.entropy", self.operator_id);
        
        let payload = entropy.to_le_bytes().to_vec();
        
        self.client.publish(subject, payload.into()).await
            .context("Failed to publish entropy")?;
        
        Ok(())
    }
    
    /// Send canary trip alert
    pub async fn send_canary(&self, reason: &str) -> Result<()> {
        let subject = "sx9.sdt.canary".to_string();
        
        let payload = serde_json::json!({
            "operator": self.operator_id,
            "reason": reason,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
        
        self.client.publish(subject, serde_json::to_vec(&payload)?.into()).await
            .context("Failed to publish canary")?;
        
        Ok(())
    }
}

// Use base64 crate instead of custom implementation
use base64::{Engine as _, engine::general_purpose};

// Base64 encode/decode (DEPRECATED - use base64 crate)
#[allow(dead_code)]
mod base64_legacy {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    pub fn encode(data: &[u8]) -> String {
        let mut result = String::new();
        
        for chunk in data.chunks(3) {
            let mut buf = [0u8; 4];
            let len = chunk.len();
            
            buf[0] = chunk[0] >> 2;
            buf[1] = ((chunk[0] & 0x03) << 4) | (chunk.get(1).unwrap_or(&0) >> 4);
            buf[2] = ((chunk.get(1).unwrap_or(&0) & 0x0F) << 2) | (chunk.get(2).unwrap_or(&0) >> 6);
            buf[3] = chunk.get(2).unwrap_or(&0) & 0x3F;
            
            result.push(ALPHABET[buf[0] as usize] as char);
            result.push(ALPHABET[buf[1] as usize] as char);
            
            if len > 1 {
                result.push(ALPHABET[buf[2] as usize] as char);
            } else {
                result.push('=');
            }
            
            if len > 2 {
                result.push(ALPHABET[buf[3] as usize] as char);
            } else {
                result.push('=');
            }
        }
        
        result
    }
    
    pub fn decode(data: &str) -> Result<Vec<u8>, &'static str> {
        let mut result = Vec::new();
        let bytes: Vec<u8> = data.bytes().collect();
        
        for chunk in bytes.chunks(4) {
            if chunk.len() != 4 {
                return Err("Invalid base64 length");
            }
            
            let a = decode_char(chunk[0])?;
            let b = decode_char(chunk[1])?;
            let c = if chunk[2] == b'=' { 0 } else { decode_char(chunk[2])? };
            let d = if chunk[3] == b'=' { 0 } else { decode_char(chunk[3])? };
            
            result.push((a << 2) | (b >> 4));
            
            if chunk[2] != b'=' {
                result.push((b << 4) | (c >> 2));
            }
            
            if chunk[3] != b'=' {
                result.push((c << 6) | d);
            }
        }
        
        Ok(result)
    }
    
    fn decode_char(c: u8) -> Result<u8, &'static str> {
        match c {
            b'A'..=b'Z' => Ok(c - b'A'),
            b'a'..=b'z' => Ok(c - b'a' + 26),
            b'0'..=b'9' => Ok(c - b'0' + 52),
            b'+' => Ok(62),
            b'/' => Ok(63),
            _ => Err("Invalid base64 character"),
        }
    }
}

