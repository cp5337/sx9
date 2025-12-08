//! CTAS v7.0 Core CLI binary
//! 
//! Main entry point for Tachyon Cortex CLI

use anyhow::Result;
use tracing::{info, error};

use ctas7_v7_core::cli::{create_cli_app, execute_cli_command};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    info!("🚀 CTAS v7.0 Core - Tachyon Cortex CLI starting...");
    
    // Parse command line arguments
    let matches = create_cli_app().get_matches();
    
    // Execute command
    if let Err(e) = execute_cli_command(matches).await {
        error!("Command failed: {}", e);
        std::process::exit(1);
    }
    
    info!("✅ Command completed successfully");
    Ok(())
}

