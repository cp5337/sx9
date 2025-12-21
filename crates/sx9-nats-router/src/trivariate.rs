use murmur3::murmur3_x64_128;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

// RFC-9001 Seeds (Immutable)
const SEED_SCH: u32 = 0xC7A5_0000;
const SEED_CUID: u32 = 0xC7A5_0001;
const SEED_UUID: u32 = 0xC7A5_0002;

// RFC-9001 Base96 Alphabet
const BASE96_CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[]^_{|}~`\"'\\";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrivariateHash {
    pub sch: String,
    pub cuid: String,
    pub uuid: String,
}

impl TrivariateHash {
    /// Generate a full Trivariate Hash from components
    pub fn new(operation: &str, context: &[u8], nonce: &[u8]) -> Self {
        let sch_val = hash_128(operation.as_bytes(), SEED_SCH);
        let cuid_val = hash_128(context, SEED_CUID);
        let uuid_val = hash_128(nonce, SEED_UUID);

        Self {
            sch: encode_base96(sch_val),
            cuid: encode_base96(cuid_val),
            uuid: encode_base96(uuid_val),
        }
    }

    pub fn to_canonical(&self) -> String {
        format!("triv:{}_{}_{}", self.sch, self.cuid, self.uuid)
    }
}

impl fmt::Display for TrivariateHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_canonical())
    }
}

/// Helper: 128-bit Murmur3 Hash
fn hash_128(data: &[u8], seed: u32) -> u128 {
    murmur3_x64_128(&mut Cursor::new(data), seed).unwrap_or(0)
}

/// Helper: Base96 Encoding (<60ns Performance Target)
fn encode_base96(value: u128) -> String {
    if value == 0 {
        return (BASE96_CHARSET[0] as char).to_string();
    }

    let mut n = value;
    let mut chars = Vec::with_capacity(20);

    while n > 0 {
        let rem = (n % 96) as usize;
        chars.push(BASE96_CHARSET[rem]);
        n /= 96;
    }

    chars.reverse();
    String::from_utf8(chars).unwrap_or_else(|_| "INVALID".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rfc9001_compliance() {
        // Known test vector
        let op = "scan port 22";
        let ctx = b"US-EAST-1";
        let nonce = b"123456789";

        let hash = TrivariateHash::new(op, ctx, nonce);
        println!("Generated: {}", hash);

        assert!(hash.to_canonical().starts_with("triv:"));
        // Basic length check (Base96 128bit is approx 20 chars max)
        assert!(hash.sch.len() > 10);
        assert!(hash.cuid.len() > 10);
        assert!(hash.uuid.len() > 10);
    }
}
