//! Tool Result Handler
//!
//! Subscribes to tool results from Kali Plasma and processes through ANN

use anyhow::Result;
use chrono::Utc;
use futures_util::StreamExt;
use std::sync::Arc;
use tracing::{info, warn, error};
use base64::{Engine as _, engine::general_purpose};
use crate::ann_daemon::{AnnDaemon, AnnObservation, AnnConfig};
use crate::advisory::AnnContext;
use crate::plasma_bus::PlasmaBus;
use crate::PlasmaDefender;

/// Tool result message from Kali Plasma
#[derive(Debug, Clone, serde::Deserialize)]
struct ToolResultMessage {
    operator_id: String,
    tool: String,
    result: String,  // base64 encoded
    success: bool,
    timestamp: u64,
}

/// Internal subscription function (doesn't require PlasmaDefender clone)
pub async fn subscribe_tool_results_internal(
    plasma_bus: Arc<PlasmaBus>,
    ann_daemon: Arc<AnnDaemon>,
) -> Result<()> {
    // This will be called with evaluate_threat callback
    // For now, simplified version
    let nats = plasma_bus.nats.clone();
    let mut subscriber = nats.subscribe("sx9.tool.result.ann").await?;
    
    info!("✅ Subscribed to sx9.tool.result.ann");
    
    while let Some(msg) = subscriber.next().await {
        // Parse and process
        let tool_result: ToolResultMessage = match serde_json::from_slice(&msg.payload) {
            Ok(r) => r,
            Err(e) => {
                warn!("Failed to parse tool result: {}", e);
                continue;
            }
        };
        
        info!("Received tool result: {} from operator {}", tool_result.tool, tool_result.operator_id);
        
        // Decode result
        let result_bytes = match general_purpose::STANDARD.decode(&tool_result.result) {
            Ok(b) => b,
            Err(e) => {
                warn!("Base64 decode error: {}", e);
                continue;
            }
        };
        
        // Create observation (simplified - would need crystal/SDT evaluation)
        let ann_obs = AnnObservation {
            hash_entropy: 0.5, // Would calculate from result
            routing_latency_ns: 0,
            sdt_state: Some(1), // OPEN
            crystal_resonance: Some(0.5),
            timestamp: Utc::now(),
        };
        
        ann_daemon.observe(ann_obs).await?;
        
        // Get advisory
        let ctx = AnnContext {
            entropy: 0.5,
            latency_score: 1.0,
        };
        
        if let Some(advisory) = ann_daemon.get_advisory(&ctx).await? {
            info!("ANN Advisory: {} (confidence: {:.2})", advisory.recommendation, advisory.confidence);
            
            nats.publish(
                "sx9.plasma.ann.advisory",
                serde_json::to_vec(&advisory)?.into()
            ).await?;
        }
    }
    
    Ok(())
}

/// Subscribe to tool results and process through ANN (public API)
pub async fn subscribe_tool_results(
    defender: Arc<PlasmaDefender>,
    ann_daemon: Arc<AnnDaemon>,
) -> Result<()> {
    let nats = defender.plasma_bus.nats.clone();
    let mut subscriber = nats.subscribe("sx9.tool.result.ann").await?;
    
    info!("✅ Subscribed to sx9.tool.result.ann");
    
    tokio::spawn(async move {
        while let Some(msg) = subscriber.next().await {
            match handle_tool_result(&msg.payload, &defender, &ann_daemon).await {
                Ok(_) => {}
                Err(e) => {
                    error!("Error processing tool result: {}", e);
                }
            }
        }
    });
    
    Ok(())
}

/// Handle a single tool result message
async fn handle_tool_result(
    payload: &[u8],
    defender: &Arc<PlasmaDefender>,
    ann_daemon: &Arc<AnnDaemon>,
) -> Result<()> {
    // Parse message
    let tool_result: ToolResultMessage = serde_json::from_slice(payload)?;
    
    info!("Received tool result: {} from operator {}", tool_result.tool, tool_result.operator_id);
    
    // Decode result payload
    let result_bytes = base64::engine::general_purpose::STANDARD
        .decode(&tool_result.result)
        .map_err(|e| anyhow::anyhow!("Base64 decode error: {}", e))?;
    
    // Evaluate through crystal & SDT
    let threat_result = defender.evaluate_threat(&result_bytes).await?;
    
    // Create ANN observation
    let ann_obs = AnnObservation {
        hash_entropy: threat_result.ring_strength,
        routing_latency_ns: 0, // Not applicable for tool results
        sdt_state: Some(threat_result.sdt_state as u8),
        crystal_resonance: Some(threat_result.ring_strength),
        timestamp: Utc::now(),
    };
    
    // Feed to ANN daemon
    ann_daemon.observe(ann_obs).await?;
    
    // Get ANN advisory
    let ctx = AnnContext {
        entropy: threat_result.ring_strength,
        latency_score: if threat_result.allowed { 1.0 } else { 0.0 },
    };
    
    if let Some(advisory) = ann_daemon.get_advisory(&ctx).await? {
        info!(
            "ANN Advisory: {} (confidence: {:.2})",
            advisory.recommendation,
            advisory.confidence
        );
        
        // Publish advisory back to Kali Plasma
        let nats = defender.plasma_bus.nats.clone();
        nats.publish(
            "sx9.plasma.ann.advisory",
            serde_json::to_vec(&advisory)?.into()
        ).await?;
    }
    
    Ok(())
}

