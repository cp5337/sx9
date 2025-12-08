//! QA System Results Module
//! 
//! Contains all result structures for the different QA levels.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QA0Result {
    pub genetic_hash: String,
    pub file_count: u64,
    pub language_breakdown: String,
    pub baseline_metrics: String,
    pub score: f64,
}

impl QA0Result {
    pub fn new() -> Self {
        Self {
            genetic_hash: String::new(),
            file_count: 0,
            language_breakdown: String::new(),
            baseline_metrics: String::new(),
            score: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QA1Result {
    pub standards_compliance: f64,
    pub usim_headers: f64,
    pub code_quality: String,
    pub violations: Vec<String>,
    pub score: f64,
}

impl QA1Result {
    pub fn new() -> Self {
        Self {
            standards_compliance: 0.0,
            usim_headers: 0.0,
            code_quality: String::new(),
            violations: Vec::new(),
            score: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QA2Result {
    pub fratricide_analysis: String,
    pub file_lock_diff: String,
    pub repo_prompt_analysis: String,
    pub score: f64,
}

impl QA2Result {
    pub fn new() -> Self {
        Self {
            fratricide_analysis: String::new(),
            file_lock_diff: String::new(),
            repo_prompt_analysis: String::new(),
            score: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QA3Result {
    pub ai_cli_analysis: String,
    pub hook_validation: f64,
    pub comprehensive_validation: String,
    pub score: f64,
}

impl QA3Result {
    pub fn new() -> Self {
        Self {
            ai_cli_analysis: String::new(),
            hook_validation: 0.0,
            comprehensive_validation: String::new(),
            score: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QA4Result {
    pub consolidation_analysis: String,
    pub dependency_reduction: f64,
    pub crate_consolidation: f64,
    pub code_quality_metrics: String,
    pub score: f64,
}

impl QA4Result {
    pub fn new() -> Self {
        Self {
            consolidation_analysis: String::new(),
            dependency_reduction: 0.0,
            crate_consolidation: 0.0,
            code_quality_metrics: String::new(),
            score: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QA5Result {
    pub operational_intelligence: String,
    pub threat_emulation: f64,
    pub intelligence_fusion: f64,
    pub countermeasures: f64,
    pub forensics: f64,
    pub investigation: f64,
    pub lisp_rdf_integration: String,
    pub xsd_orchestration: String,
    pub frontend_integration: String,
    pub database_integration: String,
    pub score: f64,
}

impl QA5Result {
    pub fn new() -> Self {
        Self {
            operational_intelligence: String::new(),
            threat_emulation: 0.0,
            intelligence_fusion: 0.0,
            countermeasures: 0.0,
            forensics: 0.0,
            investigation: 0.0,
            lisp_rdf_integration: String::new(),
            xsd_orchestration: String::new(),
            frontend_integration: String::new(),
            database_integration: String::new(),
            score: 0.0,
        }
    }
}
