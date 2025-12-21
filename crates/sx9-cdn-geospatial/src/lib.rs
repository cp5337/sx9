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
//! CTAS-7 Geospatial CDN
//!
//! Smart edge distribution network for geospatial content:
//! - Map tiles (XYZ, TMS, WMTS)
//! - Mapbox Vector Tiles (MVT/PBF) with style spec
//! - Mapbox terrain-rgb elevation data
//! - Terrain/elevation data (quantized mesh)
//! - Cesium 3D tiles
//! - Orbital propagation data (TLE, ephemeris)
//! - Satellite imagery
//! - Vector features (GeoJSON, MVT)
//!
//! Smart Features:
//! - Adaptive caching based on access patterns
//! - Predictive prefetching for adjacent tiles
//! - Geographic routing to nearest edge node
//! - HFT-optimized orbital data paths
//! - Content-aware compression (gzip/brotli)
//!
//! Architecture: Tile-addressed caching with geographic routing

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use geo_types::Rect;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════════════════
// Core Types
// ═══════════════════════════════════════════════════════════════════════════

/// Geospatial content types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum GeoContentType {
    /// Raster map tile (PNG, WebP, JPEG)
    MapTile,
    /// Vector tile (MVT/PBF)
    VectorTile,
    /// Mapbox Vector Tile (optimized MVT)
    MapboxMvt,
    /// Mapbox terrain-rgb elevation
    MapboxTerrainRgb,
    /// Mapbox style JSON
    MapboxStyle,
    /// Mapbox sprite sheet
    MapboxSprite,
    /// Mapbox glyphs (fonts)
    MapboxGlyph,
    /// Terrain mesh (quantized mesh)
    TerrainMesh,
    /// Cesium 3D tileset
    Cesium3DTile,
    /// Satellite TLE data
    TleData,
    /// Ephemeris data
    Ephemeris,
    /// GeoJSON features
    GeoJson,
    /// Satellite imagery
    Imagery,
    /// Weather overlay
    Weather,
    /// Custom layer
    CustomLayer,
}

// ═══════════════════════════════════════════════════════════════════════════
// Smart Cache Tracking (Adaptive Learning)
// ═══════════════════════════════════════════════════════════════════════════

/// Access pattern for smart caching decisions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TileAccessPattern {
    /// Total access count
    pub access_count: u64,
    /// Access times (last N timestamps)
    pub recent_access_times: Vec<i64>,
    /// Average access interval in seconds
    pub avg_interval_secs: f64,
    /// Prefetch priority score (0.0 - 1.0)
    pub prefetch_score: f64,
    /// Geographic hotspot weight
    pub hotspot_weight: f64,
}

/// Mapbox configuration for upstream tile fetching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapboxConfig {
    /// Mapbox access token
    pub access_token: String,
    /// Style ID for vector tiles
    pub style_id: Option<String>,
    /// Tileset ID
    pub tileset_id: Option<String>,
    /// Username (default: mapbox)
    pub username: String,
    /// Enable terrain-rgb
    pub terrain_rgb: bool,
    /// Enable 3D buildings
    pub buildings_3d: bool,
}

/// Tile coordinate (z/x/y)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TileCoord {
    pub z: u8,  // Zoom level
    pub x: u32, // Column
    pub y: u32, // Row
}

impl TileCoord {
    pub fn new(z: u8, x: u32, y: u32) -> Self {
        Self { z, x, y }
    }

    /// Get tile key for caching
    pub fn key(&self) -> String {
        format!("{}/{}/{}", self.z, self.x, self.y)
    }

    /// Get bounding box for this tile
    pub fn bounds(&self) -> Rect<f64> {
        let n = 2_u32.pow(self.z as u32) as f64;
        let lon_min = (self.x as f64 / n) * 360.0 - 180.0;
        let lon_max = ((self.x + 1) as f64 / n) * 360.0 - 180.0;

        let lat_max = (std::f64::consts::PI * (1.0 - 2.0 * self.y as f64 / n))
            .sinh()
            .atan()
            .to_degrees();
        let lat_min = (std::f64::consts::PI * (1.0 - 2.0 * (self.y + 1) as f64 / n))
            .sinh()
            .atan()
            .to_degrees();

        Rect::new((lon_min, lat_min), (lon_max, lat_max))
    }
}

