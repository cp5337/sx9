//! Configuration system for orbital mechanics
//!
//! Provides configurable constellation parameters, ground station networks,
//! and orbital mechanics settings.

use crate::error::{OrbitalMechanicsError, Result};
use crate::propagator::PropagatorType;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Main configuration structure for constellation design
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstellationConfig {
    /// Constellation metadata
    pub name: String,
    pub description: String,
    pub version: String,

    /// Constellation type and parameters
    pub constellation_type: ConstellationType,
    pub orbital_parameters: OrbitalParameters,

    /// Satellite specifications
    pub satellite_config: SatelliteConfig,

    /// Ground station network configuration
    pub ground_station_config: GroundStationConfig,

    /// Propagator and analysis settings
    pub analysis_config: AnalysisConfig,

    /// FSO link parameters
    pub fso_config: FsoConfig,
}

/// Types of supported constellations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstellationType {
    /// Walker Delta constellation (satellites/planes/phasing)
    WalkerDelta {
        total_satellites: usize,
        num_planes: usize,
        satellites_per_plane: usize,
        phasing_parameter: usize,
    },
    /// Custom satellite positions
    Custom {
        satellites: Vec<CustomSatellitePosition>,
    },
    /// Predefined constellation patterns
    Predefined { pattern: PredefinedPattern },
}

/// Predefined constellation patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PredefinedPattern {
    LaserLightFsoMeo,
    GlobalStarlink,
    OneWebLeo,
    CustomMeo,
}

/// Custom satellite orbital position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSatellitePosition {
    pub satellite_id: String,
    pub name: String,
    pub semi_major_axis_km: f64,
    pub eccentricity: f64,
    pub inclination_deg: f64,
    pub longitude_of_ascending_node_deg: f64,
    pub argument_of_perigee_deg: f64,
    pub mean_anomaly_deg: f64,
}

/// Orbital parameters for the constellation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalParameters {
    /// Reference altitude in kilometers
    pub altitude_km: f64,

    /// Orbital inclination in degrees
    pub inclination_deg: f64,

    /// Eccentricity (0.0 for circular)
    pub eccentricity: f64,

    /// Right ascension of ascending node spacing
    pub raan_spacing_deg: f64,

    /// Argument of perigee
    pub argument_of_perigee_deg: f64,

    /// Phase spacing between planes
    pub phase_spacing_deg: f64,
}

/// Satellite hardware and capability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatelliteConfig {
    /// Satellite mass in kg
    pub mass_kg: f64,

    /// Power generation capability in watts
    pub power_generation_w: f64,

    /// Communication capabilities
    pub communication_config: CommunicationConfig,

    /// Pointing accuracy in degrees
    pub pointing_accuracy_deg: f64,

    /// Operational lifetime in years
    pub lifetime_years: f64,
}

/// Communication system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationConfig {
    /// FSO (Free Space Optical) capabilities
    pub fso_enabled: bool,
    pub fso_wavelength_nm: f64,
    pub fso_transmit_power_w: f64,
    pub fso_beam_divergence_urad: f64,

    /// RF (Radio Frequency) backup
    pub rf_enabled: bool,
    pub rf_frequency_ghz: f64,
    pub rf_transmit_power_w: f64,

    /// Data rates
    pub max_data_rate_gbps: f64,
    pub min_data_rate_mbps: f64,
}

/// Ground station network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundStationConfig {
    /// Predefined ground station sets
    pub use_predefined_stations: bool,
    pub predefined_set: Option<PredefinedStationSet>,

    /// Custom ground stations
    pub custom_stations: Vec<CustomGroundStation>,

    /// Ground station capabilities
    pub default_capabilities: GroundStationCapabilities,
}

/// Predefined ground station sets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PredefinedStationSet {
    Ctas7Network257Stations,
    UsSpaceForceNetwork,
    NasaDeepSpaceNetwork,
    GlobalCommercialNetwork,
    Custom,
}

/// Custom ground station definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomGroundStation {
    pub station_id: String,
    pub name: String,
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub elevation_m: f64,
    pub capabilities: Option<GroundStationCapabilities>,
}

/// Ground station capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundStationCapabilities {
    pub fso_enabled: bool,
    pub rf_enabled: bool,
    pub minimum_elevation_deg: f64,
    pub maximum_range_km: f64,
    pub tracking_accuracy_deg: f64,
    pub weather_resilience_factor: f64,
}

/// Analysis and simulation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Orbital propagator type
    pub propagator_type: PropagatorType,

    /// Time step for calculations
    pub time_step_seconds: f64,

    /// Maximum propagation duration
    pub max_propagation_hours: f64,

    /// Atmospheric model
    pub atmospheric_model: AtmosphericModel,

    /// Earth model
    pub earth_model: EarthModel,
}

