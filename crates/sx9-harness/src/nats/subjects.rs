//! NATS Subject Hierarchy for SX9 Systems
//!
//! Based on RFC-9400 (Gateway & NATS Architecture) with Redux action mirroring.
//!
//! ## Namespace Structure (RFC-9400 + Extensions)
//! - `sx9.tick.*`          - Cognitive tick (1ms heartbeat)
//! - `sx9.atlas.*`         - ATLAS Daemon (OODA loop, state, commands)
//! - `sx9.bus.*`           - AtlasBus IPC (critical/standard)
//! - `sx9.plasma.*`        - Plasma ECS (entity, component, field)
//! - `sx9.hash.*`          - Hashing Engine (compute, result, verify)
//! - `sx9.mux.*`           - Neural Mux (routing, affinity)
//! - `sx9.kali.*`          - Kali Execution (tools, chains)
//! - `sx9.crate.*`         - Smart Crates (spawn, health, retire)
//! - `sx9.cdn.*`           - CDN Operations (store, retrieve)
//! - `sx9.iac.*`           - Infrastructure as Code (manifold triggers)
//! - `sx9.health.*`        - System Health (heartbeat, metrics, alerts)
//! - `sx9.gateway.*`       - Gateway (request/response, sessions)
//! - `sx9.telemetry.*`     - Telemetry (traces, spans, events)
//! - `sx9.forge.*`         - FORGE Agent System (prompt engineering, QA)
//! - `sx9.intelligence.*`  - Intelligence (Leptose, ChromaDB, EEI)
//! - `sx9.linear.*`        - Linear Integration
//! - `sx9.slack.*`         - Slack Notifications
//!
//! ## Transport Decision (RFC-9400 Section 4)
//! - **Core NATS** (~50µs): tick, atlas.ooda, bus, plasma, mux, health.heartbeat
//! - **JetStream** (~200µs): kali, hash, crate, cdn, iac, gateway, telemetry

/// Cognitive tick subjects (Core NATS - real-time)
pub mod tick {
    pub const MS1: &str = "sx9.tick.1ms";
    pub const SYNC: &str = "sx9.tick.sync";
}

/// ATLAS Daemon subjects (Core NATS for OODA, mixed for commands)
pub mod atlas {
    pub const PREFIX: &str = "sx9.atlas";

    /// OODA loop phases (Core NATS - real-time cognitive state)
    pub mod ooda {
        pub const OBSERVE: &str = "sx9.atlas.ooda.observe";
        pub const ORIENT: &str = "sx9.atlas.ooda.orient";
        pub const DECIDE: &str = "sx9.atlas.ooda.decide";
        pub const ACT: &str = "sx9.atlas.ooda.act";
    }

    /// State updates
    pub mod state {
        pub const UPDATE: &str = "sx9.atlas.state.update";
        pub const SNAPSHOT: &str = "sx9.atlas.state.snapshot";
    }

    /// Command subjects (prioritized)
    pub const CMD_CRITICAL: &str = "sx9.atlas.cmd.critical";
    pub const CMD_URGENT: &str = "sx9.atlas.cmd.urgent";
    pub const CMD_NORMAL: &str = "sx9.atlas.cmd.normal";
    pub const RESULT: &str = "sx9.atlas.result";
    pub const PLASMA: &str = "sx9.atlas.plasma";
    pub const TICK: &str = "sx9.atlas.tick";

    /// SDT (Software Defined Thyristor) subjects
    pub fn sdt_trigger(gate_id: u32) -> String {
        format!("sx9.sdt.{}.trigger", gate_id)
    }

    pub fn sdt_state(gate_id: u32) -> String {
        format!("sx9.sdt.{}.state", gate_id)
    }
}

/// AtlasBus IPC subjects (Core NATS - HFT path)
pub mod bus {
    pub const CRITICAL: &str = "sx9.bus.critical";
    pub const STANDARD: &str = "sx9.bus.standard";

    pub fn critical_for(topic: &str) -> String {
        format!("sx9.bus.critical.{}", topic)
    }

    pub fn standard_for(topic: &str) -> String {
        format!("sx9.bus.standard.{}", topic)
    }
}

