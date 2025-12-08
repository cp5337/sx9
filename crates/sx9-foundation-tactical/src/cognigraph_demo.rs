//! CTAS SlotGraph Cognigraph Demo
//! 
//! Fire up Bevy ECS with Universal Cognigraph cognitive atoms!
//! Shows real-time SlotGraph analytics with 10 Universal Node Types.

use bevy::prelude::*;
use ctas_slotgraph_tools::*;

fn main() {
    println!("🧠 FIRING UP CTAS SLOTGRAPH COGNIGRAPH!");
    println!("🚀 Universal Cognigraph + Bevy ECS = Real-time Operational Intelligence");
    
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(SlotGraphPlugin)
        .add_systems(Startup, setup_cognigraph_demo)
        .add_systems(Update, demo_monitoring_system)
        .run();
}

fn setup_cognigraph_demo(mut commands: Commands) {
    info!("🎯 Setting up Cognigraph Demo with Universal Node Types");
    
    // Create the 10 Universal Node Types (B₁ through B₁₀)
    let universal_nodes = [
        (UniversalNodeType::Source, Vec3::new(0.0, 0.0, 0.0), "Energy Source"),
        (UniversalNodeType::Transformer, Vec3::new(10.0, 0.0, 0.0), "Data Transformer"),
        (UniversalNodeType::Router, Vec3::new(20.0, 0.0, 0.0), "Traffic Router"),
        (UniversalNodeType::Buffer, Vec3::new(30.0, 0.0, 0.0), "Resource Buffer"),
        (UniversalNodeType::Gate, Vec3::new(0.0, 10.0, 0.0), "Access Gate"),
        (UniversalNodeType::Monitor, Vec3::new(10.0, 10.0, 0.0), "System Monitor"),
        (UniversalNodeType::Catalyst, Vec3::new(20.0, 10.0, 0.0), "Process Catalyst"),
        (UniversalNodeType::Inhibitor, Vec3::new(30.0, 10.0, 0.0), "Flow Inhibitor"),
        (UniversalNodeType::Relay, Vec3::new(0.0, 20.0, 0.0), "Signal Relay"),
        (UniversalNodeType::Sink, Vec3::new(10.0, 20.0, 0.0), "Data Sink"),
    ];
    
    for (i, (node_type, position, name)) in universal_nodes.iter().enumerate() {
        let atom = CognitiveAtom::new(node_type.clone(), None, *position);
        
        commands.spawn((
            CognitiveAtomBundle::new(atom),
            Name::new(format!("B{}: {}", i + 1, name)),
        ));
        
        info!("✅ Spawned {} at {:?}", name, position);
    }
    
    // Spawn some CTAS domain-specific nodes
    let ctas_nodes = [
        (UniversalNodeType::Monitor, CTASNodeType::Intelligence, Vec3::new(40.0, 0.0, 0.0)),
        (UniversalNodeType::Transformer, CTASNodeType::Threat, Vec3::new(40.0, 10.0, 0.0)),
        (UniversalNodeType::Source, CTASNodeType::Asset, Vec3::new(40.0, 20.0, 0.0)),
    ];
    
    for (universal_type, ctas_type, position) in ctas_nodes.iter() {
        let atom = CognitiveAtom::new(universal_type.clone(), Some(ctas_type.clone()), *position);
        
        commands.spawn((
            CognitiveAtomBundle::new(atom),
            Name::new(format!("CTAS {:?}", ctas_type)),
        ));
        
        info!("🎯 Spawned CTAS {:?} node", ctas_type);
    }
    
    info!("🔥 Cognigraph Demo Setup Complete! {} cognitive atoms active", universal_nodes.len() + ctas_nodes.len());
}

fn demo_monitoring_system(
    query: Query<(&CognitiveAtom, &Name)>,
    time: Res<Time>,
    metrics: Res<AnalyticsMetrics>,
) {
    // Print status every 2 seconds
    if time.elapsed_seconds() as u32 % 2 == 0 && time.delta_seconds() < 0.1 {
        let total_atoms = query.iter().count();
        
        println!("\n🧠 COGNIGRAPH STATUS (T+{:.1}s):", time.elapsed_seconds());
        println!("   💫 Active Cognitive Atoms: {}", total_atoms);
        println!("   📊 Analytics Metrics: {} active nodes, {} critical alerts", 
                metrics.active_nodes, metrics.critical_alerts);
        
        // Show node type distribution
        let mut node_counts = std::collections::HashMap::new();
        for (atom, name) in query.iter() {
            *node_counts.entry(atom.node_type.clone()).or_insert(0) += 1;
            
            if atom.activation_state == ActivationState::Active {
                println!("   ⚡ {} is ACTIVE (B{})", name.as_str(), atom.atomic_number);
            }
        }
        
        println!("   🎯 Node Type Distribution:");
        for (node_type, count) in node_counts.iter() {
            println!("      B{}: {:?} = {} atoms", 
                    node_type.atomic_number(), node_type, count);
        }
        
        // Show energy metrics
        let total_energy: f32 = query.iter()
            .map(|(atom, _)| atom.energetic.net_energy_balance())
            .sum();
        
        println!("   ⚡ Total Energy Balance: {:.2}", total_energy);
        
        if time.elapsed_seconds() > 10.0 {
            println!("\n🎉 Demo complete! SlotGraph Cognigraph successfully running!");
            std::process::exit(0);
        }
    }
}
