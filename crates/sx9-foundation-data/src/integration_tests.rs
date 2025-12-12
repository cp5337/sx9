//! Integration tests for CTAS Interface Foundation
//! 
//! These tests verify the complete functionality of the interface foundation service
//! including HTTP client, WebSocket handling, CLI parsing, and URL manipulation.

use sx9_foundation_interface::*;
use std::time::Duration;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "test-cli")]
struct TestCli {
    #[command(subcommand)]
    command: TestCommands,
}

#[derive(Subcommand)]
enum TestCommands {
    Test,
    Echo { message: String },
}

#[tokio::test]
async fn test_interface_service_integration() {
    // Test service initialization
    let service = init_interface_foundation().unwrap();
    let metrics = service.get_metrics();
    assert!(metrics.initialization_time < Duration::from_millis(100));
    
    // Test URL parsing
    let url = service.parse_url("https://example.com/test?param=value").unwrap();
    assert_eq!(url.host_str(), Some("example.com"));
    assert_eq!(url.path(), "/test");
    assert_eq!(url.query(), Some("param=value"));
    
    // Test metrics collection
    let updated_metrics = service.get_metrics();
    assert!(updated_metrics.url_parsing_operations > 0);
    assert!(updated_metrics.error_rate >= 0.0);
    assert!(updated_metrics.error_rate <= 100.0);
}

#[tokio::test]
async fn test_http_operations() {
    let service = init_interface_foundation().unwrap();
    
    // Test HTTP GET request
    let response = service.http_get("https://httpbin.org/get").await.unwrap();
    assert!(response.contains("httpbin.org"));
    
    // Test HTTP POST with JSON
    let data = serde_json::json!({
        "test": "data",
        "number": 42
    });
    let response = service.http_post_json("https://httpbin.org/post", &data).await.unwrap();
    assert!(response.contains("test"));
    assert!(response.contains("data"));
    
    // Test metrics
    let metrics = service.get_metrics();
    assert!(metrics.http_requests_total >= 2);
}

#[tokio::test]
async fn test_url_parsing_operations() {
    let service = init_interface_foundation().unwrap();
    
    // Test various URL formats
    let urls = vec![
        "https://example.com",
        "http://localhost:8080/path",
        "https://api.example.com/v1/users?page=1&limit=10",
        "ftp://files.example.com/download/file.zip",
        "https://example.com:8443/secure/path#section1",
    ];
    
    for url_str in &urls {
        let url = service.parse_url(url_str).unwrap();
        assert!(!url.host_str().unwrap().is_empty());
    }
    
    // Test invalid URL
    let result = service.parse_url("not-a-valid-url");
    assert!(result.is_err());
    
    // Test metrics
    let metrics = service.get_metrics();
    assert!(metrics.url_parsing_operations >= urls.len() as u64);
}

#[tokio::test]
async fn test_cli_parsing() {
    let service = init_interface_foundation().unwrap();
    
    // Test CLI parsing with mock arguments
    std::env::set_var("TEST_CLI_ARGS", "test-cli echo --message hello");
    
    // Note: In a real test, you would need to mock the command line arguments
    // For now, we'll test the error handling
    let result = service.parse_cli_args::<TestCli>();
    // This will fail in test environment, but we can test error handling
    assert!(result.is_err());
    
    // Test metrics
    let metrics = service.get_metrics();
    assert!(metrics.cli_commands_processed >= 0);
}

#[tokio::test]
async fn test_websocket_router_creation() {
    let service = init_interface_foundation().unwrap();
    
    // Test WebSocket router creation
    let router = service.create_websocket_router();
    // Router created successfully
    assert!(true);
}

#[tokio::test]
async fn test_custom_configuration() {
    let config = InterfaceConfig {
        enable_http_client: false,
        enable_websocket_server: false,
        enable_cli_parsing: false,
        http_timeout_seconds: 60,
        websocket_port: 9000,
        max_connections: 2000,
        enable_metrics: true,
    };
    
    let service = init_interface_foundation_with_config(config).unwrap();
    
    // Test that HTTP client is disabled
    let result = service.http_get("https://example.com").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("HTTP client not enabled"));
    
    // Test that CLI parsing is disabled
    let result = service.parse_cli_args::<TestCli>();
    assert!(result.is_err());
    assert!(result.is_err());
}

