//! Performance benchmarks for CTAS Core Foundation
//! 
//! These benchmarks measure the performance of core operations
//! to ensure the foundation service meets performance requirements.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use ctas_core_foundation::*;
use serde_json::json;

fn benchmark_uuid_generation(c: &mut Criterion) {
    let service = init_foundation().unwrap();
    
    c.bench_function("uuid_generation", |b| {
        b.iter(|| {
            black_box(service.generate_uuid())
        })
    });
}

fn benchmark_uuid_generation_batch(c: &mut Criterion) {
    let service = init_foundation().unwrap();
    
    let mut group = c.benchmark_group("uuid_generation_batch");
    for size in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("batch", size), size, |b, &size| {
            b.iter(|| {
                for _ in 0..size {
                    black_box(service.generate_uuid());
                }
            })
        });
    }
    group.finish();
}

fn benchmark_json_serialization(c: &mut Criterion) {
    let service = init_foundation().unwrap();
    let test_data = json!({
        "id": service.generate_uuid().to_string(),
        "timestamp": service.now().to_rfc3339(),
        "message": "Benchmark test data",
        "nested": {
            "value": 42,
            "flag": true,
            "array": [1, 2, 3, 4, 5]
        }
    });
    
    c.bench_function("json_serialization", |b| {
        b.iter(|| {
            black_box(service.serialize_json(&test_data).unwrap())
        })
    });
}

fn benchmark_json_deserialization(c: &mut Criterion) {
    let service = init_foundation().unwrap();
    let test_data = json!({
        "id": service.generate_uuid().to_string(),
        "timestamp": service.now().to_rfc3339(),
        "message": "Benchmark test data",
        "nested": {
            "value": 42,
            "flag": true,
            "array": [1, 2, 3, 4, 5]
        }
    });
    let json_string = service.serialize_json(&test_data).unwrap();
    
    c.bench_function("json_deserialization", |b| {
        b.iter(|| {
            let result: serde_json::Value = service.deserialize_json(&json_string).unwrap();
            black_box(result)
        })
    });
}

fn benchmark_timestamp_generation(c: &mut Criterion) {
    let service = init_foundation().unwrap();
    
    c.bench_function("timestamp_generation", |b| {
        b.iter(|| {
            black_box(service.now())
        })
    });
}

fn benchmark_logging(c: &mut Criterion) {
    let service = init_foundation().unwrap();
    
    c.bench_function("logging_info", |b| {
        b.iter(|| {
            service.log("info", "Benchmark log message");
        })
    });
    
    c.bench_function("logging_error", |b| {
        b.iter(|| {
            service.log("error", "Benchmark error message");
        })
    });
}

fn benchmark_metrics_collection(c: &mut Criterion) {
    let service = init_foundation().unwrap();
    
    // Generate some activity first
    for _ in 0..1000 {
        service.generate_uuid();
        service.now();
    }
    
    c.bench_function("metrics_collection", |b| {
        b.iter(|| {
            black_box(service.get_metrics())
        })
    });
}

fn benchmark_mixed_operations(c: &mut Criterion) {
    let service = init_foundation().unwrap();
    
    c.bench_function("mixed_operations", |b| {
        b.iter(|| {
            // Simulate typical usage pattern
            let uuid = service.generate_uuid();
            let timestamp = service.now();
            let data = json!({
                "id": uuid.to_string(),
                "timestamp": timestamp.to_rfc3339(),
                "message": "Mixed operations benchmark"
            });
            let json = service.serialize_json(&data).unwrap();
            service.log("info", "Mixed operations completed");
            black_box(json)
        })
    });
}

fn benchmark_service_initialization(c: &mut Criterion) {
    c.bench_function("service_initialization", |b| {
        b.iter(|| {
            let service = init_foundation().unwrap();
            black_box(service)
        })
    });
}

fn benchmark_custom_config_initialization(c: &mut Criterion) {
    let config = FoundationConfig {
        enable_logging: true,
        log_level: "info".to_string(),
        max_connections: 1000,
        timeout_seconds: 30,
        enable_metrics: true,
    };
    
    c.bench_function("custom_config_initialization", |b| {
        b.iter(|| {
            let service = init_foundation_with_config(config.clone()).unwrap();
            black_box(service)
        })
    });
}

fn benchmark_large_json_serialization(c: &mut Criterion) {
    let service = init_foundation().unwrap();
    
    // Create a large JSON object
    let mut large_data = json!({
        "id": service.generate_uuid().to_string(),
        "timestamp": service.now().to_rfc3339(),
        "message": "Large JSON benchmark"
    });
    
    // Add a large array
    let mut large_array = Vec::new();
    for i in 0..1000 {
        large_array.push(json!({
            "index": i,
            "uuid": service.generate_uuid().to_string(),
            "timestamp": service.now().to_rfc3339(),
            "data": format!("data_{}", i)
        }));
    }
    large_data["large_array"] = json!(large_array);
    
    c.bench_function("large_json_serialization", |b| {
        b.iter(|| {
            black_box(service.serialize_json(&large_data).unwrap())
        })
    });
}

criterion_group!(
    benches,
    benchmark_uuid_generation,
    benchmark_uuid_generation_batch,
    benchmark_json_serialization,
    benchmark_json_deserialization,
    benchmark_timestamp_generation,
    benchmark_logging,
    benchmark_metrics_collection,
    benchmark_mixed_operations,
    benchmark_service_initialization,
    benchmark_custom_config_initialization,
    benchmark_large_json_serialization
);

criterion_main!(benches);
