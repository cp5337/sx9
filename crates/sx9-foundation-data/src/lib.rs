//! SX9 Foundation Data
//!
//! Core data management and persistence layer for SX9 systems.
//! Provides unified data access, trivariate hashing, and storage abstraction.
//!
//! ## Modules
//!
//! - **sledis**: Redis-protocol-compatible cache layer (RFC-9005)
//! - **hash**: Trivariate hashing (RFC-9001)
//! - **persistence**: Persistence management
//! - **storage**: Storage abstraction
//!
//! ## RFC References
//!
//! - RFC-9001: Trivariate Hashing Standard
//! - RFC-9005: Unified Schema Specification

use anyhow::Result;
use chrono::{DateTime, Utc};
use csv::{Reader, StringRecord};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info};
use uuid::Uuid;

// SX9 Foundation Data modules
pub mod ctas_sled_kvs;  // TODO: Rename to sx9_sled_kvs
pub mod hash;
pub mod persistence;
pub mod storage;

// Sledis: Redis-protocol cache layer (RFC-9005)
#[cfg(feature = "sledis")]
pub mod sledis;

// Vector Store: LanceDB-based vector storage (RFC-9005 §3.1)
pub mod vector;

#[cfg(feature = "vector-db")]
pub use vector::{VectorDocument, VectorQueryResult, VectorStore};

pub use ctas_sled_kvs::*;
pub use hash::*;
pub use persistence::*;
pub use storage::*;

// Re-export sled for crates that need direct KVS/cache access
#[cfg(feature = "embedded-db")]
pub use sled;

// Re-export sledis types
#[cfg(feature = "sledis")]
pub use sledis::{SledisEntry, SledisError, SledisServer, SledisStore, SledisValue};

/// CTAS-7 v7.2 Foundation Data Manager
#[derive(Debug, Clone)]
pub struct FoundationDataManager {
    persistence: PersistenceManager,
    data_service: DataService,
}

impl FoundationDataManager {
    /// Create new foundation data manager
    pub fn new() -> Result<Self> {
        let persistence = PersistenceManager::new()?;
        let data_service = DataService::new(DataConfig::default())?;

        Ok(Self {
            persistence,
            data_service,
        })
    }

    /// Create with custom configuration
    pub fn with_config(config: DataConfig) -> Result<Self> {
        let persistence = PersistenceManager::new()?;
        let data_service = DataService::new(config)?;

        Ok(Self {
            persistence,
            data_service,
        })
    }

    /// Store data with CTAS-7 v7.2 trivariate hash
    pub async fn store_with_hash<T>(&self, record_type: String, data: T) -> Result<String>
    where
        T: Serialize,
    {
        let serialized = self.data_service.to_json(&data)?;
        let content = serde_json::from_str(&serialized)?;
        let record = PersistentRecord::new(record_type, content);

        self.persistence.store_record(record).await
    }

    /// Retrieve data by hash
    pub async fn retrieve_by_hash<T>(&self, hash: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        if let Some(record) = self.persistence.get_record_by_hash(hash).await? {
            let serialized = self.data_service.to_json(&record.content)?;
            let data = self.data_service.from_json(&serialized)?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    /// Get persistence manager
    pub fn persistence(&self) -> &PersistenceManager {
        &self.persistence
    }

    /// Get data service
    pub fn data_service(&self) -> &DataService {
        &self.data_service
    }

    /// Get combined metrics
    pub async fn get_combined_metrics(&self) -> Result<CombinedMetrics> {
        let data_metrics = self.data_service.get_metrics();
        let persistence_stats = self.persistence.get_statistics().await?;
        let integrity_report = self.persistence.verify_integrity().await?;

        Ok(CombinedMetrics {
            data_metrics,
            persistence_stats,
            integrity_report,
            timestamp: Utc::now(),
        })
    }
}

/// Combined metrics for CTAS-7 v7.2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedMetrics {
    pub data_metrics: DataMetrics,
    pub persistence_stats: PersistenceStatistics,
    pub integrity_report: IntegrityReport,
    pub timestamp: DateTime<Utc>,
}

impl Default for FoundationDataManager {
    fn default() -> Self {
        Self::new().expect("Failed to create foundation data manager")
    }
}

/// Data foundation service performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMetrics {
    pub initialization_time: Duration,
    pub json_operations_total: u64,
    pub yaml_operations_total: u64,
    pub toml_operations_total: u64,
    pub csv_operations_total: u64,
    pub regex_operations_total: u64,
    pub uuid_generations_total: u64,
    pub data_validations_total: u64,
    pub error_rate: f64,
    pub timestamp: DateTime<Utc>,
}

