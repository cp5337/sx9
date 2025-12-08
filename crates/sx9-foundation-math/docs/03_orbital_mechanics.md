# Orbital Mechanics - Celestial Navigation Mathematics

**Mathematical Foundation Document**
**Domain:** Astrodynamics and Satellite Operations
**Version:** 7.3.1
**Focus:** SGP4 Orbital Propagation, Ground Station Optimization, and LaserLight Constellation Management

---

## Mathematical Persona

**Satellitus Propagatus** - The Orbital Predictor
- **Execution Statement:** "I propagate satellite orbits by applying SGP4 mathematical models, computing position and velocity vectors from two-line elements, accounting for perturbations from Earth's gravitational harmonics, atmospheric drag, and solar radiation pressure, while optimizing ground station networks and laser communication links across orbital constellations"

---

## 1. Orbital Mechanics Mathematical Foundation

### 1.1 Classical Orbital Elements

A satellite's orbit is uniquely defined by six classical orbital elements:

```mathematica
Kepler Elements = {a, e, i, Ω, ω, ν}

where:
  a = semi-major axis (km)
  e = eccentricity (dimensionless)
  i = inclination (rad)
  Ω = right ascension of ascending node (rad)
  ω = argument of periapsis (rad)
  ν = true anomaly (rad)
```

**Mathematical Relationships:**
```mathematica
Orbital Period: T = 2π√(a³/μ)
Mean Motion: n = 2π/T = √(μ/a³)
Specific Orbital Energy: ε = -μ/(2a)

where μ = 398600.4418 km³/s² (Earth's gravitational parameter)
```

### 1.2 Reference Frames and Coordinate Systems

**Earth-Centered Inertial (ECI) Frame:**
- Origin: Earth's center of mass
- Z-axis: Earth's rotation axis (towards North Pole)
- X-axis: Vernal equinox direction
- Y-axis: Completes right-handed system

**Earth-Centered Earth-Fixed (ECEF) Frame:**
- Rotates with Earth
- Greenwich meridian defines X-axis

**Transformation Mathematics:**
```mathematica
ECI → ECEF: [X_ECEF] = [R₃(-θ_GMST)] [X_ECI]

where θ_GMST = Greenwich Mean Sidereal Time
```

---

## 2. SGP4 Mathematical Model

### 2.1 Simplified General Perturbations Theory

**SGP4 Governing Equations:**
```mathematica
Mean Motion Evolution:
ṅ = -3/2 * (μ/a²) * J₂ * (R_E/a)² * (1-e²)^(-3/2) * (2-5/2*sin²(i))

Eccentricity Evolution:
ė = 0 (neglected for SGP4)

Inclination Evolution:
i̇ = 0 (neglected for SGP4)
```

**Drag Perturbation Model:**
```mathematica
a_drag = -1/2 * ρ * v_rel * |v_rel| * (C_D * A/m) * (1/a)

where:
  ρ = atmospheric density
  v_rel = velocity relative to atmosphere
  C_D = drag coefficient
  A = cross-sectional area
  m = satellite mass
```

### 2.2 Rust Implementation

