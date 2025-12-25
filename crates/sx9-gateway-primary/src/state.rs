//! Gateway shared state
//!
//! Holds connections to all SX9 backend services, CDNs, and neural mux.

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::protocol::{ConnectionStatus, Database};

/// CDN configuration (SX9 port allocations)
#[derive(Debug, Clone)]
pub struct CdnConfig {
    pub id: &'static str,
    pub name: &'static str,
    pub port: u16,
    pub cdn_type: CdnType,
}

#[derive(Debug, Clone, Copy)]
pub enum CdnType {
    CloudflareR2,
    GcpCdn,
    Internal,
    Tunnel,
}

/// All CDN configurations (SX9)
pub const CDN_CONFIGS: &[CdnConfig] = &[
    CdnConfig {
        id: "cdn-static",
        name: "Static Assets",
        port: 19000,
        cdn_type: CdnType::CloudflareR2,
    },
    CdnConfig {
        id: "cdn-crates",
        name: "Rust Crates Registry",
        port: 19001,
        cdn_type: CdnType::CloudflareR2,
    },
    CdnConfig {
        id: "cdn-geo",
        name: "Geospatial Data",
        port: 19002,
        cdn_type: CdnType::GcpCdn,
    },
    CdnConfig {
        id: "cdn-models",
        name: "ML Models",
        port: 19003,
        cdn_type: CdnType::GcpCdn,
    },
    CdnConfig {
        id: "cdn-conda",
        name: "Conda Packages",
        port: 19010,
        cdn_type: CdnType::Internal,
    },
    CdnConfig {
        id: "cdn-tools",
        name: "Security Tools (Hermetic)",
        port: 19011,
        cdn_type: CdnType::Tunnel,
    },
    CdnConfig {
        id: "cdn-wasm",
        name: "WASM Modules",
        port: 19012,
        cdn_type: CdnType::Internal,
    },
    CdnConfig {
        id: "cdn-plasma",
        name: "Plasma Agent Distribution",
        port: 19013,
        cdn_type: CdnType::Tunnel,
    },
];

/// Port allocations (SX9 standard)
pub mod ports {
    // Core Infrastructure (18000-18099)
    pub const SUPABASE_POSTGRES: u16 = 18000;
    pub const SUPABASE_API: u16 = 18001;
    pub const SUPABASE_REALTIME: u16 = 18002;
    pub const NEON_POSTGRES: u16 = 18015;
    pub const NEON_POOLER: u16 = 18016;
    pub const NATS: u16 = 18020;
    pub const NATS_WS: u16 = 18021;
    pub const NATS_JETSTREAM: u16 = 18022;
    pub const REDIS: u16 = 18030;
    pub const DRAGONFLY: u16 = 18031;

    // Backend Services (18100-18199)
    pub const SX9_ORCHESTRATOR: u16 = 18100;
    pub const LEGION_ENGINE: u16 = 18101;
    pub const SCRIPT_COORDINATOR: u16 = 18102;
    pub const HASHING_ENGINE: u16 = 18105;
    pub const TRIVARIATE_SERVICE: u16 = 18106;
    pub const THALMIC_FILTER: u16 = 18110;
    pub const PROMPT_GENERATOR: u16 = 18111;
    pub const GLAF_ALLOCATOR: u16 = 18120;
    pub const CONVERGENCE_TRACKER: u16 = 18121;

    // Forge/Workflow (18300-18399)
    pub const FORGE_BACKEND: u16 = 18350;
    pub const N8N_EXTERNAL: u16 = 18351;
    pub const WORKFLOW_EXECUTOR: u16 = 18352;
    pub const TOOL_CHAIN_RUNNER: u16 = 18360;

    // Data Services (18400-18499)
    pub const SLED_HTTP_API: u16 = 18400;
    pub const SLED_ADMIN: u16 = 18401;
    pub const VECTOR_DB: u16 = 18410;
    pub const EMBEDDING_CACHE: u16 = 18411;

    // ML/AI Services (18500-18599)
    pub const ATLAS_DAEMON: u16 = 18500;
    pub const MODEL_REGISTRY: u16 = 18501;
    pub const ANN_INFERENCE: u16 = 18510;
    pub const GNN_INFERENCE: u16 = 18511;
    pub const LLM_PROXY: u16 = 18520;
    pub const EMBEDDING_SERVICE: u16 = 18521;
    pub const CLASSIFIER_SERVICE: u16 = 18522;

