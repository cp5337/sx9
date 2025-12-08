// src/fake_code_detector.rs
use anyhow::Result;
use chrono::{DateTime, Utc};
use colored::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::violations::{QualityViolation, ViolationType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FakeCodeAlert {
    pub alert_id: String,
    pub crate_name: String,
    pub file_path: String,
    pub alert_type: FakeCodeAlertType,
    pub confidence: f64,
    pub evidence: Vec<String>,
    pub line_numbers: Vec<usize>,
    pub detected_at: DateTime<Utc>,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FakeCodeAlertType {
    SystematicFakeData,     // Multiple hardcoded values across functions
    PlaceholderImplementation, // Functions returning fake/test data
    MockConfigurationDetected, // Config with localhost/test endpoints
    HardcodedCredentials,      // Security risk - fake passwords/tokens
    SimpleVersionTrap,         // "Let me create a simple version" pattern
    StubImplementation,        // todo!/unimplemented! without proper docs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Critical,  // Fake credentials, prod config with test data
    High,      // Multiple fake patterns, systematic fakery
    Medium,    // Single fake patterns, isolated issues
    Low,       // Minor placeholder issues
}

pub struct FakeCodeDetector {
    critical_patterns: HashMap<String, Regex>,
    systematic_patterns: HashMap<String, Regex>,
    context_patterns: HashMap<String, Regex>,
}

impl FakeCodeDetector {
    pub fn new() -> Self {
        let mut critical_patterns = HashMap::new();
        let mut systematic_patterns = HashMap::new();
        let mut context_patterns = HashMap::new();
        
        // CRITICAL: Security and production risks
        critical_patterns.insert(
            "hardcoded_credentials".to_string(),
            Regex::new(r#"(?i)(?:password|token|key|secret|api_key)\s*[:=]\s*"(?:test|admin|password|123|secret|token|key)""#).unwrap()
        );
        critical_patterns.insert(
            "fake_prod_config".to_string(),
            Regex::new(r#"(?i)(?:prod|production).*(?:localhost|127\.0\.0\.1|example\.com|test\.local)"#).unwrap()
        );
        critical_patterns.insert(
            "database_fake_urls".to_string(),
            Regex::new(r#"(?i)(?:database_url|db_url|connection_string)\s*[:=]\s*".*(?:localhost|test\.db|example).*""#).unwrap()
        );
        
        // SYSTEMATIC: Patterns indicating widespread fake code
        systematic_patterns.insert(
            "multiple_test_strings".to_string(),
            Regex::new(r#""(?:test_?\w*|example_?\w*|sample_?\w*|dummy_?\w*|fake_?\w*|mock_?\w*)"[^"]*"(?:test_?\w*|example_?\w*|sample_?\w*|dummy_?\w*|fake_?\w*|mock_?\w*)""#).unwrap()
        );
        systematic_patterns.insert(
            "hardcoded_magic_numbers".to_string(),
            Regex::new(r"\b(?:42|123|999|1234|9999|12345|0xDEADBEEF|0xCAFEBABE)\b").unwrap()
        );
        systematic_patterns.insert(
            "placeholder_returns_pattern".to_string(),
            Regex::new(r"return\s+(?:Ok|Some|vec!)\(.*(?:test|example|dummy|fake|mock|placeholder).*\)").unwrap()
        );
        systematic_patterns.insert(
            "fake_data_collections".to_string(),
            Regex::new(r#"(?:vec!|HashMap::new)\[.*"(?:test|example|dummy|fake|mock).*\]"#).unwrap()
        );
        
        // CONTEXTUAL: Patterns that are suspicious in context
        context_patterns.insert(
            "simple_version_comments".to_string(),
            Regex::new(r"(?i)//.*(?:simple version|simplified|quick implementation|temporary|placeholder)").unwrap()
        );
        context_patterns.insert(
            "todo_without_detail".to_string(),
            Regex::new(r"todo!\(\s*\)|todo!\(\".*(?:implement|fix|complete).*\"\)").unwrap()
        );
        context_patterns.insert(
            "catch_all_matches".to_string(),
            Regex::new(r"match\s+\w+\s*\{[^}]*_\s*=>\s*(?:todo!|unimplemented!|.*(?:test|example|dummy)).*\}").unwrap()
        );
        
        Self {
            critical_patterns,
            systematic_patterns,
            context_patterns,
        }
    }
    
    pub fn analyze_for_fake_code(&self, content: &str, file_path: &str, crate_name: &str) -> Result<Vec<FakeCodeAlert>> {
        let mut alerts = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        // Check for critical violations first
        alerts.extend(self.detect_critical_violations(content, file_path, crate_name, &lines)?);
        
        // Check for systematic fake code patterns
        alerts.extend(self.detect_systematic_fakery(content, file_path, crate_name, &lines)?);
        
        // Check for contextual suspicious patterns
        alerts.extend(self.detect_contextual_issues(content, file_path, crate_name, &lines)?);
        
        // Check for "simple version" trap patterns
        alerts.extend(self.detect_simple_version_trap(content, file_path, crate_name, &lines)?);
        
        Ok(alerts)
    }
    
    fn detect_critical_violations(&self, content: &str, file_path: &str, crate_name: &str, lines: &[&str]) -> Result<Vec<FakeCodeAlert>> {
        let mut alerts = Vec::new();
        
        for (pattern_name, regex) in &self.critical_patterns {
            let matches: Vec<_> = regex.find_iter(content).collect();
            
            if !matches.is_empty() {
                let line_numbers: Vec<usize> = matches.iter()
                    .map(|m| content[..m.start()].lines().count() + 1)
                    .collect();
                
                let evidence: Vec<String> = matches.iter()
                    .map(|m| {
                        let line_num = content[..m.start()].lines().count();
                        format!("Line {}: {}", line_num + 1, lines[line_num].trim())
                    })
                    .collect();
                
                let alert_type = match pattern_name.as_str() {
                    "hardcoded_credentials" => FakeCodeAlertType::HardcodedCredentials,
                    "fake_prod_config" => FakeCodeAlertType::MockConfigurationDetected,
                    "database_fake_urls" => FakeCodeAlertType::MockConfigurationDetected,
                    _ => FakeCodeAlertType::SystematicFakeData,
                };
                
                alerts.push(FakeCodeAlert {
                    alert_id: uuid::Uuid::new_v4().to_string(),
                    crate_name: crate_name.to_string(),
                    file_path: file_path.to_string(),
                    alert_type,
                    confidence: 0.95, // High confidence for critical patterns
                    evidence,
                    line_numbers,
                    detected_at: Utc::now(),
                    risk_level: RiskLevel::Critical,
                });
            }
        }
        
        Ok(alerts)
    }
    
    fn detect_systematic_fakery(&self, content: &str, file_path: &str, crate_name: &str, lines: &[&str]) -> Result<Vec<FakeCodeAlert>> {
        let mut alerts = Vec::new();
        let mut total_fake_patterns = 0;
        let mut all_evidence = Vec::new();
        let mut all_line_numbers = Vec::new();
        
        for (pattern_name, regex) in &self.systematic_patterns {
            let matches: Vec<_> = regex.find_iter(content).collect();
            total_fake_patterns += matches.len();
            
            for m in matches {
                let line_num = content[..m.start()].lines().count();
                all_line_numbers.push(line_num + 1);
                all_evidence.push(format!("Line {}: {} ({})", 
                    line_num + 1, 
                    lines[line_num].trim(),
                    pattern_name
                ));
            }
        }
        
        // If we found multiple systematic patterns, create a high-severity alert
        if total_fake_patterns >= 3 {
            alerts.push(FakeCodeAlert {
                alert_id: uuid::Uuid::new_v4().to_string(),
                crate_name: crate_name.to_string(),
                file_path: file_path.to_string(),
                alert_type: FakeCodeAlertType::SystematicFakeData,
                confidence: 0.85,
                evidence: all_evidence,
                line_numbers: all_line_numbers,
                detected_at: Utc::now(),
                risk_level: RiskLevel::High,
            });
        }
        
        Ok(alerts)
    }
    
    fn detect_simple_version_trap(&self, content: &str, file_path: &str, crate_name: &str, lines: &[&str]) -> Result<Vec<FakeCodeAlert>> {
        let mut alerts = Vec::new();
        
        // Look for the telltale "simple version" pattern:
        // 1. Comments mentioning simplification
        // 2. Followed by hardcoded values
        // 3. Functions that return fake data
        
        let simple_version_indicators = [
            r"(?i)//.*(?:simple version|let me create|simplified|quick fix)",
            r"(?i)//.*(?:temporary|placeholder|for now)",
            r"(?i)//.*(?:just hardcode|hard code|fake it)",
        ];
        
        let mut suspicious_comments = Vec::new();
        
        for (line_num, line) in lines.iter().enumerate() {
            for pattern in &simple_version_indicators {
                let regex = Regex::new(pattern).unwrap();
                if regex.is_match(line) {
                    suspicious_comments.push((line_num, line));
                }
            }
        }
        
        // If we find "simple version" comments, look for fake code in the next 10 lines
        for (comment_line, _comment_text) in suspicious_comments {
            let search_end = (comment_line + 10).min(lines.len());
            let mut fake_evidence = Vec::new();
            
            for line_idx in comment_line..search_end {
                let line = lines[line_idx];
                
                // Check for fake patterns in following lines
                let fake_patterns = [
                    r#""(?:test|example|dummy|fake|mock|placeholder)""#,
                    r"\b(?:42|123|999|1234)\b",
                    r"return\s+(?:Ok|Some)\(.*(?:test|example|dummy)",
                    r"todo!\(|unimplemented!\(",
                ];
                
                for pattern in &fake_patterns {
                    let regex = Regex::new(pattern).unwrap();
                    if regex.is_match(line) {
                        fake_evidence.push(format!("Line {}: {}", line_idx + 1, line.trim()));
                    }
                }
            }
            
            if !fake_evidence.is_empty() {
                alerts.push(FakeCodeAlert {
                    alert_id: uuid::Uuid::new_v4().to_string(),
                    crate_name: crate_name.to_string(),
                    file_path: file_path.to_string(),
                    alert_type: FakeCodeAlertType::SimpleVersionTrap,
                    confidence: 0.90, // Very high confidence when comment + fake code
                    evidence: fake_evidence,
                    line_numbers: vec![comment_line + 1],
                    detected_at: Utc::now(),
                    risk_level: RiskLevel::High,
                });
            }
        }
        
        Ok(alerts)
    }
    
    fn detect_contextual_issues(&self, content: &str, file_path: &str, crate_name: &str, lines: &[&str]) -> Result<Vec<FakeCodeAlert>> {
        let mut alerts = Vec::new();
        
        // Look for stub implementations - functions that are clearly placeholders
        let stub_function_regex = Regex::new(
            r"fn\s+(\w+).*\{[^}]*(?:todo!|unimplemented!|return\s+.*(?:test|example|dummy))[^}]*\}"
        ).unwrap();
        
        for cap in stub_function_regex.captures_iter(content) {
            let func_name = &cap[1];
            let match_start = cap.get(0).unwrap().start();
            let line_num = content[..match_start].lines().count() + 1;
            
            alerts.push(FakeCodeAlert {
                alert_id: uuid::Uuid::new_v4().to_string(),
                crate_name: crate_name.to_string(),
                file_path: file_path.to_string(),
                alert_type: FakeCodeAlertType::StubImplementation,
                confidence: 0.80,
                evidence: vec![format!("Function '{}' appears to be a stub implementation", func_name)],
                line_numbers: vec![line_num],
                detected_at: Utc::now(),
                risk_level: RiskLevel::Medium,
            });
        }
        
        Ok(alerts)
    }
    
    pub fn print_fake_code_alerts(&self, alerts: &[FakeCodeAlert]) {
        if alerts.is_empty() {
            return;
        }
        
        println!("\n{}", "🚨 FAKE CODE ALERTS".bright_red().bold());
        println!("{}", "=".repeat(50).bright_red());
        
        for alert in alerts {
            let risk_color = match alert.risk_level {
                RiskLevel::Critical => "red",
                RiskLevel::High => "yellow",
                RiskLevel::Medium => "blue",
                RiskLevel::Low => "white",
            };
            
            println!("\n{} {} ({})", 
                "⚠️".bright_red(),
                format!("{:?}", alert.alert_type).color(risk_color).bold(),
                format!("{:.0}% confidence", alert.confidence * 100.0).bright_white()
            );
            
            println!("   📦 Crate: {}", alert.crate_name.bright_cyan());
            println!("   📄 File: {}", alert.file_path.bright_blue());
            println!("   🎯 Lines: {}", alert.line_numbers.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ")
                .bright_white());
            
            println!("   🔍 Evidence:");
            for evidence in &alert.evidence {
                println!("      {}", evidence.bright_yellow());
            }
            
            // Provide specific guidance
            match alert.alert_type {
                FakeCodeAlertType::SimpleVersionTrap => {
                    println!("   💡 {}", "RECOMMENDATION: Ask the model: 'Did you insert fake/hardcoded values?' before accepting this code.".bright_green());
                },
                FakeCodeAlertType::HardcodedCredentials => {
                    println!("   🔐 {}", "SECURITY RISK: Remove hardcoded credentials immediately!".bright_red().bold());
                },
                FakeCodeAlertType::SystematicFakeData => {
                    println!("   🧬 {}", "PATTERN DETECTED: Multiple fake values suggest systematic code generation issues.".bright_yellow());
                },
                _ => {
                    println!("   ✋ {}", "ACTION REQUIRED: Review and replace placeholder implementations.".bright_cyan());
                }
            }
        }
    }
    
    pub fn generate_fake_code_report(&self, all_alerts: &[FakeCodeAlert]) -> Result<String> {
        let mut report = String::new();
        
        report.push_str("# Fake Code Detection Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M:%S")));
        
        let critical_count = all_alerts.iter().filter(|a| matches!(a.risk_level, RiskLevel::Critical)).count();
        let high_count = all_alerts.iter().filter(|a| matches!(a.risk_level, RiskLevel::High)).count();
        
        report.push_str("## Summary\n\n");
        report.push_str(&format!("- 🔴 Critical Issues: {}\n", critical_count));
        report.push_str(&format!("- 🟡 High Risk Issues: {}\n", high_count));
        report.push_str(&format!("- 📊 Total Alerts: {}\n\n", all_alerts.len()));
        
        if critical_count > 0 {
            report.push_str("## 🚨 CRITICAL ISSUES (Immediate Action Required)\n\n");
            for alert in all_alerts.iter().filter(|a| matches!(a.risk_level, RiskLevel::Critical)) {
                report.push_str(&format!("### {} - {}\n", alert.crate_name, alert.file_path));
                report.push_str(&format!("**Type**: {:?}\n", alert.alert_type));
                report.push_str(&format!("**Confidence**: {:.0}%\n\n", alert.confidence * 100.0));
                report.push_str("**Evidence**:\n");
                for evidence in &alert.evidence {
                    report.push_str(&format!("- {}\n", evidence));
                }
                report.push_str("\n---\n\n");
            }
        }
        
        report.push_str("## Recommendations\n\n");
        report.push_str("1. **Stop and Ask**: When a model says 'let me create a simple version', immediately ask: 'Did you insert fake/hardcoded values?'\n");
        report.push_str("2. **Code Review**: All detected files should undergo immediate human review\n");
        report.push_str("3. **Pattern Training**: Use these violations to train better prompting patterns\n");
        report.push_str("4. **Monitoring**: Run this detector after every significant code generation session\n\n");
        
        Ok(report)
    }
}

// Add uuid dependency to Cargo.toml:
// uuid = { version = "1.0", features = ["v4"] }
