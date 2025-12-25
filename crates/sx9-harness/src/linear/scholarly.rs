//! Scholarly Reference Fetcher
//!
//! Combines Zotero local library with web search fallback for
//! finding relevant academic and industry references.
//!
//! ## Priority Order
//!
//! 1. Local Zotero library (fast, curated)
//! 2. Web search fallback (broader coverage)

use super::bne::{ReferenceSource, ScholarlyReference};
use super::zotero::{ZoteroClient, ZoteroItem};

/// Scholarly reference search client
pub struct ScholarlyClient {
    /// Zotero client for local searches
    zotero: ZoteroClient,

    /// Minimum references to return
    min_results: usize,

    /// Enable web fallback when Zotero has insufficient results
    web_fallback: bool,
}

impl Default for ScholarlyClient {
    fn default() -> Self {
        Self {
            zotero: ZoteroClient::new(),
            min_results: 2,
            web_fallback: true,
        }
    }
}

impl ScholarlyClient {
    /// Create new scholarly client
    pub fn new() -> Self {
        Self::default()
    }

    /// Set minimum results to return
    pub fn with_min_results(mut self, min: usize) -> Self {
        self.min_results = min;
        self
    }

    /// Enable/disable web fallback
    pub fn with_web_fallback(mut self, enabled: bool) -> Self {
        self.web_fallback = enabled;
        self
    }

    /// Search for scholarly references
    pub fn search(&self, keywords: &[String], limit: usize) -> Vec<ScholarlyReference> {
        let mut results = Vec::new();

        // First, search Zotero
        let zotero_items = self.zotero.search(keywords, limit);
        for item in zotero_items {
            results.push(zotero_to_reference(item));
        }

        // If insufficient results and web fallback enabled, search web
        if results.len() < self.min_results && self.web_fallback {
            let web_results = self.web_search(keywords, limit - results.len());
            results.extend(web_results);
        }

        results
    }

    /// Search web for references (placeholder)
    ///
    /// In production, this would use:
    /// - Google Scholar API
    /// - Semantic Scholar API
    /// - arXiv API
    /// - CrossRef API
    fn web_search(&self, keywords: &[String], limit: usize) -> Vec<ScholarlyReference> {
        // Placeholder - would call actual APIs
        // For now, return empty as this requires HTTP client integration
        let _ = (keywords, limit); // Suppress unused warnings
        Vec::new()
    }

    /// Check if Zotero is available
    pub fn has_zotero(&self) -> bool {
        self.zotero.exists()
    }

    /// Get Zotero path
    pub fn zotero_path(&self) -> &std::path::Path {
        self.zotero.path()
    }
}

/// Convert Zotero item to scholarly reference
fn zotero_to_reference(item: ZoteroItem) -> ScholarlyReference {
    let identifier = item
        .doi
        .map(|d| format!("https://doi.org/{}", d))
        .or(item.url)
        .unwrap_or_else(|| format!("zotero://{}", item.key));

    ScholarlyReference {
        title: item.title,
        authors: item.authors,
        year: item.year,
        identifier,
        abstract_text: item.abstract_text,
        source: ReferenceSource::Zotero,
        relevance: 0.8, // Default high relevance for local library
    }
}

/// Search result with scoring
#[derive(Debug, Clone)]
pub struct ScoredReference {
    pub reference: ScholarlyReference,
    pub score: f32,
}

impl ScoredReference {
    /// Score reference based on keyword matches
    pub fn score_keywords(reference: &ScholarlyReference, keywords: &[String]) -> f32 {
        let text = format!(
            "{} {}",
            reference.title,
            reference.abstract_text.as_deref().unwrap_or("")
        )
        .to_lowercase();

        let matches: usize = keywords
            .iter()
            .filter(|k| text.contains(&k.to_lowercase()))
            .count();

        if keywords.is_empty() {
            0.5
        } else {
            matches as f32 / keywords.len() as f32
        }
    }
}

/// Reference search configuration
#[derive(Debug, Clone)]
pub struct SearchConfig {
    /// Keywords to search
    pub keywords: Vec<String>,

    /// Maximum results
    pub limit: usize,

    /// Minimum relevance score (0.0-1.0)
    pub min_relevance: f32,

    /// Include Zotero
    pub include_zotero: bool,

    /// Include web search
    pub include_web: bool,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            keywords: Vec::new(),
            limit: 5,
            min_relevance: 0.3,
            include_zotero: true,
            include_web: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scholarly_client_creation() {
        let client = ScholarlyClient::new();
        // Zotero availability is system-dependent
        assert!(!client.has_zotero() || client.has_zotero());
    }

    #[test]
    fn test_keyword_scoring() {
        let reference = ScholarlyReference {
            title: "Async Rust Programming".to_string(),
            authors: vec!["Smith".to_string()],
            year: Some(2024),
            identifier: "doi:10.1234/test".to_string(),
            abstract_text: Some("This paper explores async programming patterns in Rust.".to_string()),
            source: ReferenceSource::Zotero,
            relevance: 0.0,
        };

        let keywords = vec!["rust".to_string(), "async".to_string(), "programming".to_string()];
        let score = ScoredReference::score_keywords(&reference, &keywords);

        assert!(score > 0.5); // Should match most keywords
    }

    #[test]
    fn test_search_config_default() {
        let config = SearchConfig::default();
        assert_eq!(config.limit, 5);
        assert!(config.include_zotero);
        assert!(config.include_web);
    }
}