    // Security Tools (18600-18699)
    pub const GATEWAY: u16 = 18600; // THIS GATEWAY
    pub const NMAP_WRAPPER: u16 = 18601;
    pub const NUCLEI_WRAPPER: u16 = 18602;
    pub const MASSCAN_WRAPPER: u16 = 18603;
    pub const RECONNG_WRAPPER: u16 = 18604;
    pub const TOOL_ORCHESTRATOR: u16 = 18650;

    // Voice/Media (18700-18799)
    pub const WHISPER_STT: u16 = 18700;
    pub const ELEVENLABS_PROXY: u16 = 18701;
    pub const VOICE_PIPELINE: u16 = 18710;
    pub const MEDIA_STREAM: u16 = 18720;

    // Conda Bridge (18800-18899)
    pub const CONDA_API_MAIN: u16 = 18800;
    pub const CONDA_JUPYTER_KERNEL: u16 = 18801;
    pub const CONDA_NUMPY_SERVICE: u16 = 18810;
    pub const CONDA_SCIPY_SERVICE: u16 = 18811;
    pub const CONDA_PYTORCH_SERVICE: u16 = 18820;
    pub const CONDA_TENSORFLOW_SERVICE: u16 = 18821;
    pub const CONDA_SKLEARN_SERVICE: u16 = 18830;
    pub const CONDA_GEOPANDAS_SERVICE: u16 = 18840;
    pub const CONDA_NETWORKX_SERVICE: u16 = 18841;
    pub const CONDA_CUSTOM_ENV: u16 = 18850;

    // Monitoring (18900-18999)
    pub const PROMETHEUS: u16 = 18900;
    pub const GRAFANA: u16 = 18901;
    pub const JAEGER: u16 = 18910;
    pub const LOKI: u16 = 18920;
}

/// Neural Mux state for coordinating cognitive operations
/// Based on SX9 neural-mux architecture
#[derive(Debug, Clone, Default)]
pub struct NeuralMuxState {
    /// Current tick rate in microseconds
    pub tick_rate_us: u64,
    /// Latency in nanoseconds
    pub latency_ns: u64,
    /// Total ticks processed
    pub ticks_processed: u64,
    /// L-Star learning enabled
    pub l_star_enabled: bool,
    /// Voice orchestration enabled
    pub voice_enabled: bool,
}

/// Auto-provisioning configuration
#[derive(Debug, Clone)]
pub struct AutoProvisionConfig {
    /// Enable auto-provisioning of databases
    pub enabled: bool,
    /// Supabase project URL (from env SUPABASE_URL)
    pub supabase_url: Option<String>,
    /// Supabase anon key (from env SUPABASE_ANON_KEY)
    pub supabase_key: Option<String>,
    /// Neon connection string (from env NEON_DATABASE_URL)
    pub neon_url: Option<String>,
}

impl Default for AutoProvisionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            supabase_url: std::env::var("SUPABASE_URL").ok(),
            supabase_key: std::env::var("SUPABASE_ANON_KEY").ok(),
            neon_url: std::env::var("NEON_DATABASE_URL").ok(),
        }
    }
}

/// Shared gateway state
///
/// This struct holds connections to all backend services and is shared
/// across all WebSocket handlers via Arc.
pub struct GatewayState {
    /// Supabase HTTP client (REST API)
    pub supabase_url: RwLock<Option<String>>,
    pub supabase_key: RwLock<Option<String>>,

    /// Neon PostgreSQL connection string
    pub neon_url: RwLock<Option<String>>,

    /// NATS client for pub/sub and health
    pub nats: RwLock<Option<async_nats::Client>>,

    /// Connection statuses (updated by health checker)
    pub connection_statuses: RwLock<Vec<ConnectionStatus>>,

    /// PlasmaState from sx9-atlas-bus (if connected)
    pub plasma_snapshot: RwLock<Option<sx9_atlas_bus::PlasmaSnapshot>>,

    /// Neural Mux state for cognitive coordination
    pub neural_mux: RwLock<NeuralMuxState>,

    /// CDN health status
    pub cdn_statuses: RwLock<Vec<CdnStatus>>,

    /// Auto-provisioning config
    pub auto_provision: AutoProvisionConfig,
}

/// CDN health status
#[derive(Debug, Clone)]
pub struct CdnStatus {
    pub id: String,
    pub name: String,
    pub port: u16,
    pub healthy: bool,
    pub latency_ms: Option<f64>,
    pub last_check: u64,
}

