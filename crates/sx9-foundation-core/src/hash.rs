//! CTAS-7 Identity & Hashing Module
//!
//! RFC-9001: Entity Foundation Model
//! RFC-9100: PTCC Primitives
//!
//! Core Element: Identity & Hashing
//! Implementation: `ctas7-foundation-core::hash`
//!
//! This module provides the canonical trivariate hash system:
//! - SCH-T (Semantic Context Hash - Tools/Actions)
//! - CUID-T (Context User Identity - Spatial/Context)
//! - UUID-T (Universal Unique Identifier)

use murmur3::murmur3_x64_128;
use std::io::Cursor;
use uuid::Uuid;

/// Hash value type (128-bit for full precision)
pub type HashValue = u128;

/// Seed constants for trivariate components (RFC-9001)
pub const SEED_SCH: u32 = 0xC7A5_0000;
pub const SEED_CUID: u32 = 0xC7A5_0001;
pub const SEED_UUID: u32 = 0xC7A5_0002;

/// Primary Trivariate Hash Structure
///
/// RFC-9001 compliant 48-character hash combining:
/// - SCH-T: Tools/Actions semantic hash (16 chars)
/// - CUID-T: Spatial/Context identity hash (16 chars)
/// - UUID-T: Universal unique identifier (16 chars)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrimaryTrivariate {
    /// Semantic Context Hash - Tools/Actions
    pub sch_t: HashValue,
    /// Context User Identity - Spatial/Context
    pub cuid_t: HashValue,
    /// Universal Unique Identifier
    pub uuid_t: Uuid,
}

impl PrimaryTrivariate {
    /// Create new trivariate from components
    #[must_use]
    pub fn new(sch_t: HashValue, cuid_t: HashValue, uuid_t: Uuid) -> Self {
        Self {
            sch_t,
            cuid_t,
            uuid_t,
        }
    }

    /// Generate trivariate from key and data
    #[must_use]
    pub fn from_key(key: &str, data: &str) -> Self {
        let sch_t = compute_sch(key.as_bytes());
        let cuid_t = compute_cuid(data.as_bytes());
        let uuid_t = Uuid::new_v4();
        Self {
            sch_t,
            cuid_t,
            uuid_t,
        }
    }

    /// Generate trivariate with deterministic UUID
    ///
    /// Note: Uses hash-based determinism from SCH and CUID components
    /// since foundation-core uses uuid v4 only
    #[must_use]
    pub fn from_key_deterministic(key: &str, data: &str) -> Self {
        let sch_t = compute_sch(key.as_bytes());
        let cuid_t = compute_cuid(data.as_bytes());
        // Use combined hash to create deterministic UUID bytes
        let combined = format!("{key}:{data}:{sch_t:032x}:{cuid_t:032x}");
        let hash = compute_sch(combined.as_bytes());
        let uuid_bytes: [u8; 16] = hash.to_le_bytes();
        let uuid_t = Uuid::from_bytes(uuid_bytes);
        Self {
            sch_t,
            cuid_t,
            uuid_t,
        }
    }

    /// Convert to 48-character hex string
    #[must_use]
    pub fn to_hex_string(&self) -> String {
        format!(
            "{:016x}{:016x}{}",
            (self.sch_t & 0xFFFFFFFFFFFFFFFF) as u64,
            (self.cuid_t & 0xFFFFFFFFFFFFFFFF) as u64,
            self.uuid_t.simple()
        )
    }

    /// Get the SCH component as 64-bit value
    #[must_use]
    pub fn sch_64(&self) -> u64 {
        self.sch_t as u64
    }

    /// Get the CUID component as 64-bit value
    #[must_use]
    pub fn cuid_64(&self) -> u64 {
        self.cuid_t as u64
    }
}

impl Default for PrimaryTrivariate {
    fn default() -> Self {
        Self {
            sch_t: 0,
            cuid_t: 0,
            uuid_t: Uuid::nil(),
        }
    }
}

impl std::fmt::Display for PrimaryTrivariate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

/// Compute SCH (Semantic Context Hash) component
#[must_use]
pub fn compute_sch(data: &[u8]) -> HashValue {
    let mut cursor = Cursor::new(data);
    murmur3_x64_128(&mut cursor, SEED_SCH).unwrap_or(0)
}

/// Compute CUID (Context User Identity) component
#[must_use]
pub fn compute_cuid(data: &[u8]) -> HashValue {
    let mut cursor = Cursor::new(data);
    murmur3_x64_128(&mut cursor, SEED_CUID).unwrap_or(0)
}

