//! LSP Backend Implementation
//!
//! Core language server that handles all LSP requests.

use dashmap::DashMap;
use ropey::Rope;
use std::sync::Arc;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use sx9_harness::{SkillRegistry, Skill};

use crate::capabilities::server_capabilities;
use crate::completion::complete;
use crate::diagnostics::diagnose;
use crate::hover::hover_info;
use crate::document::DocumentIndex;
use crate::rfc::RfcIndex;

/// SX9 Language Server Backend
pub struct Sx9Backend {
    /// LSP client for sending notifications
    client: Client,

    /// Open documents
    documents: DashMap<Url, Rope>,

    /// Document index for semantic information
    document_index: Arc<DocumentIndex>,

    /// RFC index for documentation
    rfc_index: Arc<RfcIndex>,

    /// Skill registry for completions
    skill_registry: Arc<SkillRegistry>,

    /// Workspace root
    workspace_root: tokio::sync::RwLock<Option<Url>>,
}

impl Sx9Backend {
    /// Create new backend with client
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: DashMap::new(),
            document_index: Arc::new(DocumentIndex::new()),
            rfc_index: Arc::new(RfcIndex::new()),
            skill_registry: Arc::new(SkillRegistry::with_builtin_skills()),
            workspace_root: tokio::sync::RwLock::new(None),
        }
    }

    /// Get document content
    pub fn get_document(&self, uri: &Url) -> Option<Rope> {
        self.documents.get(uri).map(|r| r.clone())
    }

    /// Update document and trigger diagnostics
    async fn update_document(&self, uri: Url, text: String) {
        let rope = Rope::from_str(&text);
        self.documents.insert(uri.clone(), rope);

        // Run diagnostics
        let diagnostics = diagnose(&text, &self.skill_registry);
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Sx9Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        tracing::info!("Initializing SX9 LSP");

        // Store workspace root
        if let Some(root) = params.root_uri {
            *self.workspace_root.write().await = Some(root.clone());
            tracing::info!("Workspace root: {}", root);
        }

        Ok(InitializeResult {
            capabilities: server_capabilities(),
            server_info: Some(ServerInfo {
                name: "sx9-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        tracing::info!("SX9 LSP initialized");

        // Index workspace
        if let Some(root) = self.workspace_root.read().await.clone() {
            if let Ok(path) = root.to_file_path() {
                self.rfc_index.index_directory(&path).await;
                self.document_index.index_workspace(&path).await;
            }
        }

        self.client
            .log_message(MessageType::INFO, "SX9 Language Server ready")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        tracing::info!("Shutting down SX9 LSP");
        Ok(())
    }

    // ========================================================================
    // TEXT SYNCHRONIZATION
    // ========================================================================

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        tracing::debug!("Document opened: {}", params.text_document.uri);
        self.update_document(params.text_document.uri, params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().last() {
            self.update_document(params.text_document.uri, change.text)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        tracing::debug!("Document closed: {}", params.text_document.uri);
        self.documents.remove(&params.text_document.uri);
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        tracing::debug!("Document saved: {}", params.text_document.uri);

        // Re-index on save
        if let Some(text) = params.text {
            self.update_document(params.text_document.uri, text).await;
        }
    }

    // ========================================================================
    // COMPLETION
    // ========================================================================

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let Some(document) = self.get_document(&uri) else {
            return Ok(None);
        };

        let items = complete(
            &document,
            position,
            &self.skill_registry,
            &self.rfc_index,
            &self.document_index,
        );

        if items.is_empty() {
            Ok(None)
        } else {
            Ok(Some(CompletionResponse::Array(items)))
        }
    }

    // ========================================================================
    // HOVER
    // ========================================================================

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let Some(document) = self.get_document(&uri) else {
            return Ok(None);
        };

        Ok(hover_info(
            &document,
            position,
            &self.skill_registry,
            &self.rfc_index,
        ))
    }

    // ========================================================================
    // GO TO DEFINITION
    // ========================================================================

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let Some(document) = self.get_document(&uri) else {
            return Ok(None);
        };

        // Get word at position
        let line_idx = position.line as usize;
        let char_idx = position.character as usize;

        if let Some(line) = document.get_line(line_idx) {
            let line_str = line.to_string();
            let word = extract_word_at(&line_str, char_idx);

            // Check if it's an RFC reference
            if word.starts_with("RFC-") || word.starts_with("rfc-") {
                if let Some(location) = self.rfc_index.find_rfc(&word) {
                    return Ok(Some(GotoDefinitionResponse::Scalar(location)));
                }
            }

            // Check if it's a skill reference
            if let Some(skill) = self.skill_registry.get(&word) {
                // Return location in skill registry (virtual)
                return Ok(None); // TODO: Implement skill file locations
            }
        }

        Ok(None)
    }

    // ========================================================================
    // DOCUMENT SYMBOLS
    // ========================================================================

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;

        let Some(document) = self.get_document(&uri) else {
            return Ok(None);
        };

        let symbols = extract_document_symbols(&document, &uri);
        Ok(Some(DocumentSymbolResponse::Flat(symbols)))
    }

    // ========================================================================
    // WORKSPACE SYMBOLS
    // ========================================================================

    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        let query = params.query.to_lowercase();

        // Search skills
        let skill_symbols: Vec<_> = self
            .skill_registry
            .search(&query)
            .iter()
            .map(|skill| SymbolInformation {
                name: skill.name.clone(),
                kind: SymbolKind::FUNCTION,
                tags: None,
                deprecated: None,
                location: Location {
                    uri: Url::parse("sx9://skills").unwrap(),
                    range: Range::default(),
                },
                container_name: Some(format!("{:?}", skill.category)),
            })
            .collect();

        // Search RFCs
        let rfc_symbols: Vec<_> = self
            .rfc_index
            .search(&query)
            .iter()
            .map(|(name, location)| SymbolInformation {
                name: name.clone(),
                kind: SymbolKind::FILE,
                tags: None,
                deprecated: None,
                location: location.clone(),
                container_name: Some("RFC".to_string()),
            })
            .collect();

        let mut all_symbols = skill_symbols;
        all_symbols.extend(rfc_symbols);

        if all_symbols.is_empty() {
            Ok(None)
        } else {
            Ok(Some(all_symbols))
        }
    }

    // ========================================================================
    // CODE ACTIONS
    // ========================================================================

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri;

        let Some(document) = self.get_document(&uri) else {
            return Ok(None);
        };

        let mut actions = Vec::new();

        // Check diagnostics at this location
        for diagnostic in &params.context.diagnostics {
            if diagnostic.message.contains("N-V-N-N") {
                // Offer to fix N-V-N-N annotation
                actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                    title: "Generate N-V-N-N annotation".to_string(),
                    kind: Some(CodeActionKind::QUICKFIX),
                    diagnostics: Some(vec![diagnostic.clone()]),
                    edit: None, // TODO: Implement edit
                    command: None,
                    is_preferred: Some(true),
                    disabled: None,
                    data: None,
                }));
            }
        }

        if actions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(actions))
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Extract word at character position
fn extract_word_at(line: &str, char_idx: usize) -> String {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();

    if char_idx >= len {
        return String::new();
    }

    // Find word boundaries
    let mut start = char_idx;
    let mut end = char_idx;

    while start > 0 && is_word_char(chars[start - 1]) {
        start -= 1;
    }

    while end < len && is_word_char(chars[end]) {
        end += 1;
    }

    chars[start..end].iter().collect()
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-' || c == '.'
}

