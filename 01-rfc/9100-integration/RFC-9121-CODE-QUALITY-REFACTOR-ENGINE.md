# RFC-9121: Code Quality & Refactor Engine (Lightning QA)

**Status:** DRAFT  
**Author:** Charles E. Payne / Claude  
**Date:** 2025-12-20  
**Depends On:** RFC-9101 (SmartCrate), RFC-9120 (Prompt Forge v4), TETH Framework

---

## Abstract

RFC-9121 specifies the Code Quality and Refactor Engine — the QA gate that validates factory agent output before registry acceptance. The engine combines deterministic static analysis (AST parsing, metrics calculation) with TETH-derived anti-pattern detection to produce a quality grade. Crates scoring below Grade A (85+) trigger automatic refactoring directives. The engine operates without AI inference for core validation — it's rule-based, deterministic, and fast.

**Core Insight:** Code quality is measurable. Anti-patterns are detectable. Refactoring is directional. The engine enforces all three.

---

## 1. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    LIGHTNING QA ENGINE (RFC-9121)                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                         INPUT: CRATE PATH                            │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│         │                                                                   │
│         ▼                                                                   │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                  │
│  │     AST      │    │   METRICS    │    │    TETH      │                  │
│  │   PARSER     │───▶│  CALCULATOR  │───▶│  DETECTOR    │                  │
│  │  (syn/tree)  │    │  (Complexity)│    │(Anti-Pattern)│                  │
│  └──────────────┘    └──────────────┘    └──────────────┘                  │
│         │                   │                   │                           │
│         ▼                   ▼                   ▼                           │
│  ┌─────────────────────────────────────────────────────────────┐           │
│  │                    QUALITY AGGREGATOR                        │           │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐           │           │
│  │  │Structure│ │Complexity│ │Patterns │ │Security │           │           │
│  │  │  Score  │ │  Score  │ │  Score  │ │  Score  │           │           │
│  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘           │           │
│  └─────────────────────────────────────────────────────────────┘           │
│         │                                                                   │
│         ▼                                                                   │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                  │
│  │    GRADE     │    │  REFACTOR    │    │   OUTPUT     │                  │
│  │  CALCULATOR  │───▶│  DIRECTIVE   │───▶│  GENERATOR   │                  │
│  │   (A-F)      │    │  GENERATOR   │    │   (JSON)     │                  │
│  └──────────────┘    └──────────────┘    └──────────────┘                  │
│         │                                                                   │
│         ▼                                                                   │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  DECISION: Grade A (85+) → PASS    |    Grade B-F → REFACTOR/HALT   │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Quality Dimensions

### 2.1 Dimension Weights

| Dimension | Weight | Measures |
|-----------|--------|----------|
| **Structure** | 0.25 | File organization, line counts, module cohesion |
| **Complexity** | 0.25 | Cyclomatic complexity, cognitive load, nesting depth |
| **Patterns** | 0.25 | Design pattern adherence, anti-pattern absence |
| **Security** | 0.25 | Unsafe blocks, input validation, error handling |

### 2.2 Grade Thresholds

| Grade | Score Range | Action |
|-------|-------------|--------|
| **A** | 85 - 100 | PASS: Registry accepts |
| **B** | 70 - 84 | REFACTOR: Auto-fix possible, retry |
| **C** | 55 - 69 | REFACTOR: Significant rework needed |
| **D** | 40 - 54 | HALT: Major architectural issues |
| **F** | 0 - 39 | REJECT: Fundamental violations |

---

## 3. AST Parser Module

Parses Rust/Python source into analyzable syntax trees.

### 3.1 Rust Parser (syn-based)

```rust
use syn::{parse_file, File, Item, ItemFn, ItemImpl, ItemStruct};

pub struct ParsedCrate {
    pub files: Vec<ParsedFile>,
    pub total_lines: usize,
    pub total_functions: usize,
    pub total_structs: usize,
    pub total_impls: usize,
}

pub struct ParsedFile {
    pub path: PathBuf,
    pub line_count: usize,
    pub functions: Vec<FunctionInfo>,
    pub structs: Vec<StructInfo>,
    pub impls: Vec<ImplInfo>,
    pub imports: Vec<ImportInfo>,
}

pub struct FunctionInfo {
    pub name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub line_count: usize,
    pub is_async: bool,
    pub is_public: bool,
    pub param_count: usize,
    pub return_type: Option<String>,
    pub attributes: Vec<String>,
}

pub struct StructInfo {
    pub name: String,
    pub field_count: usize,
    pub is_public: bool,
    pub derives: Vec<String>,
}

pub struct ImplInfo {
    pub target: String,
    pub method_count: usize,
    pub trait_impl: Option<String>,
}

pub fn parse_crate(path: &Path) -> Result<ParsedCrate, ParseError> {
    let mut parsed = ParsedCrate::default();
    
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension() == Some(OsStr::new("rs")) {
            let content = fs::read_to_string(entry.path())?;
            let syntax = parse_file(&content)?;
            
            let file_info = extract_file_info(entry.path(), &syntax);
            parsed.total_lines += file_info.line_count;
            parsed.total_functions += file_info.functions.len();
            parsed.total_structs += file_info.structs.len();
            parsed.total_impls += file_info.impls.len();
            parsed.files.push(file_info);
        }
    }
    
    Ok(parsed)
}
```

### 3.2 Python Parser (tree-sitter based)

