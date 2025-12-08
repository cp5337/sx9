/*
// ┌─────────────────────────────────────────────────────────────┐
// │ █████████████████ CTAS USIM HEADER ███████████████████████ │
// ├─────────────────────────────────────────────────────────────┤
// │ 🔖 hash_id      : USIM-UUID-MANAGER-0001                   │
// │ 📁 domain       : UUID Management, File Naming, Context    │
// │ 🧠 description  : CTAS UUID manager for short, contextual  │
// │                  UUIDs that work within filename limits    │
// │ 🕸️ hash_type    : Blake2 → CTAS-shortened UUID             │
// │ 🔄 parent_node  : NODE_DOCUMENT_INTELLIGENCE                │
// │ 🧩 dependencies : uuid, blake2, base64, serde              │
// │ 🔧 tool_usage   : UUID generation, context encoding        │
// │ 📡 input_type   : Document metadata, content hashes        │
// │ 🧪 test_status  : development                               │
// │ 🧠 cognitive_fn : UUID shortening, context preservation    │
// │ ⌛ TTL Policy   : 6.5 Persistent                            │
// └─────────────────────────────────────────────────────────────┘
*/

use anyhow::Result;
use blake2::{Blake2b512, Digest};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use base64::{engine::general_purpose, Engine};
use chrono::{DateTime, Utc};

/// CTAS UUID Manager for creating short, context-aware UUIDs
#[derive(Debug, Clone)]
pub struct CTASUUIDManager {
    /// Registry of UUIDs by context
    uuid_registry: HashMap<String, CTASUUIDEntry>,
    /// Reverse lookup for collision detection
    short_uuid_lookup: HashMap<String, String>,
    /// Statistics
    total_generated: u64,
    collisions_avoided: u64,
}

/// CTAS UUID entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CTASUUIDEntry {
    pub full_uuid: String,
    pub short_uuid: String,
    pub context: UUIDContext,
    pub created_at: DateTime<Utc>,
    pub priority_level: PriorityLevel,
    pub content_hash: String,
    pub file_type: String,
    pub size_bytes: u64,
}

/// Context information for UUID generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UUIDContext {
    pub document_type: DocumentType,
    pub subject_area: String,
    pub complexity_score: f64,
    pub uniqueness_indicators: Vec<String>,
}

/// Document types for context-aware UUID generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentType {
    Architecture,      // ARCH
    Code,             // CODE  
    Documentation,    // DOCS
    Analysis,         // ANLY
    Configuration,    // CONF
    Communication,    // COMM
    Intelligence,     // INTL
    Protocol,         // PROT
    Unknown,          // UNKN
}

/// Priority levels for triage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriorityLevel {
    Critical,  // Immediate attention required
    High,      // Review within 24h
    Medium,    // Review within week
    Low,       // Archive/reference
    Archive,   // Long-term storage
}

impl CTASUUIDManager {
    /// Create new UUID manager
    pub fn new() -> Self {
        Self {
            uuid_registry: HashMap::new(),
            short_uuid_lookup: HashMap::new(),
            total_generated: 0,
            collisions_avoided: 0,
        }
    }

    /// Generate a CTAS UUID with context
    pub fn generate_ctas_uuid(
        &mut self,
        content: &str,
        file_path: Option<&str>,
        context: UUIDContext,
        priority: PriorityLevel,
        file_size: u64,
    ) -> Result<CTASUUIDEntry> {
        // Generate content hash
        let content_hash = self.generate_content_hash(content)?;
        
        // Check if we already have this content
        if let Some(existing) = self.find_by_content_hash(&content_hash) {
            return Ok(existing);
        }

        // Generate base UUID
        let full_uuid = Uuid::new_v4().to_string();
        
        // Create short UUID with context
        let short_uuid = self.create_short_uuid(
            &full_uuid,
            &context,
            &priority,
            file_path.clone(),
        )?;

        // Ensure uniqueness
        let final_short_uuid = self.ensure_unique_short_uuid(short_uuid)?;

        let file_type = file_path.clone()
            .and_then(|p| std::path::Path::new(p).extension())
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_string();

        let entry = CTASUUIDEntry {
            full_uuid: full_uuid,
            short_uuid: final_short_uuid,
            context,
            created_at: Utc::now(),
            priority_level: priority,
            content_hash,
            file_type,
            size_bytes: file_size,
        };

        // Register the UUID
        self.uuid_registry.insert(full_uuid, entry);
        self.short_uuid_lookup.insert(final_short_uuid, full_uuid);
        self.total_generated += 1;

        Ok(entry)
    }

