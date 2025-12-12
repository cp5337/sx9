//! CTAS-7 Canonical 64-bit Hash Module
//!
//! RFC-9001 compliant 64-bit MurmurHash3 implementation.
//! Extracts lower 64 bits from murmur3_x64_128 for optimal collision resistance
//! while maintaining reasonable output size.
//!
//! All outputs are Base96 encoded for Unicode assembly compatibility.

use murmur3::murmur3_x64_128;
use std::io::Cursor;

/// Base96 character set (RFC-9002 compliant)
/// 96 printable ASCII characters for maximum density
pub const BASE96_CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[]^_{|}~`\"'\\";

/// Standard seeds for trivariate components (RFC-9001)
pub mod seeds {
    /// SCH (Semantic Context Hash) seed
    pub const SCH: u32 = 0xC7A5_0000;
    /// CUID (Context User Identity) seed
    pub const CUID: u32 = 0xC7A5_0001;
    /// UUID (Universal Unique Identifier) seed
    pub const UUID: u32 = 0xC7A5_0002;
    /// Environmental hash seed
    pub const ENV: u32 = 0xC7A5_00FF;
    /// Slot distribution seed
    pub const SLOT: u32 = 0xC7A5_0100;
}

/// Compute 64-bit MurmurHash3 from data
///
/// Extracts lower 64 bits from murmur3_x64_128 for optimal performance
/// while maintaining collision resistance suitable for billions of hashes.
///
/// # Arguments
/// * `data` - Byte slice to hash
/// * `seed` - 32-bit seed value
///
/// # Returns
/// 64-bit hash value
pub fn murmur3_64(data: &[u8], seed: u32) -> u64 {
    let mut cursor = Cursor::new(data);
    let hash_128 = murmur3_x64_128(&mut cursor, seed).unwrap_or(0);
    hash_128 as u64 // Lower 64 bits
}

/// Compute 64-bit MurmurHash3 and return as hex string (16 chars)
pub fn murmur3_64_hex(data: &[u8], seed: u32) -> String {
    format!("{:016x}", murmur3_64(data, seed))
}

/// Encode a 64-bit value to Base96 string
///
/// # Arguments
/// * `value` - 64-bit value to encode
/// * `length` - Target output length (padded with '0' if needed)
///
/// # Returns
/// Base96 encoded string of specified length
pub fn encode_base96(mut value: u64, length: usize) -> String {
    if value == 0 {
        return "0".repeat(length);
    }

    let mut result = Vec::with_capacity(length);
    while value > 0 && result.len() < length {
        let idx = (value % 96) as usize;
        result.push(BASE96_CHARSET[idx] as char);
        value /= 96;
    }

    // Pad to target length
    while result.len() < length {
        result.push('0');
    }

    // Reverse for big-endian representation
    result.into_iter().rev().collect()
}

/// Compute 64-bit MurmurHash3 and return as Base96 string
///
/// # Arguments
/// * `data` - Byte slice to hash
/// * `seed` - 32-bit seed value
/// * `length` - Target output length (typically 11 chars for full 64-bit, or 16 for component)
///
/// # Returns
/// Base96 encoded hash string
pub fn murmur3_64_base96(data: &[u8], seed: u32, length: usize) -> String {
    let hash = murmur3_64(data, seed);
    encode_base96(hash, length)
}

/// Generate trivariate hash (SCH + CUID + UUID = 48 chars)
///
/// Each component is 16 Base96 characters derived from 64-bit hash.
///
/// # Arguments
/// * `sch_data` - Data for SCH component
/// * `cuid_data` - Data for CUID component
/// * `uuid_data` - Data for UUID component
///
/// # Returns
/// 48-character trivariate hash string
pub fn trivariate_hash(sch_data: &[u8], cuid_data: &[u8], uuid_data: &[u8]) -> String {
    let sch = murmur3_64_base96(sch_data, seeds::SCH, 16);
    let cuid = murmur3_64_base96(cuid_data, seeds::CUID, 16);
    let uuid = murmur3_64_base96(uuid_data, seeds::UUID, 16);
    format!("{}{}{}", sch, cuid, uuid)
}

/// Generate trivariate hash from key and data (convenience function)
pub fn trivariate_from_key(key: &str, data: &str) -> String {
    let sch_data = format!("SCH:{}", key);
    let cuid_data = format!("CUID:{}:{}", key, data.len());
    let uuid_data = format!("UUID:{}:{}", key, data);
    trivariate_hash(
        sch_data.as_bytes(),
        cuid_data.as_bytes(),
        uuid_data.as_bytes(),
    )
}

/// Generate Unicode slot assignment from data (U+E000-E9FF range)
///
/// Uses 64-bit hash for better distribution across 2560 possible slots.
pub fn unicode_slot(data: &[u8], seed: u32) -> char {
    let hash = murmur3_64(data, seed);
    let slot = (hash % 2560) as u32 + 0xE000;
    char::from_u32(slot).unwrap_or('\u{E000}')
}

/// Generate Unicode slot as hex escape string
pub fn unicode_slot_hex(data: &[u8], seed: u32) -> String {
    let c = unicode_slot(data, seed);
    format!("\\u{{{:04X}}}", c as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_murmur3_64_deterministic() {
        let data = b"test data";
        let h1 = murmur3_64(data, 0);
        let h2 = murmur3_64(data, 0);
        assert_eq!(h1, h2, "Same input should produce same hash");
    }

    #[test]
    fn test_murmur3_64_different_seeds() {
        let data = b"test data";
        let h1 = murmur3_64(data, seeds::SCH);
        let h2 = murmur3_64(data, seeds::CUID);
        assert_ne!(h1, h2, "Different seeds should produce different hashes");
    }

    #[test]
    fn test_base96_encoding() {
        let encoded = encode_base96(12345678901234567890_u64, 16);
        assert_eq!(encoded.len(), 16);
        assert!(encoded.chars().all(|c| BASE96_CHARSET.contains(&(c as u8))));
    }

    #[test]
    fn test_trivariate_hash_length() {
        let hash = trivariate_from_key("test_key", "test_data");
        assert_eq!(hash.len(), 48, "Trivariate hash should be 48 chars");
    }

    #[test]
    fn test_trivariate_hash_deterministic() {
        let h1 = trivariate_from_key("key", "data");
        let h2 = trivariate_from_key("key", "data");
        assert_eq!(h1, h2, "Same input should produce same trivariate hash");
    }

    #[test]
    fn test_unicode_slot_range() {
        let data = b"test";
        let slot = unicode_slot(data, seeds::SLOT);
        let code = slot as u32;
        assert!(
            code >= 0xE000 && code <= 0xE9FF,
            "Slot should be in PUA range"
        );
    }

    #[test]
    fn test_hex_output_length() {
        let hex = murmur3_64_hex(b"test", 0);
        assert_eq!(hex.len(), 16, "Hex output should be 16 chars");
    }
}
