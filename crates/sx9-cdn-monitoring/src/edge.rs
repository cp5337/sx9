//! Edge location implementation

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

use crate::types::{EdgeStatus, GeographicLocation};

/// Edge location in the CDN network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeLocation {
    pub id: Uuid,
    pub location: GeographicLocation,
    pub cache: LocalCache,
    pub api_server: EdgeAPI,
    pub performance_metrics: PerformanceMetrics,
    pub status: EdgeStatus,
}

/// Local cache at edge location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalCache {
    pub cache_store: HashMap<String, CachedResponse>,
    pub cache_ttl: Duration,
    pub max_size: usize,
    pub hit_rate: f64,
    pub miss_rate: f64,
}

/// Cached response from origin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedResponse {
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub ttl: Duration,
    pub cache_key: String,
    pub origin_url: String,
}

/// Edge API server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeAPI {
    pub port: u16,
    pub endpoints: Vec<String>,
    pub rate_limit: u32,
    pub active_connections: u32,
    pub response_time_avg: Duration,
}

/// Performance metrics for edge location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub requests_per_second: f64,
    pub response_time_avg: Duration,
    pub response_time_p95: Duration,
    pub response_time_p99: Duration,
    pub error_rate: f64,
    pub uptime: f64,
    pub bandwidth_usage: f64,
}

impl EdgeLocation {
    pub fn new(location: GeographicLocation, port: u16) -> Self {
        Self {
            id: Uuid::new_v4(),
            location,
            cache: LocalCache::new(),
            api_server: EdgeAPI::new(port),
            performance_metrics: PerformanceMetrics::new(),
            status: EdgeStatus::Online,
        }
    }

    pub fn is_healthy(&self) -> bool {
        matches!(self.status, EdgeStatus::Online)
    }
}

impl Default for LocalCache {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalCache {
    pub fn new() -> Self {
        Self {
            cache_store: HashMap::new(),
            cache_ttl: Duration::from_secs(300), // 5 minutes
            max_size: 1000,
            hit_rate: 0.0,
            miss_rate: 0.0,
        }
    }
}

impl EdgeAPI {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            endpoints: Vec::new(),
            rate_limit: 1000,
            active_connections: 0,
            response_time_avg: Duration::from_millis(50),
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            requests_per_second: 0.0,
            response_time_avg: Duration::from_millis(0),
            response_time_p95: Duration::from_millis(0),
            response_time_p99: Duration::from_millis(0),
            error_rate: 0.0,
            uptime: 100.0,
            bandwidth_usage: 0.0,
        }
    }
}
