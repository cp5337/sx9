//! Orbital elements and satellite state definitions

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::f64::consts::PI;
use crate::constants::*;
use crate::constants::validation::*;
use crate::error::{OrbitalMechanicsError, Result};

/// Classical orbital elements (Keplerian elements)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalElements {
    /// Semi-major axis in kilometers
    pub semi_major_axis_km: f64,

    /// Eccentricity (0 = circular, 0 < e < 1 = elliptical)
    pub eccentricity: f64,

    /// Inclination in degrees (0-180°)
    pub inclination_deg: f64,

    /// Right Ascension of Ascending Node in degrees (0-360°)
    pub raan_deg: f64,

    /// Argument of perigee in degrees (0-360°)
    pub argument_of_perigee_deg: f64,

    /// Mean anomaly at epoch in degrees (0-360°)
    pub mean_anomaly_deg: f64,
}

/// Complete satellite orbital definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatelliteOrbit {
    /// Unique satellite identifier
    pub satellite_id: String,

    /// Human-readable satellite name
    pub name: String,

    /// Orbital elements
    pub elements: OrbitalElements,

    /// Reference epoch for the orbital elements
    pub epoch: DateTime<Utc>,

    /// Orbital period in seconds
    pub period_seconds: f64,

    /// Mean motion in revolutions per day
    pub mean_motion_rev_per_day: f64,

    /// Mean motion in radians per second
    pub mean_motion_rad_per_sec: f64,
}

/// Current satellite state (position and velocity)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatelliteState {
    /// Satellite identifier
    pub satellite_id: String,

    /// Timestamp of this state
    pub timestamp: DateTime<Utc>,

    /// Position in Earth-Centered Inertial (ECI) coordinates
    pub position_eci: [f64; 3], // [x, y, z] in km

    /// Velocity in ECI coordinates
    pub velocity_eci: [f64; 3], // [vx, vy, vz] in km/s

    /// Geodetic position (latitude, longitude, altitude)
    pub geodetic: GeodeticPosition,

    /// Orbital elements at this time
    pub current_elements: Option<OrbitalElements>,

    /// Eclipse status (true if in Earth's shadow)
    pub in_eclipse: bool,

    /// Ground track velocity in km/s
    pub ground_track_velocity: f64,

    /// Current orbital radius in km
    pub orbital_radius: f64,
}

/// Geodetic position on Earth's surface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeodeticPosition {
    /// Latitude in degrees (-90° to +90°)
    pub latitude_deg: f64,

    /// Longitude in degrees (-180° to +180°)
    pub longitude_deg: f64,

    /// Altitude above sea level in kilometers
    pub altitude_km: f64,
}

impl OrbitalElements {
    /// Create new orbital elements with validation
    pub fn new(
        semi_major_axis_km: f64,
        eccentricity: f64,
        inclination_deg: f64,
        raan_deg: f64,
        argument_of_perigee_deg: f64,
        mean_anomaly_deg: f64,
    ) -> Result<Self> {
        validate_orbital_elements(
            semi_major_axis_km,
            eccentricity,
            inclination_deg,
            raan_deg,
            argument_of_perigee_deg,
            mean_anomaly_deg,
        )?;

        Ok(Self {
            semi_major_axis_km,
            eccentricity,
            inclination_deg,
            raan_deg,
            argument_of_perigee_deg,
            mean_anomaly_deg,
        })
    }

    /// Calculate orbital period using Kepler's third law
    pub fn calculate_period(&self) -> f64 {
        2.0 * PI * (self.semi_major_axis_km.powi(3) / EARTH_MU).sqrt()
    }

    /// Calculate mean motion in revolutions per day
    pub fn calculate_mean_motion_rev_per_day(&self) -> f64 {
        SOLAR_DAY_SECONDS / self.calculate_period()
    }

    /// Calculate mean motion in radians per second
    pub fn calculate_mean_motion_rad_per_sec(&self) -> f64 {
        TWO_PI / self.calculate_period()
    }

    /// Calculate apogee altitude in kilometers
    pub fn apogee_altitude_km(&self) -> f64 {
        self.semi_major_axis_km * (1.0 + self.eccentricity) - EARTH_RADIUS_KM
    }

    /// Calculate perigee altitude in kilometers
    pub fn perigee_altitude_km(&self) -> f64 {
        self.semi_major_axis_km * (1.0 - self.eccentricity) - EARTH_RADIUS_KM
    }

