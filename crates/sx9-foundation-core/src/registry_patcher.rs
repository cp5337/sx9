//! # Crate Registry Patcher
//!
//! Patches initial data to crate registry for cannon plug system verification

use serde::{Deserialize, Serialize};
use reqwest::Client;
use tracing::{info, warn, error};

/// Registry patcher for initial data seeding
#[derive(Debug)]
pub struct RegistryPatcher {
    client: Client,
    cannon_plug_url: String,
    registry_data: InitialRegistryData,
}

/// Initial data structure for crate registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialRegistryData {
    pub foundation_crates: Vec<FoundationCrateInfo>,
    pub analysis_results: Vec<MockAnalysisResult>,
    pub system_status: SystemStatusData,
    pub health_checks: Vec<HealthCheckData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundationCrateInfo {
    pub name: String,
    pub version: String,
    pub path: String,
    pub status: CrateStatus,
    pub capabilities: Vec<String>,
    pub health_endpoint: String,
    pub last_analysis: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrateStatus {
    Active,
    Inactive,
    Maintenance,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockAnalysisResult {
    pub crate_name: String,
    pub analysis_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub quality_score: f64,
    pub security_score: f64,
    pub performance_metrics: PerformanceMetrics,
    pub issues_found: Vec<AnalysisIssue>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub compilation_time_ms: u64,
    pub binary_size_kb: u64,
    pub dependency_count: u32,
    pub test_coverage: f64,
    pub benchmark_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisIssue {
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub description: String,
    pub file_path: String,
    pub line_number: Option<u32>,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueCategory {
    Security,
    Performance,
    CodeQuality,
    Documentation,
    Dependencies,
    Testing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusData {
    pub total_crates: u32,
    pub healthy_crates: u32,
    pub failed_crates: u32,
    pub last_update: chrono::DateTime<chrono::Utc>,
    pub system_load: f64,
    pub available_analyzers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckData {
    pub service_name: String,
    pub endpoint: String,
    pub status: ServiceStatus,
    pub response_time_ms: u64,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub error_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

impl RegistryPatcher {
    /// Create new registry patcher
    pub fn new(cannon_plug_url: String) -> Self {
        let client = Client::new();
        let registry_data = Self::generate_initial_data();

        Self {
            client,
            cannon_plug_url,
            registry_data,
        }
    }

    /// Generate realistic initial data for testing
    fn generate_initial_data() -> InitialRegistryData {
        let foundation_crates = vec![
            FoundationCrateInfo {
                name: "ctas7-core-foundation".to_string(),
                version: "0.1.0".to_string(),
                path: "/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-core-foundation-staging".to_string(),
                status: CrateStatus::Active,
                capabilities: vec![
                    "core_operations".to_string(),
                    "system_primitives".to_string(),
                    "foundation_apis".to_string(),
                ],
                health_endpoint: "http://localhost:18200/health/core".to_string(),
                last_analysis: chrono::Utc::now() - chrono::Duration::minutes(15),
            },
            FoundationCrateInfo {
                name: "ctas7-interface-foundation".to_string(),
                version: "0.1.0".to_string(),
                path: "/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-interface-foundation-staging".to_string(),
                status: CrateStatus::Active,
                capabilities: vec![
                    "interface_definitions".to_string(),
                    "api_contracts".to_string(),
                    "protocol_handlers".to_string(),
                ],
                health_endpoint: "http://localhost:18201/health/interface".to_string(),
                last_analysis: chrono::Utc::now() - chrono::Duration::minutes(8),
            },
            FoundationCrateInfo {
                name: "ctas7-data-foundation".to_string(),
                version: "0.1.0".to_string(),
                path: "/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-data-foundation-staging".to_string(),
                status: CrateStatus::Active,
                capabilities: vec![
                    "data_structures".to_string(),
                    "serialization".to_string(),
                    "data_validation".to_string(),
                ],
                health_endpoint: "http://localhost:18202/health/data".to_string(),
                last_analysis: chrono::Utc::now() - chrono::Duration::minutes(12),
            },
            FoundationCrateInfo {
                name: "ctas7-leptose-knowledge-engine".to_string(),
                version: "0.1.0".to_string(),
                path: "/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-candidate-crates-staging/ctas7-leptose-knowledge-engine".to_string(),
                status: CrateStatus::Active,
                capabilities: vec![
                    "unified_intelligence".to_string(),
                    "ooda_automation".to_string(),
                    "hash_orchestration".to_string(),
                    "multi_database_integration".to_string(),
                ],
                health_endpoint: "http://localhost:8080/health".to_string(),
                last_analysis: chrono::Utc::now() - chrono::Duration::minutes(2),
            },
        ];

        let analysis_results = vec![
            MockAnalysisResult {
                crate_name: "ctas7-core-foundation".to_string(),
                analysis_id: "analysis_001".to_string(),
                timestamp: chrono::Utc::now() - chrono::Duration::minutes(15),
                quality_score: 92.5,
                security_score: 88.7,
                performance_metrics: PerformanceMetrics {
                    compilation_time_ms: 1250,
                    binary_size_kb: 2048,
                    dependency_count: 12,
                    test_coverage: 95.2,
                    benchmark_score: 87.3,
                },
                issues_found: vec![
                    AnalysisIssue {
                        severity: IssueSeverity::Low,
                        category: IssueCategory::Documentation,
                        description: "Missing doc comment for public function".to_string(),
                        file_path: "src/core/mod.rs".to_string(),
                        line_number: Some(45),
                        suggestion: "Add /// documentation comment".to_string(),
                    },
                ],
                recommendations: vec![
                    "Consider adding more integration tests".to_string(),
                    "Update dependency versions".to_string(),
                ],
            },
            MockAnalysisResult {
                crate_name: "ctas7-leptose-knowledge-engine".to_string(),
                analysis_id: "analysis_002".to_string(),
                timestamp: chrono::Utc::now() - chrono::Duration::minutes(2),
                quality_score: 96.8,
                security_score: 94.2,
                performance_metrics: PerformanceMetrics {
                    compilation_time_ms: 3420,
                    binary_size_kb: 8192,
                    dependency_count: 28,
                    test_coverage: 89.5,
                    benchmark_score: 93.7,
                },
                issues_found: vec![],
                recommendations: vec![
                    "Excellent code quality maintained".to_string(),
                    "Consider optimizing async operations".to_string(),
                ],
            },
        ];

        let system_status = SystemStatusData {
            total_crates: 4,
            healthy_crates: 4,
            failed_crates: 0,
            last_update: chrono::Utc::now(),
            system_load: 0.67,
            available_analyzers: vec![
                "ctas-analyzer".to_string(),
                "statistical-analysis-cdn".to_string(),
                "unified-knowledge-engine".to_string(),
            ],
        };

        let health_checks = vec![
            HealthCheckData {
                service_name: "cannon-plug-api".to_string(),
                endpoint: "http://localhost:18100/health".to_string(),
                status: ServiceStatus::Healthy,
                response_time_ms: 12,
                last_check: chrono::Utc::now(),
                error_count: 0,
            },
            HealthCheckData {
                service_name: "ctas-analyzer".to_string(),
                endpoint: "http://localhost:18109/health".to_string(),
                status: ServiceStatus::Healthy,
                response_time_ms: 8,
                last_check: chrono::Utc::now(),
                error_count: 0,
            },
            HealthCheckData {
                service_name: "statistical-analysis-cdn".to_string(),
                endpoint: "http://localhost:18108/health".to_string(),
                status: ServiceStatus::Healthy,
                response_time_ms: 15,
                last_check: chrono::Utc::now(),
                error_count: 0,
            },
        ];

        InitialRegistryData {
            foundation_crates,
            analysis_results,
            system_status,
            health_checks,
        }
    }

    /// Patch initial data to crate registry
    pub async fn patch_registry_data(&self) -> Result<PatchResult, RegistryError> {
        info!("📡 Patching initial data to crate registry via cannon plug");

        let mut results = Vec::new();

        // Patch foundation crates
        for crate_info in &self.registry_data.foundation_crates {
            let result = self.patch_crate_info(crate_info).await?;
            results.push(("crate_info".to_string(), result));
        }

        // Patch analysis results
        for analysis in &self.registry_data.analysis_results {
            let result = self.patch_analysis_result(analysis).await?;
            results.push(("analysis_result".to_string(), result));
        }

        // Patch system status
        let status_result = self.patch_system_status(&self.registry_data.system_status).await?;
        results.push(("system_status".to_string(), status_result));

        // Patch health checks
        for health_check in &self.registry_data.health_checks {
            let result = self.patch_health_check(health_check).await?;
            results.push(("health_check".to_string(), result));
        }

        info!("✅ Successfully patched {} data items to registry", results.len());

        Ok(PatchResult {
            total_items: results.len(),
            successful_patches: results.iter().filter(|(_, success)| *success).count(),
            failed_patches: results.iter().filter(|(_, success)| !*success).count(),
            details: results,
        })
    }

    /// Patch individual crate information
    async fn patch_crate_info(&self, crate_info: &FoundationCrateInfo) -> Result<bool, RegistryError> {
        let endpoint = format!("{}/cannon/registry/crates", self.cannon_plug_url);

        let payload = serde_json::json!({
            "action": "register_crate",
            "data": {
                "name": crate_info.name,
                "version": crate_info.version,
                "path": crate_info.path,
                "status": crate_info.status,
                "capabilities": crate_info.capabilities,
                "health_endpoint": crate_info.health_endpoint,
                "last_analysis": crate_info.last_analysis,
                "timestamp": chrono::Utc::now()
            }
        });

        match self.client
            .post(&endpoint)
            .json(&payload)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                info!("✅ Patched crate info: {}", crate_info.name);
                Ok(true)
            }
            Ok(response) => {
                warn!("⚠️ Failed to patch crate {}: HTTP {}", crate_info.name, response.status());
                Ok(false)
            }
            Err(e) => {
                error!("❌ Network error patching crate {}: {}", crate_info.name, e);
                Ok(false)
            }
        }
    }

    /// Patch analysis results
    async fn patch_analysis_result(&self, analysis: &MockAnalysisResult) -> Result<bool, RegistryError> {
        let endpoint = format!("{}/cannon/registry/analysis", self.cannon_plug_url);

        let payload = serde_json::json!({
            "action": "store_analysis",
            "data": {
                "crate_name": analysis.crate_name,
                "analysis_id": analysis.analysis_id,
                "timestamp": analysis.timestamp,
                "quality_score": analysis.quality_score,
                "security_score": analysis.security_score,
                "performance_metrics": analysis.performance_metrics,
                "issues_found": analysis.issues_found,
                "recommendations": analysis.recommendations
            }
        });

        match self.client
            .post(&endpoint)
            .json(&payload)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                info!("✅ Patched analysis result: {}", analysis.crate_name);
                Ok(true)
            }
            Ok(response) => {
                warn!("⚠️ Failed to patch analysis for {}: HTTP {}", analysis.crate_name, response.status());
                Ok(false)
            }
            Err(e) => {
                error!("❌ Network error patching analysis for {}: {}", analysis.crate_name, e);
                Ok(false)
            }
        }
    }

    /// Patch system status
    async fn patch_system_status(&self, status: &SystemStatusData) -> Result<bool, RegistryError> {
        let endpoint = format!("{}/cannon/registry/status", self.cannon_plug_url);

        let payload = serde_json::json!({
            "action": "update_system_status",
            "data": {
                "total_crates": status.total_crates,
                "healthy_crates": status.healthy_crates,
                "failed_crates": status.failed_crates,
                "last_update": status.last_update,
                "system_load": status.system_load,
                "available_analyzers": status.available_analyzers
            }
        });

        match self.client
            .post(&endpoint)
            .json(&payload)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                info!("✅ Patched system status");
                Ok(true)
            }
            Ok(response) => {
                warn!("⚠️ Failed to patch system status: HTTP {}", response.status());
                Ok(false)
            }
            Err(e) => {
                error!("❌ Network error patching system status: {}", e);
                Ok(false)
            }
        }
    }

    /// Patch health check data
    async fn patch_health_check(&self, health_check: &HealthCheckData) -> Result<bool, RegistryError> {
        let endpoint = format!("{}/cannon/registry/health", self.cannon_plug_url);

        let payload = serde_json::json!({
            "action": "update_health_check",
            "data": {
                "service_name": health_check.service_name,
                "endpoint": health_check.endpoint,
                "status": health_check.status,
                "response_time_ms": health_check.response_time_ms,
                "last_check": health_check.last_check,
                "error_count": health_check.error_count
            }
        });

        match self.client
            .post(&endpoint)
            .json(&payload)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                info!("✅ Patched health check: {}", health_check.service_name);
                Ok(true)
            }
            Ok(response) => {
                warn!("⚠️ Failed to patch health check for {}: HTTP {}", health_check.service_name, response.status());
                Ok(false)
            }
            Err(e) => {
                error!("❌ Network error patching health check for {}: {}", health_check.service_name, e);
                Ok(false)
            }
        }
    }

    /// Test cannon plug connectivity
    pub async fn test_cannon_plug_connectivity(&self) -> Result<ConnectivityTestResult, RegistryError> {
        info!("🔌 Testing cannon plug system connectivity");

        let mut test_results = Vec::new();

        // Test main cannon plug API
        let main_result = self.test_endpoint(&format!("{}/health", self.cannon_plug_url)).await;
        test_results.push(("cannon-plug-api".to_string(), main_result));

        // Test registry endpoints
        let registry_endpoints = vec![
            ("/cannon/registry/crates", "crates-registry"),
            ("/cannon/registry/analysis", "analysis-registry"),
            ("/cannon/registry/status", "status-registry"),
            ("/cannon/registry/health", "health-registry"),
        ];

        for (path, name) in registry_endpoints {
            let endpoint = format!("{}{}", self.cannon_plug_url, path);
            let result = self.test_endpoint(&endpoint).await;
            test_results.push((name.to_string(), result));
        }

        // Test individual service health
        for health_check in &self.registry_data.health_checks {
            let result = self.test_endpoint(&health_check.endpoint).await;
            test_results.push((health_check.service_name.clone(), result));
        }

        let successful_tests = test_results.iter().filter(|(_, result)| result.is_success).count();
        let total_tests = test_results.len();

        info!("📊 Connectivity test completed: {}/{} tests passed", successful_tests, total_tests);

        Ok(ConnectivityTestResult {
            total_tests,
            successful_tests,
            failed_tests: total_tests - successful_tests,
            test_results,
            overall_health: if successful_tests as f64 / total_tests as f64 >= 0.8 {
                ConnectivityHealth::Healthy
            } else if successful_tests as f64 / total_tests as f64 >= 0.5 {
                ConnectivityHealth::Degraded
            } else {
                ConnectivityHealth::Unhealthy
            },
        })
    }

    async fn test_endpoint(&self, endpoint: &str) -> EndpointTestResult {
        let start_time = std::time::Instant::now();

        match self.client
            .get(endpoint)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => {
                let response_time = start_time.elapsed().as_millis() as u64;
                EndpointTestResult {
                    endpoint: endpoint.to_string(),
                    is_success: response.status().is_success(),
                    status_code: response.status().as_u16(),
                    response_time_ms: response_time,
                    error_message: None,
                }
            }
            Err(e) => {
                let response_time = start_time.elapsed().as_millis() as u64;
                EndpointTestResult {
                    endpoint: endpoint.to_string(),
                    is_success: false,
                    status_code: 0,
                    response_time_ms: response_time,
                    error_message: Some(e.to_string()),
                }
            }
        }
    }
}

/// Result of patching operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchResult {
    pub total_items: usize,
    pub successful_patches: usize,
    pub failed_patches: usize,
    pub details: Vec<(String, bool)>,
}

/// Result of connectivity testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectivityTestResult {
    pub total_tests: usize,
    pub successful_tests: usize,
    pub failed_tests: usize,
    pub test_results: Vec<(String, EndpointTestResult)>,
    pub overall_health: ConnectivityHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointTestResult {
    pub endpoint: String,
    pub is_success: bool,
    pub status_code: u16,
    pub response_time_ms: u64,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectivityHealth {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Error handling
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Registry error: {0}")]
    Registry(String),
    #[error("Connectivity error: {0}")]
    Connectivity(String),
}