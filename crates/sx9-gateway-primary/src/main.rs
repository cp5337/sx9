//! SX9 Gateway - Unified API surface for the SYNAPTIX9 cognitive engine
//!
//! This binary starts the WebSocket gateway that exposes:
//! - Database queries (Supabase, SurrealDB, Sled, Sledis)
//! - Graph operations (GLAF, fusion nodes)
//! - Workflow control (Forge, sx9-atlas-bus)
//! - PlasmaState monitoring (SDT gate, crystal resonance)
//! - Health/connection status
//!
//! ## Usage
//!
//! ```bash
//! # Start with default port (18600)
//! sx9-gateway
//!
//! # Start with custom port
//! sx9-gateway --port 8080
//!
//! # With debug logging
//! RUST_LOG=debug sx9-gateway
//! ```

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "sx9-gateway-primary")]
#[command(about = "SX9 Gateway Primary - Unified API gateway for all Synaptix9 operations")]
#[command(version)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 18600)]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    println!(r#"
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
    "#);
    
    sx9_gateway_primary::run_gateway(Some(args.port)).await
}

