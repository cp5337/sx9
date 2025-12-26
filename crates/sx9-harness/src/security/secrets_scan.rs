//! Secrets Scanning
//!
//! Detects hardcoded secrets, API keys, and credentials in source code.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

use super::Severity;

/// Secret finding from scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretFinding {
    /// Type of secret found
    pub secret_type: SecretType,

    /// File path
    pub file: String,

    /// Line number
    pub line: usize,

    /// Matched pattern (redacted)
    pub match_redacted: String,

    /// Severity
    pub severity: Severity,

    /// Rule that matched
    pub rule: String,
}

/// Types of secrets detected
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecretType {
    AwsKey,
    AwsSecret,
    GithubToken,
    GitlabToken,
    SlackToken,
    PrivateKey,
    GenericApiKey,
    GenericSecret,
    Password,
    JwtSecret,
    DatabaseUrl,
}

impl SecretType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SecretType::AwsKey => "AWS Access Key",
            SecretType::AwsSecret => "AWS Secret Key",
            SecretType::GithubToken => "GitHub Token",
            SecretType::GitlabToken => "GitLab Token",
            SecretType::SlackToken => "Slack Token",
            SecretType::PrivateKey => "Private Key",
            SecretType::GenericApiKey => "API Key",
            SecretType::GenericSecret => "Generic Secret",
            SecretType::Password => "Password",
            SecretType::JwtSecret => "JWT Secret",
            SecretType::DatabaseUrl => "Database URL",
        }
    }
}

/// Secret detection rule
struct SecretRule {
    name: &'static str,
    pattern: Regex,
    secret_type: SecretType,
    severity: Severity,
}

/// Secrets scanner
pub struct SecretsScan {
    rules: Vec<SecretRule>,
    ignore_patterns: Vec<String>,
}

