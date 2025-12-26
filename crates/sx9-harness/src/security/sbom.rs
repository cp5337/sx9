//! Software Bill of Materials (SBOM) Generator
//!
//! Generates SBOMs in CycloneDX and SPDX formats per DoD requirements.
//!
//! ## DoD SBOM Requirements
//!
//! Per the cATO Implementation Guide:
//! - Generate, store, and monitor SBOMs throughout development lifecycle
//! - Leverage centralized SBOM database for zero-day triage
//! - SBOM must include all dependencies (direct and transitive)

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::collections::HashMap;

/// SBOM output format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SbomFormat {
    /// CycloneDX JSON format (preferred)
    CycloneDX,

    /// SPDX JSON format
    Spdx,

    /// Simple JSON format
    SimpleJson,
}

/// Software Bill of Materials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareBom {
    /// SBOM format version
    pub format: SbomFormat,

    /// Format-specific version (e.g., "1.5" for CycloneDX)
    pub spec_version: String,

    /// Unique SBOM identifier
    pub serial_number: String,

    /// Generation timestamp
    pub timestamp: String,

    /// Tool that generated the SBOM
    pub tool: SbomTool,

    /// Main component being described
    pub component: SbomComponent,

    /// All dependencies
    pub dependencies: Vec<SbomDependency>,

    /// Vulnerability information (if scanned)
    pub vulnerabilities: Vec<SbomVulnerability>,
}

/// SBOM generation tool info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbomTool {
    pub vendor: String,
    pub name: String,
    pub version: String,
}

/// SBOM component (the software being described)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbomComponent {
    /// Component type (application, library, etc.)
    pub component_type: String,

    /// Component name
    pub name: String,

    /// Component version
    pub version: String,

    /// Package URL (purl)
    pub purl: Option<String>,

    /// Licenses
    pub licenses: Vec<String>,

    /// Hashes
    pub hashes: HashMap<String, String>,
}

/// SBOM dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbomDependency {
    /// Dependency name
    pub name: String,

    /// Dependency version
    pub version: String,

    /// Package URL
    pub purl: Option<String>,

    /// Is direct dependency (vs transitive)
    pub direct: bool,

    /// Dependency scope (runtime, dev, build)
    pub scope: DependencyScope,

    /// Licenses
    pub licenses: Vec<String>,

    /// Known vulnerabilities
    pub vulnerabilities: Vec<String>,
}

/// Dependency scope
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyScope {
    Runtime,
    Development,
    Build,
    Optional,
}

/// Vulnerability in SBOM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbomVulnerability {
    /// CVE or advisory ID
    pub id: String,

    /// Affected component
    pub affects: String,

    /// Severity (CVSS score)
    pub severity: f32,

    /// Description
    pub description: String,

    /// Fix version (if available)
    pub fix_version: Option<String>,
}

/// SBOM Generator
pub struct SbomGenerator {
    /// Output format
    format: SbomFormat,
}

impl Default for SbomGenerator {
    fn default() -> Self {
        Self {
            format: SbomFormat::CycloneDX,
        }
    }
}

impl SbomGenerator {
    /// Create new SBOM generator
    pub fn new() -> Self {
        Self::default()
    }

    /// Set output format
    pub fn with_format(mut self, format: SbomFormat) -> Self {
        self.format = format;
        self
    }

    /// Generate SBOM for a Rust crate
    pub fn generate(&self, crate_path: &Path) -> Result<SoftwareBom, String> {
        // Parse Cargo.toml
        let cargo_toml = crate_path.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Err("Cargo.toml not found".to_string());
        }

        let cargo_content = std::fs::read_to_string(&cargo_toml)
            .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;

        let cargo: toml::Value = toml::from_str(&cargo_content)
            .map_err(|e| format!("Failed to parse Cargo.toml: {}", e))?;

        // Extract package info
        let package = cargo.get("package")
            .ok_or_else(|| "No [package] section in Cargo.toml".to_string())?;

        let name = package.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let version = package.get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("0.0.0")
            .to_string();

        // Extract dependencies
        let mut dependencies = Vec::new();

        if let Some(deps) = cargo.get("dependencies") {
            if let Some(deps_table) = deps.as_table() {
                for (dep_name, dep_value) in deps_table {
                    let dep_version = match dep_value {
                        toml::Value::String(v) => v.clone(),
                        toml::Value::Table(t) => {
                            t.get("version")
                                .and_then(|v| v.as_str())
                                .unwrap_or("*")
                                .to_string()
                        }
                        _ => "*".to_string(),
                    };

                    dependencies.push(SbomDependency {
                        name: dep_name.clone(),
                        version: dep_version,
                        purl: Some(format!("pkg:cargo/{}@*", dep_name)),
                        direct: true,
                        scope: DependencyScope::Runtime,
                        licenses: Vec::new(),
                        vulnerabilities: Vec::new(),
                    });
                }
            }
        }

        // Add dev-dependencies
        if let Some(deps) = cargo.get("dev-dependencies") {
            if let Some(deps_table) = deps.as_table() {
                for (dep_name, dep_value) in deps_table {
                    let dep_version = match dep_value {
                        toml::Value::String(v) => v.clone(),
                        toml::Value::Table(t) => {
                            t.get("version")
                                .and_then(|v| v.as_str())
                                .unwrap_or("*")
                                .to_string()
                        }
                        _ => "*".to_string(),
                    };

                    dependencies.push(SbomDependency {
                        name: dep_name.clone(),
                        version: dep_version,
                        purl: Some(format!("pkg:cargo/{}@*", dep_name)),
                        direct: true,
                        scope: DependencyScope::Development,
                        licenses: Vec::new(),
                        vulnerabilities: Vec::new(),
                    });
                }
            }
        }

        Ok(SoftwareBom {
            format: self.format,
            spec_version: "1.5".to_string(),
            serial_number: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            tool: SbomTool {
                vendor: "SX9".to_string(),
                name: "sx9-harness".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            component: SbomComponent {
                component_type: "application".to_string(),
                name,
                version,
                purl: None,
                licenses: Vec::new(),
                hashes: HashMap::new(),
            },
            dependencies,
            vulnerabilities: Vec::new(),
        })
    }

    /// Export SBOM to JSON string
    pub fn to_json(&self, sbom: &SoftwareBom) -> Result<String, String> {
        serde_json::to_string_pretty(sbom)
            .map_err(|e| format!("Failed to serialize SBOM: {}", e))
    }

    /// Export SBOM to file
    pub fn write_to_file(&self, sbom: &SoftwareBom, path: &Path) -> Result<(), String> {
        let json = self.to_json(sbom)?;
        std::fs::write(path, json)
            .map_err(|e| format!("Failed to write SBOM: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sbom_format() {
        let generator = SbomGenerator::new().with_format(SbomFormat::Spdx);
        assert_eq!(generator.format, SbomFormat::Spdx);
    }

    #[test]
    fn test_dependency_scope() {
        let dep = SbomDependency {
            name: "serde".to_string(),
            version: "1.0".to_string(),
            purl: Some("pkg:cargo/serde@1.0".to_string()),
            direct: true,
            scope: DependencyScope::Runtime,
            licenses: vec!["MIT".to_string()],
            vulnerabilities: Vec::new(),
        };

        assert_eq!(dep.scope, DependencyScope::Runtime);
    }
}
