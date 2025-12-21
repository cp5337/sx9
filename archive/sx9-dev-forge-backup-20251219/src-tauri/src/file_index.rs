//! File Index for SX9 Dev Forge
//! 
//! Helps AI agents discover files via tag-based search
//! Transient index - rebuilds on demand
//! Future: Add symbol/rune indexing for functions/classes

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use walkdir::WalkDir;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileIndexError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub relative_path: String,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub file_type: String,
    pub last_indexed: DateTime<Utc>,
    pub size: u64,
    // Future: Add symbol/rune data
    // pub symbols: Vec<Symbol>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_files: usize,
    pub by_type: std::collections::HashMap<String, usize>,
    pub by_tag: std::collections::HashMap<String, usize>,
    pub last_indexed: DateTime<Utc>,
}

pub struct FileIndex {
    db: sled::Db,
    root_path: PathBuf,
}

impl FileIndex {
    /// Create new file index
    pub fn new(root_path: PathBuf) -> Result<Self, FileIndexError> {
        let db_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".sx9")
            .join("file_index");
        
        std::fs::create_dir_all(&db_path).map_err(|e| FileIndexError::Io(e))?;
        
        let db = sled::open(db_path)
            .map_err(|e| FileIndexError::Database(e.to_string()))?;
        
