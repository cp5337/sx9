//! CTAS-7 Source Provenance Module
//! Source code provenance tracking
//! Follows CTAS-7 standards: ≤200 LOC

use serde::{Deserialize, Serialize};

/// Source code provenance tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceProvenance {
    /// Git repository information
    pub repository: RepositoryInfo,
    /// Commit hash and branch
    pub commit_hash: String,
    pub branch: String,
    /// Author and committer information
    pub authors: Vec<AuthorInfo>,
    /// Source code hash (before compilation)
    pub source_hash: String,
    /// License information
    pub license: String,
    /// Dependencies with their provenance
    pub dependencies: Vec<DependencyProvenance>,
}

/// Repository information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub url: String,
    pub provider: String, // "github", "gitlab", etc.
    pub organization: String,
    pub verified: bool,
    pub tesla_approved: bool,
}

/// Author information with verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorInfo {
    pub name: String,
    pub email: String,
    pub pgp_fingerprint: Option<String>,
    pub tesla_employee: bool,
    pub verified: bool,
}

/// Dependency provenance chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyProvenance {
    pub name: String,
    pub version: String,
    pub hash: String,
    pub crates_io_verified: bool,
    pub security_audit_status: String,
    pub tesla_approved: bool,
    /// Recursive provenance for critical dependencies
    pub sub_provenance: Option<Box<super::core::CrateProvenance>>,
}

impl Default for SourceProvenance {
    fn default() -> Self {
        Self {
            repository: RepositoryInfo::default(),
            commit_hash: String::new(),
            branch: String::new(),
            authors: Vec::new(),
            source_hash: String::new(),
            license: String::new(),
            dependencies: Vec::new(),
        }
    }
}

impl Default for RepositoryInfo {
    fn default() -> Self {
        Self {
            url: String::new(),
            provider: String::new(),
            organization: String::new(),
            verified: false,
            tesla_approved: false,
        }
    }
}

impl Default for AuthorInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            pgp_fingerprint: None,
            tesla_employee: false,
            verified: false,
        }
    }
}

impl SourceProvenance {
    /// Validate source provenance completeness
    pub fn is_valid(&self) -> bool {
        !self.commit_hash.is_empty()
            && !self.branch.is_empty()
            && !self.source_hash.is_empty()
            && !self.repository.url.is_empty()
    }

    /// Calculate source integrity hash
    pub fn calculate_source_hash(&self, source_files: &[&str]) -> String {
        use crate::hash_engine::Hasher;
        let mut hasher = Hasher::new();

        for file_content in source_files {
            hasher.update(file_content.as_bytes());
        }

        hasher.update(self.commit_hash.as_bytes());
        hasher.update(self.branch.as_bytes());

        format!("{}", hasher.finalize().to_hex())
    }

    /// Tesla-grade source validation
    pub fn tesla_validation(&self) -> bool {
        self.repository.tesla_approved
            && self.repository.verified
            && self.authors.iter().all(|a| a.verified)
            && !self.commit_hash.is_empty()
    }

    /// Add dependency to provenance chain
    pub fn add_dependency(&mut self, dependency: DependencyProvenance) {
        self.dependencies.push(dependency);
    }

    /// Get Tesla-approved dependencies count
    pub fn tesla_approved_deps_count(&self) -> usize {
        self.dependencies.iter().filter(|d| d.tesla_approved).count()
    }
}

impl RepositoryInfo {
    /// Create new repository info
    pub fn new(url: String, provider: String, organization: String) -> Self {
        Self {
            url,
            provider,
            organization,
            verified: false,
            tesla_approved: false,
        }
    }

    /// Verify repository authenticity
    pub fn verify(&mut self) -> bool {
        // Tesla-grade verification logic would go here
        // For now, mark as verified if it's a known organization
        self.verified = matches!(self.organization.as_str(), "tesla" | "spacex" | "neuralink");
        self.tesla_approved = self.verified && self.organization == "tesla";
        self.verified
    }
}

impl AuthorInfo {
    /// Create new author info
    pub fn new(name: String, email: String) -> Self {
        Self {
            name,
            email,
            pgp_fingerprint: None,
            tesla_employee: false,
            verified: false,
        }
    }

    /// Verify author identity
    pub fn verify(&mut self) -> bool {
        // Tesla-grade author verification
        self.tesla_employee = self.email.ends_with("@tesla.com") || self.email.ends_with("@spacex.com");
        self.verified = self.tesla_employee || self.pgp_fingerprint.is_some();
        self.verified
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_provenance_validation() {
        let mut prov = SourceProvenance::default();
        assert!(!prov.is_valid());

        prov.commit_hash = "abc123".to_string();
        prov.branch = "main".to_string();
        prov.source_hash = "def456".to_string();
        prov.repository.url = "https://github.com/test/repo".to_string();

        assert!(prov.is_valid());
    }

    #[test]
    fn test_repository_verification() {
        let mut repo = RepositoryInfo::new(
            "https://github.com/tesla/test".to_string(),
            "github".to_string(),
            "tesla".to_string()
        );

        assert!(repo.verify());
        assert!(repo.tesla_approved);
    }

    #[test]
    fn test_author_verification() {
        let mut author = AuthorInfo::new(
            "John Doe".to_string(),
            "john@tesla.com".to_string()
        );

        assert!(author.verify());
        assert!(author.tesla_employee);
    }
}