/// Plasma ECS subjects (Core NATS - high frequency)
pub mod plasma {
    pub const PREFIX: &str = "sx9.plasma";

    pub mod entity {
        pub const SPAWN: &str = "sx9.plasma.entity.spawn";
        pub const DESPAWN: &str = "sx9.plasma.entity.despawn";
        pub const UPDATE: &str = "sx9.plasma.entity.update";
    }

    pub mod component {
        pub const ADD: &str = "sx9.plasma.component.add";
        pub const REMOVE: &str = "sx9.plasma.component.remove";
        pub const UPDATE: &str = "sx9.plasma.component.update";
    }

    pub mod field {
        pub const STATE: &str = "sx9.plasma.field.state";
        pub const SYNC: &str = "sx9.plasma.field.sync";
    }
}

/// Hashing Engine subjects (JetStream - durable)
pub mod hash {
    pub const PREFIX: &str = "sx9.hash";

    pub const COMPUTE: &str = "sx9.hash.compute";
    pub const RESULT: &str = "sx9.hash.result";
    pub const VERIFY: &str = "sx9.hash.verify";

    pub fn compute_for(hash_type: &str) -> String {
        format!("sx9.hash.compute.{}", hash_type)
    }

    pub fn result_for(correlation_id: &str) -> String {
        format!("sx9.hash.result.{}", correlation_id)
    }
}

/// Neural Mux subjects (Core NATS - sub-microsecond routing)
pub mod mux {
    pub const PREFIX: &str = "sx9.mux";

    pub const ROUTE: &str = "sx9.mux.route";
    pub const AFFINITY: &str = "sx9.mux.affinity";
    pub const STATS: &str = "sx9.mux.stats";

    pub fn route_for(destination: &str) -> String {
        format!("sx9.mux.route.{}", destination)
    }
}

/// Kali Execution subjects (JetStream - durable execution queue)
pub mod kali {
    pub const PREFIX: &str = "sx9.kali";

    pub const EXEC: &str = "sx9.kali.exec";
    pub const RESULT: &str = "sx9.kali.result";
    pub const CHAIN: &str = "sx9.kali.chain";
    pub const TELEMETRY: &str = "sx9.kali.telemetry";

    pub fn exec_for(tool: &str) -> String {
        format!("sx9.kali.exec.{}", tool)
    }

    pub fn result_for(correlation_id: &str) -> String {
        format!("sx9.kali.result.{}", correlation_id)
    }

    pub fn chain_for(chain_id: &str) -> String {
        format!("sx9.kali.chain.{}", chain_id)
    }
}

/// Smart Crate subjects (JetStream - spawn/retire are imperative)
pub mod crate_mgmt {
    pub const PREFIX: &str = "sx9.crate";

    pub const SPAWN: &str = "sx9.crate.spawn";
    pub const HEALTH: &str = "sx9.crate.health";
    pub const RETIRE: &str = "sx9.crate.retire";

    pub fn spawn_for(crate_name: &str) -> String {
        format!("sx9.crate.spawn.{}", crate_name)
    }

    pub fn health_for(crate_name: &str) -> String {
        format!("sx9.crate.health.{}", crate_name)
    }
}

/// CDN Operations subjects (JetStream - durable)
pub mod cdn {
    pub const PREFIX: &str = "sx9.cdn";

    pub const STORE: &str = "sx9.cdn.store";
    pub const RETRIEVE: &str = "sx9.cdn.retrieve";
    pub const REPLICATE: &str = "sx9.cdn.replicate";

    pub fn store_for(asset_type: &str) -> String {
        format!("sx9.cdn.store.{}", asset_type)
    }
}

/// Infrastructure as Code subjects (JetStream - must persist)
pub mod iac {
    pub const PREFIX: &str = "sx9.iac";

    pub const TRIGGER: &str = "sx9.iac.trigger";
    pub const SPAWN: &str = "sx9.iac.spawn";
    pub const TEARDOWN: &str = "sx9.iac.teardown";

    pub fn trigger_for(manifold: &str) -> String {
        format!("sx9.iac.trigger.{}", manifold)
    }
}

/// System Health subjects (Core NATS for heartbeat, JetStream for metrics)
pub mod health {
    pub const PREFIX: &str = "sx9.health";

