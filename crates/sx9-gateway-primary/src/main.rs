//! SX9 Gateway - Unified API surface for the SYNAPTIX9 cognitive engine
//!
//! This binary starts the WebSocket gateway that exposes:
//! - Database queries (Supabase, SurrealDB, Sled, Sledis)
//! - Graph operations (GLAF, fusion nodes)
//! - Workflow control (Forge, sx9-atlas-bus)
//! - PlasmaState monitoring (SDT gate, crystal resonance)
//! - Health/connection status
//!
//! ## Port Architecture
//! - Gateway WS:   18120 (primary), 28120 (mirror)
//! - Gateway REST: 18121
//! - Gateway gRPC: 18122
//!
//! ## Usage
//!
//! ```bash
//! # Start with default port (18120)
//! sx9-gateway
//!
//! # Start with custom port
//! sx9-gateway --port 8080
//!
//! # With debug logging
//! RUST_LOG=debug sx9-gateway
//! ```

use clap::Parser;
use tracing::{info, warn};

#[derive(Parser, Debug)]
#[command(name = "sx9-gateway-primary")]
#[command(about = "SX9 Gateway Primary - Unified API gateway for all Synaptix9 operations")]
#[command(version)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 18120)]
    port: u16,

    /// Port Manager URL for service registration
    #[arg(long, default_value = "http://localhost:18103")]
    port_manager: String,

    /// Skip Port Manager registration
    #[arg(long, default_value_t = false)]
    no_register: bool,
}

/// Register with Port Manager
async fn register_with_port_manager(url: &str, port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "service_name": "sx9-gateway-primary",
        "service_type": "gateway",
        "preferred_port": port,
        "request_mirror": true
    });

    let resp = client
        .post(format!("{}/register", url))
        .json(&payload)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await?;

    if resp.status().is_success() {
        let result: serde_json::Value = resp.json().await?;
        info!("✅ Registered with Port Manager: {:?}", result);
        Ok(())
    } else {
        anyhow::bail!("Port Manager returned {}", resp.status())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    //tracing_subscriber::fmt::init();

    let args = Args::parse();

    println!(
        r#"
    ╔═══════════════════════════════════════════════════════════════╗
    ║                                                               ║
    ║   ███████╗██╗  ██╗ █████╗        ██████╗  █████╗ ████████╗   ║
    ║   ██╔════╝╚██╗██╔╝██╔══██╗      ██╔════╝ ██╔══██╗╚══██╔══╝   ║
    ║   ███████╗ ╚███╔╝ ╚██████║█████╗██║  ███╗███████║   ██║      ║
    ║   ╚════██║ ██╔██╗  ╚═══██║╚════╝██║   ██║██╔══██║   ██║      ║
    ║   ███████║██╔╝ ██╗ █████╔╝      ╚██████╔╝██║  ██║   ██║      ║
    ║   ╚══════╝╚═╝  ╚═╝ ╚════╝        ╚═════╝ ╚═╝  ╚═╝   ╚═╝      ║
    ║                                                               ║
    ║   Unified Gateway for the SYNAPTIX9 Cognitive Engine          ║
    ║                                                               ║
    ╚═══════════════════════════════════════════════════════════════╝
    "#
    );

    // Register with Port Manager (unless disabled)
    if !args.no_register {
        match register_with_port_manager(&args.port_manager, args.port).await {
            Ok(()) => info!("🎯 Gateway registered on port {}", args.port),
            Err(e) => warn!(
                "⚠️ Port Manager registration failed: {} (continuing anyway)",
                e
            ),
        }
    }

    sx9_gateway_primary::run_gateway(Some(args.port)).await
}