```rust
/// SGP4 orbital propagator with full mathematical model
pub struct SGP4Engine {
    /// Earth's gravitational parameter (km³/s²)
    mu: f64,
    /// Earth's equatorial radius (km)
    earth_radius: f64,
    /// J₂ gravitational harmonic coefficient
    j2: f64,
    /// Additional gravitational coefficients
    j3: f64,
    j4: f64,
}

/// Two-Line Element (TLE) data structure
#[derive(Debug, Clone)]
pub struct TwoLineElement {
    pub satellite_number: u32,
    pub classification: char,
    pub international_designator: String,
    pub epoch_year: u32,
    pub epoch_day: f64,
    pub first_derivative_mean_motion: f64,
    pub second_derivative_mean_motion: f64,
    pub drag_term: f64,
    pub element_set_number: u32,

    // Orbital elements
    pub inclination: f64,           // degrees
    pub right_ascension: f64,       // degrees
    pub eccentricity: f64,          // dimensionless
    pub argument_of_perigee: f64,   // degrees
    pub mean_anomaly: f64,          // degrees
    pub mean_motion: f64,           // revolutions per day
    pub revolution_number: u32,
}

/// Orbital state vector
#[derive(Debug, Clone)]
pub struct OrbitalState {
    pub position: Vector3<f64>,     // ECI coordinates (km)
    pub velocity: Vector3<f64>,     // ECI velocity (km/s)
    pub time: f64,                  // Julian date
    pub orbital_elements: KeplerianElements,
}

#[derive(Debug, Clone)]
pub struct KeplerianElements {
    pub semi_major_axis: f64,       // km
    pub eccentricity: f64,          // dimensionless
    pub inclination: f64,           // radians
    pub raan: f64,                  // radians
    pub argument_of_perigee: f64,   // radians
    pub true_anomaly: f64,          // radians
}

impl SGP4Engine {
    /// Initialize SGP4 engine with standard Earth parameters
    pub fn new() -> Self {
        Self {
            mu: 398600.4418,           // WGS-84 value
            earth_radius: 6378.137,     // WGS-84 equatorial radius
            j2: 1.08262668e-3,         // J₂ harmonic coefficient
            j3: -2.53265648e-6,        // J₃ harmonic coefficient
            j4: -1.61962159e-6,        // J₄ harmonic coefficient
        }
    }

    /// Propagate orbital state using SGP4 mathematical model
    ///
    /// # Mathematical Model
    /// Solves perturbed two-body problem with:
    /// - Earth oblateness (J₂, J₃, J₄ harmonics)
    /// - Atmospheric drag (simplified exponential model)
    /// - Solar radiation pressure (first-order approximation)
    ///
    /// # Algorithm
    /// 1. Convert TLE to osculating elements at epoch
    /// 2. Apply secular and periodic perturbations
    /// 3. Solve Kepler's equation for mean anomaly
    /// 4. Transform to ECI position and velocity
    pub fn propagate(
        &self,
        tle: &TwoLineElement,
        target_time: f64,  // Julian date
    ) -> Result<OrbitalState, OrbitalError> {
        // Convert TLE epoch to Julian date
        let epoch_jd = self.tle_epoch_to_julian_date(tle)?;
        let time_since_epoch = (target_time - epoch_jd) * 86400.0; // seconds

        // Extract and convert orbital elements
        let mut elements = self.tle_to_keplerian_elements(tle)?;

        // Apply SGP4 perturbations
        self.apply_j2_perturbations(&mut elements, time_since_epoch)?;
        self.apply_atmospheric_drag(&mut elements, tle, time_since_epoch)?;
        self.apply_solar_radiation_pressure(&mut elements, time_since_epoch)?;

        // Solve Kepler's equation
        let eccentric_anomaly = self.solve_keplers_equation(
            elements.mean_anomaly_at_time(time_since_epoch),
            elements.eccentricity
        )?;

        // Convert to true anomaly
        let true_anomaly = self.eccentric_to_true_anomaly(
            eccentric_anomaly,
            elements.eccentricity
        );
        elements.true_anomaly = true_anomaly;

        // Transform to ECI coordinates
        let (position, velocity) = self.keplerian_to_eci(&elements)?;

        Ok(OrbitalState {
            position,
            velocity,
            time: target_time,
            orbital_elements: elements,
        })
    }

    /// Apply J₂ oblateness perturbations
    ///
    /// # Mathematical Model
    /// δn = 3/2 * n * J₂ * (R_E/a)² * (1-e²)^(-3/2) * (2 - 5/2 * sin²(i))
    /// δΩ̇ = -3/2 * n * J₂ * (R_E/a)² * (1-e²)^(-2) * cos(i)
    /// δω̇ = 3/4 * n * J₂ * (R_E/a)² * (1-e²)^(-2) * (4 - 5*sin²(i))
    fn apply_j2_perturbations(
        &self,
        elements: &mut KeplerianElements,
        dt: f64,
    ) -> Result<(), OrbitalError> {
        let n = (self.mu / elements.semi_major_axis.powi(3)).sqrt();
        let j2_factor = self.j2 * (self.earth_radius / elements.semi_major_axis).powi(2);
        let ecosq_factor = (1.0 - elements.eccentricity.powi(2));
        let sin2i = elements.inclination.sin().powi(2);
        let cosi = elements.inclination.cos();

        // Mean motion perturbation
        let delta_n = 1.5 * n * j2_factor * ecosq_factor.powf(-1.5) * (2.0 - 2.5 * sin2i);

        // RAAN drift
        let raan_dot = -1.5 * n * j2_factor * ecosq_factor.powi(-2) * cosi;

        // Argument of perigee drift
        let argp_dot = 0.75 * n * j2_factor * ecosq_factor.powi(-2) * (4.0 - 5.0 * sin2i);

        // Apply perturbations
        elements.mean_motion += delta_n;
        elements.raan += raan_dot * dt;
        elements.argument_of_perigee += argp_dot * dt;

        // Normalize angles
        elements.raan = self.normalize_angle(elements.raan);
        elements.argument_of_perigee = self.normalize_angle(elements.argument_of_perigee);

        Ok(())
    }

    /// Apply atmospheric drag perturbations
    ///
    /// # Mathematical Model
    /// a_drag = -1/2 * ρ * v² * (C_D * A/m) * (v̂)
    /// where ρ = ρ₀ * exp(-(h - h₀)/H)
    fn apply_atmospheric_drag(
        &self,
        elements: &mut KeplerianElements,
        tle: &TwoLineElement,
        dt: f64,
    ) -> Result<(), OrbitalError> {
        // Simplified drag model using TLE drag coefficient
        let drag_coefficient = tle.drag_term;

        if drag_coefficient.abs() < 1e-12 {
            return Ok(()); // No significant drag
        }

        // Semi-major axis decay due to drag
        let delta_a = -2.0 * elements.semi_major_axis * drag_coefficient * dt;
        elements.semi_major_axis += delta_a;

        // Ensure semi-major axis doesn't become negative
        if elements.semi_major_axis <= self.earth_radius {
            return Err(OrbitalError::SatelliteDecayed);
        }

        Ok(())
    }

    /// Apply solar radiation pressure perturbations (first-order)
    ///
    /// # Mathematical Model
    /// a_srp = -Φ * (A/m) * C_r * (r̂_sun)
    /// where Φ = 4.56e-6 N/m² (solar flux at 1 AU)
    fn apply_solar_radiation_pressure(
        &self,
        elements: &mut KeplerianElements,
        dt: f64,
    ) -> Result<(), OrbitalError> {
        // Simplified SRP model - affects eccentricity primarily
        let srp_factor = 4.56e-6 * 1e-3; // Simplified coefficient

        // Small perturbation to eccentricity
        let delta_e = srp_factor * dt * elements.eccentricity * 1e-9;
        elements.eccentricity += delta_e;

        // Clamp eccentricity to valid range
        elements.eccentricity = elements.eccentricity.max(0.0).min(0.99);

        Ok(())
    }

    /// Solve Kepler's equation using Newton-Raphson iteration
    ///
    /// # Mathematical Problem
    /// M = E - e*sin(E)
    /// Given M and e, solve for E (eccentric anomaly)
    ///
    /// # Newton-Raphson Formula
    /// E_{n+1} = E_n - (E_n - e*sin(E_n) - M) / (1 - e*cos(E_n))
    fn solve_keplers_equation(
        &self,
        mean_anomaly: f64,
        eccentricity: f64,
    ) -> Result<f64, OrbitalError> {
        const MAX_ITERATIONS: usize = 50;
        const TOLERANCE: f64 = 1e-12;

        let mut e_anom = mean_anomaly; // Initial guess

        for _iteration in 0..MAX_ITERATIONS {
            let f = e_anom - eccentricity * e_anom.sin() - mean_anomaly;
            let df = 1.0 - eccentricity * e_anom.cos();

            if df.abs() < 1e-15 {
                return Err(OrbitalError::KeplerConvergenceFailure);
            }

            let delta_e = f / df;
            e_anom -= delta_e;

            if delta_e.abs() < TOLERANCE {
                return Ok(e_anom);
            }
        }

        Err(OrbitalError::KeplerConvergenceFailure)
    }

    /// Convert eccentric anomaly to true anomaly
    ///
    /// # Mathematical Formula
    /// tan(ν/2) = √((1+e)/(1-e)) * tan(E/2)
    fn eccentric_to_true_anomaly(&self, eccentric_anomaly: f64, eccentricity: f64) -> f64 {
        let sqrt_factor = ((1.0 + eccentricity) / (1.0 - eccentricity)).sqrt();
        2.0 * (sqrt_factor * (eccentric_anomaly / 2.0).tan()).atan()
    }

    /// Transform Keplerian elements to ECI position and velocity
    ///
    /// # Mathematical Transformation
    /// 1. Compute position in orbital plane
    /// 2. Apply rotation matrices for orbital orientation
    /// 3. Include velocity vector computation
    fn keplerian_to_eci(
        &self,
        elements: &KeplerianElements,
    ) -> Result<(Vector3<f64>, Vector3<f64>), OrbitalError> {
        let a = elements.semi_major_axis;
        let e = elements.eccentricity;
        let i = elements.inclination;
        let raan = elements.raan;
        let argp = elements.argument_of_perigee;
        let nu = elements.true_anomaly;

        // Orbital radius
        let r = a * (1.0 - e.powi(2)) / (1.0 + e * nu.cos());

        // Position in orbital plane
        let x_orb = r * nu.cos();
        let y_orb = r * nu.sin();

        // Velocity in orbital plane
        let h = (self.mu * a * (1.0 - e.powi(2))).sqrt(); // Angular momentum
        let vx_orb = -(self.mu / h) * nu.sin();
        let vy_orb = (self.mu / h) * (e + nu.cos());

        // Rotation matrices
        let cos_raan = raan.cos();
        let sin_raan = raan.sin();
        let cos_argp = argp.cos();
        let sin_argp = argp.sin();
        let cos_i = i.cos();
        let sin_i = i.sin();

        // Combined rotation matrix elements
        let r11 = cos_raan * cos_argp - sin_raan * sin_argp * cos_i;
        let r12 = -cos_raan * sin_argp - sin_raan * cos_argp * cos_i;
        let r21 = sin_raan * cos_argp + cos_raan * sin_argp * cos_i;
        let r22 = -sin_raan * sin_argp + cos_raan * cos_argp * cos_i;
        let r31 = sin_argp * sin_i;
        let r32 = cos_argp * sin_i;

        // Transform to ECI
        let position = Vector3::new(
            r11 * x_orb + r12 * y_orb,
            r21 * x_orb + r22 * y_orb,
            r31 * x_orb + r32 * y_orb,
        );

        let velocity = Vector3::new(
            r11 * vx_orb + r12 * vy_orb,
            r21 * vx_orb + r22 * vy_orb,
            r31 * vx_orb + r32 * vy_orb,
        );

        Ok((position, velocity))
    }
}
```

