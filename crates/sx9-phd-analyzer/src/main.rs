use anyhow::Result;
use chrono::Utc;
use serde::Serialize;
use std::fs;
use std::path::Path;
use syn::visit::{self, Visit};
use walkdir::WalkDir;

#[derive(Serialize)]
struct FileMetrics {
    path: String,
    loc: usize,
    lloc: usize,
    comments: usize,
    cyclo: usize,
    halstead: Halstead,
    mi: f64,
    warnings: Vec<String>,
}
#[derive(Serialize, Clone, Copy, Default)]
struct Halstead {
    n1: usize,
    n2: usize,
    N1: usize,
    N2: usize,
    vocab: f64,
    length: f64,
    volume: f64,
    difficulty: f64,
    effort: f64,
}
#[derive(Serialize)]
struct Report {
    ts: String,
    root: String,
    totals: Totals,
    files: Vec<FileMetrics>,
}
#[derive(Serialize, Default)]
struct Totals {
    files: usize,
    loc: usize,
    lloc: usize,
    cyclo: usize,
}

struct ComplexityVisitor {
    complexity: usize,
}

impl ComplexityVisitor {
    fn new() -> Self {
        Self { complexity: 1 }
    } // Base complexity is 1
}

impl<'ast> Visit<'ast> for ComplexityVisitor {
    fn visit_expr_if(&mut self, i: &'ast syn::ExprIf) {
        self.complexity += 1;
        if let Some((_, else_branch)) = &i.else_branch {
            // fast heuristic: else if counts as 1 in many models if treated as composite
            // standard mccabe: each branch target incs complexity.
            // else is a branch, but simplistic cyclo usually counts user-visible predicates.
            // Let's stick to predicates (if) + loops.
            // NOTE: strict else doesn't add predicate, just path.
            // But 'else if' contains another ExprIf scan.
        }
        visit::visit_expr_if(self, i);
    }
    fn visit_expr_match(&mut self, i: &'ast syn::ExprMatch) {
        // Each arm is a branch
        for _ in &i.arms {
            self.complexity += 1;
        }
        visit::visit_expr_match(self, i);
    }
    fn visit_expr_loop(&mut self, i: &'ast syn::ExprLoop) {
        self.complexity += 1;
        visit::visit_expr_loop(self, i);
    }
    fn visit_expr_while(&mut self, i: &'ast syn::ExprWhile) {
        self.complexity += 1;
        visit::visit_expr_while(self, i);
    }
    fn visit_expr_for_loop(&mut self, i: &'ast syn::ExprForLoop) {
        self.complexity += 1;
        visit::visit_expr_for_loop(self, i);
    }
    // Correct: Use visit_expr_binary to inspect the full expression including operator
    fn visit_expr_binary(&mut self, i: &'ast syn::ExprBinary) {
        match i.op {
            syn::BinOp::And(_) | syn::BinOp::Or(_) => self.complexity += 1,
            _ => {}
        }
        visit::visit_expr_binary(self, i);
    }
}

fn main() -> Result<()> {
    let root = std::env::args().nth(1).unwrap_or(".".into());
    let json = std::env::args().any(|a| a == "--json");
    let (files, rep) = analyze_root(&root)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&rep)?);
    } else {
        print_human(&rep);
    }
    if !json {
        eprintln!("Analyzed {} .rs files under {}", files, root);
    }
    Ok(())
}

fn analyze_root(root: &str) -> Result<(usize, Report)> {
    // Load ignore patterns
    let mut ignores = Vec::new();
    if let Ok(content) = fs::read_to_string(".lightningignore") {
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                ignores.push(trimmed.to_string());
            }
        }
    }
    // Always ignore target and .git
    ignores.push("target/".into());
    ignores.push(".git/".into());

    let mut out = Vec::new();
    for ent in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = ent.path().to_string_lossy();

        // Check ignores
        let mut skipped = false;
        for ignore in &ignores {
            if path.contains(ignore) {
                skipped = true;
                break;
            }
        }
        if skipped {
            continue;
        }

        if ent.file_type().is_file() && ent.path().extension().map(|e| e == "rs").unwrap_or(false) {
            out.push(analyze_file(ent.path())?);
        }
    }
    let mut totals = Totals::default();
    for f in &out {
        totals.files += 1;
        totals.loc += f.loc;
        totals.lloc += f.lloc;
        totals.cyclo += f.cyclo;
    }
    let rep = Report {
        ts: Utc::now().to_rfc3339(),
        root: root.into(),
        totals,
        files: out,
    };
    Ok((rep.files.len(), rep))
}

fn analyze_file(p: &Path) -> Result<FileMetrics> {
    let s = fs::read_to_string(p)?;
    let loc = s.lines().count();
    let comments = s
        .lines()
        .filter(|l| l.trim_start().starts_with("//"))
        .count();
    let lloc = s
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with("//")
        })
        .count();

    // AST Parsing
    let cyclo = match syn::parse_file(&s) {
        Ok(ast) => {
            let mut visitor = ComplexityVisitor::new();
            visitor.visit_file(&ast);
            visitor.complexity
        }
        Err(_) => {
            // Fallback for parse errors (maybe partial code)
            1
        }
    };

    // Halstead is stil lexical approx for now (requires deeper visitor)
    // Left as heuristic for speed, but cyclo is now AST based.
    let hs = Halstead::default();
    let mi = maintainability_index(lloc as f64, cyclo as f64, 5000.0); // Dummy volume for now or calc it

    let mut warnings = Vec::new();
    if cyclo > 20 {
        warnings.push(format!("cyclomatic>{}", 20));
    }

    Ok(FileMetrics {
        path: p.display().to_string(),
        loc,
        lloc,
        comments,
        cyclo,
        halstead: hs,
        mi,
        warnings,
    })
}

fn maintainability_index(loc: f64, v_g: f64, vol: f64) -> f64 {
    if loc <= 0.0 {
        return 100.0;
    }
    let mi = 171.0 - 5.2 * (vol.max(1.0).ln()) - 0.23 * v_g - 16.2 * (loc.ln());
    (mi * 100.0 / 171.0).clamp(0.0, 100.0)
}

fn print_human(rep: &Report) {
    println!("CTAS PhD Analyzer (AST) {}", rep.ts);
    println!("Root: {}", rep.root);
    println!(
        "Totals: files={} loc={} cyclo={}",
        rep.totals.files, rep.totals.loc, rep.totals.cyclo
    );
    for f in &rep.files {
        println!("- {} | loc={} cyclo={}", f.path, f.loc, f.cyclo);
    }
}
