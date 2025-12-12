//! CTAS-7 Neural Mux
//!
//! Ultra-low latency routing via trivariate hash lookup.
//! Target: <250ns routing latency
//!
//! # Architecture
//!
//! The Neural Mux provides O(1) routing decisions based on trivariate hash
//! lookups (SCH + CUID + UUID). It uses lock-free data structures (DashMap)
//! for concurrent access without blocking.
//!
//! # Governing RFCs
//! - RFC-9004: Neural Mux Specification
//! - RFC-9002: Routing Protocol

pub mod metrics;
pub mod route_table;
pub mod router;

pub use route_table::RouteTable;
pub use router::NeuralRouter;

/// Neural Mux configuration
#[derive(Debug, Clone)]
pub struct NeuralMuxConfig {
    /// Maximum route table size
    pub max_routes: usize,
    /// Route TTL in seconds
    pub route_ttl_secs: u64,
    /// Enable metrics collection
    pub metrics_enabled: bool,
    /// Target latency threshold (nanoseconds)
    pub latency_threshold_ns: u64,
}

impl Default for NeuralMuxConfig {
    fn default() -> Self {
        Self {
            max_routes: 1_000_000,
            route_ttl_secs: 3600,
            metrics_enabled: true,
            latency_threshold_ns: 250, // <250ns target
        }
    }
}
