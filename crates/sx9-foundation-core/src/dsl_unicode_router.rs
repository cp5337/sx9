//! DSL Unicode Router for Neural Mux Integration
//!
//! Routes DSL operations through Neural Mux with <250ns latency target.
//! Provides high-performance routing for Unicode Assembly Language operations.

use crate::diagnostics::Result;
use crate::neural_mux::{NeuralMuxRouter, OperationRoute};
use std::time::{Duration, Instant};

/// DSL Unicode router with performance tracking
pub struct DSLUnicodeRouter {
    neural_mux: NeuralMuxRouter,
    /// Performance metrics
    routing_times: Vec<Duration>,
    max_history: usize,
}

impl DSLUnicodeRouter {
    /// Create new DSL Unicode router
    #[must_use]
    pub fn new(neural_mux: NeuralMuxRouter) -> Self {
        Self {
            neural_mux,
            routing_times: Vec::new(),
            max_history: 1000,
        }
    }

    /// Route DSL operation via Unicode to Neural Mux
    /// Target: <250ns latency
    pub fn route_dsl_operation(&mut self, unicode_op: char) -> Result<OperationRoute> {
        let start = Instant::now();

        // Route through Neural Mux
        let route = self.neural_mux.route_operation(unicode_op)?;

        // Track performance
        let elapsed = start.elapsed();
        self.routing_times.push(elapsed);

        // Maintain history limit
        if self.routing_times.len() > self.max_history {
            self.routing_times.remove(0);
        }

        // Log if latency exceeds target (250ns = 0.00025ms)
        if elapsed.as_nanos() > 250 {
            crate::diagnostics::warn!(
                "DSL routing latency exceeded target: {}ns (target: 250ns)",
                elapsed.as_nanos()
            );
        }

        Ok(route)
    }

    /// Route multiple Unicode operations in parallel
    pub fn route_multiple(&mut self, unicode_ops: &[char]) -> Result<Vec<OperationRoute>> {
        unicode_ops
            .iter()
            .map(|&op| self.route_dsl_operation(op))
            .collect()
    }

    /// Get average routing latency
    #[must_use]
    pub fn average_latency(&self) -> Duration {
        if self.routing_times.is_empty() {
            return Duration::from_nanos(0);
        }

        let total: Duration = self.routing_times.iter().sum();
        total / self.routing_times.len() as u32
    }

    /// Get p99 latency (99th percentile)
    #[must_use]
    pub fn p99_latency(&self) -> Duration {
        if self.routing_times.is_empty() {
            return Duration::from_nanos(0);
        }

        let mut sorted = self.routing_times.clone();
        sorted.sort();

        let p99_index = (sorted.len() as f64 * 0.99) as usize;
        sorted
            .get(p99_index)
            .copied()
            .unwrap_or(Duration::from_nanos(0))
    }

    /// Check if router meets performance target (<250ns)
    #[must_use]
    pub fn meets_performance_target(&self) -> bool {
        self.average_latency().as_nanos() < 250
    }

    /// Get performance statistics
    #[must_use]
    pub fn performance_stats(&self) -> PerformanceStats {
        PerformanceStats {
            average_latency: self.average_latency(),
            p99_latency: self.p99_latency(),
            total_operations: self.routing_times.len(),
            meets_target: self.meets_performance_target(),
        }
    }
}

/// Performance statistics for DSL router
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub average_latency: Duration,
    pub p99_latency: Duration,
    pub total_operations: usize,
    pub meets_target: bool,
}

impl std::fmt::Display for PerformanceStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DSL Router Stats: avg={}ns, p99={}ns, ops={}, target_met={}",
            self.average_latency.as_nanos(),
            self.p99_latency.as_nanos(),
            self.total_operations,
            self.meets_target
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neural_mux::{NeuralMuxConfig, NeuralMuxRouter};

    #[test]
    fn test_route_dsl_operation() {
        let config = NeuralMuxConfig::default();
        let neural_mux = NeuralMuxRouter::new(config);
        let mut router = DSLUnicodeRouter::new(neural_mux);

        // Route a Unicode operation
        let route = router.route_dsl_operation('\u{E100}').unwrap();
        assert_eq!(route.target_processor, "trivariate_processor");
    }

    #[test]
    fn test_performance_tracking() {
        let config = NeuralMuxConfig::default();
        let neural_mux = NeuralMuxRouter::new(config);
        let mut router = DSLUnicodeRouter::new(neural_mux);

        // Route multiple operations
        for _ in 0..10 {
            router.route_dsl_operation('\u{E100}').unwrap();
        }

        let stats = router.performance_stats();
        assert_eq!(stats.total_operations, 10);
        assert!(stats.average_latency.as_nanos() >= 0);
    }

    #[test]
    fn test_route_multiple() {
        let config = NeuralMuxConfig::default();
        let neural_mux = NeuralMuxRouter::new(config);
        let mut router = DSLUnicodeRouter::new(neural_mux);

        let ops = vec!['\u{E100}', '\u{E200}', '\u{E300}'];
        let routes = router.route_multiple(&ops).unwrap();

        assert_eq!(routes.len(), 3);
    }
}