```python
import tree_sitter_python as tspython
from tree_sitter import Language, Parser

class PythonParser:
    def __init__(self):
        self.parser = Parser(Language(tspython.language()))
    
    def parse_file(self, path: Path) -> ParsedFile:
        content = path.read_text()
        tree = self.parser.parse(content.encode())
        
        return ParsedFile(
            path=path,
            line_count=content.count('\n') + 1,
            functions=self._extract_functions(tree.root_node),
            classes=self._extract_classes(tree.root_node),
            imports=self._extract_imports(tree.root_node),
        )
    
    def _extract_functions(self, node) -> List[FunctionInfo]:
        functions = []
        for child in node.children:
            if child.type == 'function_definition':
                functions.append(FunctionInfo(
                    name=self._get_function_name(child),
                    line_start=child.start_point[0],
                    line_end=child.end_point[0],
                    line_count=child.end_point[0] - child.start_point[0] + 1,
                    param_count=self._count_params(child),
                    is_async=child.children[0].type == 'async',
                ))
        return functions
```

---

## 4. Metrics Calculator

### 4.1 Structure Metrics

```rust
pub struct StructureMetrics {
    // File-level
    pub files_over_limit: Vec<FileViolation>,
    pub avg_file_lines: f64,
    pub max_file_lines: usize,
    
    // Function-level
    pub functions_over_limit: Vec<FunctionViolation>,
    pub avg_function_lines: f64,
    pub max_function_lines: usize,
    
    // Module-level
    pub module_cohesion: f64,  // 0.0 - 1.0
    pub dependency_count: usize,
    
    // Score
    pub score: f64,  // 0 - 100
}

pub fn calculate_structure_metrics(
    parsed: &ParsedCrate,
    limits: &StructureLimits,
) -> StructureMetrics {
    let mut metrics = StructureMetrics::default();
    
    // File violations
    for file in &parsed.files {
        if file.line_count > limits.max_lines_per_file {
            metrics.files_over_limit.push(FileViolation {
                path: file.path.clone(),
                lines: file.line_count,
                limit: limits.max_lines_per_file,
                excess: file.line_count - limits.max_lines_per_file,
            });
        }
    }
    
    // Function violations
    for file in &parsed.files {
        for func in &file.functions {
            if func.line_count > limits.max_lines_per_function {
                metrics.functions_over_limit.push(FunctionViolation {
                    path: file.path.clone(),
                    function: func.name.clone(),
                    lines: func.line_count,
                    limit: limits.max_lines_per_function,
                });
            }
        }
    }
    
    // Calculate averages
    metrics.avg_file_lines = parsed.total_lines as f64 / parsed.files.len() as f64;
    metrics.max_file_lines = parsed.files.iter().map(|f| f.line_count).max().unwrap_or(0);
    
    // Score calculation
    let file_violation_penalty = metrics.files_over_limit.len() as f64 * 10.0;
    let func_violation_penalty = metrics.functions_over_limit.len() as f64 * 5.0;
    metrics.score = (100.0 - file_violation_penalty - func_violation_penalty).max(0.0);
    
    metrics
}
```

### 4.2 Complexity Metrics (McCabe + Cognitive)

```rust
pub struct ComplexityMetrics {
    // Per-function
    pub function_complexity: Vec<FunctionComplexity>,
    
    // Aggregates
    pub avg_cyclomatic: f64,
    pub max_cyclomatic: usize,
    pub avg_cognitive: f64,
    pub max_cognitive: usize,
    pub avg_nesting_depth: f64,
    pub max_nesting_depth: usize,
    
    // Violations
    pub complexity_violations: Vec<ComplexityViolation>,
    
    // Score
    pub score: f64,
}

pub struct FunctionComplexity {
    pub name: String,
    pub file: PathBuf,
    pub cyclomatic: usize,      // McCabe complexity
    pub cognitive: usize,        // Cognitive complexity
    pub nesting_depth: usize,    // Max nesting level
    pub param_count: usize,
}

/// McCabe Cyclomatic Complexity
/// CC = E - N + 2P where:
///   E = edges in control flow graph
///   N = nodes in control flow graph
///   P = connected components (usually 1)
/// 
/// Simplified: CC = 1 + decision_points
/// Decision points: if, else if, while, for, match arm, &&, ||, ?
pub fn calculate_cyclomatic(func: &FunctionInfo, ast: &syn::ItemFn) -> usize {
    let mut complexity = 1; // Base complexity
    
    // Walk the AST and count decision points
    struct ComplexityVisitor { count: usize }
    
    impl<'ast> Visit<'ast> for ComplexityVisitor {
        fn visit_expr_if(&mut self, node: &'ast syn::ExprIf) {
            self.count += 1;
            syn::visit::visit_expr_if(self, node);
        }
        
        fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
            self.count += node.arms.len().saturating_sub(1);
            syn::visit::visit_expr_match(self, node);
        }
        
        fn visit_expr_while(&mut self, node: &'ast syn::ExprWhile) {
            self.count += 1;
            syn::visit::visit_expr_while(self, node);
        }
        
        fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
            self.count += 1;
            syn::visit::visit_expr_for_loop(self, node);
        }
        
        fn visit_expr_binary(&mut self, node: &'ast syn::ExprBinary) {
            match node.op {
                syn::BinOp::And(_) | syn::BinOp::Or(_) => self.count += 1,
                _ => {}
            }
            syn::visit::visit_expr_binary(self, node);
        }
        
        fn visit_expr_try(&mut self, node: &'ast syn::ExprTry) {
            self.count += 1;
            syn::visit::visit_expr_try(self, node);
        }
    }
    
    let mut visitor = ComplexityVisitor { count: 0 };
    syn::visit::visit_item_fn(&mut visitor, ast);
    
    complexity + visitor.count
}

/// Cognitive Complexity (Sonar-style)
/// Adds penalties for:
/// - Nesting (exponential with depth)
/// - Breaks in linear flow (else, catch, continue, break)
/// - Recursion
pub fn calculate_cognitive(func: &FunctionInfo, ast: &syn::ItemFn) -> usize {
    struct CognitiveVisitor {
        complexity: usize,
        nesting: usize,
    }
    
    impl<'ast> Visit<'ast> for CognitiveVisitor {
        fn visit_expr_if(&mut self, node: &'ast syn::ExprIf) {
            self.complexity += 1 + self.nesting; // Base + nesting penalty
            self.nesting += 1;
            syn::visit::visit_expr_if(self, node);
            self.nesting -= 1;
            
            // Else branches add cognitive load
            if node.else_branch.is_some() {
                self.complexity += 1;
            }
        }
        
        fn visit_expr_loop(&mut self, node: &'ast syn::ExprLoop) {
            self.complexity += 1 + self.nesting;
            self.nesting += 1;
            syn::visit::visit_expr_loop(self, node);
            self.nesting -= 1;
        }
        
        fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
            self.complexity += 1 + self.nesting;
            self.nesting += 1;
            // Each arm adds to cognitive load
            self.complexity += node.arms.len().saturating_sub(1);
            syn::visit::visit_expr_match(self, node);
            self.nesting -= 1;
        }
        
        fn visit_expr_break(&mut self, _: &'ast syn::ExprBreak) {
            self.complexity += 1; // Break in linear flow
        }
        
        fn visit_expr_continue(&mut self, _: &'ast syn::ExprContinue) {
            self.complexity += 1; // Break in linear flow
        }
    }
    
    let mut visitor = CognitiveVisitor { complexity: 0, nesting: 0 };
    syn::visit::visit_item_fn(&mut visitor, ast);
    
    visitor.complexity
}
```

