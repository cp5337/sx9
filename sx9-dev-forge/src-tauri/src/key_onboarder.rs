//! Key Onboarding - Auto-import API keys from text files
//! 
//! Drop .txt files in ~/.sx9/keys/ and they auto-import to vault
//! File format: <keyname>.txt containing the API key value
//! 
//! Examples:
//!   ~/.sx9/keys/grok.txt
//!   ~/.sx9/keys/openai.txt
//!   ~/.sx9/keys/google_cloud.txt

use std::path::PathBuf;
use std::fs;
use crate::vault::{global_vault, KeyEntry};
use chrono::Utc;

pub struct KeyOnboarder {
    keys_dir: PathBuf,
}

impl KeyOnboarder {
    /// Create new key onboarder
    pub fn new() -> Self {
        let keys_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".sx9")
            .join("keys");
        
        // Create directory if it doesn't exist
        let _ = fs::create_dir_all(&keys_dir);
        
        Self { keys_dir }
    }
    
    /// Scan for new key files and import them
    pub async fn scan_and_import(&self) -> Result<Vec<String>, String> {
        let mut imported = Vec::new();
        
        // Read all .txt files in keys directory
        let entries = fs::read_dir(&self.keys_dir)
            .map_err(|e| format!("Failed to read keys directory: {}", e))?;
        
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            
            // Only process .txt files
            if path.extension().and_then(|s| s.to_str()) != Some("txt") {
                continue;
            }
            
            // Extract key name from filename
            let key_name = path.file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| "Invalid filename".to_string())?
                .to_string();
            
            // Read key value from file
            let key_value = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?
                .trim()
                .to_string();
            
            // Skip empty files
            if key_value.is_empty() {
                continue;
            }
            
            // Import to vault
            if let Ok(vault) = global_vault() {
                vault.set(&key_name, &key_value, "api").await
                    .map_err(|e| format!("Failed to import {}: {}", key_name, e))?;
                
                imported.push(key_name.clone());
                
                // Archive the file (move to .imported subdirectory)
                let archive_dir = self.keys_dir.join(".imported");
                let _ = fs::create_dir_all(&archive_dir);
                let archive_path = archive_dir.join(path.file_name().unwrap());
                let _ = fs::rename(&path, &archive_path);
            }
        }
        
        Ok(imported)
    }
    
    /// Get the keys directory path for display
    pub fn keys_dir_path(&self) -> String {
        self.keys_dir.display().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keys_dir_creation() {
        let onboarder = KeyOnboarder::new();
        assert!(onboarder.keys_dir.ends_with(".sx9/keys"));
    }
}
