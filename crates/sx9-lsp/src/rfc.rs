//! RFC Index
//!
//! Indexes RFC documents for navigation and reference.

use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use std::path::Path;
use std::sync::RwLock;

/// RFC document index
pub struct RfcIndex {
    /// RFC documents by ID (e.g., "RFC-9141")
    rfcs: RwLock<HashMap<String, RfcEntry>>,
}

/// RFC entry with metadata
#[derive(Debug, Clone)]
struct RfcEntry {
    /// RFC ID (e.g., "RFC-9141")
    id: String,

    /// Title extracted from frontmatter or first heading
    title: String,

    /// File location
    location: Location,

    /// Brief description
    description: Option<String>,

    /// Status (draft, approved, etc.)
    status: Option<String>,
}

impl RfcIndex {
    /// Create new RFC index
    pub fn new() -> Self {
        Self {
            rfcs: RwLock::new(HashMap::new()),
        }
    }

    /// Index a directory for RFCs
    pub async fn index_directory(&self, root: &Path) {
        tracing::info!("Indexing RFCs in: {:?}", root);

        // Look for RFC markdown files
        let rfc_dir = root.join("01-rfc");
        if rfc_dir.exists() {
            self.index_rfc_dir(&rfc_dir).await;
        }

        // Also check root for RFC files
        self.index_rfc_dir(root).await;

        if let Ok(rfcs) = self.rfcs.read() {
            tracing::info!("Indexed {} RFCs", rfcs.len());
        }
    }

    /// Index RFC directory recursively
    async fn index_rfc_dir(&self, dir: &Path) {
        let pattern = dir.join("**/*.md");
        if let Ok(entries) = glob::glob(pattern.to_string_lossy().as_ref()) {
            for entry in entries.flatten() {
                self.index_rfc_file(&entry).await;
            }
        }
    }

    /// Index a single RFC file
    async fn index_rfc_file(&self, path: &Path) {
        let Ok(content) = tokio::fs::read_to_string(path).await else {
            return;
        };

        // Extract RFC ID from filename or content
        let filename = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

        // Check if filename contains RFC pattern
        let rfc_id = if filename.to_uppercase().starts_with("RFC-") {
            filename.to_uppercase()
        } else {
            // Try to find RFC ID in content
            let rfc_regex = regex::Regex::new(r"RFC-(\d+)").unwrap();
            rfc_regex
                .find(&content)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default()
        };

        if rfc_id.is_empty() || !rfc_id.starts_with("RFC-") {
            return;
        }

        // Extract title from first heading
        let title = content
            .lines()
            .find(|l| l.starts_with('#'))
            .map(|l| l.trim_start_matches('#').trim().to_string())
            .unwrap_or_else(|| rfc_id.clone());

        // Extract description from first paragraph
        let description = content
            .lines()
            .skip_while(|l| l.starts_with('#') || l.is_empty())
            .take_while(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .join(" ");

        // Extract status from frontmatter
        let status = if content.starts_with("---") {
            let frontmatter_end = content[3..].find("---").map(|i| i + 3);
            if let Some(end) = frontmatter_end {
                let frontmatter = &content[3..end];
                frontmatter
                    .lines()
                    .find(|l| l.starts_with("status:"))
                    .map(|l| l.trim_start_matches("status:").trim().to_string())
            } else {
                None
            }
        } else {
            None
        };

        let uri = Url::from_file_path(path).ok();
        if let Some(uri) = uri {
            let entry = RfcEntry {
                id: rfc_id.clone(),
                title,
                location: Location {
                    uri,
                    range: Range::default(),
                },
                description: if description.is_empty() {
                    None
                } else {
                    Some(description)
                },
                status,
            };

            if let Ok(mut rfcs) = self.rfcs.write() {
                rfcs.insert(rfc_id, entry);
            }
        }
    }

    /// Find RFC by ID
    pub fn find_rfc(&self, id: &str) -> Option<Location> {
        let id = id.to_uppercase();
        self.rfcs.read().ok()?.get(&id).map(|e| e.location.clone())
    }

    /// Get RFC info for hover
    pub fn get_info(&self, id: &str) -> Option<String> {
        let id = id.to_uppercase();
        self.rfcs.read().ok()?.get(&id).map(|e| {
            let mut info = format!("## {}: {}\n\n", e.id, e.title);

            if let Some(status) = &e.status {
                info.push_str(&format!("**Status:** {}\n\n", status));
            }

            if let Some(desc) = &e.description {
                info.push_str(&format!("{}\n", desc));
            }

            info
        })
    }

    /// List all indexed RFCs
    pub fn list_all(&self) -> Vec<(String, Location)> {
        self.rfcs
            .read()
            .ok()
            .map(|rfcs| {
                rfcs.values()
                    .map(|e| (e.id.clone(), e.location.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Search RFCs by keyword
    pub fn search(&self, query: &str) -> Vec<(String, Location)> {
        let query = query.to_lowercase();
        self.rfcs
            .read()
            .ok()
            .map(|rfcs| {
                rfcs.values()
                    .filter(|e| {
                        e.id.to_lowercase().contains(&query)
                            || e.title.to_lowercase().contains(&query)
                            || e.description
                                .as_ref()
                                .map(|d| d.to_lowercase().contains(&query))
                                .unwrap_or(false)
                    })
                    .map(|e| (e.id.clone(), e.location.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl Default for RfcIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rfc_index_creation() {
        let index = RfcIndex::new();
        assert!(index.list_all().is_empty());
    }
}
