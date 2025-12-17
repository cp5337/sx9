//! ABE DropZone Secrets Watchdog
//!
//! Monitors ~/Desktop/ABE-DropZone/secrets/ for new .txt and .md files
//! and auto-imports them into the KeyVault.
//!
//! File naming convention:
//! - "Eleven Labs API Key.txt" → key name: "elevenlabs"
//! - "Gemini Key.txt" → key name: "gemini"
//! - "Neon and Supabase Key.txt" → key names: "neon", "supabase"
//!
//! File format (single key):
//! ```
//! Service Name
//!
//! key-name-identifier
//! sk_actual_key_value_here
//! ```

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::keyvault::{KeyEntry, KeyVault, KeyVaultError};

/// Secrets watchdog configuration
#[derive(Debug, Clone)]
pub struct WatchdogConfig {
    /// Path to watch (default: ~/Desktop/ABE-DropZone/secrets)
    pub watch_path: PathBuf,
    /// Debounce duration for file events
    pub debounce: Duration,
    /// Whether to process existing files on startup
    pub process_existing: bool,
    /// File extensions to watch
    pub extensions: Vec<String>,
}

impl Default for WatchdogConfig {
    fn default() -> Self {
        let watch_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Desktop")
            .join("ABE-DropZone")
            .join("secrets");

        Self {
            watch_path,
            debounce: Duration::from_secs(2),
            process_existing: true,
            extensions: vec!["txt".to_string(), "md".to_string()],
        }
    }
}

/// Parsed secret from a file
#[derive(Debug, Clone)]
pub struct ParsedSecret {
    /// Key name (normalized, lowercase, no spaces)
    pub key_name: String,
    /// Service category
    pub service: String,
    /// The actual secret value
    pub value: String,
    /// Source file path
    pub source_file: PathBuf,
    /// Optional notes from file
    pub notes: Option<String>,
}

/// Secrets Watchdog - monitors ABE DropZone and syncs to KeyVault
pub struct SecretsWatchdog {
    config: WatchdogConfig,
    vault: Arc<KeyVault>,
    /// Tracks files we've already processed (by path hash)
    processed_files: HashMap<PathBuf, String>,
}

impl SecretsWatchdog {
    /// Create new watchdog with default config
    pub fn new(vault: Arc<KeyVault>) -> Self {
        Self::with_config(vault, WatchdogConfig::default())
    }

    /// Create watchdog with custom config
    pub fn with_config(vault: Arc<KeyVault>, config: WatchdogConfig) -> Self {
        Self {
            config,
            vault,
            processed_files: HashMap::new(),
        }
    }

