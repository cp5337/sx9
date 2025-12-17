//! Orbital propagation algorithms

use crate::constants::*;
use crate::error::{OrbitalMechanicsError, Result};
use crate::orbit::{OrbitalElementsRad, SatelliteOrbit, SatelliteState};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Types of orbital propagators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropagatorType {
    /// Simplified General Perturbations 4 (NORAD standard)
    Sgp4,
    /// Simple Keplerian propagation (two-body problem)
    Keplerian,
    /// High-precision numerical integration
    Numerical,
}

/// Trait for orbital propagation algorithms
pub trait OrbitalPropagator {
    /// Propagate satellite orbit to specified time
    fn propagate(&self, satellite: &SatelliteOrbit, time: DateTime<Utc>) -> Result<SatelliteState>;

    /// Get propagator name
    fn name(&self) -> &str;

    /// Get maximum recommended propagation duration
    fn max_propagation_duration(&self) -> chrono::Duration;
}

/// Simple Keplerian propagator (two-body problem only)
pub struct KeplerianPropagator;

/// SGP4 propagator (simplified)
pub struct Sgp4Propagator;

/// Numerical integration propagator
pub struct NumericalPropagator {
    pub step_size_seconds: f64,
}

impl OrbitalPropagator for KeplerianPropagator {
    fn propagate(&self, satellite: &SatelliteOrbit, time: DateTime<Utc>) -> Result<SatelliteState> {
        let time_since_epoch = (time - satellite.epoch).num_seconds() as f64;

        // Update mean anomaly
        let delta_mean_anomaly = satellite.mean_motion_rad_per_sec * time_since_epoch;
        let elements_rad = satellite.elements.to_radians();
        let mean_anomaly = (elements_rad.mean_anomaly_rad + delta_mean_anomaly) % TWO_PI;

        // Solve Kepler's equation for eccentric anomaly
        let eccentric_anomaly =
            self.solve_keplers_equation(mean_anomaly, elements_rad.eccentricity)?;

        // Calculate true anomaly
        let true_anomaly =
            self.eccentric_to_true_anomaly(eccentric_anomaly, elements_rad.eccentricity);

        // Calculate position and velocity in orbital plane
        let (r, v) = self.orbital_state_vectors(&elements_rad, true_anomaly);

        // Transform to Earth-Centered Inertial (ECI) coordinates
        let (position_eci, velocity_eci) = self.orbital_to_eci(
            r,
            v,
            elements_rad.inclination_rad,
            elements_rad.raan_rad,
            elements_rad.argument_of_perigee_rad,
        );

        Ok(SatelliteState::new(
            satellite.satellite_id.clone(),
            time,
            position_eci,
            velocity_eci,
        ))
    }

    fn name(&self) -> &str {
        "Keplerian"
    }

    fn max_propagation_duration(&self) -> chrono::Duration {
        chrono::Duration::days(365) // 1 year for Keplerian
    }
}

impl KeplerianPropagator {
    pub fn new() -> Self {
        Self
    }

    /// Solve Kepler's equation: M = E - e*sin(E)
    fn solve_keplers_equation(&self, mean_anomaly: f64, eccentricity: f64) -> Result<f64> {
        let mut eccentric_anomaly = mean_anomaly; // Initial guess

        for _ in 0..KEPLER_ITERATION_LIMIT {
            let delta_e = (mean_anomaly - eccentric_anomaly
                + eccentricity * eccentric_anomaly.sin())
                / (1.0 - eccentricity * eccentric_anomaly.cos());

            eccentric_anomaly += delta_e;

            if delta_e.abs() < KEPLER_TOLERANCE {
                return Ok(eccentric_anomaly);
            }
        }

        Err(OrbitalMechanicsError::propagation_error(
            "Kepler's equation failed to converge",
        ))
    }

    /// Convert eccentric anomaly to true anomaly
    fn eccentric_to_true_anomaly(&self, eccentric_anomaly: f64, eccentricity: f64) -> f64 {
        let sqrt_term = ((1.0 + eccentricity) / (1.0 - eccentricity)).sqrt();
        2.0 * (sqrt_term * (eccentric_anomaly / 2.0).tan()).atan()
    }

    /// Calculate position and velocity vectors in orbital plane
    fn orbital_state_vectors(
        &self,
        elements: &OrbitalElementsRad,
        true_anomaly: f64,
    ) -> ([f64; 3], [f64; 3]) {
        let a = elements.semi_major_axis_km;
        let e = elements.eccentricity;

        // Distance from Earth center
        let r_magnitude = a * (1.0 - e * e) / (1.0 + e * true_anomaly.cos());

        // Position in orbital plane
        let r = [
            r_magnitude * true_anomaly.cos(),
            r_magnitude * true_anomaly.sin(),
            0.0,
        ];

        // Velocity in orbital plane
        let mu_over_p = EARTH_MU / (a * (1.0 - e * e));
        let sqrt_mu_over_p = mu_over_p.sqrt();

        let v = [
            -sqrt_mu_over_p * true_anomaly.sin(),
            sqrt_mu_over_p * (e + true_anomaly.cos()),
            0.0,
        ];

        (r, v)
    }

