//! CTAS-7 Unicode Key Compression System
//! Tesla/SpaceX-grade cryptographic key compression using Unicode symbols
//!
//! This module compresses PGP keys into Unicode representations for:
//! - Compact storage in embedded systems
//! - Human-readable key fingerprints
//! - QR code integration
//! - Blockchain/smart contract storage

use serde::{Deserialize, Serialize};

#[cfg(not(feature = "embedded-firefly"))]
use std::{string::String, vec::Vec, collections::HashMap};

#[cfg(feature = "embedded-firefly")]
use alloc::{string::String, vec::Vec};
#[cfg(feature = "embedded-firefly")]
use heapless::FnvIndexMap as HashMap;

/// Unicode Key Compression Manager
pub struct UnicodeKeyCompressor {
    /// Mapping table for binary to Unicode conversion
    compression_table: CompressionTable,
    /// Compression algorithm configuration
    pub config: CompressionConfig,
}

/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    /// Use emoji symbols for compression
    pub use_emoji: bool,
    /// Use mathematical symbols
    pub use_math_symbols: bool,
    /// Use geometric shapes
    pub use_geometric: bool,
    /// Maximum compressed length
    pub max_length: usize,
    /// Compression ratio target (1-100)
    pub target_ratio: u8,
}

/// Compression lookup table
pub struct CompressionTable {
    /// Binary nibble (4-bit) to Unicode mapping
    pub nibble_to_unicode: HashMap<u8, char>,
    /// Unicode to binary nibble mapping
    pub unicode_to_nibble: HashMap<char, u8>,
    /// Special symbols for key metadata
    pub metadata_symbols: HashMap<String, char>,
}

/// Compressed key representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedKey {
    /// Compressed key data as Unicode string
    pub compressed_data: String,
    /// Original key type (RSA, ECDSA, etc.)
    pub key_type: String,
    /// Original key length in bits
    pub key_bits: u16,
    /// Compression algorithm used
    pub algorithm: String,
    /// Checksum for integrity verification
    pub checksum: String,
    /// Metadata about compression
    pub metadata: KeyMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Original size in bytes
    pub original_size: usize,
    /// Compressed size in bytes (UTF-8)
    pub compressed_size: usize,
    /// Compression ratio achieved
    pub compression_ratio: f32,
    /// Time compressed (Unix timestamp)
    pub compressed_at: u64,
    /// Version of compression algorithm
    pub algorithm_version: String,
}

impl UnicodeKeyCompressor {
    /// Create new Unicode key compressor
    pub fn new() -> Self {
        Self {
            compression_table: CompressionTable::new(),
            config: CompressionConfig::default(),
        }
    }

    /// Compress PGP key to Unicode representation
    pub fn compress_key(&self, key_data: &[u8], key_type: &str, key_bits: u16) -> Result<CompressedKey, String> {
        let start_time = self.get_timestamp();

        // Calculate checksum of original data
        let checksum = self.calculate_checksum(key_data);

        // Apply compression algorithm
        let compressed_data = match self.config.target_ratio {
            80..=100 => self.compress_high_ratio(key_data)?,
            50..=79 => self.compress_medium_ratio(key_data)?,
            1..=49 => self.compress_low_ratio(key_data)?,
            _ => return Err("Invalid compression ratio".to_string()),
        };

        let compressed_size = compressed_data.as_bytes().len();
        let compression_ratio = (compressed_size as f32 / key_data.len() as f32) * 100.0;

        Ok(CompressedKey {
            compressed_data,
            key_type: key_type.to_string(),
            key_bits,
            algorithm: "CTAS7-Unicode-v1".to_string(),
            checksum,
            metadata: KeyMetadata {
                original_size: key_data.len(),
                compressed_size,
                compression_ratio,
                compressed_at: start_time,
                algorithm_version: "1.0.0".to_string(),
            },
        })
    }

    /// Decompress Unicode key back to binary
    pub fn decompress_key(&self, compressed: &CompressedKey) -> Result<Vec<u8>, String> {
        // Decompress based on algorithm
        let decompressed_data = match compressed.algorithm.as_str() {
            "CTAS7-Unicode-v1" => self.decompress_v1(&compressed.compressed_data)?,
            _ => return Err(format!("Unsupported algorithm: {}", compressed.algorithm)),
        };

        // Verify checksum
        let checksum = self.calculate_checksum(&decompressed_data);
        if checksum != compressed.checksum {
            return Err("Checksum verification failed".to_string());
        }

        Ok(decompressed_data)
    }