/// Atmospheric models for FSO analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AtmosphericModel {
    Standard,
    Tropical,
    MidLatitudeSummer,
    MidLatitudeWinter,
    SubArcticSummer,
    SubArcticWinter,
    Custom { parameters: AtmosphericParameters },
}

/// Custom atmospheric parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphericParameters {
    pub sea_level_pressure_hpa: f64,
    pub sea_level_temperature_k: f64,
    pub relative_humidity_percent: f64,
    pub visibility_km: f64,
    pub cloud_cover_percent: f64,
}

/// Earth models for orbital calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EarthModel {
    Wgs84,
    Grs80,
    Sphere,
    Custom {
        equatorial_radius_km: f64,
        flattening: f64,
        gravitational_parameter_km3_per_s2: f64,
    },
}

/// FSO link analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsoConfig {
    /// Wavelength in nanometers
    pub wavelength_nm: f64,

    /// Transmitter configuration
    pub transmitter: FsoTransmitterConfig,

    /// Receiver configuration
    pub receiver: FsoReceiverConfig,

    /// Link budget parameters
    pub link_budget: FsoLinkBudget,

    /// Atmospheric effects
    pub atmospheric_effects: bool,
    pub turbulence_model: TurbulenceModel,
}

/// FSO transmitter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsoTransmitterConfig {
    pub power_w: f64,
    pub beam_divergence_urad: f64,
    pub pointing_accuracy_urad: f64,
    pub efficiency: f64,
}

/// FSO receiver configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsoReceiverConfig {
    pub aperture_diameter_m: f64,
    pub field_of_view_urad: f64,
    pub quantum_efficiency: f64,
    pub noise_equivalent_power_w: f64,
}

/// FSO link budget parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsoLinkBudget {
    pub required_snr_db: f64,
    pub link_margin_db: f64,
    pub modulation_loss_db: f64,
    pub system_losses_db: f64,
}

/// Atmospheric turbulence models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TurbulenceModel {
    HufnagelValley,
    ClearAir,
    ModerateClimate,
    Strong,
    Custom { cn2_profile: Vec<f64> },
}

impl ConstellationConfig {
    /// Create default configuration for LaserLight FSO MEO constellation
    pub fn laserlight_fso_meo() -> Self {
        Self {
            name: "LaserLight FSO MEO".to_string(),
            description: "12-satellite MEO constellation for optical communications".to_string(),
            version: "1.0.0".to_string(),

            constellation_type: ConstellationType::WalkerDelta {
                total_satellites: 12,
                num_planes: 3,
                satellites_per_plane: 4,
                phasing_parameter: 1,
            },

            orbital_parameters: OrbitalParameters {
                altitude_km: 8000.0,
                inclination_deg: 55.0,
                eccentricity: 0.0001,
                raan_spacing_deg: 120.0, // 360/3 planes
                argument_of_perigee_deg: 0.0,
                phase_spacing_deg: 30.0, // 360/(3*4) satellites
            },

            satellite_config: SatelliteConfig {
                mass_kg: 500.0,
                power_generation_w: 2000.0,
                communication_config: CommunicationConfig {
                    fso_enabled: true,
                    fso_wavelength_nm: 1550.0,
                    fso_transmit_power_w: 1.0,
                    fso_beam_divergence_urad: 10.0,
                    rf_enabled: true,
                    rf_frequency_ghz: 26.5,
                    rf_transmit_power_w: 50.0,
                    max_data_rate_gbps: 400.0,
                    min_data_rate_mbps: 100.0,
                },
                pointing_accuracy_deg: 0.01,
                lifetime_years: 10.0,
            },

            ground_station_config: GroundStationConfig {
                use_predefined_stations: true,
                predefined_set: Some(PredefinedStationSet::Ctas7Network257Stations),
                custom_stations: Vec::new(),
                default_capabilities: GroundStationCapabilities {
                    fso_enabled: true,
                    rf_enabled: true,
                    minimum_elevation_deg: 10.0,
                    maximum_range_km: 50000.0,
                    tracking_accuracy_deg: 0.1,
                    weather_resilience_factor: 0.9,
                },
            },

            analysis_config: AnalysisConfig {
                propagator_type: PropagatorType::Sgp4,
                time_step_seconds: 60.0,
                max_propagation_hours: 168.0, // 1 week
                atmospheric_model: AtmosphericModel::Standard,
                earth_model: EarthModel::Wgs84,
            },

            fso_config: FsoConfig {
                wavelength_nm: 1550.0,
                transmitter: FsoTransmitterConfig {
                    power_w: 1.0,
                    beam_divergence_urad: 10.0,
                    pointing_accuracy_urad: 1.0,
                    efficiency: 0.8,
                },
                receiver: FsoReceiverConfig {
                    aperture_diameter_m: 0.3,
                    field_of_view_urad: 100.0,
                    quantum_efficiency: 0.7,
                    noise_equivalent_power_w: 1e-15,
                },
                link_budget: FsoLinkBudget {
                    required_snr_db: 15.0,
                    link_margin_db: 6.0,
                    modulation_loss_db: 3.0,
                    system_losses_db: 5.0,
                },
                atmospheric_effects: true,
                turbulence_model: TurbulenceModel::HufnagelValley,
            },
        }
    }