impl GatewayState {
    /// Create new gateway state with auto-provisioning
    pub fn new() -> Self {
        Self::with_config(AutoProvisionConfig::default())
    }

    /// Create gateway state with specific config
    pub fn with_config(config: AutoProvisionConfig) -> Self {
        // Initialize database statuses
        let db_statuses = vec![
            ConnectionStatus {
                db: Database::Supabase,
                connected: false,
                latency_ms: None,
                last_check: 0,
                error: None,
            },
            ConnectionStatus {
                db: Database::Neon,
                connected: false,
                latency_ms: None,
                last_check: 0,
                error: None,
            },
            ConnectionStatus {
                db: Database::Sled,
                connected: false,
                latency_ms: None,
                last_check: 0,
                error: None,
            },
            ConnectionStatus {
                db: Database::Sledis,
                connected: false,
                latency_ms: None,
                last_check: 0,
                error: None,
            },
            ConnectionStatus {
                db: Database::Nats,
                connected: false,
                latency_ms: None,
                last_check: 0,
                error: None,
            },
        ];

        // Initialize CDN statuses
        let cdn_statuses = CDN_CONFIGS
            .iter()
            .map(|cfg| CdnStatus {
                id: cfg.id.to_string(),
                name: cfg.name.to_string(),
                port: cfg.port,
                healthy: false,
                latency_ms: None,
                last_check: 0,
            })
            .collect();

        Self {
            supabase_url: RwLock::new(config.supabase_url.clone()),
            supabase_key: RwLock::new(config.supabase_key.clone()),
            neon_url: RwLock::new(config.neon_url.clone()),
            nats: RwLock::new(None),
            connection_statuses: RwLock::new(db_statuses),
            plasma_snapshot: RwLock::new(None),
            neural_mux: RwLock::new(NeuralMuxState::default()),
            cdn_statuses: RwLock::new(cdn_statuses),
            auto_provision: config,
        }
    }

    /// Connect to all backend services (auto-provision if configured)
    pub async fn connect_all(&self) -> Result<()> {
        tracing::info!("Auto-provisioning databases...");

        // Connect to Supabase
        self.connect_supabase().await?;

        // Connect to Neon
        self.connect_neon().await?;

        // Connect to NATS
        self.connect_nats().await?;

        // Initialize Neural Mux
        self.init_neural_mux().await;

        // Check CDN health
        self.check_cdn_health().await;

        Ok(())
    }

    /// Connect to Supabase (REST API)
    async fn connect_supabase(&self) -> Result<()> {
        let start = std::time::Instant::now();

        let url = self.supabase_url.read().await.clone();
        let key = self.supabase_key.read().await.clone();

        if let (Some(url), Some(key)) = (url, key) {
            // Test connection with health check
            let client = reqwest::Client::new();
            let health_url = format!("{}/rest/v1/", url);

            match client
                .get(&health_url)
                .header("apikey", &key)
                .header("Authorization", format!("Bearer {}", key))
                .send()
                .await
            {
                Ok(response) if response.status().is_success() || response.status().as_u16() == 400 => {
                    // 400 is OK - means API is responding (no table specified)
                    let latency = start.elapsed().as_secs_f64() * 1000.0;
                    self.update_connection_status(Database::Supabase, true, Some(latency), None)
                        .await;
                    tracing::info!("Connected to Supabase in {:.2}ms", latency);
                }
                Ok(response) => {
                    self.update_connection_status(
                        Database::Supabase,
                        false,
                        None,
                        Some(format!("HTTP {}", response.status())),
                    )
                    .await;
                    tracing::warn!("Supabase returned: {}", response.status());
                }
                Err(e) => {
                    self.update_connection_status(
                        Database::Supabase,
                        false,
                        None,
                        Some(e.to_string()),
                    )
                    .await;
                    tracing::warn!("Failed to connect to Supabase: {}", e);
                }
            }
        } else {
            self.update_connection_status(
                Database::Supabase,
                false,
                None,
                Some("SUPABASE_URL or SUPABASE_ANON_KEY not set".to_string()),
            )
            .await;
            tracing::info!("Supabase not configured (set SUPABASE_URL and SUPABASE_ANON_KEY)");
        }

        Ok(())
    }

