//! Sandbox for agent operations
//!
//! Provides isolation and resource limits for agent code execution.

use std::path::PathBuf;
use std::time::Duration;

/// Sandbox configuration
pub struct Sandbox {
    /// Working directory
    work_dir: PathBuf,

    /// Allowed paths for file operations
    allowed_paths: Vec<PathBuf>,

    /// Maximum execution time
    timeout: Duration,

    /// Maximum memory usage (bytes)
    max_memory: usize,

    /// Enable network access
    network_enabled: bool,
}

impl Default for Sandbox {
    fn default() -> Self {
        Self {
            work_dir: PathBuf::from("."),
            allowed_paths: Vec::new(),
            timeout: Duration::from_secs(300), // 5 minutes
            max_memory: 512 * 1024 * 1024,     // 512MB
            network_enabled: false,
        }
    }
}

impl Sandbox {
    /// Create new sandbox
    pub fn new(work_dir: PathBuf) -> Self {
        Self {
            work_dir,
            ..Default::default()
        }
    }

    /// Add allowed path
    pub fn allow_path(mut self, path: PathBuf) -> Self {
        self.allowed_paths.push(path);
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Enable network access
    pub fn with_network(mut self, enabled: bool) -> Self {
        self.network_enabled = enabled;
        self
    }

    /// Check if path is allowed
    pub fn is_path_allowed(&self, path: &PathBuf) -> bool {
        // Work directory is always allowed
        if path.starts_with(&self.work_dir) {
            return true;
        }

        // Check explicit allow list
        for allowed in &self.allowed_paths {
            if path.starts_with(allowed) {
                return true;
            }
        }

        false
    }

    /// Get working directory
    pub fn work_dir(&self) -> &PathBuf {
        &self.work_dir
    }

    /// Get timeout
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Check network access
    pub fn network_enabled(&self) -> bool {
        self.network_enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_default() {
        let sandbox = Sandbox::default();
        assert!(!sandbox.network_enabled());
        assert_eq!(sandbox.timeout(), Duration::from_secs(300));
    }

    #[test]
    fn test_path_allowed() {
        let sandbox = Sandbox::new(PathBuf::from("/home/user/project"))
            .allow_path(PathBuf::from("/tmp"));

        assert!(sandbox.is_path_allowed(&PathBuf::from("/home/user/project/src")));
        assert!(sandbox.is_path_allowed(&PathBuf::from("/tmp/test")));
        assert!(!sandbox.is_path_allowed(&PathBuf::from("/etc/passwd")));
    }
}
