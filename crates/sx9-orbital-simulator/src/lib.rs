//! CTAS-7 Orbital Mechanics Crate
//!
//! A modular and configurable orbital mechanics library for satellite constellation
//! management, orbital propagation, and visibility calculations.
//!
//! This crate provides tools for:
//! - Orbital mechanics calculations
//! - Satellite constellation design and optimization
//! - Ground station visibility analysis
//! - Free-space optical (FSO) link analysis
//! - Custom MEO satellite positioning

// Local orbital types (self-contained implementation)
pub mod ground_station;
pub use ground_station::{GroundStation, GroundStationNetwork, StationPosition};

// Local modules
pub mod config;
pub mod constants;
pub mod constellation;
pub mod coordinates;
pub mod error;
pub mod fso_analysis;
pub mod orbit;
pub mod propagator;
pub mod satellite_simulator;
pub mod visibility;

// Re-exports
pub use config::{
    load_constellation_config, save_constellation_config, ConstellationConfig as Config,
    ConstellationConfig, ConstellationType,
};
pub use constellation::Constellation;
pub use coordinates::{CoordinateSystem, Position3D};
pub use error::{OrbitalMechanicsError, Result};
pub use fso_analysis::{FsoAnalyzer, FsoLinkQuality};
pub use orbit::{
    GeodeticPosition, LookAngles, OrbitClassification, OrbitalElements, OrbitalElementsRad,
    SatelliteOrbit, SatelliteState,
};
pub use propagator::{OrbitalPropagator, PropagatorType};
pub use satellite_simulator::{
    LiveSatellite, MeoEnvironmentalConditions, ObstructionWarning, SatelliteSimulator,
    SatelliteUnicodePacket, SimulationStatistics,
};
pub use visibility::{VisibilityCalculator, VisibilityWindow};

/// Main orbital mechanics engine with live satellite simulation
pub struct OrbitalMechanicsEngine {
    constellation: Constellation,
    ground_stations: GroundStationNetwork,
    propagator: Box<dyn OrbitalPropagator>,
    fso_analyzer: FsoAnalyzer,
    /// OPERATIONAL: Live satellite simulator with Unicode packet generation
    satellite_simulator: Option<SatelliteSimulator>,
}

impl OrbitalMechanicsEngine {
    /// Create new orbital mechanics engine with default configuration
    pub fn new() -> Result<Self> {
        let config = Config::default();
        Self::with_config(config)
    }

    /// Create orbital mechanics engine with custom configuration
    pub fn with_config(config: Config) -> Result<Self> {
        let constellation = Constellation::from_config(&config)?;
        let ground_stations = GroundStationNetwork::new();
        let propagator = propagator::create_propagator(config.analysis_config.propagator_type)?;
        let fso_analyzer = FsoAnalyzer::new();

        Ok(Self {
            constellation,
            ground_stations,
            propagator,
            fso_analyzer,
            satellite_simulator: None,
        })
    }

    /// Load configuration from file
    pub fn from_config_file(path: &str) -> Result<Self> {
        let config = load_constellation_config(path)?;
        Self::with_config(config)
    }

    /// Get constellation reference
    pub fn constellation(&self) -> &Constellation {
        &self.constellation
    }

    /// Get ground station network reference
    pub fn ground_stations(&self) -> &GroundStationNetwork {
        &self.ground_stations
    }

    /// Add custom satellite to constellation
    pub fn add_satellite(&mut self, orbit: SatelliteOrbit) -> Result<()> {
        self.constellation.add_satellite(orbit)
    }

    /// Add ground station to network
    pub fn add_ground_station(&mut self, station: GroundStation) {
        self.ground_stations.add_station(station);
    }

    /// Calculate satellite position at given time
    pub fn satellite_position(
        &self,
        satellite_id: &str,
        time: chrono::DateTime<chrono::Utc>,
    ) -> Result<SatelliteState> {
        let orbit = self.constellation.get_satellite(satellite_id).ok_or(
            OrbitalMechanicsError::SatelliteNotFound(satellite_id.to_string()),
        )?;

        self.propagator.propagate(orbit, time)
    }

    /// Calculate visibility windows for all satellites and ground stations
    pub fn calculate_all_visibility_windows(
        &self,
        start_time: chrono::DateTime<chrono::Utc>,
        duration_hours: f64,
    ) -> Result<Vec<VisibilityWindow>> {
        let mut all_windows = Vec::new();
        let calculator = VisibilityCalculator::new();

        for satellite in self.constellation.satellites() {
            for station in self.ground_stations.stations() {
                let windows = calculator.calculate_windows(
                    satellite,
                    station,
                    start_time,
                    duration_hours,
                    &*self.propagator,
                )?;
                all_windows.extend(windows);
            }
        }

        Ok(all_windows)
    }

    /// Analyze FSO link quality between satellite and ground station
    pub fn analyze_fso_link(
        &self,
        satellite_id: &str,
        station_id: &str,
        time: chrono::DateTime<chrono::Utc>,
    ) -> Result<Option<FsoLinkQuality>> {
        let satellite_state = self.satellite_position(satellite_id, time)?;
        let station = self.ground_stations.get_station(station_id).ok_or(
            OrbitalMechanicsError::GroundStationNotFound(station_id.to_string()),
        )?;

        Ok(self
            .fso_analyzer
            .analyze_link(&satellite_state, station, time))
    }

