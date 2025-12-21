#![allow(clippy::items_after_statements)]
#![allow(clippy::format_push_string)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::unused_async)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::similar_names)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::unused_self)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::module_name_repetitions)]
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