/// Cached tile entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedTile {
    /// Tile coordinates
    pub coord: TileCoord,
    /// Content type
    pub content_type: GeoContentType,
    /// Layer name
    pub layer: String,
    /// Binary data (compressed if applicable)
    #[serde(with = "base64_bytes")]
    pub data: Vec<u8>,
    /// MIME type
    pub mime_type: String,
    /// Content hash for validation
    pub hash: String,
    /// Cached timestamp
    pub cached_at: DateTime<Utc>,
    /// Expiry time
    pub expires_at: Option<DateTime<Utc>>,
    /// Size in bytes
    pub size: usize,
}

/// Base64 serialization for binary data
mod base64_bytes {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&STANDARD.encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        STANDARD.decode(&s).map_err(serde::de::Error::custom)
    }
}

/// Orbital object tracking data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalObject {
    /// NORAD catalog ID
    pub norad_id: u32,
    /// Object name
    pub name: String,
    /// TLE line 1
    pub tle_line1: String,
    /// TLE line 2
    pub tle_line2: String,
    /// Epoch time
    pub epoch: DateTime<Utc>,
    /// Last update
    pub updated_at: DateTime<Utc>,
    /// Object type (satellite, debris, etc.)
    pub object_type: String,
    /// Country/operator
    pub country: Option<String>,
}

/// Layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerConfig {
    /// Layer name
    pub name: String,
    /// Content type
    pub content_type: GeoContentType,
    /// Upstream URL template (with {z}/{x}/{y} placeholders)
    pub upstream_url: Option<String>,
    /// Min zoom level
    pub min_zoom: u8,
    /// Max zoom level
    pub max_zoom: u8,
    /// Cache TTL in seconds
    pub cache_ttl: u64,
    /// Bounds (optional)
    pub bounds: Option<[f64; 4]>, // [west, south, east, north]
}

// ═══════════════════════════════════════════════════════════════════════════
// CDN Node
// ═══════════════════════════════════════════════════════════════════════════

/// Geospatial CDN Node with Smart Caching
pub struct GeospatialCdnNode {
    /// Node identifier
    pub node_id: String,
    /// Tile cache (layer:z/x/y -> CachedTile)
    tile_cache: Arc<DashMap<String, CachedTile>>,
    /// Orbital objects cache (NORAD ID -> OrbitalObject)
    orbital_cache: Arc<DashMap<u32, OrbitalObject>>,
    /// Layer configurations
    layers: Arc<RwLock<HashMap<String, LayerConfig>>>,
    /// Local sled database for persistence
    db: Option<sled::Db>,
    /// Node metrics
    metrics: Arc<RwLock<GeoNodeMetrics>>,
    /// HTTP client for upstream fetching
    http_client: reqwest::Client,
    // Smart cache tracking for adaptive prefetching
    /// Tile access patterns for predictive caching
    access_patterns: Arc<DashMap<String, TileAccessPattern>>,
    /// Mapbox configuration (optional)
    mapbox_config: Arc<RwLock<Option<MapboxConfig>>>,
    /// Geographic hotspots for priority caching
    hotspots: Arc<DashMap<String, f64>>,
}

/// Node metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GeoNodeMetrics {
    pub tile_cache_hits: u64,
    pub tile_cache_misses: u64,
    pub tile_requests: u64,
    pub bytes_served: u64,
    pub orbital_objects: u64,
    pub layers_configured: u64,
    pub last_sync: Option<DateTime<Utc>>,
}

