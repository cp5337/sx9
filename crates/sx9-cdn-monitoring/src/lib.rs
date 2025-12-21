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
//! CTAS Monitoring CDN - Tesla-Grade Refactored
//!
//! Cloudflare-inspired edge network for progress and resource tracking
//! across all CTAS operations with global distribution and intelligent caching.
//!
//! Refactored to meet Tesla-grade code standards with modular architecture.

pub mod analytics;
pub mod cache;
pub mod cdn;
pub mod edge;
pub mod origin;
pub mod routing;
pub mod types;

// Tesla-Grade Refactored Modules (All under 200 lines)
pub mod analysis_engine; // 199 lines - Pattern matching & ML analysis
pub mod cyber_operations; // 241 lines - Cyber warfare capabilities
pub mod gateway_cdn_refactored;
pub mod intelligence_reports; // 147 lines - Intelligence reporting
pub mod nginx_manager; // 253 lines - NGINX configuration management
pub mod traffic_analysis_core; // 143 lines - Core traffic analysis engine
pub mod traffic_intelligence_refactored; // 154 lines - Orchestrates intelligence modules
pub mod traffic_types; // 68 lines - Traffic analysis data types // 258 lines - Core CDN functionality

// Legacy modules (DEPRECATED - VIOLATE TESLA-GRADE STANDARDS)
pub mod component_cdn;
pub mod gateway_cdn; // 734 lines - VIOLATES STANDARDS
pub mod gateway_handlers;
pub mod intelligence_handlers;
pub mod service_registry;
pub mod traffic_analysis; // 224 lines - VIOLATES STANDARDS
pub mod traffic_intelligence; // 391 lines - VIOLATES STANDARDS

pub use cdn::{
    add_edge_location, add_origin_server, get_cdn_analytics, get_cdn_health, route_request, CTASCDN,
};

// Use refactored Tesla-grade modules
pub use cyber_operations::{
    ActiveOperation, CyberOperations, OperationStatus, OperationType, ThreatLevel,
};
pub use gateway_cdn_refactored::{
    generate_cyber_ops_nginx_config, get_gateway_status, register_gateway_service,
    start_cyber_operation, GatewayCDN,
};
pub use nginx_manager::{CyberOpsConfig, NGINXConfig, NGINXConfigManager, NGINXTemplate};
// Use Tesla-grade refactored intelligence modules
pub use analysis_engine::{AnalysisEngine, CorrelationRule, MLModel, MLModelType, PatternMatcher};
pub use intelligence_reports::{
    Classification, IndicatorType, IntelligenceManager, IntelligenceReport, ReportType,
    ThreatIndicator,
};
pub use traffic_analysis_core::{TrafficAnalysis, TrafficStatistics};
pub use traffic_intelligence_refactored::{IntelligenceStats, TrafficIntelligence};
pub use traffic_types::{
    ActivityType, AnalysisResult, RequestData, ResponseAction, SuspiciousActivity,
};

pub use component_cdn::{
    create_component_cdn_routes, ComponentCDN, ComponentCategory, ComponentEntry, ComponentType,
    VisualizationMethod,
};
pub use gateway_handlers::*;
pub use intelligence_handlers::*;
pub use service_registry::ServiceRegistry;
pub use types::*;
