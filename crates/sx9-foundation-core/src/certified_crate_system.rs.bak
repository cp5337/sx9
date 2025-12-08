// src/certified_crate_manager.rs
use anyhow::Result;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertifiedCrate {
    pub name: String,
    pub version: String,
    pub certified_at: DateTime<Utc>,
    pub certification_hash: String,
    pub function_signatures: HashMap<String, FunctionSignature>,
    pub critical_logic_blocks: Vec<CriticalBlock>,
    pub line_count: usize,
    pub complexity_score: f64,
    pub docker_image_hash: String,
    pub certification_level: CertificationLevel,
    pub waiver_reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub name: String,
    pub params: Vec<String>,
    pub return_type: String,
    pub line_range: (usize, usize),
    pub complexity: f64,
    pub critical: bool, // Can this function break the system if modified?
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalBlock {
    pub block_id: String,
    pub description: String,
    pub line_range: (usize, usize),
    pub hash: String,
    pub must_preserve: bool, // Cannot be modified even for LOC compliance
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertificationLevel {
    Gold,     // <= 200 LOC, perfect compliance
    Silver,   // 201-300 LOC, meets desired target
    Bronze,   // 301+ LOC with technical waiver
    Provisional, // Temporary certification pending review
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringViolation {
    pub violation_type: ViolationType,
    pub severity: Severity,
    pub description: String,
    pub evidence: Vec<String>,
    pub original_functionality: String,
    pub suspected_cause: SuspectedCause,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationType {
    LogicGutting,           // Code logic removed to meet LOC limits
    RushedCompilationFix,   // Quick fixes that break logic
    WarningIgnoring,        // Ignoring compiler warnings
    FunctionHollowing,      // Function signatures preserved but logic removed
    CriticalBlockDamage,    // Protected code blocks modified
    ComplexityReduction,    // Artificial complexity reduction
    SignatureViolation,     // Function signatures changed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuspectedCause {
    LOCPressure,           // Model trying to meet 200 LOC limit
    CompilationRush,       // Model rushing to fix compile errors
    WarningAvoidance,      // Model trying to eliminate warnings
    RefactoringAggression, // Over-aggressive refactoring
    LogicConfusion,        // Model doesn't understand preserved logic
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Critical,   // System-breaking changes
    High,       // Logic compromised
    Medium,     // Functionality reduced
    Low,        // Style issues only
}

pub struct CertifiedCrateManager {
    certified_crates: HashMap<String, CertifiedCrate>,
    work_directories: Vec<PathBuf>,
    repo_root: PathBuf,
    violation_patterns: HashMap<String, Regex>,
    loc_thresholds: LOCThresholds,
}

#[derive(Debug, Clone)]
pub struct LOCThresholds {
    pub desired: usize,       // 200 - ideal target
    pub acceptable: usize,    // 300 - with justification
    pub waiver_required: usize, // 301+ - needs technical waiver
}

impl CertifiedCrateManager {
    pub fn new(repo_root: PathBuf) -> Self {
        let mut violation_patterns = HashMap::new();
        
        // Logic Gutting Patterns - when models remove actual logic
        violation_patterns.insert(
            "empty_function_bodies".to_string(),
            Regex::new(r"fn\s+\w+.*\{\s*(?://.*\n\s*)*(?:todo!|unimplemented!|return\s+.*)\s*\}").unwrap()
        );
        
        violation_patterns.insert(
            "removed_error_handling".to_string(),
            Regex::new(r"\.unwrap\(\)|\.expect\(").unwrap()
        );
        
        violation_patterns.insert(
            "simplified_match_arms".to_string(),
            Regex::new(r"match\s+\w+\s*\{\s*_\s*=>\s*(?:todo!|.*default.*)\s*\}").unwrap()
        );
        
        // Rushed Compilation Fix Patterns
        violation_patterns.insert(
            "quick_suppress_warnings".to_string(),
            Regex::new(r"#\[allow\(.*\)\]").unwrap()
        );
        
        violation_patterns.insert(
            "panic_instead_of_logic".to_string(),
            Regex::new(r"panic!\(|unreachable!").unwrap()
        );
        
        violation_patterns.insert(
            "cast_instead_of_convert".to_string(),
            Regex::new(r"as\s+\w+(?:\s*[;,\)])").unwrap()
        );
        
        // Warning Ignoring Patterns
        violation_patterns.insert(
            "unused_variable_ignore".to_string(),
            Regex::new(r"let\s+_\w+\s*=|#\[allow\(unused_variables\)\]").unwrap()
        );
        
        violation_patterns.insert(
            "dead_code_allow".to_string(),
            Regex::new(r"#\[allow\(dead_code\)\]").unwrap()
        );
        
        Self {
            certified_crates: HashMap::new(),
            work_directories: Vec::new(),
            repo_root,
            violation_patterns,
            loc_thresholds: LOCThresholds {
                desired: 200,
                acceptable: 300,
                waiver_required: 301,
            },
        }
    }
    
    pub fn certify_crate(&mut self, crate_path: &Path, waiver_reasons: Vec<String>) -> Result<CertifiedCrate> {
        let crate_name = crate_path.file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        
        println!("🏆 Certifying crate: {}", crate_name);
        
        // Analyze crate structure
        let content = self.read_crate_content(crate_path)?;
        let line_count = content.lines().count();
        
        // Extract function signatures
        let function_signatures = self.extract_function_signatures(&content)?;
        
        // Identify critical logic blocks
        let critical_blocks = self.identify_critical_blocks(&content)?;
        
        // Calculate complexity score
        let complexity_score = self.calculate_complexity(&content, &function_signatures);
        
        // Generate certification hash
        let certification_hash = self.generate_certification_hash(&content, &function_signatures, &critical_blocks);
        
        // Determine certification level
        let certification_level = self.determine_certification_level(line_count, &waiver_reasons);
        
        // Create Docker image hash (simulate)
        let docker_image_hash = self.create_docker_snapshot(&crate_name, crate_path)?;
        
        let certified_crate = CertifiedCrate {
            name: crate_name.clone(),
            version: "1.0.0".to_string(),
            certified_at: Utc::now(),
            certification_hash,
            function_signatures,
            critical_logic_blocks: critical_blocks,
            line_count,
            complexity_score,
            docker_image_hash,
            certification_level,
            waiver_reasons,
        };
        
        // Save certification
        self.certified_crates.insert(crate_name, certified_crate.clone());
        self.save_certification(&certified_crate)?;
        
        println!("✅ Certification complete: {:?} (LOC: {})", 
            certified_crate.certification_level, 
            line_count
        );
        
        Ok(certified_crate)
    }
    
    pub fn validate_against_certified(&self, crate_name: &str, modified_content: &str) -> Result<Vec<RefactoringViolation>> {
        let mut violations = Vec::new();
        
        let certified = self.certified_crates.get(crate_name)
            .ok_or_else(|| anyhow::anyhow!("Crate not certified: {}", crate_name))?;
        
        println!("🔍 Validating {} against certified version", crate_name);
        
        // Check for logic gutting
        violations.extend(self.detect_logic_gutting(certified, modified_content)?);
        
        // Check for rushed fixes
        violations.extend(self.detect_rushed_fixes(certified, modified_content)?);
        
        // Check critical block preservation
        violations.extend(self.validate_critical_blocks(certified, modified_content)?);
        
        // Check function signature preservation
        violations.extend(self.validate_function_signatures(certified, modified_content)?);
        
        // Check LOC compliance patterns
        violations.extend(self.check_loc_compliance_violations(certified, modified_content)?);
        
        Ok(violations)
    }
    
    fn detect_logic_gutting(&self, certified: &CertifiedCrate, content: &str) -> Result<Vec<RefactoringViolation>> {
        let mut violations = Vec::new();
        
        // Compare complexity - if it drops significantly, likely gutted
        let new_functions = self.extract_function_signatures(content)?;
        let current_complexity = self.calculate_complexity(content, &new_functions);
        
        let complexity_drop = certified.complexity_score - current_complexity;
        if complexity_drop > certified.complexity_score * 0.3 {  // 30% complexity drop
            violations.push(RefactoringViolation {
                violation_type: ViolationType::LogicGutting,
                severity: Severity::Critical,
                description: format!("Complexity dropped by {:.1}% (from {:.2} to {:.2})", 
                    (complexity_drop / certified.complexity_score) * 100.0,
                    certified.complexity_score, 
                    current_complexity
                ),
                evidence: vec![
                    format!("Original complexity: {:.2}", certified.complexity_score),
                    format!("Current complexity: {:.2}", current_complexity),
                    format!("Drop: {:.2}", complexity_drop),
                ],
                original_functionality: "Complex business logic".to_string(),
                suspected_cause: SuspectedCause::LOCPressure,
                detected_at: Utc::now(),
            });
        }
        
        // Look for empty function bodies that used to have logic
        for (func_name, orig_sig) in &certified.function_signatures {
            if orig_sig.complexity > 2.0 { // Was a complex function
                let empty_pattern = format!(r"fn\s+{}\s*\([^)]*\).*\{{\s*(?:todo!|unimplemented!|return\s+.*)\s*\}}", func_name);
                let regex = Regex::new(&empty_pattern).unwrap();
                
                if regex.is_match(content) {
                    violations.push(RefactoringViolation {
                        violation_type: ViolationType::FunctionHollowing,
                        severity: Severity::Critical,
                        description: format!("Function '{}' was gutted - complex logic removed", func_name),
                        evidence: vec![format!("Function {} now has trivial implementation", func_name)],
                        original_functionality: format!("Complex function with {:.2} complexity", orig_sig.complexity),
                        suspected_cause: SuspectedCause::LOCPressure,
                        detected_at: Utc::now(),
                    });
                }
            }
        }
        
        Ok(violations)
    }
    
    fn detect_rushed_fixes(&self, certified: &CertifiedCrate, content: &str) -> Result<Vec<RefactoringViolation>> {
        let mut violations = Vec::new();
        
        // Check for rushed compilation fix patterns
        for (pattern_name, regex) in &self.violation_patterns {
            let matches: Vec<_> = regex.find_iter(content).collect();
            
            if !matches.is_empty() && (pattern_name.contains("quick_") || pattern_name.contains("panic_") || pattern_name.contains("cast_")) {
                let suspected_cause = if pattern_name.contains("suppress") {
                    SuspectedCause::WarningAvoidance
                } else if pattern_name.contains("panic") {
                    SuspectedCause::CompilationRush
                } else {
                    SuspectedCause::RefactoringAggression
                };
                
                violations.push(RefactoringViolation {
                    violation_type: ViolationType::RushedCompilationFix,
                    severity: if pattern_name.contains("panic") { Severity::High } else { Severity::Medium },
                    description: format!("Detected rushed fix pattern: {}", pattern_name),
                    evidence: matches.iter().map(|m| m.as_str().to_string()).collect(),
                    original_functionality: "Proper error handling and type safety".to_string(),
                    suspected_cause,
                    detected_at: Utc::now(),
                });
            }
        }
        
        Ok(violations)
    }
    
    fn validate_critical_blocks(&self, certified: &CertifiedCrate, content: &str) -> Result<Vec<RefactoringViolation>> {
        let mut violations = Vec::new();
        
        for critical_block in &certified.critical_logic_blocks {
            if critical_block.must_preserve {
                let lines: Vec<&str> = content.lines().collect();
                
                // Extract the same line range from new content
                let start_line = critical_block.line_range.0.saturating_sub(1);
                let end_line = (critical_block.line_range.1).min(lines.len());
                
                if start_line < lines.len() && end_line > start_line {
                    let current_block = lines[start_line..end_line].join("\n");
                    let current_hash = self.hash_content(&current_block);
                    
                    if current_hash != critical_block.hash {
                        violations.push(RefactoringViolation {
                            violation_type: ViolationType::CriticalBlockDamage,
                            severity: Severity::Critical,
                            description: format!("Critical block '{}' was modified", critical_block.description),
                            evidence: vec![
                                format!("Lines {}-{}", critical_block.line_range.0, critical_block.line_range.1),
                                format!("Original hash: {}", critical_block.hash[..8].to_string()),
                                format!("Current hash: {}", current_hash[..8].to_string()),
                            ],
                            original_functionality: critical_block.description.clone(),
                            suspected_cause: SuspectedCause::LogicConfusion,
                            detected_at: Utc::now(),
                        });
                    }
                } else {
                    violations.push(RefactoringViolation {
                        violation_type: ViolationType::CriticalBlockDamage,
                        severity: Severity::Critical,
                        description: format!("Critical block '{}' was completely removed", critical_block.description),
                        evidence: vec![format!("Expected lines {}-{} not found", critical_block.line_range.0, critical_block.line_range.1)],
                        original_functionality: critical_block.description.clone(),
                        suspected_cause: SuspectedCause::LOCPressure,
                        detected_at: Utc::now(),
                    });
                }
            }
        }
        
        Ok(violations)
    }
    
    fn check_loc_compliance_violations(&self, certified: &CertifiedCrate, content: &str) -> Result<Vec<RefactoringViolation>> {
        let mut violations = Vec::new();
        
        let current_loc = content.lines().count();
        let original_loc = certified.line_count;
        
        // If LOC was reduced dramatically, check if it was done properly
        if original_loc > self.loc_thresholds.desired && current_loc <= self.loc_thresholds.desired {
            let reduction_ratio = (original_loc as f64 - current_loc as f64) / original_loc as f64;
            
            if reduction_ratio > 0.4 {  // More than 40% reduction
                // Check if the reduction was achieved through logic removal
                let new_functions = self.extract_function_signatures(content).unwrap_or_default();
                let current_complexity = self.calculate_complexity(content, &new_functions);
                let complexity_ratio = current_complexity / certified.complexity_score;
                
                if complexity_ratio < 0.7 {  // Complexity dropped more than 30%
                    violations.push(RefactoringViolation {
                        violation_type: ViolationType::LogicGutting,
                        severity: Severity::High,
                        description: format!(
                            "Suspicious LOC reduction: {}→{} lines ({:.1}% reduction) with {:.1}% complexity loss", 
                            original_loc, 
                            current_loc, 
                            reduction_ratio * 100.0,
                            (1.0 - complexity_ratio) * 100.0
                        ),
                        evidence: vec![
                            format!("Original LOC: {}", original_loc),
                            format!("Current LOC: {}", current_loc),
                            format!("Complexity ratio: {:.2}", complexity_ratio),
                        ],
                        original_functionality: "Full business logic implementation".to_string(),
                        suspected_cause: SuspectedCause::LOCPressure,
                        detected_at: Utc::now(),
                    });
                }
            }
        }
        
        Ok(violations)
    }
    
    // Helper methods for analyzing code
    fn extract_function_signatures(&self, content: &str) -> Result<HashMap<String, FunctionSignature>> {
        let mut functions = HashMap::new();
        let func_regex = Regex::new(r"(?m)^(?:\s*pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\(([^)]*)\)(?:\s*->\s*([^{]+))?\s*\{").unwrap();
        
        for (line_num, line) in content.lines().enumerate() {
            if let Some(caps) = func_regex.captures(line) {
                let func_name = caps[1].to_string();
                let params = caps.get(2).map_or(Vec::new(), |m| {
                    m.as_str().split(',')
                        .map(|p| p.trim().to_string())
                        .filter(|p| !p.is_empty())
                        .collect()
                });
                let return_type = caps.get(3).map_or("()".to_string(), |m| m.as_str().trim().to_string());
                
                // Calculate function complexity (simplified)
                let func_complexity = self.estimate_function_complexity(content, line_num);
                
                functions.insert(func_name.clone(), FunctionSignature {
                    name: func_name,
                    params,
                    return_type,
                    line_range: (line_num + 1, line_num + 1), // Will be updated with proper range
                    complexity: func_complexity,
                    critical: func_complexity > 3.0 || line_num < 50, // First 50 lines often critical
                });
            }
        }
        
        Ok(functions)
    }
    
    fn identify_critical_blocks(&self, content: &str) -> Result<Vec<CriticalBlock>> {
        let mut blocks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        // Look for blocks that should never be modified
        let critical_patterns = [
            (r"// CRITICAL:", "Critical section"),
            (r"// SECURITY:", "Security implementation"),
            (r"// INVARIANT:", "System invariant"),
            (r"unsafe\s*\{", "Unsafe block"),
            (r"match\s+.*\{.*\}", "Complex match logic"),
        ];
        
        for (i, line) in lines.iter().enumerate() {
            for (pattern, description) in &critical_patterns {
                let regex = Regex::new(pattern).unwrap();
                if regex.is_match(line) {
                    // Find the block extent (simplified)
                    let block_start = i + 1;
                    let block_end = (i + 10).min(lines.len()); // Take next 10 lines as block
                    
                    let block_content = lines[i..block_end].join("\n");
                    let block_hash = self.hash_content(&block_content);
                    
                    blocks.push(CriticalBlock {
                        block_id: format!("block_{}", i),
                        description: description.to_string(),
                        line_range: (block_start, block_end),
                        hash: block_hash,
                        must_preserve: description.contains("CRITICAL") || description.contains("SECURITY"),
                    });
                }
            }
        }
        
        Ok(blocks)
    }
    
    fn calculate_complexity(&self, content: &str, functions: &HashMap<String, FunctionSignature>) -> f64 {
        let mut total_complexity = 0.0;
        
        // Basic complexity metrics
        let line_count = content.lines().count() as f64;
        let if_count = content.matches("if ").count() as f64;
        let match_count = content.matches("match ").count() as f64;
        let loop_count = content.matches("for ").count() + content.matches("while ").count();
        
        total_complexity += line_count * 0.1;
        total_complexity += if_count * 1.5;
        total_complexity += match_count * 2.0;
        total_complexity += loop_count as f64 * 2.0;
        
        // Function-based complexity
        for func in functions.values() {
            total_complexity += func.complexity;
        }
        
        total_complexity
    }
    
    fn estimate_function_complexity(&self, content: &str, start_line: usize) -> f64 {
        let lines: Vec<&str> = content.lines().collect();
        let mut complexity = 1.0; // Base complexity
        
        // Look ahead ~20 lines to estimate function complexity
        let end_search = (start_line + 20).min(lines.len());
        
        for line in &lines[start_line..end_search] {
            if line.contains("if ") { complexity += 1.0; }
            if line.contains("match ") { complexity += 2.0; }
            if line.contains("for ") || line.contains("while ") { complexity += 1.5; }
            if line.contains("unsafe") { complexity += 3.0; }
            if line.contains("unwrap") { complexity += 0.5; }
        }
        
        complexity
    }
    
    fn determine_certification_level(&self, line_count: usize, waiver_reasons: &[String]) -> CertificationLevel {
        if line_count <= self.loc_thresholds.desired {
            CertificationLevel::Gold
        } else if line_count <= self.loc_thresholds.acceptable {
            CertificationLevel::Silver
        } else if !waiver_reasons.is_empty() {
            CertificationLevel::Bronze
        } else {
            CertificationLevel::Provisional
        }
    }
    
    fn create_docker_snapshot(&self, crate_name: &str, crate_path: &Path) -> Result<String> {
        // Create Docker image hash (simplified simulation)
        let dockerfile_content = format!(
            r#"FROM rust:alpine
COPY {} /app/src/
WORKDIR /app
RUN cargo build --release
ENTRYPOINT ["./target/release/{}"]
"#,
            crate_path.display(),
            crate_name
        );
        
        Ok(self.hash_content(&dockerfile_content))
    }
    
    fn hash_content(&self, content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        hex::encode(hasher.finalize())[..16].to_string() // First 16 chars
    }
    
    fn generate_certification_hash(&self, content: &str, functions: &HashMap<String, FunctionSignature>, blocks: &[CriticalBlock]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        
        // Include function signatures in hash
        for func in functions.values() {
            hasher.update(format!("{}{:?}{}", func.name, func.params, func.return_type).as_bytes());
        }
        
        // Include critical blocks in hash
        for block in blocks {
            hasher.update(block.hash.as_bytes());
        }
        
        hex::encode(hasher.finalize())[..32].to_string() // First 32 chars
    }
    
    fn read_crate_content(&self, crate_path: &Path) -> Result<String> {
        let main_file = crate_path.join("src").join("main.rs");
        if main_file.exists() {
            Ok(fs::read_to_string(main_file)?)
        } else {
            let lib_file = crate_path.join("src").join("lib.rs");
            Ok(fs::read_to_string(lib_file)?)
        }
    }
    
    fn save_certification(&self, certified: &CertifiedCrate) -> Result<()> {
        let cert_path = self.repo_root.join(".certifications").join(format!("{}.json", certified.name));
        
        std::fs::create_dir_all(cert_path.parent().unwrap())?;
        let json = serde_json::to_string_pretty(certified)?;
        std::fs::write(cert_path, json)?;
        
        Ok(())
    }
    
    pub fn print_refactoring_violations(&self, violations: &[RefactoringViolation]) {
        if violations.is_empty() {
            println!("\n{}", "✅ No refactoring violations detected".bright_green());
            return;
        }
        
        use colored::*;
        
        println!("\n{}", "⚠️  REFACTORING VIOLATIONS DETECTED".bright_red().bold());
        println!("{}", "=".repeat(50).bright_red());
        
        let critical_count = violations.iter().filter(|v| matches!(v.severity, Severity::Critical)).count();
        if critical_count > 0 {
            println!("\n{} {} {}",
                "🚨".bright_red(),
                "CRITICAL:".bright_red().bold(),
                format!("{} critical violations - logic may be compromised!", critical_count).bright_red()
            );
        }
        
        for violation in violations {
            let severity_color = match violation.severity {
                Severity::Critical => "red",
                Severity::High => "yellow",
                Severity::Medium => "blue", 
                Severity::Low => "white",
            };
            
            println!("\n{} {} ({:?})",
                "⚠️".bright_yellow(),
                format!("{:?}", violation.violation_type).color(severity_color).bold(),
                violation.suspected_cause
            );
            
            println!("   📋 {}", violation.description.bright_white());
            println!("   🔧 Original: {}", violation.original_functionality.bright_green());
            
            if !violation.evidence.is_empty() {
                println!("   🔍 Evidence:");
                for evidence in &violation.evidence {
                    println!("      • {}", evidence.bright_cyan());
                }
            }
            
            // Specific recommendations
            let recommendation = match violation.suspected_cause {
                SuspectedCause::LOCPressure => "Consider Bronze certification with waiver instead of gutting logic",
                SuspectedCause::CompilationRush => "Take time to understand compilation errors before fixing",
                SuspectedCause::WarningAvoidance => "Address warnings properly instead of suppressing them",
                SuspectedCause::RefactoringAggression => "Preserve existing working logic during refactoring", 
                SuspectedCause::LogicConfusion => "Review critical sections before making changes",
            };
            
            println!("   💡 {}", recommendation.bright_green());
        }
        
        println!("\n{}", "🔄 RECOVERY OPTIONS:".bright_blue().bold());
        println!("   1. Restore from certified Docker image");
        println!("   2. Revert to last certified version");
        println!("   3. Apply Bronze certification with technical waiver");
        println!("   4. Manual review and re-certification required");
    }
}
