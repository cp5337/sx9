//! ingestion.rs
//! Core document ingestion engine for RFC-9105/9011-A compliance.
//! Reads documents, calculates Trivariate hashes, and extracts metadata.

use crate::types::{ProcessedDocument, DocumentRecord};
use crate::trivariate_hash_v731::TrivariateHashEngineV731;
use anyhow::{Result, Context};
use std::path::Path;
use tokio::fs;
use chrono::Utc;
use std::collections::HashMap;

pub struct DocumentIngestionEngine {
    hasher: TrivariateHashEngineV731,
}

impl DocumentIngestionEngine {
    pub fn new() -> Self {
        Self {
            hasher: TrivariateHashEngineV731::new(),
        }
    }
    
    /// Ingest a single document file
    pub async fn ingest_document(&self, path: &Path) -> Result<ProcessedDocument> {
        // Read file content
        let content = fs::read(path).await
            .with_context(|| format!("Failed to read file: {:?}", path))?;
            
        // Generate Trivariate Hash (Content-Addressing)
        let hash = self.hasher.generate_hash_from_bytes(&content);
        
        // Extract basic metadata
        let mut metadata = HashMap::new();
        if let Some(ext) = path.extension() {
            metadata.insert("extension".to_string(), ext.to_string_lossy().to_string());
        }
        if let Some(name) = path.file_name() {
             metadata.insert("filename".to_string(), name.to_string_lossy().to_string());
        }
        
        let processed = ProcessedDocument {
            id: hash,
            path: path.to_string_lossy().to_string(),
            content_type: Self::detect_content_type(path),
            size_bytes: content.len() as u64,
            created_at: Utc::now(),
            metadata,
        };
        
        Ok(processed)
    }
    
    /// Ingest a directory recursively (Stub for now)
    pub async fn ingest_directory(&self, path: &Path) -> Result<Vec<DocumentRecord>> {
        let mut records = Vec::new();
        // Simple implementation: just read dir and process files
        // This is a placeholder for recursive logic
        if path.is_dir() {
            let mut entries = fs::read_dir(path).await?;
            while let Some(entry) = entries.next_entry().await? {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    match self.ingest_document(&entry_path).await {
                        Ok(doc) => records.push(DocumentRecord { id: doc.id, status: "Success".to_string() }),
                        Err(e) => records.push(DocumentRecord { id: "Error".to_string(), status: e.to_string() }),
                    }
                }
            }
        }
        Ok(records)
    }
    
    fn detect_content_type(path: &Path) -> String {
        match path.extension().and_then(|s| s.to_str()) {
            Some("md") => "text/markdown",
            Some("rs") => "text/rust",
            Some("txt") => "text/plain",
            Some("json") => "application/json",
            Some("toml") => "application/toml",
            _ => "application/octet-stream",
        }.to_string()
    }
}