    /// Create a short UUID with context encoding
    fn create_short_uuid(
        &self,
        full_uuid: &str,
        context: &UUIDContext,
        priority: &PriorityLevel,
        file_path: Option<&str>,
    ) -> Result<String> {
        // Create context prefix (4 chars)
        let doc_type_prefix = match context.document_type {
            DocumentType::Architecture => "ARCH",
            DocumentType::Code => "CODE",
            DocumentType::Documentation => "DOCS",
            DocumentType::Analysis => "ANLY",
            DocumentType::Configuration => "CONF",
            DocumentType::Communication => "COMM",
            DocumentType::Intelligence => "INTL",
            DocumentType::Protocol => "PROT",
            DocumentType::Unknown => "UNKN",
        };

        // Priority indicator (1 char)
        let priority_char = match priority {
            PriorityLevel::Critical => "C",
            PriorityLevel::High => "H", 
            PriorityLevel::Medium => "M",
            PriorityLevel::Low => "L",
            PriorityLevel::Archive => "A",
        };

        // Create hash from UUID + context for uniqueness
        let context_data = format!(
            "{}{}{}{}",
            full_uuid,
            doc_type_prefix,
            priority_char,
            context.subject_area
        );

        let mut hasher = Blake2b512::new();
        hasher.update(context_data.as_bytes());
        let hash = hasher.finalize();

        // Take first 6 bytes and encode as base64 (8 chars)
        let short_bytes = &hash[0..6];
        let encoded = general_purpose::URL_SAFE_NO_PAD.encode(short_bytes);
        
        // Create final short UUID: TYPE-P-HASH (max 16 chars for filename safety)
        let short_uuid = format!("{}-{}-{}", doc_type_prefix, priority_char, encoded);

        Ok(short_uuid)
    }

    /// Ensure the short UUID is unique (handle collisions)
    fn ensure_unique_short_uuid(&mut self, mut short_uuid: String) -> Result<String> {
        let mut counter = 0;
        let original = short_uuid;

        while self.short_uuid_lookup.contains_key(&short_uuid) {
            counter += 1;
            short_uuid = format!("{}-{}", original, counter);
            self.collisions_avoided += 1;
            
            // Safety: prevent infinite loops
            if counter > 999 {
                return Err(anyhow::anyhow!("Unable to generate unique short UUID after 999 attempts"));
            }
        }

        Ok(short_uuid)
    }

    /// Generate content hash for deduplication
    fn generate_content_hash(&self, content: &str) -> Result<String> {
        let mut hasher = Blake2b512::new();
        hasher.update(content.as_bytes());
        let hash = hasher.finalize();
        Ok(format!("{:x}", hash))
    }

    /// Find entry by content hash
    fn find_by_content_hash(&self, content_hash: &str) -> Option<&CTASUUIDEntry> {
        self.uuid_registry
            .values()
            .find(|entry| entry.content_hash == content_hash)
    }

    /// Look up entry by short UUID
    pub fn lookup_by_short_uuid(&self, short_uuid: &str) -> Option<&CTASUUIDEntry> {
        self.short_uuid_lookup
            .get(short_uuid)
            .and_then(|full_uuid| self.uuid_registry.get(full_uuid))
    }

    /// Look up entry by full UUID
    pub fn lookup_by_full_uuid(&self, full_uuid: &str) -> Option<&CTASUUIDEntry> {
        self.uuid_registry.get(full_uuid)
    }

    /// Get all entries by document type
    pub fn get_by_document_type(&self, doc_type: &DocumentType) -> Vec<&CTASUUIDEntry> {
        self.uuid_registry
            .values()
            .filter(|entry| std::mem::discriminant(&entry.context.document_type) 
                          == std::mem::discriminant(doc_type))
            .collect()
    }

    /// Get all entries by priority level
    pub fn get_by_priority(&self, priority: &PriorityLevel) -> Vec<&CTASUUIDEntry> {
        self.uuid_registry
            .values()
            .filter(|entry| std::mem::discriminant(&entry.priority_level) 
                          == std::mem::discriminant(priority))
            .collect()
    }

    /// Get statistics
    pub fn get_stats(&self) -> UUIDManagerStats {
        let by_type = self.count_by_document_type();
        let by_priority = self.count_by_priority();

        UUIDManagerStats {
            total_generated: self.total_generated,
            total_active: self.uuid_registry.len() as u64,
            collisions_avoided: self.collisions_avoided,
            by_document_type: by_type,
            by_priority: by_priority,
        }
    }

