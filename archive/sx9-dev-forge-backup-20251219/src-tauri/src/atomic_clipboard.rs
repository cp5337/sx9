use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sled::Db;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Clipboard entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEntry {
    /// Unique identifier (UUID)
    pub id: String,
    /// Actual content/data
    pub content: String,
    /// Source of the entry ("mission", "linear", "voice", "manual", "prompt")
    pub source: String,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Additional metadata (flexible JSON)
    pub metadata: serde_json::Value,
}

/// Atomic Clipboard - Cross-system memory buffer
pub struct AtomicClipboard {
    /// Sled database for persistence
    db: Db,
    /// In-memory cache for fast access
    cache: Arc<RwLock<Vec<ClipboardEntry>>>,
    /// Storage path
    path: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum ClipboardError {
    #[error("Database error: {0}")]
    Database(#[from] sled::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Entry not found: {0}")]
    NotFound(String),
}

impl AtomicClipboard {
    /// Create new Atomic Clipboard with default path
    pub fn new() -> Result<Self, ClipboardError> {
        let path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".sx9")
            .join("clipboard");
        
        Self::with_path(&path)
    }
    
    /// Create Atomic Clipboard at specific path
    pub fn with_path(path: &PathBuf) -> Result<Self, ClipboardError> {
        // Ensure directory exists
        std::fs::create_dir_all(path)?;
        
        // Open Sled database
        let db_path = path.join("clipboard.sled");
        let db = sled::open(&db_path)?;
        
        let clipboard = Self {
            db,
            cache: Arc::new(RwLock::new(Vec::new())),
            path: path.clone(),
        };
        
        // Load existing entries into cache
        clipboard.load_cache()?;
        
        Ok(clipboard)
    }
    
    /// Load all entries from database into cache
    fn load_cache(&self) -> Result<(), ClipboardError> {
        let mut cache = futures::executor::block_on(self.cache.write());
        cache.clear();
        
        for item in self.db.iter() {
            let (_, value) = item?;
            let entry: ClipboardEntry = serde_json::from_slice(&value)?;
            cache.push(entry);
        }
        
        // Sort by created_at (newest first)
        cache.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        Ok(())
    }
    
    /// Push new entry to clipboard
    pub async fn push(&self, entry: ClipboardEntry) -> Result<String, ClipboardError> {
        let id = entry.id.clone();
        
        // Serialize and store in database
        let key = id.as_bytes();
        let value = serde_json::to_vec(&entry)?;
        self.db.insert(key, value)?;
        
        // Add to cache
        let mut cache = self.cache.write().await;
        cache.insert(0, entry); // Insert at beginning (newest first)
        
        // Limit cache size to 1000 entries
        if cache.len() > 1000 {
            cache.truncate(1000);
        }
        
        Ok(id)
    }
    
    /// List recent entries
    pub async fn list(&self, limit: usize) -> Result<Vec<ClipboardEntry>, ClipboardError> {
        let cache = self.cache.read().await;
        Ok(cache.iter().take(limit).cloned().collect())
    }
    
    /// Get specific entry by ID
    pub async fn get(&self, id: &str) -> Result<Option<ClipboardEntry>, ClipboardError> {
        // Check cache first
        let cache = self.cache.read().await;
        if let Some(entry) = cache.iter().find(|e| e.id == id) {
            return Ok(Some(entry.clone()));
        }
        
        // Fallback to database
        if let Some(value) = self.db.get(id.as_bytes())? {
            let entry: ClipboardEntry = serde_json::from_slice(&value)?;
            Ok(Some(entry))
        } else {
            Ok(None)
        }
    }
    
    /// Search entries by tag
    pub async fn search_by_tag(&self, tag: &str) -> Result<Vec<ClipboardEntry>, ClipboardError> {
        let cache = self.cache.read().await;
        Ok(cache
            .iter()
            .filter(|e| e.tags.contains(&tag.to_string()))
            .cloned()
            .collect())
    }
    
    /// Search entries by source
    pub async fn search_by_source(&self, source: &str) -> Result<Vec<ClipboardEntry>, ClipboardError> {
        let cache = self.cache.read().await;
        Ok(cache
            .iter()
            .filter(|e| e.source == source)
            .cloned()
            .collect())
    }
    
    /// Clear all entries
    pub async fn clear(&self) -> Result<(), ClipboardError> {
        self.db.clear()?;
        let mut cache = self.cache.write().await;
        cache.clear();
        Ok(())
    }
    
    /// Get statistics
    pub async fn stats(&self) -> ClipboardStats {
        let cache = self.cache.read().await;
        
        let mut sources = std::collections::HashMap::new();
        let mut tags = std::collections::HashMap::new();
        
        for entry in cache.iter() {
            *sources.entry(entry.source.clone()).or_insert(0) += 1;
            for tag in &entry.tags {
                *tags.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        
        ClipboardStats {
            total_entries: cache.len(),
            sources,
            tags,
            oldest_entry: cache.last().map(|e| e.created_at),
            newest_entry: cache.first().map(|e| e.created_at),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipboardStats {
    pub total_entries: usize,
    pub sources: std::collections::HashMap<String, usize>,
    pub tags: std::collections::HashMap<String, usize>,
    pub oldest_entry: Option<DateTime<Utc>>,
    pub newest_entry: Option<DateTime<Utc>>,
}
