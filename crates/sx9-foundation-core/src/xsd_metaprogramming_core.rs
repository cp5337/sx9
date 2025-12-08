//! XSD Metaprogramming Core - Dynamic Code Generation Engine
//! 
//! This module provides the core XSD-driven metaprogramming capabilities
//! for dynamic code generation, template processing, and automated
//! code synthesis.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{info, warn, error};

/// Metaprogramming rule for code generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaProgrammingRule {
    pub rule_id: String,
    pub trigger_pattern: String,
    pub xsd_schema_ref: String,
    pub generation_template: String,
    pub lisp_function: Option<String>,
    pub rdf_metadata: Option<String>,
    pub priority: u8,
    pub enabled: bool,
}

/// Code generation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenerationContext {
    pub source_file: PathBuf,
    pub target_file: PathBuf,
    pub comment_analysis: CommentAnalysis,
    pub noun_verb_patterns: Vec<NounVerbPattern>,
    pub context_variables: HashMap<String, String>,
    pub generation_mode: GenerationMode,
}

/// Comment analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentAnalysis {
    pub total_comments: usize,
    pub code_comments: usize,
    pub documentation_comments: usize,
    pub comment_density: f64,
    pub comment_patterns: Vec<String>,
    pub extracted_entities: Vec<String>,
}

/// Noun-verb pattern extracted from comments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NounVerbPattern {
    pub subject: String,
    pub verb: String,
    pub object: Option<String>,
    pub confidence: f64,
    pub context: String,
}

/// Generated code result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedCode {
    pub rule_id: String,
    pub generated_content: String,
    pub target_path: PathBuf,
    pub metadata: HashMap<String, String>,
    pub validation_status: ValidationStatus,
}

/// Generation mode for code synthesis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerationMode {
    TemplateBased,
    PatternBased,
    AIGenerated,
    Hybrid,
}

/// Validation status for generated code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Pending,
    Validated,
    Failed(String),
    Skipped,
}

/// Comment density analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentDensityReport {
    pub file_path: PathBuf,
    pub total_lines: usize,
    pub comment_lines: usize,
    pub code_lines: usize,
    pub density_percentage: f64,
    pub quality_score: f64,
    pub recommendations: Vec<String>,
}

/// XSD Metaprogramming Engine
pub struct XSDMetaProgrammingEngine {
    rules: HashMap<String, MetaProgrammingRule>,
    templates: HashMap<String, String>,
    context_cache: HashMap<String, CodeGenerationContext>,
}

