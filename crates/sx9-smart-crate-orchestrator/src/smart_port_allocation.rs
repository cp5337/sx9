//! Smart Port Allocation System
//!
//! Integrates with CTAS-7 Port Manager and Neural Mux for intelligent port
//! allocation with collision avoidance and neural optimization.

use crate::SmartCrateOrchestrator;
use sx9_foundation_manifold::core::data::serde::{Deserialize, Serialize};
use sx9_foundation_manifold::core::diagnostics::tracing::{debug, info, instrument};
use sx9_foundation_manifold::core::diagnostics::{anyhow, Context, Result};

/// Port allocation request with neural mux optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortAllocationRequest {
    /// Service name requesting port
    pub service_name: String,
    /// Service type for categorization
    pub service_type: String,
    /// Preferred port (if any)
    pub preferred_port: Option<u16>,
    /// Port range constraints
    pub port_range: Option<(u16, u16)>,
    /// Neural mux priority (0-100)
    pub neural_priority: u8,
    /// Enable deception/mirror ports
    pub enable_deception: bool,
}

/// Port allocation response from port manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortAllocationResponse {
    /// Allocated port number
    pub port: u16,
    /// Mirror/deception ports (if enabled)
    pub mirror_ports: Vec<u16>,
    /// Allocation ID for tracking
    pub allocation_id: String,
    /// Neural optimization score
    pub neural_score: f64,
}

/// Neural mux optimization parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralMuxOptimization {
    /// Service load prediction
    pub load_prediction: f64,
    /// Network topology score
    pub topology_score: f64,
    /// Security assessment
    pub security_score: f64,
    /// Collision probability
    pub collision_probability: f64,
}

impl SmartCrateOrchestrator {
    /// Allocates a smart port with neural mux optimization
    #[instrument(level = "debug", skip(self))]
    pub async fn allocate_smart_port(
        &self,
        request: PortAllocationRequest,
    ) -> Result<PortAllocationResponse> {
        info!(
            "Requesting smart port allocation for: {}",
            request.service_name
        );

        // Step 1: Get neural mux optimization
        let neural_optimization = self.get_neural_optimization(&request).await?;

        // Step 2: Request port from port manager with neural data
        let allocation = self
            .request_port_allocation(request, neural_optimization)
            .await?;

        // Step 3: Validate allocation with collision detection
        self.validate_port_allocation(&allocation).await?;

        info!(
            "Successfully allocated port {} for {}",
            allocation.port, allocation.allocation_id
        );

        Ok(allocation)
    }

    /// Gets neural mux optimization parameters
    #[instrument(level = "debug", skip(self))]
    async fn get_neural_optimization(
        &self,
        request: &PortAllocationRequest,
    ) -> Result<NeuralMuxOptimization> {
        debug!(
            "Requesting neural optimization for service: {}",
            request.service_name
        );

        let optimization_request = serde_json::json!({
            "service_name": request.service_name,
            "service_type": request.service_type,
            "neural_priority": request.neural_priority,
            "action": "port_optimization"
        });

        let response = self
            .http_client
            .post(&format!("{}/neural-mux/optimize", self.neural_mux_url))
            .json(&optimization_request)
            .send()
            .await
            .context("Failed to contact neural mux")?;

        if !response.status().is_success() {
            // Fallback to default optimization if neural mux unavailable
            debug!("Neural mux unavailable, using default optimization");
            return Ok(NeuralMuxOptimization {
                load_prediction: 0.5,
                topology_score: 0.7,
                security_score: 0.8,
                collision_probability: 0.1,
            });
        }

        let optimization: NeuralMuxOptimization = response
            .json()
            .await
            .context("Failed to parse neural optimization response")?;

        debug!(
            "Neural optimization received: score={:.2}",
            optimization.topology_score
        );
        Ok(optimization)
    }

    /// Requests port allocation from port manager
    #[instrument(level = "debug", skip(self))]
    async fn request_port_allocation(
        &self,
        request: PortAllocationRequest,
        neural_opt: NeuralMuxOptimization,
    ) -> Result<PortAllocationResponse> {
        debug!("Requesting port allocation with neural optimization");

        let allocation_request = serde_json::json!({
            "service_name": request.service_name,
            "service_type": request.service_type,
            "preferred_port": request.preferred_port,
            "port_range": request.port_range,
            "enable_deception": request.enable_deception,
            "neural_optimization": {
                "load_prediction": neural_opt.load_prediction,
                "topology_score": neural_opt.topology_score,
                "security_score": neural_opt.security_score,
                "collision_probability": neural_opt.collision_probability
            }
        });

        let response = self
            .http_client
            .post(&format!("{}/ports/allocate", self.port_manager_url))
            .json(&allocation_request)
            .send()
            .await
            .context("Failed to contact port manager")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Port allocation failed: {}", error_text);
        }

        let allocation: PortAllocationResponse = response
            .json()
            .await
            .context("Failed to parse port allocation response")?;

        Ok(allocation)
    }

    /// Validates port allocation for collisions
    #[instrument(level = "debug", skip(self))]
    async fn validate_port_allocation(&self, allocation: &PortAllocationResponse) -> Result<()> {
        debug!("Validating port allocation: {}", allocation.allocation_id);

        // Check for port conflicts with existing services
        let response = self
            .http_client
            .get(&format!("{}/ports", self.port_manager_url))
            .send()
            .await
            .context("Failed to get current port allocations")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to retrieve port status for validation");
        }

        let current_allocations: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse current allocations")?;

        // Simple collision check - in production this would be more sophisticated
        if let Some(allocations) = current_allocations["allocations"].as_array() {
            for existing in allocations {
                if let Some(existing_port) = existing["port"].as_u64() {
                    if existing_port as u16 == allocation.port {
                        anyhow::bail!(
                            "Port collision detected: {} already allocated",
                            allocation.port
                        );
                    }
                }
            }
        }

        debug!("Port allocation validated successfully");
        Ok(())
    }

    /// Releases a previously allocated port
    #[instrument(level = "debug", skip(self))]
    pub async fn release_port(&self, allocation_id: &str) -> Result<()> {
        info!("Releasing port allocation: {}", allocation_id);

        let response = self
            .http_client
            .delete(&format!(
                "{}/ports/{}",
                self.port_manager_url, allocation_id
            ))
            .send()
            .await
            .context("Failed to release port")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Port release failed: {}", error_text);
        }

        info!("Port allocation {} released successfully", allocation_id);
        Ok(())
    }

    /// Gets current port status and availability
    #[instrument(level = "debug", skip(self))]
    pub async fn get_port_status(&self) -> Result<serde_json::Value> {
        debug!("Retrieving current port status");

        let response = self
            .http_client
            .get(&format!("{}/ports", self.port_manager_url))
            .send()
            .await
            .context("Failed to get port status")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to retrieve port status");
        }

        let status = response
            .json()
            .await
            .context("Failed to parse port status")?;

        Ok(status)
    }
}
