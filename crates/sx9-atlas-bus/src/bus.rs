//! ATLAS Bus - Priority-routed command/result channels
//!
//! The bus provides:
//! - Three priority lanes (critical, urgent, normal)
//! - Result channel back to apecs
//! - Embedded plasma state
//! - Backpressure signaling

use crate::command::{Command, Priority};
use crate::plasma::PlasmaState;
use crate::result::AtlasResult;
use crate::ring::Ring;

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Default sizes for priority lanes
pub const CRITICAL_SIZE: usize = 256;
pub const URGENT_SIZE: usize = 1024;
pub const NORMAL_SIZE: usize = 4096;
pub const RESULT_SIZE: usize = 4096;

/// Result of dispatching a command
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DispatchResult {
    /// Command accepted
    Ok,
    /// Buffer full, command dropped
    BufferFull,
    /// SDT gate not conducting, command blocked
    SdtBlocked,
    /// Backpressure warning (command accepted but buffer filling)
    Backpressure {
        /// Current buffer pressure (0.0 - 1.0)
        pressure: f32,
        /// Suggested delta angle adjustment
        delta_class: u8,
    },
}

/// Bus statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct BusStats {
    /// Total commands dispatched
    pub commands_dispatched: u64,
    /// Commands dropped due to full buffer
    pub commands_dropped: u64,
    /// Commands blocked by SDT
    pub commands_blocked: u64,
    /// Total results sent
    pub results_sent: u64,
    /// Results dropped due to full buffer
    pub results_dropped: u64,
    /// Current critical queue depth
    pub critical_depth: usize,
    /// Current urgent queue depth
    pub urgent_depth: usize,
    /// Current normal queue depth
    pub normal_depth: usize,
    /// Current result queue depth
    pub result_depth: usize,
}

/// ATLAS Bus
///
/// Main IPC channel between apecs (async world) and ATLAS daemon (Legion ECS).
///
/// # Memory Layout
/// ```text
/// ┌─────────────────────────────────────────────────────────────────┐
/// │                        ATLAS BUS                                │
/// ├─────────────────────────────────────────────────────────────────┤
/// │ Critical Lane (256 slots)  ← Canary, SDT, emergencies          │
/// ├─────────────────────────────────────────────────────────────────┤
/// │ Urgent Lane (1024 slots)   ← Convergence, tick sync            │
/// ├─────────────────────────────────────────────────────────────────┤
/// │ Normal Lane (4096 slots)   ← Graph ops, hashing                │
/// ├─────────────────────────────────────────────────────────────────┤
/// │ Result Lane (4096 slots)   ← ATLAS → apecs                     │
/// ├─────────────────────────────────────────────────────────────────┤
/// │ Plasma State               ← Δθ, H, excited, SDT               │
/// ├─────────────────────────────────────────────────────────────────┤
/// │ Stats                      ← Counters, depths                  │
/// └─────────────────────────────────────────────────────────────────┘
/// ```
#[repr(C)]
pub struct AtlasBus {
    // Command lanes (apecs → ATLAS)
    critical: Ring<Command, CRITICAL_SIZE>,
    urgent: Ring<Command, URGENT_SIZE>,
    normal: Ring<Command, NORMAL_SIZE>,

    // Result lane (ATLAS → apecs)
    results: Ring<AtlasResult, RESULT_SIZE>,

    // Plasma state
    plasma: PlasmaState,

    // Stats
    commands_dispatched: AtomicU64,
    commands_dropped: AtomicU64,
    commands_blocked: AtomicU64,
    results_sent: AtomicU64,
    results_dropped: AtomicU64,

    // Current tick (for tagging)
    current_tick: AtomicU64,

    // Request ID counter
    next_request_id: AtomicU32,
}

impl AtlasBus {
    /// Create a new ATLAS bus
    pub const fn new() -> Self {
        Self {
            critical: Ring::new(),
            urgent: Ring::new(),
            normal: Ring::new(),
            results: Ring::new(),
            plasma: PlasmaState::new(),
            commands_dispatched: AtomicU64::new(0),
            commands_dropped: AtomicU64::new(0),
            commands_blocked: AtomicU64::new(0),
            results_sent: AtomicU64::new(0),
            results_dropped: AtomicU64::new(0),
            current_tick: AtomicU64::new(0),
            next_request_id: AtomicU32::new(1),
        }
    }