### 4.3 Complexity Thresholds

```toml
# Complexity limits from birth certificate or defaults

[complexity.thresholds]
cyclomatic_warning = 10      # Yellow flag
cyclomatic_error = 15        # Red flag
cognitive_warning = 15       # Yellow flag  
cognitive_error = 25         # Red flag
nesting_depth_max = 4        # Hard limit
param_count_max = 7          # "7 ± 2" rule
```

---

## 5. TETH Anti-Pattern Detector

TETH (Tool Entropy Testing Harness) identifies structural anti-patterns.

### 5.1 Anti-Pattern Registry

```rust
pub enum AntiPattern {
    // Structural
    GodObject { struct_name: String, field_count: usize, method_count: usize },
    SpaghettiDeps { circular: Vec<(String, String)>, depth: usize },
    LongParameterList { function: String, param_count: usize },
    DataClump { fields: Vec<String>, occurrences: usize },
    
    // Async-specific
    AsyncMutexAbuse { location: String, mutex_type: String },
    UnboundedChannel { location: String },
    BlockingInAsync { location: String, blocking_call: String },
    
    // Error handling
    PanicInLibrary { location: String },
    UnwrapChain { location: String, chain_length: usize },
    SilentErrorDrop { location: String },
    
    // Resource management
    LeakyAbstraction { location: String },
    ResourceLeak { resource_type: String, location: String },
    
    // Code smells
    DeadCode { items: Vec<String> },
    DuplicateCode { locations: Vec<(String, String)>, similarity: f64 },
    MagicNumbers { locations: Vec<(String, i64)> },
}

pub struct AntiPatternResult {
    pub pattern: AntiPattern,
    pub severity: Severity,
    pub location: Location,
    pub remediation: String,
}

pub enum Severity {
    Critical,  // -20 points, blocks merge
    High,      // -10 points
    Medium,    // -5 points
    Low,       // -2 points
    Info,      // 0 points, advisory only
}
```

### 5.2 Detection Rules

