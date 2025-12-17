use sx9_foundation_core::TrivariateHashEngine;
use serde::{Deserialize, Serialize};

/// Trivariate Hash System (H1, H2, H3)
/// Implements the "Nonagon Analytics" logic for slicing data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TrivariateHash {
    /// H1: Identity Hash (Who/What) - Semantic
    pub h1: String,
    
    /// H2: Execution Hash (How/Action) - Operational
    pub h2: String,
    
    /// H3: Context Hash (Where/When/Crystal) - Spatiotemporal
    pub h3: String,
}

impl TrivariateHash {
    pub fn new(h1: &str, h2: &str, h3: &str) -> Self {
        Self {
            h1: h1.to_string(),
            h2: h2.to_string(),
            h3: h3.to_string(),
        }
    }

    /// Generates H3 from Context + Crystal
    pub fn generate_h3(domain: &str, crystal: &str, timestamp: i64) -> String {
        let payload = format!("{}:{}:{}", domain, crystal, timestamp);
        TrivariateHashEngine::new().generate_hash_from_bytes(payload.as_bytes())
    }

    /// Returns the "Nonagon Cutter" - a combined hash for indexing
    pub fn analytical_cut(&self) -> String {
        let payload = format!("{}:{}:{}", self.h1, self.h2, self.h3);
        TrivariateHashEngine::new().generate_hash_from_bytes(payload.as_bytes())
    }
}