    /// Generate constellation status report
    pub fn constellation_report(&self, time: chrono::DateTime<chrono::Utc>) -> Result<String> {
        self.constellation
            .generate_status_report(time, &*self.propagator)
    }

    /// OPERATIONAL: Enable live satellite simulation with Unicode packet generation
    pub fn enable_satellite_simulation(&mut self) -> Result<()> {
        let propagator = propagator::create_propagator(propagator::PropagatorType::Sgp4)?;
        let simulator = SatelliteSimulator::new(propagator);
        self.satellite_simulator = Some(simulator);

        tracing::info!("Live satellite simulation enabled with Unicode packet generation");
        Ok(())
    }

    /// OPERATIONAL: Add satellite to live simulation
    pub async fn add_live_satellite(
        &mut self,
        orbit: SatelliteOrbit,
        name: String,
        norad_id: Option<u32>,
    ) -> Result<uuid::Uuid> {
        if let Some(ref simulator) = self.satellite_simulator {
            simulator
                .add_satellite(orbit, name, norad_id)
                .await
                .map_err(|e| OrbitalMechanicsError::config_error(e.to_string()))
        } else {
            Err(OrbitalMechanicsError::config_error(
                "Satellite simulation not enabled",
            ))
        }
    }

    /// OPERATIONAL: Start real-time satellite simulation
    pub async fn start_live_simulation(&self) -> Result<()> {
        if let Some(ref simulator) = self.satellite_simulator {
            simulator
                .start_simulation()
                .await
                .map_err(|e| OrbitalMechanicsError::config_error(e.to_string()))
        } else {
            Err(OrbitalMechanicsError::config_error(
                "Satellite simulation not enabled",
            ))
        }
    }

    /// OPERATIONAL: Get all live satellites with current status
    pub async fn get_live_satellites(&self) -> Result<Vec<LiveSatellite>> {
        if let Some(ref simulator) = self.satellite_simulator {
            Ok(simulator.get_all_satellites().await)
        } else {
            Err(OrbitalMechanicsError::config_error(
                "Satellite simulation not enabled",
            ))
        }
    }

    /// OPERATIONAL: Get Unicode packet transmission history
    pub async fn get_unicode_packet_history(
        &self,
        limit: Option<usize>,
    ) -> Result<Vec<SatelliteUnicodePacket>> {
        if let Some(ref simulator) = self.satellite_simulator {
            Ok(simulator.get_unicode_packet_history(limit).await)
        } else {
            Err(OrbitalMechanicsError::config_error(
                "Satellite simulation not enabled",
            ))
        }
    }

    /// OPERATIONAL: Get live simulation statistics and performance metrics
    pub async fn get_simulation_statistics(&self) -> Result<SimulationStatistics> {
        if let Some(ref simulator) = self.satellite_simulator {
            Ok(simulator.get_simulation_statistics().await)
        } else {
            Err(OrbitalMechanicsError::config_error(
                "Satellite simulation not enabled",
            ))
        }
    }

    /// Check if satellite simulation is enabled
    pub fn is_simulation_enabled(&self) -> bool {
        self.satellite_simulator.is_some()
    }
}

impl Default for OrbitalMechanicsEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create default orbital mechanics engine")
    }
}

/// Convenience function to create a LaserLight FSO MEO constellation
pub fn create_laserlight_constellation() -> Result<OrbitalMechanicsEngine> {
    let config = Config::laserlight_fso_meo();
    OrbitalMechanicsEngine::with_config(config)
}

/// Convenience function to create a custom MEO constellation
pub fn create_custom_meo_constellation(
    num_satellites: usize,
    altitude_km: f64,
    inclination_deg: f64,
    num_planes: usize,
) -> Result<OrbitalMechanicsEngine> {
    let config = Config::custom_meo(num_satellites, altitude_km, inclination_deg, num_planes);
    OrbitalMechanicsEngine::with_config(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_engine_creation() {
        let engine = OrbitalMechanicsEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_laserlight_constellation() {
        let engine = create_laserlight_constellation();
        assert!(engine.is_ok());

        let engine = engine.unwrap();
        assert_eq!(engine.constellation().satellite_count(), 12);
    }

    #[test]
    fn test_custom_constellation() {
        let engine = create_custom_meo_constellation(8, 10000.0, 60.0, 2);
        assert!(engine.is_ok());

        let engine = engine.unwrap();
        assert_eq!(engine.constellation().satellite_count(), 8);
    }

    #[test]
    fn test_satellite_position_calculation() {
        let engine = create_laserlight_constellation().unwrap();
        let time = Utc::now();

        let satellites = engine.constellation().satellites();
        if let Some(satellite) = satellites.first() {
            let position = engine.satellite_position(&satellite.satellite_id, time);
            assert!(position.is_ok());
        }
    }
}
pub mod foundation_integration;
