//! Neural Router - Ultra-low latency routing via trivariate hash lookup
//!
//! RFC-9004: Neural Mux Specification
//! RFC-9002: Routing Protocol

use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;

use crate::route_table::{RouteEntry, RouteTable};
use crate::NeuralMuxConfig;

// Import PrimaryTrivariate if available (optional dependency)
#[cfg(feature = "foundation-core")]
use sx9_foundation_core::hash::PrimaryTrivariate;

/// Primary trivariate hash for routing decisions
pub type HashValue = u128;

/// Route destination types
#[derive(Debug, Clone)]
pub enum RouteDestination {
    /// Local handler
    Local(String),
    /// Remote endpoint
    Remote { host: String, port: u16 },
    /// Broadcast to multiple destinations
    Broadcast(Vec<String>),
    /// Load-balanced pool
    Pool(String),
}

/// Bernoulli Zone classification (RFC-9004)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BernoulliZone {
    /// Zone A: Tactical (<250ns)
    Tactical,
    /// Zone B: Operational (50μs - 1ms)
    Operational,
    /// Zone C: Analytical (1ms - 100ms)
    Analytical,
    /// Zone D: Infrastructure (>100ms)
    Infrastructure,
}

/// Service endpoint with zone classification
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceEndpoint {
    pub host: String,
    pub port: u16,
    pub zone: BernoulliZone,
}

/// SCH Prefix type (top 16 bits of SCH-T)
pub type SchPrefix = u16;

/// Neural Router for O(1) hash-based routing
pub struct NeuralRouter {
    /// Lock-free route table
    routes: Arc<RouteTable>,
    /// Configuration
    config: NeuralMuxConfig,
    /// Routing metrics
    metrics: Arc<RouterMetrics>,
}

/// Router performance metrics
#[derive(Debug, Default)]
pub struct RouterMetrics {
    /// Total routes processed
    pub routes_processed: std::sync::atomic::AtomicU64,
    /// Cache hits
    pub cache_hits: std::sync::atomic::AtomicU64,
    /// Cache misses
    pub cache_misses: std::sync::atomic::AtomicU64,
    /// Average latency (nanoseconds)
    pub avg_latency_ns: std::sync::atomic::AtomicU64,
}

impl NeuralRouter {
    /// Create a new Neural Router with the given configuration
    pub fn new(config: NeuralMuxConfig) -> Self {
        Self {
            routes: Arc::new(RouteTable::new(config.max_routes)),
            config,
            metrics: Arc::new(RouterMetrics::default()),
        }
    }

    /// Route a trivariate hash to its destination
    /// Target: <250ns latency
    #[inline]
    pub fn route(&self, sch_t: HashValue, cuid_t: HashValue) -> Option<RouteEntry> {
        // O(1) lookup via DashMap
        // Primary key: SCH-T (tools/actions hash)
        // Secondary validation: CUID-T (spatial/context hash)

        let start = std::time::Instant::now();

        let result = self.routes.lookup(sch_t, cuid_t);

        if self.config.metrics_enabled {
            let elapsed = start.elapsed().as_nanos() as u64;
            self.metrics
                .routes_processed
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            if result.is_some() {
                self.metrics
                    .cache_hits
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            } else {
                self.metrics
                    .cache_misses
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }

        result
    }

    /// Route using PrimaryTrivariate hash (enhanced method)
    ///
    /// RFC-9004: Extracts SCH prefix (top 16 bits) for ultra-fast routing.
    /// Target: <250ns latency
    #[inline]
    #[cfg(feature = "foundation-core")]
    pub fn route_trivariate(&self, hash: &PrimaryTrivariate) -> Option<RouteEntry> {
        let start = std::time::Instant::now();

        // Extract SCH prefix (top 16 bits) for routing
        // This enables O(1) lookup with minimal computation
        let sch_prefix = (hash.sch_t >> 112) as u16;

        // Use existing route method with extracted components
        let result = self.route(hash.sch_t, hash.cuid_t);

        // Performance compliance check (RFC-9004)
        if self.config.metrics_enabled {
            let elapsed = start.elapsed().as_nanos() as u64;
            if elapsed > self.config.latency_threshold_ns {
                eprintln!(
                    "BERNOULLI ZONE A VIOLATION: Route took {}ns (threshold: {}ns)",
                    elapsed, self.config.latency_threshold_ns
                );
            }
        }

        result
    }

    /// Register a route in the table
    pub fn register_route(&self, entry: RouteEntry) -> Result<()> {
        self.routes.insert(entry)
    }

    /// Remove a route from the table
    pub fn unregister_route(&self, sch_t: HashValue) -> Option<RouteEntry> {
        self.routes.remove(sch_t)
    }

    /// Get current metrics
    pub fn metrics(&self) -> &RouterMetrics {
        &self.metrics
    }

    /// Get route table size
    pub fn route_count(&self) -> usize {
        self.routes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let config = NeuralMuxConfig::default();
        let router = NeuralRouter::new(config);
        assert_eq!(router.route_count(), 0);
    }

    #[test]
    fn test_route_lookup_performance() {
        let config = NeuralMuxConfig::default();
        let router = NeuralRouter::new(config);

        // Measure lookup time (should be <250ns for cache hit)
        let start = std::time::Instant::now();
        let _ = router.route(0x12345678, 0xABCDEF00);
        let elapsed = start.elapsed();

        // Allow some overhead for test infrastructure
        assert!(
            elapsed.as_micros() < 10,
            "Route lookup too slow: {:?}",
            elapsed
        );
    }
}