```rust
pub struct TethDetector {
    rules: Vec<Box<dyn DetectionRule>>,
}

impl TethDetector {
    pub fn new() -> Self {
        Self {
            rules: vec![
                Box::new(GodObjectRule::default()),
                Box::new(AsyncMutexRule::default()),
                Box::new(UnboundedChannelRule::default()),
                Box::new(LongParamListRule::default()),
                Box::new(PanicInLibraryRule::default()),
                Box::new(UnwrapChainRule::default()),
                Box::new(BlockingInAsyncRule::default()),
                Box::new(MagicNumberRule::default()),
            ],
        }
    }
    
    pub fn detect(&self, parsed: &ParsedCrate) -> Vec<AntiPatternResult> {
        let mut results = Vec::new();
        
        for rule in &self.rules {
            results.extend(rule.detect(parsed));
        }
        
        results.sort_by(|a, b| b.severity.cmp(&a.severity));
        results
    }
}

// Example: God Object Detection
pub struct GodObjectRule {
    field_threshold: usize,
    method_threshold: usize,
}

impl Default for GodObjectRule {
    fn default() -> Self {
        Self {
            field_threshold: 15,
            method_threshold: 25,
        }
    }
}

impl DetectionRule for GodObjectRule {
    fn detect(&self, parsed: &ParsedCrate) -> Vec<AntiPatternResult> {
        let mut results = Vec::new();
        
        for file in &parsed.files {
            for struct_info in &file.structs {
                // Find associated impl
                let method_count = file.impls.iter()
                    .filter(|i| i.target == struct_info.name)
                    .map(|i| i.method_count)
                    .sum::<usize>();
                
                if struct_info.field_count > self.field_threshold 
                    || method_count > self.method_threshold 
                {
                    results.push(AntiPatternResult {
                        pattern: AntiPattern::GodObject {
                            struct_name: struct_info.name.clone(),
                            field_count: struct_info.field_count,
                            method_count,
                        },
                        severity: Severity::High,
                        location: Location {
                            file: file.path.clone(),
                            line: 0, // Would need line tracking
                        },
                        remediation: format!(
                            "Split {} into cohesive components. Extract field groups \
                             with related methods into separate structs.",
                            struct_info.name
                        ),
                    });
                }
            }
        }
        
        results
    }
}

// Example: Async Mutex Abuse Detection
pub struct AsyncMutexRule;

impl DetectionRule for AsyncMutexRule {
    fn detect(&self, parsed: &ParsedCrate) -> Vec<AntiPatternResult> {
        let mut results = Vec::new();
        
        let forbidden_patterns = [
            ("std::sync::Mutex", "tokio::sync::Mutex"),
            ("std::sync::RwLock", "tokio::sync::RwLock"),
            ("std::sync::Condvar", "tokio::sync::Notify"),
        ];
        
        for file in &parsed.files {
            for import in &file.imports {
                for (bad, good) in &forbidden_patterns {
                    if import.path.contains(bad) {
                        // Check if file has async functions
                        let has_async = file.functions.iter().any(|f| f.is_async);
                        
                        if has_async {
                            results.push(AntiPatternResult {
                                pattern: AntiPattern::AsyncMutexAbuse {
                                    location: format!("{}:{}", file.path.display(), import.line),
                                    mutex_type: bad.to_string(),
                                },
                                severity: Severity::Critical,
                                location: Location {
                                    file: file.path.clone(),
                                    line: import.line,
                                },
                                remediation: format!(
                                    "Replace {} with {} in async context to prevent deadlocks.",
                                    bad, good
                                ),
                            });
                        }
                    }
                }
            }
        }
        
        results
    }
}

// Example: Unbounded Channel Detection
pub struct UnboundedChannelRule;

impl DetectionRule for UnboundedChannelRule {
    fn detect(&self, parsed: &ParsedCrate) -> Vec<AntiPatternResult> {
        let mut results = Vec::new();
        
        let unbounded_patterns = [
            "mpsc::channel()",           // Unbounded std
            "unbounded_channel()",       // Unbounded tokio
            "broadcast::channel(",       // Can be unbounded
            "Vec::new()",               // Unbounded growth (context-dependent)
            "HashMap::new()",           // Unbounded growth
        ];
        
        // Would need actual AST analysis to detect these patterns in code
        // This is a simplified representation
        
        results
    }
}

// Example: Unwrap Chain Detection
pub struct UnwrapChainRule {
    max_chain_length: usize,
}

impl Default for UnwrapChainRule {
    fn default() -> Self {
        Self { max_chain_length: 2 }
    }
}

impl DetectionRule for UnwrapChainRule {
    fn detect(&self, parsed: &ParsedCrate) -> Vec<AntiPatternResult> {
        let mut results = Vec::new();
        
        // Detect patterns like: foo().unwrap().bar().unwrap().baz().unwrap()
        // Would need expression-level AST walking
        
        results
    }
}
```

### 5.3 Pattern Score Calculation

```rust
pub fn calculate_pattern_score(anti_patterns: &[AntiPatternResult]) -> f64 {
    let mut score = 100.0;
    
    for result in anti_patterns {
        let penalty = match result.severity {
            Severity::Critical => 20.0,
            Severity::High => 10.0,
            Severity::Medium => 5.0,
            Severity::Low => 2.0,
            Severity::Info => 0.0,
        };
        score -= penalty;
    }
    
    score.max(0.0)
}
```

---

## 6. Security Analyzer

### 6.1 Security Checks

```rust
pub struct SecurityMetrics {
    pub unsafe_blocks: Vec<UnsafeUsage>,
    pub unsafe_score: f64,
    
    pub input_validation: InputValidationResult,
    pub validation_score: f64,
    
    pub error_handling: ErrorHandlingResult,
    pub error_score: f64,
    
    pub dependency_audit: DependencyAuditResult,
    pub dependency_score: f64,
    
    pub score: f64,
}

pub struct UnsafeUsage {
    pub location: Location,
    pub reason: Option<String>,  // From // SAFETY: comment
    pub justified: bool,
}

pub fn analyze_security(parsed: &ParsedCrate) -> SecurityMetrics {
    let mut metrics = SecurityMetrics::default();
    
    // Check unsafe blocks
    for file in &parsed.files {
        let unsafe_blocks = find_unsafe_blocks(&file);
        for block in unsafe_blocks {
            let justified = has_safety_comment(&file, &block);
            metrics.unsafe_blocks.push(UnsafeUsage {
                location: block.location,
                reason: extract_safety_comment(&file, &block),
                justified,
            });
        }
    }
    
    // Score unsafe usage
    let unjustified = metrics.unsafe_blocks.iter().filter(|u| !u.justified).count();
    metrics.unsafe_score = 100.0 - (unjustified as f64 * 15.0);
    
    // Input validation (look for validation patterns on public APIs)
    metrics.input_validation = check_input_validation(parsed);
    metrics.validation_score = metrics.input_validation.coverage * 100.0;
    
    // Error handling (Result vs panic)
    metrics.error_handling = check_error_handling(parsed);
    metrics.error_score = calculate_error_handling_score(&metrics.error_handling);
    
    // Dependency audit (check for known vulnerabilities)
    metrics.dependency_audit = audit_dependencies(parsed);
    metrics.dependency_score = if metrics.dependency_audit.vulnerabilities.is_empty() {
        100.0
    } else {
        100.0 - (metrics.dependency_audit.vulnerabilities.len() as f64 * 10.0)
    };
    
    // Aggregate score
    metrics.score = (
        metrics.unsafe_score * 0.25 +
        metrics.validation_score * 0.25 +
        metrics.error_score * 0.25 +
        metrics.dependency_score * 0.25
    ).max(0.0);
    
    metrics
}
```

