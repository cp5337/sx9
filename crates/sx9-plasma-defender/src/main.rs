//! Plasma-Defender Service Entry Point
//!
//! Starts the Plasma-Defender service with:
//! - HTTP server for health/metrics
//! - NATS subscription for tool results
//! - ANN daemon for threat analysis

use anyhow::Result;
use sx9_plasma_defender::{DefenderConfig, PlasmaDefender};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    sx9_foundation_core::diagnostics::tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("=== Plasma-Defender Service ===");

    // Load configuration
    let config = DefenderConfig::default(); // TODO: Load from file/env
    info!("Configuration loaded: ANN enabled = {}", config.ann_enabled);

    // Initialize Plasma-Defender
    let defender = PlasmaDefender::new(config).await?;
    info!("✅ Plasma-Defender initialized");

    // Start service (this will spawn tool result subscription)
    info!("🚀 Starting Plasma-Defender service...");
    if let Err(e) = defender.start().await {
        error!("Service error: {}", e);
        return Err(e);
    }

    Ok(())
}
