//! SX9 NATS Subject Definitions
//!
//! All subjects follow the pattern: `sx9.{domain}.{action}.{qualifier}`
//!
//! # Naming Convention
//!
//! - `sx9.tick.*` - Cognitive tick (ATLAS heartbeat)
//! - `sx9.atlas.*` - ATLAS daemon operations
//! - `sx9.bus.*` - AtlasBus IPC
//! - `sx9.kali.*` - Kali tool execution
//! - `sx9.hash.*` - Hashing engine
//! - `sx9.mux.*` - Neural Mux routing
//! - `sx9.plasma.*` - Plasma ECS
//! - `sx9.crate.*` - Smart crate lifecycle
//! - `sx9.cdn.*` - CDN operations
//! - `sx9.iac.*` - Infrastructure as Code
//! - `sx9.health.*` - System health
//! - `sx9.gateway.*` - Gateway requests
//! - `sx9.telemetry.*` - Audit and tracing

/// Subject prefix for all SX9 messages
pub const PREFIX: &str = "sx9";

// ═══════════════════════════════════════════════════════════════════════════
// TICK - Cognitive heartbeat (Core NATS - real-time)
// ═══════════════════════════════════════════════════════════════════════════

pub mod tick {
    /// Full cognitive state broadcast
    pub const COGNITIVE: &str = "sx9.tick.cognitive";
    /// Tick synchronization
    pub const SYNC: &str = "sx9.tick.sync";
    /// Tick ID announcement
    pub const ID: &str = "sx9.tick.id";
}

// ═══════════════════════════════════════════════════════════════════════════
// ATLAS - Cognitive daemon (Core NATS - real-time)
// ═══════════════════════════════════════════════════════════════════════════

pub mod atlas {
    /// OODA loop phases
    pub const OODA_OBSERVE: &str = "sx9.atlas.ooda.observe";
    pub const OODA_ORIENT: &str = "sx9.atlas.ooda.orient";
    pub const OODA_DECIDE: &str = "sx9.atlas.ooda.decide";
    pub const OODA_ACT: &str = "sx9.atlas.ooda.act";
    
    /// HD4 phases
    pub const HD4_HUNT: &str = "sx9.atlas.hd4.hunt";
    pub const HD4_DETECT: &str = "sx9.atlas.hd4.detect";
    pub const HD4_DISRUPT: &str = "sx9.atlas.hd4.disrupt";
    pub const HD4_DISABLE: &str = "sx9.atlas.hd4.disable";
    pub const HD4_DOMINATE: &str = "sx9.atlas.hd4.dominate";
    
    /// Convergence events
    pub const CONVERGENCE: &str = "sx9.atlas.convergence.>";
    
    /// Commands to ATLAS
    pub const COMMAND: &str = "sx9.atlas.command.>";
}

// ═══════════════════════════════════════════════════════════════════════════
// BUS - AtlasBus IPC (Core NATS - real-time, Zone B)
// ═══════════════════════════════════════════════════════════════════════════

pub mod bus {
    /// Critical priority lane (SDT triggers, emergencies)
    pub const CRITICAL: &str = "sx9.bus.critical.>";
    /// Urgent priority lane
    pub const URGENT: &str = "sx9.bus.urgent.>";
    /// Normal priority lane
    pub const NORMAL: &str = "sx9.bus.normal.>";
    /// Result lane
    pub const RESULT: &str = "sx9.bus.result.>";
}

// ═══════════════════════════════════════════════════════════════════════════
// KALI - Tool execution (JetStream - durable)
// ═══════════════════════════════════════════════════════════════════════════

pub mod kali {
    /// Tool execution requests
    /// Format: sx9.kali.exec.{tool_rune}
    pub const EXEC: &str = "sx9.kali.exec.>";
    
    /// Execution results
    /// Format: sx9.kali.result.{correlation_id}
    pub const RESULT: &str = "sx9.kali.result.>";
    
    /// Tool chain orchestration
    /// Format: sx9.kali.chain.{chain_id}
    pub const CHAIN: &str = "sx9.kali.chain.>";
    
    /// Execution telemetry
    pub const TELEMETRY: &str = "sx9.kali.telemetry.>";
    
    /// Build execution subject for specific tool
    pub fn exec_tool(tool_rune: &str) -> String {
        format!("sx9.kali.exec.{}", tool_rune)
    }
    
    /// Build result subject for correlation ID
    pub fn result_for(correlation_id: &str) -> String {
        format!("sx9.kali.result.{}", correlation_id)
    }
    
