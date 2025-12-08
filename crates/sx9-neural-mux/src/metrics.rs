//! Neural Mux Metrics - Performance monitoring for routing operations

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// Metrics collector for Neural Mux performance
#[derive(Debug)]
pub struct MuxMetrics {
    /// Total routing operations
    pub total_routes: AtomicU64,
    /// Successful route lookups
    pub route_hits: AtomicU64,
    /// Failed route lookups (cache miss)
    pub route_misses: AtomicU64,
    /// Routes under 250ns threshold
    pub fast_routes: AtomicU64,
    /// Routes over 250ns threshold
    pub slow_routes: AtomicU64,
    /// Cumulative latency (nanoseconds) for averaging
    cumulative_latency_ns: AtomicU64,
    /// Start time for uptime calculation
    start_time: Instant,
}

impl Default for MuxMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl MuxMetrics {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            total_routes: AtomicU64::new(0),
            route_hits: AtomicU64::new(0),
            route_misses: AtomicU64::new(0),
            fast_routes: AtomicU64::new(0),
            slow_routes: AtomicU64::new(0),
            cumulative_latency_ns: AtomicU64::new(0),
            start_time: Instant::now(),
        }
    }

    /// Record a routing operation
    pub fn record_route(&self, latency_ns: u64, hit: bool) {
        self.total_routes.fetch_add(1, Ordering::Relaxed);
        self.cumulative_latency_ns.fetch_add(latency_ns, Ordering::Relaxed);

        if hit {
            self.route_hits.fetch_add(1, Ordering::Relaxed);
        } else {
            self.route_misses.fetch_add(1, Ordering::Relaxed);
        }

        // Target: <250ns
        if latency_ns < 250 {
            self.fast_routes.fetch_add(1, Ordering::Relaxed);
        } else {
            self.slow_routes.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Get average latency in nanoseconds
    pub fn avg_latency_ns(&self) -> u64 {
        let total = self.total_routes.load(Ordering::Relaxed);
        if total == 0 {
            return 0;
        }
        self.cumulative_latency_ns.load(Ordering::Relaxed) / total
    }

    /// Get cache hit rate (0.0 - 1.0)
    pub fn hit_rate(&self) -> f64 {
        let total = self.total_routes.load(Ordering::Relaxed);
        if total == 0 {
            return 0.0;
        }
        self.route_hits.load(Ordering::Relaxed) as f64 / total as f64
    }

    /// Get percentage of routes under 250ns threshold
    pub fn fast_route_percentage(&self) -> f64 {
        let total = self.total_routes.load(Ordering::Relaxed);
        if total == 0 {
            return 0.0;
        }
        self.fast_routes.load(Ordering::Relaxed) as f64 / total as f64 * 100.0
    }

    /// Get uptime duration
    pub fn uptime(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    /// Reset all metrics
    pub fn reset(&self) {
        self.total_routes.store(0, Ordering::Relaxed);
        self.route_hits.store(0, Ordering::Relaxed);
        self.route_misses.store(0, Ordering::Relaxed);
        self.fast_routes.store(0, Ordering::Relaxed);
        self.slow_routes.store(0, Ordering::Relaxed);
        self.cumulative_latency_ns.store(0, Ordering::Relaxed);
    }

    /// Export metrics as JSON-compatible struct
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            total_routes: self.total_routes.load(Ordering::Relaxed),
            route_hits: self.route_hits.load(Ordering::Relaxed),
            route_misses: self.route_misses.load(Ordering::Relaxed),
            fast_routes: self.fast_routes.load(Ordering::Relaxed),
            slow_routes: self.slow_routes.load(Ordering::Relaxed),
            avg_latency_ns: self.avg_latency_ns(),
            hit_rate: self.hit_rate(),
            fast_route_percentage: self.fast_route_percentage(),
            uptime_secs: self.uptime().as_secs(),
        }
    }
}

/// Snapshot of metrics for serialization
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetricsSnapshot {
    pub total_routes: u64,
    pub route_hits: u64,
    pub route_misses: u64,
    pub fast_routes: u64,
    pub slow_routes: u64,
    pub avg_latency_ns: u64,
    pub hit_rate: f64,
    pub fast_route_percentage: f64,
    pub uptime_secs: u64,
}
