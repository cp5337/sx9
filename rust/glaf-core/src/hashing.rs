use murmur3::murmur3_x64_128;
use std::io::Cursor;
use uuid::Uuid;

/// Standard Seeds from RFC-9001 Section 4.2
pub const SEED_SCH: u32 = 0xC7A5_0000;
pub const SEED_CUID: u32 = 0xC7A5_0001;
pub const SEED_UUID: u32 = 0xC7A5_0002;
const BASE96_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[]^_{|}~`\"'\\";

/// Murmur3-64 implementation from RFC-9001 Section 4.1
pub fn murmur3_64(data: &[u8], seed: u32) -> u64 {
    let hash_128 = murmur3_x64_128(&mut Cursor::new(data), seed).unwrap_or(0);
    hash_128 as u64
}

/// Base96 Encoding from RFC-9001 Section 4.3
pub fn base96_encode(mut value: u128) -> String {
    if value == 0 {
        return "0".to_string();
    }
    let mut result = Vec::new();
    while value > 0 {
        let remainder = (value % 96) as usize;
        result.push(BASE96_CHARS[remainder]);
        value /= 96;
    }
    result.reverse();
    String::from_utf8(result).unwrap()
}

/// Trivariate Hash Structure
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrivariateHash {
    pub sch: String,
    pub cuid: String,
    pub uuid: Uuid,
}

impl TrivariateHash {
    pub fn new(context: &str, user_ctx: &str) -> Self {
        let sch_raw = murmur3_64(context.as_bytes(), SEED_SCH) as u128;
        let cuid_raw = murmur3_64(user_ctx.as_bytes(), SEED_CUID) as u128;
        
        Self {
            sch: base96_encode(sch_raw),
            cuid: base96_encode(cuid_raw),
            uuid: Uuid::now_v7(),
        }
    }

    pub fn to_canonical_string(&self) -> String {
        format!("triv:{}_{}_{}", self.sch, self.cuid, self.uuid)
    }
}
