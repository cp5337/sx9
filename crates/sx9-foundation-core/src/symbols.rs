//! Symbolic representation system for CTAS v7.0
//! 
//! UTF8-encoded task hashes and entropy symbols for visual identification

use serde::{Deserialize, Serialize};

/// Entropy symbols for persona identification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntropySymbol {
    /// Squid symbol (⧉) - DevSecOps
    Squid,
    /// Vortex symbol (🌀) - Strategic Ops
    Vortex,
    /// Diamond symbol (◊) - Analytics
    Diamond,
    /// Spiral symbol (⟐) - Research
    Spiral,
    /// Cross symbol (⊕) - Operations
    Cross,
}

impl EntropySymbol {
    /// Get the UTF8 character for this symbol
    pub fn as_char(&self) -> char {
        match self {
            Self::Squid => '⧉',
            Self::Vortex => '🌀',
            Self::Diamond => '◊',
            Self::Spiral => '⟐',
            Self::Cross => '⊕',
        }
    }
    
    /// Get symbol name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Squid => "squid",
            Self::Vortex => "vortex",
            Self::Diamond => "diamond",
            Self::Spiral => "spiral",
            Self::Cross => "cross",
        }
    }
}

impl Default for EntropySymbol {
    fn default() -> Self {
        Self::Squid
    }
}

/// Trust level scale (1-5)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TrustLevel {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

impl TrustLevel {
    /// Get trust level name
    pub fn name(&self) -> &'static str {
        match self {
            Self::One => "minimal",
            Self::Two => "low",
            Self::Three => "medium",
            Self::Four => "high",
            Self::Five => "maximum",
        }
    }
    
    /// Check if trust level meets minimum requirement
    pub fn meets_threshold(&self, min_level: TrustLevel) -> bool {
        *self >= min_level
    }
}

impl Default for TrustLevel {
    fn default() -> Self {
        Self::Three
    }
}

/// Generate symbolic hash for task identification
pub fn generate_symbolic_hash(cuid: &str, entropy: EntropySymbol, content: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    cuid.hash(&mut hasher);
    entropy.name().hash(&mut hasher);
    content.hash(&mut hasher);
    
    let hash = hasher.finish();
    format!("{}{:x}", entropy.as_char(), hash)
}

/// Parse symbolic hash to extract components
pub fn parse_symbolic_hash(hash: &str) -> Option<(EntropySymbol, u64)> {
    if hash.len() < 2 {
        return None;
    }
    
    let symbol_char = hash.chars().next()?;
    let symbol = match symbol_char {
        '⧉' => EntropySymbol::Squid,
        '🌀' => EntropySymbol::Vortex,
        '◊' => EntropySymbol::Diamond,
        '⟐' => EntropySymbol::Spiral,
        '⊕' => EntropySymbol::Cross,
        _ => return None,
    };
    
    let hex_part = &hash[1..];
    let hash_value = u64::from_str_radix(hex_part, 16).ok()?;
    
    Some((symbol, hash_value))
}

