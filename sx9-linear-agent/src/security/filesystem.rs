//! Filesystem Guard
//!
//! Validates and restricts filesystem operations.

use std::path::{Path, PathBuf};
use anyhow::{Result, anyhow};

/// Filesystem operation guard
pub struct FileSystemGuard {
    /// Root directory for operations
    root: PathBuf,

    /// Denied patterns
    denied_patterns: Vec<String>,

    /// Maximum file size (bytes)
    max_file_size: usize,
}

impl FileSystemGuard {
    /// Create new filesystem guard
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            denied_patterns: vec![
                ".env".to_string(),
                ".git/config".to_string(),
                "id_rsa".to_string(),
                "credentials".to_string(),
                "secrets".to_string(),
            ],
            max_file_size: 10 * 1024 * 1024, // 10MB
        }
    }

    /// Add denied pattern
    pub fn deny_pattern(mut self, pattern: &str) -> Self {
        self.denied_patterns.push(pattern.to_string());
        self
    }

    /// Set max file size
    pub fn with_max_size(mut self, size: usize) -> Self {
        self.max_file_size = size;
        self
    }

    /// Validate path for read
    pub fn validate_read(&self, path: &Path) -> Result<PathBuf> {
        let canonical = self.canonicalize(path)?;

        // Check if within root
        if !canonical.starts_with(&self.root) {
            return Err(anyhow!("Path outside root: {:?}", path));
        }

        // Check denied patterns
        self.check_denied(&canonical)?;

        Ok(canonical)
    }

    /// Validate path for write
    pub fn validate_write(&self, path: &Path, content: &[u8]) -> Result<PathBuf> {
        let canonical = self.canonicalize(path)?;

        // Check if within root
        if !canonical.starts_with(&self.root) {
            return Err(anyhow!("Path outside root: {:?}", path));
        }

        // Check denied patterns
        self.check_denied(&canonical)?;

        // Check file size
        if content.len() > self.max_file_size {
            return Err(anyhow!(
                "File size {} exceeds limit {}",
                content.len(),
                self.max_file_size
            ));
        }

        Ok(canonical)
    }

    /// Canonicalize path, handling non-existent paths
    fn canonicalize(&self, path: &Path) -> Result<PathBuf> {
        if path.is_absolute() {
            Ok(path.to_path_buf())
        } else {
            Ok(self.root.join(path))
        }
    }

    /// Check against denied patterns
    fn check_denied(&self, path: &Path) -> Result<()> {
        let path_str = path.to_string_lossy();

        for pattern in &self.denied_patterns {
            if path_str.contains(pattern) {
                return Err(anyhow!("Path matches denied pattern: {}", pattern));
            }
        }

        Ok(())
    }

    /// Get root directory
    pub fn root(&self) -> &PathBuf {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_read() {
        let guard = FileSystemGuard::new(PathBuf::from("/home/user/project"));

        // Valid path
        assert!(guard.validate_read(Path::new("src/main.rs")).is_ok());

        // Denied pattern
        assert!(guard.validate_read(Path::new(".env")).is_err());
    }

    #[test]
    fn test_validate_write_size() {
        let guard = FileSystemGuard::new(PathBuf::from("/home/user/project"))
            .with_max_size(100);

        let small_content = b"hello";
        let large_content = vec![0u8; 200];

        assert!(guard.validate_write(Path::new("test.txt"), small_content).is_ok());
        assert!(guard.validate_write(Path::new("test.txt"), &large_content).is_err());
    }
}
