//! Ground Station Registry
//!
//! 257 LaserLight FSO ground stations for satellite network.
//! Seed data from TeleGeography submarine cable landings + GEE atmospheric analysis.
//!
//! Architecture:
//! - Each ground station → cloned to WASM sensor
//! - WASM sensors run HFT route optimization (scaled packets)
//! - Real server pings for latency measurement
//! - ANN for 12 satellite beam routing
//!
//! Data Source: Cable landing locations from submarinecablemap.com
//! Note: Original data had lat/lon inversion - use `fix_coordinate_inversion()` to correct.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ground station with FSO characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundStation {
    /// Unique station ID (e.g., "GS-001-NYC")
    pub station_id: String,

    /// Human-readable name
    pub name: String,

    /// Geographic coordinates
    pub latitude: f64,
    pub longitude: f64,
    pub elevation_m: f64,

    /// Location metadata
    pub country: String,
    pub region: String,
    pub city: String,

    /// Cable landing association
    pub cable_landing_id: Option<String>,
    pub cable_names: Vec<String>,

    /// FSO characteristics (from GEE analysis)
    pub fso_suitability_score: f64,  // 0-100
    pub clear_sky_days_per_year: u16,
    pub avg_cloud_cover_pct: f64,
    pub avg_water_vapor_mm: f64,

    /// Network properties
    pub cluster_id: u8,  // 0-9 clusters for WASM mapping
    pub trivariate_hash: Option<String>,

    /// Operational status
    pub operational: bool,
    pub wasm_sensor_deployed: bool,
    pub last_ping_ms: Option<u32>,
    pub last_updated: DateTime<Utc>,
}

impl GroundStation {
    /// Create from cable landing with lat/lon (properly ordered)
    pub fn from_cable_landing(
        id: &str,
        name: &str,
        latitude: f64,
        longitude: f64,
        country: &str,
        cable_names: Vec<String>,
    ) -> Self {
        Self {
            station_id: id.to_string(),
            name: name.to_string(),
            latitude,
            longitude,
            elevation_m: 0.0,
            country: country.to_string(),
            region: String::new(),
            city: name.to_string(),
            cable_landing_id: Some(id.to_string()),
            cable_names,
            fso_suitability_score: 0.0,
            clear_sky_days_per_year: 0,
            avg_cloud_cover_pct: 0.0,
            avg_water_vapor_mm: 0.0,
            cluster_id: 0,
            trivariate_hash: None,
            operational: false,
            wasm_sensor_deployed: false,
            last_ping_ms: None,
            last_updated: Utc::now(),
        }
    }

    /// Fix lat/lon inversion (swap if inverted)
    pub fn fix_coordinate_inversion(&mut self) {
        // Detect inversion: latitude should be -90 to 90, longitude -180 to 180
        // If lat is outside -90..90 but lon is inside, they're swapped
        if self.latitude.abs() > 90.0 && self.longitude.abs() <= 90.0 {
            std::mem::swap(&mut self.latitude, &mut self.longitude);
        }
    }

    /// Validate coordinates are in valid ranges
    pub fn validate_coordinates(&self) -> Result<(), String> {
        if self.latitude < -90.0 || self.latitude > 90.0 {
            return Err(format!(
                "Invalid latitude {} for station {} (must be -90 to 90)",
                self.latitude, self.station_id
            ));
        }
        if self.longitude < -180.0 || self.longitude > 180.0 {
            return Err(format!(
                "Invalid longitude {} for station {} (must be -180 to 180)",
                self.longitude, self.station_id
            ));
        }
        Ok(())
    }

    /// Assign to cluster based on geographic region
    pub fn assign_cluster(&mut self) {
        // 10 clusters based on longitude bands
        // -180 to -144: 0 (Pacific West)
        // -144 to -108: 1 (Americas West)
        // -108 to -72:  2 (Americas Central)
        // -72 to -36:   3 (Americas East)
        // -36 to 0:     4 (Atlantic)
        // 0 to 36:      5 (Europe/Africa West)
        // 36 to 72:     6 (Middle East)
        // 72 to 108:    7 (Asia West)
        // 108 to 144:   8 (Asia East)
        // 144 to 180:   9 (Pacific East)
        self.cluster_id = ((self.longitude + 180.0) / 36.0).floor() as u8;
        if self.cluster_id > 9 {
            self.cluster_id = 9;
        }
    }
}

/// Ground station network (257 stations)
#[derive(Debug, Clone, Default)]
pub struct GroundStationNetwork {
    stations: HashMap<String, GroundStation>,
}

impl GroundStationNetwork {
    pub fn new() -> Self {
        Self {
            stations: HashMap::new(),
        }
    }

    /// Add station to network
    pub fn add(&mut self, station: GroundStation) {
        self.stations.insert(station.station_id.clone(), station);
    }

    /// Get station by ID
    pub fn get(&self, station_id: &str) -> Option<&GroundStation> {
        self.stations.get(station_id)
    }

    /// Get all stations
    pub fn all(&self) -> impl Iterator<Item = &GroundStation> {
        self.stations.values()
    }