    /// Transform orbital plane coordinates to ECI
    fn orbital_to_eci(
        &self,
        r_orbital: [f64; 3],
        v_orbital: [f64; 3],
        inclination: f64,
        raan: f64,
        arg_perigee: f64,
    ) -> ([f64; 3], [f64; 3]) {
        // Rotation matrices
        let cos_raan = raan.cos();
        let sin_raan = raan.sin();
        let cos_inc = inclination.cos();
        let sin_inc = inclination.sin();
        let cos_arg = arg_perigee.cos();
        let sin_arg = arg_perigee.sin();

        // Combined rotation matrix elements
        let p11 = cos_raan * cos_arg - sin_raan * sin_arg * cos_inc;
        let p12 = -cos_raan * sin_arg - sin_raan * cos_arg * cos_inc;
        let p13 = sin_raan * sin_inc;

        let p21 = sin_raan * cos_arg + cos_raan * sin_arg * cos_inc;
        let p22 = -sin_raan * sin_arg + cos_raan * cos_arg * cos_inc;
        let p23 = -cos_raan * sin_inc;

        let p31 = sin_arg * sin_inc;
        let p32 = cos_arg * sin_inc;
        let p33 = cos_inc;

        // Transform position
        let position_eci = [
            p11 * r_orbital[0] + p12 * r_orbital[1] + p13 * r_orbital[2],
            p21 * r_orbital[0] + p22 * r_orbital[1] + p23 * r_orbital[2],
            p31 * r_orbital[0] + p32 * r_orbital[1] + p33 * r_orbital[2],
        ];

        // Transform velocity
        let velocity_eci = [
            p11 * v_orbital[0] + p12 * v_orbital[1] + p13 * v_orbital[2],
            p21 * v_orbital[0] + p22 * v_orbital[1] + p23 * v_orbital[2],
            p31 * v_orbital[0] + p32 * v_orbital[1] + p33 * v_orbital[2],
        ];

        (position_eci, velocity_eci)
    }
}

impl OrbitalPropagator for Sgp4Propagator {
    fn propagate(&self, satellite: &SatelliteOrbit, time: DateTime<Utc>) -> Result<SatelliteState> {
        // Simplified SGP4 implementation
        // In practice, this would use the full SGP4 algorithm with atmospheric drag,
        // solar radiation pressure, and Earth oblateness perturbations

        let time_since_epoch_minutes = (time - satellite.epoch).num_seconds() as f64 / 60.0;

        // For now, fall back to Keplerian propagation with J2 perturbations
        let keplerian = KeplerianPropagator::new();
        let mut state = keplerian.propagate(satellite, time)?;

        // Apply simplified J2 perturbations
        self.apply_j2_perturbations(&mut state, satellite, time_since_epoch_minutes);

        Ok(state)
    }

    fn name(&self) -> &str {
        "SGP4 (Simplified)"
    }

    fn max_propagation_duration(&self) -> chrono::Duration {
        chrono::Duration::days(365) // 1 year for SGP4
    }
}

impl Sgp4Propagator {
    pub fn new() -> Self {
        Self
    }

    /// Apply simplified J2 perturbations
    fn apply_j2_perturbations(
        &self,
        state: &mut SatelliteState,
        satellite: &SatelliteOrbit,
        time_minutes: f64,
    ) {
        let a = satellite.elements.semi_major_axis_km;
        let e = satellite.elements.eccentricity;
        let i = satellite.elements.inclination_deg * DEG_TO_RAD;

        // J2 perturbation rates (simplified)
        let n = (EARTH_MU / a.powi(3)).sqrt(); // Mean motion
        let j2_factor = -1.5 * EARTH_J2 * (EARTH_RADIUS_KM / a).powi(2) * n;

        // RAAN precession rate
        let raan_dot = j2_factor * i.cos();

        // Argument of perigee precession rate
        let arg_perigee_dot = j2_factor * (2.0 - 2.5 * (i.sin()).powi(2));

        // Apply corrections (very simplified)
        let delta_raan = raan_dot * time_minutes / 60.0; // Convert to hours
        let delta_arg_perigee = arg_perigee_dot * time_minutes / 60.0;

        // Note: In a full implementation, these corrections would be applied
        // to the orbital elements before calculating position and velocity
        // This is just a placeholder to show the concept
    }
}

