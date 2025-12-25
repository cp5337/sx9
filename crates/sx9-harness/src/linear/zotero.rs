//! Zotero SQLite Integration
//!
//! Queries the local Zotero SQLite database for scholarly references.
//! Default location: ~/Zotero/zotero.sqlite
//!
//! ## Schema Overview
//!
//! Key tables:
//! - items: Core item records
//! - itemData: Key-value metadata
//! - fields: Field definitions (title, abstract, DOI, etc.)
//! - creators: Author/contributor records
//! - itemCreators: Many-to-many item-creator links

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Zotero item from local database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoteroItem {
    /// Zotero internal ID
    pub item_id: i64,

    /// Item key (unique identifier)
    pub key: String,

    /// Item type (journalArticle, book, etc.)
    pub item_type: String,

    /// Title
    pub title: String,

    /// Authors
    pub authors: Vec<String>,

    /// Year published
    pub year: Option<i32>,

    /// DOI
    pub doi: Option<String>,

    /// URL
    pub url: Option<String>,

    /// Abstract
    pub abstract_text: Option<String>,

    /// Publication/Journal name
    pub publication: Option<String>,

    /// Tags
    pub tags: Vec<String>,
}

/// Zotero database client
pub struct ZoteroClient {
    /// Path to zotero.sqlite
    db_path: PathBuf,

    /// Connection state
    connected: bool,
}

