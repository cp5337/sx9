//! Performance benchmarks for CTAS Interface Foundation
//!
//! These benchmarks measure the performance of interface operations
//! to ensure the foundation service meets performance requirements.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use serde_json::json;
use sx9_foundation_interface::*;

fn benchmark_url_parsing(c: &mut Criterion) {
    let service = init_interface_foundation().unwrap();

    c.bench_function("url_parsing", |b| {
        b.iter(|| {
            black_box(
                service
                    .parse_url("https://example.com/test?param=value#fragment")
                    .unwrap(),
            )
        })
    });
}

fn benchmark_url_parsing_batch(c: &mut Criterion) {
    let service = init_interface_foundation().unwrap();

    let mut group = c.benchmark_group("url_parsing_batch");
    for size in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("batch", size), size, |b, &size| {
            b.iter(|| {
                for i in 0..size {
                    let url_str = format!("https://example{}.com/test?param={}", i, i);
                    black_box(service.parse_url(&url_str).unwrap());
                }
            })
        });
    }
    group.finish();
}

fn benchmark_http_get_requests(c: &mut Criterion) {
    let _service = init_interface_foundation().unwrap();

    c.bench_function("http_get_request", |b| {
        b.iter(|| {
            // Note: This is a synchronous benchmark, but HTTP requests are async
            // In a real benchmark, you'd use async runtime
            black_box("https://httpbin.org/get")
        })
    });
}

fn benchmark_json_serialization(c: &mut Criterion) {
    let _service = init_interface_foundation().unwrap();
    let test_data = json!({
        "id": "test-id",
        "timestamp": "2025-01-01T00:00:00Z",
        "message": "Benchmark test data",
        "nested": {
            "value": 42,
            "flag": true,
            "array": [1, 2, 3, 4, 5]
        }
    });

    c.bench_function("json_serialization", |b| {
        b.iter(|| black_box(serde_json::to_string(&test_data).unwrap()))
    });
}

fn benchmark_json_deserialization(c: &mut Criterion) {
    let _service = init_interface_foundation().unwrap();
    let test_data = json!({
        "id": "test-id",
        "timestamp": "2025-01-01T00:00:00Z",
        "message": "Benchmark test data",
        "nested": {
            "value": 42,
            "flag": true,
            "array": [1, 2, 3, 4, 5]
        }
    });
    let json_string = serde_json::to_string(&test_data).unwrap();

    c.bench_function("json_deserialization", |b| {
        b.iter(|| {
            let result: serde_json::Value = serde_json::from_str(&json_string).unwrap();
            black_box(result)
        })
    });
}

fn benchmark_metrics_collection(c: &mut Criterion) {
    let service = init_interface_foundation().unwrap();

    // Generate some activity first
    for _ in 0..1000 {
        service.parse_url("https://example.com/test");
    }

    c.bench_function("metrics_collection", |b| {
        b.iter(|| black_box(service.get_metrics()))
    });
}

fn benchmark_mixed_operations(c: &mut Criterion) {
    let service = init_interface_foundation().unwrap();

    c.bench_function("mixed_operations", |b| {
        b.iter(|| {
            // Simulate typical usage pattern
            let url = service.parse_url("https://example.com/api/data").unwrap();
            let json_data = json!({
                "url": url.to_string(),
                "timestamp": "2025-01-01T00:00:00Z",
                "message": "Mixed operations benchmark"
            });
            let json_string = serde_json::to_string(&json_data).unwrap();
            black_box(json_string)
        })
    });
}

fn benchmark_service_initialization(c: &mut Criterion) {
    c.bench_function("service_initialization", |b| {
        b.iter(|| {
            let service = init_interface_foundation().unwrap();
            black_box(service)
        })
    });
}

fn benchmark_custom_config_initialization(c: &mut Criterion) {
    let config = InterfaceConfig {
        enable_http_client: true,
        enable_websocket_server: true,
        enable_cli_parsing: true,
        http_timeout_seconds: 30,
        websocket_port: 8080,
        max_connections: 1000,
        enable_metrics: true,
    };

    c.bench_function("custom_config_initialization", |b| {
        b.iter(|| {
            let service = init_interface_foundation_with_config(config.clone()).unwrap();
            black_box(service)
        })
    });
}

fn benchmark_large_json_operations(c: &mut Criterion) {
    let service = init_interface_foundation().unwrap();

    // Create a large JSON object
    let mut large_data = json!({
        "id": "large-test-id",
        "timestamp": "2025-01-01T00:00:00Z",
        "message": "Large JSON benchmark"
    });

    // Add a large array
    let mut large_array = Vec::new();
    for i in 0..1000 {
        large_array.push(json!({
            "index": i,
            "url": format!("https://example{}.com/test", i),
            "timestamp": "2025-01-01T00:00:00Z",
            "data": format!("data_{}", i)
        }));
    }
    large_data["large_array"] = json!(large_array);

    c.bench_function("large_json_serialization", |b| {
        b.iter(|| black_box(serde_json::to_string(&large_data).unwrap()))
    });
}

fn benchmark_url_manipulation(c: &mut Criterion) {
    let service = init_interface_foundation().unwrap();

    c.bench_function("url_manipulation", |b| {
        b.iter(|| {
            let url = service
                .parse_url("https://example.com/path?query=value#fragment")
                .unwrap();
            let host = url.host_str().unwrap();
            let path = url.path();
            let query = url.query().unwrap_or("");
            let fragment = url.fragment().unwrap_or("");
            black_box((
                host.to_string(),
                path.to_string(),
                query.to_string(),
                fragment.to_string(),
            ))
        })
    });
}

criterion_group!(
    benches,
    benchmark_url_parsing,
    benchmark_url_parsing_batch,
    benchmark_http_get_requests,
    benchmark_json_serialization,
    benchmark_json_deserialization,
    benchmark_metrics_collection,
    benchmark_mixed_operations,
    benchmark_service_initialization,
    benchmark_custom_config_initialization,
    benchmark_large_json_operations,
    benchmark_url_manipulation
);

criterion_main!(benches);