/// Data foundation service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataConfig {
    pub enable_json_processing: bool,
    pub enable_yaml_processing: bool,
    pub enable_toml_processing: bool,
    pub enable_csv_processing: bool,
    pub enable_regex_processing: bool,
    pub enable_uuid_generation: bool,
    pub enable_data_validation: bool,
    pub max_csv_records: usize,
    pub regex_cache_size: usize,
    pub enable_metrics: bool,
}

impl Default for DataConfig {
    fn default() -> Self {
        Self {
            enable_json_processing: true,
            enable_yaml_processing: true,
            enable_toml_processing: true,
            enable_csv_processing: true,
            enable_regex_processing: true,
            enable_uuid_generation: true,
            enable_data_validation: true,
            max_csv_records: 10000,
            regex_cache_size: 100,
            enable_metrics: true,
        }
    }
}

/// Core Data Foundation Service
#[derive(Debug)]
pub struct DataService {
    config: DataConfig,
    metrics: DataMetrics,
    start_time: Instant,
    json_operations: Arc<std::sync::atomic::AtomicU64>,
    yaml_operations: Arc<std::sync::atomic::AtomicU64>,
    toml_operations: Arc<std::sync::atomic::AtomicU64>,
    csv_operations: Arc<std::sync::atomic::AtomicU64>,
    regex_operations: Arc<std::sync::atomic::AtomicU64>,
    uuid_generations: Arc<std::sync::atomic::AtomicU64>,
    data_validations: Arc<std::sync::atomic::AtomicU64>,
    error_count: Arc<std::sync::atomic::AtomicU64>,
    regex_cache: Arc<std::sync::Mutex<HashMap<String, Regex>>>,
}

impl Clone for DataService {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            metrics: self.metrics.clone(),
            start_time: self.start_time,
            json_operations: Arc::clone(&self.json_operations),
            yaml_operations: Arc::clone(&self.yaml_operations),
            toml_operations: Arc::clone(&self.toml_operations),
            csv_operations: Arc::clone(&self.csv_operations),
            regex_operations: Arc::clone(&self.regex_operations),
            uuid_generations: Arc::clone(&self.uuid_generations),
            data_validations: Arc::clone(&self.data_validations),
            error_count: Arc::clone(&self.error_count),
            regex_cache: Arc::clone(&self.regex_cache),
        }
    }
}

impl DataService {
    /// Initialize the data foundation service
    pub fn new(config: DataConfig) -> Result<Self> {
        let start_time = Instant::now();

        let metrics = DataMetrics {
            initialization_time: start_time.elapsed(),
            json_operations_total: 0,
            yaml_operations_total: 0,
            toml_operations_total: 0,
            csv_operations_total: 0,
            regex_operations_total: 0,
            uuid_generations_total: 0,
            data_validations_total: 0,
            error_rate: 0.0,
            timestamp: Utc::now(),
        };

        info!(
            "Data foundation service initialized in {:?}",
            metrics.initialization_time
        );

        Ok(Self {
            config,
            metrics,
            start_time,
            json_operations: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            yaml_operations: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            toml_operations: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            csv_operations: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            regex_operations: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            uuid_generations: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            data_validations: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            error_count: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            regex_cache: Arc::new(std::sync::Mutex::new(HashMap::new())),
        })
    }

    /// Serialize data to JSON
    pub fn to_json<T: Serialize>(&self, data: &T) -> Result<String> {
        if !self.config.enable_json_processing {
            return Err(anyhow::anyhow!("JSON processing not enabled"));
        }

        self.json_operations
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match serde_json::to_string(data) {
            Ok(json) => {
                debug!("Data serialized to JSON successfully");
                Ok(json)
            }
            Err(e) => {
                self.error_count
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                error!("JSON serialization failed: {}", e);
                Err(e.into())
            }
        }
    }

    /// Deserialize data from JSON
    pub fn from_json<T: for<'de> Deserialize<'de>>(&self, json: &str) -> Result<T> {
        if !self.config.enable_json_processing {
            return Err(anyhow::anyhow!("JSON processing not enabled"));
        }

        self.json_operations
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match serde_json::from_str(json) {
            Ok(data) => {
                debug!("Data deserialized from JSON successfully");
                Ok(data)
            }
            Err(e) => {
                self.error_count
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                error!("JSON deserialization failed: {}", e);
                Err(e.into())
            }
        }
    }

    /// Serialize data to YAML
    pub fn to_yaml<T: Serialize>(&self, data: &T) -> Result<String> {
        if !self.config.enable_yaml_processing {
            return Err(anyhow::anyhow!("YAML processing not enabled"));
        }

        self.yaml_operations
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match serde_yaml::to_string(data) {
            Ok(yaml) => {
                debug!("Data serialized to YAML successfully");
                Ok(yaml)
            }
            Err(e) => {
                self.error_count
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                error!("YAML serialization failed: {}", e);
                Err(e.into())
            }
        }
    }

