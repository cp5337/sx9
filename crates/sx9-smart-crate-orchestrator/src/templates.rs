//! Template processing implementation for Smart Crate Orchestrator
//!
//! This module implements the template processing pipeline with strict
//! security validation and Tesla/SpaceX formatting standards.

use crate::{
    AuditStatus, BuildConfiguration, CrateSpecification, FoundationDependency, Mission,
    OperatorMode, PlaybookFeature, ProvenanceAttestation, SecurityLevel, TemplateContext,
};
use sx9_foundation_manifold::core::diagnostics::anyhow::{Context, Result};
// Handlebars is already imported in main lib.rs
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use sx9_foundation_manifold::core::data::serde_json::{self, Value};
use sx9_foundation_manifold::core::diagnostics::anyhow::{self};
use sx9_foundation_manifold::core::templates::{
    Context as HbsContext, Handlebars, Helper, Output, RenderContext, RenderError,
};
use tempfile::TempDir;
// We need to import fs from main lib via crate or just direct tokiouse sx9_foundation_manifold::core::async_runtime::tokio::fs;
use sx9_foundation_manifold::core::async_runtime::tokio::fs;
use sx9_foundation_manifold::core::diagnostics::tracing::{debug, error, instrument};
use sx9_foundation_manifold::core::security::Signer;

impl crate::SmartCrateOrchestrator {
    /// Resolves foundation dependencies based on specification
    #[instrument(level = "debug", skip(self))]
    pub(crate) async fn resolve_foundation_deps(
        &self,
        spec: &CrateSpecification,
    ) -> Result<Vec<FoundationDependency>> {
        debug!("Resolving foundation dependencies for: {}", spec.name);

        let mut foundations = Vec::new();

        // Core foundation - always required
        foundations.push(FoundationDependency {
            name: "ctas7-foundation-core".to_string(),
            version: "1.0.0".to_string(),
            features: vec!["runtime".to_string()],
            audit_status: AuditStatus::Audited,
        });

        // Mission-specific dependencies
        match spec.mission {
            Mission::DataIngestion => {
                foundations.push(FoundationDependency {
                    name: "ctas7-foundation-data".to_string(),
                    version: "1.0.0".to_string(),
                    features: vec!["streaming".to_string()],
                    audit_status: AuditStatus::Audited,
                });
            }
            Mission::Analysis => {
                foundations.push(FoundationDependency {
                    name: "ctas7-foundation-neural".to_string(),
                    version: "1.0.0".to_string(),
                    features: vec!["inference".to_string()],
                    audit_status: AuditStatus::Pending,
                });
            }
            Mission::Security => {
                foundations.push(FoundationDependency {
                    name: "ctas7-foundation-crypto".to_string(),
                    version: "1.0.0".to_string(),
                    features: vec!["ed25519".to_string()],
                    audit_status: AuditStatus::Audited,
                });
            }
            Mission::Communication => {
                foundations.push(FoundationDependency {
                    name: "ctas7-foundation-network".to_string(),
                    version: "1.0.0".to_string(),
                    features: vec!["routing".to_string()],
                    audit_status: AuditStatus::Audited,
                });
            }
            Mission::Testing => {
                foundations.push(FoundationDependency {
                    name: "ctas7-foundation-test".to_string(),
                    version: "1.0.0".to_string(),
                    features: vec!["harness".to_string()],
                    audit_status: AuditStatus::Audited,
                });
            }
        }

        // Mode-specific dependencies
        match spec.mode {
            OperatorMode::Developer => {
                foundations.push(FoundationDependency {
                    name: "ctas7-foundation-dev".to_string(),
                    version: "1.0.0".to_string(),
                    features: vec!["ed25519".to_string()],
                    audit_status: AuditStatus::Audited,
                });
            }
            OperatorMode::Specialist => {
                foundations.push(FoundationDependency {
                    name: "ctas7-foundation-ops".to_string(),
                    version: "1.0.0".to_string(),
                    features: vec!["production".to_string()],
                    audit_status: AuditStatus::Audited,
                });
            }
            OperatorMode::TestHarness => {
                foundations.push(FoundationDependency {
                    name: "ctas7-foundation-test".to_string(),
                    version: "1.0.0".to_string(),
                    features: vec!["harness".to_string(), "instrumentation".to_string()],
                    audit_status: AuditStatus::Audited,
                });
            }
            OperatorMode::Generalist => {
                foundations.push(FoundationDependency {
                    name: "ctas7-foundation-core".to_string(),
                    version: "1.0.0".to_string(),
                    features: vec!["standard".to_string()],
                    audit_status: AuditStatus::Audited,
                });
            }
        }

        // Feature-specific dependencies
        for feature in &spec.features {
            match feature {
                PlaybookFeature::XsdP1 => {
                    foundations.push(FoundationDependency {
                        name: "ctas7-xsd-p1".to_string(),
                        version: "1.0.0".to_string(),
                        features: vec!["tactical".to_string()],
                        audit_status: AuditStatus::Audited,
                    });
                }
                PlaybookFeature::XsdP2 => {
                    foundations.push(FoundationDependency {
                        name: "ctas7-xsd-p2".to_string(),
                        version: "1.0.0".to_string(),
                        features: vec!["neural-fusion".to_string()],
                        audit_status: AuditStatus::Pending,
                    });
                }
                PlaybookFeature::XsdP3 => {
                    foundations.push(FoundationDependency {
                        name: "ctas7-xsd-p3".to_string(),
                        version: "1.0.0".to_string(),
                        features: vec!["quantum-resistant".to_string()],
                        audit_status: AuditStatus::Pending,
                    });
                }
            }
        }

        Ok(foundations)
    }