    pub const HEARTBEAT: &str = "sx9.health.heartbeat";
    pub const METRICS: &str = "sx9.health.metrics";
    pub const ALERT: &str = "sx9.health.alert";

    pub fn heartbeat_for(service: &str) -> String {
        format!("sx9.health.heartbeat.{}", service)
    }

    pub fn metrics_for(service: &str) -> String {
        format!("sx9.health.metrics.{}", service)
    }

    pub fn alert_for(severity: &str) -> String {
        format!("sx9.health.alert.{}", severity)
    }
}

/// Smart Crate Heartbeat subjects (RFC-9141 Zero-Trust)
/// Core NATS for local heartbeat (~50µs), distributed pub/sub pattern.
pub mod heartbeat {
    pub const PREFIX: &str = "sx9.heartbeat";

    /// Local heartbeat broadcast (crates publish here)
    pub const LOCAL: &str = "sx9.heartbeat.local";

    /// Global aggregated state (orchestrator publishes here)
    pub const GLOBAL: &str = "sx9.heartbeat.global";

    /// Unauthorized crate alert (CRITICAL - zero-trust violation)
    pub const ALERT_UNAUTHORIZED: &str = "sx9.heartbeat.alert.unauthorized";

    /// Missing heartbeat alert (WARNING - crate may be down)
    pub const ALERT_MISSING: &str = "sx9.heartbeat.alert.missing";

    /// Per-crate heartbeat subject pattern
    /// Crates publish to: sx9.heartbeat.crate.{name}
    /// Orchestrator subscribes to: sx9.heartbeat.crate.*
    pub fn for_crate(crate_name: &str) -> String {
        format!("sx9.heartbeat.crate.{}", crate_name)
    }

    /// Wildcard subscription for orchestrator
    pub const CRATE_WILDCARD: &str = "sx9.heartbeat.crate.*";
}

/// Gateway subjects (JetStream - request durability)
pub mod gateway {
    pub const PREFIX: &str = "sx9.gateway";

    pub const REQUEST: &str = "sx9.gateway.request";
    pub const RESPONSE: &str = "sx9.gateway.response";
    pub const SESSION: &str = "sx9.gateway.session";

    pub fn request_for(endpoint: &str) -> String {
        format!("sx9.gateway.request.{}", endpoint)
    }

    pub fn response_for(correlation_id: &str) -> String {
        format!("sx9.gateway.response.{}", correlation_id)
    }
}

/// Telemetry subjects (JetStream - audit trail)
pub mod telemetry {
    pub const PREFIX: &str = "sx9.telemetry";

    pub const TRACE: &str = "sx9.telemetry.trace";
    pub const SPAN: &str = "sx9.telemetry.span";
    pub const EVENT: &str = "sx9.telemetry.event";

    pub fn trace_for(trace_id: &str) -> String {
        format!("sx9.telemetry.trace.{}", trace_id)
    }
}

/// FORGE Agent System subjects
pub mod forge {
    pub const PREFIX: &str = "sx9.forge";

    /// Agent coordination
    pub mod agent {
        pub const SPAWN: &str = "sx9.forge.agent.spawn";
        pub const HEARTBEAT: &str = "sx9.forge.agent.heartbeat";
        pub const RESULT: &str = "sx9.forge.agent.result";

        pub fn by_persona(persona: &str) -> String {
            format!("sx9.forge.agent.{}", persona.to_lowercase())
        }
    }

    /// Task routing by priority
    pub mod task {
        pub const P0_SUBMIT: &str = "sx9.forge.task.p0.submit";
        pub const P0_COMPLETE: &str = "sx9.forge.task.p0.complete";
        pub const P1_SUBMIT: &str = "sx9.forge.task.p1.submit";
        pub const P1_COMPLETE: &str = "sx9.forge.task.p1.complete";
        pub const P2_SUBMIT: &str = "sx9.forge.task.p2.submit";
        pub const P2_COMPLETE: &str = "sx9.forge.task.p2.complete";

        pub fn by_id(task_id: &str) -> String {
            format!("sx9.forge.task.{}", task_id)
        }
    }

