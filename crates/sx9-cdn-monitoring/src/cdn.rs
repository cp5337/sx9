//! Main CTAS CDN implementation

use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tracing::info;
use uuid::Uuid;

use crate::analytics::CDNAnalytics;
use crate::cache::CacheManager;
use crate::edge::EdgeLocation;
use crate::origin::OriginServer;
use crate::routing::RouteOptimizer;
use crate::types::*;

/// Main CTAS CDN structure
pub struct CTASCDN {
    pub edge_locations: Arc<Mutex<HashMap<Uuid, EdgeLocation>>>,
    pub origin_servers: Arc<Mutex<Vec<OriginServer>>>,
    pub route_optimizer: RouteOptimizer,
    pub cache_manager: CacheManager,
    pub analytics: CDNAnalytics,
}

impl Default for CTASCDN {
    fn default() -> Self {
        Self::new()
    }
}

impl CTASCDN {
    /// Create a new CTAS CDN instance
    pub fn new() -> Self {
        Self {
            edge_locations: Arc::new(Mutex::new(HashMap::new())),
            origin_servers: Arc::new(Mutex::new(Vec::new())),
            route_optimizer: RouteOptimizer::new(),
            cache_manager: CacheManager::new(),
            analytics: CDNAnalytics::new(),
        }
    }

    /// Add an edge location to the CDN
    pub async fn add_edge_location(&self, location: EdgeLocation) -> Result<(), CDNError> {
        let mut edges = self.edge_locations.lock().unwrap();
        edges.insert(location.id, location);
        info!("Added edge location to CDN network");
        Ok(())
    }

    /// Add an origin server to the CDN
    pub async fn add_origin_server(&self, origin: OriginServer) -> Result<(), CDNError> {
        let mut origins = self.origin_servers.lock().unwrap();
        origins.push(origin);
        info!("Added origin server to CDN network");
        Ok(())
    }

    /// Route a request through the CDN
    pub async fn route_request(&self, request: CDNRequest) -> Result<CDNResponse, CDNError> {
        let start_time = Instant::now();

        // Find best edge location
        let edge_location = self.route_optimizer.find_best_edge(&request).await?;

        // Check cache first
        if let Some(cached_response) = self.cache_manager.get_cached_response(&request).await? {
            return Ok(CDNResponse {
                id: Uuid::new_v4(),
                status_code: 200,
                headers: HashMap::new(),
                body: cached_response.data,
                served_from: ServeLocation::EdgeCache,
                response_time: start_time.elapsed(),
                cache_hit: true,
                timestamp: Utc::now(),
            });
        }

        // Route to origin server
        let origin_response = self.route_to_origin(&request).await?;

        // Cache the response
        self.cache_manager
            .cache_response(&request, &origin_response)
            .await?;

        // Update analytics
        self.analytics
            .record_request(&request, &origin_response)
            .await;

        Ok(CDNResponse {
            id: Uuid::new_v4(),
            status_code: 200,
            headers: HashMap::new(),
            body: origin_response,
            served_from: ServeLocation::OriginServer,
            response_time: start_time.elapsed(),
            cache_hit: false,
            timestamp: Utc::now(),
        })
    }

    /// Route request to origin server
    async fn route_to_origin(&self, request: &CDNRequest) -> Result<serde_json::Value, CDNError> {
        let origins = self.origin_servers.lock().unwrap();
        let origin = origins.first().ok_or(CDNError::NoOriginServers)?;

        // Make HTTP request to origin
        // This would use hyper or reqwest in a real implementation
        Ok(serde_json::json!({
            "message": "Response from origin server",
            "endpoint": request.endpoint,
            "timestamp": Utc::now()
        }))
    }

    /// Get CDN analytics
    pub async fn get_analytics(&self) -> CDNAnalytics {
        self.analytics.clone()
    }

    /// Get system health
    pub async fn get_system_health(&self) -> SystemHealth {
        let edges = self.edge_locations.lock().unwrap();
        let origins = self.origin_servers.lock().unwrap();

        let healthy_edges = edges.values().filter(|e| e.is_healthy()).count();
        let healthy_origins = origins.iter().filter(|o| o.is_healthy()).count();

        SystemHealth {
            total_edge_locations: edges.len(),
            healthy_edge_locations: healthy_edges,
            total_origin_servers: origins.len(),
            healthy_origin_servers: healthy_origins,
            overall_health: if healthy_edges > 0 && healthy_origins > 0 {
                "Healthy"
            } else {
                "Degraded"
            }
            .to_string(),
            last_updated: Utc::now(),
        }
    }
}

/// Global CDN instance
lazy_static::lazy_static! {
    pub static ref CTAS_CDN: CTASCDN = CTASCDN::new();
}

/// Convenience functions for CDN operations
pub async fn add_edge_location(location: EdgeLocation) -> Result<(), CDNError> {
    CTAS_CDN.add_edge_location(location).await
}

pub async fn add_origin_server(origin: OriginServer) -> Result<(), CDNError> {
    CTAS_CDN.add_origin_server(origin).await
}

pub async fn route_request(request: CDNRequest) -> Result<CDNResponse, CDNError> {
    CTAS_CDN.route_request(request).await
}

pub async fn get_cdn_analytics() -> CDNAnalytics {
    CTAS_CDN.get_analytics().await
}

pub async fn get_cdn_health() -> SystemHealth {
    CTAS_CDN.get_system_health().await
}