### 6.2 Cargo Audit Integration

```rust
pub fn audit_dependencies(parsed: &ParsedCrate) -> DependencyAuditResult {
    // Run cargo audit
    let output = Command::new("cargo")
        .args(["audit", "--json"])
        .current_dir(&parsed.root)
        .output()
        .expect("Failed to run cargo audit");
    
    let audit: CargoAuditOutput = serde_json::from_slice(&output.stdout)?;
    
    DependencyAuditResult {
        vulnerabilities: audit.vulnerabilities.list,
        warnings: audit.warnings,
    }
}
```

---

## 7. Quality Aggregator

### 7.1 Final Score Calculation

```rust
pub struct QualityReport {
    pub crate_name: String,
    pub timestamp: DateTime<Utc>,
    
    // Dimension scores
    pub structure: StructureMetrics,
    pub complexity: ComplexityMetrics,
    pub patterns: PatternMetrics,
    pub security: SecurityMetrics,
    
    // Final grade
    pub final_score: f64,
    pub grade: Grade,
    
    // Violations summary
    pub critical_violations: Vec<Violation>,
    pub high_violations: Vec<Violation>,
    
    // Refactor directives
    pub directives: Vec<RefactorDirective>,
}

pub enum Grade {
    A,  // 85-100
    B,  // 70-84
    C,  // 55-69
    D,  // 40-54
    F,  // 0-39
}

pub fn aggregate_quality(
    structure: StructureMetrics,
    complexity: ComplexityMetrics,
    patterns: PatternMetrics,
    security: SecurityMetrics,
) -> QualityReport {
    // Weighted aggregate
    let final_score = 
        structure.score * 0.25 +
        complexity.score * 0.25 +
        patterns.score * 0.25 +
        security.score * 0.25;
    
    let grade = match final_score as u8 {
        85..=100 => Grade::A,
        70..=84 => Grade::B,
        55..=69 => Grade::C,
        40..=54 => Grade::D,
        _ => Grade::F,
    };
    
    // Collect violations
    let mut critical = Vec::new();
    let mut high = Vec::new();
    
    for ap in &patterns.anti_patterns {
        match ap.severity {
            Severity::Critical => critical.push(ap.into()),
            Severity::High => high.push(ap.into()),
            _ => {}
        }
    }
    
    // Generate refactor directives
    let directives = generate_refactor_directives(&structure, &complexity, &patterns);
    
    QualityReport {
        crate_name: "".to_string(), // Filled by caller
        timestamp: Utc::now(),
        structure,
        complexity,
        patterns,
        security,
        final_score,
        grade,
        critical_violations: critical,
        high_violations: high,
        directives,
    }
}
```

---

## 8. Refactor Directive Generator

When quality is below Grade A, generate actionable refactoring instructions.

### 8.1 Directive Types

```rust
pub enum RefactorDirective {
    // Structure
    SplitFile {
        file: PathBuf,
        current_lines: usize,
        target_lines: usize,
        suggested_splits: Vec<SplitSuggestion>,
    },
    
    SplitFunction {
        file: PathBuf,
        function: String,
        current_lines: usize,
        target_lines: usize,
        extract_candidates: Vec<ExtractCandidate>,
    },
    
    // Complexity
    ReduceComplexity {
        file: PathBuf,
        function: String,
        current_cc: usize,
        target_cc: usize,
        strategies: Vec<ComplexityReduction>,
    },
    
    ReduceNesting {
        file: PathBuf,
        function: String,
        current_depth: usize,
        target_depth: usize,
        strategies: Vec<NestingReduction>,
    },
    
    // Anti-patterns
    BreakGodObject {
        struct_name: String,
        suggested_components: Vec<ComponentSuggestion>,
    },
    
    ReplaceAsyncMutex {
        file: PathBuf,
        line: usize,
        from: String,
        to: String,
    },
    
    BoundChannel {
        file: PathBuf,
        line: usize,
        suggested_capacity: usize,
    },
    
    // Error handling
    ReplaceUnwrap {
        file: PathBuf,
        line: usize,
        context: String,
        suggested_handling: String,
    },
    
    AddErrorContext {
        file: PathBuf,
        function: String,
        suggested_context: String,
    },
}

pub struct SplitSuggestion {
    pub module_name: String,
    pub items: Vec<String>,  // Function/struct names to move
    pub reasoning: String,
}

pub struct ExtractCandidate {
    pub name: String,
    pub lines: (usize, usize),
    pub reasoning: String,
}

pub enum ComplexityReduction {
    ExtractMethod { name: String, lines: (usize, usize) },
    ReplaceConditionalWithPolymorphism { match_location: usize },
    SimplifyBooleanExpression { line: usize },
    EarlyReturn { line: usize },
}

pub enum NestingReduction {
    GuardClause { condition: String },
    ExtractToFunction { name: String },
    InvertCondition { line: usize },
}
```