    /// Get stations in cluster
    pub fn by_cluster(&self, cluster_id: u8) -> Vec<&GroundStation> {
        self.stations
            .values()
            .filter(|s| s.cluster_id == cluster_id)
            .collect()
    }

    /// Station count
    pub fn count(&self) -> usize {
        self.stations.len()
    }

    /// Fix all coordinate inversions
    pub fn fix_all_inversions(&mut self) {
        for station in self.stations.values_mut() {
            station.fix_coordinate_inversion();
        }
    }

    /// Assign all stations to clusters
    pub fn assign_all_clusters(&mut self) {
        for station in self.stations.values_mut() {
            station.assign_cluster();
        }
    }

    /// Load from JSON seed data
    pub fn load_from_json(json_str: &str) -> anyhow::Result<Self> {
        let stations: Vec<GroundStation> = serde_json::from_str(json_str)?;
        let mut network = Self::new();
        for station in stations {
            network.add(station);
        }
        network.fix_all_inversions();
        network.assign_all_clusters();
        Ok(network)
    }

    /// Export to GeoJSON for visualization
    pub fn to_geojson(&self) -> serde_json::Value {
        let features: Vec<serde_json::Value> = self
            .stations
            .values()
            .map(|s| {
                serde_json::json!({
                    "type": "Feature",
                    "geometry": {
                        "type": "Point",
                        "coordinates": [s.longitude, s.latitude]
                    },
                    "properties": {
                        "station_id": s.station_id,
                        "name": s.name,
                        "country": s.country,
                        "cluster_id": s.cluster_id,
                        "fso_score": s.fso_suitability_score,
                        "operational": s.operational,
                        "cables": s.cable_names
                    }
                })
            })
            .collect();

        serde_json::json!({
            "type": "FeatureCollection",
            "features": features
        })
    }
}

/// Supabase table schema for ground_stations
/// Run this SQL in Supabase to create the receiver table:
pub const SUPABASE_SCHEMA: &str = r#"
-- Ground Stations table for 257 LaserLight FSO network
CREATE TABLE IF NOT EXISTS ground_stations (
    station_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    elevation_m DOUBLE PRECISION DEFAULT 0,
    country TEXT NOT NULL,
    region TEXT,
    city TEXT,
    cable_landing_id TEXT,
    cable_names TEXT[],
    fso_suitability_score DOUBLE PRECISION DEFAULT 0,
    clear_sky_days_per_year INTEGER DEFAULT 0,
    avg_cloud_cover_pct DOUBLE PRECISION DEFAULT 0,
    avg_water_vapor_mm DOUBLE PRECISION DEFAULT 0,
    cluster_id SMALLINT DEFAULT 0,
    trivariate_hash TEXT,
    operational BOOLEAN DEFAULT FALSE,
    wasm_sensor_deployed BOOLEAN DEFAULT FALSE,
    last_ping_ms INTEGER,
    last_updated TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for common queries
CREATE INDEX IF NOT EXISTS idx_ground_stations_cluster ON ground_stations(cluster_id);
CREATE INDEX IF NOT EXISTS idx_ground_stations_country ON ground_stations(country);
CREATE INDEX IF NOT EXISTS idx_ground_stations_operational ON ground_stations(operational);

-- Geospatial index (requires PostGIS extension)
-- CREATE INDEX IF NOT EXISTS idx_ground_stations_location
--     ON ground_stations USING GIST (ST_MakePoint(longitude, latitude));

-- RLS policies
ALTER TABLE ground_stations ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Public read access" ON ground_stations FOR SELECT USING (true);
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_inversion() {
        let mut station = GroundStation::from_cable_landing(
            "GS-001",
            "Test Station",
            -74.0060,  // Inverted: this is longitude
            40.7128,   // Inverted: this is latitude
            "USA",
            vec!["TAT-14".to_string()],
        );

        // Before fix: lat=-74, lon=40 (inverted)
        assert!(station.latitude.abs() > 90.0);

        station.fix_coordinate_inversion();

        // After fix: lat=40.7128, lon=-74.0060 (correct)
        assert!((station.latitude - 40.7128).abs() < 0.001);
        assert!((station.longitude - (-74.0060)).abs() < 0.001);
    }

    #[test]
    fn test_cluster_assignment() {
        let mut station = GroundStation::from_cable_landing(
            "GS-001",
            "New York",
            40.7128,
            -74.0060,
            "USA",
            vec![],
        );
        station.assign_cluster();

        // NYC at -74 longitude should be cluster 2 or 3 (Americas)
        assert!(station.cluster_id == 2 || station.cluster_id == 3);
    }

    #[test]
    fn test_geojson_export() {
        let mut network = GroundStationNetwork::new();
        network.add(GroundStation::from_cable_landing(
            "GS-001",
            "Test",
            40.0,
            -74.0,
            "USA",
            vec![],
        ));

        let geojson = network.to_geojson();
        assert_eq!(geojson["type"], "FeatureCollection");
        assert_eq!(geojson["features"].as_array().unwrap().len(), 1);
    }
}