/// Extract document symbols from content
fn extract_document_symbols(document: &Rope, uri: &Url) -> Vec<SymbolInformation> {
    let mut symbols = Vec::new();
    let text = document.to_string();

    // Find function definitions (Rust)
    let fn_regex = regex::Regex::new(r"(?m)^\s*(pub\s+)?(async\s+)?fn\s+(\w+)").unwrap();
    for cap in fn_regex.captures_iter(&text) {
        if let Some(name) = cap.get(3) {
            let line = text[..cap.get(0).unwrap().start()]
                .chars()
                .filter(|&c| c == '\n')
                .count();

            symbols.push(SymbolInformation {
                name: name.as_str().to_string(),
                kind: SymbolKind::FUNCTION,
                tags: None,
                deprecated: None,
                location: Location {
                    uri: uri.clone(),
                    range: Range {
                        start: Position {
                            line: line as u32,
                            character: 0,
                        },
                        end: Position {
                            line: line as u32,
                            character: 0,
                        },
                    },
                },
                container_name: None,
            });
        }
    }

    // Find struct definitions
    let struct_regex = regex::Regex::new(r"(?m)^\s*(pub\s+)?struct\s+(\w+)").unwrap();
    for cap in struct_regex.captures_iter(&text) {
        if let Some(name) = cap.get(2) {
            let line = text[..cap.get(0).unwrap().start()]
                .chars()
                .filter(|&c| c == '\n')
                .count();

            symbols.push(SymbolInformation {
                name: name.as_str().to_string(),
                kind: SymbolKind::STRUCT,
                tags: None,
                deprecated: None,
                location: Location {
                    uri: uri.clone(),
                    range: Range {
                        start: Position {
                            line: line as u32,
                            character: 0,
                        },
                        end: Position {
                            line: line as u32,
                            character: 0,
                        },
                    },
                },
                container_name: None,
            });
        }
    }

    // Find N-V-N-N annotations
    let nvnn_regex = regex::Regex::new(r"//\s*([A-Z_]+)_([A-Z_]+)_([A-Z_]+)_([A-Z_]+)").unwrap();
    for cap in nvnn_regex.captures_iter(&text) {
        let annotation = cap.get(0).unwrap().as_str();
        let line = text[..cap.get(0).unwrap().start()]
            .chars()
            .filter(|&c| c == '\n')
            .count();

        symbols.push(SymbolInformation {
            name: annotation.to_string(),
            kind: SymbolKind::CONSTANT,
            tags: None,
            deprecated: None,
            location: Location {
                uri: uri.clone(),
                range: Range {
                    start: Position {
                        line: line as u32,
                        character: 0,
                    },
                    end: Position {
                        line: line as u32,
                        character: 0,
                    },
                },
            },
            container_name: Some("N-V-N-N".to_string()),
        });
    }

    symbols
}