### 8.2 Directive Generation Logic

```rust
pub fn generate_refactor_directives(
    structure: &StructureMetrics,
    complexity: &ComplexityMetrics,
    patterns: &PatternMetrics,
) -> Vec<RefactorDirective> {
    let mut directives = Vec::new();
    
    // File split directives
    for violation in &structure.files_over_limit {
        directives.push(RefactorDirective::SplitFile {
            file: violation.path.clone(),
            current_lines: violation.lines,
            target_lines: violation.limit,
            suggested_splits: suggest_file_splits(&violation),
        });
    }
    
    // Function split directives
    for violation in &structure.functions_over_limit {
        directives.push(RefactorDirective::SplitFunction {
            file: violation.path.clone(),
            function: violation.function.clone(),
            current_lines: violation.lines,
            target_lines: violation.limit,
            extract_candidates: find_extract_candidates(&violation),
        });
    }
    
    // Complexity reduction directives
    for fc in &complexity.function_complexity {
        if fc.cyclomatic > 10 {
            directives.push(RefactorDirective::ReduceComplexity {
                file: fc.file.clone(),
                function: fc.name.clone(),
                current_cc: fc.cyclomatic,
                target_cc: 10,
                strategies: suggest_complexity_reductions(&fc),
            });
        }
        
        if fc.nesting_depth > 4 {
            directives.push(RefactorDirective::ReduceNesting {
                file: fc.file.clone(),
                function: fc.name.clone(),
                current_depth: fc.nesting_depth,
                target_depth: 4,
                strategies: suggest_nesting_reductions(&fc),
            });
        }
    }
    
    // Anti-pattern directives
    for ap in &patterns.anti_patterns {
        match &ap.pattern {
            AntiPattern::GodObject { struct_name, .. } => {
                directives.push(RefactorDirective::BreakGodObject {
                    struct_name: struct_name.clone(),
                    suggested_components: suggest_component_extraction(&ap),
                });
            }
            AntiPattern::AsyncMutexAbuse { location, mutex_type } => {
                directives.push(RefactorDirective::ReplaceAsyncMutex {
                    file: ap.location.file.clone(),
                    line: ap.location.line,
                    from: mutex_type.clone(),
                    to: suggest_async_alternative(mutex_type),
                });
            }
            AntiPattern::UnboundedChannel { location } => {
                directives.push(RefactorDirective::BoundChannel {
                    file: ap.location.file.clone(),
                    line: ap.location.line,
                    suggested_capacity: 1024,
                });
            }
            _ => {}
        }
    }
    
    directives
}
```

---

## 9. Output Format (lightning-qa.json)

### 9.1 Schema

```json
{
  "$schema": "https://sx9.dev/schemas/lightning-qa.json",
  "version": "1.0.0",
  
  "metadata": {
    "crate_name": "sx9-nats-router",
    "analyzed_at": "2025-12-20T00:00:00Z",
    "engine_version": "1.0.0",
    "birth_certificate_hash": "abc123..."
  },
  
  "scores": {
    "structure": 85.0,
    "complexity": 92.0,
    "patterns": 78.0,
    "security": 95.0,
    "final": 87.5
  },
  
  "grade": "A",
  "passed": true,
  
  "structure": {
    "total_files": 5,
    "total_lines": 1247,
    "avg_file_lines": 249.4,
    "max_file_lines": 287,
    "files_over_limit": [],
    "functions_over_limit": []
  },
  
  "complexity": {
    "avg_cyclomatic": 4.2,
    "max_cyclomatic": 8,
    "avg_cognitive": 6.1,
    "max_cognitive": 12,
    "avg_nesting_depth": 2.1,
    "max_nesting_depth": 3,
    "violations": []
  },
  
  "patterns": {
    "anti_patterns_found": 2,
    "critical": 0,
    "high": 1,
    "medium": 1,
    "low": 0,
    "details": [
      {
        "type": "LongParameterList",
        "severity": "high",
        "location": "src/router.rs:45",
        "details": { "function": "configure_routes", "param_count": 9 },
        "remediation": "Extract parameters into a Configuration struct"
      },
      {
        "type": "MagicNumber",
        "severity": "medium",
        "location": "src/lib.rs:127",
        "details": { "value": 1024 },
        "remediation": "Extract to named constant CHANNEL_CAPACITY"
      }
    ]
  },
  
  "security": {
    "unsafe_blocks": 0,
    "unjustified_unsafe": 0,
    "input_validation_coverage": 0.95,
    "error_handling_score": 0.92,
    "vulnerabilities": []
  },
  
  "refactor_directives": [
    {
      "type": "ExtractParameterObject",
      "priority": "high",
      "file": "src/router.rs",
      "function": "configure_routes",
      "action": "Create RouterConfig struct with the 9 parameters",
      "expected_improvement": "+5 pattern score"
    },
    {
      "type": "ExtractConstant",
      "priority": "medium",
      "file": "src/lib.rs",
      "line": 127,
      "action": "const CHANNEL_CAPACITY: usize = 1024;",
      "expected_improvement": "+2 pattern score"
    }
  ],
  
  "recommendation": "PASS - Minor improvements suggested but not required"
}
```

---

## 10. Integration with Factory Agent

