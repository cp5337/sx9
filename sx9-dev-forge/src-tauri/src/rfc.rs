//! RFC Index and Loader for SX9 Dev Forge

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RfcError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("RFC not found: {0}")]
    NotFound(String),
    #[error("Invalid RFC path: {0}")]
    InvalidPath(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfcMeta {
    pub id: String,
    pub title: String,
    pub status: RfcStatus,
    pub category: String,
    pub path: PathBuf,
    pub dependencies: Vec<String>,
    pub implementations: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RfcStatus {
    Draft,
    Active,
    Implemented,
    Deprecated,
    Superseded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfcIndex {
    pub rfcs: HashMap<String, RfcMeta>,
    pub categories: HashMap<String, Vec<String>>,
    pub tags: HashMap<String, Vec<String>>,
}

impl RfcIndex {
    pub fn new() -> Self {
        Self {
            rfcs: HashMap::new(),
            categories: HashMap::new(),
            tags: HashMap::new(),
        }
    }

    pub fn add(&mut self, rfc: RfcMeta) {
        let id = rfc.id.clone();
        let category = rfc.category.clone();
        let tags = rfc.tags.clone();

        // Add to categories index
        self.categories
            .entry(category)
            .or_default()
            .push(id.clone());

        // Add to tags index
        for tag in tags {
            self.tags.entry(tag).or_default().push(id.clone());
        }

        self.rfcs.insert(id, rfc);
    }

    pub fn get(&self, id: &str) -> Option<&RfcMeta> {
        self.rfcs.get(id)
    }

    pub fn search(&self, query: &str) -> Vec<&RfcMeta> {
        let query_lower = query.to_lowercase();
        self.rfcs
            .values()
            .filter(|rfc| {
                rfc.id.to_lowercase().contains(&query_lower)
                    || rfc.title.to_lowercase().contains(&query_lower)
                    || rfc.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn by_category(&self, category: &str) -> Vec<&RfcMeta> {
        self.categories
            .get(category)
            .map(|ids| ids.iter().filter_map(|id| self.rfcs.get(id)).collect())
            .unwrap_or_default()
    }

    pub fn by_tag(&self, tag: &str) -> Vec<&RfcMeta> {
        self.tags
            .get(tag)
            .map(|ids| ids.iter().filter_map(|id| self.rfcs.get(id)).collect())
            .unwrap_or_default()
    }
}

pub struct RfcLoader {
    base_path: PathBuf,
}

impl RfcLoader {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    /// Load all RFCs from the base path
    pub fn load_all(&self) -> Result<RfcIndex, RfcError> {
        let mut index = RfcIndex::new();

        // Expected structure:
        // 01-rfc/
        //   9000-core/
        //   9100-integration/
        //   9300-cognitive/
        //   9400-application/
        //   9500-platform/
        //   9800-operational/

        let categories = [
            ("9000-core", "Core"),
            ("9100-integration", "Integration"),
            ("9300-cognitive", "Cognitive"),
            ("9400-application", "Application"),
            ("9500-platform", "Platform"),
            ("9800-operational", "Operational"),
        ];

        for (dir_name, category_name) in categories {
            let category_path = self.base_path.join(dir_name);
            if category_path.exists() {
                self.load_category(&mut index, &category_path, category_name)?;
            }
        }

        Ok(index)
    }

    fn load_category(
        &self,
        index: &mut RfcIndex,
        path: &Path,
        category: &str,
    ) -> Result<(), RfcError> {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |e| e == "md" || e == "yaml") {
                if let Ok(rfc) = self.parse_rfc(&path, category) {
                    index.add(rfc);
                }
            }
        }

        Ok(())
    }

    fn parse_rfc(&self, path: &Path, category: &str) -> Result<RfcMeta, RfcError> {
        let content = std::fs::read_to_string(path)?;
        let filename = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| RfcError::InvalidPath(path.display().to_string()))?;

        // Try to extract RFC ID from filename (e.g., "RFC-9001" or "9001-title")
        let id = if filename.starts_with("RFC-") {
            filename.to_string()
        } else {
            // Extract number prefix
            let num: String = filename.chars().take_while(|c| c.is_ascii_digit()).collect();
            if num.is_empty() {
                filename.to_string()
            } else {
                format!("RFC-{}", num)
            }
        };

        // Try to extract title from first markdown heading or filename
        let title = content
            .lines()
            .find(|line| line.starts_with("# "))
            .map(|line| line.trim_start_matches("# ").to_string())
            .unwrap_or_else(|| filename.to_string());

        // Try to parse YAML frontmatter if present
        let (status, dependencies, implementations, tags) = if content.starts_with("---") {
            self.parse_frontmatter(&content)
        } else {
            (RfcStatus::Active, vec![], vec![], vec![])
        };

        Ok(RfcMeta {
            id,
            title,
            status,
            category: category.to_string(),
            path: path.to_path_buf(),
            dependencies,
            implementations,
            tags,
        })
    }

    fn parse_frontmatter(
        &self,
        content: &str,
    ) -> (RfcStatus, Vec<String>, Vec<String>, Vec<String>) {
        if let Some(end) = content[3..].find("---") {
            let frontmatter = &content[3..3 + end];
            
            #[derive(Deserialize, Default)]
            struct FrontMatter {
                #[serde(default)]
                status: Option<RfcStatus>,
                #[serde(default)]
                dependencies: Vec<String>,
                #[serde(default)]
                implementations: Vec<String>,
                #[serde(default)]
                tags: Vec<String>,
            }

            if let Ok(fm) = serde_yaml::from_str::<FrontMatter>(frontmatter) {
                return (
                    fm.status.unwrap_or(RfcStatus::Active),
                    fm.dependencies,
                    fm.implementations,
                    fm.tags,
                );
            }
        }

        (RfcStatus::Active, vec![], vec![], vec![])
    }

    /// Read full RFC content
    pub fn read_content(&self, rfc_id: &str) -> Result<String, RfcError> {
        // Search for the RFC file
        for entry in walkdir::WalkDir::new(&self.base_path)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                if stem.contains(&rfc_id.replace("RFC-", "")) {
                    return Ok(std::fs::read_to_string(path)?);
                }
            }
        }

        Err(RfcError::NotFound(rfc_id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rfc_index() {
        let mut index = RfcIndex::new();
        index.add(RfcMeta {
            id: "RFC-9001".to_string(),
            title: "Test RFC".to_string(),
            status: RfcStatus::Active,
            category: "Core".to_string(),
            path: PathBuf::from("test.md"),
            dependencies: vec![],
            implementations: vec![],
            tags: vec!["test".to_string()],
        });

        assert!(index.get("RFC-9001").is_some());
        assert_eq!(index.search("test").len(), 1);
    }
}
