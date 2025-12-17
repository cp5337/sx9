//! Traffic Intelligence Module (Tesla-Grade Refactored)
//!
//! Orchestrates advanced traffic analysis, threat detection, and intelligence
//! gathering capabilities using focused sub-modules.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

use crate::analysis_engine::AnalysisEngine;
use crate::cyber_operations::ThreatLevel;
use crate::intelligence_reports::{
    IntelligenceManager, IntelligenceReport, ReportType, ThreatIndicator,
};
use crate::traffic_analysis_core::{TrafficAnalysis, TrafficStatistics};
use crate::traffic_types::{
    ActivityType, AnalysisResult, RequestData, ResponseAction, SuspiciousActivity,
};

/// Traffic Intelligence System (Tesla-Grade)
pub struct TrafficIntelligence {
    pub traffic_analysis: TrafficAnalysis,
    pub intelligence_manager: IntelligenceManager,
    pub analysis_engine: AnalysisEngine,
    pub suspicious_activities: Vec<SuspiciousActivity>,
}

impl TrafficIntelligence {
    pub fn new() -> Self {
        Self {
            traffic_analysis: TrafficAnalysis::new(),
            intelligence_manager: IntelligenceManager::new(),
            analysis_engine: AnalysisEngine::new(),
            suspicious_activities: Vec::new(),
        }
    }

    /// Analyze incoming traffic for threats
    pub async fn analyze_traffic(&mut self, request_data: &RequestData) -> AnalysisResult {
        // Use analysis engine for comprehensive threat detection
        let engine_result = self.analysis_engine.analyze_request(request_data).await;

        // Use traffic analysis for baseline detection
        let traffic_result = self.traffic_analysis.analyze_request(request_data);

        // Combine results for enhanced accuracy
        let combined_threat_score =
            (engine_result.threat_score + traffic_result.threat_score) / 2.0;
        let mut combined_indicators = engine_result.indicators;
        combined_indicators.extend(traffic_result.indicators);

        // Create suspicious activity if threshold exceeded
        if combined_threat_score > 0.6 {
            let suspicious_activity = SuspiciousActivity {
                activity_id: format!("act_{}", uuid::Uuid::new_v4()),
                activity_type: self.classify_activity_type(request_data),
                source_ip: request_data.source_ip.clone(),
                country: request_data.country.clone(),
                timestamp: request_data.timestamp,
                severity: self.calculate_threat_level(combined_threat_score),
                description: format!(
                    "Suspicious activity detected from {}",
                    request_data.source_ip
                ),
                indicators: combined_indicators.clone(),
                response_taken: Some(format!("{:?}", engine_result.response_action)),
            };

            self.suspicious_activities.push(suspicious_activity);
            warn!(
                "🚨 Suspicious activity detected: score {:.2}",
                combined_threat_score
            );
        }

        AnalysisResult {
            threat_score: combined_threat_score,
            indicators: combined_indicators,
            response_action: engine_result.response_action,
            confidence: (engine_result.confidence + traffic_result.confidence) / 2.0,
            analysis_time: Utc::now(),
        }
    }

    /// Generate intelligence report
    pub async fn generate_intelligence_report(
        &mut self,
        report_type: ReportType,
    ) -> IntelligenceReport {
        self.intelligence_manager
            .generate_intelligence_report(report_type)
            .await
    }

    /// Add threat indicator
    pub fn add_threat_indicator(&mut self, indicator: ThreatIndicator) {
        self.intelligence_manager.add_threat_indicator(indicator);
    }

    /// Get suspicious activities summary
    pub fn get_suspicious_activities_summary(&self) -> HashMap<ActivityType, usize> {
        let mut summary = HashMap::new();
        for activity in &self.suspicious_activities {
            *summary.entry(activity.activity_type.clone()).or_insert(0) += 1;
        }
        summary
    }

    /// Clear old suspicious activities
    pub fn cleanup_old_activities(&mut self, hours_old: i64) {
        let cutoff_time = Utc::now() - chrono::Duration::hours(hours_old);
        self.suspicious_activities
            .retain(|activity| activity.timestamp > cutoff_time);
        info!("🧹 Cleaned up activities older than {} hours", hours_old);
    }

    /// Classify activity type based on request data
    fn classify_activity_type(&self, request_data: &RequestData) -> ActivityType {
        let path_lower = request_data.path.to_lowercase();
        let payload_lower = request_data.payload.to_lowercase();
        let ua_lower = request_data.user_agent.to_lowercase();

        if path_lower.contains("../") || payload_lower.contains("../") {
            ActivityType::DirectoryTraversal
        } else if payload_lower.contains("select") || payload_lower.contains("union") {
            ActivityType::SQLInjection
        } else if payload_lower.contains("<script>") || payload_lower.contains("javascript:") {
            ActivityType::XSSAttempt
        } else if ua_lower.contains("bot") || ua_lower.contains("crawler") {
            ActivityType::BotnetActivity
        } else if ua_lower.contains("nmap") || ua_lower.contains("masscan") {
            ActivityType::PortScanning
        } else {
            ActivityType::MaliciousPayload
        }
    }

    /// Calculate threat level from score
    fn calculate_threat_level(&self, score: f64) -> ThreatLevel {
        match score {
            s if s >= 0.9 => ThreatLevel::Warfare,
            s if s >= 0.7 => ThreatLevel::Critical,
            s if s >= 0.5 => ThreatLevel::High,
            s if s >= 0.3 => ThreatLevel::Medium,
            _ => ThreatLevel::Low,
        }
    }

    /// Get intelligence statistics
    pub fn get_intelligence_stats(&self) -> IntelligenceStats {
        IntelligenceStats {
            total_suspicious_activities: self.suspicious_activities.len(),
            total_reports: self.intelligence_manager.reports.len(),
            total_indicators: self.intelligence_manager.threat_indicators.len(),
            analysis_requests: self.traffic_analysis.total_requests,
            blocked_requests: self.traffic_analysis.blocked_requests,
            current_threat_score: self.traffic_analysis.threat_score,
        }
    }
}

/// Intelligence Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceStats {
    pub total_suspicious_activities: usize,
    pub total_reports: usize,
    pub total_indicators: usize,
    pub analysis_requests: u64,
    pub blocked_requests: u64,
    pub current_threat_score: f64,
}
