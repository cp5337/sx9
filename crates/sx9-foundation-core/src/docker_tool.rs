use crate::{Tool, ToolResult, Content, Config};
use anyhow::Result;
use async_trait::async_trait;
use bollard::{Docker, API_DEFAULT_VERSION};
use bollard::container::{CreateContainerOptions, Config as ContainerConfig, StartContainerOptions};
use bollard::image::CreateImageOptions;
use bollard::models::{CreateImageInfo, ContainerCreateResponse};
use serde_json::{json, Value};
use tracing::{info, error};
use futures::StreamExt;

/// Tesla-grade Docker operations tool
pub struct DockerTool {
    docker: Docker,
    config: Config,
}

impl DockerTool {
    pub fn new(config: &Config) -> Result<Self> {
        let docker = Docker::connect_with_socket_defaults()?;

        Ok(Self {
            docker,
            config: config.clone(),
        })
    }

    async fn create_smart_crate_container(&self, args: Value) -> Result<ToolResult> {
        let image = args.get("image")
            .and_then(|v| v.as_str())
            .unwrap_or("ctas7/smart-crate:latest");

        let name = args.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("smart-crate-instance");

        let ports = args.get("ports")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(|| vec!["18000:8080"]);

        info!("🐳 Creating smart crate container: {} with image: {}", name, image);

        // Create container configuration
        let mut exposed_ports = std::collections::HashMap::new();
        let mut port_bindings = std::collections::HashMap::new();

        for port_mapping in &ports {
            if let Some((host_port, container_port)) = port_mapping.split_once(':') {
                exposed_ports.insert(format!("{}/tcp", container_port), std::collections::HashMap::new());
                port_bindings.insert(
                    format!("{}/tcp", container_port),
                    Some(vec![bollard::models::PortBinding {
                        host_ip: Some("0.0.0.0".to_string()),
                        host_port: Some(host_port.to_string()),
                    }]),
                );
            }
        }

        let config = ContainerConfig {
            image: Some(image.to_string()),
            exposed_ports: Some(exposed_ports),
            env: Some(vec![
                "CTAS7_MODE=enterprise".to_string(),
                "NEURAL_MUX_ENDPOINT=http://host.docker.internal:15180".to_string(),
                "LOG_LEVEL=info".to_string(),
            ]),
            host_config: Some(bollard::models::HostConfig {
                port_bindings: Some(port_bindings),
                restart_policy: Some(bollard::models::RestartPolicy {
                    name: Some(bollard::models::RestartPolicyNameEnum::UNLESS_STOPPED),
                    maximum_retry_count: None,
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        // Create container
        let response: ContainerCreateResponse = self.docker
            .create_container(Some(CreateContainerOptions { name }), config)
            .await?;

        // Start container
        self.docker
            .start_container(&response.id, None::<StartContainerOptions<String>>)
            .await?;

        let result = json!({
            "container_id": response.id,
            "container_name": name,
            "image": image,
            "status": "created_and_started",
            "ports": ports,
            "warnings": response.warnings,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        Ok(ToolResult {
            content: vec![Content::Text {
                text: serde_json::to_string_pretty(&result)?,
            }],
            is_error: false,
        })
    }

    async fn list_smart_crate_containers(&self) -> Result<ToolResult> {
        let containers = self.docker
            .list_containers(Some(bollard::container::ListContainersOptions::<String> {
                all: true,
                filters: {
                    let mut filters = std::collections::HashMap::new();
                    filters.insert("label".to_string(), vec!["ctas7=smart-crate".to_string()]);
                    filters
                },
                ..Default::default()
            }))
            .await?;

        let container_info: Vec<_> = containers.iter().map(|container| {
            json!({
                "id": container.id,
                "image": container.image,
                "names": container.names,
                "state": container.state,
                "status": container.status,
                "ports": container.ports,
                "created": container.created
            })
        }).collect();

        let result = json!({
            "containers": container_info,
            "total": container_info.len(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        Ok(ToolResult {
            content: vec![Content::Text {
                text: serde_json::to_string_pretty(&result)?,
            }],
            is_error: false,
        })
    }

    async fn stop_container(&self, args: Value) -> Result<ToolResult> {
        let container_id = args.get("container_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing container_id"))?;

        info!("🛑 Stopping container: {}", container_id);

        self.docker
            .stop_container(container_id, None)
            .await?;

        let result = json!({
            "container_id": container_id,
            "action": "stopped",
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        Ok(ToolResult {
            content: vec![Content::Text {
                text: serde_json::to_string_pretty(&result)?,
            }],
            is_error: false,
        })
    }

    async fn build_smart_crate_image(&self, args: Value) -> Result<ToolResult> {
        let dockerfile = args.get("dockerfile")
            .and_then(|v| v.as_str())
            .unwrap_or("FROM rust:1.75-slim\nWORKDIR /app\nCOPY . .\nRUN cargo build --release\nEXPOSE 8080\nCMD [\"./target/release/smart-crate\"]");

        let tag = args.get("tag")
            .and_then(|v| v.as_str())
            .unwrap_or("ctas7/smart-crate:latest");

        info!("🔨 Building smart crate image: {}", tag);

        // Create temporary build context
        let build_context = tar::Builder::new(Vec::new());
        // In a real implementation, we'd create a proper build context

        let mut image_build_stream = self.docker.build_image(
            bollard::image::BuildImageOptions {
                dockerfile: "Dockerfile".to_string(),
                t: tag.to_string(),
                rm: true,
                ..Default::default()
            },
            None,
            Some(build_context.into_inner()?.into()),
        );

        let mut build_logs = Vec::new();
        while let Some(msg) = image_build_stream.next().await {
            match msg {
                Ok(info) => {
                    if let Some(stream) = &info.stream {
                        build_logs.push(stream.clone());
                    }
                }
                Err(e) => {
                    error!("Build error: {}", e);
                    return Ok(ToolResult {
                        content: vec![Content::Text {
                            text: format!("Build failed: {}", e),
                        }],
                        is_error: true,
                    });
                }
            }
        }

        let result = json!({
            "image_tag": tag,
            "action": "built",
            "build_logs": build_logs,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        Ok(ToolResult {
            content: vec![Content::Text {
                text: serde_json::to_string_pretty(&result)?,
            }],
            is_error: false,
        })
    }
}

#[async_trait]
impl Tool for DockerTool {
    fn name(&self) -> &str {
        "docker_operations"
    }

    fn description(&self) -> &str {
        "Tesla-grade Docker operations for smart crate deployment and management"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["create_container", "list_containers", "stop_container", "build_image"],
                    "description": "Docker operation to perform"
                },
                "image": {
                    "type": "string",
                    "description": "Docker image name (for create_container)"
                },
                "name": {
                    "type": "string",
                    "description": "Container name (for create_container)"
                },
                "ports": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Port mappings in format 'host:container'"
                },
                "container_id": {
                    "type": "string",
                    "description": "Container ID (for stop_container)"
                },
                "dockerfile": {
                    "type": "string",
                    "description": "Dockerfile content (for build_image)"
                },
                "tag": {
                    "type": "string",
                    "description": "Image tag (for build_image)"
                }
            },
            "required": ["operation"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let operation = args.get("operation")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing operation"))?;

        match operation {
            "create_container" => self.create_smart_crate_container(args).await,
            "list_containers" => self.list_smart_crate_containers().await,
            "stop_container" => self.stop_container(args).await,
            "build_image" => self.build_smart_crate_image(args).await,
            _ => Err(anyhow::anyhow!("Unknown operation: {}", operation)),
        }
    }
}