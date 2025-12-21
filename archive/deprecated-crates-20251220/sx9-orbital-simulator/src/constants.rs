//! Physical and mathematical constants for orbital mechanics

use std::f64::consts::PI;

/// Mathematical constants
pub const TWO_PI: f64 = 2.0 * PI;
pub const HALF_PI: f64 = PI / 2.0;
pub const DEG_TO_RAD: f64 = PI / 180.0;
pub const RAD_TO_DEG: f64 = 180.0 / PI;

/// Earth physical constants (WGS84)
pub const EARTH_RADIUS_KM: f64 = 6378.137; // Equatorial radius
pub const EARTH_POLAR_RADIUS_KM: f64 = 6356.7523142; // Polar radius
pub const EARTH_FLATTENING: f64 = 1.0 / 298.257223563; // WGS84 flattening
pub const EARTH_MU: f64 = 398600.4418; // Gravitational parameter km³/s²
pub const EARTH_J2: f64 = 1.08262668e-3; // Second zonal harmonic
pub const EARTH_ROTATION_RATE: f64 = 7.2921159e-5; // rad/s

/// Time constants
pub const SIDEREAL_DAY_SECONDS: f64 = 86164.0905; // seconds
pub const SOLAR_DAY_SECONDS: f64 = 86400.0; // seconds
pub const JULIAN_CENTURY_DAYS: f64 = 36525.0;
pub const J2000_EPOCH_JD: f64 = 2451545.0; // Julian Day for J2000.0 epoch

/// Unit conversions
pub const KM_TO_M: f64 = 1000.0;
pub const M_TO_KM: f64 = 1.0 / 1000.0;
pub const HOURS_TO_SECONDS: f64 = 3600.0;
pub const DAYS_TO_SECONDS: f64 = 86400.0;
pub const ARCSEC_TO_RAD: f64 = PI / (180.0 * 3600.0);

/// Physical constants
pub const SPEED_OF_LIGHT: f64 = 299792458.0; // m/s
pub const BOLTZMANN_CONSTANT: f64 = 1.380649e-23; // J/K
pub const PLANCK_CONSTANT: f64 = 6.62607015e-34; // J⋅s

/// Atmospheric constants
pub const SEA_LEVEL_PRESSURE_PA: f64 = 101325.0; // Pascal
pub const SEA_LEVEL_TEMPERATURE_K: f64 = 288.15; // Kelvin
pub const ATMOSPHERIC_SCALE_HEIGHT_KM: f64 = 8.5; // km
pub const DRY_AIR_GAS_CONSTANT: f64 = 287.052; // J/(kg⋅K)

/// Orbit classification by altitude
pub const LEO_MIN_ALTITUDE_KM: f64 = 160.0; // Minimum sustainable LEO
pub const LEO_MAX_ALTITUDE_KM: f64 = 2000.0; // LEO to MEO boundary
pub const MEO_MIN_ALTITUDE_KM: f64 = 2000.0; // MEO lower boundary
pub const MEO_MAX_ALTITUDE_KM: f64 = 35786.0; // MEO to GEO boundary
pub const GEO_ALTITUDE_KM: f64 = 35786.0; // Geostationary orbit
pub const HEO_MIN_ALTITUDE_KM: f64 = 35786.0; // High Earth Orbit

/// Common orbital altitudes
pub const ISS_ALTITUDE_KM: f64 = 408.0; // International Space Station
pub const STARLINK_ALTITUDE_KM: f64 = 550.0; // Starlink constellation
pub const GPS_ALTITUDE_KM: f64 = 20200.0; // GPS satellites
pub const LASERLIGHT_FSO_ALTITUDE_KM: f64 = 8000.0; // LaserLight FSO MEO

/// Propagation and numerical constants
pub const SGP4_MAX_DAYS: f64 = 365.25; // Maximum SGP4 propagation period
pub const KEPLER_ITERATION_LIMIT: usize = 50; // Maximum iterations for Kepler's equation
pub const KEPLER_TOLERANCE: f64 = 1e-12; // Convergence tolerance
pub const NEWTON_RAPHSON_TOLERANCE: f64 = 1e-15; // Newton-Raphson tolerance

/// Free Space Optical (FSO) constants
pub const FSO_WAVELENGTH_1550NM: f64 = 1550e-9; // meters
pub const FSO_WAVELENGTH_1064NM: f64 = 1064e-9; // meters
pub const FSO_BEAM_DIVERGENCE_TYPICAL: f64 = 10e-6; // radians
pub const FSO_ATMOSPHERIC_ATTENUATION_CLEAR: f64 = 0.1; // dB/km at 1550nm

/// Radio frequency constants
pub const RF_FREQUENCY_L_BAND: f64 = 1.5e9; // Hz
pub const RF_FREQUENCY_S_BAND: f64 = 2.4e9; // Hz
pub const RF_FREQUENCY_C_BAND: f64 = 6.0e9; // Hz
pub const RF_FREQUENCY_X_BAND: f64 = 10.0e9; // Hz
pub const RF_FREQUENCY_KU_BAND: f64 = 14.0e9; // Hz
pub const RF_FREQUENCY_KA_BAND: f64 = 26.5e9; // Hz

/// Ground station parameters
pub const MIN_ELEVATION_ANGLE_DEG: f64 = 10.0; // Typical minimum elevation
pub const EARTH_HORIZON_ANGLE_DEG: f64 = 0.0; // Geometric horizon
pub const ATMOSPHERIC_REFRACTION_DEG: f64 = 0.6; // Typical atmospheric refraction