impl XSDMetaProgrammingEngine {
    /// Create a new XSD metaprogramming engine
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            templates: HashMap::new(),
            context_cache: HashMap::new(),
        }
    }

    /// Add a metaprogramming rule
    pub fn add_rule(&mut self, rule: MetaProgrammingRule) {
        info!("📝 Adding metaprogramming rule: {}", rule.rule_id);
        self.rules.insert(rule.rule_id.clone(), rule);
    }

    /// Generate code based on context
    pub fn generate_code(&self, context: &CodeGenerationContext) -> Vec<GeneratedCode> {
        let mut results = Vec::new();
        
        for pattern in &context.noun_verb_patterns {
            if let Some(rule) = self.find_matching_rule(pattern) {
                let generated = self.apply_rule(rule, pattern, context);
                results.push(generated);
            }
        }
        
        info!("🚀 Generated {} code artifacts", results.len());
        results
    }

    /// Find matching rule for a pattern
    fn find_matching_rule(&self, pattern: &NounVerbPattern) -> Option<&MetaProgrammingRule> {
        for rule in self.rules.values() {
            if !rule.enabled {
                continue;
            }
            
            if pattern.subject.to_lowercase().contains(&rule.trigger_pattern.to_lowercase()) ||
               pattern.verb.to_lowercase().contains(&rule.trigger_pattern.to_lowercase()) {
                return Some(rule);
            }
        }
        None
    }

    /// Apply a rule to generate code
    fn apply_rule(&self, rule: &MetaProgrammingRule, pattern: &NounVerbPattern, context: &CodeGenerationContext) -> GeneratedCode {
        let mut template = rule.generation_template.clone();
        
        // Replace placeholders with actual values
        template = template.replace("{subject}", &pattern.subject);
        template = template.replace("{verb}", &pattern.verb);
        if let Some(object) = &pattern.object {
            template = template.replace("{object}", object);
        }
        
        // Apply context variables
        for (key, value) in &context.context_variables {
            template = template.replace(&format!("{{{}}}", key), value);
        }
        
        GeneratedCode {
            rule_id: rule.rule_id.clone(),
            generated_content: template,
            target_path: context.target_file.clone(),
            metadata: HashMap::new(),
            validation_status: ValidationStatus::Pending,
        }
    }

    /// Analyze comment density in code
    pub fn analyze_comment_density(&self, analysis: &CommentAnalysis) -> CommentDensityReport {
        let total_lines = analysis.total_comments + analysis.code_comments;
        let density_percentage = if total_lines > 0 {
            (analysis.total_comments as f64 / total_lines as f64) * 100.0
        } else {
            0.0
        };
        
        let quality_score = self.calculate_quality_score(analysis);
        let recommendations = self.generate_recommendations(analysis);
        
        CommentDensityReport {
            file_path: PathBuf::new(), // Will be set by caller
            total_lines,
            comment_lines: analysis.total_comments,
            code_lines: analysis.code_comments,
            density_percentage,
            quality_score,
            recommendations,
        }
    }

    /// Calculate quality score for comment analysis
    fn calculate_quality_score(&self, analysis: &CommentAnalysis) -> f64 {
        let mut score = 0.0;
        
        // Factor in comment density (optimal range: 15-30%)
        let density = analysis.comment_density;
        if density >= 15.0 && density <= 30.0 {
            score += 40.0;
        } else if density > 0.0 {
            score += 20.0;
        }
        
        // Factor in documentation comments
        if analysis.documentation_comments > 0 {
            score += 30.0;
        }
        
        // Factor in pattern diversity
        let pattern_diversity = analysis.comment_patterns.len() as f64;
        score += (pattern_diversity * 5.0).min(30.0);
        
        score
    }

    /// Generate recommendations based on analysis
    fn generate_recommendations(&self, analysis: &CommentAnalysis) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if analysis.comment_density < 10.0 {
            recommendations.push("Consider adding more comments to improve code documentation".to_string());
        } else if analysis.comment_density > 40.0 {
            recommendations.push("Consider reducing comment density - code may be over-documented".to_string());
        }
        
        if analysis.documentation_comments == 0 {
            recommendations.push("Add documentation comments for public APIs and complex functions".to_string());
        }
        
        if analysis.comment_patterns.len() < 3 {
            recommendations.push("Diversify comment patterns for better code understanding".to_string());
        }
        
        recommendations
    }

    /// Get all registered rules
    pub fn get_rules(&self) -> Vec<&MetaProgrammingRule> {
        self.rules.values().collect()
    }

    /// Enable or disable a rule
    pub fn set_rule_enabled(&mut self, rule_id: &str, enabled: bool) -> bool {
        if let Some(rule) = self.rules.get_mut(rule_id) {
            rule.enabled = enabled;
            info!("🔧 Rule {} {}", rule_id, if enabled { "enabled" } else { "disabled" });
            true
        } else {
            warn!("⚠️ Rule not found: {}", rule_id);
            false
        }
    }

    /// Clear all rules
    pub fn clear_rules(&mut self) {
        info!("🗑️ Clearing all metaprogramming rules");
        self.rules.clear();
    }
}

impl Default for XSDMetaProgrammingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = XSDMetaProgrammingEngine::new();
        assert_eq!(engine.rules.len(), 0);
    }

    #[test]
    fn test_rule_addition() {
        let mut engine = XSDMetaProgrammingEngine::new();
        let rule = MetaProgrammingRule {
            rule_id: "test_rule".to_string(),
            trigger_pattern: "test".to_string(),
            xsd_schema_ref: "test.xsd".to_string(),
            generation_template: "pub struct {subject} {{ /* {verb} */ }}".to_string(),
            lisp_function: None,
            rdf_metadata: None,
            priority: 1,
            enabled: true,
        };
        
        engine.add_rule(rule);
        assert_eq!(engine.rules.len(), 1);
    }

    #[test]
    fn test_comment_density_analysis() {
        let engine = XSDMetaProgrammingEngine::new();
        let analysis = CommentAnalysis {
            total_comments: 10,
            code_comments: 90,
            documentation_comments: 5,
            comment_density: 10.0,
            comment_patterns: vec!["//".to_string(), "/*".to_string()],
            extracted_entities: vec!["User".to_string(), "create".to_string()],
        };
        
        let report = engine.analyze_comment_density(&analysis);
        assert_eq!(report.total_lines, 100);
        assert_eq!(report.comment_lines, 10);
        assert_eq!(report.density_percentage, 10.0);
    }
}
