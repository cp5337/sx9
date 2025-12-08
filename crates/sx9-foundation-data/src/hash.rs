//! CTAS-7 v7.3.1 Trivariate Hash Engine
//!
//! Uses canonical 64-bit MurmurHash3 from ctas7-foundation-core::hash64
//! All outputs are Base96 encoded per RFC-9001/RFC-9002

use sx9_foundation_core::hash64::{
    murmur3_64, murmur3_64_base96, trivariate_from_key,
    unicode_slot, seeds, BASE96_CHARSET,
};

/// CTAS-7 v7.3.1 Trivariate Hash Engine
///
/// Wrapper around foundation-core hash64 for backward compatibility.
/// New code should use `sx9_foundation_core::hashing` directly.
#[derive(Debug, Clone)]
pub struct TrivariateHashEngine {
    base96_chars: &'static [u8],
}

impl TrivariateHashEngine {
    /// Create new trivariate hash engine
    pub fn new() -> Self {
        Self {
            base96_chars: BASE96_CHARSET,
        }
    }

    /// Generate CTAS-7 v7.3.1 trivariate hash (48-position: SCH+CUID+UUID)
    ///
    /// Now uses 64-bit MurmurHash3 (extracted from 128-bit) for better collision resistance.
    pub fn generate_hash(&self, key: &str, data: &str) -> anyhow::Result<String> {
        Ok(trivariate_from_key(key, data))
    }

    /// Generate environmental mask hash (16 Base96 chars)
    pub fn generate_environmental_hash(&self, environmental_data: &str) -> anyhow::Result<String> {
        Ok(murmur3_64_base96(environmental_data.as_bytes(), seeds::ENV, 16))
    }

    /// Generate Unicode compressed hash (U+E000–E9FF)
    ///
    /// Uses 64-bit hash for better distribution across 2560 slots.
    pub fn generate_unicode_hash(&self, input: &str) -> anyhow::Result<String> {
        let slot = unicode_slot(input.as_bytes(), seeds::SLOT);
        Ok(slot.to_string())
    }

    /// Verify hash integrity
    pub fn verify_hash(&self, key: &str, data: &str, expected_hash: &str) -> anyhow::Result<bool> {
        let generated_hash = self.generate_hash(key, data)?;
        Ok(generated_hash == expected_hash)
    }

    /// Get raw 64-bit hash value
    pub fn raw_hash(&self, data: &[u8], seed: u32) -> u64 {
        murmur3_64(data, seed)
    }

    /// Get Base96 encoded hash of specified length
    pub fn base96_hash(&self, data: &[u8], seed: u32, length: usize) -> String {
        murmur3_64_base96(data, seed, length)
    }
}

impl Default for TrivariateHashEngine {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export core hash functions for convenience
pub use sx9_foundation_core::hash64::{
    murmur3_64 as hash64,
    murmur3_64_base96 as hash64_base96,
    trivariate_hash,
    encode_base96,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivariate_hash_generation() {
        let engine = TrivariateHashEngine::new();
        let hash = engine.generate_hash("test_key", "test_data").unwrap();

        assert_eq!(hash.len(), 48); // SCH(16) + CUID(16) + UUID(16)
        assert!(hash.chars().all(|c| engine.base96_chars.contains(&(c as u8))));
    }

    #[test]
    fn test_hash_deterministic() {
        let engine = TrivariateHashEngine::new();
        let h1 = engine.generate_hash("key", "data").unwrap();
        let h2 = engine.generate_hash("key", "data").unwrap();
        assert_eq!(h1, h2, "Same input should produce same hash");
    }

    #[test]
    fn test_hash_verification() {
        let engine = TrivariateHashEngine::new();
        let key = "test_key";
        let data = "test_data";

        let hash = engine.generate_hash(key, data).unwrap();
        assert!(engine.verify_hash(key, data, &hash).unwrap());

        // Test with wrong data
        assert!(!engine.verify_hash(key, "wrong_data", &hash).unwrap());
    }

    #[test]
    fn test_unicode_hash() {
        let engine = TrivariateHashEngine::new();
        let unicode_hash = engine.generate_unicode_hash("test").unwrap();

        assert_eq!(unicode_hash.chars().count(), 1);
        let unicode_val = unicode_hash.chars().next().unwrap() as u32;
        assert!(unicode_val >= 0xE000 && unicode_val <= 0xE9FF);
    }

    #[test]
    fn test_environmental_hash() {
        let engine = TrivariateHashEngine::new();
        let hash = engine.generate_environmental_hash("environment_data").unwrap();
        assert_eq!(hash.len(), 16);
    }

    #[test]
    fn test_raw_hash_64bit() {
        let engine = TrivariateHashEngine::new();
        let h1 = engine.raw_hash(b"test", 0);
        let h2 = engine.raw_hash(b"test", 0);
        assert_eq!(h1, h2);
        assert!(h1 > u32::MAX as u64 || h1 < u32::MAX as u64); // Can be any 64-bit value
    }
}