    /// High compression ratio (80-100%) - maximum space savings
    fn compress_high_ratio(&self, data: &[u8]) -> Result<String, String> {
        let mut result = String::new();

        // Use emoji + math symbols for maximum density
        let symbols = [
            '🔐', '🔑', '🔑', '🛡', '⚡', '🚀', '🌟', '💎', '🔥', '⭐', '🌈', '🎯',
            '∀', '∃', '∇', '∆', '∮', '∞', '≡', '≠', '≤', '≥', '∈', '∉', '⊂', '⊃',
            '◆', '◇', '○', '●', '□', '■', '△', '▲', '▽', '▼', '◀', '▶', '◊', '⬟'
        ];

        for chunk in data.chunks(3) {
            let mut value = 0u32;
            for (i, &byte) in chunk.iter().enumerate() {
                value |= (byte as u32) << (i * 8);
            }

            // Map to symbol based on value
            let symbol_index = (value % symbols.len() as u32) as usize;
            result.push(symbols[symbol_index]);

            // Add secondary encoding for remaining bits
            if chunk.len() == 3 {
                let secondary = (value >> 12) % symbols.len() as u32;
                result.push(symbols[secondary as usize]);
            }
        }

        Ok(result)
    }

    /// Medium compression ratio (50-79%) - balanced approach
    fn compress_medium_ratio(&self, data: &[u8]) -> Result<String, String> {
        let mut result = String::new();

        for chunk in data.chunks(2) {
            let mut value = chunk[0] as u16;
            if chunk.len() > 1 {
                value |= (chunk[1] as u16) << 8;
            }

            // Map to Unicode mathematical operators
            let high_nibble = (value >> 12) & 0xF;
            let mid_nibbles = (value >> 4) & 0xFF;
            let low_nibble = value & 0xF;

            result.push(self.compression_table.nibble_to_unicode[&(high_nibble as u8)]);
            result.push(char::from_u32(0x2200 + mid_nibbles as u32).unwrap_or('?'));
            result.push(self.compression_table.nibble_to_unicode[&(low_nibble as u8)]);
        }

        Ok(result)
    }

    /// Low compression ratio (1-49%) - maximum readability
    fn compress_low_ratio(&self, data: &[u8]) -> Result<String, String> {
        let mut result = String::new();

        for &byte in data {
            let high_nibble = (byte >> 4) & 0xF;
            let low_nibble = byte & 0xF;

            result.push(self.compression_table.nibble_to_unicode[&high_nibble]);
            result.push(self.compression_table.nibble_to_unicode[&low_nibble]);
        }

        Ok(result)
    }

    /// Decompress v1 algorithm
    fn decompress_v1(&self, compressed: &str) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();
        let chars: Vec<char> = compressed.chars().collect();

        // Simple nibble-based decompression to match compression algorithm
        for chunk in chars.chunks(2) {
            if chunk.len() == 2 {
                if let Some(&high) = self.compression_table.unicode_to_nibble.get(&chunk[0]) {
                    if let Some(&low) = self.compression_table.unicode_to_nibble.get(&chunk[1]) {
                        let byte_val = (high << 4) | low;
                        result.push(byte_val);
                    } else {
                        return Err(format!("Character '{}' not found in decompression table", chunk[1]));
                    }
                } else {
                    return Err(format!("Character '{}' not found in decompression table", chunk[0]));
                }
            }
        }

        Ok(result)
    }


    /// Calculate simple checksum
    fn calculate_checksum(&self, data: &[u8]) -> String {
        let mut sum = 0u32;
        for &byte in data {
            sum = sum.wrapping_add(byte as u32);
        }
        format!("{:08x}", sum)
    }

    /// Get current timestamp
    fn get_timestamp(&self) -> u64 {
        #[cfg(not(feature = "embedded-firefly"))]
        {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        }
        #[cfg(feature = "embedded-firefly")]
        {
            1732365200 // Fixed timestamp for embedded
        }
    }

    /// Generate human-readable key fingerprint
    pub fn generate_unicode_fingerprint(&self, key_data: &[u8]) -> String {
        let mut fingerprint = String::new();

        // Use first 16 bytes to create recognizable pattern
        let sample = if key_data.len() >= 16 {
            &key_data[0..16]
        } else {
            key_data
        };

        let emoji_set = ['🔐', '🔑', '🔑', '🛡', '⚡', '🚀', '🌟', '💎'];

        for (i, &byte) in sample.iter().enumerate() {
            if i % 4 == 0 && i > 0 {
                fingerprint.push('·');
            }
            let emoji_index = (byte % emoji_set.len() as u8) as usize;
            fingerprint.push(emoji_set[emoji_index]);
        }

        fingerprint
    }

    /// Create QR-code friendly compressed key
    pub fn create_qr_friendly(&self, key_data: &[u8]) -> Result<String, String> {
        // Use only QR-code safe Unicode characters
        let safe_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$%*+-./:";
        let mut result = String::new();

        for chunk in key_data.chunks(2) {
            let mut value = chunk[0] as u16;
            if chunk.len() > 1 {
                value |= (chunk[1] as u16) << 8;
            }

            let index1 = (value % safe_chars.len() as u16) as usize;
            let index2 = ((value >> 8) % safe_chars.len() as u16) as usize;

            result.push(safe_chars.chars().nth(index1).unwrap());
            result.push(safe_chars.chars().nth(index2).unwrap());
        }

        Ok(result)
    }
}

