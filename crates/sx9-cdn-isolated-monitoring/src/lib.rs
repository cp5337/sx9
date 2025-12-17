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
