use std::process::Command;
use log::{info, error};

/// CTAS Task Binary: Criminal Investigation
/// Task ID: uuid-010-003-001
/// Category: Defense Evasion
/// HD4 Phase: Dominate

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let target = std::env::args().nth(1).unwrap_or_else(|| "localhost".to_string());
    
    info!("🎯 CTAS Task: Criminal Investigation");
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
        .args(&["run", "--rm", "ctas7/kali-tools:7.3.1", "uuid-010-003-001", target])
        .spawn();
}
