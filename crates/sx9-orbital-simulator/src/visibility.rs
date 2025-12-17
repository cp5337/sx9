//! Visibility calculations between satellites and ground stations

use crate::constants::*;
use crate::error::{OrbitalMechanicsError, Result};
use crate::ground_station::GroundStation;
use crate::orbit::{LookAngles, SatelliteOrbit, SatelliteState};
use crate::propagator::OrbitalPropagator;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Visibility window between satellite and ground station
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibilityWindow {
    pub satellite_id: String,
    pub station_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration_seconds: f64,
    pub max_elevation_time: DateTime<Utc>,
    pub max_elevation_deg: f64,
    pub min_range_km: f64,
    pub pass_type: PassType,
}

/// Type of satellite pass
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassType {
    /// Regular pass with acquisition, tracking, and loss of signal
    Normal,
    /// Satellite always visible (geostationary-like)
    Continuous,
    /// Partial pass (starts or ends outside observation window)
    Partial,
}

/// Visibility calculator
pub struct VisibilityCalculator {
    pub min_elevation_deg: f64,
    pub time_step_seconds: f64,
}

impl VisibilityCalculator {
    /// Create new visibility calculator
    pub fn new() -> Self {
        Self {
            min_elevation_deg: defaults::MIN_ELEVATION_DEG,
            time_step_seconds: 60.0, // 1 minute
        }
    }

    /// Create with custom parameters
    pub fn with_params(min_elevation_deg: f64, time_step_seconds: f64) -> Self {
        Self {
            min_elevation_deg,
            time_step_seconds,
        }
    }

    /// Calculate visibility windows
    pub fn calculate_windows(
        &self,
        satellite: &SatelliteOrbit,
        station: &GroundStation,
        start_time: DateTime<Utc>,
        duration_hours: f64,
        propagator: &dyn OrbitalPropagator,
    ) -> Result<Vec<VisibilityWindow>> {
        let mut windows = Vec::new();
        let end_time = start_time + Duration::seconds((duration_hours * 3600.0) as i64);

        let mut current_time = start_time;
        let mut in_pass = false;
        let mut pass_start = None;
        let mut max_elevation = 0.0;
        let mut max_elevation_time = start_time;
        let mut min_range = f64::INFINITY;

        while current_time <= end_time {
            let state = propagator.propagate(satellite, current_time)?;
            let look_angles = state.look_angles_from_station(
                station.position.latitude_deg,
                station.position.longitude_deg,
                station.position.elevation_m,
            );

            let visible = look_angles.elevation_deg >= self.min_elevation_deg;

            if visible && !in_pass {
                // Start of pass
                in_pass = true;
                pass_start = Some(current_time);
                max_elevation = look_angles.elevation_deg;
                max_elevation_time = current_time;
                min_range = look_angles.range_km;
            } else if visible && in_pass {
                // Continue pass - check for maximum elevation
                if look_angles.elevation_deg > max_elevation {
                    max_elevation = look_angles.elevation_deg;
                    max_elevation_time = current_time;
                }
                if look_angles.range_km < min_range {
                    min_range = look_angles.range_km;
                }
            } else if !visible && in_pass {
                // End of pass
                if let Some(start) = pass_start {
                    let duration = (current_time - start).num_seconds() as f64;

                    windows.push(VisibilityWindow {
                        satellite_id: satellite.satellite_id.clone(),
                        station_id: station.station_id.clone(),
                        start_time: start,
                        end_time: current_time,
                        duration_seconds: duration,
                        max_elevation_time,
                        max_elevation_deg: max_elevation,
                        min_range_km: min_range,
                        pass_type: PassType::Normal,
                    });
                }

                in_pass = false;
                max_elevation = 0.0;
                min_range = f64::INFINITY;
            }

            current_time += Duration::seconds(self.time_step_seconds as i64);
        }

        // Handle pass still in progress at end of observation period
        if in_pass {
            if let Some(start) = pass_start {
                let duration = (end_time - start).num_seconds() as f64;

                windows.push(VisibilityWindow {
                    satellite_id: satellite.satellite_id.clone(),
                    station_id: station.station_id.clone(),
                    start_time: start,
                    end_time: end_time,
                    duration_seconds: duration,
                    max_elevation_time,
                    max_elevation_deg: max_elevation,
                    min_range_km: min_range,
                    pass_type: PassType::Partial,
                });
            }
        }

        Ok(windows)
    }

    /// Calculate next pass time
    pub fn next_pass(
        &self,
        satellite: &SatelliteOrbit,
        station: &GroundStation,
        from_time: DateTime<Utc>,
        propagator: &dyn OrbitalPropagator,
    ) -> Result<Option<VisibilityWindow>> {
        let windows = self.calculate_windows(
            satellite, station, from_time, 48.0, // Search 48 hours ahead
            propagator,
        )?;

        Ok(windows.into_iter().next())
    }
}

impl Default for VisibilityCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ground_station::{GroundStation, StationPosition};
    use crate::orbit::{OrbitalElements, SatelliteOrbit};
    use crate::propagator::KeplerianPropagator;
    use chrono::Utc;

    #[test]
    fn test_visibility_calculation() {
        let calculator = VisibilityCalculator::new();
        let propagator = KeplerianPropagator::new();

        let elements = OrbitalElements::new(7000.0, 0.0, 55.0, 0.0, 0.0, 0.0).unwrap();
        let satellite = SatelliteOrbit::new(
            "TEST-01".to_string(),
            "Test Satellite".to_string(),
            elements,
            Utc::now(),
        );

        let station = GroundStation {
            station_id: "GS-001".to_string(),
            name: "Test Station".to_string(),
            position: StationPosition {
                latitude_deg: 40.0,
                longitude_deg: -105.0,
                elevation_m: 1600.0,
            },
        };

        let windows =
            calculator.calculate_windows(&satellite, &station, Utc::now(), 24.0, &propagator);

        assert!(windows.is_ok());
    }
}
