//! RFC-9001 — Synaptix9 Trivariate Hashing Standard
//!
//! Implements the canonical hashing standard for CTAS-7.3.1.
//!
//! # Components
//! - SCH (Synaptic Convergent Hash): Murmur3-128 based content hash
//! - CUID (Contextual Unique Identifier): 16-char Base96 context hash
//! - UUID (Universally Unique Identifier): UUIDv7 for lineage
//!
//! # Format
//! `triv:[SCH]_[CUID]_[UUID]`

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;
use chrono::Utc;
use crate::hash64::{murmur3_64_base96, encode_base96, seeds};

/// Trivariate Hash Structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TrivariateHash {
    pub sch: String,
    pub cuid: String,
    pub uuid: Uuid,
}

impl TrivariateHash {
    /// Generate a new Trivariate Hash
    pub fn new(content: &str, context: &ContextFrame) -> Self {
        let sch = generate_sch(content, context);
        let cuid = generate_cuid(context);
        // TODO: Use UUIDv7 when feature is enabled. Fallback to v4 for now if v7 fails or isn't available.
        // Ideally: let uuid = Uuid::now_v7();
        let uuid = Uuid::new_v4(); 

        Self { sch, cuid, uuid }
    }

    /// Get canonical string representation
    pub fn to_canonical_string(&self) -> String {
        format!("triv:{}_{}_{}", self.sch, self.cuid, self.uuid)
    }
}

impl fmt::Display for TrivariateHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_canonical_string())
    }
}

/// Context Frame for Hash Generation
#[derive(Debug, Clone)]
pub struct ContextFrame {
    pub domain_mask: u8,
    pub execution_mask: u8,
    pub delta_angle: f32,
    pub agent_id: u16,
    pub exec_env: u8,
    pub lineage: u16,
    pub nonce: u16,
}

impl Default for ContextFrame {
    fn default() -> Self {
        Self {
            domain_mask: 0,
            execution_mask: 0,
            delta_angle: 0.0,
            agent_id: 0,
            exec_env: 0,
            lineage: 0,
            nonce: 0,
        }
    }
}

/// Generate SCH (Synaptic Convergent Hash)
///
/// Uses Murmur3-64 on:
/// - Raw operation text (normalized NVNN)
/// - Domain bitmask
/// - Execution bitmask
/// - Delta angle class
///
/// RFC-9001: SCH is 16 Base96 characters (64-bit hash)
pub fn generate_sch(content: &str, context: &ContextFrame) -> String {
    let normalized_content = normalize_nvnn(content);
    let input = format!(
        "{}:{}:{}:{}",
        normalized_content,
        context.domain_mask,
        context.execution_mask,
        get_delta_angle_class(context.delta_angle)
    );

    // Use canonical Base96 encoding (RFC-9001 Section 4.1)
    murmur3_64_base96(input.as_bytes(), seeds::SCH, 16)
}

/// Normalize content to NVNN grammar (Noun Verb Noun Noun)
fn normalize_nvnn(content: &str) -> String {
    // Simple normalization: lowercase, trim, collapse spaces
    // In a real implementation, this would parse and restructure to NVNN
    content.trim().to_lowercase().split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Get Delta Angle Class
fn get_delta_angle_class(angle: f32) -> u8 {
    match angle {
        a if a < 2.0 => 0,   // None
        a if a < 10.0 => 1,  // Micro
        a if a < 25.0 => 2,  // Soft
        a if a < 60.0 => 3,  // Hard
        _ => 4,              // Critical
    }
}

/// Generate CUID (Contextual Unique Identifier)
///
/// 16 characters, Base96 (RFC-9001 Section 6.1)
/// Slots:
/// 1–4: Timestamp shard
/// 5–7: Execution Env
/// 8–9: Agent ID
/// 10–11: Delta-Angle Derivative
/// 12: State Flag
/// 13–14: Lineage
/// 15–16: Nonce
pub fn generate_cuid(context: &ContextFrame) -> String {
    let timestamp = Utc::now().timestamp() as u64;
    
    // Pack context into input bytes for hashing
    let mut input = Vec::with_capacity(16);
    input.extend_from_slice(&timestamp.to_be_bytes());
    input.push(context.exec_env);
    input.extend_from_slice(&context.agent_id.to_be_bytes());
    
    let delta_class = get_delta_angle_class(context.delta_angle);
    input.push(delta_class);
    input.push(1); // State flag (default Warm)
    input.extend_from_slice(&context.lineage.to_be_bytes());
    input.extend_from_slice(&context.nonce.to_be_bytes());
    
    // Use canonical Base96 encoding (RFC-9001 Section 4.1)
    murmur3_64_base96(&input, seeds::CUID, 16)
}

/// Supersession Logic
#[derive(Debug, PartialEq, Eq)]
pub enum SupersessionType {
    None,
    Micro,
    Soft,
    Hard,
    Critical,
}

pub fn check_supersession(delta_angle: f32) -> SupersessionType {
    match delta_angle {
        a if a < 2.0 => SupersessionType::None,
        a if a < 10.0 => SupersessionType::Micro,
        a if a < 25.0 => SupersessionType::Soft,
        a if a < 60.0 => SupersessionType::Hard,
        _ => SupersessionType::Critical,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sch_generation() {
        let context = ContextFrame::default();
        let sch = generate_sch("test operation", &context);
        assert_eq!(sch.len(), 16); // RFC-9001: SCH is 16 Base96 chars
    }

    #[test]
    fn test_cuid_generation() {
        let context = ContextFrame::default();
        let cuid = generate_cuid(&context);
        assert_eq!(cuid.len(), 16);
    }

    #[test]
    fn test_trivariate_hash_format() {
        let context = ContextFrame::default();
        let hash = TrivariateHash::new("test", &context);
        let canonical = hash.to_canonical_string();
        assert!(canonical.starts_with("triv:"));
        // triv: + 16 (SCH) + 1 (_) + 16 (CUID) + 1 (_) + 36 (UUID) = 70 chars
        assert_eq!(canonical.len(), 70);
    }
}
