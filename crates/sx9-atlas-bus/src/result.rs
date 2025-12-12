//! ATLAS result definitions
//!
//! Results are sent from ATLAS daemon (Legion ECS) back to apecs (async world).

use crate::command::MAX_BATCH_NODES;

/// Result from ATLAS daemon
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AtlasResult {
    /// Result type and payload
    pub kind: ResultKind,

    /// Request ID for correlation with command
    pub request_id: u32,

    /// Tick ID when result was generated
    pub tick_id: u64,

    /// Processing time in nanoseconds
    pub latency_ns: u64,

    /// Success/error flag
    pub success: bool,

    /// Error code (0 = success)
    pub error_code: u8,

    /// Padding
    _pad: [u8; 6],
}

/// Result variants
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ResultKind {
    // ========================================================================
    // Graph Results
    // ========================================================================
    /// Shortest path result
    Path {
        /// Path as node IDs
        nodes: [u32; MAX_BATCH_NODES],
        /// Number of nodes in path
        count: u8,
        /// Total path cost/distance
        cost: f32,
    },

    /// BFS result
    BfsResult {
        /// Discovered nodes
        nodes: [u32; MAX_BATCH_NODES],
        /// Number of nodes discovered
        count: u8,
        /// Depths of each node
        depths: [u8; MAX_BATCH_NODES],
    },

    /// All paths result (just count, paths stored separately)
    AllPathsCount { count: u32 },

    // ========================================================================
    // GLAF Results
    // ========================================================================
    /// Matroid rank result
    MatroidRank { rank: u32 },

    /// Independence check result
    Independent { is_independent: bool },

    /// Maximum independent set
    MaxIndependentSet {
        nodes: [u32; MAX_BATCH_NODES],
        count: u8,
    },

    // ========================================================================
    // Convergence Results
    // ========================================================================
    /// Convergence check result
    Convergence {
        converged: bool,
        current_delta: f32,
        ticks_stable: u16,
    },

    /// Convergence rate
    ConvergenceRate {
        rate: f32,
        trend: i8, // -1 = diverging, 0 = stable, 1 = converging
    },

    // ========================================================================
    // Hash Results
    // ========================================================================
    /// Batch hash result (pointer to output buffer)
    BatchHash {
        /// Pointer to output buffer
        output_ptr: u64,
        /// Number of hashes generated
        count: u32,
    },

    /// Trivariate hash result
    TrivariateHash { sch: u64, cuid: u128, uuid: u128 },

    // ========================================================================
    // Tick Results
    // ========================================================================
    /// Tick sync acknowledgment
    TickAck { tick_id: u64, drift_ns: i64 },

    /// Tick state
    TickState {
        current_tick: u64,
        tick_rate_ns: u64,
        ticks_processed: u64,
    },

    // ========================================================================
    // SDT Results
    // ========================================================================
    /// SDT state
    SdtState {
        gate_id: u32,
        state: u8, // 0=OFF, 1=PRIMED, 2=CONDUCTING, 3=LATCHED
        last_trigger_tick: u64,
    },

    // ========================================================================
    // Plasma Results
    // ========================================================================
    /// Plasma state
    PlasmaState {
        field_id: u32,
        delta_angle: u16,
        entropy: u32,
        excited: bool,
    },

    // ========================================================================
    // Control Results
    // ========================================================================
    /// Pong response
    Pong {
        seq: u32,
        request_timestamp_ns: u64,
        response_timestamp_ns: u64,
    },

    /// Stats response
    Stats {
        commands_processed: u64,
        avg_latency_ns: u64,
        queue_depth: u32,
        uptime_secs: u64,
    },

    /// Generic acknowledgment
    Ack,

    /// Error result
    Error {
        code: u16,
        // Message stored elsewhere
    },
}

impl AtlasResult {
    /// Create a successful result
    #[inline]
    pub fn ok(kind: ResultKind, request_id: u32) -> Self {
        Self {
            kind,
            request_id,
            tick_id: 0,
            latency_ns: 0,
            success: true,
            error_code: 0,
            _pad: [0; 6],
        }
    }

    /// Create an error result
    #[inline]
    pub fn err(code: u8, request_id: u32) -> Self {
        Self {
            kind: ResultKind::Error { code: code as u16 },
            request_id,
            tick_id: 0,
            latency_ns: 0,
            success: false,
            error_code: code,
            _pad: [0; 6],
        }
    }

    /// Set tick ID
    #[inline]
    pub fn with_tick(mut self, tick_id: u64) -> Self {
        self.tick_id = tick_id;
        self
    }

    /// Set latency
    #[inline]
    pub fn with_latency(mut self, latency_ns: u64) -> Self {
        self.latency_ns = latency_ns;
        self
    }
}

/// Error codes
pub mod error {
    pub const OK: u8 = 0;
    pub const UNKNOWN_COMMAND: u8 = 1;
    pub const INVALID_NODE: u8 = 2;
    pub const PATH_NOT_FOUND: u8 = 3;
    pub const BUFFER_FULL: u8 = 4;
    pub const TIMEOUT: u8 = 5;
    pub const SDT_LOCKED: u8 = 6;
    pub const PLASMA_FAULT: u8 = 7;
    pub const INTERNAL: u8 = 255;
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn test_result_size() {
        println!("AtlasResult size: {} bytes", size_of::<AtlasResult>());
        println!("ResultKind size: {} bytes", size_of::<ResultKind>());

        // Should be under 512 bytes
        assert!(size_of::<AtlasResult>() <= 512);
    }

    #[test]
    fn test_result_creation() {
        let result = AtlasResult::ok(ResultKind::MatroidRank { rank: 5 }, 123)
            .with_tick(42)
            .with_latency(1000);

        assert!(result.success);
        assert_eq!(result.request_id, 123);
        assert_eq!(result.tick_id, 42);
        assert_eq!(result.latency_ns, 1000);
    }

    #[test]
    fn test_error_result() {
        let result = AtlasResult::err(error::PATH_NOT_FOUND, 456);

        assert!(!result.success);
        assert_eq!(result.error_code, error::PATH_NOT_FOUND);
        assert_eq!(result.request_id, 456);
    }
}
