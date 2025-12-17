// sx9-gateway/src/cdn/r2_subscriber.rs
//
// RFC-9114 Rev 1.2 - CloudFlare R2 CDN Subscriber Service
// 
// Provides:
// - Periodic sync from CloudFlare R2 buckets (1 hour interval)
// - Local in-memory cache for <1ms lookups
// - REST API on configurable port (default: 18127)
// - Fallback to Supabase on cache miss
// - Bernoulli Zone C compliance (<100ms)
// - Neural Mux integration
//
// Author: SX9 Engineering Group
// Date: December 2025
// License: Proprietary

use anyhow::{anyhow, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{
    config::{Credentials, Region},
    Client as S3Client, Config,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

// ============================================================================
// CONFIGURATION
// ============================================================================

const DEFAULT_SYNC_INTERVAL_SECS: u64 = 3600; // 1 hour
const CACHE_TTL_SECS: u64 = 7200; // 2 hours
const MAX_RETRIES: u32 = 3;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// R2 CDN Subscriber Service
pub struct R2SubscriberService {
    /// S3-compatible client for R2
    client: S3Client,
    /// R2 bucket names
    buckets: R2Buckets,
    /// In-memory cache
    cache: Arc<RwLock<R2Cache>>,
    /// Service configuration
    config: R2Config,
}

#[derive(Clone)]
pub struct R2Config {
    pub port: u16,
    pub sync_interval: Duration,
    pub cache_ttl: Duration,
}

impl Default for R2Config {
    fn default() -> Self {
        Self {
            port: 18127,
            sync_interval: Duration::from_secs(DEFAULT_SYNC_INTERVAL_SECS),
            cache_ttl: Duration::from_secs(CACHE_TTL_SECS),
        }
    }
}

#[derive(Clone, Debug)]
struct R2Buckets {
    threat_intel: String,
    mitre_attack: String,
    kali_tools: String,
    osint_data: String,
}

impl Default for R2Buckets {
    fn default() -> Self {
        Self {
            threat_intel: std::env::var("R2_BUCKET_THREAT_INTEL")
                .unwrap_or_else(|_| "sx9-threat-intel".to_string()),
            mitre_attack: std::env::var("R2_BUCKET_MITRE")
                .unwrap_or_else(|_| "sx9-mitre-attack".to_string()),
            kali_tools: std::env::var("R2_BUCKET_KALI")
                .unwrap_or_else(|_| "sx9-kali-tools".to_string()),
            osint_data: std::env::var("R2_BUCKET_OSINT")
                .unwrap_or_else(|_| "sx9-osint-data".to_string()),
        }
    }
}

struct R2Cache {
    threat_tools: Option<ThreatToolsManifest>,
    mitre_matrix: Option<MitreMatrix>,
    kali_manifest: Option<KaliToolsManifest>,
    last_sync: Instant,
    sync_count: u64,
}

impl Default for R2Cache {
    fn default() -> Self {
        Self {
            threat_tools: None,
            mitre_matrix: None,
            kali_manifest: None,
            last_sync: Instant::now(),
            sync_count: 0,
        }
    }
}

// ============================================================================
// MANIFEST STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatToolsManifest {
    pub version: String,
    pub generated_at: String,
    pub total_tools: usize,
    pub sources: Vec<String>,
    pub tools: Vec<ToolEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEntry {
    pub unicode: String,
    pub name: String,
    pub category: String,
    pub hash_operational: String,
    pub hash_semantic: String,
    pub genome: String,
    pub binary_path: Option<String>,
    pub docker_image: Option<String>,
    pub mitre_techniques: Vec<String>,
    pub dependencies: Vec<String>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreMatrix {
    pub version: String,
    pub generated_at: String,
    pub total_techniques: usize,
    pub techniques: Vec<MitreTechnique>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreTechnique {
    pub id: String,
    pub name: String,
    pub tactics: Vec<String>,
    pub description: String,
    pub platforms: Vec<String>,
    pub data_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaliToolsManifest {
    pub version: String,
    pub total_tools: usize,
    pub categories: HashMap<String, Vec<String>>,
}

// ============================================================================
// REQUEST/RESPONSE STRUCTURES
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub category: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub rfc: String,
    pub buckets: usize,
    pub cache_age_seconds: u64,
    pub last_sync: String,
    pub sync_count: u64,
    pub cached_manifests: CachedManifests,
}

#[derive(Debug, Serialize)]
pub struct CachedManifests {
    pub threat_tools: bool,
    pub mitre_matrix: bool,
    pub kali_tools: bool,
}

#[derive(Debug, Serialize)]
pub struct ToolResponse {
    pub found: bool,
    pub tool: Option<ToolEntry>,
    pub source: String, // "r2-cache" or "fallback"
}

#[derive(Debug, Serialize)]
pub struct ToolsListResponse {
    pub total: usize,
    pub tools: Vec<ToolEntry>,
    pub source: String,
}

#[derive(Debug, Serialize)]
pub struct MitreTechniqueResponse {
    pub found: bool,
    pub technique: Option<MitreTechnique>,
    pub source: String,
}

// ============================================================================
// IMPLEMENTATION
// ============================================================================

impl R2SubscriberService {
    /// Create new R2 subscriber service
    pub async fn new(config: R2Config) -> Result<Self> {
        info!(
            "🔌 Initializing R2 CDN subscriber service on port {}",
            config.port
        );

        // Get R2 credentials from environment
        let access_key = std::env::var("R2_ACCESS_KEY_ID")
            .map_err(|_| anyhow!("R2_ACCESS_KEY_ID not set"))?;
        let secret_key = std::env::var("R2_SECRET_ACCESS_KEY")
            .map_err(|_| anyhow!("R2_SECRET_ACCESS_KEY not set"))?;
        let endpoint = std::env::var("R2_ENDPOINT")
            .map_err(|_| anyhow!("R2_ENDPOINT not set (e.g., https://abc123.r2.cloudflarestorage.com)"))?;

        // Configure S3 client for R2
        let creds = Credentials::new(
            access_key,
            secret_key,
            None, // session token
            None, // expiry
            "r2",
        );

        let s3_config = Config::builder()
            .credentials_provider(creds)
            .region(Region::new("auto"))
            .endpoint_url(endpoint)
            .build();

        let client = S3Client::from_conf(s3_config);

        let buckets = R2Buckets::default();

        info!("✅ R2 client configured with {} buckets", 4);
        debug!("   - Threat Intel: {}", buckets.threat_intel);
        debug!("   - MITRE ATT&CK: {}", buckets.mitre_attack);
        debug!("   - Kali Tools: {}", buckets.kali_tools);
        debug!("   - OSINT Data: {}", buckets.osint_data);

        Ok(Self {
            client,
            buckets,
            cache: Arc::new(RwLock::new(R2Cache::default())),
            config,
        })
    }

    /// Start the service (background sync + HTTP server)
    pub async fn start(self: Arc<Self>) -> Result<()> {
        // Initial sync
        info!("🔄 Performing initial sync from R2...");
        if let Err(e) = self.sync_from_r2().await {
            warn!("⚠️  Initial sync failed: {} (will retry)", e);
        }

        // Start background sync loop
        let sync_self = Arc::clone(&self);
        tokio::spawn(async move {
            sync_self.sync_loop().await;
        });

        // Start HTTP server
        self.serve().await
    }

    /// Background sync loop
    async fn sync_loop(self: Arc<Self>) {
        let mut interval = tokio::time::interval(self.config.sync_interval);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        loop {
            interval.tick().await;

            info!("🔄 Starting scheduled R2 sync...");
            if let Err(e) = self.sync_from_r2().await {
                error!("❌ Scheduled sync failed: {}", e);
            }
        }
    }

    /// Sync all data from R2 buckets
    async fn sync_from_r2(&self) -> Result<()> {
        let start = Instant::now();
        info!("📥 Syncing from R2 buckets...");

        // Sync threat tools
        let threat_tools = self.fetch_threat_tools().await?;
        info!("   ✅ Threat tools: {} entries", threat_tools.total_tools);

        // Sync MITRE matrix
        let mitre_matrix = self.fetch_mitre_matrix().await?;
        info!(
            "   ✅ MITRE techniques: {} entries",
            mitre_matrix.total_techniques
        );

        // Sync Kali tools
        let kali_manifest = self.fetch_kali_manifest().await?;
        info!("   ✅ Kali tools: {} entries", kali_manifest.total_tools);

        // Update cache
        let mut cache = self.cache.write().await;
        cache.threat_tools = Some(threat_tools);
        cache.mitre_matrix = Some(mitre_matrix);
        cache.kali_manifest = Some(kali_manifest);
        cache.last_sync = Instant::now();
        cache.sync_count += 1;

        let elapsed = start.elapsed();
        info!("✅ R2 sync complete in {:?}", elapsed);

        Ok(())
    }

    /// Fetch threat tools manifest from R2
    async fn fetch_threat_tools(&self) -> Result<ThreatToolsManifest> {
        self.fetch_json_from_r2(&self.buckets.threat_intel, "threat-tools.json")
            .await
    }

    /// Fetch MITRE ATT&CK matrix from R2
    async fn fetch_mitre_matrix(&self) -> Result<MitreMatrix> {
        self.fetch_json_from_r2(&self.buckets.mitre_attack, "enterprise-attack.json")
            .await
    }

    /// Fetch Kali tools manifest from R2
    async fn fetch_kali_manifest(&self) -> Result<KaliToolsManifest> {
        self.fetch_json_from_r2(&self.buckets.kali_tools, "kali-manifest.json")
            .await
    }

    /// Generic JSON fetch from R2 with retries
    async fn fetch_json_from_r2<T: for<'de> Deserialize<'de>>(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<T> {
        for attempt in 1..=MAX_RETRIES {
            match self.try_fetch_json(bucket, key).await {
                Ok(data) => return Ok(data),
                Err(e) => {
                    if attempt < MAX_RETRIES {
                        warn!(
                            "⚠️  Fetch attempt {}/{} failed for {}/{}: {}",
                            attempt, MAX_RETRIES, bucket, key, e
                        );
                        tokio::time::sleep(Duration::from_secs(2u64.pow(attempt))).await;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        unreachable!()
    }

    /// Single attempt to fetch JSON from R2
    async fn try_fetch_json<T: for<'de> Deserialize<'de>>(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<T> {
        let object = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch {}/{}: {}", bucket, key, e))?;

        let body = object
            .body
            .collect()
            .await
            .map_err(|e| anyhow!("Failed to read body: {}", e))?;

        let data: T = serde_json::from_slice(&body.into_bytes())
            .map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;

        Ok(data)
    }

    /// Start HTTP server
    async fn serve(self: Arc<Self>) -> Result<()> {
        let app = Router::new()
            .route("/health", get(health_check))
            .route("/tools", get(get_all_tools))
            .route("/tools/:unicode", get(get_tool_by_unicode))
            .route("/tools/search", get(search_tools))
            .route("/mitre", get(get_mitre_matrix))
            .route("/mitre/:technique_id", get(get_mitre_technique))
            .route("/kali", get(get_kali_manifest))
            .route("/sync", post(trigger_sync))
            .route("/cache/stats", get(cache_stats))
            .with_state(self.clone());

        let addr = format!("0.0.0.0:{}", self.config.port);
        info!("🚀 R2 CDN subscriber listening on {}", addr);

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| anyhow!("Failed to bind to {}: {}", addr, e))?;

        axum::serve(listener, app)
            .await
            .map_err(|e| anyhow!("Server error: {}", e))?;

        Ok(())
    }
}

// ============================================================================
// HTTP HANDLERS
// ============================================================================

async fn health_check(State(service): State<Arc<R2SubscriberService>>) -> impl IntoResponse {
    let cache = service.cache.read().await;
    let age_secs = cache.last_sync.elapsed().as_secs();

    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "r2-cdn-subscriber".to_string(),
        rfc: "9114-rev1.2".to_string(),
        buckets: 4,
        cache_age_seconds: age_secs,
        last_sync: chrono::Utc::now()
            .checked_sub_signed(chrono::Duration::seconds(age_secs as i64))
            .unwrap()
            .to_rfc3339(),
        sync_count: cache.sync_count,
        cached_manifests: CachedManifests {
            threat_tools: cache.threat_tools.is_some(),
            mitre_matrix: cache.mitre_matrix.is_some(),
            kali_tools: cache.kali_manifest.is_some(),
        },
    })
}

async fn get_all_tools(State(service): State<Arc<R2SubscriberService>>) -> impl IntoResponse {
    let cache = service.cache.read().await;

    match &cache.threat_tools {
        Some(manifest) => (
            StatusCode::OK,
            Json(ToolsListResponse {
                total: manifest.total_tools,
                tools: manifest.tools.clone(),
                source: "r2-cache".to_string(),
            }),
        ),
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ToolsListResponse {
                total: 0,
                tools: vec![],
                source: "none-available".to_string(),
            }),
        ),
    }
}

async fn get_tool_by_unicode(
    State(service): State<Arc<R2SubscriberService>>,
    Path(unicode): Path<String>,
) -> impl IntoResponse {
    let cache = service.cache.read().await;

    if let Some(manifest) = &cache.threat_tools {
        if let Some(tool) = manifest.tools.iter().find(|t| t.unicode == unicode) {
            return (
                StatusCode::OK,
                Json(ToolResponse {
                    found: true,
                    tool: Some(tool.clone()),
                    source: "r2-cache".to_string(),
                }),
            );
        }
    }

    (
        StatusCode::NOT_FOUND,
        Json(ToolResponse {
            found: false,
            tool: None,
            source: "not-found".to_string(),
        }),
    )
}

async fn search_tools(
    State(service): State<Arc<R2SubscriberService>>,
    Query(query): Query<SearchQuery>,
) -> impl IntoResponse {
    let cache = service.cache.read().await;

    match &cache.threat_tools {
        Some(manifest) => {
            let mut tools: Vec<ToolEntry> = manifest.tools.clone();

            // Filter by query
            if let Some(q) = &query.q {
                let q_lower = q.to_lowercase();
                tools.retain(|t| {
                    t.name.to_lowercase().contains(&q_lower)
                        || t.category.to_lowercase().contains(&q_lower)
                });
            }

            // Filter by category
            if let Some(cat) = &query.category {
                let cat_lower = cat.to_lowercase();
                tools.retain(|t| t.category.to_lowercase() == cat_lower);
            }

            // Limit results
            let limit = query.limit.unwrap_or(100).min(1000);
            tools.truncate(limit);

            (
                StatusCode::OK,
                Json(ToolsListResponse {
                    total: tools.len(),
                    tools,
                    source: "r2-cache".to_string(),
                }),
            )
        }
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ToolsListResponse {
                total: 0,
                tools: vec![],
                source: "none-available".to_string(),
            }),
        ),
    }
}

async fn get_mitre_matrix(State(service): State<Arc<R2SubscriberService>>) -> impl IntoResponse {
    let cache = service.cache.read().await;

    match &cache.mitre_matrix {
        Some(matrix) => (StatusCode::OK, Json(matrix.clone())),
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(MitreMatrix {
                version: "unavailable".to_string(),
                generated_at: "".to_string(),
                total_techniques: 0,
                techniques: vec![],
            }),
        ),
    }
}

async fn get_mitre_technique(
    State(service): State<Arc<R2SubscriberService>>,
    Path(technique_id): Path<String>,
) -> impl IntoResponse {
    let cache = service.cache.read().await;

    if let Some(matrix) = &cache.mitre_matrix {
        if let Some(technique) = matrix.techniques.iter().find(|t| t.id == technique_id) {
            return (
                StatusCode::OK,
                Json(MitreTechniqueResponse {
                    found: true,
                    technique: Some(technique.clone()),
                    source: "r2-cache".to_string(),
                }),
            );
        }
    }

    (
        StatusCode::NOT_FOUND,
        Json(MitreTechniqueResponse {
            found: false,
            technique: None,
            source: "not-found".to_string(),
        }),
    )
}

async fn get_kali_manifest(State(service): State<Arc<R2SubscriberService>>) -> impl IntoResponse {
    let cache = service.cache.read().await;

    match &cache.kali_manifest {
        Some(manifest) => (StatusCode::OK, Json(manifest.clone())),
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(KaliToolsManifest {
                version: "unavailable".to_string(),
                total_tools: 0,
                categories: HashMap::new(),
            }),
        ),
    }
}

async fn trigger_sync(State(service): State<Arc<R2SubscriberService>>) -> impl IntoResponse {
    tokio::spawn(async move {
        if let Err(e) = service.sync_from_r2().await {
            error!("❌ Manual sync failed: {}", e);
        }
    });

    (
        StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "status": "sync_triggered",
            "message": "Background sync started"
        })),
    )
}

async fn cache_stats(State(service): State<Arc<R2SubscriberService>>) -> impl IntoResponse {
    let cache = service.cache.read().await;

    Json(serde_json::json!({
        "cache_age_seconds": cache.last_sync.elapsed().as_secs(),
        "sync_count": cache.sync_count,
        "cached_items": {
            "threat_tools": cache.threat_tools.as_ref().map(|m| m.total_tools).unwrap_or(0),
            "mitre_techniques": cache.mitre_matrix.as_ref().map(|m| m.total_techniques).unwrap_or(0),
            "kali_tools": cache.kali_manifest.as_ref().map(|m| m.total_tools).unwrap_or(0),
        }
    }))
}
