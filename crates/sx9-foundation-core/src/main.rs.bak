use std::process::Command;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct CrateMigrationTarget {
    name: String,
    dependency_count: usize,
    dependencies: Vec<String>,
    foundation_deps: Vec<String>,
    migration_effort: MigrationEffort,
    memory_savings_mb: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MigrationEffort {
    Trivial,   // 0-2 deps
    Easy,      // 3-4 deps  
    Medium,    // 5-8 deps
    Hard,      // 9-15 deps
    Complex,   // 16+ deps
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 CTAS Foundation Migration Analyzer");
    println!("{}", "=".repeat(50));
    
    // Get cargo metadata
    let output = Command::new("cargo")
        .args(&["metadata", "--format-version", "1", "--no-deps"])
        .output()?;
    
    let metadata: Value = serde_json::from_slice(&output.stdout)?;
    
    // Analyze crates by dependency count
    let mut migration_targets = Vec::new();
    
    if let Some(packages) = metadata["packages"].as_array() {
        println!("📊 Analyzing {} crates for foundation migration...", packages.len());
        
        for package in packages {
            let crate_name = package["name"].as_str().unwrap_or("unknown").to_string();
            let dependencies = package["dependencies"].as_array();
            
            let mut deps = Vec::new();
            if let Some(deps_array) = dependencies {
                for dep in deps_array {
                    let dep_name = dep["name"].as_str().unwrap_or("unknown").to_string();
                    deps.push(dep_name);
                }
            }
            
            let target = analyze_migration_target(&crate_name, &deps);
            migration_targets.push(target);
        }
    }
    
    // Sort by dependency count (lowest first)
    migration_targets.sort_by(|a, b| a.dependency_count.cmp(&b.dependency_count));
    
    // Display analysis
    display_migration_targets(&migration_targets);
    display_effort_breakdown(&migration_targets);
    display_migration_plan(&migration_targets);
    
    Ok(())
}

fn analyze_migration_target(crate_name: &str, dependencies: &[String]) -> CrateMigrationTarget {
    let dependency_count = dependencies.len();
    let foundation_deps = identify_foundation_dependencies(dependencies);
    let migration_effort = categorize_migration_effort(dependency_count);
    let memory_savings = estimate_memory_savings(&foundation_deps);
    
    CrateMigrationTarget {
        name: crate_name.to_string(),
        dependency_count,
        dependencies: dependencies.to_vec(),
        foundation_deps,
        migration_effort,
        memory_savings_mb: memory_savings,
    }
}

fn identify_foundation_dependencies(dependencies: &[String]) -> Vec<String> {
    let foundation_deps = vec![
        "tokio", "serde", "anyhow", "tracing", "serde_json", 
        "chrono", "uuid", "reqwest", "tracing-subscriber", "thiserror"
    ];
    
    dependencies.iter()
        .filter(|dep| foundation_deps.iter().any(|fd| dep.contains(fd)))
        .cloned()
        .collect()
}

fn categorize_migration_effort(dependency_count: usize) -> MigrationEffort {
    match dependency_count {
        0..=2 => MigrationEffort::Trivial,
        3..=4 => MigrationEffort::Easy,
        5..=8 => MigrationEffort::Medium,
        9..=15 => MigrationEffort::Hard,
        _ => MigrationEffort::Complex,
    }
}

fn estimate_memory_savings(foundation_deps: &[String]) -> f64 {
    let mut savings = 0.0;
    
    for dep in foundation_deps {
        match dep.as_str() {
            "tokio" => savings += 3.0,
            "serde" => savings += 1.5,
            "reqwest" => savings += 2.0,
            "chrono" => savings += 1.0,
            "uuid" => savings += 0.5,
            "tracing" => savings += 0.5,
            "serde_json" => savings += 0.5,
            "anyhow" => savings += 0.3,
            "tracing-subscriber" => savings += 0.5,
            "thiserror" => savings += 0.2,
            _ => savings += 0.3,
        }
    }
    
    savings
}

fn display_migration_targets(targets: &[CrateMigrationTarget]) {
    println!("\n🎯 Migration Targets by Dependency Count");
    println!("{}", "=".repeat(50));
    
    // Group by dependency count
    let mut count_groups: HashMap<usize, Vec<&CrateMigrationTarget>> = HashMap::new();
    for target in targets {
        count_groups.entry(target.dependency_count)
            .and_modify(|group| group.push(target))
            .or_insert_with(|| vec![target]);
    }
    
    // Display in order (lowest first)
    let mut counts: Vec<_> = count_groups.keys().collect();
    counts.sort();
    
    for &count in counts {
        let group = &count_groups[&count];
        println!("\n📦 {} Dependencies ({} crates):", count, group.len());
        
        for target in group {
            let foundation_percentage = if target.dependency_count > 0 {
                (target.foundation_deps.len() as f64 / target.dependency_count as f64) * 100.0
            } else {
                0.0
            };
            
            println!("  {}: {} foundation deps ({:.1}%), {:.1}MB savings", 
                target.name, target.foundation_deps.len(), foundation_percentage, target.memory_savings_mb);
        }
    }
}

fn display_effort_breakdown(targets: &[CrateMigrationTarget]) {
    println!("\n⚡ Migration Effort Breakdown");
    println!("{}", "=".repeat(50));
    
    let mut effort_groups: HashMap<MigrationEffort, Vec<&CrateMigrationTarget>> = HashMap::new();
    for target in targets {
        effort_groups.entry(target.migration_effort.clone())
            .and_modify(|group| group.push(target))
            .or_insert_with(|| vec![target]);
    }
    
    // Display in effort order
    let effort_order = vec![
        MigrationEffort::Trivial,
        MigrationEffort::Easy,
        MigrationEffort::Medium,
        MigrationEffort::Hard,
        MigrationEffort::Complex,
    ];
    
    for effort in effort_order {
        if let Some(group) = effort_groups.get(&effort) {
            let total_savings: f64 = group.iter().map(|t| t.memory_savings_mb).sum();
            let avg_deps: f64 = group.iter().map(|t| t.dependency_count as f64).sum::<f64>() / group.len() as f64;
            
            println!("\n{:?} ({} crates):", effort, group.len());
            println!("  Average deps: {:.1}", avg_deps);
            println!("  Total memory savings: {:.1}MB", total_savings);
            println!("  Top targets:");
            
            // Show top 5 by savings
            let mut sorted_group = group.to_vec();
            sorted_group.sort_by(|a, b| b.memory_savings_mb.partial_cmp(&a.memory_savings_mb).unwrap());
            
            for target in sorted_group.iter().take(5) {
                println!("    {}: {:.1}MB savings", target.name, target.memory_savings_mb);
            }
        }
    }
}

fn display_migration_plan(targets: &[CrateMigrationTarget]) {
    println!("\n🚀 Foundation Migration Plan");
    println!("{}", "=".repeat(50));
    
    // Focus on crates with 4 or fewer dependencies
    let easy_targets: Vec<_> = targets.iter()
        .filter(|t| t.dependency_count <= 4)
        .collect();
    
    println!("\n🎯 Phase 1: Easy Targets (≤4 deps) - {} crates", easy_targets.len());
    
    let mut total_savings = 0.0;
    let mut total_foundation_deps = 0;
    
    for target in &easy_targets {
        println!("  {}: {} deps → 1 foundation dep, {:.1}MB savings", 
            target.name, target.dependency_count, target.memory_savings_mb);
        total_savings += target.memory_savings_mb;
        total_foundation_deps += target.foundation_deps.len();
    }
    
    println!("\n📊 Phase 1 Summary:");
    println!("  Total memory savings: {:.1}MB", total_savings);
    println!("  Foundation deps eliminated: {}", total_foundation_deps);
    println!("  Average savings per crate: {:.1}MB", total_savings / easy_targets.len() as f64);
    
    // Show migration steps
    println!("\n🔧 Migration Steps:");
    println!("  1. Create ctas-foundation crate (DONE)");
    println!("  2. Migrate {} easy targets first", easy_targets.len());
    println!("  3. Update Cargo.toml files to use ctas-foundation");
    println!("  4. Remove individual dependency declarations");
    println!("  5. Test and validate");
    
    // Show example migration
    if let Some(example) = easy_targets.first() {
        println!("\n📝 Example Migration for {}:", example.name);
        println!("  BEFORE:");
        println!("    [dependencies]");
        for dep in &example.dependencies {
            println!("    {} = \"1.0\"", dep);
        }
        println!("  AFTER:");
        println!("    [dependencies]");
        println!("    ctas-foundation = {{ workspace = true }}");
    }
}
