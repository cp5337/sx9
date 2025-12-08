//! Murmur3 Trivariate Base96 Hash Generation
//! High-speed contextual operations with SCH-CUID-UUID structure

use anyhow::Result;
use std::collections::HashMap;
use crate::TrivariatRequest;

// Base96 character set (91 actual characters)
const BASE96: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[]^_{|}~";

#[derive(Debug)]
pub struct TrivariatHash {
    pub sch: String,
    pub cuid: String,
    pub uuid: String,
    pub hash48: String,
    pub method: String,
    pub algorithm: String,
    pub seeds: HashMap<String, String>,
}

/// Generate Murmur3 Base96 trivariate hash (3x16 positions)
pub async fn generate_murmur3_trivariate(request: &TrivariatRequest) -> Result<TrivariatHash> {
    let timestamp = chrono::Utc::now().timestamp().to_string();
    
    // SCH - Positions 1-16 (semantic envelope) - Seed 0x1234
    let sch_data = format!("sch_{}_{}_{}",request.crate_name, request.stage, timestamp);
    let sch_hash = murmur3_to_base96(sch_data.as_bytes(), 16, 0x1234);
    
    // CUID - Positions 17-32 (spatio-temporal context) - Seed 0x5678
    let cuid_data = format!("cuid_{}_{}_{}", timestamp, request.crate_name, request.stage);
    let cuid_hash = murmur3_to_base96(cuid_data.as_bytes(), 16, 0x5678);
    
    // UUID - Positions 33-48 (persistence & audit) - Seed 0x9abc
    let uuid_data = format!("uuid_{}_{}_{}",request.crate_name, timestamp, request.stage);
    let uuid_hash = murmur3_to_base96(uuid_data.as_bytes(), 16, 0x9abc);
    
    // Full 48-position hash
    let hash48 = format!("{}{}{}", sch_hash, cuid_hash, uuid_hash);
    
    let mut seeds = HashMap::new();
    seeds.insert("sch".to_string(), "0x1234".to_string());
    seeds.insert("cuid".to_string(), "0x5678".to_string());
    seeds.insert("uuid".to_string(), "0x9abc".to_string());
    
    Ok(TrivariatHash {
        sch: sch_hash,
        cuid: cuid_hash,
        uuid: uuid_hash,
        hash48,
        method: "MURMUR3_BASE96".to_string(),
        algorithm: "murmur3_32bit_base96_encoding".to_string(),
        seeds,
    })
}

/// Pure Rust Murmur3 32-bit hash implementation
fn murmur3_hash(data: &[u8], seed: u32) -> u32 {
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;
    const R1: u32 = 15;
    const R2: u32 = 13;
    const M: u32 = 5;
    const N: u32 = 0xe6546b64;
    
    let mut h1 = seed;
    let length = data.len();
    
    // Process 4-byte chunks
    for chunk in data.chunks_exact(4) {
        let mut k1 = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(R1);
        k1 = k1.wrapping_mul(C2);
        
        h1 ^= k1;
        h1 = h1.rotate_left(R2);
        h1 = h1.wrapping_mul(M).wrapping_add(N);
    }
    
    // Process remaining bytes
    let remainder = &data[length - (length % 4)..];
    let mut k1 = 0u32;
    
    match remainder.len() {
        3 => {
            k1 ^= (remainder[2] as u32) << 16;
            k1 ^= (remainder[1] as u32) << 8;
            k1 ^= remainder[0] as u32;
        }
        2 => {
            k1 ^= (remainder[1] as u32) << 8;
            k1 ^= remainder[0] as u32;
        }
        1 => {
            k1 ^= remainder[0] as u32;
        }
        _ => {}
    }
    
    if remainder.len() > 0 {
        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(R1);
        k1 = k1.wrapping_mul(C2);
        h1 ^= k1;
    }
    
    // Finalization
    h1 ^= length as u32;
    h1 ^= h1 >> 16;
    h1 = h1.wrapping_mul(0x85ebca6b);
    h1 ^= h1 >> 13;
    h1 = h1.wrapping_mul(0xc2b2ae35);
    h1 ^= h1 >> 16;
    
    h1
}

/// Convert data to Base96 string using Murmur3 hashing
fn murmur3_to_base96(data: &[u8], length: usize, seed: u32) -> String {
    let mut result = String::with_capacity(length);
    
    for i in 0..length {
        // Use different seeds for each position to get variation
        let position_seed = seed.wrapping_add((i as u32).wrapping_mul(0x9e3779b9)); // Golden ratio multiplier
        
        // Create position-specific input
        let mut position_data = data.to_vec();
        position_data.extend_from_slice(&(i as u32).to_le_bytes());
        
        let hash_val = murmur3_hash(&position_data, position_seed);
        let char_index = (hash_val as usize) % BASE96.len();
        
        if let Some(ch) = BASE96.chars().nth(char_index) {
            result.push(ch);
        }
    }
    
    result
}