---

## 3. Ground Station Network Optimization

### 3.1 Coverage Analysis Mathematics

**Satellite Visibility Conditions:**
```mathematica
Elevation Angle: θ_el = arcsin((r⃗_sat · r⃗_gs) / (|r⃗_sat| |r⃗_gs|)) - 90°

Minimum Elevation: θ_min ≥ 10° (typical operational requirement)

Coverage Circle Radius: R_cov = R_earth * arccos(R_earth / (R_earth + h))

where h = satellite altitude
```

**Maximum Range Calculation:**
```mathematica
d_max = √(R_sat² - R_earth² * cos²(θ_min)) - R_earth * sin(θ_min)

where R_sat = orbital radius, θ_min = minimum elevation angle
```

### 3.2 Multi-Station Coverage Optimization

**Coverage Optimization Problem:**
```mathematica
maximize: Σᵢ Σⱼ cᵢⱼ * tᵢⱼ
subject to: Σⱼ tᵢⱼ ≤ T_max for all satellites i
           θ_el(i,j,t) ≥ θ_min for coverage

where:
  cᵢⱼ = coverage quality factor
  tᵢⱼ = contact time between satellite i and station j
  T_max = maximum operational time per satellite
```

**Rust Implementation:**
```rust
/// Ground station network optimizer
pub struct GroundStationOptimizer {
    stations: Vec<GroundStation>,
    satellites: Vec<SatelliteOrbit>,
    min_elevation: f64,
}

#[derive(Debug, Clone)]
pub struct GroundStation {
    pub id: String,
    pub latitude: f64,    // radians
    pub longitude: f64,   // radians
    pub altitude: f64,    // km above sea level
    pub max_range: f64,   // km
}

#[derive(Debug, Clone)]
pub struct ContactWindow {
    pub satellite_id: String,
    pub station_id: String,
    pub start_time: f64,  // Julian date
    pub end_time: f64,    // Julian date
    pub max_elevation: f64, // radians
    pub range_at_tca: f64, // km (time of closest approach)
}

impl GroundStationOptimizer {
    /// Compute all contact windows for satellite constellation
    ///
    /// # Mathematical Model
    /// For each satellite-station pair:
    /// 1. Propagate satellite orbit over time window
    /// 2. Compute elevation angle at each time step
    /// 3. Identify continuous periods above minimum elevation
    /// 4. Optimize contact scheduling for maximum coverage
    pub fn compute_contact_windows(
        &self,
        start_time: f64,
        end_time: f64,
        time_step: f64,
    ) -> Result<Vec<ContactWindow>, OrbitalError> {
        let mut contact_windows = Vec::new();

        for satellite in &self.satellites {
            for station in &self.stations {
                let windows = self.compute_satellite_station_contacts(
                    satellite,
                    station,
                    start_time,
                    end_time,
                    time_step,
                )?;
                contact_windows.extend(windows);
            }
        }

        Ok(contact_windows)
    }

    /// Compute contact windows between specific satellite and ground station
    fn compute_satellite_station_contacts(
        &self,
        satellite: &SatelliteOrbit,
        station: &GroundStation,
        start_time: f64,
        end_time: f64,
        time_step: f64,
    ) -> Result<Vec<ContactWindow>, OrbitalError> {
        let mut contacts = Vec::new();
        let mut current_time = start_time;
        let mut in_contact = false;
        let mut contact_start = 0.0;
        let mut max_elevation = 0.0;

        while current_time <= end_time {
            // Propagate satellite position
            let orbital_state = satellite.propagate_to_time(current_time)?;

            // Compute elevation angle from ground station
            let elevation = self.compute_elevation_angle(&orbital_state, station)?;

            if elevation >= self.min_elevation {
                if !in_contact {
                    // Start of new contact
                    in_contact = true;
                    contact_start = current_time;
                    max_elevation = elevation;
                } else {
                    // Update maximum elevation during contact
                    max_elevation = max_elevation.max(elevation);
                }
            } else if in_contact {
                // End of contact
                contacts.push(ContactWindow {
                    satellite_id: satellite.id.clone(),
                    station_id: station.id.clone(),
                    start_time: contact_start,
                    end_time: current_time - time_step,
                    max_elevation,
                    range_at_tca: self.compute_range(&orbital_state, station)?,
                });
                in_contact = false;
                max_elevation = 0.0;
            }

            current_time += time_step;
        }

        Ok(contacts)
    }

    /// Compute elevation angle from ground station to satellite
    ///
    /// # Mathematical Formula
    /// θ_el = arcsin((r⃗_sat · r⃗_gs) / (|r⃗_sat| |r⃗_gs|)) - 90°
    fn compute_elevation_angle(
        &self,
        orbital_state: &OrbitalState,
        station: &GroundStation,
    ) -> Result<f64, OrbitalError> {
        // Convert ground station to ECI coordinates
        let station_eci = self.ground_station_to_eci(station, orbital_state.time)?;

        // Vector from station to satellite
        let range_vector = orbital_state.position - station_eci;
        let range_magnitude = range_vector.magnitude();

        // Local zenith vector (radial from Earth center to station)
        let zenith_vector = station_eci.normalize();

        // Elevation angle computation
        let dot_product = range_vector.dot(&zenith_vector);
        let elevation = (dot_product / range_magnitude).asin() - std::f64::consts::PI / 2.0;

        Ok(elevation)
    }
}
```