    /// Create custom MEO constellation configuration
    pub fn custom_meo(
        num_satellites: usize,
        altitude_km: f64,
        inclination_deg: f64,
        num_planes: usize,
    ) -> Self {
        let satellites_per_plane = num_satellites / num_planes;
        let mut config = Self::laserlight_fso_meo();

        config.name = "Custom MEO Constellation".to_string();
        config.constellation_type = ConstellationType::WalkerDelta {
            total_satellites: num_satellites,
            num_planes,
            satellites_per_plane,
            phasing_parameter: 1,
        };

        config.orbital_parameters.altitude_km = altitude_km;
        config.orbital_parameters.inclination_deg = inclination_deg;
        config.orbital_parameters.raan_spacing_deg = 360.0 / num_planes as f64;
        config.orbital_parameters.phase_spacing_deg = 360.0 / num_satellites as f64;

        config
    }

    /// Create configuration with custom satellite positions
    pub fn from_custom_positions(satellites: Vec<CustomSatellitePosition>) -> Self {
        let mut config = Self::laserlight_fso_meo();

        config.name = "Custom Positioned Constellation".to_string();
        config.constellation_type = ConstellationType::Custom { satellites };

        config
    }
}

impl Default for ConstellationConfig {
    fn default() -> Self {
        Self::laserlight_fso_meo()
    }
}

/// Load constellation configuration from JSON file
pub fn load_constellation_config<P: AsRef<Path>>(path: P) -> Result<ConstellationConfig> {
    let content = fs::read_to_string(path).map_err(|e| {
        OrbitalMechanicsError::ConfigError(format!("Failed to read config file: {}", e))
    })?;

    serde_json::from_str(&content)
        .map_err(|e| OrbitalMechanicsError::ConfigError(format!("Failed to parse config: {}", e)))
}

/// Save constellation configuration to JSON file
pub fn save_constellation_config<P: AsRef<Path>>(
    config: &ConstellationConfig,
    path: P,
) -> Result<()> {
    let content = serde_json::to_string_pretty(config).map_err(|e| {
        OrbitalMechanicsError::ConfigError(format!("Failed to serialize config: {}", e))
    })?;

    fs::write(path, content).map_err(|e| {
        OrbitalMechanicsError::ConfigError(format!("Failed to write config file: {}", e))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = ConstellationConfig::default();
        assert_eq!(config.name, "LaserLight FSO MEO");

        if let ConstellationType::WalkerDelta {
            total_satellites, ..
        } = config.constellation_type
        {
            assert_eq!(total_satellites, 12);
        } else {
            panic!("Expected WalkerDelta constellation type");
        }
    }

    #[test]
    fn test_custom_meo_config() {
        let config = ConstellationConfig::custom_meo(8, 10000.0, 60.0, 2);

        if let ConstellationType::WalkerDelta {
            total_satellites,
            num_planes,
            ..
        } = config.constellation_type
        {
            assert_eq!(total_satellites, 8);
            assert_eq!(num_planes, 2);
        } else {
            panic!("Expected WalkerDelta constellation type");
        }

        assert_eq!(config.orbital_parameters.altitude_km, 10000.0);
        assert_eq!(config.orbital_parameters.inclination_deg, 60.0);
    }

    #[test]
    fn test_config_serialization() {
        let config = ConstellationConfig::default();
        let json = serde_json::to_string_pretty(&config).unwrap();
        assert!(json.contains("LaserLight FSO MEO"));

        let deserialized: ConstellationConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, config.name);
    }

    #[test]
    fn test_config_file_operations() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_config.json");

        let config = ConstellationConfig::default();
        save_constellation_config(&config, &file_path).unwrap();

        let loaded_config = load_constellation_config(&file_path).unwrap();
        assert_eq!(loaded_config.name, config.name);
    }
}
