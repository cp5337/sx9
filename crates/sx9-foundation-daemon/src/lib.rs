// 🏗️ CTAS-7 Foundation Daemon Library
// Core modules for enterprise PM2 replacement

pub mod services {
    pub mod abe_controlled_access;
    pub mod api_vault_client;
    pub mod backend_mcp_server;
    pub mod service_discovery;
}

pub mod testing {
    pub mod performance_test_harness;
}

pub mod foundation_integration;

/// DSL (Domain Specific Language) for L2 hash-driven operations
/// Replaces verbose XSD/XML playbooks with Rust builder functions
pub mod dsl;

/// Threat Reaction Module - Recognize-Formulate-React Architecture
pub mod threat_reaction;