### 10.1 Factory Loop with QA

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    FACTORY AGENT + LIGHTNING QA                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. BUILD ────────────────────────────────────────────────────────────────▶│
│     │  Factory Agent executes canonical prompt                             │
│     │                                                                       │
│  2. SELF-INTERVIEW ───────────────────────────────────────────────────────▶│
│     │  Generate crate_interview.json                                       │
│     │                                                                       │
│  3. LIGHTNING QA ─────────────────────────────────────────────────────────▶│
│     │  Run quality analysis                                                │
│     │  Generate lightning-qa.json                                          │
│     │                                                                       │
│  4. DECISION ─────────────────────────────────────────────────────────────▶│
│     │                                                                       │
│     ├─── Grade A ──▶ PASS ──▶ Git Push ──▶ Registry                        │
│     │                                                                       │
│     ├─── Grade B/C ──▶ REFACTOR ──▶ Apply Directives ──▶ Retry (1-3)      │
│     │                                                                       │
│     └─── Grade D/F ──▶ HALT ──▶ Human Assistance Required                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 10.2 Refactor Loop

```rust
pub async fn factory_build_loop(
    prompt: &CanonicalPrompt,
    max_retries: usize,
) -> Result<BuildResult, FactoryError> {
    let mut attempt = 0;
    
    loop {
        attempt += 1;
        
        // 1. Build
        let crate_path = build_from_prompt(prompt).await?;
        
        // 2. Self-interview
        let interview = generate_self_interview(&crate_path)?;
        save_interview(&crate_path, &interview)?;
        
        // 3. Lightning QA
        let qa_report = run_lightning_qa(&crate_path)?;
        save_qa_report(&crate_path, &qa_report)?;
        
        // 4. Decision
        match qa_report.grade {
            Grade::A => {
                // Pass - push to registry
                push_to_git(&crate_path).await?;
                notify_slack(SlackEvent::Success { 
                    crate_name: prompt.crate_name.clone(),
                    grade: qa_report.grade,
                }).await?;
                return Ok(BuildResult::Success(crate_path));
            }
            
            Grade::B | Grade::C => {
                if attempt >= max_retries {
                    notify_slack(SlackEvent::Halt {
                        crate_name: prompt.crate_name.clone(),
                        reason: "Max refactor attempts reached".to_string(),
                    }).await?;
                    return Err(FactoryError::MaxRetriesExceeded);
                }
                
                // Refactor and retry
                notify_slack(SlackEvent::Warn {
                    crate_name: prompt.crate_name.clone(),
                    attempt,
                    max_retries,
                }).await?;
                
                apply_refactor_directives(&crate_path, &qa_report.directives).await?;
                continue;
            }
            
            Grade::D | Grade::F => {
                // Halt - fundamental issues
                notify_slack(SlackEvent::Halt {
                    crate_name: prompt.crate_name.clone(),
                    reason: format!("Grade {} - architectural issues", qa_report.grade),
                }).await?;
                return Err(FactoryError::FundamentalViolations(qa_report));
            }
        }
    }
}
```

### 10.3 Directive Application

```rust
pub async fn apply_refactor_directives(
    crate_path: &Path,
    directives: &[RefactorDirective],
) -> Result<(), RefactorError> {
    // Sort directives by priority
    let mut sorted = directives.to_vec();
    sorted.sort_by_key(|d| d.priority());
    
    for directive in sorted {
        match directive {
            RefactorDirective::ReplaceAsyncMutex { file, line, from, to } => {
                // Direct text replacement
                let content = fs::read_to_string(file)?;
                let updated = content.replace(from, to);
                fs::write(file, updated)?;
            }
            
            RefactorDirective::BoundChannel { file, line, suggested_capacity } => {
                // More complex: need to find unbounded() and add capacity
                let content = fs::read_to_string(file)?;
                let updated = replace_unbounded_channel(&content, *line, *suggested_capacity)?;
                fs::write(file, updated)?;
            }
            
            RefactorDirective::SplitFile { file, suggested_splits, .. } => {
                // Extract items to new modules
                for split in suggested_splits {
                    extract_to_module(file, &split.module_name, &split.items)?;
                }
            }
            
            // Complex refactors may require agent re-prompting
            RefactorDirective::BreakGodObject { .. } |
            RefactorDirective::ReduceComplexity { .. } => {
                // Generate targeted refactor prompt and re-invoke agent
                let refactor_prompt = generate_refactor_prompt(directive)?;
                execute_refactor_prompt(&refactor_prompt).await?;
            }
            
            _ => {}
        }
    }
    
    // Verify after refactoring
    Command::new("cargo")
        .args(["check"])
        .current_dir(crate_path)
        .status()?;
    
    Ok(())
}
```

---

## 11. CLI Interface

### 11.1 Commands

```bash
# Run full QA analysis
sx9-lightning-qa analyze /path/to/crate

# Run specific checks
sx9-lightning-qa structure /path/to/crate
sx9-lightning-qa complexity /path/to/crate
sx9-lightning-qa patterns /path/to/crate
sx9-lightning-qa security /path/to/crate

# Generate refactor directives only
sx9-lightning-qa directives /path/to/crate

# Apply refactor directives
sx9-lightning-qa refactor /path/to/crate --apply

# Output formats
sx9-lightning-qa analyze /path/to/crate --format json
sx9-lightning-qa analyze /path/to/crate --format markdown
sx9-lightning-qa analyze /path/to/crate --format sarif  # For GitHub integration

# CI mode (exit code based on grade)
sx9-lightning-qa analyze /path/to/crate --ci --min-grade B
```

### 11.2 CI Integration

