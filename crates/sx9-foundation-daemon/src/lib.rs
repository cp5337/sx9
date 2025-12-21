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
