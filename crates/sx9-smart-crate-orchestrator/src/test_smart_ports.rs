//! Test Smart Port Allocation System
//!
//! Tests the integration between Smart Crate Orchestrator, Port Manager,
//! and Neural Mux for intelligent port allocation.

use sx9_smart_crate_orchestrator::{
    SmartCrateOrchestrator, PortAllocationRequest, PortAllocationResponse
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .init();

    println!("🚀 Testing Smart Port Allocation System");
    println!("=======================================");

    // Initialize the orchestrator
    let orchestrator = SmartCrateOrchestrator::new("./templates", "./generated").await?;

    // Test 1: Basic port allocation
    println!("\n📦 Test 1: Basic Port Allocation");
    let request = PortAllocationRequest {
        service_name: "test-neural-service".to_string(),
        service_type: "neural-inference".to_string(),
        preferred_port: None,
        port_range: Some((19000, 19100)),
        neural_priority: 85,
        enable_deception: true,
    };

    match orchestrator.allocate_smart_port(request).await {
        Ok(allocation) => {
            println!("✅ Port allocated successfully!");
            println!("   Port: {}", allocation.port);
            println!("   Allocation ID: {}", allocation.allocation_id);
            println!("   Mirror Ports: {:?}", allocation.mirror_ports);
            println!("   Neural Score: {:.2}", allocation.neural_score);

            // Test port status
            println!("\n📊 Current Port Status:");
            match orchestrator.get_port_status().await {
                Ok(status) => {
                    println!("{}", serde_json::to_string_pretty(&status)?);
                }
                Err(e) => println!("⚠️  Failed to get port status: {}", e),
            }

            // Release the port
            println!("\n🔓 Releasing port allocation...");
            match orchestrator.release_port(&allocation.allocation_id).await {
                Ok(_) => println!("✅ Port released successfully"),
                Err(e) => println!("⚠️  Failed to release port: {}", e),
            }
        }
        Err(e) => {
            println!("❌ Port allocation failed: {}", e);
        }
    }

    // Test 2: High-priority neural service
    println!("\n📦 Test 2: High-Priority Neural Service");
    let request = PortAllocationRequest {
        service_name: "ctas7-neural-mux-processor".to_string(),
        service_type: "neural-mux".to_string(),
        preferred_port: Some(19050),
        port_range: Some((19000, 19200)),
        neural_priority: 95,
        enable_deception: false,
    };

    match orchestrator.allocate_smart_port(request).await {
        Ok(allocation) => {
            println!("✅ High-priority port allocated!");
            println!("   Port: {}", allocation.port);
            println!("   Neural Score: {:.2}", allocation.neural_score);

            // Keep this allocation active for demonstration
            println!("   Keeping allocation active for system demonstration");
        }
        Err(e) => {
            println!("❌ High-priority allocation failed: {}", e);
        }
    }

    // Test 3: WASM service with specific requirements
    println!("\n📦 Test 3: WASM Service Allocation");
    let request = PortAllocationRequest {
        service_name: "wasm-browser-processor".to_string(),
        service_type: "wasm-runtime".to_string(),
        preferred_port: None,
        port_range: Some((19100, 19200)),
        neural_priority: 70,
        enable_deception: true,
    };

    match orchestrator.allocate_smart_port(request).await {
        Ok(allocation) => {
            println!("✅ WASM service port allocated!");
            println!("   Port: {}", allocation.port);
            println!("   Deception Ports: {:?}", allocation.mirror_ports);
        }
        Err(e) => {
            println!("❌ WASM allocation failed: {}", e);
        }
    }

    println!("\n🎯 Smart Port Allocation Tests Complete");
    println!("======================================");

    Ok(())
}