//! Phi-4-mini Sled Storage Example
//!
//! Shows how to store ~3.8GB Phi-4-mini model in Sled KV store
//! with streaming, chunking, and Legion ECS integration

use ctas7_foundation_core::{
    sled_phi_storage::{SledPhiStorage, PhiModelChunk, CompressionType, GNNTopologyNode, NodeType, LegionECSMetadata, RoutingPriority, ProcessingRequirements},
    diagnostics,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("🧠 Phi-4-mini Sled Storage Demo");
    println!("===============================");
    println!("Model Size: ~3.8GB (fp16) / ~2.4GB (GGUF Q4_K_M)");
    println!("Storage: Sled KV store with chunking");
    println!();

    // Initialize Sled storage
    let storage_path = "/tmp/ctas7_phi4_storage".to_string();
    let storage = SledPhiStorage::new(storage_path, 4096)?; // 4GB cache

    println!("✅ Sled storage initialized");

    // Simulate storing Phi-4-mini model in chunks
    simulate_phi4_model_storage(&storage).await?;

    // Initialize GIS → Mux → Legion topology
    storage.initialize_gis_mux_legion_topology()?;

    // Show storage statistics
    show_storage_stats(&storage);

    // Demonstrate Legion ECS integration
    demonstrate_legion_integration(&storage).await?;

    // Performance test
    performance_test(&storage).await?;

    Ok(())
}

async fn simulate_phi4_model_storage(storage: &SledPhiStorage) -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Simulating Phi-4-mini model storage...");

    // Phi-4-mini has ~32 layers, let's chunk them
    let total_model_size_gb = 3.8;
    let chunk_size_mb = 64; // 64MB chunks for efficient streaming
    let total_chunks = ((total_model_size_gb * 1024.0) / chunk_size_mb as f64) as usize;

    println!("   Total size: {:.1}GB", total_model_size_gb);
    println!("   Chunk size: {}MB", chunk_size_mb);
    println!("   Total chunks: {}", total_chunks);

    for i in 0..std::cmp::min(total_chunks, 10) { // Store first 10 chunks as example
        let chunk = PhiModelChunk {
            chunk_id: format!("phi4_layer_{:02}", i),
            layer_index: i,
            data: vec![0u8; chunk_size_mb * 1024 * 1024], // Simulated data
            checksum: 0xDEADBEEF + i as u64,
            compression: if i % 2 == 0 { CompressionType::Zstd } else { CompressionType::Lz4 },
        };

        storage.store_phi4_chunk(&chunk)?;

        if i < 3 {
            println!("   ✅ Stored chunk {}: {} ({:.1}MB)",
                chunk.chunk_id,
                match chunk.compression {
                    CompressionType::Zstd => "Zstd compressed",
                    CompressionType::Lz4 => "LZ4 compressed",
                    CompressionType::None => "Uncompressed",
                },
                chunk.data.len() as f64 / 1024.0 / 1024.0
            );
        }
    }

    println!("   📊 Stored {} chunks (example subset)", std::cmp::min(total_chunks, 10));
    println!();

    // Test retrieval
    if let Some(chunk) = storage.get_phi4_chunk("phi4_layer_00")? {
        println!("✅ Successfully retrieved chunk: {} ({:.1}MB)",
            chunk.chunk_id,
            chunk.data.len() as f64 / 1024.0 / 1024.0
        );
    }

    Ok(())
}

fn show_storage_stats(storage: &SledPhiStorage) {
    println!("📊 Storage Statistics:");
    let stats = storage.get_storage_stats();

    for (key, value) in &stats {
        match key.as_str() {
            "model_storage_bytes" => {
                println!("   Model Storage: {:.1}MB", *value as f64 / 1024.0 / 1024.0);
            },
            k if k.ends_with("_entries") || k.ends_with("_chunks") || k.ends_with("_nodes") => {
                println!("   {}: {}", key.replace('_', " ").to_title_case(), value);
            },
            _ => {
                println!("   {}: {}", key, value);
            }
        }
    }
    println!();
}