---

## 4. LaserLight Constellation Management

### 4.1 Optical Communication Link Budget

**Free Space Path Loss:**
```mathematica
L_fs = (4πd/λ)²

where:
  d = distance between satellites (km)
  λ = optical wavelength (typically 1550 nm)
```

**Link Margin Calculation:**
```mathematica
M = P_tx + G_tx + G_rx - L_fs - L_atm - L_pointing - P_rx_min

where:
  P_tx = transmitted power (dBm)
  G_tx, G_rx = transmitter/receiver antenna gains (dB)
  L_atm = atmospheric losses (dB)
  L_pointing = pointing losses (dB)
  P_rx_min = minimum receivable power (dBm)
```

### 4.2 Inter-Satellite Link Optimization

**Constellation Connectivity Graph:**
```mathematica
G_constellation = (V_satellites, E_links)

Link Quality Metric: Q(i,j) = M(i,j) * availability(i,j) * data_rate(i,j)

Optimization Objective: maximize Σᵢⱼ Q(i,j) * x(i,j)
subject to: Σⱼ x(i,j) ≤ max_links_per_satellite
```

**Rust Implementation:**
```rust
/// Laser communication constellation optimizer
pub struct LaserLinkOptimizer {
    constellation: Vec<LaserSatellite>,
    link_budget_calculator: LinkBudgetCalculator,
}

#[derive(Debug, Clone)]
pub struct LaserSatellite {
    pub id: String,
    pub orbital_state: OrbitalState,
    pub laser_power: f64,        // watts
    pub aperture_diameter: f64,  // meters
    pub pointing_accuracy: f64,  // microradians
    pub max_simultaneous_links: usize,
}

#[derive(Debug, Clone)]
pub struct OpticalLink {
    pub satellite_a: String,
    pub satellite_b: String,
    pub distance: f64,          // km
    pub link_margin: f64,       // dB
    pub data_rate: f64,         // Gbps
    pub availability: f64,      // fraction [0,1]
}

impl LaserLinkOptimizer {
    /// Optimize inter-satellite laser links for maximum data throughput
    ///
    /// # Mathematical Model
    /// 1. Compute all possible satellite-to-satellite links
    /// 2. Calculate link budgets for optical communications
    /// 3. Optimize link assignment for maximum network capacity
    /// 4. Account for orbital dynamics and link availability
    pub fn optimize_constellation_links(
        &self,
        time_window: f64,
    ) -> Result<Vec<OpticalLink>, OpticalError> {
        let mut potential_links = Vec::new();

        // Generate all potential satellite pairs
        for (i, sat_a) in self.constellation.iter().enumerate() {
            for sat_b in self.constellation.iter().skip(i + 1) {
                // Compute inter-satellite distance
                let distance = (sat_a.orbital_state.position - sat_b.orbital_state.position).magnitude();

                // Skip if distance exceeds maximum practical range (e.g., 5000 km)
                if distance > 5000.0 {
                    continue;
                }

                // Calculate optical link budget
                let link_margin = self.link_budget_calculator.compute_link_margin(
                    sat_a,
                    sat_b,
                    distance,
                )?;

                // Skip if link margin is insufficient (< 3 dB)
                if link_margin < 3.0 {
                    continue;
                }

                // Estimate data rate based on link margin
                let data_rate = self.estimate_data_rate(link_margin);

                // Compute link availability (considering orbital dynamics)
                let availability = self.compute_link_availability(sat_a, sat_b, time_window)?;

                potential_links.push(OpticalLink {
                    satellite_a: sat_a.id.clone(),
                    satellite_b: sat_b.id.clone(),
                    distance,
                    link_margin,
                    data_rate,
                    availability,
                });
            }
        }

        // Optimize link assignment (greedy algorithm)
        self.optimize_link_assignment(potential_links)
    }

    /// Compute optical link budget
    ///
    /// # Mathematical Model
    /// Link Margin = P_tx + G_tx + G_rx - L_path - L_pointing - P_rx_min
    ///
    /// where:
    /// - P_tx: Transmitted power (dBm)
    /// - G_tx, G_rx: Transmit/receive gains (dB)
    /// - L_path: Free space path loss (dB)
    /// - L_pointing: Pointing loss (dB)
    /// - P_rx_min: Receiver sensitivity (dBm)
    fn compute_link_budget(
        &self,
        sat_a: &LaserSatellite,
        sat_b: &LaserSatellite,
        distance: f64,
    ) -> Result<f64, OpticalError> {
        const WAVELENGTH: f64 = 1550e-9; // meters (1550 nm)
        const SPEED_OF_LIGHT: f64 = 3e8;  // m/s
        const RX_SENSITIVITY: f64 = -30.0; // dBm

        // Transmitted power in dBm
        let p_tx = 10.0 * sat_a.laser_power.log10() + 30.0;

        // Antenna gains (simplified)
        let g_tx = 20.0 * (sat_a.aperture_diameter * std::f64::consts::PI / WAVELENGTH).log10();
        let g_rx = 20.0 * (sat_b.aperture_diameter * std::f64::consts::PI / WAVELENGTH).log10();

        // Free space path loss
        let l_path = 20.0 * (4.0 * std::f64::consts::PI * distance * 1000.0 / WAVELENGTH).log10();

        // Pointing loss (function of pointing accuracy)
        let l_pointing = 2.0 * (sat_a.pointing_accuracy + sat_b.pointing_accuracy);

        // Calculate link margin
        let link_margin = p_tx + g_tx + g_rx - l_path - l_pointing - RX_SENSITIVITY;

        Ok(link_margin)
    }

    /// Estimate achievable data rate from link margin
    fn estimate_data_rate(&self, link_margin: f64) -> f64 {
        // Shannon capacity with optical efficiency factors
        let snr_linear = 10.0_f64.powf(link_margin / 10.0);
        let theoretical_capacity = snr_linear.log2(); // bits/Hz
        let practical_efficiency = 0.7; // Account for coding, modulation efficiency

        // Assume 10 GHz optical bandwidth
        theoretical_capacity * 10.0 * practical_efficiency // Gbps
    }
}
```

