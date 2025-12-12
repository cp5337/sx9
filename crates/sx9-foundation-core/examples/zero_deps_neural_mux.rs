use sx9_foundation_core::{diagnostics, unified_neural_mux::UnifiedNeuralMux};

/// Zero-Dependency Neural Mux Demo
///
/// Demonstrates the core neural multiplexing capabilities without external dependencies.
/// Uses the UnifiedNeuralMux for routing operations across different system components.
#[tokio::main]
async fn main() -> diagnostics::Result<()> {
    // Initialize logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    diagnostics::info!("Initializing Zero-Dependency Neural Mux Demo...");
    diagnostics::info!("System: CTAS-7 Foundation Core");
    diagnostics::info!("Mode: Autonomous Routing");

    // Initialize the neural mux
    let mut neural_mux = UnifiedNeuralMux::new();

    diagnostics::info!(" Neural Mux initialized successfully");

    // Simulate some unified operations for routing
    let test_ops = vec![
        ('\u{E001}', "System Keepalive"),
        ('\u{E300}', "Intelligence Query"),
        ('\u{E400}', "Environmental Scan"),
    ];

    print_header();

    for (op_char, description) in test_ops {
        diagnostics::info!("Processing request: {} ({})", description, op_char);

        let op_str = op_char.to_string();
        // Use default priority and source system for the demo
        match neural_mux
            .route_unified_operation(
                &op_str,
                "LegacyDemo",
                sx9_foundation_core::neural_mux::Priority::Medium,
            )
            .await
        {
            Ok(routes) => {
                for route in routes {
                    println!(
                        "✓ Routed: {} -> {} (Priority: {:?})",
                        description, route.target_processor, route.priority
                    );
                }
            }
            Err(e) => {
                println!("✗ Routing Failed: {}", e);
            }
        }
    }

    print_footer();

    Ok(())
}

fn print_header() {
    println!("\n==================================================");
    println!("   NON-DETERMINISTIC NEURAL ROUTING MATRIX");
    println!("==================================================\n");
}

fn print_footer() {
    println!("\n==================================================");
    println!("   END OF DEMONSTRATION");
    println!("==================================================\n");
}
