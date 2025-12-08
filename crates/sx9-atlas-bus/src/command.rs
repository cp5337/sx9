//! ATLAS command definitions
//!
//! Commands are sent from apecs (async world) to ATLAS daemon (Legion ECS).

/// Maximum nodes in a batch operation
#[cfg(not(test))]
pub const MAX_BATCH_NODES: usize = 64;

#[cfg(test)]
pub const MAX_BATCH_NODES: usize = 8;

/// Priority levels for command routing
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    /// Normal operations (graph queries, etc)
    Normal = 0,
    /// Urgent (convergence signals, tick sync)
    Urgent = 1,
    /// Critical (canary trips, SDT triggers, emergencies)
    Critical = 2,
}

/// Command sent to ATLAS daemon
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Command {
    /// Command type and payload
    pub kind: CommandKind,
    
    /// Trivariate SCH hash for lineage tracking
    pub sch_hash: u64,
    
    /// Tick ID when command was created
    pub tick_id: u64,
    
    /// Command priority
    pub priority: Priority,
    
    /// Request ID for response correlation
    pub request_id: u32,
    
    /// Padding for alignment
    _pad: [u8; 3],
}

/// Command variants
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CommandKind {
    // ========================================================================
    // Graph Operations
    // ========================================================================
    
    /// Find shortest path between two nodes (Dijkstra)
    Dijkstra {
        src: u32,
        dst: u32,
        max_hops: u8,
    },
    
    /// Breadth-first search from source
    Bfs {
        src: u32,
        max_depth: u8,
    },
    
    /// Find all paths between nodes
    AllPaths {
        src: u32,
        dst: u32,
        max_paths: u8,
    },
    
    // ========================================================================
    // GLAF Operations
    // ========================================================================
    
    /// Calculate matroid rank for node set
    MatroidRank {
        nodes: [u32; MAX_BATCH_NODES],
        count: u8,
    },
    
    /// Check matroid independence
    MatroidIndependent {
        nodes: [u32; MAX_BATCH_NODES],
        count: u8,
    },
    
    /// Find maximum independent set
    MaxIndependentSet {
        nodes: [u32; MAX_BATCH_NODES],
        count: u8,
    },
    
    // ========================================================================
    // Convergence Operations
    // ========================================================================
    
    /// Check if entity has converged
    ConvergenceCheck {
        entity_id: u32,
        epsilon: f32,
        window: u16,
    },
    
    /// Calculate convergence rate
    ConvergenceRate {
        entity_id: u32,
        window: u16,
    },
    
    // ========================================================================
    // Hash Operations
    // ========================================================================
    
    /// Batch hash generation (pointer to external data)
    BatchHash {
        /// Pointer to data buffer (must be valid for duration of command)
        data_ptr: u64,
        /// Number of items to hash
        count: u32,
        /// Size of each item in bytes
        item_size: u16,
    },
    
    /// Generate trivariate hash
    TrivariateHash {
        /// Domain mask
        domain: u8,
        /// Execution mask
        execution: u8,
        /// Delta angle class
        delta_class: u8,
    },
    
    // ========================================================================
    // Tick Operations
    // ========================================================================
    
    /// Sync tick across distributed ATLAS instances
    TickSync {
        tick_id: u64,
        timestamp_ns: u64,
    },
    
    /// Query current tick state
    TickQuery,
    
    // ========================================================================
    // SDT Operations
    // ========================================================================
    
    /// Trigger SDT gate
    SdtTrigger {
        gate_id: u32,
        reason: u16,
    },
    
    /// Reset SDT gate
    SdtReset {
        gate_id: u32,
    },
    
    /// Query SDT state
    SdtQuery {
        gate_id: u32,
    },
    
    // ========================================================================
    // Plasma Operations
    // ========================================================================
    
    /// Update plasma field
    PlasmaUpdate {
        field_id: u32,
        delta_angle: u16,
        entropy: u32,
        excited: bool,
    },
    
    /// Query plasma state
    PlasmaQuery {
        field_id: u32,
    },
    
    // ========================================================================
    // Control Operations
    // ========================================================================
    
    /// Ping (for latency measurement)
    Ping {
        seq: u32,
        timestamp_ns: u64,
    },
    
    /// Shutdown ATLAS daemon
    Shutdown,
    
    /// Request stats
    Stats,
}

impl Command {
    /// Create a new command with default metadata
    #[inline]
    pub fn new(kind: CommandKind) -> Self {
        Self {
            kind,
            sch_hash: 0,
            tick_id: 0,
            priority: Priority::Normal,
            request_id: 0,
            _pad: [0; 3],
        }
    }
    
    /// Create command with priority
    #[inline]
    pub fn with_priority(kind: CommandKind, priority: Priority) -> Self {
        Self {
            kind,
            sch_hash: 0,
            tick_id: 0,
            priority,
            request_id: 0,
            _pad: [0; 3],
        }
    }
    
    /// Create critical command
    #[inline]
    pub fn critical(kind: CommandKind) -> Self {
        Self::with_priority(kind, Priority::Critical)
    }
    
    /// Create urgent command
    #[inline]
    pub fn urgent(kind: CommandKind) -> Self {
        Self::with_priority(kind, Priority::Urgent)
    }
    
    /// Set SCH hash for lineage tracking
    #[inline]
    pub fn with_hash(mut self, sch_hash: u64) -> Self {
        self.sch_hash = sch_hash;
        self
    }
    
    /// Set tick ID
    #[inline]
    pub fn with_tick(mut self, tick_id: u64) -> Self {
        self.tick_id = tick_id;
        self
    }
    
    /// Set request ID for response correlation
    #[inline]
    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.request_id = request_id;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;
    
    #[test]
    fn test_command_size() {
        // Ensure command fits in reasonable size
        println!("Command size: {} bytes", size_of::<Command>());
        println!("CommandKind size: {} bytes", size_of::<CommandKind>());
        
        // Should be under 512 bytes for cache efficiency
        assert!(size_of::<Command>() <= 512);
    }
    
    #[test]
    fn test_command_creation() {
        let cmd = Command::new(CommandKind::Dijkstra { src: 1, dst: 2, max_hops: 5 })
            .with_hash(0xDEADBEEF)
            .with_tick(42)
            .with_request_id(123);
        
        assert_eq!(cmd.sch_hash, 0xDEADBEEF);
        assert_eq!(cmd.tick_id, 42);
        assert_eq!(cmd.request_id, 123);
        assert_eq!(cmd.priority, Priority::Normal);
    }
    
    #[test]
    fn test_priority() {
        let normal = Command::new(CommandKind::Ping { seq: 0, timestamp_ns: 0 });
        let urgent = Command::urgent(CommandKind::Ping { seq: 0, timestamp_ns: 0 });
        let critical = Command::critical(CommandKind::Ping { seq: 0, timestamp_ns: 0 });
        
        assert_eq!(normal.priority, Priority::Normal);
        assert_eq!(urgent.priority, Priority::Urgent);
        assert_eq!(critical.priority, Priority::Critical);
    }
}

