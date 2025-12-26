//! Document Index
//!
//! Indexes workspace documents for semantic navigation.

use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

/// Document index for semantic information
pub struct DocumentIndex {
    /// Indexed symbols by file
    symbols: RwLock<HashMap<PathBuf, Vec<DocumentSymbol>>>,

    /// Agent definitions found
    agents: RwLock<HashMap<String, Location>>,

    /// Skill usages found
    skill_usages: RwLock<HashMap<String, Vec<Location>>>,
}

impl DocumentIndex {
    /// Create new document index
    pub fn new() -> Self {
        Self {
            symbols: RwLock::new(HashMap::new()),
            agents: RwLock::new(HashMap::new()),
            skill_usages: RwLock::new(HashMap::new()),
        }
    }

    /// Index a workspace directory
    pub async fn index_workspace(&self, root: &Path) {
        tracing::info!("Indexing workspace: {:?}", root);

        // Find all Rust files
        let pattern = root.join("**/*.rs");
        if let Ok(entries) = glob::glob(pattern.to_string_lossy().as_ref()) {
            for entry in entries.flatten() {
                self.index_file(&entry).await;
            }
        }

        // Find all TypeScript files
        let pattern = root.join("**/*.ts");
        if let Ok(entries) = glob::glob(pattern.to_string_lossy().as_ref()) {
            for entry in entries.flatten() {
                self.index_file(&entry).await;
            }
        }

        tracing::info!("Workspace indexing complete");
    }

    /// Index a single file
    pub async fn index_file(&self, path: &Path) {
        let Ok(content) = tokio::fs::read_to_string(path).await else {
            return;
        };

        let uri = Url::from_file_path(path).ok();

        // Extract symbols
        let symbols = self.extract_symbols(&content, path);
        if let Ok(mut map) = self.symbols.write() {
            map.insert(path.to_path_buf(), symbols);
        }

        // Find agent definitions
        self.find_agent_definitions(&content, &uri);

        // Find skill usages
        self.find_skill_usages(&content, &uri);
    }

    /// Extract symbols from content
    fn extract_symbols(&self, content: &str, _path: &Path) -> Vec<DocumentSymbol> {
        let mut symbols = Vec::new();

        // Find struct definitions
        let struct_regex = regex::Regex::new(r"(?m)^\s*(pub\s+)?struct\s+(\w+)").unwrap();
        for (line_num, line) in content.lines().enumerate() {
            if let Some(caps) = struct_regex.captures(line) {
                if let Some(name) = caps.get(2) {
                    symbols.push(DocumentSymbol {
                        name: name.as_str().to_string(),
                        detail: None,
                        kind: SymbolKind::STRUCT,
                        tags: None,
                        deprecated: None,
                        range: Range {
                            start: Position {
                                line: line_num as u32,
                                character: 0,
                            },
                            end: Position {
                                line: line_num as u32,
                                character: line.len() as u32,
                            },
                        },
                        selection_range: Range {
                            start: Position {
                                line: line_num as u32,
                                character: name.start() as u32,
                            },
                            end: Position {
                                line: line_num as u32,
                                character: name.end() as u32,
                            },
                        },
                        children: None,
                    });
                }
            }
        }

        symbols
    }

    /// Find agent definitions in code
    fn find_agent_definitions(&self, content: &str, uri: &Option<Url>) {
        let Some(uri) = uri else { return };

        // Look for Agent struct instantiation
        let agent_regex = regex::Regex::new(r#"Agent\s*\{[^}]*name:\s*["'](\w+)["']"#).unwrap();

        for (line_num, line) in content.lines().enumerate() {
            if let Some(caps) = agent_regex.captures(line) {
                if let Some(name) = caps.get(1) {
                    if let Ok(mut agents) = self.agents.write() {
                        agents.insert(
                            name.as_str().to_string(),
                            Location {
                                uri: uri.clone(),
                                range: Range {
                                    start: Position {
                                        line: line_num as u32,
                                        character: 0,
                                    },
                                    end: Position {
                                        line: line_num as u32,
                                        character: line.len() as u32,
                                    },
                                },
                            },
                        );
                    }
                }
            }
        }
    }

    /// Find skill usages in code
    fn find_skill_usages(&self, content: &str, uri: &Option<Url>) {
        let Some(uri) = uri else { return };

        // Look for skill ID patterns
        let skill_regex = regex::Regex::new(r#"["']([a-z]+\.[a-z_]+)["']"#).unwrap();

        for (line_num, line) in content.lines().enumerate() {
            for caps in skill_regex.captures_iter(line) {
                if let Some(skill_id) = caps.get(1) {
                    let id = skill_id.as_str().to_string();
                    let location = Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position {
                                line: line_num as u32,
                                character: skill_id.start() as u32,
                            },
                            end: Position {
                                line: line_num as u32,
                                character: skill_id.end() as u32,
                            },
                        },
                    };

                    if let Ok(mut usages) = self.skill_usages.write() {
                        usages.entry(id).or_default().push(location);
                    }
                }
            }
        }
    }

    /// Get agent definition location
    pub fn get_agent_location(&self, name: &str) -> Option<Location> {
        self.agents.read().ok()?.get(name).cloned()
    }

    /// Get skill usage locations
    pub fn get_skill_usages(&self, skill_id: &str) -> Vec<Location> {
        self.skill_usages
            .read()
            .ok()
            .and_then(|u| u.get(skill_id).cloned())
            .unwrap_or_default()
    }
}

impl Default for DocumentIndex {
    fn default() -> Self {
        Self::new()
    }
}
