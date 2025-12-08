//! Test configuration for CTAS QA System

use crate::{QASystemConfig, CTASQASystem};
use tempfile::TempDir;
use std::fs;

#[tokio::test]
async fn test_qa_system_creation() {
    let config = QASystemConfig::default();
    let qa_system = CTASQASystem::new(config);
    
    // Just verify it can be created without panicking
    assert!(true);
}

#[tokio::test]
async fn test_qa_analysis_with_temp_repo() -> anyhow::Result<()> {
    // Create a temporary directory with some test files
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path().to_str().unwrap();
    
    // Create some test files
    fs::write(
        temp_dir.path().join("test.rs"),
        r#"
fn main() {
    println!("Hello, world!");
}

fn duplicate_function() {
    println!("This is duplicated");
}
"#,
    )?;
    
    fs::write(
        temp_dir.path().join("test2.rs"),
        r#"
fn duplicate_function() {
    println!("This is duplicated");
}
"#,
    )?;
    
    // Create QA system
    let config = QASystemConfig::default();
    let qa_system = CTASQASystem::new(config);
    
    // Run analysis
    let result = qa_system.run_complete_analysis(repo_path).await;
    
    // Should complete without errors
    assert!(result.is_ok());
    
    let report = result.unwrap();
    
    // Check that we got some results
    assert!(!report.repo_path.is_empty());
    assert!(report.overall_score >= 0.0);
    assert!(report.overall_score <= 100.0);
    
    Ok(())
}

#[test]
fn test_config_serialization() {
    let config = QASystemConfig::default();
    
    // Test JSON serialization
    let json = serde_json::to_string(&config).expect("Should serialize to JSON");
    let _deserialized: QASystemConfig = serde_json::from_str(&json).expect("Should deserialize from JSON");
    
    // Test YAML serialization
    let yaml = serde_yaml::to_string(&config).expect("Should serialize to YAML");
    let _deserialized: QASystemConfig = serde_yaml::from_str(&yaml).expect("Should deserialize from YAML");
}