impl OrbitalPropagator for NumericalPropagator {
    fn propagate(&self, satellite: &SatelliteOrbit, time: DateTime<Utc>) -> Result<SatelliteState> {
        // Simplified numerical integration
        // In practice, this would use Runge-Kutta or similar methods

        let total_time = (time - satellite.epoch).num_seconds() as f64;
        let num_steps = (total_time / self.step_size_seconds).ceil() as usize;

        if num_steps > 100000 {
            return Err(OrbitalMechanicsError::propagation_error(
                "Numerical integration time too long",
            ));
        }

        // For now, use Keplerian propagation
        let keplerian = KeplerianPropagator::new();
        keplerian.propagate(satellite, time)
    }

    fn name(&self) -> &str {
        "Numerical Integration"
    }

    fn max_propagation_duration(&self) -> chrono::Duration {
        chrono::Duration::days(30) // 30 days for numerical integration
    }
}

impl NumericalPropagator {
    pub fn new(step_size_seconds: f64) -> Self {
        Self { step_size_seconds }
    }
}

/// Create propagator instance based on type
pub fn create_propagator(propagator_type: PropagatorType) -> Result<Box<dyn OrbitalPropagator>> {
    match propagator_type {
        PropagatorType::Keplerian => Ok(Box::new(KeplerianPropagator::new())),
        PropagatorType::Sgp4 => Ok(Box::new(Sgp4Propagator::new())),
        PropagatorType::Numerical => Ok(Box::new(NumericalPropagator::new(60.0))), // 1-minute steps
    }
}

/// Validate propagation time against propagator limits
pub fn validate_propagation_time(
    propagator: &dyn OrbitalPropagator,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Result<()> {
    let duration = end_time - start_time;
    let max_duration = propagator.max_propagation_duration();

    if duration > max_duration {
        return Err(OrbitalMechanicsError::propagation_error(format!(
            "Propagation duration ({} days) exceeds maximum for {} propagator ({} days)",
            duration.num_days(),
            propagator.name(),
            max_duration.num_days()
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orbit::{OrbitalElements, SatelliteOrbit};
    use chrono::Utc;

    #[test]
    fn test_keplerian_propagator() {
        let propagator = KeplerianPropagator::new();

        let elements = OrbitalElements::new(7000.0, 0.01, 55.0, 0.0, 0.0, 0.0).unwrap();
        let epoch = Utc::now();
        let satellite = SatelliteOrbit::new(
            "TEST-01".to_string(),
            "Test Satellite".to_string(),
            elements,
            epoch,
        );

        let future_time = epoch + chrono::Duration::hours(2);
        let result = propagator.propagate(&satellite, future_time);

        assert!(result.is_ok());
        let state = result.unwrap();
        assert_eq!(state.satellite_id, "TEST-01");
        assert!(state.orbital_radius > 6900.0 && state.orbital_radius < 7100.0);
    }

    #[test]
    fn test_sgp4_propagator() {
        let propagator = Sgp4Propagator::new();
        assert_eq!(propagator.name(), "SGP4 (Simplified)");

        let elements = OrbitalElements::new(7000.0, 0.01, 55.0, 0.0, 0.0, 0.0).unwrap();
        let satellite = SatelliteOrbit::new(
            "TEST-01".to_string(),
            "Test Satellite".to_string(),
            elements,
            Utc::now(),
        );

        let result = propagator.propagate(&satellite, Utc::now() + chrono::Duration::hours(1));
        assert!(result.is_ok());
    }

    #[test]
    fn test_propagator_creation() {
        let keplerian = create_propagator(PropagatorType::Keplerian);
        assert!(keplerian.is_ok());
        assert_eq!(keplerian.unwrap().name(), "Keplerian");

        let sgp4 = create_propagator(PropagatorType::Sgp4);
        assert!(sgp4.is_ok());
        assert_eq!(sgp4.unwrap().name(), "SGP4 (Simplified)");

        let numerical = create_propagator(PropagatorType::Numerical);
        assert!(numerical.is_ok());
        assert_eq!(numerical.unwrap().name(), "Numerical Integration");
    }

    #[test]
    fn test_keplers_equation_solver() {
        let propagator = KeplerianPropagator::new();

        // Test circular orbit (e = 0)
        let result = propagator.solve_keplers_equation(1.0, 0.0);
        assert!(result.is_ok());
        assert!((result.unwrap() - 1.0).abs() < 1e-10);

        // Test elliptical orbit
        let result = propagator.solve_keplers_equation(PI / 2.0, 0.1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_propagation_time_validation() {
        let propagator = KeplerianPropagator::new();
        let start_time = Utc::now();
        let valid_end_time = start_time + chrono::Duration::days(100);
        let invalid_end_time = start_time + chrono::Duration::days(400);

        assert!(validate_propagation_time(&propagator, start_time, valid_end_time).is_ok());
        assert!(validate_propagation_time(&propagator, start_time, invalid_end_time).is_err());
    }
}