    /// 4-stage Unified Pipeline (RFC-9130)
    pub mod pipeline {
        pub const THALMIC: &str = "sx9.forge.pipeline.thalmic";
        pub const PROMPTSCRIPT: &str = "sx9.forge.pipeline.promptscript";
        pub const BIRTH: &str = "sx9.forge.pipeline.birth";

        pub fn qa_grade(grade: char) -> String {
            format!("sx9.forge.pipeline.qa.{}", grade)
        }

        pub fn qa_crate(crate_name: &str) -> String {
            format!("sx9.forge.qa.{}", crate_name)
        }
    }

    /// Harness modes (RFC-9130)
    pub mod harness {
        pub const AUTONOMOUS: &str = "sx9.forge.harness.autonomous";
        pub const RESEARCH: &str = "sx9.forge.harness.research";
        pub const BUILD: &str = "sx9.forge.harness.build";
        pub const SECURITY: &str = "sx9.forge.harness.security";
        pub const PLANNING: &str = "sx9.forge.harness.planning";

        pub fn by_mode(mode: &str) -> String {
            format!("sx9.forge.harness.{}", mode.to_lowercase())
        }
    }

    /// Governance and drift signals (CLSGS Annex A / RFC-9142)
    pub mod governance {
        /// Drift signal detected
        pub const DRIFT_DETECTED: &str = "sx9.forge.governance.drift.detected";
        /// Drift signal by vector type
        pub const DRIFT_ROLE: &str = "sx9.forge.governance.drift.role";
        pub const DRIFT_CONSTRAINT: &str = "sx9.forge.governance.drift.constraint";
        pub const DRIFT_COUPLING: &str = "sx9.forge.governance.drift.coupling";
        pub const DRIFT_AUTHORITY: &str = "sx9.forge.governance.drift.authority";
        pub const DRIFT_PATTERN: &str = "sx9.forge.governance.drift.pattern";

        /// Governance gates (RFC-9142 Section 7.1)
        pub const GATE_OBSERVE: &str = "sx9.forge.governance.gate.observe";
        pub const GATE_WARN: &str = "sx9.forge.governance.gate.warn";
        pub const GATE_BLOCK: &str = "sx9.forge.governance.gate.block";
        pub const GATE_ESCALATE: &str = "sx9.forge.governance.gate.escalate";

        /// Lineage events (CLSGS Annex A.4)
        pub const LINEAGE_REGRESSION: &str = "sx9.forge.governance.lineage.regression";
        pub const LINEAGE_LOSS: &str = "sx9.forge.governance.lineage.annotation_loss";
        pub const LINEAGE_EXPANSION: &str = "sx9.forge.governance.lineage.scope_expansion";

        /// Drift signal for specific component
        pub fn drift_for(component: &str) -> String {
            format!("sx9.forge.governance.drift.{}", component.to_lowercase())
        }

        /// Gate event for specific PR
        pub fn gate_for_pr(pr_id: &str) -> String {
            format!("sx9.forge.governance.gate.pr.{}", pr_id)
        }

        /// Lineage analysis for commit
        pub fn lineage_for_commit(sha: &str) -> String {
            format!("sx9.forge.governance.lineage.commit.{}", sha)
        }
    }

    /// Static QA signals (CLSGS Annex A.3.2)
    pub mod static_qa {
        pub const PASSED: &str = "sx9.forge.qa.static.passed";
        pub const FAILED: &str = "sx9.forge.qa.static.failed";
        pub const RUNNING: &str = "sx9.forge.qa.static.running";

        pub fn for_crate(crate_name: &str) -> String {
            format!("sx9.forge.qa.static.{}", crate_name)
        }
    }

    /// Semantic QA signals (CLSGS Annex A.3.2)
    pub mod semantic_qa {
        pub const PASSED: &str = "sx9.forge.qa.semantic.passed";
        pub const DRIFT_ADVISORY: &str = "sx9.forge.qa.semantic.drift_advisory";
        pub const RUNNING: &str = "sx9.forge.qa.semantic.running";

        pub fn for_crate(crate_name: &str) -> String {
            format!("sx9.forge.qa.semantic.{}", crate_name)
        }
    }
}

/// Ops Main subjects (monitoring, health, alerts)
pub mod ops {
    pub const PREFIX: &str = "sx9.ops";

