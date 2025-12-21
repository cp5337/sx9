//! KeyVault re-export from sx9-foundation-core
//! 
//! Uses the canonical KeyVault implementation instead of custom code

pub use sx9_foundation_core::keyvault::{
    KeyVault,
    KeyEntry,
    KeyEntrySummary,
    KeyVaultError,
};

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// Re-export keys module from foundation-core
pub mod keys {
    pub use sx9_foundation_core::keyvault::keys::*;
    
    // Custom addition: ALL keys array
    pub const ALL: &[&str] = &[
        MAPBOX, ELEVENLABS, OPENAI, ANTHROPIC, LINEAR, GITHUB,
        SUPABASE, NATS, AWS_ACCESS, AWS_SECRET, GOOGLE, 
        CLOUDFLARE, GEMINI, GROK
    ];
}

/// Vault statistics (custom addition for dev-forge)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultStats {
    pub total: usize,
    pub active: usize,
    pub inactive: usize,
    pub with_value: usize,
    pub vault_path: String,
}

static GLOBAL_VAULT: OnceLock<KeyVault> = OnceLock::new();

/// Get or initialize the global KeyVault instance
pub fn global_vault() -> Result<&'static KeyVault, KeyVaultError> {
    if let Some(vault) = GLOBAL_VAULT.get() {
        return Ok(vault);
    }
    
    eprintln!("🔐 Initializing KeyVault from foundation-core...");
    let vault = KeyVault::new()?;
    eprintln!("✅ KeyVault initialized successfully");
    
    match GLOBAL_VAULT.set(vault) {
        Ok(_) => Ok(GLOBAL_VAULT.get().unwrap()),
        Err(_) => Ok(GLOBAL_VAULT.get().unwrap()), // Another thread initialized it
    }
}

/// Extension trait to add stats() method to KeyVault
pub trait KeyVaultExt {
    async fn stats(&self) -> VaultStats;
}

impl KeyVaultExt for KeyVault {
    async fn stats(&self) -> VaultStats {
        let entries = self.list_entries().await;
        let active_count = entries.iter().filter(|e| e.active).count();
        
        VaultStats {
            total: entries.len(),
            active: active_count,
            inactive: entries.len() - active_count,
            with_value: entries.len(), // All entries have values in foundation-core
            vault_path: KeyVault::default_vault_dir().display().to_string(),
        }
    }
}