/// Error tolerances for calculations
pub const POSITION_TOLERANCE_KM: f64 = 1e-6; // Position accuracy
pub const VELOCITY_TOLERANCE_KM_S: f64 = 1e-9; // Velocity accuracy
pub const ANGLE_TOLERANCE_RAD: f64 = 1e-12; // Angular accuracy
pub const TIME_TOLERANCE_SECONDS: f64 = 1e-6; // Time accuracy

/// Maximum values for validation
pub const MAX_ECCENTRICITY: f64 = 0.99; // Maximum allowed eccentricity
pub const MAX_INCLINATION_DEG: f64 = 180.0; // Maximum inclination
pub const MAX_SEMI_MAJOR_AXIS_KM: f64 = 1e8; // Maximum semi-major axis
pub const MIN_SEMI_MAJOR_AXIS_KM: f64 = EARTH_RADIUS_KM + 160.0; // Minimum altitude

/// Default configuration values
pub mod defaults {
    /// Default propagation time step
    pub const TIME_STEP_SECONDS: f64 = 60.0;

    /// Default simulation duration
    pub const SIMULATION_DURATION_HOURS: f64 = 24.0;

    /// Default ground station minimum elevation
    pub const MIN_ELEVATION_DEG: f64 = 10.0;

    /// Default FSO transmit power
    pub const FSO_TRANSMIT_POWER_W: f64 = 1.0;

    /// Default FSO receiver aperture
    pub const FSO_RECEIVER_APERTURE_M: f64 = 0.3;

    /// Default atmospheric visibility
    pub const ATMOSPHERIC_VISIBILITY_KM: f64 = 23.0;
}

/// Validation functions for orbital parameters
pub mod validation {
    use super::*;
    use crate::error::{OrbitalMechanicsError, Result};

    /// Validate semi-major axis
    pub fn validate_semi_major_axis(a_km: f64) -> Result<()> {
        if a_km < MIN_SEMI_MAJOR_AXIS_KM || a_km > MAX_SEMI_MAJOR_AXIS_KM {
            return Err(OrbitalMechanicsError::invalid_elements(
                format!("Semi-major axis {:.1} km outside valid range [{:.1}, {:.1}] km",
                    a_km, MIN_SEMI_MAJOR_AXIS_KM, MAX_SEMI_MAJOR_AXIS_KM)
            ));
        }
        Ok(())
    }

    /// Validate eccentricity
    pub fn validate_eccentricity(e: f64) -> Result<()> {
        if e < 0.0 || e >= MAX_ECCENTRICITY {
            return Err(OrbitalMechanicsError::invalid_elements(
                format!("Eccentricity {:.6} outside valid range [0.0, {:.2})",
                    e, MAX_ECCENTRICITY)
            ));
        }
        Ok(())
    }

    /// Validate inclination
    pub fn validate_inclination(i_deg: f64) -> Result<()> {
        if i_deg < 0.0 || i_deg > MAX_INCLINATION_DEG {
            return Err(OrbitalMechanicsError::invalid_elements(
                format!("Inclination {:.3}° outside valid range [0.0, {:.1}]°",
                    i_deg, MAX_INCLINATION_DEG)
            ));
        }
        Ok(())
    }

    /// Validate angle in degrees (0-360)
    pub fn validate_angle_0_360(angle_deg: f64, name: &str) -> Result<()> {
        if angle_deg < 0.0 || angle_deg >= 360.0 {
            return Err(OrbitalMechanicsError::invalid_elements(
                format!("{} {:.3}° outside valid range [0.0, 360.0)°", name, angle_deg)
            ));
        }
        Ok(())
    }

    /// Validate all basic orbital elements
    pub fn validate_orbital_elements(
        a_km: f64,
        e: f64,
        i_deg: f64,
        raan_deg: f64,
        arg_per_deg: f64,
        mean_anom_deg: f64,
    ) -> Result<()> {
        validate_semi_major_axis(a_km)?;
        validate_eccentricity(e)?;
        validate_inclination(i_deg)?;
        validate_angle_0_360(raan_deg, "RAAN")?;
        validate_angle_0_360(arg_per_deg, "Argument of perigee")?;
        validate_angle_0_360(mean_anom_deg, "Mean anomaly")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::validation::*;

    #[test]
    fn test_constants() {
        assert!((TWO_PI - 2.0 * PI).abs() < 1e-15);
        assert!((DEG_TO_RAD * 180.0 - PI).abs() < 1e-15);
        assert!((RAD_TO_DEG * PI - 180.0).abs() < 1e-15);
    }

    #[test]
    fn test_validation_functions() {
        // Valid orbital elements
        assert!(validate_semi_major_axis(7000.0).is_ok());
        assert!(validate_eccentricity(0.01).is_ok());
        assert!(validate_inclination(55.0).is_ok());
        assert!(validate_angle_0_360(45.0, "test").is_ok());

        // Invalid orbital elements
        assert!(validate_semi_major_axis(1000.0).is_err()); // Too low
        assert!(validate_eccentricity(-0.1).is_err()); // Negative
        assert!(validate_inclination(200.0).is_err()); // Too high
        assert!(validate_angle_0_360(400.0, "test").is_err()); // Out of range
    }

    #[test]
    fn test_complete_orbital_validation() {
        // Valid LEO orbit
        assert!(validate_orbital_elements(
            EARTH_RADIUS_KM + 400.0, // ISS-like altitude
            0.001,  // Nearly circular
            51.6,   // ISS inclination
            0.0,    // RAAN
            0.0,    // Argument of perigee
            0.0     // Mean anomaly
        ).is_ok());

        // Invalid orbit (too low altitude)
        assert!(validate_orbital_elements(
            EARTH_RADIUS_KM + 100.0, // Below atmospheric limit
            0.0,
            28.5,
            0.0,
            0.0,
            0.0
        ).is_err());
    }
}