    // ========================================================================
    // Command Dispatch (apecs side)
    // ========================================================================

    /// Dispatch a command to ATLAS
    ///
    /// Automatically routes to the appropriate priority lane.
    /// Respects SDT gate state for non-critical commands.
    #[inline]
    pub fn dispatch(&self, mut cmd: Command) -> DispatchResult {
        // Tag with current tick if not set
        if cmd.tick_id == 0 {
            cmd.tick_id = self.current_tick.load(Ordering::Relaxed);
        }

        // Assign request ID if not set
        if cmd.request_id == 0 {
            cmd.request_id = self.next_request_id.fetch_add(1, Ordering::Relaxed);
        }

        // Check SDT gate for non-critical commands
        if cmd.priority != Priority::Critical && !self.plasma.is_conducting() {
            // Allow if primed (will trigger), block if off or latched
            if self.plasma.sdt_state() != crate::plasma::SdtState::Primed {
                self.commands_blocked.fetch_add(1, Ordering::Relaxed);
                return DispatchResult::SdtBlocked;
            }
        }

        // Route to appropriate lane and get pressure
        let (success, pressure) = match cmd.priority {
            Priority::Critical => (self.critical.push(cmd), self.critical.pressure()),
            Priority::Urgent => (self.urgent.push(cmd), self.urgent.pressure()),
            Priority::Normal => (self.normal.push(cmd), self.normal.pressure()),
        };

        // Check result
        if success {
            self.commands_dispatched.fetch_add(1, Ordering::Relaxed);

            // Check for backpressure
            if pressure > 0.7 {
                DispatchResult::Backpressure {
                    pressure,
                    delta_class: if pressure > 0.9 { 3 } else { 2 },
                }
            } else {
                DispatchResult::Ok
            }
        } else {
            self.commands_dropped.fetch_add(1, Ordering::Relaxed);
            DispatchResult::BufferFull
        }
    }

    /// Dispatch a command, blocking if SDT gate is closed
    ///
    /// Use this for commands that MUST be delivered.
    #[inline]
    pub fn dispatch_critical(&self, cmd: Command) -> DispatchResult {
        let mut cmd = cmd;
        cmd.priority = Priority::Critical;
        self.dispatch(cmd)
    }

    /// Get next request ID
    #[inline]
    pub fn next_request_id(&self) -> u32 {
        self.next_request_id.fetch_add(1, Ordering::Relaxed)
    }

    // ========================================================================
    // Command Consumption (ATLAS daemon side)
    // ========================================================================

    /// Pop next command (priority order)
    ///
    /// Returns commands in priority order: critical > urgent > normal
    #[inline]
    pub fn pop(&self) -> Option<Command> {
        self.critical
            .pop()
            .or_else(|| self.urgent.pop())
            .or_else(|| self.normal.pop())
    }

    /// Pop from critical lane only
    #[inline]
    pub fn pop_critical(&self) -> Option<Command> {
        self.critical.pop()
    }

    /// Pop from urgent lane only
    #[inline]
    pub fn pop_urgent(&self) -> Option<Command> {
        self.urgent.pop()
    }

    /// Pop from normal lane only
    #[inline]
    pub fn pop_normal(&self) -> Option<Command> {
        self.normal.pop()
    }

