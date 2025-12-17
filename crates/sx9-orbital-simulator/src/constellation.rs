//! Satellite constellation design and management

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Timelike, Datelike};
use std::collections::HashMap;
use crate::config::{ConstellationConfig, ConstellationType, CustomSatellitePosition, PredefinedPattern};
use crate::orbit::{SatelliteOrbit, OrbitalElements};
use crate::constants::*;
use crate::error::{OrbitalMechanicsError, Result};
use crate::propagator::OrbitalPropagator;

/// Satellite constellation management
#[derive(Debug, Clone)]
pub struct Constellation {
    /// Constellation metadata
    pub name: String,
    pub description: String,
    pub constellation_type: ConstellationType,

    /// Satellites in the constellation
    satellites: HashMap<String, SatelliteOrbit>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}


impl Constellation {
    /// Create new empty constellation
    pub fn new(name: String, description: String, constellation_type: ConstellationType) -> Self {
        let now = Utc::now();

        Self {
            name,
            description,
            constellation_type,
            satellites: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Create constellation from configuration
    pub fn from_config(config: &ConstellationConfig) -> Result<Self> {
        let mut constellation = Self::new(
            config.name.clone(),
            config.description.clone(),
            config.constellation_type.clone(),
        );

        // Generate satellites based on constellation type
        match &config.constellation_type {
            ConstellationType::WalkerDelta {
                total_satellites,
                num_planes,
                satellites_per_plane: _,
                phasing_parameter,
            } => {
                constellation.generate_walker_delta(
                    *total_satellites,
                    *num_planes,
                    *phasing_parameter,
                    &config.orbital_parameters,
                )?;
            }
            ConstellationType::Custom { satellites } => {
                constellation.add_custom_satellites(satellites)?;
            }
            ConstellationType::Predefined { pattern } => {
                constellation.generate_predefined_pattern(pattern, &config.orbital_parameters)?;
            }
        }

        Ok(constellation)
    }

    /// Generate Walker Delta constellation pattern
    fn generate_walker_delta(
        &mut self,
        total_satellites: usize,
        num_planes: usize,
        phasing_parameter: usize,
        orbital_params: &crate::config::OrbitalParameters,
    ) -> Result<()> {
        if total_satellites % num_planes != 0 {
            return Err(OrbitalMechanicsError::config_error(
                "Total satellites must be divisible by number of planes for Walker Delta pattern"
            ));
        }

        let satellites_per_plane = total_satellites / num_planes;
        let epoch = Utc::now();

        // RAAN spacing between planes
        let raan_spacing = 360.0 / num_planes as f64;

        // Mean anomaly spacing within each plane
        let ma_spacing = 360.0 / satellites_per_plane as f64;

        // Phase offset between planes
        let phase_offset = (phasing_parameter as f64) * (360.0 / total_satellites as f64);

        for plane_idx in 0..num_planes {
            let raan = (plane_idx as f64) * raan_spacing;

            for sat_idx in 0..satellites_per_plane {
                let sat_number = plane_idx * satellites_per_plane + sat_idx + 1;
                let satellite_id = format!("SAT-{:03}", sat_number);
                let name = format!("{} Satellite {}", self.name, sat_number);

                // Mean anomaly with phasing
                let base_ma = (sat_idx as f64) * ma_spacing;
                let phased_ma = (base_ma + (plane_idx as f64) * phase_offset) % 360.0;

                let elements = OrbitalElements::new(
                    EARTH_RADIUS_KM + orbital_params.altitude_km,
                    orbital_params.eccentricity,
                    orbital_params.inclination_deg,
                    raan,
                    orbital_params.argument_of_perigee_deg,
                    phased_ma,
                )?;

                let satellite = SatelliteOrbit::new(satellite_id, name, elements, epoch);
                self.add_satellite(satellite)?;
            }
        }

        Ok(())
    }

    /// Add custom satellites from configuration
    fn add_custom_satellites(&mut self, satellites: &[CustomSatellitePosition]) -> Result<()> {
        let epoch = Utc::now();

        for sat_config in satellites {
            let elements = OrbitalElements::new(
                sat_config.semi_major_axis_km,
                sat_config.eccentricity,
                sat_config.inclination_deg,
                sat_config.longitude_of_ascending_node_deg,
                sat_config.argument_of_perigee_deg,
                sat_config.mean_anomaly_deg,
            )?;

            let satellite = SatelliteOrbit::new(
                sat_config.satellite_id.clone(),
                sat_config.name.clone(),
                elements,
                epoch,
            );

            self.add_satellite(satellite)?;
        }

        Ok(())
    }

    /// Generate predefined constellation patterns
    fn generate_predefined_pattern(
        &mut self,
        pattern: &PredefinedPattern,
        orbital_params: &crate::config::OrbitalParameters,
    ) -> Result<()> {
        match pattern {
            PredefinedPattern::LaserLightFsoMeo => {
                self.generate_laserlight_fso_constellation()?;
            }
            PredefinedPattern::GlobalStarlink => {
                self.generate_starlink_like_constellation()?;
            }
            PredefinedPattern::OneWebLeo => {
                self.generate_oneweb_like_constellation()?;
            }
            PredefinedPattern::CustomMeo => {
                // Use provided orbital parameters
                self.generate_walker_delta(12, 3, 1, orbital_params)?;
            }
        }

        Ok(())
    }

    /// Generate LaserLight FSO MEO constellation (12 satellites, 3 planes, 8000 km altitude)
    pub fn generate_laserlight_fso_constellation(&mut self) -> Result<()> {
        let orbital_params = crate::config::OrbitalParameters {
            altitude_km: LASERLIGHT_FSO_ALTITUDE_KM,
            inclination_deg: 55.0,
            eccentricity: 0.0001, // Nearly circular
            raan_spacing_deg: 120.0, // 3 planes
            argument_of_perigee_deg: 0.0,
            phase_spacing_deg: 30.0, // Walker Delta 12/3/1
        };

        self.generate_walker_delta(12, 3, 1, &orbital_params)?;

        // Update satellite IDs and names for LaserLight
        let satellite_ids: Vec<String> = self.satellites.keys().cloned().collect();
        for (idx, old_id) in satellite_ids.iter().enumerate() {
            if let Some(mut satellite) = self.satellites.remove(old_id) {
                satellite.satellite_id = format!("LASERLIGHT-FSO-{:02}", idx + 1);
                satellite.name = format!("LaserLight FSO Satellite {}", idx + 1);
                self.satellites.insert(satellite.satellite_id.clone(), satellite);
            }
        }

        Ok(())
    }

    /// Generate Starlink-like constellation (simplified)
    fn generate_starlink_like_constellation(&mut self) -> Result<()> {
        // Simplified Starlink constellation: 24 satellites in 3 planes at 550 km
        let orbital_params = crate::config::OrbitalParameters {
            altitude_km: STARLINK_ALTITUDE_KM,
            inclination_deg: 53.0,
            eccentricity: 0.0001,
            raan_spacing_deg: 120.0,
            argument_of_perigee_deg: 0.0,
            phase_spacing_deg: 15.0, // Walker Delta 24/3/1
        };

        self.generate_walker_delta(24, 3, 1, &orbital_params)
    }

    /// Generate OneWeb-like constellation (simplified)
    fn generate_oneweb_like_constellation(&mut self) -> Result<()> {
        // Simplified OneWeb constellation: 18 satellites in 6 planes at 1200 km
        let orbital_params = crate::config::OrbitalParameters {
            altitude_km: 1200.0,
            inclination_deg: 87.4, // Near-polar
            eccentricity: 0.0001,
            raan_spacing_deg: 60.0, // 6 planes
            argument_of_perigee_deg: 0.0,
            phase_spacing_deg: 20.0, // Walker Delta 18/6/1
        };

        self.generate_walker_delta(18, 6, 1, &orbital_params)
    }

    /// Add satellite to constellation
    pub fn add_satellite(&mut self, satellite: SatelliteOrbit) -> Result<()> {
        if self.satellites.contains_key(&satellite.satellite_id) {
            return Err(OrbitalMechanicsError::config_error(
                format!("Satellite {} already exists in constellation", satellite.satellite_id)
            ));
        }

        self.satellites.insert(satellite.satellite_id.clone(), satellite);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove satellite from constellation
    pub fn remove_satellite(&mut self, satellite_id: &str) -> Result<SatelliteOrbit> {
        match self.satellites.remove(satellite_id) {
            Some(satellite) => {
                self.updated_at = Utc::now();
                Ok(satellite)
            }
            None => Err(OrbitalMechanicsError::SatelliteNotFound(satellite_id.to_string()))
        }
    }

    /// Get satellite by ID
    pub fn get_satellite(&self, satellite_id: &str) -> Option<&SatelliteOrbit> {
        self.satellites.get(satellite_id)
    }

    /// Get mutable satellite by ID
    pub fn get_satellite_mut(&mut self, satellite_id: &str) -> Option<&mut SatelliteOrbit> {
        self.satellites.get_mut(satellite_id)
    }

    /// Get all satellites
    pub fn satellites(&self) -> impl Iterator<Item = &SatelliteOrbit> {
        self.satellites.values()
    }

    /// Get satellite count
    pub fn satellite_count(&self) -> usize {
        self.satellites.len()
    }

    /// Check if constellation is empty
    pub fn is_empty(&self) -> bool {
        self.satellites.is_empty()
    }

    /// Calculate constellation coverage statistics
    pub fn coverage_statistics(&self) -> ConstellationCoverage {
        let mut total_inclination = 0.0;
        let mut max_inclination = 0.0f64;
        let mut min_altitude = f64::INFINITY;
        let mut max_altitude = 0.0f64;
        let mut orbital_periods = Vec::new();

        for satellite in self.satellites.values() {
            let inclination = satellite.elements.inclination_deg;
            total_inclination += inclination;
            max_inclination = max_inclination.max(inclination);

            let altitude = satellite.elements.semi_major_axis_km - EARTH_RADIUS_KM;
            min_altitude = min_altitude.min(altitude);
            max_altitude = max_altitude.max(altitude);

            orbital_periods.push(satellite.period_seconds);
        }

        let avg_inclination = if !self.satellites.is_empty() {
            total_inclination / self.satellites.len() as f64
        } else {
            0.0
        };

        // Calculate latitude coverage
        let latitude_coverage = if max_inclination > 0.0 {
            ConstellationLatitudeCoverage {
                max_north_latitude: max_inclination,
                max_south_latitude: -max_inclination,
                global_coverage_percent: if max_inclination >= 85.0 { 100.0 } else { max_inclination / 90.0 * 100.0 },
            }
        } else {
            ConstellationLatitudeCoverage {
                max_north_latitude: 0.0,
                max_south_latitude: 0.0,
                global_coverage_percent: 0.0,
            }
        };

        ConstellationCoverage {
            satellite_count: self.satellites.len(),
            average_inclination_deg: avg_inclination,
            altitude_range_km: (min_altitude, max_altitude),
            orbital_period_range_seconds: if !orbital_periods.is_empty() {
                (*orbital_periods.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
                 *orbital_periods.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())
            } else {
                (0.0, 0.0)
            },
            latitude_coverage,
        }
    }

    /// Generate constellation status report
    pub fn generate_status_report(&self, time: DateTime<Utc>, propagator: &dyn OrbitalPropagator) -> Result<String> {
        let mut report = String::new();

        report.push_str("=== CTAS-7 Constellation Status Report ===\n");
        report.push_str(&format!("Constellation: {}\n", self.name));
        report.push_str(&format!("Description: {}\n", self.description));
        report.push_str(&format!("Report Time: {}\n", time.format("%Y-%m-%d %H:%M:%S UTC")));
        report.push_str(&format!("Satellites: {}\n", self.satellites.len()));
        report.push_str("\n");

        let coverage = self.coverage_statistics();
        report.push_str("Coverage Statistics:\n");
        report.push_str(&format!("  Average Inclination: {:.1}°\n", coverage.average_inclination_deg));
        report.push_str(&format!("  Altitude Range: {:.1} - {:.1} km\n",
            coverage.altitude_range_km.0, coverage.altitude_range_km.1));
        report.push_str(&format!("  Global Coverage: {:.1}%\n", coverage.latitude_coverage.global_coverage_percent));
        report.push_str(&format!("  Latitude Coverage: {:.1}°N to {:.1}°S\n",
            coverage.latitude_coverage.max_north_latitude,
            coverage.latitude_coverage.max_south_latitude.abs()));
        report.push_str("\n");

        report.push_str("Satellite Positions:\n");
        for satellite in self.satellites.values() {
            match propagator.propagate(satellite, time) {
                Ok(state) => {
                    report.push_str(&format!("  {}: Lat {:.3}°, Lon {:.3}°, Alt {:.1} km, Eclipse: {}\n",
                        satellite.satellite_id,
                        state.geodetic.latitude_deg,
                        state.geodetic.longitude_deg,
                        state.geodetic.altitude_km,
                        if state.in_eclipse { "Yes" } else { "No" }
                    ));
                }
                Err(_) => {
                    report.push_str(&format!("  {}: Position calculation failed\n", satellite.satellite_id));
                }
            }
        }

        report.push_str("\n");
        report.push_str(&format!("Report generated at: {}\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        Ok(report)
    }

    /// Export constellation to TLE format
    pub fn to_tle_format(&self, time: DateTime<Utc>) -> Result<String> {
        let mut tle_data = String::new();

        for satellite in self.satellites.values() {
            // Line 0: Satellite name
            tle_data.push_str(&format!("{}\n", satellite.name));

            // Line 1: Catalog number, classification, launch year/number, epoch, derivatives, drag, ephemeris type, element number
            tle_data.push_str(&format!("1 {:5}U {:8} {:14.8} {:10} {:8} 0 {:4}9\n",
                25544, // Placeholder catalog number
                "98067A", // Placeholder international designator
                self.julian_day_from_datetime(time),
                " .00002182", // First derivative of mean motion
                " 00000-0", // Second derivative of mean motion
                9999 // Element number
            ));

            // Line 2: Inclination, RAAN, eccentricity, argument of perigee, mean anomaly, mean motion, revolution number
            tle_data.push_str(&format!("2 {:5} {:8.4} {:8.4} {:7} {:8.4} {:8.4} {:11.8}{:5}\n",
                25544, // Catalog number (same as line 1)
                satellite.elements.inclination_deg,
                satellite.elements.raan_deg,
                (satellite.elements.eccentricity * 10000000.0) as u32, // Remove decimal point
                satellite.elements.argument_of_perigee_deg,
                satellite.elements.mean_anomaly_deg,
                satellite.mean_motion_rev_per_day,
                99999 // Revolution number at epoch
            ));
        }

        Ok(tle_data)
    }

    /// Convert DateTime to Julian Day (simplified)
    fn julian_day_from_datetime(&self, dt: DateTime<Utc>) -> f64 {
        // Simplified Julian Day calculation
        let year = dt.year() as f64;
        let day_of_year = dt.ordinal() as f64;
        let hour_fraction = (dt.hour() as f64 + dt.minute() as f64 / 60.0 + dt.second() as f64 / 3600.0) / 24.0;

        // Approximate Julian Day (not exact, but sufficient for TLE format)
        2440000.0 + (year - 1970.0) * 365.25 + day_of_year + hour_fraction
    }
}

/// Constellation coverage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstellationCoverage {
    pub satellite_count: usize,
    pub average_inclination_deg: f64,
    pub altitude_range_km: (f64, f64),
    pub orbital_period_range_seconds: (f64, f64),
    pub latitude_coverage: ConstellationLatitudeCoverage,
}

/// Latitude coverage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstellationLatitudeCoverage {
    pub max_north_latitude: f64,
    pub max_south_latitude: f64,
    pub global_coverage_percent: f64,
}

/// Default LaserLight FSO constellation
impl Default for Constellation {
    fn default() -> Self {
        let mut constellation = Self::new(
            "LaserLight FSO MEO".to_string(),
            "12-satellite MEO constellation for optical communications".to_string(),
            ConstellationType::Predefined {
                pattern: PredefinedPattern::LaserLightFsoMeo
            },
        );

        constellation.generate_laserlight_fso_constellation()
            .expect("Failed to generate default LaserLight constellation");

        constellation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ConstellationConfig;

    #[test]
    fn test_walker_delta_constellation() {
        let mut constellation = Constellation::new(
            "Test Walker".to_string(),
            "Test constellation".to_string(),
            ConstellationType::WalkerDelta {
                total_satellites: 12,
                num_planes: 3,
                satellites_per_plane: 4,
                phasing_parameter: 1,
            },
        );

        let orbital_params = crate::config::OrbitalParameters {
            altitude_km: 8000.0,
            inclination_deg: 55.0,
            eccentricity: 0.0001,
            raan_spacing_deg: 120.0,
            argument_of_perigee_deg: 0.0,
            phase_spacing_deg: 30.0,
        };

        let result = constellation.generate_walker_delta(12, 3, 1, &orbital_params);
        assert!(result.is_ok());
        assert_eq!(constellation.satellite_count(), 12);
    }

    #[test]
    fn test_laserlight_constellation() {
        let constellation = Constellation::default();
        assert_eq!(constellation.satellite_count(), 12);
        assert_eq!(constellation.name, "LaserLight FSO MEO");

        let coverage = constellation.coverage_statistics();
        assert_eq!(coverage.satellite_count, 12);
        assert!((coverage.average_inclination_deg - 55.0).abs() < 0.1);
    }

    #[test]
    fn test_constellation_from_config() {
        let config = ConstellationConfig::laserlight_fso_meo();
        let constellation = Constellation::from_config(&config);

        assert!(constellation.is_ok());
        let constellation = constellation.unwrap();
        assert_eq!(constellation.satellite_count(), 12);
    }

    #[test]
    fn test_satellite_operations() {
        let mut constellation = Constellation::new(
            "Test".to_string(),
            "Test".to_string(),
            ConstellationType::Custom {
                satellites: vec![]
            },
        );

        let elements = OrbitalElements::new(7000.0, 0.0, 55.0, 0.0, 0.0, 0.0).unwrap();
        let satellite = SatelliteOrbit::new(
            "TEST-01".to_string(),
            "Test Satellite".to_string(),
            elements,
            Utc::now(),
        );

        // Add satellite
        assert!(constellation.add_satellite(satellite).is_ok());
        assert_eq!(constellation.satellite_count(), 1);

        // Get satellite
        assert!(constellation.get_satellite("TEST-01").is_some());

        // Remove satellite
        assert!(constellation.remove_satellite("TEST-01").is_ok());
        assert_eq!(constellation.satellite_count(), 0);
    }
}