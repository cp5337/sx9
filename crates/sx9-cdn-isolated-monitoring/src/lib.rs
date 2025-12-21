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
//! CTAS 7.0 Isolated Monitoring CDN
//!
//! High-fidelity, low-resource statistical monitoring system with CDN distribution
//! Designed for containerized isolation with academic-grade statistical analysis
//!
//! Features:
//! - High-precision performance monitoring (<2% overhead)
//! - Academic-grade statistical analysis and visualization  
//! - CDN-distributed monitoring with edge caching
//! - Complete container isolation for pristine results
//! - Real-time dashboard with scholarly reporting

pub mod core;

// Core monitoring types
pub use core::{MetricType, MonitoringCDN, MonitoringConfig, MonitoringError, PerformanceMetric};
// CTAS-7 Gold Disk Retrofit Integration
pub mod foundation_integration;