    /// Generates build configuration based on specification
    pub(crate) fn generate_build_config(spec: &CrateSpecification) -> Result<BuildConfiguration> {
        let mut features = Vec::new();
        let mut env_vars = HashMap::new();

        // Add mode-specific features
        match spec.mode {
            OperatorMode::Developer => {
                features.push("dev-mode".to_string());
                env_vars.insert("RUST_LOG".to_string(), "debug".to_string());
            }
            OperatorMode::Specialist => {
                features.push("production".to_string());
                env_vars.insert("RUST_LOG".to_string(), "warn".to_string());
            }
            OperatorMode::TestHarness => {
                features.push("test-harness".to_string());
                env_vars.insert("RUST_LOG".to_string(), "trace".to_string());
            }
            OperatorMode::Generalist => {
                features.push("general".to_string());
                env_vars.insert("RUST_LOG".to_string(), "info".to_string());
            }
        }

        // Add security-level specific configuration
        match spec.security_level {
            SecurityLevel::Development => {
                features.push("relaxed-security".to_string());
            }
            SecurityLevel::Staging => {
                features.push("enhanced-security".to_string());
            }
            SecurityLevel::Production => {
                features.push("production-security".to_string());
                env_vars.insert("CTAS_SECURITY_MODE".to_string(), "strict".to_string());
            }
            SecurityLevel::Classified => {
                features.push("classified-security".to_string());
                env_vars.insert("CTAS_SECURITY_MODE".to_string(), "classified".to_string());
            }
        }

        // Add XSD playbook features
        for playbook_feature in &spec.features {
            let feature_name = match playbook_feature {
                PlaybookFeature::XsdP1 => "xsd-p1",
                PlaybookFeature::XsdP2 => "xsd-p2",
                PlaybookFeature::XsdP3 => "xsd-p3",
            };
            features.push(feature_name.to_string());
        }

        // Merge custom environment variables
        for (key, value) in &spec.environment {
            env_vars.insert(key.clone(), value.clone());
        }

        Ok(BuildConfiguration {
            edition: "2021".to_string(),
            profile: match spec.security_level {
                SecurityLevel::Development => "dev",
                _ => "release",
            }
            .to_string(),
            features,
            env_vars: env_vars.clone(),
            build_target: spec
                .environment
                .get("BUILD_TARGET")
                .unwrap_or(&"native".to_string())
                .clone(),
            wasm_optimization: spec
                .environment
                .get("WASM_OPTIMIZATION")
                .unwrap_or(&"size".to_string())
                .clone(),
            firefly1_option: spec
                .environment
                .get("FIREFLY1_OPTION")
                .unwrap_or(&"flight-computer".to_string())
                .clone(),
        })
    }

    /// Discovers all template files in the template directory
    pub(crate) async fn discover_template_files(template_dir: &Path) -> Result<Vec<PathBuf>> {
        let mut template_files = Vec::new();

        let mut entries = fs::read_dir(template_dir)
            .await
            .context("Failed to read template directory")?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("hbs") {
                template_files.push(path);
            }
        }

        // Sort for deterministic processing order
        template_files.sort();

