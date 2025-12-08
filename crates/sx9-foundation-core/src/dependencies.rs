//! Consolidated dependency management for orchestrators

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Standard dependencies required across all orchestrators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardDependencies {
    pub logging: LoggingConfig,
    pub xsd_validation: XsdConfig,
    pub telemetry: TelemetryConfig,
    pub security: SecurityConfig,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub output_format: String,
    pub file_path: Option<String>,
}

/// XSD validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XsdConfig {
    pub schema_path: String,
    pub validation_enabled: bool,
    pub strict_mode: bool,
}

/// Telemetry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub sample_rate: f32,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub tls_enabled: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
}

/// Dependency resolver for orchestrator initialization
pub struct DependencyResolver {
    standard_deps: StandardDependencies,
    custom_deps: HashMap<Uuid, HashMap<String, String>>,
}

impl DependencyResolver {
    pub fn new(standard_deps: StandardDependencies) -> Self {
        Self {
            standard_deps,
            custom_deps: HashMap::new(),
        }
    }

    pub fn register_custom_dependency(&mut self, orchestrator_id: Uuid, key: String, value: String) {
        self.custom_deps
            .entry(orchestrator_id)
            .or_insert_with(HashMap::new)
            .insert(key, value);
    }

    pub fn get_dependencies(&self, orchestrator_id: Uuid) -> (StandardDependencies, Option<&HashMap<String, String>>) {
        (self.standard_deps.clone(), self.custom_deps.get(&orchestrator_id))
    }
}

impl Default for StandardDependencies {
    fn default() -> Self {
        Self {
            logging: LoggingConfig {
                level: "info".to_string(),
                output_format: "json".to_string(),
                file_path: None,
            },
            xsd_validation: XsdConfig {
                schema_path: "schemas/ctas7.xsd".to_string(),
                validation_enabled: true,
                strict_mode: false,
            },
            telemetry: TelemetryConfig {
                enabled: true,
                endpoint: "http://localhost:18200/telemetry".to_string(),
                sample_rate: 1.0,
            },
            security: SecurityConfig {
                tls_enabled: false,
                cert_path: None,
                key_path: None,
            },
        }
    }
}