    /// Check if orbit is circular (eccentricity < 0.01)
    pub fn is_circular(&self) -> bool {
        self.eccentricity < 0.01
    }

    /// Check if orbit is elliptical
    pub fn is_elliptical(&self) -> bool {
        self.eccentricity > 0.01 && self.eccentricity < 1.0
    }

    /// Get orbit classification by altitude
    pub fn orbit_classification(&self) -> OrbitClassification {
        let altitude = self.semi_major_axis_km - EARTH_RADIUS_KM;

        if altitude < LEO_MAX_ALTITUDE_KM {
            OrbitClassification::Leo
        } else if altitude < MEO_MAX_ALTITUDE_KM {
            OrbitClassification::Meo
        } else if (altitude - GEO_ALTITUDE_KM).abs() < 100.0 {
            OrbitClassification::Geo
        } else {
            OrbitClassification::Heo
        }
    }

    /// Convert to radians for calculations
    pub fn to_radians(&self) -> OrbitalElementsRad {
        OrbitalElementsRad {
            semi_major_axis_km: self.semi_major_axis_km,
            eccentricity: self.eccentricity,
            inclination_rad: self.inclination_deg * DEG_TO_RAD,
            raan_rad: self.raan_deg * DEG_TO_RAD,
            argument_of_perigee_rad: self.argument_of_perigee_deg * DEG_TO_RAD,
            mean_anomaly_rad: self.mean_anomaly_deg * DEG_TO_RAD,
        }
    }
}

/// Orbital elements in radians (for calculations)
#[derive(Debug, Clone)]
pub struct OrbitalElementsRad {
    pub semi_major_axis_km: f64,
    pub eccentricity: f64,
    pub inclination_rad: f64,
    pub raan_rad: f64,
    pub argument_of_perigee_rad: f64,
    pub mean_anomaly_rad: f64,
}

/// Orbit classification by altitude
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrbitClassification {
    /// Low Earth Orbit (160-2000 km)
    Leo,
    /// Medium Earth Orbit (2000-35786 km)
    Meo,
    /// Geostationary Earth Orbit (~35786 km)
    Geo,
    /// High Earth Orbit (>35786 km)
    Heo,
}

impl SatelliteOrbit {
    /// Create new satellite orbit
    pub fn new(
        satellite_id: String,
        name: String,
        elements: OrbitalElements,
        epoch: DateTime<Utc>,
    ) -> Self {
        let period_seconds = elements.calculate_period();
        let mean_motion_rev_per_day = elements.calculate_mean_motion_rev_per_day();
        let mean_motion_rad_per_sec = elements.calculate_mean_motion_rad_per_sec();

        Self {
            satellite_id,
            name,
            elements,
            epoch,
            period_seconds,
            mean_motion_rev_per_day,
            mean_motion_rad_per_sec,
        }
    }

    /// Create circular orbit at specified altitude and inclination
    pub fn circular_orbit(
        satellite_id: String,
        name: String,
        altitude_km: f64,
        inclination_deg: f64,
        raan_deg: f64,
        mean_anomaly_deg: f64,
        epoch: DateTime<Utc>,
    ) -> Result<Self> {
        let semi_major_axis = EARTH_RADIUS_KM + altitude_km;

        let elements = OrbitalElements::new(
            semi_major_axis,
            0.0, // Circular orbit
            inclination_deg,
            raan_deg,
            0.0, // Not relevant for circular orbits
            mean_anomaly_deg,
        )?;

        Ok(Self::new(satellite_id, name, elements, epoch))
    }

    /// Update mean anomaly for a given time
    pub fn update_mean_anomaly(&self, time: DateTime<Utc>) -> f64 {
        let time_since_epoch = (time - self.epoch).num_seconds() as f64;
        let delta_mean_anomaly = self.mean_motion_rad_per_sec * time_since_epoch;

        // Normalize to 0-2π range
        let updated_mean_anomaly = (self.elements.mean_anomaly_deg * DEG_TO_RAD + delta_mean_anomaly) % TWO_PI;

        updated_mean_anomaly * RAD_TO_DEG
    }

    /// Get orbital velocity for circular orbit
    pub fn circular_velocity(&self) -> f64 {
        (EARTH_MU / self.elements.semi_major_axis_km).sqrt()
    }

    /// Check if satellite passes over a given latitude
    pub fn passes_over_latitude(&self, latitude_deg: f64) -> bool {
        self.elements.inclination_deg >= latitude_deg.abs()
    }

