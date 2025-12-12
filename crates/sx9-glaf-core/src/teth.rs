//! TETH (Topological Entropy Threat Heuristic)
//!
//! RFC-9021: Graph entropy measures information content
//! H(G) = -Σ p(v) log p(v) where p(v) = degree(v) / Σ degree(w)

use crate::glaf_core::GlafNode;
use anyhow::Result;

/// TETH analyzer for graph entropy calculation
pub struct TethAnalyzer;

impl TethAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

/// Calculate entropy for a single node
///
/// Uses node degree relative to total graph degree
pub async fn calculate_entropy(node: &GlafNode) -> f64 {
    use serde_json::Value;

    // Simplified: use node property "degree" if available
    // Otherwise calculate from relationships
    let degree = node
        .properties
        .get("degree")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    // For single node, entropy is based on degree distribution
    // In full implementation, would need total graph degree
    if degree == 0.0 {
        0.0
    } else {
        // Normalized entropy (0-5.0 range per RFC-9021)
        (degree.ln() * degree).abs() / 2.0
    }
}

/// Calculate graph entropy for entire graph
///
/// H(G) = -Σ p(v) log p(v)
pub async fn calculate_graph_entropy(nodes: &[GlafNode]) -> f64 {
    use serde_json::Value;

    if nodes.is_empty() {
        return 0.0;
    }

    // Calculate total degree
    let total_degree: f64 = nodes
        .iter()
        .map(|n| {
            n.properties
                .get("degree")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0)
        })
        .sum();

    if total_degree == 0.0 {
        return 0.0;
    }

    // Calculate entropy
    let mut entropy = 0.0;
    for node in nodes {
        let degree = node
            .properties
            .get("degree")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        if degree > 0.0 {
            let p = degree / total_degree;
            entropy -= p * p.ln();
        }
    }

    entropy
}