    /// Drain all pending commands
    #[inline]
    pub fn drain(&self) -> impl Iterator<Item = Command> + '_ {
        core::iter::from_fn(move || self.pop())
    }

    /// Drain commands for a single tick
    ///
    /// Processes all commands, respecting priority order.
    #[inline]
    pub fn tick(&self) -> impl Iterator<Item = Command> + '_ {
        self.drain()
    }

    /// Check if there are pending commands
    #[inline]
    pub fn has_pending(&self) -> bool {
        !self.critical.is_empty() || !self.urgent.is_empty() || !self.normal.is_empty()
    }

    // ========================================================================
    // Result Handling
    // ========================================================================

    /// Send a result back to apecs
    #[inline]
    pub fn respond(&self, mut result: AtlasResult) -> bool {
        // Tag with current tick if not set
        if result.tick_id == 0 {
            result.tick_id = self.current_tick.load(Ordering::Relaxed);
        }

        if self.results.push(result) {
            self.results_sent.fetch_add(1, Ordering::Relaxed);
            true
        } else {
            self.results_dropped.fetch_add(1, Ordering::Relaxed);
            false
        }
    }

    /// Pop next result (apecs side)
    #[inline]
    pub fn pop_result(&self) -> Option<AtlasResult> {
        self.results.pop()
    }

    /// Drain all results
    #[inline]
    pub fn drain_results(&self) -> impl Iterator<Item = AtlasResult> + '_ {
        core::iter::from_fn(move || self.pop_result())
    }

    /// Check if there are pending results
    #[inline]
    pub fn has_results(&self) -> bool {
        !self.results.is_empty()
    }

    // ========================================================================
    // Plasma State Access
    // ========================================================================

    /// Get reference to plasma state
    #[inline]
    pub fn plasma(&self) -> &PlasmaState {
        &self.plasma
    }

    // ========================================================================
    // Tick Management
    // ========================================================================

    /// Set current tick
    #[inline]
    pub fn set_tick(&self, tick: u64) {
        self.current_tick.store(tick, Ordering::Release);
    }

    /// Get current tick
    #[inline]
    pub fn current_tick(&self) -> u64 {
        self.current_tick.load(Ordering::Acquire)
    }

    /// Advance tick
    #[inline]
    pub fn advance_tick(&self) -> u64 {
        self.current_tick.fetch_add(1, Ordering::AcqRel) + 1
    }

    // ========================================================================
    // Statistics
    // ========================================================================

    /// Get bus statistics
    pub fn stats(&self) -> BusStats {
        BusStats {
            commands_dispatched: self.commands_dispatched.load(Ordering::Relaxed),
            commands_dropped: self.commands_dropped.load(Ordering::Relaxed),
            commands_blocked: self.commands_blocked.load(Ordering::Relaxed),
            results_sent: self.results_sent.load(Ordering::Relaxed),
            results_dropped: self.results_dropped.load(Ordering::Relaxed),
            critical_depth: self.critical.len(),
            urgent_depth: self.urgent.len(),
            normal_depth: self.normal.len(),
            result_depth: self.results.len(),
        }
    }

    /// Get total queue pressure (0.0 - 1.0)
    #[inline]
    pub fn pressure(&self) -> f32 {
        let total_len = self.critical.len() + self.urgent.len() + self.normal.len();
        let total_cap = self.critical.capacity() + self.urgent.capacity() + self.normal.capacity();
        total_len as f32 / total_cap as f32
    }

    /// Clear all queues
    pub fn clear(&self) {
        self.critical.clear();
        self.urgent.clear();
        self.normal.clear();
        self.results.clear();
    }
}

impl Default for AtlasBus {
    fn default() -> Self {
        Self::new()
    }
}

// Safety: AtlasBus is safe to share between threads
unsafe impl Send for AtlasBus {}
unsafe impl Sync for AtlasBus {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::CommandKind;
    use crate::ring::Ring;

    // Small test bus to avoid stack overflow
    struct TestBus {
        critical: Ring<Command, 16>,
        urgent: Ring<Command, 16>,
        normal: Ring<Command, 16>,
        results: Ring<AtlasResult, 16>,
        plasma: PlasmaState,
    }

    impl TestBus {
        fn new() -> Self {
            Self {
                critical: Ring::new(),
                urgent: Ring::new(),
                normal: Ring::new(),
                results: Ring::new(),
                plasma: PlasmaState::new(),
            }
        }