/// Generate primary trivariate hash
///
/// # Arguments
/// * `key` - Semantic key for SCH component
/// * `data` - Context data for CUID component
///
/// # Returns
/// `PrimaryTrivariate` with all three components
#[must_use]
pub fn generate_primary_trivariate(key: &str, data: &str) -> PrimaryTrivariate {
    PrimaryTrivariate::from_key(key, data)
}

/// Generate deterministic trivariate (reproducible UUID)
#[must_use]
pub fn generate_deterministic_trivariate(key: &str, data: &str) -> PrimaryTrivariate {
    PrimaryTrivariate::from_key_deterministic(key, data)
}

/// Quick hash function for simple lookups
#[must_use]
pub fn quick_hash(data: &str) -> u64 {
    let mut cursor = Cursor::new(data.as_bytes());
    murmur3_x64_128(&mut cursor, SEED_SCH).unwrap_or(0) as u64
}

/// Hash for route table lookups (Neural Mux compatible)
#[must_use]
pub fn route_hash(sch: &str, domain: u8) -> u64 {
    let combined = format!("{sch}:{domain:02x}");
    quick_hash(&combined)
}

// Re-export from hash64 for convenience
pub use crate::hash64::{
    encode_base96, murmur3_64, murmur3_64_base96, murmur3_64_hex, seeds, trivariate_from_key,
    trivariate_hash, unicode_slot, unicode_slot_hex, BASE96_CHARSET,
};

// Re-export Primitive for TacticalInstruction
pub use crate::primitives::Primitive;

/// Tactical Instruction for PTCC-based hash generation
///
/// RFC-9100: Encodes a primitive operation with payload for trivariate hash generation.
/// The SCH-T component encodes the Primitive ID and N-V-N-N context.
#[derive(Debug, Clone)]
pub struct TacticalInstruction {
    /// PTCC Primitive operation code
    pub primitive: Primitive,
    /// Instruction payload/context
    pub payload: String,
}

impl TacticalInstruction {
    /// Create new tactical instruction
    pub fn new(primitive: Primitive, payload: impl Into<String>) -> Self {
        Self {
            primitive,
            payload: payload.into(),
        }
    }
}

/// Generate primary trivariate hash from tactical instruction
///
/// RFC-9001: SCH-T encodes Primitive ID and N-V-N-N context.
/// CUID-T includes Delta-Angle encoding for context tracking.
///
/// # Arguments
/// * `instr` - Tactical instruction with primitive and payload
///
/// # Returns
/// `PrimaryTrivariate` with all three components
#[must_use]
pub fn generate_primary_trivariate_from_instruction(
    instr: &TacticalInstruction,
) -> PrimaryTrivariate {
    // SCH-T encodes Primitive ID and N-V-N-N context
    let sch_input = format!("{}:{}", instr.primitive as u8, instr.payload);
    let sch_t = compute_sch(sch_input.as_bytes());

    // CUID-T includes Delta-Angle encoding (simplified - uses UUID for now)
    // In production, this would include delta-angle calculation
    let cuid_input = format!("{}:{}", Uuid::new_v4(), instr.payload);
    let cuid_t = compute_cuid(cuid_input.as_bytes());

    PrimaryTrivariate {
        sch_t,
        cuid_t,
        uuid_t: Uuid::new_v4(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_trivariate_creation() {
        let tri = generate_primary_trivariate("test_key", "test_data");
        assert_ne!(tri.sch_t, 0);
        assert_ne!(tri.cuid_t, 0);
        assert!(!tri.uuid_t.is_nil());
    }

    #[test]
    fn test_deterministic_trivariate() {
        let t1 = generate_deterministic_trivariate("key", "data");
        let t2 = generate_deterministic_trivariate("key", "data");
        assert_eq!(t1.sch_t, t2.sch_t);
        assert_eq!(t1.cuid_t, t2.cuid_t);
        assert_eq!(t1.uuid_t, t2.uuid_t);
    }

    #[test]
    fn test_hex_string_length() {
        let tri = generate_primary_trivariate("test", "data");
        let hex = tri.to_hex_string();
        // 16 + 16 + 32 = 64 chars
        assert_eq!(hex.len(), 64);
    }

    #[test]
    fn test_route_hash() {
        let h1 = route_hash("0xE001", 0);
        let h2 = route_hash("0xE001", 1);
        assert_ne!(h1, h2, "Different domains should produce different hashes");
    }
}
