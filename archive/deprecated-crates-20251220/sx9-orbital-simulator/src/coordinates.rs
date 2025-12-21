//! Coordinate system transformations

use serde::{Deserialize, Serialize};
use crate::constants::*;

/// 3D position vector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Geodetic position (latitude, longitude, altitude)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeodeticPosition {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub altitude_km: f64,
}

/// Coordinate system types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinateSystem {
    /// Earth-Centered Inertial
    Eci,
    /// Earth-Centered Earth-Fixed
    Ecef,
    /// Geodetic (Lat/Lon/Alt)
    Geodetic,
    /// Topocentric (South/East/Zenith)
    Topocentric,
}

impl Position3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn to_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

impl From<[f64; 3]> for Position3D {
    fn from(arr: [f64; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}