    /// Health monitoring
    pub mod health {
        pub const CHECK: &str = "sx9.ops.health.check";
        pub const STATUS: &str = "sx9.ops.health.status";
        pub const DEGRADED: &str = "sx9.ops.health.degraded";
        pub const RECOVERED: &str = "sx9.ops.health.recovered";

        pub fn service(name: &str) -> String {
            format!("sx9.ops.health.{}", name)
        }
    }

    /// Service discovery
    pub mod discovery {
        pub const REGISTER: &str = "sx9.ops.discovery.register";
        pub const DEREGISTER: &str = "sx9.ops.discovery.deregister";
        pub const LOOKUP: &str = "sx9.ops.discovery.lookup";
        pub const LIST: &str = "sx9.ops.discovery.list";
    }

    /// Metrics and telemetry
    pub mod metrics {
        pub const PUSH: &str = "sx9.ops.metrics.push";
        pub const QUERY: &str = "sx9.ops.metrics.query";

        pub fn service(name: &str) -> String {
            format!("sx9.ops.metrics.{}", name)
        }
    }

    /// Alerts
    pub mod alerts {
        pub const CRITICAL: &str = "sx9.ops.alerts.critical";
        pub const WARNING: &str = "sx9.ops.alerts.warning";
        pub const INFO: &str = "sx9.ops.alerts.info";
        pub const RESOLVED: &str = "sx9.ops.alerts.resolved";
    }

    /// Port management
    pub mod ports {
        pub const ALLOCATE: &str = "sx9.ops.ports.allocate";
        pub const RELEASE: &str = "sx9.ops.ports.release";
        pub const STATUS: &str = "sx9.ops.ports.status";
    }

    /// Container orchestration
    pub mod containers {
        pub const START: &str = "sx9.ops.containers.start";
        pub const STOP: &str = "sx9.ops.containers.stop";
        pub const RESTART: &str = "sx9.ops.containers.restart";
        pub const LOGS: &str = "sx9.ops.containers.logs";
    }

    /// Database operations
    pub mod db {
        pub const BACKUP: &str = "sx9.ops.db.backup";
        pub const RESTORE: &str = "sx9.ops.db.restore";
        pub const VALIDATE: &str = "sx9.ops.db.validate";
        pub const MIGRATE: &str = "sx9.ops.db.migrate";
    }
}

/// Intelligence subjects (mirrors Redux actions)
/// Redux: `intelligence/leptose/connect` → NATS: `sx9.intelligence.leptose.connect`
pub mod intelligence {
    pub const PREFIX: &str = "sx9.intelligence";

    /// Leptose connection
    pub mod leptose {
        pub const CONNECT: &str = "sx9.intelligence.leptose.connect";
        pub const CONNECTED: &str = "sx9.intelligence.leptose.connected";
        pub const DISCONNECTED: &str = "sx9.intelligence.leptose.disconnected";
        pub const ERROR: &str = "sx9.intelligence.leptose.error";
    }

    /// ChromaDB connection
    pub mod chromadb {
        pub const CONNECT: &str = "sx9.intelligence.chromadb.connect";
        pub const CONNECTED: &str = "sx9.intelligence.chromadb.connected";
        pub const DISCONNECTED: &str = "sx9.intelligence.chromadb.disconnected";
        pub const ERROR: &str = "sx9.intelligence.chromadb.error";
    }

    /// Pattern queries
    pub mod patterns {
        pub const QUERY: &str = "sx9.intelligence.patterns.query";
        pub const LOADING: &str = "sx9.intelligence.patterns.loading";
        pub const SUCCESS: &str = "sx9.intelligence.patterns.success";
        pub const ERROR: &str = "sx9.intelligence.patterns.error";
        pub const APPLY: &str = "sx9.intelligence.patterns.apply";
    }

    /// Tool queries
    pub mod tools {
        pub const QUERY: &str = "sx9.intelligence.tools.query";
        pub const LOADING: &str = "sx9.intelligence.tools.loading";
        pub const SUCCESS: &str = "sx9.intelligence.tools.success";
        pub const ERROR: &str = "sx9.intelligence.tools.error";
        pub const SELECT: &str = "sx9.intelligence.tools.select";
    }