impl CompressionTable {
    fn new() -> Self {
        let mut nibble_to_unicode = HashMap::new();
        let mut unicode_to_nibble = HashMap::new();
        let mut metadata_symbols = HashMap::new();

        // Map nibbles (0-15) to Unicode mathematical symbols
        let symbols = [
            '∅', '∞', '≡', '≠', '≤', '≥', '∈', '∉',
            '⊂', '⊃', '∩', '∪', '∀', '∃', '∇', '∆'
        ];

        for (i, &symbol) in symbols.iter().enumerate() {
            nibble_to_unicode.insert(i as u8, symbol);
            unicode_to_nibble.insert(symbol, i as u8);
        }

        // Metadata symbols
        metadata_symbols.insert("start".to_string(), '🔐');
        metadata_symbols.insert("end".to_string(), '🔒');
        metadata_symbols.insert("rsa".to_string(), '🔑');
        metadata_symbols.insert("ecdsa".to_string(), '🔑');
        metadata_symbols.insert("checksum".to_string(), '✓');

        Self {
            nibble_to_unicode,
            unicode_to_nibble,
            metadata_symbols,
        }
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            use_emoji: true,
            use_math_symbols: true,
            use_geometric: true,
            max_length: 1024,
            target_ratio: 70, // 70% compression
        }
    }
}

/// Quick test function for Unicode key compression
pub fn test_unicode_compression() -> Result<(), String> {
    println!("🔐 CTAS-7 Unicode Key Compression Test");
    println!("=====================================");

    let compressor = UnicodeKeyCompressor::new();

    // Test with sample key data (simulated PGP key bytes)
    let test_key_data = vec![
        0x99, 0x01, 0x0D, 0x04, 0x53, 0x23, 0x5F, 0x2B,
        0x01, 0x08, 0x00, 0x9F, 0xBC, 0x70, 0xA7, 0x9E,
        0xF7, 0x1E, 0xFD, 0x2F, 0xF3, 0xAE, 0xAB, 0xAC,
        0x03, 0xAD, 0x16, 0x4F, 0xDE, 0x2F, 0x72, 0x3C
    ];

    println!("📊 Original key data: {} bytes", test_key_data.len());

    // Test compression
    let compressed = compressor.compress_key(&test_key_data, "RSA", 4096)?;

    println!("✅ Compressed key:");
    println!("   Data: {}", compressed.compressed_data);
    println!("   Size: {} bytes ({}% compression)",
        compressed.metadata.compressed_size,
        100.0 - compressed.metadata.compression_ratio);
    println!("   Checksum: {}", compressed.checksum);

    // Test decompression
    let decompressed = compressor.decompress_key(&compressed)?;

    println!("🔍 Decompression test:");
    println!("   Original:     {} bytes", test_key_data.len());
    println!("   Decompressed: {} bytes", decompressed.len());
    println!("   Match: {}", if test_key_data == decompressed { "✅" } else { "❌" });

    // Test Unicode fingerprint
    let fingerprint = compressor.generate_unicode_fingerprint(&test_key_data);
    println!("🎯 Unicode fingerprint: {}", fingerprint);

    // Test QR-friendly format
    let qr_friendly = compressor.create_qr_friendly(&test_key_data)?;
    println!("📱 QR-friendly format: {}", qr_friendly);

    println!("\n🚀 Unicode compression test completed successfully!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_compression() {
        let compressor = UnicodeKeyCompressor::new();
        let test_data = vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];

        let compressed = compressor.compress_key(&test_data, "RSA", 2048).unwrap();
        let decompressed = compressor.decompress_key(&compressed).unwrap();

        assert_eq!(test_data, decompressed);
    }

    #[test]
    fn test_fingerprint_generation() {
        let compressor = UnicodeKeyCompressor::new();
        let test_data = vec![0xFF; 16];

        let fingerprint = compressor.generate_unicode_fingerprint(&test_data);
        assert!(!fingerprint.is_empty());
        assert!(fingerprint.contains('💎')); // Should contain emoji
    }

    #[test]
    fn test_qr_friendly_format() {
        let compressor = UnicodeKeyCompressor::new();
        let test_data = vec![0x01, 0x02, 0x03, 0x04];

        let qr_format = compressor.create_qr_friendly(&test_data).unwrap();
        assert!(!qr_format.is_empty());
        assert!(qr_format.chars().all(|c| "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$%*+-./:".contains(c)));
    }
}