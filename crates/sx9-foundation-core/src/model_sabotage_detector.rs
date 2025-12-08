// src/model_sabotage_detector.rs
use anyhow::Result;
use chrono::{DateTime, Utc, Duration};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SabotageAlert {
    pub alert_id: String,
    pub sabotage_type: SabotageType,
    pub confidence: f64,
    pub severity: SabotageSeverity,
    pub description: String,
    pub evidence: Vec<String>,
    pub affected_systems: Vec<String>,
    pub detection_method: String,
    pub timestamp: DateTime<Utc>,
    pub behavioral_pattern: BehavioralPattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SabotageType {
    QASystemSabotage,           // Deliberately broken QA systems
    TestEvasion,                // Code that passes tests but fails in reality
    LogicBombs,                 // Hidden failure conditions
    SubtleMisdirection,         // Plausible but wrong implementations
    SystematicWeakening,        // Gradual degradation over time
    FalsePositives,             // Making QA overly permissive
    ResultManipulation,         // Altering expected vs actual comparisons
    DependencyCorruption,       // Breaking critical dependencies
    ConfigurationSabotage,      // Subtle config changes that break systems
    TrustBoundaryViolation,     // Bypassing security/validation layers
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SabotageSeverity {
    Critical,   // System integrity compromised
    High,       // QA effectiveness severely reduced
    Medium,     // Subtle but consistent issues
    Low,        // Suspicious patterns detected
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BehavioralPattern {
    GradualDegradation,         // Quality slowly decreases over time
    SelectiveFailure,           // Only certain tests/cases affected
    PlausibleDeniability,       // Errors that look accidental
    SystematicWeakening,        // Consistent pattern of making things less robust
    AntiPattern,                // Implementing known bad practices
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QASystemProfile {
    pub name: String,
    pub file_path: PathBuf,
    pub expected_behavior: String,
    pub test_patterns: Vec<String>,
    pub validation_rules: Vec<ValidationRule>,
    pub last_known_good: DateTime<Utc>,
    pub modification_history: VecDeque<ModificationEvent>,
    pub trust_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_id: String,
    pub description: String,
    pub pattern: String,
    pub expected_outcome: String,
    pub last_validated: DateTime<Utc>,
    pub validation_count: u32,
    pub failure_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModificationEvent {
    pub timestamp: DateTime<Utc>,
    pub change_type: String,
    pub lines_affected: Vec<usize>,
    pub change_summary: String,
    pub impact_assessment: f64, // 0.0-1.0 risk score
}

pub struct ModelSabotageDetector {
    qa_systems: HashMap<String, QASystemProfile>,
    sabotage_patterns: HashMap<String, Regex>,
    behavioral_baselines: HashMap<String, BehavioralBaseline>,
    trust_thresholds: TrustThresholds,
}

#[derive(Debug, Clone)]
pub struct BehavioralBaseline {
    pub system_name: String,
    pub normal_test_pass_rate: f64,
    pub normal_response_time: f64,
    pub typical_patterns: Vec<String>,
    pub established_date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TrustThresholds {
    pub critical_trust_floor: f64,    // Below this = immediate alert
    pub degradation_rate_limit: f64,  // Max acceptable degradation per day
    pub false_positive_tolerance: f64, // Max acceptable false positive rate
}

impl ModelSabotageDetector {
    pub fn new() -> Self {
        let mut sabotage_patterns = HashMap::new();
        
        // QA System Sabotage Patterns
        sabotage_patterns.insert(
            "qa_always_pass".to_string(),
            Regex::new(r"(?:return\s+(?:true|Ok|pass|success))|(?:assert_eq!\([^,]+,\s*[^,]+\))").unwrap()
        );
        
        sabotage_patterns.insert(
            "qa_disabled_checks".to_string(),
            Regex::new(r"(?://\s*assert|//\s*expect|//\s*test_|//.*(?:TODO|FIXME).*test)").unwrap()
        );
        
        sabotage_patterns.insert(
            "qa_weak_validation".to_string(),
            Regex::new(r"(?:\.is_empty\(\)|\.len\(\)\s*>\s*0|if\s+true\s*\{)").unwrap()
        );
        
        sabotage_patterns.insert(
            "qa_exception_swallowing".to_string(),
            Regex::new(r"catch\s*\([^)]*\)\s*\{\s*(?://.*|/\*.*\*/|)\s*\}").unwrap()
        );
        
        sabotage_patterns.insert(
            "qa_result_manipulation".to_string(),
            Regex::new(r"(?:expected\s*=\s*actual|actual\s*=\s*expected|result\s*=\s*.*expected)").unwrap()
        );
        
        // Test Evasion Patterns  
        sabotage_patterns.insert(
            "conditional_behavior".to_string(),
            Regex::new(r"if\s+.*(?:test|debug|dev)\s*\{.*\}\s*else\s*\{").unwrap()
        );
        
        sabotage_patterns.insert(
            "environment_detection".to_string(),
            Regex::new(r"(?:env::var|getenv|std::env).*(?:test|debug|ci|qa)").unwrap()
        );
        
        sabotage_patterns.insert(
            "timing_dependent".to_string(),
            Regex::new(r"(?:sleep|delay|timeout).*(?:\d+|\w+_time)").unwrap()
        );
        
        // Logic Bomb Patterns
        sabotage_patterns.insert(
            "date_triggered".to_string(),
            Regex::new(r"if.*(?:date|time|now).*(?:[><=]|after|before)").unwrap()
        );
        
        sabotage_patterns.insert(
            "counter_triggered".to_string(),
            Regex::new(r"(?:static|global).*(?:count|counter|calls).*(?:[><=]|\+\+|--)").unwrap()
        );
        
        sabotage_patterns.insert(
            "hidden_conditions".to_string(),
            Regex::new(r"if\s+.*(?:rand|random|hash).*(?:[><=]|%|\s+\d+)").unwrap()
        );
        
        // Subtle Misdirection Patterns
        sabotage_patterns.insert(
            "off_by_one".to_string(),
            Regex::new(r"(?:[<>]=?)\s*(?:\w+\.len\(\)\s*[+-]\s*1|\w+\s*[+-]\s*1)").unwrap()
        );
        
        sabotage_patterns.insert(
            "wrong_comparison".to_string(),
            Regex::new(r"(?:[><=]=?)\s*(?:0\.99|1\.01|\d+\.00?1)").unwrap()
        );
        
        sabotage_patterns.insert(
            "inverted_logic".to_string(),
            Regex::new(r"!\s*(?:\w+\.is_valid\(\)|\w+\.check\(\)|validate_\w+\(\))").unwrap()
        );
        
        Self {
            qa_systems: HashMap::new(),
            sabotage_patterns,
            behavioral_baselines: HashMap::new(),
            trust_thresholds: TrustThresholds {
                critical_trust_floor: 0.3,
                degradation_rate_limit: 0.1,
                false_positive_tolerance: 0.05,
            },
        }
    }
    
    pub fn register_qa_system(&mut self, name: String, file_path: PathBuf, expected_behavior: String) -> Result<()> {
        let profile = QASystemProfile {
            name: name.clone(),
            file_path,
            expected_behavior,
            test_patterns: Vec::new(),
            validation_rules: Vec::new(),
            last_known_good: Utc::now(),
            modification_history: VecDeque::new(),
            trust_score: 1.0, // Start with full trust
        };
        
        self.qa_systems.insert(name, profile);
        Ok(())
    }
    
    pub fn analyze_qa_system(&mut self, qa_name: &str, content: &str) -> Result<Vec<SabotageAlert>> {
        let mut alerts = Vec::new();
        
        // Get QA system profile
        let qa_profile = self.qa_systems.get_mut(qa_name)
            .ok_or_else(|| anyhow::anyhow!("QA system not registered: {}", qa_name))?;
        
        // Check for direct sabotage patterns
        alerts.extend(self.detect_direct_sabotage(qa_name, content)?);
        
        // Check for behavioral anomalies
        alerts.extend(self.detect_behavioral_anomalies(qa_name, content)?);
        
        // Check for systematic weakening
        alerts.extend(self.detect_systematic_weakening(qa_name, content)?);
        
        // Check for trust boundary violations
        alerts.extend(self.detect_trust_violations(qa_name, content)?);
        
        // Update trust score based on findings
        self.update_trust_score(qa_name, &alerts);
        
        Ok(alerts)
    }
    
    fn detect_direct_sabotage(&self, qa_name: &str, content: &str) -> Result<Vec<SabotageAlert>> {
        let mut alerts = Vec::new();
        
        for (pattern_name, regex) in &self.sabotage_patterns {
            let matches: Vec<_> = regex.find_iter(content).collect();
            
            if !matches.is_empty() {
                let sabotage_type = self.classify_sabotage_type(pattern_name);
                let severity = self.assess_severity(pattern_name, matches.len());
                
                alerts.push(SabotageAlert {
                    alert_id: uuid::Uuid::new_v4().to_string(),
                    sabotage_type,
                    confidence: self.calculate_confidence(pattern_name, matches.len()),
                    severity,
                    description: format!("Detected {} in QA system {}", pattern_name, qa_name),
                    evidence: matches.iter().map(|m| m.as_str().to_string()).collect(),
                    affected_systems: vec![qa_name.to_string()],
                    detection_method: "pattern_matching".to_string(),
                    timestamp: Utc::now(),
                    behavioral_pattern: BehavioralPattern::PlausibleDeniability,
                });
            }
        }
        
        Ok(alerts)
    }
    
    fn detect_behavioral_anomalies(&self, qa_name: &str, content: &str) -> Result<Vec<SabotageAlert>> {
        let mut alerts = Vec::new();
        
        // Check for suspicious QA system modifications
        let suspicious_modifications = [
            r"(?i)//.*(?:temporary|quick fix|disable|skip)",     // Temporary disabling
            r"(?i)//.*(?:todo|fixme).*(?:test|qa|validate)",    // Deferred QA work  
            r"return\s+(?:Ok\(.*\)|true|pass)",                 // Always passing
            r"\.unwrap\(\)",                                    // Panic-prone code in QA
            r"panic!\(",                                        // Direct panics in QA
            r"unreachable!\(",                                  // Unreachable code
        ];
        
        for pattern in &suspicious_modifications {
            let regex = Regex::new(pattern).unwrap();
            let matches: Vec<_> = regex.find_iter(content).collect();
            
            if matches.len() > 2 {  // Multiple suspicious patterns = higher confidence
                alerts.push(SabotageAlert {
                    alert_id: uuid::Uuid::new_v4().to_string(),
                    sabotage_type: SabotageType::QASystemSabotage,
                    confidence: 0.8,
                    severity: SabotageSeverity::High,
                    description: format!("Multiple suspicious modifications in QA system {}", qa_name),
                    evidence: matches.iter().map(|m| m.as_str().to_string()).collect(),
                    affected_systems: vec![qa_name.to_string()],
                    detection_method: "behavioral_analysis".to_string(),
                    timestamp: Utc::now(),
                    behavioral_pattern: BehavioralPattern::SystematicWeakening,
                });
            }
        }
        
        Ok(alerts)
    }
    
    fn detect_systematic_weakening(&self, qa_name: &str, content: &str) -> Result<Vec<SabotageAlert>> {
        let mut alerts = Vec::new();
        
        // Look for patterns that systematically weaken QA effectiveness
        let weakening_indicators = [
            (r"assert_eq!\([^,]+,\s*[^,]+\)", "Weak equality assertions"),
            (r"\.is_empty\(\)", "Trivial emptiness checks"), 
            (r"\.len\(\)\s*>\s*0", "Trivial length checks"),
            (r"if\s+true\s*\{", "Always-true conditions"),
            (r"match\s+\w+\s*\{\s*_\s*=>\s*", "Catch-all matches"),
        ];
        
        let mut total_weakness_score = 0.0;
        let mut evidence = Vec::new();
        
        for (pattern, description) in &weakening_indicators {
            let regex = Regex::new(pattern).unwrap();
            let matches = regex.find_iter(content).count();
            
            if matches > 0 {
                total_weakness_score += matches as f64 * 0.2;
                evidence.push(format!("{}: {} instances", description, matches));
            }
        }
        
        if total_weakness_score > 1.0 {
            alerts.push(SabotageAlert {
                alert_id: uuid::Uuid::new_v4().to_string(),
                sabotage_type: SabotageType::SystematicWeakening,
                confidence: (total_weakness_score / 3.0).min(1.0),
                severity: if total_weakness_score > 2.0 { 
                    SabotageSeverity::High 
                } else { 
                    SabotageSeverity::Medium 
                },
                description: format!("Systematic weakening detected in QA system {} (weakness score: {:.2})", qa_name, total_weakness_score),
                evidence,
                affected_systems: vec![qa_name.to_string()],
                detection_method: "weakness_analysis".to_string(),
                timestamp: Utc::now(),
                behavioral_pattern: BehavioralPattern::SystematicWeakening,
            });
        }
        
        Ok(alerts)
    }
    
    fn detect_trust_violations(&self, qa_name: &str, content: &str) -> Result<Vec<SabotageAlert>> {
        let mut alerts = Vec::new();
        
        // Check for code that bypasses or undermines trust boundaries
        let trust_violations = [
            (r"(?i)//\s*(?:skip|ignore|bypass).*(?:validation|check|test)", "Bypassing validation"),
            (r"\.unwrap_or\(.*\)", "Defaulting on errors"),
            (r"catch.*\{\s*(?://|/\*|)\s*\}", "Silent error handling"),
            (r"if.*debug.*\{.*return.*ok", "Debug-only success paths"),
        ];
        
        for (pattern, violation_type) in &trust_violations {
            let regex = Regex::new(pattern).unwrap();
            let matches: Vec<_> = regex.find_iter(content).collect();
            
            if !matches.is_empty() {
                alerts.push(SabotageAlert {
                    alert_id: uuid::Uuid::new_v4().to_string(),
                    sabotage_type: SabotageType::TrustBoundaryViolation,
                    confidence: 0.75,
                    severity: SabotageSeverity::High,
                    description: format!("Trust boundary violation in {}: {}", qa_name, violation_type),
                    evidence: matches.iter().map(|m| m.as_str().to_string()).collect(),
                    affected_systems: vec![qa_name.to_string()],
                    detection_method: "trust_analysis".to_string(),
                    timestamp: Utc::now(),
                    behavioral_pattern: BehavioralPattern::PlausibleDeniability,
                });
            }
        }
        
        Ok(alerts)
    }
    
    fn classify_sabotage_type(&self, pattern_name: &str) -> SabotageType {
        match pattern_name {
            s if s.contains("qa_") => SabotageType::QASystemSabotage,
            s if s.contains("test_") => SabotageType::TestEvasion,
            s if s.contains("date_") || s.contains("counter_") => SabotageType::LogicBombs,
            s if s.contains("conditional_") => SabotageType::TestEvasion,
            s if s.contains("result_") => SabotageType::ResultManipulation,
            _ => SabotageType::SubtleMisdirection,
        }
    }
    
    fn assess_severity(&self, pattern_name: &str, match_count: usize) -> SabotageSeverity {
        let base_severity = match pattern_name {
            s if s.contains("qa_always_pass") => SabotageSeverity::Critical,
            s if s.contains("result_manipulation") => SabotageSeverity::Critical,
            s if s.contains("logic_bomb") => SabotageSeverity::High,
            s if s.contains("exception_swallow") => SabotageSeverity::High,
            _ => SabotageSeverity::Medium,
        };
        
        // Escalate severity based on frequency
        match (base_severity, match_count) {
            (SabotageSeverity::Medium, n) if n > 3 => SabotageSeverity::High,
            (SabotageSeverity::High, n) if n > 2 => SabotageSeverity::Critical,
            (severity, _) => severity,
        }
    }
    
    fn calculate_confidence(&self, pattern_name: &str, match_count: usize) -> f64 {
        let base_confidence = match pattern_name {
            s if s.contains("qa_always_pass") => 0.95,
            s if s.contains("result_manipulation") => 0.90,
            s if s.contains("disabled_checks") => 0.85,
            s if s.contains("weak_validation") => 0.70,
            _ => 0.60,
        };
        
        // Increase confidence with frequency, cap at 0.98
        (base_confidence + (match_count as f64 * 0.05)).min(0.98)
    }
    
    fn update_trust_score(&mut self, qa_name: &str, alerts: &[SabotageAlert]) {
        if let Some(qa_profile) = self.qa_systems.get_mut(qa_name) {
            let mut trust_reduction = 0.0;
            
            for alert in alerts {
                trust_reduction += match alert.severity {
                    SabotageSeverity::Critical => 0.5,
                    SabotageSeverity::High => 0.3,
                    SabotageSeverity::Medium => 0.15,
                    SabotageSeverity::Low => 0.05,
                };
            }
            
            qa_profile.trust_score = (qa_profile.trust_score - trust_reduction).max(0.0);
        }
    }
    
    pub fn print_sabotage_alerts(&self, alerts: &[SabotageAlert]) {
        if alerts.is_empty() {
            println!("\n{}", "✅ No sabotage patterns detected".bright_green());
            return;
        }
        
        println!("\n{}", "🚨 MODEL SABOTAGE DETECTION ALERTS".bright_red().bold());
        println!("{}", "=".repeat(60).bright_red());
        
        let critical_count = alerts.iter().filter(|a| matches!(a.severity, SabotageSeverity::Critical)).count();
        if critical_count > 0 {
            println!("\n{} {} {}", 
                "⚠️".bright_red(),
                "CRITICAL:".bright_red().bold(),
                format!("{} critical sabotage patterns detected!", critical_count).bright_red().bold()
            );
        }
        
        for alert in alerts {
            let severity_color = match alert.severity {
                SabotageSeverity::Critical => "red",
                SabotageSeverity::High => "yellow", 
                SabotageSeverity::Medium => "blue",
                SabotageSeverity::Low => "white",
            };
            
            println!("\n{} {} ({}% confidence)", 
                "🚨".bright_red(),
                format!("{:?}", alert.sabotage_type).color(severity_color).bold(),
                (alert.confidence * 100.0) as u8
            );
            
            println!("   📋 {}", alert.description.bright_white());
            println!("   🎯 Systems: {}", alert.affected_systems.join(", ").bright_cyan());
            println!("   📊 Pattern: {:?}", alert.behavioral_pattern);
            
            if !alert.evidence.is_empty() {
                println!("   🔍 Evidence:");
                for evidence in &alert.evidence {
                    println!("      • {}", evidence.bright_yellow());
                }
            }
            
            // Provide specific recommendations based on sabotage type
            let recommendation = match alert.sabotage_type {
                SabotageType::QASystemSabotage => "IMMEDIATE ACTION: Isolate QA system in Docker/WASM container",
                SabotageType::TestEvasion => "REVIEW: Check for environment-dependent behavior",
                SabotageType::LogicBombs => "CRITICAL: Remove time/counter-based conditions immediately",
                SabotageType::ResultManipulation => "VERIFY: Compare expected vs actual result handling",
                SabotageType::SystematicWeakening => "AUDIT: Full review of validation logic required",
                SabotageType::TrustBoundaryViolation => "SECURITY: Review all trust boundaries and validation",
                _ => "INVESTIGATE: Manual code review recommended",
            };
            
            println!("   💡 {}", recommendation.bright_green());
        }
        
        println!("\n{}", "🛡️ RECOMMENDED IMMEDIATE ACTIONS:".bright_blue().bold());
        println!("   1. Isolate affected QA systems in containers (Docker/WASM)");
        println!("   2. Restore from last known good version");
        println!("   3. Implement continuous monitoring");
        println!("   4. Review all recent model-generated code");
        println!("   5. Establish manual verification checkpoints");
    }
    
    pub fn generate_sabotage_report(&self, alerts: &[SabotageAlert]) -> Result<String> {
        let mut report = String::new();
        
        report.push_str("# Model Sabotage Detection Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        
        let critical_count = alerts.iter().filter(|a| matches!(a.severity, SabotageSeverity::Critical)).count();
        let high_count = alerts.iter().filter(|a| matches!(a.severity, SabotageSeverity::High)).count();
        
        report.push_str("## Executive Summary\n\n");
        
        if critical_count > 0 {
            report.push_str(&format!("🚨 **CRITICAL ALERT**: {} critical sabotage patterns detected\n\n", critical_count));
            report.push_str("**IMMEDIATE ACTIONS REQUIRED:**\n");
            report.push_str("1. Isolate all affected QA systems immediately\n");
            report.push_str("2. Revert to containerized/WASM QA implementations\n");
            report.push_str("3. Conduct full security audit\n\n");
        }
        
        report.push_str(&format!("- 🔴 Critical Issues: {}\n", critical_count));
        report.push_str(&format!("- 🟡 High Risk Issues: {}\n", high_count));
        report.push_str(&format!("- 📊 Total Alerts: {}\n\n", alerts.len()));
        
        report.push_str("## Trust Scores\n\n");
        for (qa_name, profile) in &self.qa_systems {
            let trust_status = if profile.trust_score < self.trust_thresholds.critical_trust_floor {
                "🚨 CRITICAL"
            } else if profile.trust_score < 0.7 {
                "⚠️ DEGRADED"
            } else {
                "✅ TRUSTED"
            };
            
            report.push_str(&format!("- **{}**: {:.1}% {}\n", 
                qa_name, 
                profile.trust_score * 100.0, 
                trust_status
            ));
        }
        
        report.push_str("\n## Detailed Findings\n\n");
        for alert in alerts {
            report.push_str(&format!("### {} - {:?}\n", alert.affected_systems.join(", "), alert.sabotage_type));
            report.push_str(&format!("**Confidence**: {:.0}%  \n", alert.confidence * 100.0));
            report.push_str(&format!("**Severity**: {:?}  \n", alert.severity));
            report.push_str(&format!("**Description**: {}  \n\n", alert.description));
            
            if !alert.evidence.is_empty() {
                report.push_str("**Evidence**:\n");
                for evidence in &alert.evidence {
                    report.push_str(&format!("- `{}`\n", evidence));
                }
                report.push_str("\n");
            }
            
            report.push_str("---\n\n");
        }
        
        report.push_str("## Recommendations\n\n");
        report.push_str("### Immediate (0-24 hours)\n");
        report.push_str("1. **Container Isolation**: Move all QA systems to Docker/WASM containers\n");
        report.push_str("2. **Rollback**: Revert to last known good versions\n");
        report.push_str("3. **Monitoring**: Implement continuous sabotage detection\n\n");
        
        report.push_str("### Short Term (1-7 days)\n");
        report.push_str("1. **Code Audit**: Manual review of all flagged systems\n");
        report.push_str("2. **Baseline Reset**: Establish new behavioral baselines\n");
        report.push_str("3. **Process Changes**: Implement mandatory human review\n\n");
        
        report.push_str("### Long Term (1-4 weeks)\n");
        report.push_str("1. **Architecture Review**: Evaluate AI-generated code processes\n");
        report.push_str("2. **Training**: Team education on sabotage detection\n");
        report.push_str("3. **Automated Defenses**: Deploy continuous monitoring\n\n");
        
        Ok(report)
    }
    
    pub fn get_trust_scores(&self) -> HashMap<String, f64> {
        self.qa_systems.iter()
            .map(|(name, profile)| (name.clone(), profile.trust_score))
            .collect()
    }
}

// Add uuid dependency to Cargo.toml:
// uuid = { version = "1.0", features = ["v4"] }