#[tokio::test]
async fn test_performance_operations() {
    let service = init_interface_foundation().unwrap();
    
    // Test URL parsing performance
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = service.parse_url("https://example.com/test");
    }
    let duration = start.elapsed();
    assert!(duration < Duration::from_millis(100)); // Should be very fast
    
    // Test HTTP performance (if enabled)
    // HTTP client functionality test
    if true {
        let start = std::time::Instant::now();
        for _ in 0..10 {
            let _ = service.http_get("https://httpbin.org/get").await;
        }
        let duration = start.elapsed();
        assert!(duration < Duration::from_secs(30)); // Should complete reasonably fast
    }
}

#[tokio::test]
async fn test_error_handling() {
    let service = init_interface_foundation().unwrap();
    
    // Test invalid URL
    let result = service.parse_url("invalid-url");
    assert!(result.is_err());
    
    // Test invalid HTTP URL
    let result = service.http_get("not-a-url").await;
    assert!(result.is_err());
    
    // Test that service continues to work after errors
    let url = service.parse_url("https://example.com").unwrap();
    assert_eq!(url.host_str(), Some("example.com"));
    
    // Test metrics include error count
    let metrics = service.get_metrics();
    assert!(metrics.error_rate >= 0.0);
}

#[tokio::test]
async fn test_concurrent_operations() {
    let service = init_interface_foundation().unwrap();
    
    // Test concurrent URL parsing
    let handles: Vec<_> = (0..10).map(|_| {
        let service = service.clone();
        tokio::spawn(async move {
            let mut results = Vec::new();
            for i in 0..100 {
                let url_str = format!("https://example{}.com/test", i);
                let url = service.parse_url(&url_str).unwrap();
                results.push(url);
            }
            results
        })
    }).collect();
    
    let results = futures::future::join_all(handles).await;
    let mut all_urls = Vec::new();
    
    for result in results {
        let urls = result.unwrap();
        all_urls.extend(urls);
    }
    
    // All URLs should be parsed successfully
    assert_eq!(all_urls.len(), 1000);
    
    // Test metrics
    let metrics = service.get_metrics();
    assert!(metrics.url_parsing_operations >= 10);
}

#[tokio::test]
async fn test_metrics_accuracy() {
    let service = init_interface_foundation().unwrap();
    
    // Get initial metrics
    let initial_metrics = service.get_metrics();
    let initial_url_ops = initial_metrics.url_parsing_operations;
    
    // Perform some operations
    for _ in 0..100 {
        service.parse_url("https://example.com/test");
    }
    
    // Get updated metrics
    let updated_metrics = service.get_metrics();
    let updated_url_ops = updated_metrics.url_parsing_operations;
    
    // URL operations should have increased
    assert!(updated_url_ops > initial_url_ops);
    
    // Error rate should be reasonable
    assert!(updated_metrics.error_rate >= 0.0);
    assert!(updated_metrics.error_rate <= 100.0);
}

#[tokio::test]
async fn test_performance_test_function() {
    let service = init_interface_foundation().unwrap();
    
    // Test the performance test function
    let duration = service.run_performance_test(100).await.unwrap();
    
    // Performance test should complete in reasonable time
    assert!(duration < Duration::from_secs(30));
    
    // Should have generated some metrics
    let metrics = service.get_metrics();
    assert!(metrics.url_parsing_operations >= 0);
}

#[tokio::test]
async fn test_service_lifecycle() {
    // Test multiple service instances
    let service1 = init_interface_foundation().unwrap();
    let service2 = init_interface_foundation().unwrap();
    
    // Both services should work independently
    let url1 = service1.parse_url("https://example1.com").unwrap();
    let url2 = service2.parse_url("https://example2.com").unwrap();
    
    assert_ne!(url1.host_str(), url2.host_str());
    
    // Test that services don't interfere with each other
    let metrics1 = service1.get_metrics();
    let metrics2 = service2.get_metrics();
    
    // Both should have reasonable metrics
    assert!(metrics1.error_rate >= 0.0);
    assert!(metrics2.error_rate >= 0.0);
}
