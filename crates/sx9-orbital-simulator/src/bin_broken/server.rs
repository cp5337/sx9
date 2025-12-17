use ctas7_orbital_mechanics::config::Config;
use ctas7_orbital_mechanics::satellite_simulator::SatelliteSimulator;
use anyhow::Result;
use tokio::net::TcpListener;
use tracing::{info, error};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "18123".to_string())
        .parse::<u16>()?;

    info!("Starting Orbital Mechanics Server on port {}", port);

    let simulator = Arc::new(SatelliteSimulator::new());

    // Initialize constellation if configured
    if config.enable_constellation {
        simulator.initialize_constellation().await?;
        info!("Constellation initialized");
    }

    // Start TCP listener
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    info!("Server listening on port {}", port);

    // Basic health check endpoint
    let listener_clone = listener.try_clone()?;
    tokio::spawn(async move {
        loop {
            if let Ok((socket, addr)) = listener_clone.accept().await {
                info!("Accepted connection from {}", addr);
                // Connection handling logic would go here
                drop(socket);
            }
        }
    });

    // Keep the server running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        info!("Orbital Mechanics Server is running");
    }
}