    /// Threat queries
    pub mod threats {
        pub const QUERY: &str = "sx9.intelligence.threats.query";
        pub const LOADING: &str = "sx9.intelligence.threats.loading";
        pub const SUCCESS: &str = "sx9.intelligence.threats.success";
        pub const ERROR: &str = "sx9.intelligence.threats.error";
    }

    /// EEI (Essential Elements of Information) - RFC-9200
    pub mod eei {
        pub const ASK: &str = "sx9.intelligence.eei.ask";
        pub const LOADING: &str = "sx9.intelligence.eei.loading";
        pub const ANSWER: &str = "sx9.intelligence.eei.answer";
        pub const ERROR: &str = "sx9.intelligence.eei.error";
        pub const GAP: &str = "sx9.intelligence.eei.gap";
    }

    /// Collection tasking - RFC-9200
    pub mod collection {
        pub const TASK: &str = "sx9.intelligence.collection.task";
        pub const RESULT: &str = "sx9.intelligence.collection.result";
        pub const STATION_HEARTBEAT: &str = "sx9.intelligence.collection.station.heartbeat";
    }

    /// Status refresh
    pub mod status {
        pub const REFRESH: &str = "sx9.intelligence.status.refresh";
    }
}

/// Linear integration (top-level for cross-system use)
pub mod linear {
    pub const PREFIX: &str = "sx9.linear";

    pub const CREATE: &str = "sx9.linear.issue.create";
    pub const UPDATE: &str = "sx9.linear.issue.update";
    pub const COMMENT: &str = "sx9.linear.issue.comment";
    pub const WEBHOOK: &str = "sx9.linear.webhook";

    pub fn issue(issue_id: &str) -> String {
        format!("sx9.linear.issue.{}", issue_id)
    }
}

/// Slack integration (top-level for cross-system use)
pub mod slack {
    pub const PREFIX: &str = "sx9.slack";

    pub const NOTIFY: &str = "sx9.slack.notify";
    pub const THREAD: &str = "sx9.slack.thread";

    /// @ mention routing
    pub mod mention {
        pub const RECEIVED: &str = "sx9.slack.mention.received";
        pub const ROUTED: &str = "sx9.slack.mention.routed";

        /// Mention for specific agent: sx9.slack.mention.{agent}
        pub fn agent(agent_name: &str) -> String {
            format!("sx9.slack.mention.{}", agent_name.to_lowercase())
        }
    }

    /// Reply to thread
    pub mod reply {
        pub const SEND: &str = "sx9.slack.reply.send";
        pub const SENT: &str = "sx9.slack.reply.sent";
    }

    pub fn channel(channel: &str) -> String {
        format!("sx9.slack.channel.{}", channel)
    }
}

/// Multi-Agent routing (AI providers via @ mentions)
pub mod agent {
    pub const PREFIX: &str = "sx9.agent";

    /// Agent registry
    pub mod registry {
        pub const REGISTER: &str = "sx9.agent.registry.register";
        pub const DEREGISTER: &str = "sx9.agent.registry.deregister";
        pub const LIST: &str = "sx9.agent.registry.list";
        pub const HEALTH: &str = "sx9.agent.registry.health";
    }

    /// Agent request/response pattern
    pub mod request {
        pub const CLAUDE: &str = "sx9.agent.claude.request";
        pub const GPT: &str = "sx9.agent.gpt.request";
        pub const GEMINI: &str = "sx9.agent.gemini.request";
        pub const GROK: &str = "sx9.agent.grok.request";
        pub const CURSOR: &str = "sx9.agent.cursor.request";

        pub fn for_agent(agent_name: &str) -> String {
            format!("sx9.agent.{}.request", agent_name.to_lowercase())
        }
    }

    pub mod response {
        pub const CLAUDE: &str = "sx9.agent.claude.response";
        pub const GPT: &str = "sx9.agent.gpt.response";
        pub const GEMINI: &str = "sx9.agent.gemini.response";
        pub const GROK: &str = "sx9.agent.grok.response";
        pub const CURSOR: &str = "sx9.agent.cursor.response";

        pub fn for_agent(agent_name: &str) -> String {
            format!("sx9.agent.{}.response", agent_name.to_lowercase())
        }
    }

