use std::collections::HashMap;
use super::analysis_engine::RepoPromptAnalysis;

/// Cache manager for analysis performance optimization
#[derive(Debug, Clone)]
pub struct CacheManager {
    analysis_cache: HashMap<String, RepoPromptAnalysis>,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            analysis_cache: HashMap::new(),
        }
    }

    /// Get cached analysis if available
    pub fn get_cached_analysis(&self, content_hash: &str) -> Option<RepoPromptAnalysis> {
        self.analysis_cache.get(content_hash).cloned()
    }

    /// Cache analysis result
    pub fn cache_analysis(&mut self, content_hash: &str, analysis: &RepoPromptAnalysis) {
        self.analysis_cache.insert(content_hash.to_string(), analysis);
    }

    /// Get total number of analyses
    pub fn get_total_analyses(&self) -> u64 {
        self.analysis_cache.len() as u64
    }

    /// Get number of cached analyses
    pub fn get_cached_count(&self) -> u64 {
        self.analysis_cache.len() as u64
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.analysis_cache.clear();
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        CacheStats {
            total_entries: self.analysis_cache.len(),
            memory_usage_estimate: self.analysis_cache.len() * 1024, // Rough estimate
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub memory_usage_estimate: usize,
} 