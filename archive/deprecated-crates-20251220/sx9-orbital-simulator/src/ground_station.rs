//! Ground station definitions and network management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::constants::*;
use crate::error::{OrbitalMechanicsError, Result};

/// Ground station definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundStation {
    pub station_id: String,
    pub name: String,
    pub position: StationPosition,
}

/// Ground station position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationPosition {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub elevation_m: f64,
}

/// Ground station network
#[derive(Debug, Clone)]
pub struct GroundStationNetwork {
    stations: HashMap<String, GroundStation>,
}

impl GroundStationNetwork {
    /// Create new empty network
    pub fn new() -> Self {
        Self {
            stations: HashMap::new(),
        }
    }

    /// Add station to network
    pub fn add_station(&mut self, station: GroundStation) {
        self.stations.insert(station.station_id.clone(), station);
    }

    /// Get station by ID
    pub fn get_station(&self, station_id: &str) -> Option<&GroundStation> {
        self.stations.get(station_id)
    }

    /// Get all stations
    pub fn stations(&self) -> impl Iterator<Item = &GroundStation> {
        self.stations.values()
    }

    /// Station count
    pub fn station_count(&self) -> usize {
        self.stations.len()
    }
}

impl Default for GroundStationNetwork {
    fn default() -> Self {
        Self::new()
    }
}