    /// Build chain subject for chain ID
    pub fn chain_for(chain_id: &str) -> String {
        format!("sx9.kali.chain.{}", chain_id)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HASH - Hashing engine (JetStream - durable)
// ═══════════════════════════════════════════════════════════════════════════

pub mod hash {
    /// Hash computation requests
    pub const COMPUTE: &str = "sx9.hash.compute.>";
    pub const COMPUTE_TRIVARIATE: &str = "sx9.hash.compute.trivariate";
    pub const COMPUTE_SCH: &str = "sx9.hash.compute.sch";
    pub const COMPUTE_CUID: &str = "sx9.hash.compute.cuid";
    
    /// Hash results
    /// Format: sx9.hash.result.{correlation_id}
    pub const RESULT: &str = "sx9.hash.result.>";
    
    /// Hash verification
    pub const VERIFY: &str = "sx9.hash.verify.>";
    
    /// Build result subject for correlation ID
    pub fn result_for(correlation_id: &str) -> String {
        format!("sx9.hash.result.{}", correlation_id)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MUX - Neural Mux routing (Core NATS - sub-microsecond)
// ═══════════════════════════════════════════════════════════════════════════

pub mod mux {
    /// Routing decisions
    pub const ROUTE: &str = "sx9.mux.route.>";
    /// Affinity updates
    pub const AFFINITY: &str = "sx9.mux.affinity.>";
    /// Routing statistics
    pub const STATS: &str = "sx9.mux.stats";
}

// ═══════════════════════════════════════════════════════════════════════════
// PLASMA - ECS operations (Core NATS - real-time)
// ═══════════════════════════════════════════════════════════════════════════

pub mod plasma {
    /// Entity lifecycle
    pub const ENTITY_CREATE: &str = "sx9.plasma.entity.create";
    pub const ENTITY_DESTROY: &str = "sx9.plasma.entity.destroy";
    pub const ENTITY_UPDATE: &str = "sx9.plasma.entity.update";
    
    /// Component updates
    pub const COMPONENT: &str = "sx9.plasma.component.>";
    
    /// Field state
    pub const FIELD: &str = "sx9.plasma.field.>";
}

// ═══════════════════════════════════════════════════════════════════════════
// CRATE - Smart crate lifecycle (JetStream - durable)
// ═══════════════════════════════════════════════════════════════════════════

pub mod crate_lifecycle {
    /// Crate spawning requests
    pub const SPAWN: &str = "sx9.crate.spawn.>";
    /// Crate health updates
    pub const HEALTH: &str = "sx9.crate.health.>";
    /// Crate retirement
    pub const RETIRE: &str = "sx9.crate.retire.>";
    /// Crate status queries
    pub const STATUS: &str = "sx9.crate.status.>";
}

// ═══════════════════════════════════════════════════════════════════════════
// CDN - Content distribution (JetStream - durable)
// ═══════════════════════════════════════════════════════════════════════════

pub mod cdn {
    /// Store content requests
    pub const STORE: &str = "sx9.cdn.store.>";
    /// Retrieve content requests
    pub const RETRIEVE: &str = "sx9.cdn.retrieve.>";
    /// Replication events
    pub const REPLICATE: &str = "sx9.cdn.replicate.>";
    /// CDN node health
    pub const HEALTH: &str = "sx9.cdn.health.>";
    
    /// Build store subject for CDN type
    pub fn store_for(cdn_type: &str) -> String {
        format!("sx9.cdn.store.{}", cdn_type)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// IAC - Infrastructure as Code (JetStream - durable)
// ═══════════════════════════════════════════════════════════════════════════

pub mod iac {
    /// Manifold trigger requests (Unicode triggers)
    pub const TRIGGER: &str = "sx9.iac.trigger.>";
    /// Spawn events
    pub const SPAWN: &str = "sx9.iac.spawn.>";
    /// Teardown events
    pub const TEARDOWN: &str = "sx9.iac.teardown.>";
    /// IAC status
    pub const STATUS: &str = "sx9.iac.status.>";
}

// ═══════════════════════════════════════════════════════════════════════════
// HEALTH - System health (Core NATS - real-time)
// ═══════════════════════════════════════════════════════════════════════════

pub mod health {
    /// Service heartbeats
    /// Format: sx9.health.heartbeat.{service_name}
    pub const HEARTBEAT: &str = "sx9.health.heartbeat.>";
    /// Metrics export
    pub const METRICS: &str = "sx9.health.metrics.>";
    /// Alerts
    pub const ALERT: &str = "sx9.health.alert.>";
    
    /// Build heartbeat subject for service
    pub fn heartbeat_for(service: &str) -> String {
        format!("sx9.health.heartbeat.{}", service)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// GATEWAY - External API (JetStream - durable)
// ═══════════════════════════════════════════════════════════════════════════

pub mod gateway {
    /// Inbound requests
    pub const REQUEST: &str = "sx9.gateway.request.>";
    /// Outbound responses
    pub const RESPONSE: &str = "sx9.gateway.response.>";
    /// Session management
    pub const SESSION: &str = "sx9.gateway.session.>";
}

// ═══════════════════════════════════════════════════════════════════════════
// TELEMETRY - Audit and tracing (JetStream - durable)
// ═══════════════════════════════════════════════════════════════════════════

pub mod telemetry {
    /// Distributed traces
    pub const TRACE: &str = "sx9.telemetry.trace.>";
    /// Trace spans
    pub const SPAN: &str = "sx9.telemetry.span.>";
    /// Audit events
    pub const EVENT: &str = "sx9.telemetry.event.>";
    /// Promotion lineage (Class E → execution)
    pub const PROMOTION: &str = "sx9.telemetry.promotion.>";
}

// ═══════════════════════════════════════════════════════════════════════════
// Utility functions
// ═══════════════════════════════════════════════════════════════════════════

/// Check if a subject uses JetStream (durable) or Core NATS
pub fn is_jetstream_subject(subject: &str) -> bool {
    subject.starts_with("sx9.kali.")
        || subject.starts_with("sx9.hash.")
        || subject.starts_with("sx9.crate.")
        || subject.starts_with("sx9.cdn.")
        || subject.starts_with("sx9.iac.")
        || subject.starts_with("sx9.gateway.")
        || subject.starts_with("sx9.telemetry.")
}

/// Check if subject is real-time (Core NATS)
pub fn is_realtime_subject(subject: &str) -> bool {
    subject.starts_with("sx9.tick.")
        || subject.starts_with("sx9.atlas.")
        || subject.starts_with("sx9.bus.")
        || subject.starts_with("sx9.mux.")
        || subject.starts_with("sx9.plasma.")
        || subject.starts_with("sx9.health.")
}