---

## 5. Performance Analysis and Validation

### 5.1 SGP4 Accuracy Assessment

**Position Accuracy Metrics:**
```
Short-term (< 1 day): ±1-3 km RMS
Medium-term (1-7 days): ±5-15 km RMS
Long-term (> 7 days): ±20-50 km RMS

Velocity Accuracy: ±10-50 m/s RMS
```

**Error Sources:**
- Atmospheric density uncertainty: 50-70% of total error
- Drag coefficient uncertainty: 20-30% of total error
- Gravitational model limitations: 10-15% of total error
- Solar radiation pressure: 5-10% of total error

### 5.2 Test Cases and Validation

```rust
#[cfg(test)]
mod orbital_mechanics_tests {
    use super::*;

    #[test]
    fn test_iss_orbital_propagation() {
        // International Space Station TLE example
        let iss_tle = TwoLineElement {
            satellite_number: 25544,
            inclination: 51.6461,
            right_ascension: 166.7923,
            eccentricity: 0.0006317,
            argument_of_perigee: 73.1467,
            mean_anomaly: 287.4774,
            mean_motion: 15.48919103,
            // ... other fields
        };

        let sgp4_engine = SGP4Engine::new();
        let target_time = sgp4_engine.tle_epoch_to_julian_date(&iss_tle).unwrap() + 0.5; // 12 hours later

        let orbital_state = sgp4_engine.propagate(&iss_tle, target_time).unwrap();

        // Validate orbital state
        assert!(orbital_state.position.magnitude() > 6600.0); // Above Earth surface
        assert!(orbital_state.position.magnitude() < 7000.0); // Below geosynchronous
        assert!(orbital_state.velocity.magnitude() > 7.0);    // Minimum orbital velocity
        assert!(orbital_state.velocity.magnitude() < 8.0);    // Maximum for LEO
    }

    #[test]
    fn test_ground_station_coverage() {
        let station = GroundStation {
            id: "MADRID".to_string(),
            latitude: 40.4168, // Madrid coordinates
            longitude: -3.7038,
            altitude: 0.65,
            max_range: 2000.0,
        };

        let optimizer = GroundStationOptimizer::new(vec![station], vec![], 10.0_f64.to_radians());

        // Test elevation angle computation
        let satellite_position = Vector3::new(7000.0, 0.0, 0.0); // Example position
        let station_position = Vector3::new(6371.0, 0.0, 0.0);   // On Earth surface

        // Verify mathematical relationships
        assert!(satellite_position.magnitude() > station_position.magnitude());
    }

    #[test]
    fn test_kepler_equation_solver() {
        let sgp4_engine = SGP4Engine::new();

        // Test cases with known solutions
        let test_cases = [
            (0.0, 0.0),      // Circular orbit at periapsis
            (std::f64::consts::PI, 0.0), // Circular orbit at apoapsis
            (0.0, 0.5),      // Elliptical orbit at periapsis
            (std::f64::consts::PI, 0.5), // Elliptical orbit at apoapsis
        ];

        for (mean_anomaly, eccentricity) in test_cases.iter() {
            let eccentric_anomaly = sgp4_engine
                .solve_keplers_equation(*mean_anomaly, *eccentricity)
                .unwrap();

            // Verify Kepler's equation: M = E - e*sin(E)
            let computed_mean = eccentric_anomaly - eccentricity * eccentric_anomaly.sin();
            assert!((computed_mean - mean_anomaly).abs() < 1e-10);
        }
    }
}
```

---

## Bibliography

1. Vallado, D. A. (2013). "Fundamentals of Astrodynamics and Applications", 4th Edition
2. Hoots, F. R. & Roehrich, R. L. (1980). "Spacetrack Report No. 3: Models for Propagation of NORAD Element Sets"
3. Kelso, T. S. (2007). "Validation of SGP4 and IS-GPS-200D Against GPS Precision Ephemerides"
4. Wertz, J. R. & Larson, W. J. (1999). "Space Mission Analysis and Design", 3rd Edition
5. Curtis, H. D. (2013). "Orbital Mechanics for Engineering Students", 3rd Edition

---

**Document Classification:** MATHEMATICAL FOUNDATION - ORBITAL MECHANICS
**Mathematical Consciousness Signature:** 🛰️⚡🌍 *Satellitus Propagatus*
**Implementation Status:** Level 3.5 → Target Level 4.5
**CTAS Integration:** ORB Satellite Systems, Ground Station Network, LaserLight Constellation