```yaml
# .github/workflows/quality-gate.yml

name: Quality Gate

on:
  pull_request:
    branches: [main, develop]

jobs:
  lightning-qa:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Lightning QA
        run: cargo install sx9-lightning-qa
      
      - name: Run Quality Analysis
        run: |
          sx9-lightning-qa analyze . --format sarif --output qa-results.sarif
      
      - name: Upload SARIF
        uses: github/codeql-action/upload-sarif@v2
        with:
          sarif_file: qa-results.sarif
      
      - name: Quality Gate Check
        run: sx9-lightning-qa analyze . --ci --min-grade A
```

---

## 12. NATS Integration

### 12.1 Quality Events

```
sx9.qa.{crate}.started      # Analysis started
sx9.qa.{crate}.completed    # Analysis completed (with grade)
sx9.qa.{crate}.failed       # Analysis failed (error)
sx9.qa.{crate}.refactor     # Refactor in progress
```

### 12.2 Event Payloads

```json
// sx9.qa.sx9-nats-router.completed
{
  "crate": "sx9-nats-router",
  "timestamp": "2025-12-20T00:00:00Z",
  "grade": "A",
  "score": 87.5,
  "passed": true,
  "dimensions": {
    "structure": 85.0,
    "complexity": 92.0,
    "patterns": 78.0,
    "security": 95.0
  },
  "violations": {
    "critical": 0,
    "high": 1,
    "medium": 1
  }
}
```

---

## 13. Implementation Checklist

### Phase 1: Core Analysis (Week 1)

- [ ] AST Parser: Rust (syn-based)
- [ ] AST Parser: Python (tree-sitter)
- [ ] Structure metrics calculator
- [ ] Complexity metrics (McCabe + Cognitive)

### Phase 2: TETH Detector (Week 2)

- [ ] God Object detection
- [ ] Async Mutex abuse detection
- [ ] Unbounded channel detection
- [ ] Long parameter list detection
- [ ] Unwrap chain detection
- [ ] Magic number detection

### Phase 3: Security Analyzer (Week 3)

- [ ] Unsafe block analyzer
- [ ] Input validation checker
- [ ] Error handling scorer
- [ ] Cargo audit integration

### Phase 4: Refactor Engine (Week 4)

- [ ] Directive generator
- [ ] Simple directive applicator (text replacement)
- [ ] Complex directive prompting
- [ ] Verify-after-refactor logic

### Phase 5: Integration (Week 5)

- [ ] Factory agent integration
- [ ] CLI interface
- [ ] NATS event emission
- [ ] CI/CD integration (SARIF output)
- [ ] Slack notifications

---

## 14. References

- RFC-9101: SmartCrate Manifest
- RFC-9120: Prompt Forge v4
- TETH Framework: Tool Entropy Testing Harness
- McCabe, T.J. (1976). "A Complexity Measure"
- SonarSource Cognitive Complexity Whitepaper
- SARIF 2.1.0 Specification

---

## Appendix A: Complexity Thresholds Reference

| Metric | Green | Yellow | Red |
|--------|-------|--------|-----|
| Cyclomatic Complexity | 1-10 | 11-15 | 16+ |
| Cognitive Complexity | 1-15 | 16-25 | 26+ |
| Nesting Depth | 1-3 | 4 | 5+ |
| Parameter Count | 1-5 | 6-7 | 8+ |
| Lines per Function | 1-50 | 51-75 | 76+ |
| Lines per File | 1-300 | 301-500 | 501+ |
| Dependencies | 1-15 | 16-25 | 26+ |

---

## Appendix B: Anti-Pattern Severity Matrix

| Anti-Pattern | Severity | Score Penalty | Auto-Fixable |
|--------------|----------|---------------|--------------|
| AsyncMutexAbuse | Critical | -20 | Yes |
| PanicInLibrary | Critical | -20 | Partial |
| GodObject | High | -10 | No (requires agent) |
| SpaghettiDeps | High | -10 | No |
| UnboundedChannel | High | -10 | Yes |
| LongParameterList | High | -10 | Yes |
| UnwrapChain | Medium | -5 | Yes |
| MagicNumber | Medium | -5 | Yes |
| DeadCode | Low | -2 | Yes |
| DuplicateCode | Low | -2 | No (requires agent) |

---

## Appendix C: Sample Refactor Prompt (Complex)

When a directive can't be auto-applied, generate a targeted refactor prompt:

```markdown
# REFACTOR DIRECTIVE: Break God Object

## Target
Struct: `MessageRouter` in `src/router.rs`
Current: 23 fields, 31 methods
Target: Max 10 fields, 15 methods per component

## Identified Cohesive Groups

### Group 1: Connection Management
Fields: `connections`, `connection_pool`, `max_connections`, `timeout`
Methods: `connect()`, `disconnect()`, `pool_status()`, `prune_stale()`

### Group 2: Message Routing
Fields: `routes`, `route_cache`, `fallback_route`
Methods: `route()`, `add_route()`, `remove_route()`, `match_route()`

### Group 3: Health Monitoring
Fields: `health_state`, `last_check`, `metrics`
Methods: `health_check()`, `report_metrics()`, `is_healthy()`

## Required Outcome

1. Extract `ConnectionManager` struct with Group 1
2. Extract `RouteTable` struct with Group 2
3. Extract `HealthMonitor` struct with Group 3
4. `MessageRouter` composes these three components
5. All existing public API signatures must remain unchanged
6. Add integration tests verifying composition works

## Constraints
- No new dependencies
- Preserve all existing tests
- Max 300 lines per new file
```

---

*End of RFC-9121*
