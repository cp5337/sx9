//! CLI Manifest System - Frontend Integration
//!
//! Provides CLI manifests for n8n-type system, Apple native frontend,
//! and other development interfaces

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// CLI Manifest for frontend integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLIManifest {
    pub manifest_version: String,
    pub ctas_version: String,
    pub cli_endpoints: Vec<CLIEndpoint>,
    pub n8n_integration: N8NIntegration,
    pub apple_native_integration: AppleNativeIntegration,
    pub dev_interfaces: Vec<DevInterface>,
    pub commands: Vec<CLICommand>,
}

/// CLI Endpoint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLIEndpoint {
    pub name: String,
    pub endpoint: String,
    pub method: String,
    pub description: String,
    pub parameters: Vec<CLIParameter>,
    pub response_format: String,
}

/// CLI Parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLIParameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
    pub description: String,
    pub default_value: Option<String>,
}

/// N8N Integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N8NIntegration {
    pub enabled: bool,
    pub webhook_endpoints: Vec<String>,
    pub workflow_triggers: Vec<WorkflowTrigger>,
    pub linear_integration: LinearIntegration,
}

/// Apple Native Integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleNativeIntegration {
    pub enabled: bool,
    pub swift_bridge_endpoints: Vec<String>,
    pub ios_app_integration: iOSIntegration,
    pub macos_app_integration: macOSIntegration,
}

/// Linear workspace integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearIntegration {
    pub enabled: bool,
    pub workspace_url: String,
    pub api_endpoints: Vec<String>,
    pub issue_tracking: bool,
    pub project_management: bool,
}

/// Workflow trigger for automation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTrigger {
    pub trigger_name: String,
    pub trigger_type: String,
    pub endpoint: String,
    pub conditions: Vec<String>,
}

/// Development interface definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevInterface {
    pub interface_name: String,
    pub interface_type: String, // "web", "cli", "native"
    pub endpoints: Vec<String>,
    pub authentication: AuthConfig,
}

/// iOS Integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct iOSIntegration {
    pub bundle_id: String,
    pub swift_bridge_version: String,
    pub supported_features: Vec<String>,
}

/// macOS Integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct macOSIntegration {
    pub bundle_id: String,
    pub swift_bridge_version: String,
    pub supported_features: Vec<String>,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub auth_type: String,
    pub required: bool,
    pub token_endpoint: Option<String>,
}

/// CLI Command definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLICommand {
    pub command: String,
    pub description: String,
    pub usage: String,
    pub flags: Vec<CLIFlag>,
    pub examples: Vec<String>,
}

/// CLI Flag definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLIFlag {
    pub flag: String,
    pub description: String,
    pub required: bool,
    pub default_value: Option<String>,
}

impl CLIManifest {
    pub fn new() -> Self {
        Self {
            manifest_version: "1.0.0".to_string(),
            ctas_version: "7.0.0".to_string(),
            cli_endpoints: Vec::new(),
            n8n_integration: N8NIntegration::default(),
            apple_native_integration: AppleNativeIntegration::default(),
            dev_interfaces: Vec::new(),
            commands: Vec::new(),
        }
    }

    /// Generate CLI manifest with all endpoints and integrations
    pub async fn generate_cli_manifest(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Generate CTE integration endpoints
        self.add_cte_endpoints();

        // Generate foundation endpoints
        self.add_foundation_endpoints();

        // Generate Linear integration
        self.configure_linear_integration();

        // Generate Apple native integration
        self.configure_apple_native_integration();

        // Generate n8n workflow integration
        self.configure_n8n_integration();

        println!(
            "📋 CLI Manifest generated with {} endpoints",
            self.cli_endpoints.len()
        );
        Ok(())
    }

    /// Add CTE-specific endpoints
    fn add_cte_endpoints(&mut self) {
        self.cli_endpoints.push(CLIEndpoint {
            name: "cte_health".to_string(),
            endpoint: "http://localhost:15180/health".to_string(),
            method: "GET".to_string(),
            description: "CTE health check endpoint".to_string(),
            parameters: vec![],
            response_format: "json".to_string(),
        });

        self.cli_endpoints.push(CLIEndpoint {
            name: "repo_status".to_string(),
            endpoint: "http://localhost:15180/repo/status".to_string(),
            method: "GET".to_string(),
            description: "Repository status and metrics".to_string(),
            parameters: vec![],
            response_format: "json".to_string(),
        });

        self.cli_endpoints.push(CLIEndpoint {
            name: "playbook_mux".to_string(),
            endpoint: "http://localhost:15180/mux/playbook".to_string(),
            method: "GET".to_string(),
            description: "Playbook CDN resolver".to_string(),
            parameters: vec![
                CLIParameter {
                    name: "id".to_string(),
                    param_type: "string".to_string(),
                    required: true,
                    description: "Playbook ID".to_string(),
                    default_value: None,
                },
                CLIParameter {
                    name: "fmt".to_string(),
                    param_type: "string".to_string(),
                    required: false,
                    description: "Format (xml/asm)".to_string(),
                    default_value: Some("xml".to_string()),
                },
            ],
            response_format: "json".to_string(),
        });
    }