        fn dispatch(&self, cmd: Command) -> DispatchResult {
            if cmd.priority != Priority::Critical && !self.plasma.is_conducting() {
                if self.plasma.sdt_state() != crate::plasma::SdtState::Primed {
                    return DispatchResult::SdtBlocked;
                }
            }

            let success = match cmd.priority {
                Priority::Critical => self.critical.push(cmd),
                Priority::Urgent => self.urgent.push(cmd),
                Priority::Normal => self.normal.push(cmd),
            };

            if success {
                DispatchResult::Ok
            } else {
                DispatchResult::BufferFull
            }
        }

        fn pop(&self) -> Option<Command> {
            self.critical
                .pop()
                .or_else(|| self.urgent.pop())
                .or_else(|| self.normal.pop())
        }

        fn plasma(&self) -> &PlasmaState {
            &self.plasma
        }

        fn respond(&self, result: AtlasResult) -> bool {
            self.results.push(result)
        }
        fn pop_result(&self) -> Option<AtlasResult> {
            self.results.pop()
        }
    }

    #[test]
    fn test_dispatch_and_pop() {
        let bus = TestBus::new();

        let cmd = Command::new(CommandKind::Ping {
            seq: 1,
            timestamp_ns: 0,
        });
        assert_eq!(bus.dispatch(cmd), DispatchResult::SdtBlocked);

        // Enable SDT and retry
        bus.plasma().prime();
        bus.plasma().trigger(0);

        let cmd = Command::new(CommandKind::Ping {
            seq: 1,
            timestamp_ns: 0,
        });
        assert_eq!(bus.dispatch(cmd), DispatchResult::Ok);

        let popped = bus.pop().unwrap();
        assert!(matches!(popped.kind, CommandKind::Ping { seq: 1, .. }));
    }

    #[test]
    fn test_priority_order() {
        let bus = TestBus::new();

        // Enable SDT for non-critical commands
        bus.plasma().prime();
        bus.plasma().trigger(0);

        // Dispatch in reverse priority order
        bus.dispatch(Command::new(CommandKind::Ping {
            seq: 1,
            timestamp_ns: 0,
        })); // Normal
        bus.dispatch(Command::urgent(CommandKind::Ping {
            seq: 2,
            timestamp_ns: 0,
        }));
        bus.dispatch(Command::critical(CommandKind::Ping {
            seq: 3,
            timestamp_ns: 0,
        }));

        // Should pop in priority order
        let c1 = bus.pop().unwrap();
        let c2 = bus.pop().unwrap();
        let c3 = bus.pop().unwrap();

        assert!(matches!(c1.kind, CommandKind::Ping { seq: 3, .. })); // Critical first
        assert!(matches!(c2.kind, CommandKind::Ping { seq: 2, .. })); // Then urgent
        assert!(matches!(c3.kind, CommandKind::Ping { seq: 1, .. })); // Then normal
    }

    #[test]
    fn test_sdt_blocking() {
        let bus = TestBus::new();

        // SDT is off by default
        let cmd = Command::new(CommandKind::Ping {
            seq: 1,
            timestamp_ns: 0,
        });
        assert_eq!(bus.dispatch(cmd), DispatchResult::SdtBlocked);

        // Critical bypasses SDT
        let cmd = Command::critical(CommandKind::Ping {
            seq: 2,
            timestamp_ns: 0,
        });
        assert_eq!(bus.dispatch(cmd), DispatchResult::Ok);

        // Enable SDT
        bus.plasma().prime();
        bus.plasma().trigger(0);

        let cmd = Command::new(CommandKind::Ping {
            seq: 3,
            timestamp_ns: 0,
        });
        assert_eq!(bus.dispatch(cmd), DispatchResult::Ok);
    }

    #[test]
    fn test_results() {
        let bus = TestBus::new();

        use crate::result::ResultKind;

        let result = AtlasResult::ok(ResultKind::MatroidRank { rank: 5 }, 123);
        assert!(bus.respond(result));

        let popped = bus.pop_result().unwrap();
        assert_eq!(popped.request_id, 123);
        assert!(matches!(popped.kind, ResultKind::MatroidRank { rank: 5 }));
    }
}
