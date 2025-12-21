//! Free Space Optical (FSO) link analysis

use crate::constants::*;
use crate::ground_station::GroundStation;
use crate::orbit::SatelliteState;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// FSO link quality assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsoLinkQuality {
    pub satellite_id: String,
    pub station_id: String,
    pub timestamp: DateTime<Utc>,
    pub elevation_angle_deg: f64,
    pub azimuth_angle_deg: f64,
    pub range_km: f64,
    pub atmospheric_transmission: f64,
    pub link_margin_db: f64,
    pub estimated_throughput_gbps: f64,
    pub weather_impact_factor: f64,
}

/// FSO link analyzer
pub struct FsoAnalyzer {
    pub wavelength_nm: f64,
    pub transmit_power_w: f64,
    pub receiver_aperture_m: f64,
}

impl FsoAnalyzer {
    /// Create new FSO analyzer
    pub fn new() -> Self {
        Self {
            wavelength_nm: FSO_WAVELENGTH_1550NM * 1e9,
            transmit_power_w: defaults::FSO_TRANSMIT_POWER_W,
            receiver_aperture_m: defaults::FSO_RECEIVER_APERTURE_M,
        }
    }

    /// Analyze FSO link quality
    pub fn analyze_link(
        &self,
        satellite_state: &SatelliteState,
        station: &GroundStation,
        time: DateTime<Utc>,
    ) -> Option<FsoLinkQuality> {
        let look_angles = satellite_state.look_angles_from_station(
            station.position.latitude_deg,
            station.position.longitude_deg,
            station.position.elevation_m,
        );

        if look_angles.elevation_deg < defaults::MIN_ELEVATION_DEG {
            return None;
        }

        // Simplified atmospheric transmission
        let zenith_angle = 90.0 - look_angles.elevation_deg;
        let airmass = 1.0 / (zenith_angle.to_radians().cos());
        let atmospheric_transmission = (-0.1 * airmass).exp();

        // Free space loss
        let free_space_loss_db = 20.0 * (look_angles.range_km * 1000.0).log10()
            + 20.0 * (self.wavelength_nm * 1e-9).log10()
            - 147.55;

        // Link margin calculation
        let transmit_power_dbm = 10.0 * self.transmit_power_w.log10() + 30.0;
        let receiver_sensitivity_dbm = -40.0;
        let link_margin_db = transmit_power_dbm - receiver_sensitivity_dbm - free_space_loss_db;

        // Throughput estimation
        let throughput_factor = (link_margin_db / 20.0).min(1.0).max(0.0);
        let estimated_throughput_gbps = 400.0 * throughput_factor * atmospheric_transmission;

        Some(FsoLinkQuality {
            satellite_id: satellite_state.satellite_id.clone(),
            station_id: station.station_id.clone(),
            timestamp: time,
            elevation_angle_deg: look_angles.elevation_deg,
            azimuth_angle_deg: look_angles.azimuth_deg,
            range_km: look_angles.range_km,
            atmospheric_transmission,
            link_margin_db,
            estimated_throughput_gbps,
            weather_impact_factor: 0.9, // Assume good weather
        })
    }
}

impl Default for FsoAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