    /// Agent status/heartbeat
    pub mod status {
        pub const HEARTBEAT: &str = "sx9.agent.status.heartbeat";
        pub const BUSY: &str = "sx9.agent.status.busy";
        pub const AVAILABLE: &str = "sx9.agent.status.available";
        pub const ERROR: &str = "sx9.agent.status.error";

        pub fn for_agent(agent_name: &str) -> String {
            format!("sx9.agent.{}.status", agent_name.to_lowercase())
        }
    }

    /// Task assignment (from Linear or direct)
    pub mod task {
        pub const ASSIGN: &str = "sx9.agent.task.assign";
        pub const ACCEPT: &str = "sx9.agent.task.accept";
        pub const REJECT: &str = "sx9.agent.task.reject";
        pub const COMPLETE: &str = "sx9.agent.task.complete";
        pub const HANDOFF: &str = "sx9.agent.task.handoff";
    }
}

/// Check if subject should use JetStream (RFC-9400 Section 4)
pub fn is_jetstream_subject(subject: &str) -> bool {
    subject.starts_with("sx9.kali.")
        || subject.starts_with("sx9.hash.")
        || subject.starts_with("sx9.crate.")
        || subject.starts_with("sx9.cdn.")
        || subject.starts_with("sx9.iac.")
        || subject.starts_with("sx9.gateway.")
        || subject.starts_with("sx9.telemetry.")
        || subject.starts_with("sx9.health.metrics")
        || subject.starts_with("sx9.health.alert")
}