    /// Start the watchdog (blocking)
    pub async fn start(&mut self) -> Result<(), WatchdogError> {
        info!(
            "🐕 Secrets Watchdog starting - watching: {:?}",
            self.config.watch_path
        );

        // Process existing files first
        if self.config.process_existing {
            self.process_existing_files().await?;
        }

        // Set up file watcher
        let (tx, mut rx) = mpsc::channel::<PathBuf>(100);

        let watch_path = self.config.watch_path.clone();
        let extensions = self.config.extensions.clone();

        // Spawn blocking watcher thread
        std::thread::spawn(move || {
            let rt = tokio::runtime::Handle::current();
            let tx_clone = tx.clone();

            let mut watcher = RecommendedWatcher::new(
                move |res: Result<Event, notify::Error>| {
                    if let Ok(event) = res {
                        match event.kind {
                            EventKind::Create(_) | EventKind::Modify(_) => {
                                for path in event.paths {
                                    if let Some(ext) = path.extension() {
                                        let ext_str = ext.to_string_lossy().to_lowercase();
                                        if extensions.contains(&ext_str) {
                                            let tx = tx_clone.clone();
                                            rt.spawn(async move {
                                                let _ = tx.send(path).await;
                                            });
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                },
                Config::default(),
            )
            .expect("Failed to create watcher");

            watcher
                .watch(&watch_path, RecursiveMode::NonRecursive)
                .expect("Failed to watch path");

            // Keep thread alive
            loop {
                std::thread::sleep(Duration::from_secs(60));
            }
        });

        // Process incoming file events
        info!("🐕 Watchdog active - waiting for secrets...");

        while let Some(path) = rx.recv().await {
            // Debounce
            tokio::time::sleep(self.config.debounce).await;

            if let Err(e) = self.process_file(&path).await {
                error!("Failed to process secret file {:?}: {}", path, e);
            }
        }

        Ok(())
    }

    /// Process all existing files in the watch directory
    async fn process_existing_files(&mut self) -> Result<(), WatchdogError> {
        if !self.config.watch_path.exists() {
            warn!("Watch path does not exist: {:?}", self.config.watch_path);
            return Ok(());
        }

        let entries = std::fs::read_dir(&self.config.watch_path)
            .map_err(|e| WatchdogError::Io(e.to_string()))?;

        let mut count = 0;
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if self.config.extensions.contains(&ext_str) {
                    if let Err(e) = self.process_file(&path).await {
                        warn!("Failed to process {:?}: {}", path, e);
                    } else {
                        count += 1;
                    }
                }
            }
        }

        info!("🐕 Processed {} existing secret files", count);
        Ok(())
    }

    /// Process a single secret file
    async fn process_file(&mut self, path: &Path) -> Result<(), WatchdogError> {
        // Read file content
        let content =
            std::fs::read_to_string(path).map_err(|e| WatchdogError::Io(e.to_string()))?;

        // Check if content changed
        let content_hash = format!("{:x}", md5::compute(&content));
        if let Some(prev_hash) = self.processed_files.get(path) {
            if prev_hash == &content_hash {
                debug!("File unchanged, skipping: {:?}", path);
                return Ok(());
            }
        }

        // Parse secrets from file
        let secrets = self.parse_secret_file(path, &content)?;

        // Store each secret in vault
        for secret in secrets {
            info!(
                "🔐 Importing secret: {} (from {:?})",
                secret.key_name,
                secret.source_file.file_name()
            );

            let entry = KeyEntry {
                name: secret.key_name.clone(),
                value: secret.value,
                service: secret.service,
                created_at: chrono::Utc::now(),
                last_used: None,
                usage_count: 0,
                active: true,
                expires_at: None,
                notes: secret.notes,
            };

            self.vault
                .store_entry(entry)
                .await
                .map_err(|e| WatchdogError::Vault(e))?;
        }

        // Mark as processed
        self.processed_files
            .insert(path.to_path_buf(), content_hash);

        Ok(())
    }

    /// Parse a secret file into one or more secrets
    fn parse_secret_file(
        &self,
        path: &Path,
        content: &str,
    ) -> Result<Vec<ParsedSecret>, WatchdogError> {
        let mut secrets = Vec::new();
        let file_name = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        // Derive service name from filename
        let service = self.normalize_service_name(&file_name);

        // Parse content - look for key patterns
        let lines: Vec<&str> = content.lines().collect();

        // Strategy 1: Last non-empty line is the key value
        // (Common format: Title\n\nidentifier\nkey_value)
        let mut key_value = None;
        let mut key_identifier = None;

        for (i, line) in lines.iter().enumerate().rev() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                if key_value.is_none() {
                    // Last non-empty line is likely the key
                    if Self::looks_like_api_key(trimmed) {
                        key_value = Some(trimmed.to_string());
                    }
                } else if key_identifier.is_none() {
                    // Second-to-last non-empty might be identifier
                    key_identifier = Some(trimmed.to_string());
                    break;
                }
            }
        }

        if let Some(value) = key_value {
            let key_name = self.derive_key_name(&file_name, key_identifier.as_deref());

            // Handle files with multiple services (e.g., "Neon and Supabase Key.txt")
            if file_name.to_lowercase().contains(" and ") {
                // Split into multiple keys with same value
                let parts: Vec<&str> = file_name.split(" and ").collect();
                for part in parts {
                    let name = self.normalize_key_name(part);
                    if !name.is_empty() && name != "key" {
                        secrets.push(ParsedSecret {
                            key_name: name.clone(),
                            service: name,
                            value: value.clone(),
                            source_file: path.to_path_buf(),
                            notes: Some(format!(
                                "Auto-imported from ABE DropZone: {:?}",
                                path.file_name()
                            )),
                        });
                    }
                }
            } else {
                secrets.push(ParsedSecret {
                    key_name: key_name.clone(),
                    service,
                    value,
                    source_file: path.to_path_buf(),
                    notes: Some(format!(
                        "Auto-imported from ABE DropZone: {:?}",
                        path.file_name()
                    )),
                });
            }
        }

        if secrets.is_empty() {
            warn!("Could not parse any secrets from {:?}", path);
        }

        Ok(secrets)
    }

    /// Check if a string looks like an API key
    fn looks_like_api_key(s: &str) -> bool {
        // Common patterns:
        // - sk_xxx (Stripe, ElevenLabs)
        // - AIza... (Google)
        // - ghp_xxx (GitHub)
        // - Long alphanumeric strings
        let s = s.trim();

        // Must be at least 20 chars
        if s.len() < 20 {
            return false;
        }

        // Common prefixes
        let prefixes = ["sk_", "pk_", "AIza", "ghp_", "gho_", "xoxb-", "xoxp-"];
        for prefix in prefixes {
            if s.starts_with(prefix) {
                return true;
            }
        }

        // Long alphanumeric string (possibly with - or _)
        let alnum_count = s.chars().filter(|c| c.is_alphanumeric()).count();
        alnum_count >= 20 && (alnum_count as f64 / s.len() as f64) > 0.8
    }

    /// Normalize service name from filename
    fn normalize_service_name(&self, filename: &str) -> String {
        let lower = filename.to_lowercase();

        // Known service mappings
        let mappings = [
            ("eleven labs", "voice"),
            ("elevenlabs", "voice"),
            ("gemini", "ai"),
            ("openai", "ai"),
            ("anthropic", "ai"),
            ("neon", "database"),
            ("supabase", "database"),
            ("github", "vcs"),
            ("linear", "project"),
            ("mapbox", "geo"),
        ];

        for (pattern, service) in mappings {
            if lower.contains(pattern) {
                return service.to_string();
            }
        }

        "api".to_string()
    }

    /// Derive key name from filename and optional identifier
    fn derive_key_name(&self, filename: &str, identifier: Option<&str>) -> String {
        // Try identifier first
        if let Some(id) = identifier {
            let normalized = self.normalize_key_name(id);
            if !normalized.is_empty() && normalized != "key" {
                return normalized;
            }
        }

        // Fall back to filename
        self.normalize_key_name(filename)
    }

    /// Normalize a key name (lowercase, no spaces, remove common words)
    fn normalize_key_name(&self, name: &str) -> String {
        let lower = name.to_lowercase();

        // Remove common suffixes/words
        let cleaned = lower
            .replace(" api key", "")
            .replace(" key", "")
            .replace("api ", "")
            .replace(" ", "_")
            .replace("-", "_");

        // Known normalizations
        match cleaned.as_str() {
            "eleven_labs" | "elevenlabs" | "eleven" => "elevenlabs".to_string(),
            s => s.to_string(),
        }
    }
}

/// Watchdog errors
#[derive(Debug, thiserror::Error)]
pub enum WatchdogError {
    #[error("I/O error: {0}")]
    Io(String),
    #[error("Vault error: {0}")]
    Vault(#[from] KeyVaultError),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Watch error: {0}")]
    Watch(String),
}

/// One-shot import: scan ABE DropZone and import all secrets
pub async fn import_dropzone_secrets(vault: Arc<KeyVault>) -> Result<usize, WatchdogError> {
    let mut watchdog = SecretsWatchdog::new(vault);
    watchdog.config.process_existing = true;

    watchdog.process_existing_files().await?;

    Ok(watchdog.processed_files.len())
}

/// Get the default ABE DropZone secrets path
pub fn default_dropzone_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Desktop")
        .join("ABE-DropZone")
        .join("secrets")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looks_like_api_key() {
        assert!(SecretsWatchdog::looks_like_api_key(
            "sk_76f303defe5dde260260781c9896827c1740cb3538264ff5"
        ));
        assert!(SecretsWatchdog::looks_like_api_key(
            "AIzaSyBxxxxxxxxxxxxxxxxxxxxxxx"
        ));
        assert!(!SecretsWatchdog::looks_like_api_key("short"));
        assert!(!SecretsWatchdog::looks_like_api_key("Eleven Labs API Key"));
    }

    #[test]
    fn test_normalize_key_name() {
        let vault = Arc::new(KeyVault::new().unwrap());
        let watchdog = SecretsWatchdog::new(vault);

        assert_eq!(
            watchdog.normalize_key_name("Eleven Labs API Key"),
            "elevenlabs"
        );
        assert_eq!(watchdog.normalize_key_name("Gemini Key"), "gemini");
        assert_eq!(watchdog.normalize_key_name("Neon"), "neon");
    }
}
