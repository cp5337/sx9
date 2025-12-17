//! Core types for CTAS CDN

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

/// Geographic location for edge nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub city: String,
    pub country: String,
    pub region: String,
}

/// Edge location status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeStatus {
    Online,
    Offline,
    Maintenance,
    Overloaded,
}

/// Origin server status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OriginStatus {
    Healthy,
    Unhealthy,
    Maintenance,
    Unknown,
}

/// Where the response was served from
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServeLocation {
    EdgeCache,
    OriginServer,
    FallbackCache,
}

/// CDN request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CDNRequest {
    pub id: Uuid,
    pub endpoint: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
    pub client_location: Option<GeographicLocation>,
    pub timestamp: DateTime<Utc>,
}

/// CDN response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CDNResponse {
    pub id: Uuid,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: serde_json::Value,
    pub served_from: ServeLocation,
    pub response_time: Duration,
    pub cache_hit: bool,
    pub timestamp: DateTime<Utc>,
}

/// System health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub total_edge_locations: usize,
    pub healthy_edge_locations: usize,
    pub total_origin_servers: usize,
    pub healthy_origin_servers: usize,
    pub overall_health: String,
    pub last_updated: DateTime<Utc>,
}

/// CDN error types
#[derive(Debug, thiserror::Error)]
pub enum CDNError {
    #[error("No origin servers available")]
    NoOriginServers,
    #[error("No edge locations available")]
    NoEdgeLocations,
    #[error("Request blocked by security filter")]
    RequestBlocked,
    #[error("Cache error: {0}")]
    CacheError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    #[error("Port {0} is out of range")]
    PortOutOfRange(u16),
    #[error("Operation not found: {0}")]
    OperationNotFound(String),
}