    /// Connect to Neon PostgreSQL
    async fn connect_neon(&self) -> Result<()> {
        let start = std::time::Instant::now();

        let url = self.neon_url.read().await.clone();

        if let Some(url) = url {
            // For now, just validate the URL format
            // Full connection will use tokio-postgres or sqlx
            if url.starts_with("postgres://") || url.starts_with("postgresql://") {
                let latency = start.elapsed().as_secs_f64() * 1000.0;
                self.update_connection_status(Database::Neon, true, Some(latency), None)
                    .await;
                tracing::info!("Neon PostgreSQL configured in {:.2}ms", latency);
            } else {
                self.update_connection_status(
                    Database::Neon,
                    false,
                    None,
                    Some("Invalid connection string format".to_string()),
                )
                .await;
            }
        } else {
            self.update_connection_status(
                Database::Neon,
                false,
                None,
                Some("NEON_DATABASE_URL not set".to_string()),
            )
            .await;
            tracing::info!("Neon not configured (set NEON_DATABASE_URL)");
        }

        Ok(())
    }

    /// Connect to NATS
    async fn connect_nats(&self) -> Result<()> {
        let start = std::time::Instant::now();
        let url = format!("localhost:{}", ports::NATS);

        match async_nats::connect(&url).await {
            Ok(client) => {
                let latency = start.elapsed().as_secs_f64() * 1000.0;
                *self.nats.write().await = Some(client);

                self.update_connection_status(Database::Nats, true, Some(latency), None)
                    .await;
                tracing::info!("Connected to NATS at {} in {:.2}ms", url, latency);
            }
            Err(e) => {
                self.update_connection_status(Database::Nats, false, None, Some(e.to_string()))
                    .await;
                tracing::warn!("Failed to connect to NATS: {}", e);
            }
        }

        Ok(())
    }

    /// Initialize Neural Mux state
    async fn init_neural_mux(&self) {
        let mut mux = self.neural_mux.write().await;
        mux.tick_rate_us = 1000; // 1ms default tick rate
        mux.latency_ns = 0;
        mux.ticks_processed = 0;
        mux.l_star_enabled = true;
        mux.voice_enabled = false;
        tracing::info!(
            "Neural Mux initialized with {}us tick rate",
            mux.tick_rate_us
        );
    }

    /// Check CDN health
    async fn check_cdn_health(&self) {
        let mut statuses = self.cdn_statuses.write().await;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        for status in statuses.iter_mut() {
            // Try to connect to CDN port
            let start = std::time::Instant::now();
            match tokio::net::TcpStream::connect(format!("localhost:{}", status.port)).await {
                Ok(_) => {
                    status.healthy = true;
                    status.latency_ms = Some(start.elapsed().as_secs_f64() * 1000.0);
                    tracing::debug!("CDN {} healthy at port {}", status.id, status.port);
                }
                Err(_) => {
                    status.healthy = false;
                    status.latency_ms = None;
                }
            }
            status.last_check = now;
        }
    }

    /// Update connection status for a database
    async fn update_connection_status(
        &self,
        db: Database,
        connected: bool,
        latency_ms: Option<f64>,
        error: Option<String>,
    ) {
        let mut statuses = self.connection_statuses.write().await;
        if let Some(status) = statuses.iter_mut().find(|s| s.db == db) {
            status.connected = connected;
            status.latency_ms = latency_ms;
            status.last_check = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            status.error = error;
        }
    }

    /// Get all connection statuses
    pub async fn get_connection_statuses(&self) -> Vec<ConnectionStatus> {
        self.connection_statuses.read().await.clone()
    }

    /// Get CDN statuses
    pub async fn get_cdn_statuses(&self) -> Vec<CdnStatus> {
        self.cdn_statuses.read().await.clone()
    }

    /// Update plasma snapshot from sx9-atlas-bus
    pub async fn update_plasma(&self, snapshot: sx9_atlas_bus::PlasmaSnapshot) {
        let mut plasma = self.plasma_snapshot.write().await;
        *plasma = Some(snapshot);
    }

    /// Get current plasma snapshot
    pub async fn get_plasma(&self) -> Option<sx9_atlas_bus::PlasmaSnapshot> {
        self.plasma_snapshot.read().await.clone()
    }

    /// Get Neural Mux state
    pub async fn get_neural_mux(&self) -> NeuralMuxState {
        self.neural_mux.read().await.clone()
    }

    /// Update Neural Mux tick
    pub async fn tick_neural_mux(&self) {
        let mut mux = self.neural_mux.write().await;
        mux.ticks_processed += 1;
    }
}

impl Default for GatewayState {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared state wrapped in Arc for handler access
pub type SharedState = Arc<GatewayState>;