    /// Deserialize data from YAML
    pub fn from_yaml<T: for<'de> Deserialize<'de>>(&self, yaml: &str) -> Result<T> {
        if !self.config.enable_yaml_processing {
            return Err(anyhow::anyhow!("YAML processing not enabled"));
        }

        self.yaml_operations
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match serde_yaml::from_str(yaml) {
            Ok(data) => {
                debug!("Data deserialized from YAML successfully");
                Ok(data)
            }
            Err(e) => {
                self.error_count
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                error!("YAML deserialization failed: {}", e);
                Err(e.into())
            }
        }
    }

    /// Serialize data to TOML
    pub fn to_toml<T: Serialize>(&self, data: &T) -> Result<String> {
        if !self.config.enable_toml_processing {
            return Err(anyhow::anyhow!("TOML processing not enabled"));
        }

        self.toml_operations
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match toml::to_string(data) {
            Ok(toml) => {
                debug!("Data serialized to TOML successfully");
                Ok(toml)
            }
            Err(e) => {
                self.error_count
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                error!("TOML serialization failed: {}", e);
                Err(e.into())
            }
        }
    }

    /// Deserialize data from TOML
    pub fn from_toml<T: for<'de> Deserialize<'de>>(&self, toml: &str) -> Result<T> {
        if !self.config.enable_toml_processing {
            return Err(anyhow::anyhow!("TOML processing not enabled"));
        }

        self.toml_operations
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match toml::from_str(toml) {
            Ok(data) => {
                debug!("Data deserialized from TOML successfully");
                Ok(data)
            }
            Err(e) => {
                self.error_count
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                error!("TOML deserialization failed: {}", e);
                Err(e.into())
            }
        }
    }

    /// Parse CSV data
    pub fn parse_csv(&self, csv_data: &str) -> Result<Vec<StringRecord>> {
        if !self.config.enable_csv_processing {
            return Err(anyhow::anyhow!("CSV processing not enabled"));
        }

        self.csv_operations
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut reader = Reader::from_reader(csv_data.as_bytes());
        let mut records = Vec::new();

        for result in reader.records() {
            match result {
                Ok(record) => {
                    if records.len() >= self.config.max_csv_records {
                        break;
                    }
                    records.push(record);
                }
                Err(e) => {
                    self.error_count
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    error!("CSV parsing failed: {}", e);
                    return Err(e.into());
                }
            }
        }

        debug!("CSV parsed successfully: {} records", records.len());
        Ok(records)
    }

    /// Generate a new UUID
    pub fn generate_uuid(&self) -> Result<Uuid> {
        if !self.config.enable_uuid_generation {
            return Err(anyhow::anyhow!("UUID generation not enabled"));
        }

        self.uuid_generations
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let uuid = Uuid::new_v4();
        debug!("UUID generated: {}", uuid);
        Ok(uuid)
    }

    /// Compile and cache a regex pattern
    pub fn compile_regex(&self, pattern: &str) -> Result<Regex> {
        if !self.config.enable_regex_processing {
            return Err(anyhow::anyhow!("Regex processing not enabled"));
        }

        self.regex_operations
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        // Check cache first
        {
            let cache = self.regex_cache.lock().unwrap();
            if let Some(regex) = cache.get(pattern) {
                return Ok(regex.clone());
            }
        }

        // Compile and cache
        match Regex::new(pattern) {
            Ok(regex) => {
                let mut cache = self.regex_cache.lock().unwrap();
                if cache.len() < self.config.regex_cache_size {
                    cache.insert(pattern.to_string(), regex.clone());
                }
                debug!("Regex compiled and cached: {}", pattern);
                Ok(regex)
            }
            Err(e) => {
                self.error_count
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                error!("Regex compilation failed: {}", e);
                Err(e.into())
            }
        }
    }

    /// Validate data structure
    pub fn validate_data<T: Serialize>(&self, data: &T) -> Result<bool> {
        if !self.config.enable_data_validation {
            return Err(anyhow::anyhow!("Data validation not enabled"));
        }

        self.data_validations
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        // Basic validation - try to serialize and deserialize
        match self.to_json(data) {
            Ok(json) => match self.from_json::<serde_json::Value>(&json) {
                Ok(_) => {
                    debug!("Data validation successful");
                    Ok(true)
                }
                Err(e) => {
                    self.error_count
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    error!("Data validation failed: {}", e);
                    Ok(false)
                }
            },
            Err(e) => {
                self.error_count
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                error!("Data validation failed: {}", e);
                Ok(false)
            }
        }
    }

