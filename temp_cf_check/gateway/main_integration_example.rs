// sx9-gateway/src/main.rs
//
// SX9 Gateway Main Entry Point
// Integration example for R2 CDN Subscriber

use anyhow::Result;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber;

// Import your existing modules
// use crate::neural_mux::NeuralMux;
// use crate::port_manager::PortManager;

// Import CDN modules
mod cdn;
use cdn::{R2Config, R2SubscriberService};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 Starting SX9 Gateway v7.1.1");

    // ========================================================================
    // STEP 1: Initialize Port Manager (your existing code)
    // ========================================================================
    
    // let port_manager = PortManager::new(18104).await?;
    // info!("✅ Port Manager initialized on port 18104");

    // ========================================================================
    // STEP 2: Initialize Neural Mux (your existing code)
    // ========================================================================
    
    // let neural_mux = NeuralMux::new(18107).await?;
    // info!("✅ Neural Mux initialized on port 18107");

    // ========================================================================
    // STEP 3: Initialize R2 CDN Subscriber (NEW!)
    // ========================================================================
    
    // Configure R2 service
    let r2_config = R2Config {
        port: 18127,
        sync_interval: std::time::Duration::from_secs(3600), // 1 hour
        cache_ttl: std::time::Duration::from_secs(7200),     // 2 hours
    };

    // Create R2 service
    let r2_service = Arc::new(R2SubscriberService::new(r2_config).await?);
    info!("✅ R2 CDN Subscriber initialized on port 18127");

    // Register with Port Manager (your existing code)
    // let r2_port = port_manager.allocate_port(
    //     "r2-cdn-subscriber",
    //     PortType::CDN,
    //     18127,
    // ).await?;

    // Spawn R2 service
    let r2_handle = tokio::spawn({
        let r2 = Arc::clone(&r2_service);
        async move {
            if let Err(e) = r2.start().await {
                tracing::error!("❌ R2 CDN subscriber error: {}", e);
            }
        }
    });

    // ========================================================================
    // STEP 4: Register with Neural Mux (your existing code)
    // ========================================================================
    
    // neural_mux.register_route(RouteEntry {
    //     hash_prefix: hash("r2-cdn"),
    //     target: RouteDest::LocalCDN {
    //         name: "r2-cdn-subscriber",
    //         port: 18127,
    //         protocol: Protocol::HTTP,
    //     },
    //     latency_zone: BernoulliZone::C,  // Analytical (<100ms)
    //     fallback: Some(RouteDest::Supabase),
    // })?;

    info!("✅ R2 CDN registered with Neural Mux");

    // ========================================================================
    // STEP 5: Start main gateway services (your existing code)
    // ========================================================================
    
    info!("🌐 SX9 Gateway fully initialized");
    info!("   - WebSocket: 18120");
    info!("   - REST API:  18121");
    info!("   - gRPC:      18122");
    info!("   - Port Mgr:  18104");
    info!("   - Neural Mux: 18107");
    info!("   - R2 CDN:    18127");

    // Wait for all services
    tokio::select! {
        _ = r2_handle => {
            info!("R2 service terminated");
        }
        _ = tokio::signal::ctrl_c() => {
            info!("🛑 Shutdown signal received");
        }
    }

    info!("👋 SX9 Gateway shutting down");
    Ok(())
}
