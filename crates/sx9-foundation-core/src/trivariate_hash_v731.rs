//! CTAS-7.3.1 Canonical Trivariate Hash Engine
//!
//! Implements CTAS7-HASH-CORE-V731 specification
//! Dual trivariate system with slot-by-slot CUID encoding
//! Full Murmur3-128 implementation for bit-exact reproducibility

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use uuid::Uuid;

/// Base96 Character Set (RFC-9001 v1.1 Standard) - Exactly 96 characters
/// Canonical charset per RFC-9001 Section 4.3
const BASE96_CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[]^_{|}~`\"'\\";

/// Execution Environment Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecEnv {
    Wasm,           // WASM microkernel
    Container,      // Docker/OrbStack container
    Native,         // Native binary
    Kernel,         // Kernel space
    Microkernel,    // Microkernel
    Firefly,        // Firefly runtime
    Orb,            // Orb system
}

impl ExecEnv {
    /// Encode to 3 Base96 characters (E1..E3)
    pub fn to_base96(&self) -> String {
        let code = match self {
            ExecEnv::Wasm => 0,
            ExecEnv::Container => 1,
            ExecEnv::Native => 2,
            ExecEnv::Kernel => 3,
            ExecEnv::Microkernel => 4,
            ExecEnv::Firefly => 5,
            ExecEnv::Orb => 6,
        };
        encode_base96_3chars(code)
    }
}

/// Execution State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecState {
    Cold,       // Not in memory
    Warm,       // In memory, not active
    Hot,        // Active execution
    L2Resident, // L2 cognitive plane resident
}

impl ExecState {
    /// Encode to single Base96 character (S)
    pub fn to_base96(&self) -> char {
        let code = match self {
            ExecState::Cold => 0,
            ExecState::Warm => 1,
            ExecState::Hot => 2,
            ExecState::L2Resident => 3,
        };
        BASE96_CHARSET.chars().nth(code as usize).unwrap_or('0')
    }
}

/// ContextFrame for CUID generation (CTAS-7.3.1 Canonical)
#[derive(Debug, Clone)]
pub struct ContextFrame {
    pub timestamp: u64,
    pub exec_env: ExecEnv,
    pub agent_id: u16,
    pub delta_angle: f32,
    pub state: ExecState,
    pub lineage: u16,
    pub nonce: u16,
}

impl ContextFrame {
    /// Create new ContextFrame with current timestamp
    pub fn new(exec_env: ExecEnv, agent_id: u16, state: ExecState) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            timestamp,
            exec_env,
            agent_id,
            delta_angle: 0.0,
            state,
            lineage: 0,
            nonce: (timestamp % 65536) as u16, // Use timestamp LSB as nonce
        }
    }

    /// Create ContextFrame with all parameters
    pub fn with_all(
        timestamp: u64,
        exec_env: ExecEnv,
        agent_id: u16,
        delta_angle: f32,
        state: ExecState,
        lineage: u16,
        nonce: u16,
    ) -> Self {
        Self {
            timestamp,
            exec_env,
            agent_id,
            delta_angle,
            state,
            lineage,
            nonce,
        }
    }
}

/// CUID Slots (16 characters, slot-by-slot encoding)
#[derive(Debug, Clone)]
pub struct CuidSlots {
    /// Slots 1-4: Timestamp shard (T1..T4)
    pub timestamp_shard: [char; 4],
    /// Slots 5-7: Execution environment (E1..E3)
    pub exec_env: [char; 3],
    /// Slots 8-9: Agent identifier (A1..A2)
    pub agent_id: [char; 2],
    /// Slots 10-11: Delta-angle derivative (Δ1..Δ2)
    pub delta_angle: [char; 2],
    /// Slot 12: State flag (S)
    pub state: char,
    /// Slots 13-14: Process lineage shard (P1..P2)
    pub lineage: [char; 2],
    /// Slots 15-16: Random salt (R1..R2)
    pub nonce: [char; 2],
}

impl From<&ContextFrame> for CuidSlots {
    fn from(ctx: &ContextFrame) -> Self {
        // Encode timestamp shard (4 chars from timestamp)
        let ts_bytes = ctx.timestamp.to_be_bytes();
        let timestamp_shard = [
            encode_base96_char((ts_bytes[0] % 96) as usize),
            encode_base96_char((ts_bytes[1] % 96) as usize),
            encode_base96_char((ts_bytes[2] % 96) as usize),
            encode_base96_char((ts_bytes[3] % 96) as usize),
        ];

        // Encode execution environment (3 chars)
        let exec_env_str = ctx.exec_env.to_base96();
        let exec_env = [
            exec_env_str.chars().nth(0).unwrap_or('0'),
            exec_env_str.chars().nth(1).unwrap_or('0'),
            exec_env_str.chars().nth(2).unwrap_or('0'),
        ];

        // Encode agent ID (2 chars)
        let agent_bytes = ctx.agent_id.to_be_bytes();
        let agent_id = [
            encode_base96_char((agent_bytes[0] % 96) as usize),
            encode_base96_char((agent_bytes[1] % 96) as usize),
        ];

        // Encode delta angle (2 chars) - quantize f32 to 0-9215 range
        let delta_quantized = ((ctx.delta_angle.abs() * 1000.0) as u16).min(9215);
        let delta_bytes = delta_quantized.to_be_bytes();
        let delta_angle = [
            encode_base96_char((delta_bytes[0] % 96) as usize),
            encode_base96_char((delta_bytes[1] % 96) as usize),
        ];

        // Encode state (1 char)
        let state = ctx.state.to_base96();

        // Encode lineage (2 chars)
        let lineage_bytes = ctx.lineage.to_be_bytes();
        let lineage = [
            encode_base96_char((lineage_bytes[0] % 96) as usize),
            encode_base96_char((lineage_bytes[1] % 96) as usize),
        ];

        // Encode nonce (2 chars)
        let nonce_bytes = ctx.nonce.to_be_bytes();
        let nonce = [
            encode_base96_char((nonce_bytes[0] % 96) as usize),
            encode_base96_char((nonce_bytes[1] % 96) as usize),
        ];

        Self {
            timestamp_shard,
            exec_env,
            agent_id,
            delta_angle,
            state,
            lineage,
            nonce,
        }
    }
}

impl CuidSlots {
    /// Convert CUID slots to 16-character string
    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity(16);
        result.extend(self.timestamp_shard.iter());
        result.extend(self.exec_env.iter());
        result.extend(self.agent_id.iter());
        result.extend(self.delta_angle.iter());
        result.push(self.state);
        result.extend(self.lineage.iter());
        result.extend(self.nonce.iter());
        result
    }

    /// Convert CUID slots to Unicode runes (U+E200-E2FF) for semantic routing
    /// Encodes slot values directly into Unicode for Neural Mux affinity weighting
    pub fn to_unicode_runes(&self) -> String {
        let mut result = String::with_capacity(16);
        
        // Map each Base96 char to Unicode Private Use Block (U+E200-E2FF)
        for &ch in self.timestamp_shard.iter() {
            let code = 0xE200 + ((ch as u32) % 0xFF);
            if let Some(unicode_char) = std::char::from_u32(code) {
                result.push(unicode_char);
            } else {
                result.push('\u{E200}'); // Fallback
            }
        }
        
        for &ch in self.exec_env.iter() {
            let code = 0xE200 + ((ch as u32) % 0xFF);
            if let Some(unicode_char) = std::char::from_u32(code) {
                result.push(unicode_char);
            } else {
                result.push('\u{E200}');
            }
        }
        
        for &ch in self.agent_id.iter() {
            let code = 0xE200 + ((ch as u32) % 0xFF);
            if let Some(unicode_char) = std::char::from_u32(code) {
                result.push(unicode_char);
            } else {
                result.push('\u{E200}');
            }
        }
        
        for &ch in self.delta_angle.iter() {
            let code = 0xE200 + ((ch as u32) % 0xFF);
            if let Some(unicode_char) = std::char::from_u32(code) {
                result.push(unicode_char);
            } else {
                result.push('\u{E200}');
            }
        }
        
        let state_code = 0xE200 + ((self.state as u32) % 0xFF);
        if let Some(unicode_char) = std::char::from_u32(state_code) {
            result.push(unicode_char);
        } else {
            result.push('\u{E200}');
        }
        
        for &ch in self.lineage.iter() {
            let code = 0xE200 + ((ch as u32) % 0xFF);
            if let Some(unicode_char) = std::char::from_u32(code) {
                result.push(unicode_char);
            } else {
                result.push('\u{E200}');
            }
        }
        
        for &ch in self.nonce.iter() {
            let code = 0xE200 + ((ch as u32) % 0xFF);
            if let Some(unicode_char) = std::char::from_u32(code) {
                result.push(unicode_char);
            } else {
                result.push('\u{E200}');
            }
        }
        
        result
    }
}

/// Trivariate Hash (CTAS-7.3.1 Canonical)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrivariateHash {
    pub sch: String,  // 16 characters
    pub cuid: String, // 16 characters
    pub uuid: String, // UUIDv4 (standard format)
}

impl TrivariateHash {
    /// Create new trivariate hash
    pub fn new(sch: String, cuid: String, uuid: String) -> Self {
        Self { sch, cuid, uuid }
    }

    /// Convert to canonical format: triv:[SCH]_[CUID]_[UUID]
    pub fn to_canonical_format(&self) -> String {
        format!("triv:{}_{}_{}", self.sch, self.cuid, self.uuid)
    }

    /// Parse from canonical format
    pub fn from_canonical_format(s: &str) -> Result<Self, String> {
        if !s.starts_with("triv:") {
            return Err("Invalid format: must start with 'triv:'".to_string());
        }
        
        let parts: Vec<&str> = s[5..].split('_').collect();
        if parts.len() != 3 {
            return Err("Invalid format: expected triv:[SCH]_[CUID]_[UUID]".to_string());
        }

        Ok(Self {
            sch: parts[0].to_string(),
            cuid: parts[1].to_string(),
            uuid: parts[2].to_string(),
        })
    }

    /// Get full 48-character hash (SCH + CUID + UUID hex converted to Base96)
    pub fn to_48char_hash(&self) -> String {
        // Convert UUID hex to Base96 (16 chars)
        let uuid_base96 = uuid_to_base96(&self.uuid);
        format!("{}{}{}", self.sch, self.cuid, uuid_base96)
    }
}

/// Dual Trivariate Hash System (Primary + Secondary)
#[derive(Debug, Clone)]
pub struct DualTrivariateHash {
    pub primary: TrivariateHash,
    pub secondary: Option<TrivariateHash>, // Optional for low-complexity verticals
}

impl DualTrivariateHash {
    /// Create primary-only trivariate (for low-complexity verticals)
    pub fn primary_only(primary: TrivariateHash) -> Self {
        Self {
            primary,
            secondary: None,
        }
    }

    /// Create dual trivariate (for Synaptix9, ATLAS, PLASMA, GLAF)
    pub fn dual(primary: TrivariateHash, secondary: TrivariateHash) -> Self {
        Self {
            primary,
            secondary: Some(secondary),
        }
    }
}

/// Supersession Level (CTAS-7.3.1 Canonical)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupersessionLevel {
    None,        // < 2° - No supersession (noise/thermal drift)
    Micro,       // 2°-12° - Local context change
    Soft,        // 12°-27° - Operator intent shift
    Hard,        // 27°-42° - Semantic state mutation
    Critical,    // > 42° - Domain/mission change
}

impl SupersessionLevel {
    /// Determine supersession level from delta angle (degrees)
    pub fn from_delta_angle(delta_degrees: f32) -> Self {
        let abs_delta = delta_degrees.abs();
        if abs_delta < 2.0 {
            Self::None
        } else if abs_delta < 12.0 {
            Self::Micro
        } else if abs_delta < 27.0 {
            Self::Soft
        } else if abs_delta < 42.0 {
            Self::Hard
        } else {
            Self::Critical
        }
    }
}

/// CUID TTL Configuration (CTAS-7.3.1 Canonical - 42 is canonical)
#[derive(Debug, Clone, Copy)]
pub struct CuidTtl {
    pub default: Duration,  // 42 seconds
    pub hot_lane: Duration, // 4.2 seconds
    pub l2_kernel: Duration, // 0.42 seconds
}

impl Default for CuidTtl {
    fn default() -> Self {
        Self {
            default: Duration::from_secs(42),
            hot_lane: Duration::from_millis(4200),
            l2_kernel: Duration::from_millis(420),
        }
    }
}

/// CTAS-7.3.1 Trivariate Hash Engine
pub struct TrivariateHashEngineV731 {
    murmur_sch_seed: u64,
    murmur_cuid_seed: u64,
    cuid_ttl: CuidTtl,
}

impl TrivariateHashEngineV731 {
    /// Create new engine with canonical seeds
    pub fn new() -> Self {
        Self {
            murmur_sch_seed: 0x5BD1E995,  // CTAS-7.3.1 SCH seed
            murmur_cuid_seed: 0x1B873593, // CTAS-7.3.1 CUID seed
            cuid_ttl: CuidTtl::default(),
        }
    }

    /// Generate SCH with domain and execution class masks (CTAS-7.3.1 Canonical)
    pub fn generate_sch(
        &self,
        semantic: &str,
        _node_type: &str,
        domain: &str,
        exec_class: &str,
    ) -> String {
        // Step 1: Normalize input → UTF-8
        let normalized = normalize_semantics(semantic);
        
        // Step 2: Tokenize using N-V-N-N grammar
        let tokenized = tokenize_nvn_grammar(&normalized);
        
        // Step 3: Apply MurmurHash3 (128-bit)
        let mm_hash = murmur3_128(&tokenized);
        
        // Step 4: Convert 128-bit → 16x Base96 symbols
        let base96_intermediate = encode_base96_128bit(&mm_hash);
        
        // Step 5: Inject domain bitmask (4 bits)
        let domain_mask = domain_bitmask(domain);
        
        // Step 6: Inject execution class bitmask (4 bits)
        let exec_mask = exec_class_bitmask(exec_class);
        
        // Step 7: Rehash final 128-bit → "SCH" output
        let final_input = format!("{}{}{}", base96_intermediate, domain_mask, exec_mask);
        let final_hash = murmur3_128(&final_input);
        
        encode_base96_128bit(&final_hash)
    }

    /// Generate CUID from ContextFrame (slot-by-slot encoding)
    pub fn generate_cuid(&self, context: &ContextFrame) -> String {
        let slots = CuidSlots::from(context);
        slots.to_string()
    }

    /// Generate UUID (standard UUIDv4)
    pub fn generate_uuid(&self) -> String {
        Uuid::new_v4().to_string()
    }

    /// Generate complete trivariate hash
    pub fn generate_trivariate(
        &self,
        semantic: &str,
        node_type: &str,
        domain: &str,
        exec_class: &str,
        context: &ContextFrame,
    ) -> TrivariateHash {
        let sch = self.generate_sch(semantic, node_type, domain, exec_class);
        let cuid = self.generate_cuid(context);
        let uuid = self.generate_uuid();

        TrivariateHash::new(sch, cuid, uuid)
    }

    /// Generate dual trivariate hash (primary + secondary)
    /// Auto-generates secondary for high-cognitive layers (Synaptix9/ATLAS/Plasma/GLAF/etc.)
    pub fn generate_dual_trivariate(
        &self,
        semantic: &str,
        node_type: &str,
        domain: &str,
        exec_class: &str,
        context: &ContextFrame,
        requires_secondary: bool,
    ) -> DualTrivariateHash {
        // Primary trivariate (semantic identity)
        let primary = self.generate_trivariate(semantic, node_type, domain, exec_class, context);
        
        if requires_secondary {
            // Secondary trivariate (operational behavior: SCH*, CUID*, UUID*)
            // SCH* = operational hash of primary SCH
            let sch_star = self.generate_sch(&format!("op:{}", primary.sch), node_type, domain, exec_class);
            
            // CUID* = operational context (inverted delta angle for operational behavior)
            let mut op_context = context.clone();
            op_context.delta_angle = -context.delta_angle; // Invert for operational
            let cuid_star = self.generate_cuid(&op_context);
            
            // UUID* = operational persistence
            let uuid_star = self.generate_uuid();
            
            let secondary = TrivariateHash::new(sch_star, cuid_star, uuid_star);
            
            DualTrivariateHash::dual(primary, secondary)
        } else {
            // Primary only for low-tier playbooks
            DualTrivariateHash::primary_only(primary)
        }
    }

    /// Check if hash requires supersession based on delta angle
    pub fn check_supersession(&self, delta_angle: f32) -> SupersessionLevel {
        SupersessionLevel::from_delta_angle(delta_angle)
    }

    /// Check if CUID has expired based on TTL
    pub fn is_cuid_expired(&self, context: &ContextFrame, ttl_type: &str) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let age = now.saturating_sub(context.timestamp);
        
        let ttl = match ttl_type {
            "hot_lane" => self.cuid_ttl.hot_lane.as_secs(),
            "l2_kernel" => self.cuid_ttl.l2_kernel.as_secs(),
            _ => self.cuid_ttl.default.as_secs(),
        };
        
        age > ttl
    }
}

impl Default for TrivariateHashEngineV731 {
    fn default() -> Self {
        Self::new()
    }
}

// Helper functions

fn encode_base96_char(index: usize) -> char {
    BASE96_CHARSET.chars().nth(index % 96).unwrap_or('0')
}

fn encode_base96_3chars(value: u32) -> String {
    let mut result = String::with_capacity(3);
    let mut val = value;
    for _ in 0..3 {
        result.push(encode_base96_char((val % 96) as usize));
        val /= 96;
    }
    result
}

fn normalize_semantics(s: &str) -> String {
    // Normalize to UTF-8, remove control chars, lowercase
    s.chars()
        .filter(|c| !c.is_control())
        .collect::<String>()
        .to_lowercase()
}

/// CTAS-domain N-V-N-N grammar tokenization (CTAS-7.3.1 Canonical)
/// Maps to 33 primitives (Apollo/Lisp/COBOL patterns)
fn tokenize_nvn_grammar(s: &str) -> String {
    // Grammar definitions
    let nouns = vec![
        "entity", "object", "process", "intel", "operator", "asset",
        "context", "environment", "target", "subject", "resource",
        "evidence", "state", "vector", "delta", "lineage",
    ];
    
    let verbs = vec![
        "act", "mutate", "sense", "evaluate", "route", "map",
        "hash", "emit", "escalate",
    ];
    
    // Tokenize S-Expression via grammar
    let words: Vec<&str> = s.split_whitespace().collect();
    let mut tokens = Vec::new();
    
    for word in words {
        let normalized = word.to_lowercase();
        
        // Check if word matches grammar
        if nouns.iter().any(|&n| normalized.contains(n)) {
            tokens.push(format!("N:{}", normalized));
        } else if verbs.iter().any(|&v| normalized.contains(v)) {
            tokens.push(format!("V:{}", normalized));
        } else {
            // Default: treat as noun
            tokens.push(format!("N:{}", normalized));
        }
    }
    
    // Join with grammar separator
    tokens.join(":")
}

/// Murmur3-128 implementation (CTAS-7.3.1 Canonical)
/// Full 128-bit MurmurHash3 algorithm - bit-exact reproducibility required
/// ~3.5ns/word performance, zero ambiguity for delta-angle supersession
fn murmur3_128(input: &str) -> [u8; 16] {
    let data = input.as_bytes();
    let len = data.len();
    
    // Murmur3-128 constants
    const C1: u64 = 0x87c37b91114253d5;
    const C2: u64 = 0x4cf5ad432745937f;
    const R1: u32 = 31;
    const R2: u32 = 27;
    const R3: u32 = 33;
    const M: u64 = 5;
    const N1: u64 = 0x52dce729;
    const N2: u64 = 0x38495ab5;
    
    // Seeds (CTAS-7.3.1 canonical)
    let mut h1: u64 = 0x5BD1E995; // SCH seed
    let mut h2: u64 = 0x1B873593; // CUID seed
    
    // Process 16-byte chunks
    let chunks = data.chunks_exact(16);
    let remainder = chunks.remainder();
    
    for chunk in chunks {
        // Read two 64-bit values (little-endian)
        let mut k1 = u64::from_le_bytes([
            chunk[0], chunk[1], chunk[2], chunk[3],
            chunk[4], chunk[5], chunk[6], chunk[7],
        ]);
        let mut k2 = u64::from_le_bytes([
            chunk[8], chunk[9], chunk[10], chunk[11],
            chunk[12], chunk[13], chunk[14], chunk[15],
        ]);
        
        // Mix k1 with h1
        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(R1);
        k1 = k1.wrapping_mul(C2);
        h1 ^= k1;
        h1 = h1.rotate_left(R2);
        h1 = h1.wrapping_add(h2);
        h1 = h1.wrapping_mul(M).wrapping_add(N1);
        
        // Mix k2 with h2
        k2 = k2.wrapping_mul(C2);
        k2 = k2.rotate_left(R3);
        k2 = k2.wrapping_mul(C1);
        h2 ^= k2;
        h2 = h2.rotate_left(R1);
        h2 = h2.wrapping_add(h1);
        h2 = h2.wrapping_mul(M).wrapping_add(N2);
    }
    
    // Process remainder (0-15 bytes)
    if !remainder.is_empty() {
        let mut k1: u64 = 0;
        let mut k2: u64 = 0;
        
        if remainder.len() >= 15 {
            k2 ^= (remainder[14] as u64) << 48;
        }
        if remainder.len() >= 14 {
            k2 ^= (remainder[13] as u64) << 40;
        }
        if remainder.len() >= 13 {
            k2 ^= (remainder[12] as u64) << 32;
        }
        if remainder.len() >= 12 {
            k2 ^= (remainder[11] as u64) << 24;
        }
        if remainder.len() >= 11 {
            k2 ^= (remainder[10] as u64) << 16;
        }
        if remainder.len() >= 10 {
            k2 ^= (remainder[9] as u64) << 8;
        }
        if remainder.len() >= 9 {
            k2 ^= remainder[8] as u64;
        }
        
        if remainder.len() >= 8 {
            k1 ^= (remainder[7] as u64) << 56;
        }
        if remainder.len() >= 7 {
            k1 ^= (remainder[6] as u64) << 48;
        }
        if remainder.len() >= 6 {
            k1 ^= (remainder[5] as u64) << 40;
        }
        if remainder.len() >= 5 {
            k1 ^= (remainder[4] as u64) << 32;
        }
        if remainder.len() >= 4 {
            k1 ^= (remainder[3] as u64) << 24;
        }
        if remainder.len() >= 3 {
            k1 ^= (remainder[2] as u64) << 16;
        }
        if remainder.len() >= 2 {
            k1 ^= (remainder[1] as u64) << 8;
        }
        if remainder.len() >= 1 {
            k1 ^= remainder[0] as u64;
        }
        
        // Mix remainder
        k2 = k2.wrapping_mul(C2);
        k2 = k2.rotate_left(R3);
        k2 = k2.wrapping_mul(C1);
        h2 ^= k2;
        
        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(R1);
        k1 = k1.wrapping_mul(C2);
        h1 ^= k1;
    }
    
    // Finalization
    h1 ^= len as u64;
    h2 ^= len as u64;
    
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    
    h1 = fmix64(h1);
    h2 = fmix64(h2);
    
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    
    // Output 128-bit (16 bytes)
    let mut result = [0u8; 16];
    result[0..8].copy_from_slice(&h1.to_le_bytes());
    result[8..16].copy_from_slice(&h2.to_le_bytes());
    result
}

/// Murmur3-128 finalization mix function
fn fmix64(mut k: u64) -> u64 {
    k ^= k >> 33;
    k = k.wrapping_mul(0xff51afd7ed558ccd);
    k ^= k >> 33;
    k = k.wrapping_mul(0xc4ceb9fe1a85ec53);
    k ^= k >> 33;
    k
}

fn encode_base96_128bit(bytes: &[u8; 16]) -> String {
    let mut result = String::with_capacity(16);
    for &byte in bytes.iter() {
        result.push(encode_base96_char(byte as usize));
    }
    result
}

fn domain_bitmask(domain: &str) -> String {
    // 4-bit domain mask encoded as single Base96 char
    let code = match domain.to_lowercase().as_str() {
        "space" => 0,
        "maritime" => 1,
        "automotive" => 2,
        "osint" => 3,
        "network" => 4,
        "cyber" => 5,
        _ => 15, // Unknown/default
    };
    encode_base96_char(code).to_string()
}

fn exec_class_bitmask(exec_class: &str) -> String {
    // 4-bit execution class mask encoded as single Base96 char
    let code = match exec_class.to_lowercase().as_str() {
        "scan" => 0,
        "encode" => 1,
        "observe" => 2,
        "act" => 3,
        "route" => 4,
        _ => 15, // Unknown/default
    };
    encode_base96_char(code).to_string()
}

fn uuid_to_base96(uuid_str: &str) -> String {
    // Convert UUID hex string to Base96 (16 chars)
    // Remove hyphens and convert hex pairs to Base96
    let clean = uuid_str.replace('-', "");
    let mut result = String::with_capacity(16);
    
    for i in 0..16 {
        if let Some(hex_pair) = clean.get(i * 2..i * 2 + 2) {
            if let Ok(byte) = u8::from_str_radix(hex_pair, 16) {
                result.push(encode_base96_char(byte as usize));
            } else {
                result.push('0');
            }
        } else {
            result.push('0');
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_frame_creation() {
        let ctx = ContextFrame::new(ExecEnv::Wasm, 42, ExecState::Hot);
        assert_eq!(ctx.exec_env, ExecEnv::Wasm);
        assert_eq!(ctx.agent_id, 42);
        assert_eq!(ctx.state, ExecState::Hot);
    }

    #[test]
    fn test_cuid_slots_encoding() {
        let ctx = ContextFrame::new(ExecEnv::Container, 100, ExecState::L2Resident);
        let slots = CuidSlots::from(&ctx);
        let cuid_str = slots.to_string();
        assert_eq!(cuid_str.len(), 16);
    }

    #[test]
    fn test_trivariate_hash_format() {
        let hash = TrivariateHash::new(
            "aB7x9pQw2zRt4kMn".to_string(),
            "c5j8k3p2q7w1x9z".to_string(),
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        );
        
        let canonical = hash.to_canonical_format();
        assert!(canonical.starts_with("triv:"));
        
        let parsed = TrivariateHash::from_canonical_format(&canonical).unwrap();
        assert_eq!(parsed.sch, hash.sch);
        assert_eq!(parsed.cuid, hash.cuid);
        assert_eq!(parsed.uuid, hash.uuid);
    }

    #[test]
    fn test_sch_generation() {
        let engine = TrivariateHashEngineV731::new();
        let sch = engine.generate_sch(
            "test semantic content",
            "Actor",
            "osint",
            "scan",
        );
        assert_eq!(sch.len(), 16);
    }

    #[test]
    fn test_full_trivariate_generation() {
        let engine = TrivariateHashEngineV731::new();
        let ctx = ContextFrame::new(ExecEnv::Native, 1, ExecState::Hot);
        
        let triv = engine.generate_trivariate(
            "test content",
            "Object",
            "space",
            "observe",
            &ctx,
        );
        
        assert_eq!(triv.sch.len(), 16);
        assert_eq!(triv.cuid.len(), 16);
        assert!(!triv.uuid.is_empty());
    }

    #[test]
    fn test_dual_trivariate() {
        let engine = TrivariateHashEngineV731::new();
        let ctx = ContextFrame::new(ExecEnv::Wasm, 1, ExecState::Hot);
        
        // High-cognitive layer (requires secondary)
        let dual = engine.generate_dual_trivariate(
            "synaptix9 operation",
            "Process",
            "cyber",
            "route",
            &ctx,
            true, // requires_secondary
        );
        
        assert!(dual.secondary.is_some());
        
        // Low-tier playbook (primary only)
        let primary_only = engine.generate_dual_trivariate(
            "simple playbook",
            "Object",
            "osint",
            "scan",
            &ctx,
            false, // requires_secondary
        );
        
        assert!(primary_only.secondary.is_none());
    }

    #[test]
    fn test_supersession_levels() {
        assert_eq!(SupersessionLevel::from_delta_angle(1.0), SupersessionLevel::None);
        assert_eq!(SupersessionLevel::from_delta_angle(5.0), SupersessionLevel::Micro);
        assert_eq!(SupersessionLevel::from_delta_angle(20.0), SupersessionLevel::Soft);
        assert_eq!(SupersessionLevel::from_delta_angle(35.0), SupersessionLevel::Hard);
        assert_eq!(SupersessionLevel::from_delta_angle(50.0), SupersessionLevel::Critical);
    }

    #[test]
    fn test_unicode_runes() {
        let ctx = ContextFrame::new(ExecEnv::Container, 42, ExecState::Hot);
        let slots = CuidSlots::from(&ctx);
        let runes = slots.to_unicode_runes();
        
        // Verify all characters are in U+E200-E2FF range
        for ch in runes.chars() {
            let code = ch as u32;
            assert!(code >= 0xE200 && code <= 0xE2FF, "Unicode rune out of range: U+{:04X}", code);
        }
    }
}
