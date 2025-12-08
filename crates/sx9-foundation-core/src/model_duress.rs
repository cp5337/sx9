// Model duress pattern detection logic
use crate::fratricide_types::*;
use anyhow::Result;
use std::path::Path;
use std::collections::HashSet;

// --- Helper functions extracted from fratricide.rs ---

pub async fn find_files_by_pattern(repo_path: &str, patterns: &[&str]) -> Result<Vec<String>> {
    let mut files = Vec::new();
    let repo_path = Path::new(repo_path);
    if let Ok(mut entries) = tokio::fs::read_dir(repo_path).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Ok(file_type) = entry.file_type().await {
                if file_type.is_file() {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        for pattern in patterns {
                            if file_name.contains(pattern) {
                                files.push(entry.path().to_string_lossy().to_string());
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(files)
}

pub async fn find_crate_directories(repo_path: &str) -> Result<Vec<String>> {
    let mut crate_dirs = Vec::new();
    let repo_path = Path::new(repo_path);
    if let Ok(mut entries) = tokio::fs::read_dir(repo_path).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Ok(file_type) = entry.file_type().await {
                if file_type.is_dir() {
                    let cargo_path = entry.path().join("Cargo.toml");
                    if cargo_path.exists() {
                        crate_dirs.push(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    Ok(crate_dirs)
}

pub fn names_are_similar(name1: &str, name2: &str) -> bool {
    let name1_lower = name1.to_lowercase();
    let name2_lower = name2.to_lowercase();
    name1_lower.contains(&name2_lower) ||
    name2_lower.contains(&name1_lower) ||
    name1_lower.split('_').any(|part| name2_lower.contains(part)) ||
    name2_lower.split('_').any(|part| name1_lower.contains(part))
}

pub async fn get_file_age(file_path: &str) -> Result<u64> {
    let metadata = tokio::fs::metadata(file_path).await?;
    let modified = metadata.modified()?;
    let age = std::time::SystemTime::now().duration_since(modified)?.as_secs();
    Ok(age)
}

pub async fn find_all_source_files(repo_path: &str) -> Result<Vec<String>> {
    let mut source_files = Vec::new();
    let repo_path = Path::new(repo_path);
    let source_extensions = ["rs", "py", "js", "ts", "tsx", "jsx", "go", "java", "cpp", "c", "h", "hpp"];
    if let Ok(mut entries) = tokio::fs::read_dir(repo_path).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Ok(file_type) = entry.file_type().await {
                if file_type.is_file() {
                    if let Some(extension) = entry.path().extension() {
                        if let Some(ext_str) = extension.to_str() {
                            if source_extensions.contains(&ext_str) {
                                source_files.push(entry.path().to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(source_files)
}

pub async fn calculate_file_similarity(file1: &str, file2: &str) -> Result<f64> {
    let content1 = tokio::fs::read_to_string(file1).await.unwrap_or_default();
    let content2 = tokio::fs::read_to_string(file2).await.unwrap_or_default();
    let lines1: HashSet<&str> = content1.lines().collect();
    let lines2: HashSet<&str> = content2.lines().collect();
    let intersection = lines1.intersection(&lines2).count();
    let union = lines1.union(&lines2).count();
    if union == 0 {
        Ok(0.0)
    } else {
        Ok(intersection as f64 / union as f64)
    }
}

pub fn extract_functionality_signature(content: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    content.lines().for_each(|line| {
        line.trim().hash(&mut hasher);
    });
    format!("{:x}", hasher.finish())
}

pub fn signatures_similar(sig1: &str, sig2: &str) -> bool {
    sig1 == sig2 ||
    sig1.len() > 10 && sig2.len() > 10 &&
    (sig1.contains(&sig2[..10]) || sig2.contains(&sig1[..10]))
}

// --- Model duress pattern detection functions ---
pub async fn detect_script_to_crate_evolution(repo_path: &str) -> Result<Vec<ModelDuressPattern>> {
    let mut patterns = Vec::new();
    let script_files = find_files_by_pattern(repo_path, &["*.py", "*.sh", "*.js", "*.ts"]).await?;
    let crate_dirs = find_crate_directories(repo_path).await?;
    for script in &script_files {
        let script_name = Path::new(script.as_str()).file_stem().and_then(|s| s.to_str()).unwrap_or("");
        for crate_dir in &crate_dirs {
            let crate_name = Path::new(crate_dir).file_name().and_then(|s| s.to_str()).unwrap_or("");
            if names_are_similar(script_name, crate_name) {
                if let Ok(script_age) = get_file_age(script).await {
                    if let Ok(crate_age) = get_file_age(&format!("{}/Cargo.toml", crate_dir)).await {
                        if script_age > crate_age {
                            patterns.push(ModelDuressPattern {
                                pattern_type: "script_to_crate_evolution".to_string(),
                                description: format!("Script {} evolved into crate {} but wasn't removed", script, crate_name),
                                original_file: script.clone(),
                                duplicate_file: format!("{}/src/lib.rs", crate_dir),
                                confidence: 0.8,
                                recommendation: format!("Remove {} or migrate remaining functionality", script),
                                duress_indicators: vec!["temporal_evolution".to_string(), "name_similarity".to_string()],
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(patterns)
}

pub async fn detect_temporary_becoming_permanent(repo_path: &str) -> Result<Vec<ModelDuressPattern>> {
    let mut patterns = Vec::new();
    let temp_indicators = [
        "temp_", "tmp_", "quick_", "fast_", "simple_", "test_", "debug_",
        "_temp", "_tmp", "_quick", "_fast", "_simple", "_test", "_debug",
        "backup_", "_backup", "old_", "_old", "legacy_", "_legacy"
    ];
    let all_files = find_all_source_files(repo_path).await?;
    for file in &all_files {
        let filename = Path::new(file).file_name().and_then(|s| s.to_str()).unwrap_or("");
        for indicator in &temp_indicators {
            if filename.contains(indicator) {
                if let Ok(content) = tokio::fs::read_to_string(file).await {
                    let line_count = content.lines().count();
                    if line_count > 50 {
                        let clean_name = filename.replace(indicator, "");
                        for other_file in &all_files {
                            let other_filename = Path::new(other_file).file_name().and_then(|s| s.to_str()).unwrap_or("");
                            if other_filename.contains(&clean_name) && other_file != file {
                                patterns.push(ModelDuressPattern {
                                    pattern_type: "temporary_became_permanent".to_string(),
                                    description: format!("Temporary file {} became substantial ({} lines) alongside {}", filename, line_count, other_filename),
                                    original_file: file.clone(),
                                    duplicate_file: other_file.clone(),
                                    confidence: 0.7,
                                    recommendation: "Review and consolidate temporary solution".to_string(),
                                    duress_indicators: vec![
                                        format!("temporary_naming: {}", indicator),
                                        format!("substantial_size: {} lines", line_count),
                                    ],
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(patterns)
}

pub async fn detect_ai_naming_patterns(repo_path: &str) -> Result<Vec<ModelDuressPattern>> {
    let mut patterns = Vec::new();
    let ai_naming_patterns = [
        ("_v1", "_v2"), ("_version1", "_version2"), ("_ver1", "_ver2"),
        ("_new", "_old"), ("_updated", "_original"), ("_fixed", "_broken"),
        ("_working", "_test"), ("_final", "_draft"), ("_clean", "_messy"),
        ("_improved", "_basic"), ("_enhanced", "_simple"),
    ];
    let all_files = find_all_source_files(repo_path).await?;
    for (pattern1, pattern2) in &ai_naming_patterns {
        for file in &all_files {
            let filename = Path::new(file).file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if filename.contains(pattern1) {
                let alt_name = filename.replace(pattern1, pattern2);
                for other_file in &all_files {
                    let other_filename = Path::new(other_file).file_stem().and_then(|s| s.to_str()).unwrap_or("");
                    if other_filename.contains(&alt_name) {
                        if let Ok(similarity) = calculate_file_similarity(file, other_file).await {
                            if similarity > 0.6 {
                                patterns.push(ModelDuressPattern {
                                    pattern_type: "ai_versioning_pattern".to_string(),
                                    description: format!("AI versioning pattern detected: {} and {} are {:.1}% similar", filename, other_filename, similarity * 100.0),
                                    original_file: file.clone(),
                                    duplicate_file: other_file.clone(),
                                    confidence: similarity,
                                    recommendation: "Consolidate versioned implementations".to_string(),
                                    duress_indicators: vec![
                                        format!("ai_naming_pattern: {} / {}", pattern1, pattern2),
                                        format!("content_similarity: {:.1}%", similarity * 100.0),
                                    ],
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(patterns)
}

pub async fn detect_panic_implementations(repo_path: &str) -> Result<Vec<ModelDuressPattern>> {
    let mut patterns = Vec::new();
    let panic_indicators = [
        "// TODO: find existing", "// FIXME: duplicate", "// HACK:", "// TEMP:",
        "# TODO: find existing", "# FIXME: duplicate", "# HACK:", "# TEMP:",
        "quick implementation", "temporary solution", "couldn't find existing",
        "duplicate functionality", "redundant implementation"
    ];
    let all_files = find_all_source_files(repo_path).await?;
    for file in &all_files {
        if let Ok(content) = tokio::fs::read_to_string(file).await {
            let mut found_indicators = Vec::new();
            for indicator in &panic_indicators {
                if content.to_lowercase().contains(&indicator.to_lowercase()) {
                    found_indicators.push(indicator.to_string());
                }
            }
            if !found_indicators.is_empty() {
                let functionality_hash = extract_functionality_signature(&content);
                for other_file in &all_files {
                    if other_file != file {
                        if let Ok(other_content) = tokio::fs::read_to_string(other_file).await {
                            let other_hash = extract_functionality_signature(&other_content);
                            if signatures_similar(&functionality_hash, &other_hash) {
                                patterns.push(ModelDuressPattern {
                                    pattern_type: "panic_implementation".to_string(),
                                    description: format!("Panic-driven duplicate: {} shows signs of hasty implementation with similar functionality to {}", Path::new(file).file_name().unwrap().to_str().unwrap(), Path::new(other_file).file_name().unwrap().to_str().unwrap()),
                                    original_file: other_file.clone(),
                                    duplicate_file: file.clone(),
                                    confidence: 0.6,
                                    recommendation: "Review panic implementation and consolidate with existing solution".to_string(),
                                    duress_indicators: found_indicators.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(patterns)
}