        Ok(Self { db, root_path })
    }

    /// Index entire workspace
    pub fn index_workspace(&self) -> Result<usize, FileIndexError> {
        let mut count = 0;
        
        // Clear existing index
        self.db.clear().map_err(|e| FileIndexError::Database(e.to_string()))?;
        
        for entry in WalkDir::new(&self.root_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| !self.should_ignore(e.path()))
        {
            let path = entry.path();
            let tags = self.auto_tag_file(path);
            
            let file_entry = FileEntry {
                path: path.to_path_buf(),
                relative_path: path.strip_prefix(&self.root_path)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string(),
                tags,
                description: None,
                file_type: self.detect_file_type(path),
                last_indexed: Utc::now(),
                size: entry.metadata().map(|m| m.len()).unwrap_or(0),
            };
            
            self.store_entry(&file_entry)?;
            count += 1;
        }
        
        Ok(count)
    }

    /// Auto-tag file based on path and extension
    fn auto_tag_file(&self, path: &Path) -> Vec<String> {
        let mut tags = Vec::new();
        let path_str = path.to_string_lossy().to_lowercase();
        
        // Directory-based tags
        if path_str.contains("src/api") { tags.push("api".into()); }
        if path_str.contains("src/auth") { tags.push("auth".into()); }
        if path_str.contains("src/components") { tags.push("ui".into()); }
        if path_str.contains("src/pages") { tags.push("page".into()); }
        if path_str.contains("src/hooks") { tags.push("hook".into()); }
        if path_str.contains("src-tauri") { tags.push("backend".into()); }
        if path_str.contains("/cli/") { tags.push("cli".into()); }
        
        // Extension-based tags
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            tags.push(ext.to_string());
            match ext {
                "rs" => tags.push("rust".into()),
                "ts" | "tsx" => {
                    tags.push("typescript".into());
                    if ext == "tsx" { tags.push("react".into()); }
                }
                "toml" | "yaml" | "json" => tags.push("config".into()),
                "md" => tags.push("docs".into()),
                "css" => tags.push("style".into()),
                _ => {}
            }
        }
        
        // Filename pattern tags
        if path_str.contains("test") { tags.push("test".into()); }
        if path_str.contains("util") { tags.push("utility".into()); }
        if path_str.contains("command") { tags.push("command".into()); }
        if path_str.contains("vault") { tags.push("vault".into()); }
        if path_str.contains("clipboard") { tags.push("clipboard".into()); }
        if path_str.contains("mission") { tags.push("mission".into()); }
        
        tags
    }

    /// Detect file type from extension
    fn detect_file_type(&self, path: &Path) -> String {
        path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    /// Check if file should be ignored
    fn should_ignore(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        // Ignore common directories
        path_str.contains("/node_modules/") ||
        path_str.contains("/target/") ||
        path_str.contains("/.git/") ||
        path_str.contains("/dist/") ||
        path_str.contains("/build/") ||
        path_str.contains("/.next/")
    }

    /// Store file entry in database
    fn store_entry(&self, entry: &FileEntry) -> Result<(), FileIndexError> {
        let key = entry.relative_path.as_bytes();
        let value = serde_json::to_vec(entry)?;
        self.db.insert(key, value)
            .map_err(|e| FileIndexError::Database(e.to_string()))?;
        Ok(())
    }

    /// Search files by tags (OR logic - any tag matches)
    pub fn search_by_tags(&self, tags: &[String]) -> Result<Vec<FileEntry>, FileIndexError> {
        let results: Vec<FileEntry> = self.db.iter()
            .filter_map(|item| item.ok())
            .filter_map(|(_, value)| serde_json::from_slice::<FileEntry>(&value).ok())
            .filter(|entry| tags.iter().any(|tag| entry.tags.contains(tag)))
            .collect();
        
        Ok(results)
    }

    /// Search files by text in path
    pub fn search_by_path(&self, query: &str) -> Result<Vec<FileEntry>, FileIndexError> {
        let query_lower = query.to_lowercase();
        let results: Vec<FileEntry> = self.db.iter()
            .filter_map(|item| item.ok())
            .filter_map(|(_, value)| serde_json::from_slice::<FileEntry>(&value).ok())
            .filter(|entry| entry.relative_path.to_lowercase().contains(&query_lower))
            .collect();
        
        Ok(results)
    }

    /// Get recent files (by index time)
    pub fn recent(&self, limit: usize) -> Result<Vec<FileEntry>, FileIndexError> {
        let mut results: Vec<FileEntry> = self.db.iter()
            .filter_map(|item| item.ok())
            .filter_map(|(_, value)| serde_json::from_slice::<FileEntry>(&value).ok())
            .collect();
        
        results.sort_by(|a, b| b.last_indexed.cmp(&a.last_indexed));
        results.truncate(limit);
        
        Ok(results)
    }

    /// Get index statistics
    pub fn stats(&self) -> Result<IndexStats, FileIndexError> {
        let mut by_type = std::collections::HashMap::new();
        let mut by_tag = std::collections::HashMap::new();
        let mut total = 0;
        let mut last_indexed = Utc::now();

        for item in self.db.iter().filter_map(|i| i.ok()) {
            if let Ok(entry) = serde_json::from_slice::<FileEntry>(&item.1) {
                total += 1;
                *by_type.entry(entry.file_type.clone()).or_insert(0) += 1;
                
                for tag in &entry.tags {
                    *by_tag.entry(tag.clone()).or_insert(0) += 1;
                }
                
                if entry.last_indexed < last_indexed {
                    last_indexed = entry.last_indexed;
                }
            }
        }

        Ok(IndexStats {
            total_files: total,
            by_type,
            by_tag,
            last_indexed,
        })
    }

    /// Add custom tag to file
    pub fn tag_file(&self, relative_path: &str, tag: String) -> Result<(), FileIndexError> {
        let key = relative_path.as_bytes();
        
        if let Some(value) = self.db.get(key)
            .map_err(|e| FileIndexError::Database(e.to_string()))? 
        {
            let mut entry: FileEntry = serde_json::from_slice(&value)?;
            if !entry.tags.contains(&tag) {
                entry.tags.push(tag);
                self.store_entry(&entry)?;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_tag_rust_file() {
        let index = FileIndex::new(PathBuf::from(".")).unwrap();
        let tags = index.auto_tag_file(Path::new("src-tauri/src/lib.rs"));
        
        assert!(tags.contains(&"backend".to_string()));
        assert!(tags.contains(&"rust".to_string()));
        assert!(tags.contains(&"rs".to_string()));
    }

    #[test]
    fn test_auto_tag_react_component() {
        let index = FileIndex::new(PathBuf::from(".")).unwrap();
        let tags = index.auto_tag_file(Path::new("src/components/Button.tsx"));
        
        assert!(tags.contains(&"ui".to_string()));
        assert!(tags.contains(&"typescript".to_string()));
        assert!(tags.contains(&"react".to_string()));
    }
}