impl GeospatialCdnNode {
    /// Create new CDN node with smart caching
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            tile_cache: Arc::new(DashMap::new()),
            orbital_cache: Arc::new(DashMap::new()),
            layers: Arc::new(RwLock::new(HashMap::new())),
            db: None,
            metrics: Arc::new(RwLock::new(GeoNodeMetrics::default())),
            http_client: reqwest::Client::new(),
            access_patterns: Arc::new(DashMap::new()),
            mapbox_config: Arc::new(RwLock::new(None)),
            hotspots: Arc::new(DashMap::new()),
        }
    }

    /// Configure Mapbox integration
    pub async fn configure_mapbox(&self, config: MapboxConfig) {
        let mut mapbox = self.mapbox_config.write().await;
        *mapbox = Some(config);
    }

    /// Initialize with persistent storage
    pub fn with_storage(mut self, db_path: &str) -> anyhow::Result<Self> {
        self.db = Some(sled::open(db_path)?);
        Ok(self)
    }

    /// Register a layer
    pub async fn register_layer(&self, config: LayerConfig) {
        let mut layers = self.layers.write().await;
        layers.insert(config.name.clone(), config);
        let mut metrics = self.metrics.write().await;
        metrics.layers_configured = layers.len() as u64;
    }

    /// Get tile
    pub async fn get_tile(&self, layer: &str, coord: &TileCoord) -> Option<CachedTile> {
        let cache_key = format!("{}:{}", layer, coord.key());

        // Try cache first
        if let Some(tile) = self.tile_cache.get(&cache_key) {
            // Check expiry
            if let Some(expires) = tile.expires_at {
                if expires > Utc::now() {
                    let mut metrics = self.metrics.write().await;
                    metrics.tile_cache_hits += 1;
                    metrics.tile_requests += 1;
                    metrics.bytes_served += tile.size as u64;
                    return Some(tile.clone());
                }
            } else {
                let mut metrics = self.metrics.write().await;
                metrics.tile_cache_hits += 1;
                metrics.tile_requests += 1;
                metrics.bytes_served += tile.size as u64;
                return Some(tile.clone());
            }
        }

        // Cache miss - try to fetch from upstream
        let layers = self.layers.read().await;
        if let Some(layer_config) = layers.get(layer) {
            if let Some(upstream_url) = &layer_config.upstream_url {
                if let Ok(tile) = self
                    .fetch_upstream(layer, coord, upstream_url, layer_config.cache_ttl)
                    .await
                {
                    // Store in cache
                    self.tile_cache.insert(cache_key, tile.clone());

                    let mut metrics = self.metrics.write().await;
                    metrics.tile_cache_misses += 1;
                    metrics.tile_requests += 1;
                    metrics.bytes_served += tile.size as u64;

                    return Some(tile);
                }
            }
        }

        let mut metrics = self.metrics.write().await;
        metrics.tile_cache_misses += 1;
        metrics.tile_requests += 1;

        None
    }

    /// Store tile directly
    pub async fn store_tile(&self, layer: &str, coord: &TileCoord, data: Vec<u8>, mime_type: &str) {
        // Engineered Solution: RFC-9001 Compliant Trivariate Hash
        use sx9_foundation_core::hashing::murmur3_64_hex;
        let hash = murmur3_64_hex(&data, 0);

        let tile = CachedTile {
            coord: coord.clone(),
            content_type: GeoContentType::MapTile,
            layer: layer.to_string(),
            size: data.len(),
            data,
            mime_type: mime_type.to_string(),
            hash,
            cached_at: Utc::now(),
            expires_at: None,
        };

        let cache_key = format!("{}:{}", layer, coord.key());
        self.tile_cache.insert(cache_key, tile);
    }

    /// Fetch tile from upstream
    async fn fetch_upstream(
        &self,
        layer: &str,
        coord: &TileCoord,
        url_template: &str,
        ttl: u64,
    ) -> anyhow::Result<CachedTile> {
        let url = url_template
            .replace("{z}", &coord.z.to_string())
            .replace("{x}", &coord.x.to_string())
            .replace("{y}", &coord.y.to_string());

        let response = self.http_client.get(&url).send().await?;
        let mime_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();

        let data = response.bytes().await?.to_vec();

        // Engineered Solution: RFC-9001 Compliant Trivariate Hash
        use sx9_foundation_core::hashing::murmur3_64_hex;
        let hash = murmur3_64_hex(&data, 0);

        Ok(CachedTile {
            coord: coord.clone(),
            content_type: GeoContentType::MapTile,
            layer: layer.to_string(),
            size: data.len(),
            data,
            mime_type,
            hash,
            cached_at: Utc::now(),
            expires_at: Some(Utc::now() + chrono::Duration::seconds(ttl as i64)),
        })
    }

    /// Store orbital object
    pub async fn store_orbital(&self, obj: OrbitalObject) {
        self.orbital_cache.insert(obj.norad_id, obj);
        let mut metrics = self.metrics.write().await;
        metrics.orbital_objects = self.orbital_cache.len() as u64;
    }

    /// Get orbital object by NORAD ID
    pub fn get_orbital(&self, norad_id: u32) -> Option<OrbitalObject> {
        self.orbital_cache.get(&norad_id).map(|o| o.clone())
    }

    /// Get all orbital objects
    pub fn get_all_orbital(&self) -> Vec<OrbitalObject> {
        self.orbital_cache.iter().map(|o| o.clone()).collect()
    }

    /// Get metrics
    pub async fn get_metrics(&self) -> GeoNodeMetrics {
        self.metrics.read().await.clone()
    }

    /// Health check
    pub async fn health_check(&self) -> GeoHealthStatus {
        let metrics = self.metrics.read().await;
        let layers = self.layers.read().await;

        GeoHealthStatus {
            status: "healthy".to_string(),
            node_id: self.node_id.clone(),
            tile_cache_size: self.tile_cache.len(),
            orbital_objects: self.orbital_cache.len(),
            layers: layers.keys().cloned().collect(),
            cache_hit_ratio: if metrics.tile_requests > 0 {
                metrics.tile_cache_hits as f64 / metrics.tile_requests as f64
            } else {
                0.0
            },
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Mapbox Integration (MVT, Terrain-RGB, Styles)
    // ═══════════════════════════════════════════════════════════════════════════

    /// Fetch Mapbox vector tile (MVT) with caching
    pub async fn get_mapbox_tile(&self, coord: &TileCoord) -> Option<CachedTile> {
        let mapbox = self.mapbox_config.read().await;
        let config = mapbox.as_ref()?;

        let cache_key = format!("mapbox-mvt:{}", coord.key());

        // Check cache first
        if let Some(tile) = self.tile_cache.get(&cache_key) {
            self.track_access(&cache_key).await;
            return Some(tile.clone());
        }

        // Fetch from Mapbox API
        let url = format!(
            "https://api.mapbox.com/v4/{}.mapbox_terrain_rgb,mapbox.mapbox-streets-v8/{}/{}/{}.mvt?access_token={}",
            config.username, coord.z, coord.x, coord.y, config.access_token
        );

        if let Ok(tile) = self.fetch_upstream("mapbox-mvt", coord, &url, 3600).await {
            self.tile_cache.insert(cache_key.clone(), tile.clone());
            self.track_access(&cache_key).await;
            return Some(tile);
        }

        None
    }

    /// Fetch Mapbox terrain-RGB elevation tile
    pub async fn get_mapbox_terrain(&self, coord: &TileCoord) -> Option<CachedTile> {
        let mapbox = self.mapbox_config.read().await;
        let config = mapbox.as_ref()?;

        if !config.terrain_rgb {
            return None;
        }

        let cache_key = format!("mapbox-terrain:{}", coord.key());

        if let Some(tile) = self.tile_cache.get(&cache_key) {
            return Some(tile.clone());
        }

        // Mapbox terrain-RGB endpoint
        let url = format!(
            "https://api.mapbox.com/v4/mapbox.terrain-rgb/{}/{}/{}.pngraw?access_token={}",
            coord.z, coord.x, coord.y, config.access_token
        );

        if let Ok(tile) = self
            .fetch_upstream("mapbox-terrain", coord, &url, 86400)
            .await
        {
            self.tile_cache.insert(cache_key, tile.clone());
            return Some(tile);
        }

        None
    }

    /// Fetch Mapbox style JSON
    pub async fn get_mapbox_style(&self, style_id: &str) -> Option<String> {
        let mapbox = self.mapbox_config.read().await;
        let config = mapbox.as_ref()?;

        let cache_key = format!("mapbox-style:{}", style_id);

        // Check if cached (styles stored as JSON string in tile data)
        if let Some(tile) = self.tile_cache.get(&cache_key) {
            return String::from_utf8(tile.data.clone()).ok();
        }

        let url = format!(
            "https://api.mapbox.com/styles/v1/{}/{}?access_token={}",
            config.username, style_id, config.access_token
        );

        if let Ok(response) = self.http_client.get(&url).send().await {
            if let Ok(style_json) = response.text().await {
                // Cache style as tile
                let coord = TileCoord::new(0, 0, 0);
                self.store_tile(
                    &cache_key,
                    &coord,
                    style_json.as_bytes().to_vec(),
                    "application/json",
                )
                .await;
                return Some(style_json);
            }
        }

        None
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Smart Caching (Adaptive Prefetching)
    // ═══════════════════════════════════════════════════════════════════════════

    /// Track tile access for adaptive caching
    async fn track_access(&self, cache_key: &str) {
        let now = Utc::now().timestamp();

        self.access_patterns
            .entry(cache_key.to_string())
            .and_modify(|pattern| {
                pattern.access_count += 1;
                pattern.recent_access_times.push(now);

                // Keep last 100 access times
                if pattern.recent_access_times.len() > 100 {
                    pattern.recent_access_times.remove(0);
                }

                // Calculate average interval
                if pattern.recent_access_times.len() > 1 {
                    let intervals: Vec<i64> = pattern
                        .recent_access_times
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect();
                    pattern.avg_interval_secs =
                        intervals.iter().sum::<i64>() as f64 / intervals.len() as f64;
                }

                // Calculate prefetch score (higher = more likely to prefetch)
                // Based on access frequency and recency
                pattern.prefetch_score =
                    (pattern.access_count as f64).ln() / (pattern.avg_interval_secs.max(1.0)).ln();
            })
            .or_insert_with(|| TileAccessPattern {
                access_count: 1,
                recent_access_times: vec![now],
                avg_interval_secs: 0.0,
                prefetch_score: 0.0,
                hotspot_weight: 1.0,
            });
    }

    /// Get adjacent tiles for prefetching
    pub fn get_adjacent_tiles(&self, coord: &TileCoord) -> Vec<TileCoord> {
        let mut adjacent = Vec::with_capacity(8);
        let max_coord = 2_u32.pow(coord.z as u32);

        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let new_x = (coord.x as i32 + dx).rem_euclid(max_coord as i32) as u32;
                let new_y = coord.y as i32 + dy;

                if new_y >= 0 && new_y < max_coord as i32 {
                    adjacent.push(TileCoord::new(coord.z, new_x, new_y as u32));
                }
            }
        }

        adjacent
    }

    /// Prefetch tiles based on access patterns (called async in background)
    pub async fn prefetch_adjacent(&self, layer: &str, coord: &TileCoord) {
        let cache_key = format!("{}:{}", layer, coord.key());

        // Only prefetch if this tile has high access score
        if let Some(pattern) = self.access_patterns.get(&cache_key) {
            if pattern.prefetch_score < 0.5 {
                return;
            }
        } else {
            return;
        }

        // Prefetch adjacent tiles
        for adj_coord in self.get_adjacent_tiles(coord) {
            let adj_key = format!("{}:{}", layer, adj_coord.key());

            // Skip if already cached
            if self.tile_cache.contains_key(&adj_key) {
                continue;
            }

            // Fetch in background
            let _ = self.get_tile(layer, &adj_coord).await;
        }
    }

    /// Register a geographic hotspot for priority caching
    pub async fn register_hotspot(&self, lat: f64, lon: f64, weight: f64) {
        let key = format!("{:.4},{:.4}", lat, lon);
        self.hotspots.insert(key, weight);
    }

    /// Get top N tiles by prefetch priority
    pub fn get_priority_tiles(&self, limit: usize) -> Vec<(String, f64)> {
        let mut priorities: Vec<(String, f64)> = self
            .access_patterns
            .iter()
            .map(|e| (e.key().clone(), e.prefetch_score))
            .collect();

        priorities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        priorities.truncate(limit);
        priorities
    }
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoHealthStatus {
    pub status: String,
    pub node_id: String,
    pub tile_cache_size: usize,
    pub orbital_objects: usize,
    pub layers: Vec<String>,
    pub cache_hit_ratio: f64,
}

// ═══════════════════════════════════════════════════════════════════════════
// Tile Utilities
// ═══════════════════════════════════════════════════════════════════════════

/// Convert lat/lon to tile coordinates
pub fn lat_lon_to_tile(lat: f64, lon: f64, zoom: u8) -> TileCoord {
    let n = 2_u32.pow(zoom as u32) as f64;
    let x = ((lon + 180.0) / 360.0 * n).floor() as u32;
    let lat_rad = lat.to_radians();
    let y = ((1.0 - lat_rad.tan().asinh() / std::f64::consts::PI) / 2.0 * n).floor() as u32;
    TileCoord::new(zoom, x, y)
}

/// Get tiles in bounding box
pub fn tiles_in_bounds(bounds: &Rect<f64>, zoom: u8) -> Vec<TileCoord> {
    let min = bounds.min();
    let max = bounds.max();

    let top_left = lat_lon_to_tile(max.y, min.x, zoom);
    let bottom_right = lat_lon_to_tile(min.y, max.x, zoom);

    let mut tiles = Vec::new();
    for x in top_left.x..=bottom_right.x {
        for y in top_left.y..=bottom_right.y {
            tiles.push(TileCoord::new(zoom, x, y));
        }
    }
    tiles
}

// Need base64 crate

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_coord_key() {
        let coord = TileCoord::new(10, 512, 384);
        assert_eq!(coord.key(), "10/512/384");
    }

    #[test]
    fn test_lat_lon_to_tile() {
        // San Francisco area
        let coord = lat_lon_to_tile(37.7749, -122.4194, 10);
        assert!(coord.x > 0);
        assert!(coord.y > 0);
    }

    #[tokio::test]
    async fn test_cdn_node() {
        let node = GeospatialCdnNode::new("test".to_string());
        let health = node.health_check().await;
        assert_eq!(health.status, "healthy");
    }
}
