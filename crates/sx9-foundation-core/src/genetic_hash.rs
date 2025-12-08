// ✅ Blake3 ACCEPTABLE in this file for:
//    - Genetic hash tracking (dev stage markers)
//    - Evolutionary tracking (Command/Control level)
//    - NOT for operational addressing
//! Genetic Hash Generation
//! Tracks code evolution through QA stages using Blake3

use anyhow::Result;
use blake3::Hasher;
use serde::{Deserialize, Serialize};
use crate::GeneticHashRequest;

#[derive(Debug, Serialize)]
pub struct GeneticHash {
    pub hash: String,
    pub method: String,
    pub evolution_stage: String,
    pub lineage_chain: Vec<String>,
}

/// Generate genetic hash for QA stage evolution
pub async fn generate_genetic_hash(request: &GeneticHashRequest) -> Result<GeneticHash> {
    // Create genetic hash input combining all evolution factors
    let genetic_input = format!(
        "{}:{}:{}:{}:{}:{}",
        request.stage,
        request.crate_name,
        request.context,
        request.previous_hash.as_deref().unwrap_or(""),
        request.source_data,
        request.timestamp
    );

    // Generate Blake3 genetic hash
    let mut hasher = Hasher::new();
    hasher.update(genetic_input.as_bytes());
    let genetic_hash = hasher.finalize().to_hex().to_string();

    // Build lineage chain
    let mut lineage_chain = vec![];
    if let Some(prev_hash) = &request.previous_hash {
        lineage_chain.push(prev_hash[..16].to_string()); // Previous generation
    }
    lineage_chain.push(genetic_hash[..16].to_string()); // Current generation

    Ok(GeneticHash {
        hash: genetic_hash,
        method: "blake3_genetic_evolution".to_string(),
        evolution_stage: get_evolution_stage(&request.stage),
        lineage_chain,
    })
}

fn get_evolution_stage(stage: &str) -> String {
    match stage {
        "QA0" => "RawCode".to_string(),           // Baseline genetic material
        "QA1" => "Adapting".to_string(),          // Tesla standards adaptation
        "QA2" => "Hardening".to_string(),         // Security/fratricide hardening
        "QA3" => "Optimizing".to_string(),        // Performance optimization
        "QA5" => "SystemIntegrated".to_string(),  // Full system integration
        _ => format!("Unknown({})", stage),
    }
}