    /// Add foundation-specific endpoints
    fn add_foundation_endpoints(&mut self) {
        self.cli_endpoints.push(CLIEndpoint {
            name: "trivariate_hash".to_string(),
            endpoint: "/foundation/hash/generate".to_string(),
            method: "POST".to_string(),
            description: "Generate trivariate hash (Murmur3)".to_string(),
            parameters: vec![
                CLIParameter {
                    name: "content".to_string(),
                    param_type: "string".to_string(),
                    required: true,
                    description: "Content to hash".to_string(),
                    default_value: None,
                },
                CLIParameter {
                    name: "context".to_string(),
                    param_type: "string".to_string(),
                    required: true,
                    description: "Context for CUID".to_string(),
                    default_value: None,
                },
                CLIParameter {
                    name: "primitive_type".to_string(),
                    param_type: "string".to_string(),
                    required: true,
                    description: "CTAS primitive type".to_string(),
                    default_value: None,
                },
            ],
            response_format: "json".to_string(),
        });
    }

    /// Configure Linear workspace integration
    fn configure_linear_integration(&mut self) {
        self.n8n_integration.linear_integration = LinearIntegration {
            enabled: true,
            workspace_url: "https://linear.app/ctas".to_string(),
            api_endpoints: vec![
                "/api/issues".to_string(),
                "/api/projects".to_string(),
                "/api/teams".to_string(),
                "/api/cycles".to_string(),
            ],
            issue_tracking: true,
            project_management: true,
        };
    }

    /// Configure Apple native integration
    fn configure_apple_native_integration(&mut self) {
        self.apple_native_integration = AppleNativeIntegration {
            enabled: true,
            swift_bridge_endpoints: vec![
                "/swift/foundation/core".to_string(),
                "/swift/foundation/data".to_string(),
                "/swift/foundation/interface".to_string(),
                "/swift/foundation/tactical".to_string(),
            ],
            ios_app_integration: iOSIntegration {
                bundle_id: "com.ctas.dev.ios".to_string(),
                swift_bridge_version: "1.0.0".to_string(),
                supported_features: vec![
                    "trivariate_hash".to_string(),
                    "linear_integration".to_string(),
                    "cte_health".to_string(),
                ],
            },
            macos_app_integration: macOSIntegration {
                bundle_id: "com.ctas.dev.macos".to_string(),
                swift_bridge_version: "1.0.0".to_string(),
                supported_features: vec![
                    "full_foundation_access".to_string(),
                    "development_tools".to_string(),
                    "workflow_automation".to_string(),
                ],
            },
        };
    }

    /// Configure n8n workflow integration
    fn configure_n8n_integration(&mut self) {
        self.n8n_integration = N8NIntegration {
            enabled: true,
            webhook_endpoints: vec![
                "/webhook/cte/health".to_string(),
                "/webhook/foundation/update".to_string(),
                "/webhook/linear/issue".to_string(),
            ],
            workflow_triggers: vec![
                WorkflowTrigger {
                    trigger_name: "foundation_health_check".to_string(),
                    trigger_type: "schedule".to_string(),
                    endpoint: "/webhook/cte/health".to_string(),
                    conditions: vec!["every_5_minutes".to_string()],
                },
                WorkflowTrigger {
                    trigger_name: "linear_issue_update".to_string(),
                    trigger_type: "webhook".to_string(),
                    endpoint: "/webhook/linear/issue".to_string(),
                    conditions: vec!["issue_status_changed".to_string()],
                },
            ],
            linear_integration: LinearIntegration::default(),
        };
    }

    /// Export manifest as JSON for frontend consumption
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Export manifest as YAML for configuration
    pub fn export_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

impl Default for CLIManifest {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for N8NIntegration {
    fn default() -> Self {
        Self {
            enabled: false,
            webhook_endpoints: Vec::new(),
            workflow_triggers: Vec::new(),
            linear_integration: LinearIntegration::default(),
        }
    }
}

impl Default for AppleNativeIntegration {
    fn default() -> Self {
        Self {
            enabled: false,
            swift_bridge_endpoints: Vec::new(),
            ios_app_integration: iOSIntegration::default(),
            macos_app_integration: macOSIntegration::default(),
        }
    }
}

impl Default for LinearIntegration {
    fn default() -> Self {
        Self {
            enabled: false,
            workspace_url: String::new(),
            api_endpoints: Vec::new(),
            issue_tracking: false,
            project_management: false,
        }
    }
}

impl Default for iOSIntegration {
    fn default() -> Self {
        Self {
            bundle_id: String::new(),
            swift_bridge_version: String::new(),
            supported_features: Vec::new(),
        }
    }
}

impl Default for macOSIntegration {
    fn default() -> Self {
        Self {
            bundle_id: String::new(),
            swift_bridge_version: String::new(),
            supported_features: Vec::new(),
        }
    }
}
