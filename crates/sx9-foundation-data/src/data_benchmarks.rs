use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sx9_foundation_data::{DataService, DataConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct BenchmarkData {
    id: u32,
    name: String,
    email: String,
    age: u32,
    active: bool,
    tags: Vec<String>,
}

fn create_benchmark_data() -> BenchmarkData {
    BenchmarkData {
        id: 12345,
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        age: 30,
        active: true,
        tags: vec!["user".to_string(), "premium".to_string(), "verified".to_string()],
    }
}

fn create_large_benchmark_data() -> Vec<BenchmarkData> {
    (0..1000)
        .map(|i| BenchmarkData {
            id: i,
            name: format!("User {}", i),
            email: format!("user{}@example.com", i),
            age: 20 + (i % 50),
            active: i % 2 == 0,
            tags: vec![
                "user".to_string(),
                if i % 3 == 0 { "premium".to_string() } else { "basic".to_string() },
                if i % 5 == 0 { "verified".to_string() } else { "unverified".to_string() },
            ],
        })
        .collect()
}

fn benchmark_json_serialization(c: &mut Criterion) {
    let service = DataService::new(DataConfig::default()).unwrap();
    let data = create_benchmark_data();
    
    c.bench_function("json_serialization", |b| {
        b.iter(|| {
            let _ = service.to_json(black_box(&data));
        })
    });
}

fn benchmark_json_deserialization(c: &mut Criterion) {
    let service = DataService::new(DataConfig::default()).unwrap();
    let data = create_benchmark_data();
    let json = service.to_json(&data).unwrap();
    
    c.bench_function("json_deserialization", |b| {
        b.iter(|| {
            let _: BenchmarkData = service.from_json(black_box(&json)).unwrap();
        })
    });
}

fn benchmark_json_roundtrip(c: &mut Criterion) {
    let service = DataService::new(DataConfig::default()).unwrap();
    let data = create_benchmark_data();
    
    c.bench_function("json_roundtrip", |b| {
        b.iter(|| {
            let json = service.to_json(black_box(&data)).unwrap();
            let _: BenchmarkData = service.from_json(black_box(&json)).unwrap();
        })
    });
}

fn benchmark_yaml_serialization(c: &mut Criterion) {
    let service = DataService::new(DataConfig::default()).unwrap();
    let data = create_benchmark_data();
    
    c.bench_function("yaml_serialization", |b| {
        b.iter(|| {
            let _ = service.to_yaml(black_box(&data));
        })
    });
}

fn benchmark_yaml_deserialization(c: &mut Criterion) {
    let service = DataService::new(DataConfig::default()).unwrap();
    let data = create_benchmark_data();
    let yaml = service.to_yaml(&data).unwrap();
    
    c.bench_function("yaml_deserialization", |b| {
        b.iter(|| {
            let _: BenchmarkData = service.from_yaml(black_box(&yaml)).unwrap();
        })
    });
}

fn benchmark_toml_serialization(c: &mut Criterion) {
    let service = DataService::new(DataConfig::default()).unwrap();
    let data = create_benchmark_data();
    
    c.bench_function("toml_serialization", |b| {
        b.iter(|| {
            let _ = service.to_toml(black_box(&data));
        })
    });
}

fn benchmark_toml_deserialization(c: &mut Criterion) {
    let service = DataService::new(DataConfig::default()).unwrap();
    let data = create_benchmark_data();
    let toml = service.to_toml(&data).unwrap();
    
    c.bench_function("toml_deserialization", |b| {
        b.iter(|| {
            let _: BenchmarkData = service.from_toml(black_box(&toml)).unwrap();
        })
    });
}

fn benchmark_uuid_generation(c: &mut Criterion) {
    let service = DataService::new(DataConfig::default()).unwrap();
    
    c.bench_function("uuid_generation", |b| {
        b.iter(|| {
            let _ = service.generate_uuid();
        })
    });
}

fn benchmark_regex_compilation(c: &mut Criterion) {
    let service = DataService::new(DataConfig::default()).unwrap();
    let patterns = vec![
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
        r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$",
        r"^[A-Za-z0-9_-]{8,}$",
        r"^\d{4}-\d{2}-\d{2}$",
        r"^[A-Z]{2}\d{2}[A-Z0-9]{4}\d{7}([A-Z0-9]?){0,16}$",
    ];
    
    c.bench_function("regex_compilation", |b| {
        b.iter(|| {
            for pattern in &patterns {
                let _ = service.compile_regex(black_box(pattern));
            }
        })
    });
}

fn benchmark_csv_parsing(c: &mut Criterion) {
    let service = DataService::new(DataConfig::default()).unwrap();
    let csv_data = "id,name,email,age,active\n1,John,john@example.com,30,true\n2,Jane,jane@example.com,25,false\n3,Bob,bob@example.com,35,true\n4,Alice,alice@example.com,28,false\n5,Charlie,charlie@example.com,42,true";
    
    c.bench_function("csv_parsing", |b| {
        b.iter(|| {
            let _ = service.parse_csv(black_box(csv_data));
        })
    });
}

fn benchmark_data_validation(c: &mut Criterion) {
    let service = DataService::new(DataConfig::default()).unwrap();
    let data = create_benchmark_data();
    
    c.bench_function("data_validation", |b| {
        b.iter(|| {
            let _ = service.validate_data(black_box(&data));
        })
    });
}

fn benchmark_large_dataset_json(c: &mut Criterion) {
    let service = DataService::new(DataConfig::default()).unwrap();
    let data = create_large_benchmark_data();
    
    c.bench_function("large_dataset_json_serialization", |b| {
        b.iter(|| {
            let _ = service.to_json(black_box(&data));
        })
    });
}

fn benchmark_concurrent_operations(c: &mut Criterion) {
    let service = DataService::new(DataConfig::default()).unwrap();
    let data = create_benchmark_data();
    
    c.bench_function("concurrent_operations", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..10).map(|_| {
                let service = service.clone();
                let data = &data;
                std::thread::spawn(move || {
                    for _ in 0..100 {
                        let _ = service.to_json(data);
                        let _ = service.generate_uuid();
                        let _ = service.compile_regex(r"\d+");
                    }
                })
            }).collect();
            
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

criterion_group!(
    benches,
    benchmark_json_serialization,
    benchmark_json_deserialization,
    benchmark_json_roundtrip,
    benchmark_yaml_serialization,
    benchmark_yaml_deserialization,
    benchmark_toml_serialization,
    benchmark_toml_deserialization,
    benchmark_uuid_generation,
    benchmark_regex_compilation,
    benchmark_csv_parsing,
    benchmark_data_validation,
    benchmark_large_dataset_json,
    benchmark_concurrent_operations
);

criterion_main!(benches);