    /// Calculate ground track spacing at equator
    pub fn ground_track_spacing_km(&self) -> f64 {
        // Earth rotation during one orbital period
        let earth_rotation_angle = EARTH_ROTATION_RATE * self.period_seconds;
        earth_rotation_angle * EARTH_RADIUS_KM
    }
}

impl SatelliteState {
    /// Create new satellite state
    pub fn new(
        satellite_id: String,
        timestamp: DateTime<Utc>,
        position_eci: [f64; 3],
        velocity_eci: [f64; 3],
    ) -> Self {
        let geodetic = Self::eci_to_geodetic(position_eci);
        let orbital_radius = (position_eci[0].powi(2) + position_eci[1].powi(2) + position_eci[2].powi(2)).sqrt();
        let ground_track_velocity = (velocity_eci[0].powi(2) + velocity_eci[1].powi(2) + velocity_eci[2].powi(2)).sqrt();
        let in_eclipse = Self::calculate_eclipse_status(position_eci);

        Self {
            satellite_id,
            timestamp,
            position_eci,
            velocity_eci,
            geodetic,
            current_elements: None,
            in_eclipse,
            ground_track_velocity,
            orbital_radius,
        }
    }

    /// Convert ECI position to geodetic coordinates
    fn eci_to_geodetic(position_eci: [f64; 3]) -> GeodeticPosition {
        let x = position_eci[0];
        let y = position_eci[1];
        let z = position_eci[2];

        let r = (x*x + y*y + z*z).sqrt();
        let latitude_deg = (z / r).asin() * RAD_TO_DEG;
        let longitude_deg = y.atan2(x) * RAD_TO_DEG;
        let altitude_km = r - EARTH_RADIUS_KM;

        GeodeticPosition {
            latitude_deg,
            longitude_deg,
            altitude_km,
        }
    }

    /// Simple eclipse calculation (satellite in Earth's shadow)
    fn calculate_eclipse_status(position_eci: [f64; 3]) -> bool {
        let x = position_eci[0];
        let y = position_eci[1];
        let z = position_eci[2];

        // Simplified eclipse model: satellite on night side and within shadow cone
        let shadow_radius = EARTH_RADIUS_KM * 1.1; // Add margin for penumbra
        x < 0.0 && (y*y + z*z).sqrt() < shadow_radius
    }

    /// Calculate look angles from ground station
    pub fn look_angles_from_station(&self, station_lat_deg: f64, station_lon_deg: f64, station_alt_m: f64) -> LookAngles {
        // Convert ground station to ECI coordinates
        let lat_rad = station_lat_deg * DEG_TO_RAD;
        let lon_rad = station_lon_deg * DEG_TO_RAD;
        let r_station = EARTH_RADIUS_KM + station_alt_m / 1000.0;

        let x_station = r_station * lat_rad.cos() * lon_rad.cos();
        let y_station = r_station * lat_rad.cos() * lon_rad.sin();
        let z_station = r_station * lat_rad.sin();

        // Range vector from station to satellite
        let dx = self.position_eci[0] - x_station;
        let dy = self.position_eci[1] - y_station;
        let dz = self.position_eci[2] - z_station;
        let range = (dx*dx + dy*dy + dz*dz).sqrt();

        // Convert to topocentric coordinates (SEZ)
        let sin_lat = lat_rad.sin();
        let cos_lat = lat_rad.cos();
        let sin_lon = lon_rad.sin();
        let cos_lon = lon_rad.cos();

        let s = -dx * sin_lat * cos_lon - dy * sin_lat * sin_lon + dz * cos_lat;
        let e = -dx * sin_lon + dy * cos_lon;
        let z = dx * cos_lat * cos_lon + dy * cos_lat * sin_lon + dz * sin_lat;

        // Calculate elevation and azimuth
        let elevation_rad = (z / range).asin();
        let azimuth_rad = e.atan2(s);

        LookAngles {
            elevation_deg: elevation_rad * RAD_TO_DEG,
            azimuth_deg: if azimuth_rad < 0.0 {
                azimuth_rad * RAD_TO_DEG + 360.0
            } else {
                azimuth_rad * RAD_TO_DEG
            },
            range_km: range,
            range_rate_km_per_s: 0.0, // Would need velocity calculation
        }
    }