/// Check if subject uses Core NATS (real-time, <50µs)
pub fn is_core_nats_subject(subject: &str) -> bool {
    subject.starts_with("sx9.tick.")
        || subject.starts_with("sx9.atlas.ooda.")
        || subject.starts_with("sx9.bus.")
        || subject.starts_with("sx9.plasma.")
        || subject.starts_with("sx9.mux.")
        || subject.starts_with("sx9.health.heartbeat")
        || subject.starts_with("sx9.heartbeat.") // Smart crate heartbeats (RFC-9141)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_subjects() {
        assert_eq!(tick::MS1, "sx9.tick.1ms");
        assert_eq!(tick::SYNC, "sx9.tick.sync");
    }

    #[test]
    fn test_atlas_ooda() {
        assert_eq!(atlas::ooda::OBSERVE, "sx9.atlas.ooda.observe");
        assert_eq!(atlas::ooda::ORIENT, "sx9.atlas.ooda.orient");
        assert_eq!(atlas::ooda::DECIDE, "sx9.atlas.ooda.decide");
        assert_eq!(atlas::ooda::ACT, "sx9.atlas.ooda.act");
    }

    #[test]
    fn test_kali_subjects() {
        assert_eq!(kali::exec_for("nmap"), "sx9.kali.exec.nmap");
        assert_eq!(kali::result_for("abc123"), "sx9.kali.result.abc123");
    }

    #[test]
    fn test_forge_subjects() {
        assert_eq!(forge::agent::by_persona("Axiom"), "sx9.forge.agent.axiom");
        assert_eq!(
            forge::task::by_id("uuid-001-002-003"),
            "sx9.forge.task.uuid-001-002-003"
        );
        assert_eq!(forge::pipeline::qa_grade('A'), "sx9.forge.pipeline.qa.A");
        assert_eq!(forge::harness::by_mode("Build"), "sx9.forge.harness.build");
    }

    #[test]
    fn test_intelligence_subjects_mirror_redux() {
        assert_eq!(
            intelligence::leptose::CONNECT,
            "sx9.intelligence.leptose.connect"
        );
        assert_eq!(
            intelligence::patterns::QUERY,
            "sx9.intelligence.patterns.query"
        );
        assert_eq!(intelligence::eei::ASK, "sx9.intelligence.eei.ask");
    }

    #[test]
    fn test_jetstream_detection() {
        assert!(is_jetstream_subject("sx9.kali.exec.nmap"));
        assert!(is_jetstream_subject("sx9.hash.compute.murmur3"));
        assert!(is_jetstream_subject("sx9.gateway.request.api"));
        assert!(!is_jetstream_subject("sx9.tick.1ms"));
        assert!(!is_jetstream_subject("sx9.atlas.ooda.observe"));
    }

    #[test]
    fn test_core_nats_detection() {
        assert!(is_core_nats_subject("sx9.tick.1ms"));
        assert!(is_core_nats_subject("sx9.atlas.ooda.observe"));
        assert!(is_core_nats_subject("sx9.plasma.entity.spawn"));
        assert!(is_core_nats_subject("sx9.health.heartbeat.atlas"));
        assert!(!is_core_nats_subject("sx9.kali.exec.nmap"));
    }

    #[test]
    fn test_linear_slack_top_level() {
        assert_eq!(linear::CREATE, "sx9.linear.issue.create");
        assert_eq!(linear::issue("SX9-123"), "sx9.linear.issue.SX9-123");
        assert_eq!(slack::NOTIFY, "sx9.slack.notify");
        assert_eq!(
            slack::channel("forge-alerts"),
            "sx9.slack.channel.forge-alerts"
        );
    }

    #[test]
    fn test_slack_mentions() {
        assert_eq!(slack::mention::RECEIVED, "sx9.slack.mention.received");
        assert_eq!(
            slack::mention::agent("Claude"),
            "sx9.slack.mention.claude"
        );
        assert_eq!(
            slack::mention::agent("GPT"),
            "sx9.slack.mention.gpt"
        );
    }

    #[test]
    fn test_agent_routing() {
        assert_eq!(agent::request::CLAUDE, "sx9.agent.claude.request");
        assert_eq!(agent::request::GPT, "sx9.agent.gpt.request");
        assert_eq!(agent::response::GEMINI, "sx9.agent.gemini.response");
        assert_eq!(
            agent::request::for_agent("Grok"),
            "sx9.agent.grok.request"
        );
        assert_eq!(
            agent::response::for_agent("Cursor"),
            "sx9.agent.cursor.response"
        );
    }

    #[test]
    fn test_agent_registry() {
        assert_eq!(agent::registry::REGISTER, "sx9.agent.registry.register");
        assert_eq!(agent::registry::LIST, "sx9.agent.registry.list");
        assert_eq!(agent::task::ASSIGN, "sx9.agent.task.assign");
        assert_eq!(agent::task::HANDOFF, "sx9.agent.task.handoff");
    }

    #[test]
    fn test_governance_drift_subjects() {
        assert_eq!(
            forge::governance::DRIFT_DETECTED,
            "sx9.forge.governance.drift.detected"
        );
        assert_eq!(
            forge::governance::DRIFT_ROLE,
            "sx9.forge.governance.drift.role"
        );
        assert_eq!(
            forge::governance::DRIFT_CONSTRAINT,
            "sx9.forge.governance.drift.constraint"
        );
        assert_eq!(
            forge::governance::drift_for("agent_registry"),
            "sx9.forge.governance.drift.agent_registry"
        );
    }

    #[test]
    fn test_governance_gate_subjects() {
        assert_eq!(
            forge::governance::GATE_OBSERVE,
            "sx9.forge.governance.gate.observe"
        );
        assert_eq!(
            forge::governance::GATE_ESCALATE,
            "sx9.forge.governance.gate.escalate"
        );
        assert_eq!(
            forge::governance::gate_for_pr("123"),
            "sx9.forge.governance.gate.pr.123"
        );
    }

    #[test]
    fn test_governance_lineage_subjects() {
        assert_eq!(
            forge::governance::LINEAGE_REGRESSION,
            "sx9.forge.governance.lineage.regression"
        );
        assert_eq!(
            forge::governance::LINEAGE_LOSS,
            "sx9.forge.governance.lineage.annotation_loss"
        );
        assert_eq!(
            forge::governance::lineage_for_commit("abc123"),
            "sx9.forge.governance.lineage.commit.abc123"
        );
    }

    #[test]
    fn test_static_semantic_qa_subjects() {
        assert_eq!(forge::static_qa::PASSED, "sx9.forge.qa.static.passed");
        assert_eq!(forge::static_qa::FAILED, "sx9.forge.qa.static.failed");
        assert_eq!(
            forge::static_qa::for_crate("sx9-harness"),
            "sx9.forge.qa.static.sx9-harness"
        );
        assert_eq!(
            forge::semantic_qa::DRIFT_ADVISORY,
            "sx9.forge.qa.semantic.drift_advisory"
        );
    }
}