    /// Get current performance metrics
    pub fn get_metrics(&self) -> DataMetrics {
        let json_ops = self
            .json_operations
            .load(std::sync::atomic::Ordering::Relaxed);
        let yaml_ops = self
            .yaml_operations
            .load(std::sync::atomic::Ordering::Relaxed);
        let toml_ops = self
            .toml_operations
            .load(std::sync::atomic::Ordering::Relaxed);
        let csv_ops = self
            .csv_operations
            .load(std::sync::atomic::Ordering::Relaxed);
        let regex_ops = self
            .regex_operations
            .load(std::sync::atomic::Ordering::Relaxed);
        let uuid_ops = self
            .uuid_generations
            .load(std::sync::atomic::Ordering::Relaxed);
        let validation_ops = self
            .data_validations
            .load(std::sync::atomic::Ordering::Relaxed);
        let errors = self.error_count.load(std::sync::atomic::Ordering::Relaxed);

        let total_operations =
            json_ops + yaml_ops + toml_ops + csv_ops + regex_ops + uuid_ops + validation_ops;
        let error_rate = if total_operations > 0 {
            (errors as f64 / total_operations as f64) * 100.0
        } else {
            0.0
        };

        DataMetrics {
            initialization_time: self.metrics.initialization_time,
            json_operations_total: json_ops,
            yaml_operations_total: yaml_ops,
            toml_operations_total: toml_ops,
            csv_operations_total: csv_ops,
            regex_operations_total: regex_ops,
            uuid_generations_total: uuid_ops,
            data_validations_total: validation_ops,
            error_rate,
            timestamp: Utc::now(),
        }
    }

    /// Get the current configuration
    pub fn get_config(&self) -> &DataConfig {
        &self.config
    }

    /// Run performance test
    pub fn run_performance_test(&self) -> Result<Duration> {
        let start = Instant::now();

        // Test JSON operations
        let test_data = serde_json::json!({
            "test": "data",
            "number": 42,
            "array": [1, 2, 3]
        });

        for _ in 0..1000 {
            let _ = self.to_json(&test_data)?;
            let _ = self.from_json::<serde_json::Value>(&self.to_json(&test_data)?)?;
        }

        // Test UUID generation
        for _ in 0..1000 {
            let _ = self.generate_uuid()?;
        }

        // Test regex operations
        for i in 0..100 {
            let pattern = format!(r"\d+\.\d+\.\d+\.\d+{}", i);
            let _ = self.compile_regex(&pattern)?;
        }

        let duration = start.elapsed();
        info!("Performance test completed in {:?}", duration);
        Ok(duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_data_foundation() -> Result<DataService> {
        let config = DataConfig::default();
        DataService::new(config)
    }

    #[test]
    fn test_data_service_initialization() {
        let service = init_data_foundation().unwrap();
        assert!(service.config.enable_json_processing);
        assert!(service.config.enable_yaml_processing);
        assert!(service.config.enable_toml_processing);
        assert!(service.config.enable_csv_processing);
        assert!(service.config.enable_regex_processing);
        assert!(service.config.enable_uuid_generation);
        assert!(service.config.enable_data_validation);
    }

    #[test]
    fn test_json_operations() {
        let service = init_data_foundation().unwrap();
        let test_data = serde_json::json!({"test": "data", "number": 42});

        let json = service.to_json(&test_data).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("data"));
        assert!(json.contains("42"));

        let parsed: serde_json::Value = service.from_json(&json).unwrap();
        assert_eq!(parsed["test"], "data");
        assert_eq!(parsed["number"], 42);
    }

    #[test]
    fn test_uuid_generation() {
        let service = init_data_foundation().unwrap();
        let uuid1 = service.generate_uuid().unwrap();
        let uuid2 = service.generate_uuid().unwrap();

        assert_ne!(uuid1, uuid2);
        assert!(uuid1.to_string().len() > 0);
        assert!(uuid2.to_string().len() > 0);
    }

    #[test]
    fn test_regex_compilation() {
        let service = init_data_foundation().unwrap();
        let regex = service.compile_regex(r"\d+").unwrap();

        assert!(regex.is_match("123"));
        assert!(!regex.is_match("abc"));
    }

    #[test]
    fn test_performance_test() {
        let service = init_data_foundation().unwrap();
        let duration = service.run_performance_test().unwrap();

        // Performance test should complete in reasonable time
        assert!(duration < Duration::from_secs(10));
    }
}

// Re-export commonly used data processing dependencies
pub use anyhow::Error as AnyError;
pub use serde_json;
pub use serde_yaml;
pub use toml;
