use std::process::Command;
use log::{info, error};

/// CTAS Task Binary: Obtain Operational Resources
/// Task ID: uuid-004-000-001
/// Category: Resource Development
/// HD4 Phase: Detect

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let target = std::env::args().nth(1).unwrap_or_else(|| "localhost".to_string());
    
    info!("🎯 CTAS Task: Obtain Operational Resources");
    info!("📍 Target: {}", target);
    
    match execute_task(&target).await {
        Ok(_) => {
            info!("✅ Task completed successfully");
            std::process::exit(0);
        }
        Err(e) => {
            error!("❌ Task failed: {}", e);
            info!("⚠️  Escalating to container...");
            escalate_to_container(&target);
        }
    }
}

async fn execute_task(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Execute primary tool
    let output = Command::new("nmap")
        .arg(target)
        .output()?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err("Tool execution failed".into())
    }
}

fn escalate_to_container(target: &str) {
    info!("🐳 Starting container execution...");
    let _ = Command::new("docker")
        .args(&["run", "--rm", "ctas7/kali-tools:7.3.1", "uuid-004-000-001", target])
        .spawn();
}
