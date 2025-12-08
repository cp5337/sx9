//! CTAS QA5 System - Refactored Modular Architecture
//! 
//! A comprehensive quality assurance system with modular design,
//! XSD integration, and cognitive processing capabilities.
//! 
//! QA Levels:
//! - QA0: Baseline (Genetic Hashing)
//! - QA1: Standards Enforcement
//! - QA2: Security Analysis
//! - QA3: Performance Analysis
//! - QA5: Final Integration (XSD + LISP + EEI + OODA)

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// QA5 Refactored Modules
pub mod qa_orchestrator;
pub mod qa_census;
pub mod xsd_integration;

// Legacy modules (to be refactored)
pub mod census;
pub mod fratricide;
pub mod integration;
pub mod levels;

// Re-export main components
pub use qa_orchestrator::{
    QAOrchestrator, QAConfig, QALevel, QAResults, QAIntegration,
    IntegrationResult, QA5Results, QA5Summary
};
pub use qa_census::CensusIntegration;
pub use xsd_integration::{
    XSDIntegration, CrateXSDIntegration, CTASXSDIntegration,
    TrivariatHashBundle, USIMHeader, XSDValidationResult
};

/// Main QA5 System Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QASystemConfig {
    /// Enable all QA levels
    pub enable_all_levels: bool,
    /// QA level configuration
    pub qa_levels: QALevelConfig,
    /// Hash tracking configuration
    pub hash_config: HashConfig,
    /// Integration settings
    pub integrations: IntegrationConfig,
    /// Output settings
    pub output: OutputConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QALevelConfig {
    pub qa0_census: bool,
    pub qa1_standards: bool,
    pub qa2_fratricide: bool,
    pub qa3_ai_integration: bool,
    pub qa5_final_integration: bool, // NEW: QA5 level
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashConfig {
    pub use_blake3: bool,
    pub genetic_hash_baseline: bool,
    pub usim_headers: bool,
    pub forge_hashing: bool,
    pub trivariate_hashing: bool, // NEW: Trivariate hash support
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub repo_prompt: bool,
    pub ai_cli: bool,
    pub python_direct: bool,
    pub hook_systems: bool,
    pub xsd_integration: bool, // NEW: XSD integration
    pub eei_processing: bool,  // NEW: EEI processing
    pub ooda_processing: bool, // NEW: OODA processing
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: String, // "json", "markdown", "both"
    pub output_dir: PathBuf,
    pub generate_reports: bool,
    pub real_time_logging: bool,
}

impl Default for QASystemConfig {
    fn default() -> Self {
        Self {
            enable_all_levels: true,
            qa_levels: QALevelConfig {
                qa0_census: true,
                qa1_standards: true,
                qa2_fratricide: true,
                qa3_ai_integration: true,
                qa5_final_integration: true, // NEW: Enable QA5
            },
            hash_config: HashConfig {
                use_blake3: true,
                genetic_hash_baseline: true,
                usim_headers: true,
                forge_hashing: true,
                trivariate_hashing: true, // NEW: Enable trivariate
            },
            integrations: IntegrationConfig {
                repo_prompt: true,
                ai_cli: true,
                python_direct: true,
                hook_systems: true,
                xsd_integration: true, // NEW: Enable XSD
                eei_processing: true,  // NEW: Enable EEI
                ooda_processing: true, // NEW: Enable OODA
            },
            output: OutputConfig {
                format: "both".to_string(),
                output_dir: PathBuf::from("~/.ctas/qa-reports"),
                generate_reports: true,
                real_time_logging: true,
            },
        }
    }
}

/// Main QA5 System Entry Point
pub struct CTASQASystem {
    config: QASystemConfig,
    orchestrator: QAOrchestrator,
    census_integration: CensusIntegration,
    xsd_integration: XSDIntegration,
}

impl CTASQASystem {
    pub fn new(config: QASystemConfig) -> Self {
        let mut orchestrator = QAOrchestrator::new(QAConfig {
            workspace_root: ".".to_string(),
            enabled_integrations: vec![
                "census".to_string(),
                "xsd".to_string(),
                "genetic_hash".to_string(),
                "standards".to_string(),
                "security".to_string(),
                "performance".to_string(),
            ],
            output_format: qa_orchestrator::OutputFormat::Json,
            parallel_execution: true,
        });

        // Register integrations
        orchestrator.register_integration(
            "census".to_string(),
            Box::new(CensusIntegration::new())
        );

        let mut xsd_integration = XSDIntegration::new();
        // Add common crates for XSD validation
        xsd_integration.add_crate("ctas-core".to_string());
        xsd_integration.add_crate("ctas-genetic-hash".to_string());
        xsd_integration.add_crate("ctas-qa-system".to_string());

        orchestrator.register_integration(
            "xsd".to_string(),
            Box::new(xsd_integration.clone())
        );

        Self {
            config,
            orchestrator,
            census_integration: CensusIntegration::new(),
            xsd_integration,
        }
    }

    /// Run complete QA5 analysis
    pub async fn run_complete_analysis(&self, repo_path: &str) -> Result<QAAnalysisReport> {
        tracing::info!("🚀 Starting CTAS QA5 Analysis for: {}", repo_path);
        
        let mut report = QAAnalysisReport::new(repo_path);
        
        // QA0: Baseline Analysis
        if self.config.qa_levels.qa0_census {
            tracing::info!("📊 QA0: Running Baseline Analysis");
            let qa0_result = self.orchestrator.run_qa_analysis(QALevel::QA0).await?;
            report.qa0_result = Some(qa0_result);
        }

        // QA1: Standards Enforcement
        if self.config.qa_levels.qa1_standards {
            tracing::info!("⚖️ QA1: Running Standards Enforcement");
            let qa1_result = self.orchestrator.run_qa_analysis(QALevel::QA1).await?;
            report.qa1_result = Some(qa1_result);
        }

        // QA2: Security Analysis
        if self.config.qa_levels.qa2_fratricide {
            tracing::info!("🔍 QA2: Running Security Analysis");
            let qa2_result = self.orchestrator.run_qa_analysis(QALevel::QA2).await?;
            report.qa2_result = Some(qa2_result);
        }

        // QA3: Performance Analysis
        if self.config.qa_levels.qa3_ai_integration {
            tracing::info!("🤖 QA3: Running Performance Analysis");
            let qa3_result = self.orchestrator.run_qa_analysis(QALevel::QA3).await?;
            report.qa3_result = Some(qa3_result);
        }

        // QA5: Final Integration Analysis
        if self.config.qa_levels.qa5_final_integration {
            tracing::info!("🎯 QA5: Running Final Integration Analysis");
            let qa5_result = self.orchestrator.run_qa_analysis(QALevel::QA5).await?;
            report.qa5_result = Some(qa5_result);
        }

        // Generate comprehensive report
        report.overall_score = self.calculate_overall_score(&report);
        report.recommendations = self.generate_recommendations(&report);

        tracing::info!("✅ CTAS QA5 Analysis Complete - Score: {:.1}%", report.overall_score);
        
        Ok(report)
    }

    fn calculate_overall_score(&self, report: &QAAnalysisReport) -> f64 {
        let mut total_score = 0.0;
        let mut level_count = 0;

        if report.qa0_result.is_some() {
            total_score += 20.0; // QA0 baseline
            level_count += 1;
        }
        if report.qa1_result.is_some() {
            total_score += 20.0; // QA1 standards
            level_count += 1;
        }
        if report.qa2_result.is_some() {
            total_score += 20.0; // QA2 security
            level_count += 1;
        }
        if report.qa3_result.is_some() {
            total_score += 20.0; // QA3 performance
            level_count += 1;
        }
        if report.qa5_result.is_some() {
            total_score += 20.0; // QA5 final integration
            level_count += 1;
        }

        if level_count > 0 {
            total_score / level_count as f64
        } else {
            0.0
        }
    }

    fn generate_recommendations(&self, _report: &QAAnalysisReport) -> Vec<String> {
        vec![
            "QA5 system operational with modular architecture".to_string(),
            "XSD integration enabled for schema validation".to_string(),
            "Census integration using existing Python scripts".to_string(),
        ]
    }
}

/// QA Analysis Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAAnalysisReport {
    pub repo_path: String,
    pub qa0_result: Option<QAResults>,
    pub qa1_result: Option<QAResults>,
    pub qa2_result: Option<QAResults>,
    pub qa3_result: Option<QAResults>,
    pub qa5_result: Option<QAResults>, // NEW: QA5 results
    pub overall_score: f64,
    pub recommendations: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl QAAnalysisReport {
    pub fn new(repo_path: &str) -> Self {
        Self {
            repo_path: repo_path.to_string(),
            qa0_result: None,
            qa1_result: None,
            qa2_result: None,
            qa3_result: None,
            qa5_result: None,
            overall_score: 0.0,
            recommendations: Vec::new(),
            timestamp: chrono::Utc::now(),
        }
    }
}

// Convenience function to run QA5 analysis
pub async fn run_qa5_analysis(repo_path: &str) -> Result<QAAnalysisReport> {
    let config = QASystemConfig::default();
    let qa_system = CTASQASystem::new(config);
    qa_system.run_complete_analysis(repo_path).await
}