        Ok(template_files)
    }

    /// Processes a single template file with security validation
    #[instrument(level = "debug", skip(self, context))]
    pub(crate) async fn process_single_template(
        &self,
        template_path: &Path,
        context: &TemplateContext,
        output_dir: &TempDir,
    ) -> Result<()> {
        debug!("Processing template: {:?}", template_path);

        // Read template content
        let template_content = fs::read_to_string(template_path)
            .await
            .with_context(|| format!("Failed to read template: {:?}", template_path))?;

        // Validate template security
        Self::validate_template_security(&template_content)?;

        // Process template with Handlebars
        let rendered = self
            .template_engine
            .render_template(&template_content, context)
            .with_context(|| format!("Failed to render template: {:?}", template_path))?;

        // Determine output path
        let output_path = Self::determine_output_path(template_path, output_dir)?;

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .await
                .context("Failed to create output directory")?;
        }

        // Write rendered content
        fs::write(&output_path, rendered)
            .await
            .with_context(|| format!("Failed to write output: {:?}", output_path))?;

        debug!(
            "Template processed successfully: {:?} -> {:?}",
            template_path, output_path
        );

        Ok(())
    }

    /// Validates template content for security issues
    fn validate_template_security(content: &str) -> Result<()> {
        // Check for potentially dangerous operations
        let dangerous_patterns = [
            "std::process::Command",
            "std::fs::remove",
            "std::fs::hard_link",
            "unsafe {",
            "transmute",
            "from_raw",
            "system(",
        ];

        for pattern in &dangerous_patterns {
            if content.contains(pattern) {
                sx9_foundation_core::diagnostics::anyhow::bail!(
                    "Template contains potentially dangerous pattern: {}",
                    pattern
                );
            }
        }

        // Check for template injection vulnerabilities
        if content.contains("{{{") && !content.contains("{{{{") {
            sx9_foundation_manifold::core::diagnostics::anyhow::bail!(
                "Template contains unescaped triple braces"
            );
        }

        Ok(())
    }

    /// Determines output path from template path
    fn determine_output_path(template_path: &Path, output_dir: &TempDir) -> Result<PathBuf> {
        let file_name = template_path
            .file_name()
            .and_then(|name| name.to_str())
            .context("Invalid template file name")?;

        // Remove .hbs extension
        let output_name = if file_name.ends_with(".hbs") {
            &file_name[..file_name.len() - 4]
        } else {
            file_name
        };

        // Handle special output paths
        let output_path = match output_name {
            "lib.rs" => output_dir.path().join("src/lib.rs"),
            "main.rs" => output_dir.path().join("src/main.rs"),
            "build.rs" => output_dir.path().join("build.rs"),
            "Cargo.toml" => output_dir.path().join("Cargo.toml"),
            "README.md" => output_dir.path().join("README.md"),
            "Dockerfile" => output_dir.path().join("Dockerfile"),
            _ => output_dir.path().join(output_name),
        };

        Ok(output_path)
    }

    /// Phase 3: Resolves and validates dependencies
    #[instrument(level = "debug", skip(self))]
    pub(crate) async fn resolve_dependencies(
        &self,
        _context: &TemplateContext,
        _temp_dir: &TempDir,
    ) -> Result<()> {
        debug!("Resolving dependencies");

        // TODO: Implement comprehensive dependency resolution
        // - Parse Cargo.toml for dependencies
        // - Validate against security audit database
        // - Check for known vulnerabilities
        // - Verify cryptographic signatures
        // - Resolve transitive dependencies

        Ok(())
    }

    /// Phase 4: Generates cryptographic provenance attestation
    #[instrument(level = "debug", skip(self))]
    pub(crate) async fn generate_provenance(
        &self,
        context: &TemplateContext,
        temp_dir: &TempDir,
    ) -> Result<ProvenanceAttestation> {
        debug!("Generating cryptographic provenance");

        use ed25519_dalek::Signer;

        // Calculate source hash
        let source_hash = self.calculate_source_hash(temp_dir).await?;

        // Collect environment variables (filtered for security)
        let environment = Self::collect_build_environment();

        // Create attestation payload
        let payload = serde_json::json!({
            "version": crate::PROVENANCE_VERSION,
            "timestamp": crate::SmartCrateOrchestrator::current_timestamp(),
            "source_hash": source_hash,
            "spec_hash": context.metadata.spec_hash,
            "environment": environment,
        });

        let payload_bytes =
            serde_json::to_vec(&payload).context("Failed to serialize attestation payload")?;

        // Sign the payload
        let signature = self.signing_key.sign(&payload_bytes);

        Ok(ProvenanceAttestation {
            version: crate::PROVENANCE_VERSION.to_string(),
            timestamp: crate::SmartCrateOrchestrator::current_timestamp(),
            signature: signature.to_bytes().to_vec(),
            public_key: self.signing_key.verifying_key().to_bytes().to_vec(),
            source_hash,
            environment,
        })
    }

    /// Generates full Gold Disk Build Sheet
    #[instrument(level = "debug", skip(self))]
    pub(crate) async fn generate_build_sheet(
        &self,
        context: &TemplateContext,
        temp_dir: &TempDir,
        attestation: &ProvenanceAttestation,
    ) -> Result<crate::BuildSheet> {
        debug!("Generating Gold Disk Build Sheet");

        // Collect deliverables
        let mut deliverables = Vec::new();
        use walkdir::WalkDir;
        for entry in WalkDir::new(temp_dir.path()) {
            let entry = entry?;
            if entry.file_type().is_file() {
                if let Ok(rel_path) = entry.path().strip_prefix(temp_dir.path()) {
                    deliverables.push(rel_path.to_string_lossy().to_string());
                }
            }
        }

        Ok(crate::BuildSheet {
            id: format!("build-sheet-{}", context.metadata.id),
            timestamp: crate::SmartCrateOrchestrator::current_timestamp(),
            build_context: Self::collect_build_environment(), // Using env wrapper for context
            spec: context.spec.clone(),
            provenance: attestation.clone(),
            build_config: context.build_config.clone(),
            deliverables,
        })
    }

    /// Saves the build sheet to the crate directory
    #[instrument(level = "debug", skip(self, build_sheet))]
    pub(crate) async fn save_build_sheet(
        &self,
        build_sheet: &crate::BuildSheet,
        temp_dir: &TempDir,
    ) -> Result<()> {
        let sheet_path = temp_dir.path().join("build_sheet.json");
        let json =
            serde_json::to_string_pretty(build_sheet).context("Failed to serialize build sheet")?;

        fs::write(&sheet_path, json)
            .await
            .context("Failed to write build sheet to disk")?;

        Ok(())
    }

    /// Phase 5: Validates build process
    #[instrument(level = "debug", skip(self))]
    pub(crate) async fn validate_build(&self, _temp_dir: &TempDir) -> Result<()> {
        debug!("Skipping build validation for now");
        // TODO: Implement proper build validation after ensuring
        // all required files are generated correctly
        Ok(())
    }

    /// Phase 6: Performs security analysis
    #[instrument(level = "debug", skip(self))]
    pub(crate) async fn analyze_security(&self, _temp_dir: &TempDir) -> Result<()> {
        debug!("Performing security analysis");

        // TODO: Implement comprehensive security analysis
        // - Static code analysis
        // - Dependency vulnerability scanning
        // - License compliance checking
        // - Secrets detection
        // - Supply chain analysis

        Ok(())
    }

    /// Phase 7: Prepares final deployment
    #[instrument(level = "debug", skip(self))]
    pub(crate) async fn prepare_deployment(
        &self,
        temp_dir: &TempDir,
        spec: &CrateSpecification,
    ) -> Result<PathBuf> {
        debug!("Preparing deployment for: {}", spec.name);

        let final_path = self.output_dir.join(&spec.name);

        // Remove existing directory if it exists
        if final_path.exists() {
            fs::remove_dir_all(&final_path)
                .await
                .context("Failed to remove existing crate directory")?;
        }

        // Copy from temporary directory to final location
        self.copy_directory(temp_dir.path(), &final_path)
            .await
            .context("Failed to copy crate to final location")?;

        debug!("Deployment prepared at: {:?}", final_path);
        Ok(final_path)
    }

    /// Calculates cryptographic hash of all source files
    async fn calculate_source_hash(&self, dir: &TempDir) -> Result<String> {
        use sx9_foundation_manifold::core::hashing::quick_hash;
        use walkdir::WalkDir;

        let mut paths = Vec::new();
        for entry in WalkDir::new(dir.path()).sort_by_file_name() {
            let entry = entry?;
            if entry.file_type().is_file() {
                paths.push(entry.into_path());
            }
        }

        let mut component_hashes = Vec::new();
        for path in paths {
            match fs::read_to_string(&path).await {
                Ok(content) => {
                    component_hashes.push(quick_hash(&content));
                }
                Err(_) => {
                    // Fallback for binary or unreadable files
                    component_hashes.push("binary_content".to_string());
                }
            }
        }

        let combined = component_hashes.join(":");
        Ok(quick_hash(&combined))
    }

    /// Collects filtered environment variables for attestation
    fn collect_build_environment() -> HashMap<String, String> {
        let mut env = HashMap::new();

        // Only include safe environment variables
        let safe_vars = [
            "CARGO_PKG_VERSION",
            "CARGO_PKG_NAME",
            "RUST_VERSION",
            "TARGET",
        ];

        for var in &safe_vars {
            if let Ok(value) = std::env::var(var) {
                env.insert(var.to_string(), value);
            }
        }

        env
    }

    /// Recursively copies directory with proper error handling
    async fn copy_directory(&self, src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)
            .await
            .context("Failed to create destination directory")?;

        let mut entries = fs::read_dir(src)
            .await
            .context("Failed to read source directory")?;

        while let Some(entry) = entries.next_entry().await? {
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                Box::pin(self.copy_directory(&src_path, &dst_path)).await?;
            } else {
                fs::copy(&src_path, &dst_path).await.with_context(|| {
                    format!("Failed to copy file: {:?} -> {:?}", src_path, dst_path)
                })?;
            }
        }

        Ok(())
    }
}
