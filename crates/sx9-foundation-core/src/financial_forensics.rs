use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::types::{PatternDetectionEngine, AnomalyDetector, ComplianceChecker, SanctionEntry};

/// Financial Forensics Engine
#[derive(Debug, Clone)]
pub struct FinancialForensics {
    pub pattern_engine: PatternDetectionEngine,
    pub anomaly_detector: AnomalyDetector,
    pub compliance_checker: ComplianceChecker,
    pub sanctions_database: DashMap<String, SanctionEntry>,
}

impl FinancialForensics {
    pub fn new() -> Self {
        Self {
            pattern_engine: PatternDetectionEngine {
                structuring_detector: crate::types::StructuringDetector::new(),
            },
            anomaly_detector: AnomalyDetector::new(),
            compliance_checker: ComplianceChecker::new(),
            sanctions_database: DashMap::new(),
        }
    }

    pub async fn detect_patterns(&self, transaction_data: &str) -> Result<Vec<String>> {
        let patterns = self.pattern_engine.detect_patterns(transaction_data).await?;
        info!("Detected {} patterns in transaction data", patterns.len());
        Ok(patterns)
    }

    pub async fn detect_anomalies(&self, transaction_data: &str) -> Result<Vec<String>> {
        let anomalies = self.anomaly_detector.detect_anomalies(transaction_data).await?;
        info!("Detected {} anomalies in transaction data", anomalies.len());
        Ok(anomalies)
    }

    pub async fn check_compliance(&self, entity: &str) -> Result<bool> {
        let is_compliant = self.compliance_checker.check_compliance(entity).await?;
        info!("Compliance check for {}: {}", entity, is_compliant);
        Ok(is_compliant)
    }

    pub async fn check_sanctions(&self, entity: &str) -> Result<Vec<SanctionEntry>> {
        let mut matches = vec![];
        
        // Check against sanctions database
        for entry in self.sanctions_database.iter() {
            if entry.key().to_lowercase().contains(&entity.to_lowercase()) {
                matches.push(entry.value().clone());
            }
        }

        info!("Found {} sanctions matches for {}", matches.len(), entity);
        Ok(matches)
    }

    pub async fn add_sanction_entry(&self, entry: SanctionEntry) -> Result<()> {
        self.sanctions_database.insert(entry.entity.clone(), entry);
        info!("Added new sanction entry to database");
        Ok(())
    }

    pub async fn perform_forensic_analysis(&self, case_id: &str, data: &str) -> Result<ForensicReport> {
        let patterns = self.detect_patterns(data).await?;
        let anomalies = self.detect_anomalies(data).await?;
        
        let report = ForensicReport {
            case_id: case_id.to_string(),
            timestamp: Utc::now(),
            patterns_detected: patterns,
            anomalies_found: anomalies,
            risk_assessment: self.calculate_risk_score(&patterns, &anomalies),
            recommendations: self.generate_recommendations(&patterns, &anomalies),
        };

        info!("Generated forensic report for case: {}", case_id);
        Ok(report)
    }

    fn calculate_risk_score(&self, patterns: &[String], anomalies: &[String]) -> f64 {
        let pattern_score = patterns.len() as f64 * 0.1;
        let anomaly_score = anomalies.len() as f64 * 0.2;
        (pattern_score + anomaly_score).min(1.0)
    }

    fn generate_recommendations(&self, patterns: &[String], anomalies: &[String]) -> Vec<String> {
        let mut recommendations = vec![];
        
        if !patterns.is_empty() {
            recommendations.push("Investigate detected patterns further".to_string());
        }
        
        if !anomalies.is_empty() {
            recommendations.push("Review anomalies for potential fraud".to_string());
        }
        
        if patterns.len() + anomalies.len() > 5 {
            recommendations.push("Consider escalating to senior investigator".to_string());
        }

        recommendations
    }

    pub fn get_sanctions_count(&self) -> usize {
        self.sanctions_database.len()
    }

    pub fn clear_sanctions_database(&self) {
        self.sanctions_database.clear();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicReport {
    pub case_id: String,
    pub timestamp: DateTime<Utc>,
    pub patterns_detected: Vec<String>,
    pub anomalies_found: Vec<String>,
    pub risk_assessment: f64,
    pub recommendations: Vec<String>,
}

#[async_trait]
pub trait FinancialForensicsTrait {
    async fn detect_patterns(&self, transaction_data: &str) -> Result<Vec<String>>;
    async fn detect_anomalies(&self, transaction_data: &str) -> Result<Vec<String>>;
    async fn perform_forensic_analysis(&self, case_id: &str, data: &str) -> Result<ForensicReport>;
}

impl FinancialForensicsTrait for FinancialForensics {
    async fn detect_patterns(&self, transaction_data: &str) -> Result<Vec<String>> {
        self.detect_patterns(transaction_data).await
    }

    async fn detect_anomalies(&self, transaction_data: &str) -> Result<Vec<String>> {
        self.detect_anomalies(transaction_data).await
    }

    async fn perform_forensic_analysis(&self, case_id: &str, data: &str) -> Result<ForensicReport> {
        self.perform_forensic_analysis(case_id, data).await
    }
}

