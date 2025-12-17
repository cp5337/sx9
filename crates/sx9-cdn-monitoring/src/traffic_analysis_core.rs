//! Core Traffic Analysis Engine
//!
//! Focused traffic analysis implementation meeting Tesla-grade standards.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::cyber_operations::ThreatLevel;
use crate::traffic_types::{AnalysisResult, RequestData, ResponseAction, SuspiciousActivity};

/// Traffic Analysis Engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficAnalysis {
    pub total_requests: u64,
    pub requests_per_second: f64,
    pub unique_visitors: u64,
    pub top_countries: Vec<(String, u64)>,
    pub suspicious_activity: Vec<SuspiciousActivity>,
    pub attack_attempts: u64,
    pub blocked_requests: u64,
    pub threat_score: f64,
}

impl TrafficAnalysis {
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            requests_per_second: 0.0,
            unique_visitors: 0,
            top_countries: Vec::new(),
            suspicious_activity: Vec::new(),
            attack_attempts: 0,
            blocked_requests: 0,
            threat_score: 0.0,
        }
    }

    /// Analyze incoming traffic for threats
    pub fn analyze_request(&mut self, request_data: &RequestData) -> AnalysisResult {
        let mut threat_score = 0.0;
        let mut indicators = Vec::new();
        let mut response_action = ResponseAction::Log;

        // IP Reputation Analysis
        if self.is_suspicious_ip(&request_data.source_ip) {
            threat_score += 0.3;
            indicators.push("suspicious_ip".to_string());
        }

        // Geolocation Analysis
        if self.is_restricted_country(&request_data.country) {
            threat_score += 0.4;
            indicators.push("restricted_geography".to_string());
            response_action = ResponseAction::Block;
        }

        // User Agent Analysis
        if self.is_suspicious_user_agent(&request_data.user_agent) {
            threat_score += 0.2;
            indicators.push("suspicious_user_agent".to_string());
        }

        // Pattern Matching
        if self.matches_attack_pattern(&request_data.path, &request_data.payload) {
            threat_score += 0.5;
            indicators.push("attack_pattern_matched".to_string());
            response_action = ResponseAction::Block;
        }

        // Update statistics
        self.total_requests += 1;
        if threat_score > 0.5 {
            self.attack_attempts += 1;
            if matches!(response_action, ResponseAction::Block) {
                self.blocked_requests += 1;
            }
        }

        AnalysisResult {
            threat_score,
            indicators,
            response_action,
            confidence: self.calculate_confidence(threat_score),
            analysis_time: Utc::now(),
        }
    }

    /// Check if IP is suspicious
    fn is_suspicious_ip(&self, ip: &str) -> bool {
        let suspicious_ranges = ["127.", "192.168.", "10.", "172.16."];
        suspicious_ranges.iter().any(|&range| ip.starts_with(range))
    }

    /// Check if country is restricted
    fn is_restricted_country(&self, country: &str) -> bool {
        let restricted = ["CN", "RU", "IR", "KP"];
        restricted.contains(&country)
    }

    /// Check if user agent is suspicious
    fn is_suspicious_user_agent(&self, user_agent: &str) -> bool {
        let suspicious_patterns = [
            "bot", "crawler", "spider", "scraper", "sqlmap", "nikto", "nmap", "masscan",
        ];
        let ua_lower = user_agent.to_lowercase();
        suspicious_patterns
            .iter()
            .any(|&pattern| ua_lower.contains(pattern))
    }

    /// Check if request matches attack patterns
    fn matches_attack_pattern(&self, path: &str, payload: &str) -> bool {
        let attack_patterns = [
            "SELECT * FROM",
            "UNION SELECT",
            "../",
            "../../",
            "<script>",
            "javascript:",
            "eval(",
            "cmd=",
        ];
        let combined = format!("{} {}", path, payload).to_lowercase();
        attack_patterns
            .iter()
            .any(|&pattern| combined.contains(&pattern.to_lowercase()))
    }

    /// Calculate confidence score
    fn calculate_confidence(&self, threat_score: f64) -> f64 {
        (threat_score * 0.8 + 0.2).min(1.0)
    }

    /// Get traffic statistics
    pub fn get_statistics(&self) -> TrafficStatistics {
        TrafficStatistics {
            total_requests: self.total_requests,
            requests_per_second: self.requests_per_second,
            unique_visitors: self.unique_visitors,
            attack_attempts: self.attack_attempts,
            blocked_requests: self.blocked_requests,
            threat_score: self.threat_score,
            suspicious_activities: self.suspicious_activity.len(),
        }
    }
}

/// Traffic Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficStatistics {
    pub total_requests: u64,
    pub requests_per_second: f64,
    pub unique_visitors: u64,
    pub attack_attempts: u64,
    pub blocked_requests: u64,
    pub threat_score: f64,
    pub suspicious_activities: usize,
}