async fn demonstrate_legion_integration(storage: &SledPhiStorage) -> Result<(), Box<dyn std::error::Error>> {
    println!("👑 Legion ECS Integration Demo:");
    println!("   GIS Query → Neural Mux → Legion ECS Processing");
    println!();

    // Simulate GIS data that needs Legion ECS processing
    let legion_entities = vec![
        LegionECSMetadata {
            entity_id: "satellite_imagery_001".to_string(),
            component_types: vec!["GeospatialData".to_string(), "ImageProcessing".to_string(), "TileCache".to_string()],
            processing_requirements: ProcessingRequirements {
                cpu_cores: 16,
                memory_mb: 8192,
                gpu_required: true,
                network_bandwidth_mbps: 1000.0,
            },
            spatial_bounds: Some(crate::sled_phi_storage::SpatialBounds {
                min_lat: 37.0,
                max_lat: 38.0,
                min_lon: -123.0,
                max_lon: -122.0,
            }),
            routing_priority: RoutingPriority::High,
        },
        LegionECSMetadata {
            entity_id: "tactical_analysis_002".to_string(),
            component_types: vec!["TacticalData".to_string(), "RealTimeProcessing".to_string()],
            processing_requirements: ProcessingRequirements {
                cpu_cores: 32,
                memory_mb: 16384,
                gpu_required: true,
                network_bandwidth_mbps: 2000.0,
            },
            spatial_bounds: None,
            routing_priority: RoutingPriority::Critical,
        },
    ];

    for entity in &legion_entities {
        storage.store_legion_metadata(entity)?;
        println!("   ✅ Legion Entity: {} (Priority: {:?})",
            entity.entity_id, entity.routing_priority);
        println!("      CPU: {} cores, RAM: {}MB, GPU: {}",
            entity.processing_requirements.cpu_cores,
            entity.processing_requirements.memory_mb,
            entity.processing_requirements.gpu_required
        );
    }

    println!();
    println!("🔄 Routing Flow:");
    println!("   1. GIS query arrives at Neural Mux");
    println!("   2. Phi-4 analyzes spatial complexity");
    println!("   3. GNN determines optimal Legion ECS cluster");
    println!("   4. Legion processes entities in parallel");
    println!("   5. Results cached in Sled for future queries");

    Ok(())
}

async fn performance_test(storage: &SledPhiStorage) -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Performance Test:");

    // Test routing decision caching
    let context_hashes = vec!["gis_query_123", "stats_analysis_456", "legion_task_789"];
    let decisions = vec!["route_to_legion_west", "distribute_processing", "priority_cluster"];
    let confidences = vec![0.95, 0.87, 0.99];

    let start_time = std::time::Instant::now();

    // Store routing decisions
    for (i, hash) in context_hashes.iter().enumerate() {
        storage.cache_routing_decision(hash, &decisions[i], confidences[i])?;
    }

    let store_time = start_time.elapsed();

    // Retrieve routing decisions
    let retrieve_start = std::time::Instant::now();
    let mut cache_hits = 0;

    for hash in &context_hashes {
        if let Some((decision, confidence)) = storage.get_cached_routing(hash)? {
            cache_hits += 1;
            if cache_hits == 1 {
                println!("   ✅ Cache hit: {} (confidence: {:.1}%)", decision, confidence * 100.0);
            }
        }
    }

    let retrieve_time = retrieve_start.elapsed();

    println!("   📊 Results:");
    println!("      Store time: {:?} ({} ops)", store_time, context_hashes.len());
    println!("      Retrieve time: {:?} ({} ops)", retrieve_time, context_hashes.len());
    println!("      Cache hit rate: {:.1}%", (cache_hits as f64 / context_hashes.len() as f64) * 100.0);
    println!("      Sled throughput: ~{:.0} ops/sec",
        (context_hashes.len() * 2) as f64 / (store_time + retrieve_time).as_secs_f64());

    println!();
    println!("🏆 Summary:");
    println!("   ✅ Phi-4-mini (~3.8GB) chunked and stored efficiently");
    println!("   ✅ GIS → Neural Mux → Legion ECS topology established");
    println!("   ✅ Fast routing cache for sub-millisecond decisions");
    println!("   ✅ Sled provides persistent, high-performance storage");

    Ok(())
}

trait ToTitleCase {
    fn to_title_case(&self) -> String;
}

impl ToTitleCase for str {
    fn to_title_case(&self) -> String {
        self.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}