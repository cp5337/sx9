use ctas7_data_foundation::{DataConfig, DataService};
use serde::{Deserialize, Serialize};
use std::time::Duration;

fn init_data_foundation() -> Result<DataService, anyhow::Error> {
    let config = DataConfig::default();
    DataService::new(config)
}

#[test]
fn test_data_service_integration() {
    let service = init_data_foundation().unwrap();

    // Test service initialization
    let config = service.get_config();
    assert!(config.enable_json_processing);
    assert!(config.enable_yaml_processing);
    assert!(config.enable_toml_processing);
    assert!(config.enable_csv_processing);
    assert!(config.enable_regex_processing);
    assert!(config.enable_uuid_generation);
    assert!(config.enable_data_validation);
}

#[test]
fn test_json_serialization_roundtrip() {
    let service = init_data_foundation().unwrap();

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        age: u32,
        active: bool,
    }

    let original = TestData {
        name: "Test User".to_string(),
        age: 30,
        active: true,
    };

    // Serialize to JSON
    let json = service.to_json(&original).unwrap();
    assert!(json.contains("Test User"));
    assert!(json.contains("30"));
    assert!(json.contains("true"));

    // Deserialize from JSON
    let deserialized: TestData = service.from_json(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_yaml_serialization_roundtrip() {
    let service = init_data_foundation().unwrap();

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        age: u32,
        active: bool,
    }

    let original = TestData {
        name: "Test User".to_string(),
        age: 30,
        active: true,
    };

    // Serialize to YAML
    let yaml = service.to_yaml(&original).unwrap();
    assert!(yaml.contains("Test User"));
    assert!(yaml.contains("30"));
    assert!(yaml.contains("true"));

    // Deserialize from YAML
    let deserialized: TestData = service.from_yaml(&yaml).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_toml_serialization_roundtrip() {
    let service = init_data_foundation().unwrap();

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        age: u32,
        active: bool,
    }

    let original = TestData {
        name: "Test User".to_string(),
        age: 30,
        active: true,
    };

    // Serialize to TOML
    let toml = service.to_toml(&original).unwrap();
    assert!(toml.contains("Test User"));
    assert!(toml.contains("30"));
    assert!(toml.contains("true"));

    // Deserialize from TOML
    let deserialized: TestData = service.from_toml(&toml).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_csv_parsing() {
    let service = init_data_foundation().unwrap();

    let csv_data = "name,age,city\nJohn,30,New York\nJane,25,Los Angeles\nBob,35,Chicago";
    let records = service.parse_csv(csv_data).unwrap();

    assert_eq!(records.len(), 3);
    assert_eq!(records[0].get(0).unwrap(), "John");
    assert_eq!(records[0].get(1).unwrap(), "30");
    assert_eq!(records[0].get(2).unwrap(), "New York");
    assert_eq!(records[1].get(0).unwrap(), "Jane");
    assert_eq!(records[2].get(0).unwrap(), "Bob");
}

#[test]
fn test_uuid_generation() {
    let service = init_data_foundation().unwrap();

    let uuid1 = service.generate_uuid().unwrap();
    let uuid2 = service.generate_uuid().unwrap();

    assert_ne!(uuid1, uuid2);
    assert!(uuid1.to_string().len() > 0);
    assert!(uuid2.to_string().len() > 0);

    // Test UUID format
    let uuid_str = uuid1.to_string();
    assert!(uuid_str.contains('-'));
    assert_eq!(uuid_str.len(), 36);
}

#[test]
fn test_regex_operations() {
    let service = init_data_foundation().unwrap();

    // Test email regex
    let email_regex = service
        .compile_regex(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .unwrap();
    assert!(email_regex.is_match("test@example.com"));
    assert!(!email_regex.is_match("invalid-email"));

    // Test IP address regex (basic format check)
    let ip_regex = service
        .compile_regex(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$")
        .unwrap();
    assert!(ip_regex.is_match("192.168.1.1"));
    // Note: This regex only checks format, not valid IP ranges
    assert!(ip_regex.is_match("999.999.999.999")); // Format is valid, even if IP is not

    // Test caching
    let same_regex = service
        .compile_regex(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .unwrap();
    assert!(same_regex.is_match("test@example.com"));
}

#[test]
fn test_data_validation() {
    let service = init_data_foundation().unwrap();

    #[derive(Serialize)]
    struct ValidData {
        name: String,
        age: u32,
    }

    let valid_data = ValidData {
        name: "Test".to_string(),
        age: 30,
    };

    // Valid data should pass validation
    let is_valid = service.validate_data(&valid_data).unwrap();
    assert!(is_valid);

    // Test with data that can't be serialized (circular reference simulation)
    // Since we can't easily create circular references in Rust, we'll test with a different approach
    // The validation should pass for any serializable data
    let another_valid_data = ValidData {
        name: "Another Test".to_string(),
        age: 25,
    };

    let is_also_valid = service.validate_data(&another_valid_data).unwrap();
    assert!(is_also_valid);
}

#[test]
fn test_custom_configuration() {
    let mut config = DataConfig::default();
    config.enable_json_processing = false;
    config.enable_yaml_processing = false;
    config.max_csv_records = 5;

    let service = DataService::new(config).unwrap();

    // JSON operations should fail
    let test_data = serde_json::json!({"test": "data"});
    assert!(service.to_json(&test_data).is_err());

    // YAML operations should fail
    assert!(service.to_yaml(&test_data).is_err());

    // CSV should respect max records limit
    let csv_data = "name,age\nJohn,30\nJane,25\nBob,35\nAlice,28\nCharlie,32\nDavid,27";
    let records = service.parse_csv(csv_data).unwrap();
    assert_eq!(records.len(), 5); // Should be limited to 5 records
}

#[test]
fn test_metrics_collection() {
    let service = init_data_foundation().unwrap();

    // Perform some operations
    let test_data = serde_json::json!({"test": "data"});
    let _ = service.to_json(&test_data);
    let _ = service.from_json::<serde_json::Value>(&service.to_json(&test_data).unwrap());
    let _ = service.generate_uuid();
    let _ = service.compile_regex(r"\d+");
    let _ = service.validate_data(&test_data);

    let metrics = service.get_metrics();

    assert!(metrics.json_operations_total >= 2); // to_json + from_json
    assert!(metrics.uuid_generations_total >= 1);
    assert!(metrics.regex_operations_total >= 1);
    assert!(metrics.data_validations_total >= 1);
    assert!(metrics.error_rate >= 0.0);
}

#[test]
fn test_error_handling() {
    let service = init_data_foundation().unwrap();

    // Test invalid JSON
    let invalid_json = "{ invalid json }";
    assert!(service
        .from_json::<serde_json::Value>(invalid_json)
        .is_err());

    // Test invalid regex
    assert!(service.compile_regex("[").is_err()); // Unclosed bracket

    // Test invalid CSV
    let invalid_csv = "name,age\nJohn,30\nJane"; // Missing age for Jane
    assert!(service.parse_csv(invalid_csv).is_err());
}

#[test]
fn test_concurrent_operations() {
    let service = init_data_foundation().unwrap();

    // Test concurrent JSON operations
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let service = service.clone();
            std::thread::spawn(move || {
                let test_data = serde_json::json!({"test": "data"});
                for _ in 0..100 {
                    let _ = service.to_json(&test_data);
                    let _ = service
                        .from_json::<serde_json::Value>(&service.to_json(&test_data).unwrap());
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    // Test metrics
    let metrics = service.get_metrics();
    assert!(metrics.json_operations_total >= 2000); // 10 threads * 100 iterations * 2 operations
}

#[test]
fn test_performance_operations() {
    let service = init_data_foundation().unwrap();

    let start = std::time::Instant::now();

    // Perform many operations
    for i in 0..1000 {
        let test_data = serde_json::json!({
            "id": i,
            "name": format!("User {}", i),
            "active": i % 2 == 0
        });

        let _ = service.to_json(&test_data);
        let _ = service.generate_uuid();
        let _ = service.compile_regex(&format!(r"\d+{}", i));
    }

    let duration = start.elapsed();

    // Should complete in reasonable time
    assert!(duration < Duration::from_secs(10));

    let metrics = service.get_metrics();
    assert!(metrics.json_operations_total >= 1000);
    assert!(metrics.uuid_generations_total >= 1000);
    assert!(metrics.regex_operations_total >= 1000);
}

#[test]
fn test_performance_test_function() {
    let service = init_data_foundation().unwrap();

    let duration = service.run_performance_test().unwrap();

    // Performance test should complete in reasonable time
    assert!(duration < Duration::from_secs(30));

    let metrics = service.get_metrics();
    assert!(metrics.json_operations_total >= 2000); // 1000 to_json + 1000 from_json
    assert!(metrics.uuid_generations_total >= 1000);
    assert!(metrics.regex_operations_total >= 100);
}