    /// Count entries by document type
    fn count_by_document_type(&self) -> HashMap<String, u64> {
        let mut counts = HashMap::new();
        
        for entry in self.uuid_registry.values() {
            let type_name = format!("{:?}", entry.context.document_type);
            *counts.entry(type_name).or_insert(0) += 1;
        }
        
        counts
    }

    /// Count entries by priority level
    fn count_by_priority(&self) -> HashMap<String, u64> {
        let mut counts = HashMap::new();
        
        for entry in self.uuid_registry.values() {
            let priority_name = format!("{:?}", entry.priority_level);
            *counts.entry(priority_name).or_insert(0) += 1;
        }
        
        counts
    }

    /// Export registry for persistence
    pub fn export_registry(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.uuid_registry)
            .map_err(|e| anyhow::anyhow!("Failed to serialize registry: {}", e))
    }

    /// Import registry from persistence
    pub fn import_registry(&mut self, registry_json: &str) -> Result<()> {
        let registry: HashMap<String, CTASUUIDEntry> = serde_json::from_str(registry_json)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize registry: {}", e))?;

        // Rebuild lookup tables
        self.short_uuid_lookup.clear();
        for (full_uuid, entry) in &registry {
            self.short_uuid_lookup.insert(entry.short_uuid, full_uuid);
        }

        self.uuid_registry = registry;
        self.total_generated = self.uuid_registry.len() as u64;

        Ok(())
    }
}

/// Statistics for the UUID manager
#[derive(Debug, Serialize)]
pub struct UUIDManagerStats {
    pub total_generated: u64,
    pub total_active: u64,
    pub collisions_avoided: u64,
    pub by_document_type: HashMap<String, u64>,
    pub by_priority: HashMap<String, u64>,
}

impl DocumentType {
    /// Detect document type from file path and content
    pub fn detect(file_path: Option<&str>, content: &str) -> Self {
        // Check file extension first
        if let Some(path) = file_path.clone() {
            if let Some(ext) = std::path::Path::new(path).extension().and_then(|e| e.to_str()) {
                match ext.to_lowercase().as_str() {
                    "rs" | "py" | "js" | "ts" | "go" | "java" | "cpp" | "c" => return DocumentType::Code,
                    "md" | "txt" | "doc" | "docx" => return DocumentType::Documentation,
                    "yaml" | "yml" | "toml" | "json" | "conf" | "ini" => return DocumentType::Configuration,
                    _ => {}
                }
            }
        }

        // Check content patterns
        let content_lower = content.to_lowercase();
        
        if content_lower.contains("architecture") || content_lower.contains("system design") {
            DocumentType::Architecture
        } else if content_lower.contains("protocol") || content_lower.contains("specification") {
            DocumentType::Protocol  
        } else if content_lower.contains("analysis") || content_lower.contains("report") {
            DocumentType::Analysis
        } else if content_lower.contains("intelligence") || content_lower.contains("threat") {
            DocumentType::Intelligence
        } else if content_lower.contains("email") || content_lower.contains("message") {
            DocumentType::Communication
        } else {
            DocumentType::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_generation() {
        let mut manager = CTASUUIDManager::new();
        
        let context = UUIDContext {
            document_type: DocumentType::Architecture,
            subject_area: "CTAS System".to_string(),
            complexity_score: 0.8,
            uniqueness_indicators: vec!["UUID".to_string(), "System".to_string()],
        };

        let entry = manager.generate_ctas_uuid(
            "This is a test document about CTAS architecture",
            Some("test.md"),
            context,
            PriorityLevel::High,
            1024,
        ).unwrap();

        assert!(entry.short_uuid.starts_with("ARCH-H-"));
        assert_eq!(entry.file_type, "md");
        assert_eq!(entry.size_bytes, 1024);
    }

    #[test]
    fn test_collision_handling() {
        let mut manager = CTASUUIDManager::new();
        
        // Force a collision by inserting directly
        manager.short_uuid_lookup.insert("ARCH-H-abc123".to_string(), "test-uuid".to_string());
        
        let unique = manager.ensure_unique_short_uuid("ARCH-H-abc123".to_string()).unwrap();
        assert_eq!(unique, "ARCH-H-abc123-1");
        assert_eq!(manager.collisions_avoided, 1);
    }

    #[test]
    fn test_document_type_detection() {
        assert!(matches!(
            DocumentType::detect(Some("test.rs"), "fn main() {}"),
            DocumentType::Code
        ));

        assert!(matches!(
            DocumentType::detect(None, "This document describes the system architecture"),
            DocumentType::Architecture
        ));
    }
}