impl ZoteroClient {
    /// Default Zotero database path
    pub fn default_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/Users".to_string());
        PathBuf::from(home).join("Zotero").join("zotero.sqlite")
    }

    /// Create client with default path
    pub fn new() -> Self {
        Self {
            db_path: Self::default_path(),
            connected: false,
        }
    }

    /// Create client with custom path
    pub fn with_path(path: impl AsRef<Path>) -> Self {
        Self {
            db_path: path.as_ref().to_path_buf(),
            connected: false,
        }
    }

    /// Check if Zotero database exists
    pub fn exists(&self) -> bool {
        self.db_path.exists()
    }

    /// Get database path
    pub fn path(&self) -> &Path {
        &self.db_path
    }

    /// Search items by keywords
    ///
    /// Note: This requires rusqlite dependency. For now, returns placeholder
    /// items. In production, this would execute:
    ///
    /// ```sql
    /// SELECT i.itemID, i.key, it.typeName,
    ///        (SELECT value FROM itemData id
    ///         JOIN itemDataValues idv ON id.valueID = idv.valueID
    ///         JOIN fields f ON id.fieldID = f.fieldID
    ///         WHERE id.itemID = i.itemID AND f.fieldName = 'title') as title
    /// FROM items i
    /// JOIN itemTypes it ON i.itemTypeID = it.itemTypeID
    /// WHERE i.itemID IN (
    ///     SELECT id.itemID FROM itemData id
    ///     JOIN itemDataValues idv ON id.valueID = idv.valueID
    ///     WHERE idv.value LIKE '%keyword%'
    /// )
    /// LIMIT 10
    /// ```
    pub fn search(&self, keywords: &[String], limit: usize) -> Vec<ZoteroItem> {
        if !self.exists() {
            // Zotero database not found
            return Vec::new();
        }

        // In production, this would use rusqlite to query the database
        // For now, return empty - the scholarly module will fall back to web search
        let _ = (keywords, limit); // Suppress unused warnings

        Vec::new()
    }

    /// Get item by key
    pub fn get_by_key(&self, key: &str) -> Option<ZoteroItem> {
        if !self.exists() {
            return None;
        }

        let _ = key; // Suppress unused warning
        None
    }

    /// Get recent items
    pub fn recent(&self, limit: usize) -> Vec<ZoteroItem> {
        if !self.exists() {
            return Vec::new();
        }

        let _ = limit; // Suppress unused warning
        Vec::new()
    }

    /// Get items by tag
    pub fn by_tag(&self, tag: &str, limit: usize) -> Vec<ZoteroItem> {
        if !self.exists() {
            return Vec::new();
        }

        let _ = (tag, limit); // Suppress unused warnings
        Vec::new()
    }

    /// Test connection to database
    pub fn test_connection(&mut self) -> bool {
        if !self.exists() {
            self.connected = false;
            return false;
        }

        // Would test opening the database here
        self.connected = true;
        true
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

impl Default for ZoteroClient {
    fn default() -> Self {
        Self::new()
    }
}

/// SQL query builder for Zotero database
#[allow(dead_code)]
pub struct ZoteroQuery {
    keywords: Vec<String>,
    item_types: Vec<String>,
    tags: Vec<String>,
    year_from: Option<i32>,
    year_to: Option<i32>,
    limit: usize,
}

impl Default for ZoteroQuery {
    fn default() -> Self {
        Self {
            keywords: Vec::new(),
            item_types: Vec::new(),
            tags: Vec::new(),
            year_from: None,
            year_to: None,
            limit: 10,
        }
    }
}

#[allow(dead_code)]
impl ZoteroQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keywords(mut self, keywords: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.keywords = keywords.into_iter().map(|k| k.into()).collect();
        self
    }

    pub fn item_types(mut self, types: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.item_types = types.into_iter().map(|t| t.into()).collect();
        self
    }

    pub fn tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tags = tags.into_iter().map(|t| t.into()).collect();
        self
    }

    pub fn year_range(mut self, from: Option<i32>, to: Option<i32>) -> Self {
        self.year_from = from;
        self.year_to = to;
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Build SQL query string
    pub fn to_sql(&self) -> String {
        let mut sql = String::from(
            r#"
            SELECT DISTINCT i.itemID, i.key, it.typeName as itemType
            FROM items i
            JOIN itemTypes it ON i.itemTypeID = it.itemTypeID
            "#,
        );

        // Add keyword search joins
        if !self.keywords.is_empty() {
            sql.push_str(
                r#"
                JOIN itemData id ON i.itemID = id.itemID
                JOIN itemDataValues idv ON id.valueID = idv.valueID
                "#,
            );
        }

        // Add tag joins
        if !self.tags.is_empty() {
            sql.push_str(
                r#"
                JOIN itemTags itg ON i.itemID = itg.itemID
                JOIN tags t ON itg.tagID = t.tagID
                "#,
            );
        }

        sql.push_str(" WHERE 1=1 ");

        // Add keyword conditions
        if !self.keywords.is_empty() {
            let keyword_conditions: Vec<String> = self
                .keywords
                .iter()
                .map(|k| format!("idv.value LIKE '%{}%'", k.replace('\'', "''")))
                .collect();
            sql.push_str(&format!(" AND ({}) ", keyword_conditions.join(" OR ")));
        }

        // Add item type filter
        if !self.item_types.is_empty() {
            let types: Vec<String> = self
                .item_types
                .iter()
                .map(|t| format!("'{}'", t.replace('\'', "''")))
                .collect();
            sql.push_str(&format!(" AND it.typeName IN ({}) ", types.join(", ")));
        }

        // Add tag filter
        if !self.tags.is_empty() {
            let tags: Vec<String> = self
                .tags
                .iter()
                .map(|t| format!("'{}'", t.replace('\'', "''")))
                .collect();
            sql.push_str(&format!(" AND t.name IN ({}) ", tags.join(", ")));
        }

        sql.push_str(&format!(" LIMIT {} ", self.limit));

        sql
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_path() {
        let path = ZoteroClient::default_path();
        assert!(path.to_string_lossy().contains("Zotero"));
        assert!(path.to_string_lossy().ends_with("zotero.sqlite"));
    }

    #[test]
    fn test_client_creation() {
        let client = ZoteroClient::new();
        // Path exists check is filesystem-dependent
        assert!(!client.is_connected());
    }

    #[test]
    fn test_custom_path() {
        let client = ZoteroClient::with_path("/custom/path/zotero.sqlite");
        assert_eq!(client.path().to_string_lossy(), "/custom/path/zotero.sqlite");
    }

    #[test]
    fn test_query_builder() {
        let query = ZoteroQuery::new()
            .keywords(["rust", "async"])
            .item_types(["journalArticle", "conferencePaper"])
            .limit(20);

        let sql = query.to_sql();
        assert!(sql.contains("LIKE '%rust%'"));
        assert!(sql.contains("LIKE '%async%'"));
        assert!(sql.contains("journalArticle"));
        assert!(sql.contains("LIMIT 20"));
    }
}