impl Default for SecretsScan {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretsScan {
    /// Create new secrets scanner with default rules
    pub fn new() -> Self {
        let rules = vec![
            SecretRule {
                name: "aws-access-key",
                pattern: Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(),
                secret_type: SecretType::AwsKey,
                severity: Severity::Critical,
            },
            SecretRule {
                name: "aws-secret-key",
                pattern: Regex::new(r#"(?i)(aws_secret|aws_secret_key|secret_access_key)\s*[:=]\s*["']?[A-Za-z0-9/+=]{40}["']?"#).unwrap(),
                secret_type: SecretType::AwsSecret,
                severity: Severity::Critical,
            },
            SecretRule {
                name: "github-token",
                pattern: Regex::new(r"gh[pousr]_[A-Za-z0-9_]{36,}").unwrap(),
                secret_type: SecretType::GithubToken,
                severity: Severity::Critical,
            },
            SecretRule {
                name: "gitlab-token",
                pattern: Regex::new(r"glpat-[A-Za-z0-9\-]{20,}").unwrap(),
                secret_type: SecretType::GitlabToken,
                severity: Severity::Critical,
            },
            SecretRule {
                name: "slack-token",
                pattern: Regex::new(r"xox[baprs]-[0-9]{10,}-[0-9]{10,}-[a-zA-Z0-9]{20,}").unwrap(),
                secret_type: SecretType::SlackToken,
                severity: Severity::Critical,
            },
            SecretRule {
                name: "private-key",
                pattern: Regex::new(r"-----BEGIN (RSA |EC |DSA |OPENSSH )?PRIVATE KEY-----").unwrap(),
                secret_type: SecretType::PrivateKey,
                severity: Severity::Critical,
            },
            SecretRule {
                name: "generic-api-key",
                pattern: Regex::new(r#"(?i)(api[_-]?key|apikey)\s*[:=]\s*["']?[A-Za-z0-9]{20,}["']?"#).unwrap(),
                secret_type: SecretType::GenericApiKey,
                severity: Severity::High,
            },
            SecretRule {
                name: "generic-secret",
                pattern: Regex::new(r#"(?i)(secret|token)\s*[:=]\s*["'][A-Za-z0-9]{16,}["']"#).unwrap(),
                secret_type: SecretType::GenericSecret,
                severity: Severity::High,
            },
            SecretRule {
                name: "password",
                pattern: Regex::new(r#"(?i)(password|passwd|pwd)\s*[:=]\s*["'][^"']{8,}["']"#).unwrap(),
                secret_type: SecretType::Password,
                severity: Severity::High,
            },
            SecretRule {
                name: "jwt-secret",
                pattern: Regex::new(r#"(?i)(jwt[_-]?secret|signing[_-]?key)\s*[:=]\s*["'][A-Za-z0-9+/=]{20,}["']"#).unwrap(),
                secret_type: SecretType::JwtSecret,
                severity: Severity::Critical,
            },
            SecretRule {
                name: "database-url",
                pattern: Regex::new(r#"(?i)(postgres|mysql|mongodb|redis)://[^:]+:[^@]+@[^\s]+"#).unwrap(),
                secret_type: SecretType::DatabaseUrl,
                severity: Severity::Critical,
            },
        ];

        Self {
            rules,
            ignore_patterns: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "target".to_string(),
                "vendor".to_string(),
                ".env.example".to_string(),
            ],
        }
    }

    /// Add ignore pattern
    pub fn ignore(mut self, pattern: &str) -> Self {
        self.ignore_patterns.push(pattern.to_string());
        self
    }

    /// Scan project for secrets
    pub async fn scan(&self, project_path: &Path) -> Vec<SecretFinding> {
        let mut findings = Vec::new();

        for entry in WalkDir::new(project_path)
            .into_iter()
            .filter_entry(|e| !self.should_ignore(e.path()))
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    // Only scan text files
                    let ext_str = ext.to_string_lossy();
                    if self.is_scannable_extension(&ext_str) {
                        if let Ok(content) = std::fs::read_to_string(entry.path()) {
                            self.scan_content(&content, entry.path(), &mut findings);
                        }
                    }
                }
            }
        }

        findings
    }

    /// Check if path should be ignored
    fn should_ignore(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.ignore_patterns.iter().any(|p| path_str.contains(p))
    }

    /// Check if file extension is scannable
    fn is_scannable_extension(&self, ext: &str) -> bool {
        matches!(
            ext,
            "rs" | "ts" | "tsx" | "js" | "jsx" | "py" | "go" | "java"
                | "rb" | "php" | "sh" | "bash" | "zsh" | "yaml" | "yml"
                | "json" | "toml" | "env" | "conf" | "config" | "ini"
                | "properties" | "xml" | "md" | "txt"
        )
    }

    /// Scan content for secrets
    fn scan_content(&self, content: &str, path: &Path, findings: &mut Vec<SecretFinding>) {
        for (line_num, line) in content.lines().enumerate() {
            for rule in &self.rules {
                if let Some(mat) = rule.pattern.find(line) {
                    // Redact the match
                    let redacted = self.redact_match(mat.as_str());

                    findings.push(SecretFinding {
                        secret_type: rule.secret_type,
                        file: path.to_string_lossy().to_string(),
                        line: line_num + 1,
                        match_redacted: redacted,
                        severity: rule.severity,
                        rule: rule.name.to_string(),
                    });
                }
            }
        }
    }

    /// Redact a matched secret
    fn redact_match(&self, s: &str) -> String {
        if s.len() <= 8 {
            "*".repeat(s.len())
        } else {
            format!("{}...{}", &s[..4], "*".repeat(s.len() - 8))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aws_key_detection() {
        let scanner = SecretsScan::new();
        let content = "aws_access_key_id = AKIAIOSFODNN7EXAMPLE";
        let mut findings = Vec::new();
        scanner.scan_content(content, Path::new("test.rs"), &mut findings);
        assert!(!findings.is_empty());
        assert_eq!(findings[0].secret_type, SecretType::AwsKey);
    }

    #[test]
    fn test_github_token_detection() {
        let scanner = SecretsScan::new();
        let content = "token = ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
        let mut findings = Vec::new();
        scanner.scan_content(content, Path::new("test.rs"), &mut findings);
        assert!(!findings.is_empty());
        assert_eq!(findings[0].secret_type, SecretType::GithubToken);
    }

    #[test]
    fn test_redaction() {
        let scanner = SecretsScan::new();
        assert_eq!(scanner.redact_match("short"), "*****");
        // "longsecretvalue" is 15 chars: first 4 + "..." + (15-8) asterisks = 7 asterisks
        assert_eq!(scanner.redact_match("longsecretvalue"), "long...*******");
    }
}