    /// Check if satellite is visible from ground station
    pub fn is_visible_from_station(&self, station_lat_deg: f64, station_lon_deg: f64, station_alt_m: f64, min_elevation_deg: f64) -> bool {
        let look_angles = self.look_angles_from_station(station_lat_deg, station_lon_deg, station_alt_m);
        look_angles.elevation_deg >= min_elevation_deg
    }
}

/// Look angles from ground station to satellite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookAngles {
    /// Elevation angle in degrees (0° = horizon, 90° = zenith)
    pub elevation_deg: f64,

    /// Azimuth angle in degrees (0° = North, 90° = East)
    pub azimuth_deg: f64,

    /// Range distance in kilometers
    pub range_km: f64,

    /// Range rate in km/s (positive = receding)
    pub range_rate_km_per_s: f64,
}

impl GeodeticPosition {
    /// Create new geodetic position with validation
    pub fn new(latitude_deg: f64, longitude_deg: f64, altitude_km: f64) -> Result<Self> {
        if latitude_deg < -90.0 || latitude_deg > 90.0 {
            return Err(OrbitalMechanicsError::CoordinateError(
                format!("Latitude {:.3}° outside valid range [-90°, +90°]", latitude_deg)
            ));
        }

        let longitude_normalized = if longitude_deg < -180.0 {
            longitude_deg + 360.0
        } else if longitude_deg > 180.0 {
            longitude_deg - 360.0
        } else {
            longitude_deg
        };

        Ok(Self {
            latitude_deg,
            longitude_deg: longitude_normalized,
            altitude_km,
        })
    }

    /// Convert to Earth-Centered Earth-Fixed (ECEF) coordinates
    pub fn to_ecef(&self) -> [f64; 3] {
        let lat_rad = self.latitude_deg * DEG_TO_RAD;
        let lon_rad = self.longitude_deg * DEG_TO_RAD;
        let r = EARTH_RADIUS_KM + self.altitude_km;

        [
            r * lat_rad.cos() * lon_rad.cos(),
            r * lat_rad.cos() * lon_rad.sin(),
            r * lat_rad.sin(),
        ]
    }

    /// Calculate distance to another geodetic position
    pub fn distance_to(&self, other: &GeodeticPosition) -> f64 {
        let lat1_rad = self.latitude_deg * DEG_TO_RAD;
        let lon1_rad = self.longitude_deg * DEG_TO_RAD;
        let lat2_rad = other.latitude_deg * DEG_TO_RAD;
        let lon2_rad = other.longitude_deg * DEG_TO_RAD;

        // Haversine formula
        let dlat = lat2_rad - lat1_rad;
        let dlon = lon2_rad - lon1_rad;

        let a = (dlat/2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (dlon/2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();

        EARTH_RADIUS_KM * c
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_orbital_elements_creation() {
        let elements = OrbitalElements::new(
            7000.0, // 600 km altitude
            0.01,   // Slightly elliptical
            55.0,   // MEO inclination
            0.0,    // RAAN
            0.0,    // Argument of perigee
            0.0     // Mean anomaly
        );

        assert!(elements.is_ok());
        let elements = elements.unwrap();
        assert_eq!(elements.orbit_classification(), OrbitClassification::Leo);
    }

    #[test]
    fn test_circular_orbit_creation() {
        let orbit = SatelliteOrbit::circular_orbit(
            "TEST-01".to_string(),
            "Test Satellite".to_string(),
            8000.0, // LaserLight altitude
            55.0,   // Inclination
            0.0,    // RAAN
            0.0,    // Mean anomaly
            Utc::now()
        );

        assert!(orbit.is_ok());
        let orbit = orbit.unwrap();
        assert!(orbit.elements.is_circular());
        assert_eq!(orbit.elements.orbit_classification(), OrbitClassification::Meo);
    }

    #[test]
    fn test_geodetic_position() {
        let pos = GeodeticPosition::new(40.0, -105.0, 0.0);
        assert!(pos.is_ok());

        let pos = pos.unwrap();
        let ecef = pos.to_ecef();
        assert!(ecef[0] > 0.0); // Positive X for this longitude
        assert!(ecef[2] > 0.0); // Positive Z for North latitude
    }

    #[test]
    fn test_orbital_period_calculation() {
        let elements = OrbitalElements::new(7000.0, 0.0, 55.0, 0.0, 0.0, 0.0).unwrap();
        let period = elements.calculate_period();

        // Period should be approximately 98 minutes for 600 km altitude
        assert!(period > 5800.0 && period < 6000.0);
    }
}