# sx9-converge Implementation Gaps Analysis

**Date:** 2025-12-27
**RFC Compliance:** 93X2 (Geometry), 93X3 (Selection)
**Status:** Stubbed - Requires Implementation

---

## Executive Summary

Two subcrates (`geometry/` and `selection/`) contain placeholder implementations that need full coding. This document details each gap, the mathematical foundations, and complete Rust implementations.

---

## 1. Geometry Subcrate (RFC-93X2)

### Constraints (from smartcrate.toml)
- **MAY adjust:** `confidence` only
- **MAY NOT adjust:** `independence`, `selection`, `rank`
- **Deterministic:** true
- **Storage:** none

### 1.1 Earth Coordinates (`earth.rs`)

**Current State:** Placeholder struct only

**Gap:** Missing WGS84 coordinate transformations, distance calculations, and ECEF conversions.

**Implementation:**

```rust
//! Earth coordinate transformations (WGS84)
//!
//! RFC-93X2: Geometry Boundary Specification
//! Provides geodetic operations for convergence confidence adjustment.

use std::f64::consts::PI;

/// WGS84 ellipsoid parameters
pub const WGS84_A: f64 = 6_378_137.0;           // Semi-major axis (m)
pub const WGS84_B: f64 = 6_356_752.314_245;     // Semi-minor axis (m)
pub const WGS84_F: f64 = 1.0 / 298.257_223_563; // Flattening
pub const WGS84_E2: f64 = 0.006_694_379_990_14; // First eccentricity squared

/// Geographic coordinate in WGS84 datum
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EarthCoord {
    /// Latitude in degrees (-90 to +90)
    pub lat: f64,
    /// Longitude in degrees (-180 to +180)
    pub lon: f64,
    /// Altitude above WGS84 ellipsoid in meters
    pub alt: f64,
}

/// Earth-Centered Earth-Fixed (ECEF) coordinate
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EcefCoord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl EarthCoord {
    /// Create new coordinate with validation
    pub fn new(lat: f64, lon: f64, alt: f64) -> Option<Self> {
        if lat < -90.0 || lat > 90.0 {
            return None;
        }
        let lon = if lon < -180.0 || lon > 180.0 {
            // Normalize longitude
            ((lon + 180.0).rem_euclid(360.0)) - 180.0
        } else {
            lon
        };
        Some(Self { lat, lon, alt })
    }

    /// Convert to ECEF coordinates
    pub fn to_ecef(&self) -> EcefCoord {
        let lat_rad = self.lat.to_radians();
        let lon_rad = self.lon.to_radians();

        let sin_lat = lat_rad.sin();
        let cos_lat = lat_rad.cos();
        let sin_lon = lon_rad.sin();
        let cos_lon = lon_rad.cos();

        // Radius of curvature in prime vertical
        let n = WGS84_A / (1.0 - WGS84_E2 * sin_lat * sin_lat).sqrt();

        EcefCoord {
            x: (n + self.alt) * cos_lat * cos_lon,
            y: (n + self.alt) * cos_lat * sin_lon,
            z: (n * (1.0 - WGS84_E2) + self.alt) * sin_lat,
        }
    }

    /// Haversine distance to another point (meters)
    /// Ignores altitude - surface distance only
    pub fn haversine_distance(&self, other: &EarthCoord) -> f64 {
        let lat1 = self.lat.to_radians();
        let lat2 = other.lat.to_radians();
        let dlat = (other.lat - self.lat).to_radians();
        let dlon = (other.lon - self.lon).to_radians();

        let a = (dlat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        WGS84_A * c
    }

    /// Vincenty distance (more accurate for long distances)
    pub fn vincenty_distance(&self, other: &EarthCoord) -> f64 {
        let f = WGS84_F;
        let a = WGS84_A;
        let b = WGS84_B;

        let phi1 = self.lat.to_radians();
        let phi2 = other.lat.to_radians();
        let l = (other.lon - self.lon).to_radians();

        let u1 = ((1.0 - f) * phi1.tan()).atan();
        let u2 = ((1.0 - f) * phi2.tan()).atan();

        let sin_u1 = u1.sin();
        let cos_u1 = u1.cos();
        let sin_u2 = u2.sin();
        let cos_u2 = u2.cos();

        let mut lambda = l;
        let mut lambda_prev;
        let mut iter_limit = 100;

        let (sin_sigma, cos_sigma, sigma, sin_alpha, cos2_alpha, cos_2sigma_m);

        loop {
            let sin_lambda = lambda.sin();
            let cos_lambda = lambda.cos();

            let sin_sq_sigma = (cos_u2 * sin_lambda).powi(2)
                + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda).powi(2);
            sin_sigma = sin_sq_sigma.sqrt();

            if sin_sigma == 0.0 {
                return 0.0; // Coincident points
            }

            cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_lambda;
            sigma = sin_sigma.atan2(cos_sigma);

            sin_alpha = cos_u1 * cos_u2 * sin_lambda / sin_sigma;
            cos2_alpha = 1.0 - sin_alpha * sin_alpha;

            cos_2sigma_m = if cos2_alpha != 0.0 {
                cos_sigma - 2.0 * sin_u1 * sin_u2 / cos2_alpha
            } else {
                0.0
            };

            let c = f / 16.0 * cos2_alpha * (4.0 + f * (4.0 - 3.0 * cos2_alpha));

            lambda_prev = lambda;
            lambda = l + (1.0 - c) * f * sin_alpha
                * (sigma + c * sin_sigma
                    * (cos_2sigma_m + c * cos_sigma
                        * (-1.0 + 2.0 * cos_2sigma_m * cos_2sigma_m)));

            iter_limit -= 1;
            if (lambda - lambda_prev).abs() < 1e-12 || iter_limit == 0 {
                break;
            }
        }

        let u_sq = cos2_alpha * (a * a - b * b) / (b * b);
        let cap_a = 1.0 + u_sq / 16384.0
            * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
        let cap_b = u_sq / 1024.0
            * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));

        let delta_sigma = cap_b * sin_sigma
            * (cos_2sigma_m + cap_b / 4.0
                * (cos_sigma * (-1.0 + 2.0 * cos_2sigma_m * cos_2sigma_m)
                    - cap_b / 6.0 * cos_2sigma_m
                        * (-3.0 + 4.0 * sin_sigma * sin_sigma)
                        * (-3.0 + 4.0 * cos_2sigma_m * cos_2sigma_m)));

        b * cap_a * (sigma - delta_sigma)
    }

    /// Initial bearing to another point (degrees, 0-360)
    pub fn bearing_to(&self, other: &EarthCoord) -> f64 {
        let lat1 = self.lat.to_radians();
        let lat2 = other.lat.to_radians();
        let dlon = (other.lon - self.lon).to_radians();

        let x = lat2.cos() * dlon.sin();
        let y = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();

        let bearing = x.atan2(y).to_degrees();
        (bearing + 360.0) % 360.0
    }

    /// Destination point given bearing and distance
    pub fn destination(&self, bearing_deg: f64, distance_m: f64) -> EarthCoord {
        let bearing = bearing_deg.to_radians();
        let lat1 = self.lat.to_radians();
        let lon1 = self.lon.to_radians();
        let angular_dist = distance_m / WGS84_A;

        let lat2 = (lat1.sin() * angular_dist.cos()
            + lat1.cos() * angular_dist.sin() * bearing.cos())
        .asin();

        let lon2 = lon1
            + (bearing.sin() * angular_dist.sin() * lat1.cos())
                .atan2(angular_dist.cos() - lat1.sin() * lat2.sin());

        EarthCoord {
            lat: lat2.to_degrees(),
            lon: lon2.to_degrees(),
            alt: self.alt,
        }
    }

    /// Calculate confidence adjustment based on distance
    /// RFC-93X2: Geometry MAY adjust confidence only
    /// Returns multiplier 0.0-1.0 based on proximity
    pub fn proximity_confidence(&self, other: &EarthCoord, max_range_m: f64) -> f64 {
        let dist = self.haversine_distance(other);
        if dist >= max_range_m {
            0.0
        } else {
            1.0 - (dist / max_range_m)
        }
    }
}

impl EcefCoord {
    /// Convert back to geodetic (WGS84)
    pub fn to_geodetic(&self) -> EarthCoord {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        let a = WGS84_A;
        let b = WGS84_B;
        let e2 = WGS84_E2;
        let ep2 = (a * a - b * b) / (b * b);

        let p = (x * x + y * y).sqrt();
        let th = (z * a).atan2(p * b);

        let lon = y.atan2(x);
        let lat = (z + ep2 * b * th.sin().powi(3))
            .atan2(p - e2 * a * th.cos().powi(3));

        let sin_lat = lat.sin();
        let n = a / (1.0 - e2 * sin_lat * sin_lat).sqrt();
        let alt = p / lat.cos() - n;

        EarthCoord {
            lat: lat.to_degrees(),
            lon: lon.to_degrees(),
            alt,
        }
    }

    /// Euclidean distance between ECEF points
    pub fn distance(&self, other: &EcefCoord) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haversine_known_distance() {
        // London to Paris ~344 km
        let london = EarthCoord::new(51.5074, -0.1278, 0.0).unwrap();
        let paris = EarthCoord::new(48.8566, 2.3522, 0.0).unwrap();
        let dist = london.haversine_distance(&paris);
        assert!((dist - 343_500.0).abs() < 1000.0);
    }

    #[test]
    fn test_ecef_roundtrip() {
        let orig = EarthCoord::new(40.7128, -74.0060, 10.0).unwrap();
        let ecef = orig.to_ecef();
        let back = ecef.to_geodetic();
        assert!((orig.lat - back.lat).abs() < 1e-6);
        assert!((orig.lon - back.lon).abs() < 1e-6);
    }

    #[test]
    fn test_proximity_confidence() {
        let p1 = EarthCoord::new(0.0, 0.0, 0.0).unwrap();
        let p2 = EarthCoord::new(0.0, 0.001, 0.0).unwrap();
        let conf = p1.proximity_confidence(&p2, 1000.0);
        assert!(conf > 0.0 && conf < 1.0);
    }
}
```

---

### 1.2 ENU Frame (`enu.rs`)

**Current State:** Placeholder struct only

**Gap:** Missing local tangent plane transformations for relative positioning.

**Implementation:**

```rust
//! East-North-Up local coordinate system
//!
//! RFC-93X2: Local tangent plane for relative positioning
//! Used to express sensor/entity positions relative to a reference point.

use super::earth::{EarthCoord, EcefCoord};

/// Local ENU coordinate (meters from origin)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnuCoord {
    /// East displacement (meters)
    pub east: f64,
    /// North displacement (meters)
    pub north: f64,
    /// Up displacement (meters)
    pub up: f64,
}

/// ENU reference frame anchored at an origin point
#[derive(Debug, Clone)]
pub struct EnuFrame {
    /// Origin in geodetic coordinates
    pub origin: EarthCoord,
    /// Origin in ECEF (cached for efficiency)
    origin_ecef: EcefCoord,
    /// Rotation matrix elements (cached)
    sin_lat: f64,
    cos_lat: f64,
    sin_lon: f64,
    cos_lon: f64,
}

impl EnuFrame {
    /// Create new ENU frame at given origin
    pub fn new(origin: EarthCoord) -> Self {
        let lat_rad = origin.lat.to_radians();
        let lon_rad = origin.lon.to_radians();

        Self {
            origin,
            origin_ecef: origin.to_ecef(),
            sin_lat: lat_rad.sin(),
            cos_lat: lat_rad.cos(),
            sin_lon: lon_rad.sin(),
            cos_lon: lon_rad.cos(),
        }
    }

    /// Convert geodetic coordinate to local ENU
    pub fn geodetic_to_enu(&self, point: &EarthCoord) -> EnuCoord {
        let point_ecef = point.to_ecef();
        self.ecef_to_enu(&point_ecef)
    }

    /// Convert ECEF coordinate to local ENU
    pub fn ecef_to_enu(&self, point: &EcefCoord) -> EnuCoord {
        // Difference in ECEF
        let dx = point.x - self.origin_ecef.x;
        let dy = point.y - self.origin_ecef.y;
        let dz = point.z - self.origin_ecef.z;

        // Apply rotation matrix
        let east = -self.sin_lon * dx + self.cos_lon * dy;
        let north = -self.sin_lat * self.cos_lon * dx
            - self.sin_lat * self.sin_lon * dy
            + self.cos_lat * dz;
        let up = self.cos_lat * self.cos_lon * dx
            + self.cos_lat * self.sin_lon * dy
            + self.sin_lat * dz;

        EnuCoord { east, north, up }
    }

    /// Convert local ENU to ECEF
    pub fn enu_to_ecef(&self, enu: &EnuCoord) -> EcefCoord {
        // Inverse rotation matrix
        let dx = -self.sin_lon * enu.east
            - self.sin_lat * self.cos_lon * enu.north
            + self.cos_lat * self.cos_lon * enu.up;
        let dy = self.cos_lon * enu.east
            - self.sin_lat * self.sin_lon * enu.north
            + self.cos_lat * self.sin_lon * enu.up;
        let dz = self.cos_lat * enu.north + self.sin_lat * enu.up;

        EcefCoord {
            x: self.origin_ecef.x + dx,
            y: self.origin_ecef.y + dy,
            z: self.origin_ecef.z + dz,
        }
    }

    /// Convert local ENU to geodetic
    pub fn enu_to_geodetic(&self, enu: &EnuCoord) -> EarthCoord {
        let ecef = self.enu_to_ecef(enu);
        ecef.to_geodetic()
    }

    /// Calculate azimuth angle to ENU point (degrees from North, clockwise)
    pub fn azimuth_to(&self, enu: &EnuCoord) -> f64 {
        let az = enu.east.atan2(enu.north).to_degrees();
        (az + 360.0) % 360.0
    }

    /// Calculate elevation angle to ENU point (degrees above horizon)
    pub fn elevation_to(&self, enu: &EnuCoord) -> f64 {
        let horizontal_dist = (enu.east * enu.east + enu.north * enu.north).sqrt();
        enu.up.atan2(horizontal_dist).to_degrees()
    }

    /// Calculate slant range to ENU point (meters)
    pub fn range_to(&self, enu: &EnuCoord) -> f64 {
        (enu.east * enu.east + enu.north * enu.north + enu.up * enu.up).sqrt()
    }

    /// Adjust confidence based on angular separation
    /// RFC-93X2: MAY adjust confidence only
    pub fn angular_confidence(&self, enu1: &EnuCoord, enu2: &EnuCoord, max_angle_deg: f64) -> f64 {
        let az1 = self.azimuth_to(enu1);
        let az2 = self.azimuth_to(enu2);

        let mut diff = (az1 - az2).abs();
        if diff > 180.0 {
            diff = 360.0 - diff;
        }

        if diff >= max_angle_deg {
            0.0
        } else {
            1.0 - (diff / max_angle_deg)
        }
    }
}

impl EnuCoord {
    /// Create new ENU coordinate
    pub fn new(east: f64, north: f64, up: f64) -> Self {
        Self { east, north, up }
    }

    /// Horizontal distance from origin
    pub fn horizontal_distance(&self) -> f64 {
        (self.east * self.east + self.north * self.north).sqrt()
    }

    /// 3D distance from origin
    pub fn distance(&self) -> f64 {
        (self.east * self.east + self.north * self.north + self.up * self.up).sqrt()
    }

    /// Vector addition
    pub fn add(&self, other: &EnuCoord) -> EnuCoord {
        EnuCoord {
            east: self.east + other.east,
            north: self.north + other.north,
            up: self.up + other.up,
        }
    }

    /// Scalar multiplication
    pub fn scale(&self, factor: f64) -> EnuCoord {
        EnuCoord {
            east: self.east * factor,
            north: self.north * factor,
            up: self.up * factor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enu_origin_is_zero() {
        let origin = EarthCoord::new(40.0, -74.0, 0.0).unwrap();
        let frame = EnuFrame::new(origin);
        let enu = frame.geodetic_to_enu(&origin);
        assert!(enu.east.abs() < 1e-6);
        assert!(enu.north.abs() < 1e-6);
        assert!(enu.up.abs() < 1e-6);
    }

    #[test]
    fn test_enu_roundtrip() {
        let origin = EarthCoord::new(40.0, -74.0, 100.0).unwrap();
        let frame = EnuFrame::new(origin);
        let point = EarthCoord::new(40.001, -73.999, 110.0).unwrap();

        let enu = frame.geodetic_to_enu(&point);
        let back = frame.enu_to_geodetic(&enu);

        assert!((point.lat - back.lat).abs() < 1e-6);
        assert!((point.lon - back.lon).abs() < 1e-6);
    }

    #[test]
    fn test_azimuth_north() {
        let origin = EarthCoord::new(0.0, 0.0, 0.0).unwrap();
        let frame = EnuFrame::new(origin);
        let north = EnuCoord::new(0.0, 1000.0, 0.0);
        let az = frame.azimuth_to(&north);
        assert!((az - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_azimuth_east() {
        let origin = EarthCoord::new(0.0, 0.0, 0.0).unwrap();
        let frame = EnuFrame::new(origin);
        let east = EnuCoord::new(1000.0, 0.0, 0.0);
        let az = frame.azimuth_to(&east);
        assert!((az - 90.0).abs() < 1e-6);
    }
}
```

---

### 1.3 Intercept Geometry (`intercept.rs`)

**Current State:** Stub function returning None

**Gap:** Missing trajectory intersection calculations for convergence detection.

**Implementation:**

```rust
//! Intercept geometry calculations
//!
//! RFC-93X2: Calculate when/where trajectories converge
//! Used to predict threat/response convergence points.

use super::enu::EnuCoord;

/// 3D trajectory defined by position + velocity
#[derive(Debug, Clone, Copy)]
pub struct Trajectory {
    /// Current position (ENU meters)
    pub position: EnuCoord,
    /// Velocity vector (meters/second)
    pub velocity: EnuCoord,
}

/// Intercept result
#[derive(Debug, Clone)]
pub struct InterceptResult {
    /// Time to intercept (seconds from now)
    pub time_s: f64,
    /// Intercept position
    pub position: EnuCoord,
    /// Miss distance at closest approach (meters)
    pub miss_distance: f64,
    /// Confidence in intercept (0.0-1.0)
    pub confidence: f64,
}

impl Trajectory {
    /// Create new trajectory
    pub fn new(position: EnuCoord, velocity: EnuCoord) -> Self {
        Self { position, velocity }
    }

    /// Position at time t
    pub fn position_at(&self, t: f64) -> EnuCoord {
        EnuCoord {
            east: self.position.east + self.velocity.east * t,
            north: self.position.north + self.velocity.north * t,
            up: self.position.up + self.velocity.up * t,
        }
    }

    /// Speed magnitude
    pub fn speed(&self) -> f64 {
        self.velocity.distance()
    }
}

/// Calculate intercept between two trajectories
/// Returns None if trajectories are parallel or diverging
pub fn calculate_intercept(t1: &Trajectory, t2: &Trajectory) -> Option<InterceptResult> {
    // Find time of closest approach using calculus
    // Distance^2 = |p1 + v1*t - p2 - v2*t|^2
    // d(D^2)/dt = 0 gives us the minimum

    let dp = EnuCoord {
        east: t1.position.east - t2.position.east,
        north: t1.position.north - t2.position.north,
        up: t1.position.up - t2.position.up,
    };

    let dv = EnuCoord {
        east: t1.velocity.east - t2.velocity.east,
        north: t1.velocity.north - t2.velocity.north,
        up: t1.velocity.up - t2.velocity.up,
    };

    // a*t^2 + b*t + c = distance^2
    // Derivative: 2*a*t + b = 0 => t = -b/(2*a)
    let a = dv.east * dv.east + dv.north * dv.north + dv.up * dv.up;
    let b = 2.0 * (dp.east * dv.east + dp.north * dv.north + dp.up * dv.up);
    let c = dp.east * dp.east + dp.north * dp.north + dp.up * dp.up;

    // Parallel trajectories (same velocity)
    if a.abs() < 1e-10 {
        return None;
    }

    let t_closest = -b / (2.0 * a);

    // Negative time means closest approach was in the past
    if t_closest < 0.0 {
        return None;
    }

    // Calculate positions at closest approach
    let p1 = t1.position_at(t_closest);
    let p2 = t2.position_at(t_closest);

    let miss_distance = EnuCoord {
        east: p1.east - p2.east,
        north: p1.north - p2.north,
        up: p1.up - p2.up,
    }
    .distance();

    // Midpoint as intercept position
    let intercept_pos = EnuCoord {
        east: (p1.east + p2.east) / 2.0,
        north: (p1.north + p2.north) / 2.0,
        up: (p1.up + p2.up) / 2.0,
    };

    // Confidence based on miss distance (closer = higher confidence)
    // Using exponential decay: e^(-miss_distance/1000)
    let confidence = (-miss_distance / 1000.0).exp().min(1.0);

    Some(InterceptResult {
        time_s: t_closest,
        position: intercept_pos,
        miss_distance,
        confidence,
    })
}

/// Calculate intercept with lead (proportional navigation)
/// target: what we're intercepting
/// interceptor: our trajectory
/// speed: interceptor speed (we can adjust heading)
pub fn proportional_navigation(
    target: &Trajectory,
    interceptor_pos: &EnuCoord,
    interceptor_speed: f64,
    nav_gain: f64,
) -> Option<InterceptResult> {
    // Simplified PN: aim at predicted target position

    // Initial guess: time to target at current range / closing speed
    let initial_range = EnuCoord {
        east: target.position.east - interceptor_pos.east,
        north: target.position.north - interceptor_pos.north,
        up: target.position.up - interceptor_pos.up,
    }
    .distance();

    let closing_speed = interceptor_speed + target.speed() * 0.5; // Rough estimate
    let mut t_intercept = initial_range / closing_speed;

    // Iterate to refine intercept time
    for _ in 0..10 {
        let target_pos = target.position_at(t_intercept);
        let range_at_t = EnuCoord {
            east: target_pos.east - interceptor_pos.east,
            north: target_pos.north - interceptor_pos.north,
            up: target_pos.up - interceptor_pos.up,
        }
        .distance();

        t_intercept = range_at_t / interceptor_speed;
    }

    if t_intercept < 0.0 || t_intercept > 3600.0 {
        // Sanity check: no intercept if > 1 hour
        return None;
    }

    let intercept_pos = target.position_at(t_intercept);

    // Calculate required velocity vector
    let heading = EnuCoord {
        east: (intercept_pos.east - interceptor_pos.east) / t_intercept,
        north: (intercept_pos.north - interceptor_pos.north) / t_intercept,
        up: (intercept_pos.up - interceptor_pos.up) / t_intercept,
    };

    let miss_distance = 0.0; // Perfect intercept with PN

    // Confidence based on maneuverability (nav_gain factor)
    let confidence = (nav_gain / 5.0).min(1.0);

    Some(InterceptResult {
        time_s: t_intercept,
        position: intercept_pos,
        miss_distance,
        confidence,
    })
}

/// Check if two regions overlap (for zone convergence)
pub fn regions_overlap(
    center1: &EnuCoord,
    radius1: f64,
    center2: &EnuCoord,
    radius2: f64,
) -> bool {
    let dist = EnuCoord {
        east: center1.east - center2.east,
        north: center1.north - center2.north,
        up: center1.up - center2.up,
    }
    .distance();

    dist < (radius1 + radius2)
}

/// Calculate overlap confidence between two regions
/// RFC-93X2: MAY adjust confidence based on overlap
pub fn overlap_confidence(
    center1: &EnuCoord,
    radius1: f64,
    center2: &EnuCoord,
    radius2: f64,
) -> f64 {
    let dist = EnuCoord {
        east: center1.east - center2.east,
        north: center1.north - center2.north,
        up: center1.up - center2.up,
    }
    .distance();

    let combined = radius1 + radius2;

    if dist >= combined {
        0.0 // No overlap
    } else if dist <= (radius1 - radius2).abs() {
        1.0 // Complete containment
    } else {
        // Partial overlap: linear interpolation
        1.0 - (dist / combined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_on_collision() {
        let t1 = Trajectory::new(
            EnuCoord::new(0.0, 0.0, 0.0),
            EnuCoord::new(10.0, 0.0, 0.0),
        );
        let t2 = Trajectory::new(
            EnuCoord::new(100.0, 0.0, 0.0),
            EnuCoord::new(-10.0, 0.0, 0.0),
        );

        let result = calculate_intercept(&t1, &t2).unwrap();
        assert!((result.time_s - 5.0).abs() < 0.1);
        assert!(result.miss_distance < 1.0);
    }

    #[test]
    fn test_parallel_no_intercept() {
        let t1 = Trajectory::new(
            EnuCoord::new(0.0, 0.0, 0.0),
            EnuCoord::new(10.0, 0.0, 0.0),
        );
        let t2 = Trajectory::new(
            EnuCoord::new(0.0, 100.0, 0.0),
            EnuCoord::new(10.0, 0.0, 0.0),
        );

        let result = calculate_intercept(&t1, &t2);
        // Parallel, constant miss distance
        assert!(result.is_none() || result.unwrap().miss_distance > 50.0);
    }

    #[test]
    fn test_regions_overlap() {
        let c1 = EnuCoord::new(0.0, 0.0, 0.0);
        let c2 = EnuCoord::new(50.0, 0.0, 0.0);

        assert!(regions_overlap(&c1, 30.0, &c2, 30.0));
        assert!(!regions_overlap(&c1, 20.0, &c2, 20.0));
    }

    #[test]
    fn test_overlap_confidence() {
        let c1 = EnuCoord::new(0.0, 0.0, 0.0);
        let c2 = EnuCoord::new(0.0, 0.0, 0.0);

        // Same center = full overlap
        let conf = overlap_confidence(&c1, 100.0, &c2, 100.0);
        assert!((conf - 1.0).abs() < 0.01);
    }
}
```

---

## 2. Selection Subcrate (RFC-93X3)

### Constraints (from smartcrate.toml)
- **Algorithm:** deterministic-greedy
- **Ordering:** explicit
- **Deterministic:** true
- **Storage:** none

### 2.1 Partition Matroid (`partition.rs`)

**Current State:** Placeholder struct

**Gap:** Missing partition matroid constraint enforcement.

**Implementation:**

```rust
//! Partition Matroid for Selection Constraints
//!
//! RFC-93X3: Matroid-Constrained Selection Framework
//!
//! A partition matroid partitions the ground set into disjoint groups,
//! with capacity constraints on each group.

use std::collections::HashMap;

/// Element with weight and group assignment
#[derive(Debug, Clone)]
pub struct Element {
    /// Unique identifier
    pub id: usize,
    /// Weight/priority for selection
    pub weight: f64,
    /// Group this element belongs to
    pub group: usize,
}

/// Partition matroid constraint
#[derive(Debug, Clone)]
pub struct PartitionMatroid {
    /// Group capacities (max elements selectable from each group)
    capacities: HashMap<usize, usize>,
    /// Current selection count per group
    selected: HashMap<usize, usize>,
    /// Total elements in each group
    group_sizes: HashMap<usize, usize>,
}

impl PartitionMatroid {
    /// Create partition matroid with uniform capacity
    pub fn uniform(groups: &[usize], capacity: usize) -> Self {
        let mut capacities = HashMap::new();
        let mut group_sizes = HashMap::new();

        for &g in groups {
            capacities.insert(g, capacity);
            *group_sizes.entry(g).or_insert(0) += 1;
        }

        Self {
            capacities,
            selected: HashMap::new(),
            group_sizes,
        }
    }

    /// Create partition matroid with specified capacities
    pub fn with_capacities(caps: HashMap<usize, usize>) -> Self {
        Self {
            capacities: caps,
            selected: HashMap::new(),
            group_sizes: HashMap::new(),
        }
    }

    /// Check if element can be added (independence oracle)
    pub fn can_add(&self, element: &Element) -> bool {
        let current = self.selected.get(&element.group).copied().unwrap_or(0);
        let capacity = self.capacities.get(&element.group).copied().unwrap_or(0);
        current < capacity
    }

    /// Add element to selection (must check can_add first)
    pub fn add(&mut self, element: &Element) -> bool {
        if !self.can_add(element) {
            return false;
        }
        *self.selected.entry(element.group).or_insert(0) += 1;
        true
    }

    /// Remove element from selection
    pub fn remove(&mut self, element: &Element) {
        if let Some(count) = self.selected.get_mut(&element.group) {
            if *count > 0 {
                *count -= 1;
            }
        }
    }

    /// Reset selection state
    pub fn reset(&mut self) {
        self.selected.clear();
    }

    /// Get remaining capacity for a group
    pub fn remaining_capacity(&self, group: usize) -> usize {
        let current = self.selected.get(&group).copied().unwrap_or(0);
        let capacity = self.capacities.get(&group).copied().unwrap_or(0);
        capacity.saturating_sub(current)
    }

    /// Compute rank of current selection
    pub fn rank(&self) -> usize {
        self.selected.values().sum()
    }

    /// Maximum possible rank (sum of all capacities)
    pub fn max_rank(&self) -> usize {
        self.capacities.values().sum()
    }

    /// Check if selection is maximal (can't add any more)
    pub fn is_maximal(&self) -> bool {
        self.capacities.iter().all(|(g, cap)| {
            self.selected.get(g).copied().unwrap_or(0) >= *cap
        })
    }
}

/// Create partition from element assignments
pub fn elements_to_partition(elements: &[Element]) -> PartitionMatroid {
    let groups: Vec<usize> = elements.iter().map(|e| e.group).collect();
    PartitionMatroid::uniform(&groups, 1)
}

/// Helper: create disjoint groups from a flat list
pub fn create_groups(num_elements: usize, num_groups: usize) -> Vec<usize> {
    (0..num_elements)
        .map(|i| i % num_groups)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_constraint() {
        let mut matroid = PartitionMatroid::with_capacities(
            [(0, 2), (1, 1)].into_iter().collect()
        );

        let e1 = Element { id: 0, weight: 1.0, group: 0 };
        let e2 = Element { id: 1, weight: 1.0, group: 0 };
        let e3 = Element { id: 2, weight: 1.0, group: 0 };
        let e4 = Element { id: 3, weight: 1.0, group: 1 };

        assert!(matroid.add(&e1));
        assert!(matroid.add(&e2));
        assert!(!matroid.add(&e3)); // Group 0 full
        assert!(matroid.add(&e4));
        assert_eq!(matroid.rank(), 3);
    }

    #[test]
    fn test_remaining_capacity() {
        let mut matroid = PartitionMatroid::with_capacities(
            [(0, 3)].into_iter().collect()
        );

        assert_eq!(matroid.remaining_capacity(0), 3);

        let e = Element { id: 0, weight: 1.0, group: 0 };
        matroid.add(&e);

        assert_eq!(matroid.remaining_capacity(0), 2);
    }
}
```

---

### 2.2 Laminar Matroid (`laminar.rs`)

**Current State:** Placeholder struct

**Gap:** Missing laminar family (nested set) constraint.

**Implementation:**

```rust
//! Laminar Matroid Family
//!
//! RFC-93X3: Hierarchical nested-set constraints
//!
//! A laminar family is a collection of sets where any two sets are either
//! disjoint or one contains the other. This models hierarchical constraints
//! like "max 5 from category A, max 2 from subcategory A1".

use std::collections::{HashMap, HashSet};

/// A set in the laminar family with capacity constraint
#[derive(Debug, Clone)]
pub struct LaminarSet {
    /// Set identifier
    pub id: usize,
    /// Elements in this set
    pub elements: HashSet<usize>,
    /// Maximum elements selectable from this set
    pub capacity: usize,
    /// Parent set (None if root)
    pub parent: Option<usize>,
    /// Child sets
    pub children: Vec<usize>,
}

/// Laminar matroid constraint system
#[derive(Debug, Clone)]
pub struct LaminarMatroid {
    /// All sets in the family
    sets: HashMap<usize, LaminarSet>,
    /// Root set IDs (sets with no parent)
    roots: Vec<usize>,
    /// Current selection count per set
    selected: HashMap<usize, usize>,
    /// Element to set membership (element -> all containing sets)
    element_sets: HashMap<usize, Vec<usize>>,
}

impl LaminarMatroid {
    /// Create empty laminar matroid
    pub fn new() -> Self {
        Self {
            sets: HashMap::new(),
            roots: Vec::new(),
            selected: HashMap::new(),
            element_sets: HashMap::new(),
        }
    }

    /// Add a set to the family
    pub fn add_set(&mut self, set: LaminarSet) {
        let set_id = set.id;

        // Update element membership
        for &elem in &set.elements {
            self.element_sets
                .entry(elem)
                .or_insert_with(Vec::new)
                .push(set_id);
        }

        // Track roots
        if set.parent.is_none() {
            self.roots.push(set_id);
        }

        self.sets.insert(set_id, set);
    }

    /// Check if adding element violates any constraint
    pub fn can_add(&self, element: usize) -> bool {
        let containing_sets = match self.element_sets.get(&element) {
            Some(sets) => sets,
            None => return true, // Element not in any constrained set
        };

        for &set_id in containing_sets {
            if let Some(set) = self.sets.get(&set_id) {
                let current = self.selected.get(&set_id).copied().unwrap_or(0);
                if current >= set.capacity {
                    return false;
                }
            }
        }

        true
    }

    /// Add element to selection
    pub fn add(&mut self, element: usize) -> bool {
        if !self.can_add(element) {
            return false;
        }

        if let Some(sets) = self.element_sets.get(&element) {
            for &set_id in sets {
                *self.selected.entry(set_id).or_insert(0) += 1;
            }
        }

        true
    }

    /// Remove element from selection
    pub fn remove(&mut self, element: usize) {
        if let Some(sets) = self.element_sets.get(&element) {
            for &set_id in sets {
                if let Some(count) = self.selected.get_mut(&set_id) {
                    if *count > 0 {
                        *count -= 1;
                    }
                }
            }
        }
    }

    /// Reset selection state
    pub fn reset(&mut self) {
        self.selected.clear();
    }

    /// Get effective capacity for an element
    /// (minimum remaining capacity across all containing sets)
    pub fn effective_capacity(&self, element: usize) -> usize {
        let containing_sets = match self.element_sets.get(&element) {
            Some(sets) => sets,
            None => return usize::MAX,
        };

        containing_sets
            .iter()
            .filter_map(|&set_id| {
                self.sets.get(&set_id).map(|set| {
                    let current = self.selected.get(&set_id).copied().unwrap_or(0);
                    set.capacity.saturating_sub(current)
                })
            })
            .min()
            .unwrap_or(usize::MAX)
    }

    /// Compute rank (total selected)
    pub fn rank(&self) -> usize {
        // Sum of root-level selections (to avoid double-counting nested)
        self.roots
            .iter()
            .map(|&id| self.selected.get(&id).copied().unwrap_or(0))
            .sum()
    }

    /// Verify laminar property (sets are disjoint or nested)
    pub fn verify_laminar(&self) -> bool {
        let set_ids: Vec<usize> = self.sets.keys().copied().collect();

        for i in 0..set_ids.len() {
            for j in (i + 1)..set_ids.len() {
                let s1 = &self.sets[&set_ids[i]];
                let s2 = &self.sets[&set_ids[j]];

                let intersection: HashSet<_> =
                    s1.elements.intersection(&s2.elements).collect();

                // Must be disjoint, s1 ⊆ s2, or s2 ⊆ s1
                if !intersection.is_empty()
                    && intersection.len() != s1.elements.len()
                    && intersection.len() != s2.elements.len()
                {
                    return false;
                }
            }
        }

        true
    }
}

impl Default for LaminarMatroid {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for hierarchical laminar families
pub struct LaminarBuilder {
    matroid: LaminarMatroid,
    next_id: usize,
}

impl LaminarBuilder {
    pub fn new() -> Self {
        Self {
            matroid: LaminarMatroid::new(),
            next_id: 0,
        }
    }

    /// Add root set
    pub fn add_root(&mut self, elements: HashSet<usize>, capacity: usize) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.matroid.add_set(LaminarSet {
            id,
            elements,
            capacity,
            parent: None,
            children: Vec::new(),
        });

        id
    }

    /// Add child set (must be subset of parent)
    pub fn add_child(
        &mut self,
        parent_id: usize,
        elements: HashSet<usize>,
        capacity: usize,
    ) -> Option<usize> {
        // Verify subset relationship
        if let Some(parent) = self.matroid.sets.get(&parent_id) {
            if !elements.is_subset(&parent.elements) {
                return None;
            }
        } else {
            return None;
        }

        let id = self.next_id;
        self.next_id += 1;

        self.matroid.add_set(LaminarSet {
            id,
            elements,
            capacity,
            parent: Some(parent_id),
            children: Vec::new(),
        });

        // Update parent's children
        if let Some(parent) = self.matroid.sets.get_mut(&parent_id) {
            parent.children.push(id);
        }

        Some(id)
    }

    pub fn build(self) -> LaminarMatroid {
        self.matroid
    }
}

impl Default for LaminarBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_laminar_hierarchy() {
        let mut builder = LaminarBuilder::new();

        // Root: max 3 from {0,1,2,3,4}
        let root = builder.add_root([0, 1, 2, 3, 4].into_iter().collect(), 3);

        // Child: max 1 from {0,1}
        builder.add_child(root, [0, 1].into_iter().collect(), 1);

        let mut matroid = builder.build();

        // Can add element 0
        assert!(matroid.add(0));
        // Cannot add element 1 (child constraint hit)
        assert!(!matroid.can_add(1));
        // Can add element 2
        assert!(matroid.add(2));
        assert!(matroid.add(3));
        // Cannot add element 4 (root constraint hit)
        assert!(!matroid.can_add(4));
    }

    #[test]
    fn test_verify_laminar() {
        let mut builder = LaminarBuilder::new();
        let root = builder.add_root([0, 1, 2, 3].into_iter().collect(), 2);
        builder.add_child(root, [0, 1].into_iter().collect(), 1);

        let matroid = builder.build();
        assert!(matroid.verify_laminar());
    }
}
```

---

### 2.3 Greedy Selector (`greedy.rs`)

**Current State:** Stub returning empty

**Gap:** Missing deterministic greedy selection algorithm.

**Implementation:**

```rust
//! Deterministic Greedy Selection Algorithm
//!
//! RFC-93X3: Matroid-constrained selection with explicit ordering
//!
//! Implements the classic greedy algorithm for matroid optimization:
//! Sort by weight descending, add if independent.

use super::laminar::LaminarMatroid;
use super::partition::{Element, PartitionMatroid};
use std::cmp::Ordering;

/// Selection result
#[derive(Debug, Clone)]
pub struct SelectionResult {
    /// Selected element IDs (in selection order)
    pub selected: Vec<usize>,
    /// Total weight of selection
    pub total_weight: f64,
    /// Elements rejected (with reason)
    pub rejected: Vec<(usize, String)>,
}

/// Greedy selector for partition matroids
#[derive(Debug)]
pub struct GreedySelector {
    /// Elements to select from
    elements: Vec<Element>,
    /// Selection result
    result: Option<SelectionResult>,
}

impl GreedySelector {
    /// Create new selector with elements
    pub fn new(elements: Vec<Element>) -> Self {
        Self {
            elements,
            result: None,
        }
    }

    /// Run greedy selection with partition constraint
    pub fn select_partition(&mut self, matroid: &mut PartitionMatroid) -> &SelectionResult {
        // Sort by weight descending (deterministic: use ID as tiebreaker)
        let mut sorted: Vec<&Element> = self.elements.iter().collect();
        sorted.sort_by(|a, b| {
            match b.weight.partial_cmp(&a.weight) {
                Some(Ordering::Equal) | None => a.id.cmp(&b.id),
                Some(ord) => ord,
            }
        });

        let mut selected = Vec::new();
        let mut rejected = Vec::new();
        let mut total_weight = 0.0;

        matroid.reset();

        for elem in sorted {
            if matroid.can_add(elem) {
                matroid.add(elem);
                selected.push(elem.id);
                total_weight += elem.weight;
            } else {
                rejected.push((
                    elem.id,
                    format!("Group {} at capacity", elem.group),
                ));
            }
        }

        self.result = Some(SelectionResult {
            selected,
            total_weight,
            rejected,
        });

        self.result.as_ref().unwrap()
    }

    /// Get result (if selection has been run)
    pub fn result(&self) -> Option<&SelectionResult> {
        self.result.as_ref()
    }
}

/// Greedy selector for laminar matroids
#[derive(Debug)]
pub struct LaminarGreedySelector {
    /// Element weights by ID
    weights: Vec<(usize, f64)>,
    /// Selection result
    result: Option<SelectionResult>,
}

impl LaminarGreedySelector {
    /// Create new selector with weights
    pub fn new(weights: Vec<(usize, f64)>) -> Self {
        Self {
            weights,
            result: None,
        }
    }

    /// Run greedy selection with laminar constraint
    pub fn select(&mut self, matroid: &mut LaminarMatroid) -> &SelectionResult {
        // Sort by weight descending (deterministic: use ID as tiebreaker)
        let mut sorted = self.weights.clone();
        sorted.sort_by(|a, b| {
            match b.1.partial_cmp(&a.1) {
                Some(Ordering::Equal) | None => a.0.cmp(&b.0),
                Some(ord) => ord,
            }
        });

        let mut selected = Vec::new();
        let mut rejected = Vec::new();
        let mut total_weight = 0.0;

        matroid.reset();

        for (id, weight) in sorted {
            if matroid.can_add(id) {
                matroid.add(id);
                selected.push(id);
                total_weight += weight;
            } else {
                rejected.push((id, "Laminar constraint violated".to_string()));
            }
        }

        self.result = Some(SelectionResult {
            selected,
            total_weight,
            rejected,
        });

        self.result.as_ref().unwrap()
    }
}

/// Intersection of two matroids (more complex)
/// For M1 ∩ M2, we need the matroid intersection algorithm
pub struct MatroidIntersection {
    partition: PartitionMatroid,
    laminar: LaminarMatroid,
}

impl MatroidIntersection {
    pub fn new(partition: PartitionMatroid, laminar: LaminarMatroid) -> Self {
        Self { partition, laminar }
    }

    /// Check if element can be added to both matroids
    pub fn can_add(&self, element: &Element) -> bool {
        self.partition.can_add(element) && self.laminar.can_add(element.id)
    }

    /// Greedy selection on intersection (suboptimal but deterministic)
    pub fn greedy_select(&mut self, elements: &[Element]) -> SelectionResult {
        let mut sorted: Vec<&Element> = elements.iter().collect();
        sorted.sort_by(|a, b| {
            match b.weight.partial_cmp(&a.weight) {
                Some(Ordering::Equal) | None => a.id.cmp(&b.id),
                Some(ord) => ord,
            }
        });

        let mut selected = Vec::new();
        let mut rejected = Vec::new();
        let mut total_weight = 0.0;

        self.partition.reset();
        self.laminar.reset();

        for elem in sorted {
            if self.can_add(elem) {
                self.partition.add(elem);
                self.laminar.add(elem.id);
                selected.push(elem.id);
                total_weight += elem.weight;
            } else {
                let reason = if !self.partition.can_add(elem) {
                    format!("Partition group {} full", elem.group)
                } else {
                    "Laminar constraint".to_string()
                };
                rejected.push((elem.id, reason));
            }
        }

        SelectionResult {
            selected,
            total_weight,
            rejected,
        }
    }
}

/// Weighted selection with convergence scoring
/// Combines tactical P/T/H values for ordering
pub fn convergence_weighted_selection(
    elements: &[(usize, f64, f64, f64)], // (id, p, t, h)
    partition: &mut PartitionMatroid,
    groups: &[usize],
) -> SelectionResult {
    // Convert to Elements with convergence-weighted scores
    let weighted: Vec<Element> = elements
        .iter()
        .zip(groups.iter())
        .map(|((id, p, t, h), &group)| {
            // RFC-9024: weighted convergence = 0.6*p + 0.4*h
            let weight = 0.6 * p + 0.4 * h;
            Element {
                id: *id,
                weight,
                group,
            }
        })
        .collect();

    let mut selector = GreedySelector::new(weighted);
    selector.select_partition(partition).clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_greedy_partition_selection() {
        let elements = vec![
            Element { id: 0, weight: 5.0, group: 0 },
            Element { id: 1, weight: 3.0, group: 0 },
            Element { id: 2, weight: 4.0, group: 1 },
            Element { id: 3, weight: 2.0, group: 1 },
        ];

        let mut matroid = PartitionMatroid::with_capacities(
            [(0, 1), (1, 1)].into_iter().collect()
        );

        let mut selector = GreedySelector::new(elements);
        let result = selector.select_partition(&mut matroid);

        // Should select highest from each group: 0 (5.0) and 2 (4.0)
        assert_eq!(result.selected, vec![0, 2]);
        assert!((result.total_weight - 9.0).abs() < 0.001);
    }

    #[test]
    fn test_deterministic_ordering() {
        // Same weights - should use ID as tiebreaker
        let elements = vec![
            Element { id: 2, weight: 5.0, group: 0 },
            Element { id: 0, weight: 5.0, group: 0 },
            Element { id: 1, weight: 5.0, group: 0 },
        ];

        let mut matroid = PartitionMatroid::with_capacities(
            [(0, 2)].into_iter().collect()
        );

        let mut selector = GreedySelector::new(elements);
        let result = selector.select_partition(&mut matroid);

        // Should select in ID order when weights equal: 0, 1
        assert_eq!(result.selected, vec![0, 1]);
    }

    #[test]
    fn test_convergence_weighted() {
        let elements = vec![
            (0, 0.9, 0.5, 0.7), // weight = 0.6*0.9 + 0.4*0.7 = 0.82
            (1, 0.5, 0.5, 0.9), // weight = 0.6*0.5 + 0.4*0.9 = 0.66
            (2, 0.8, 0.5, 0.8), // weight = 0.6*0.8 + 0.4*0.8 = 0.80
        ];
        let groups = vec![0, 0, 0];

        let mut matroid = PartitionMatroid::with_capacities(
            [(0, 2)].into_iter().collect()
        );

        let result = convergence_weighted_selection(&elements, &mut matroid, &groups);

        // Should select 0 (0.82) and 2 (0.80)
        assert_eq!(result.selected, vec![0, 2]);
    }
}
```

---

## 3. Integration Points

### 3.1 Wire to ConvergeDetector

Update `detector.rs` to use geometry for confidence adjustment:

```rust
use crate::geometry::{earth::EarthCoord, intercept::overlap_confidence};

impl ConvergeDetector {
    /// Adjust convergence based on geographic proximity
    pub fn adjust_for_proximity(
        &self,
        entity1_loc: &EarthCoord,
        entity2_loc: &EarthCoord,
        max_range_m: f64,
    ) -> f64 {
        entity1_loc.proximity_confidence(entity2_loc, max_range_m)
    }
}
```

### 3.2 Wire to ActionEvent Selection

Use selection for deterministic action-set prioritization:

```rust
use crate::selection::greedy::convergence_weighted_selection;

impl ConvergeDetector {
    /// Select top-k actions from candidates using matroid constraints
    pub fn select_actions(
        &self,
        candidates: &[ActionEvent],
        max_per_source: usize,
    ) -> Vec<ActionEvent> {
        // Implementation using greedy selector
    }
}
```

---

## 4. Summary Table

| File | Current LOC | Required LOC | Status |
|------|-------------|--------------|--------|
| `geometry/earth.rs` | 9 | ~280 | Stub |
| `geometry/enu.rs` | 7 | ~180 | Stub |
| `geometry/intercept.rs` | 6 | ~200 | Stub |
| `selection/partition.rs` | 6 | ~150 | Stub |
| `selection/laminar.rs` | 6 | ~220 | Stub |
| `selection/greedy.rs` | 13 | ~250 | Stub |
| **Total** | **47** | **~1280** | **3% complete** |

---

## 5. Implementation Order

1. **Phase 1:** `earth.rs` - Foundation for all geometry
2. **Phase 2:** `enu.rs` - Depends on earth.rs
3. **Phase 3:** `partition.rs` - Simplest matroid
4. **Phase 4:** `greedy.rs` - Uses partition
5. **Phase 5:** `laminar.rs` - More complex matroid
6. **Phase 6:** `intercept.rs` - Uses enu.rs
7. **Phase 7:** Integration with ConvergeDetector

---

## 6. GLAF System Status

### 6.1 Overview

GLAF (Graph Learning & Analytics Fabric) spans two repositories:

| Component | Location | Language | Purpose |
|-----------|----------|----------|---------|
| **graph-db** | `/Users/cp5337/Developer/graph-db/` | TypeScript/React | GLAF UI & Hot Path |
| **sx9-glaf-core** | `crates/sx9-glaf-core/` | Rust | Convergence algorithms |
| **sx9-ops-main** | `apps/sx9-ops-main/src/components/glaf/` | TypeScript/React | OPS integration |

### 6.2 graph-db (External GLAF UI)

**Status:** ~75% Implemented, ~40 TypeScript errors pending

#### Core GLAF Modules (`src/lib/glaf/`)

| File | LOC | Status | Description |
|------|-----|--------|-------------|
| `legionHotPath.ts` | 244 | ✅ Complete | Nonagon routing, Crystal tuning, Unicode addressing |
| `synaptixBridge.ts` | 365 | ✅ Complete | Workflow↔GLAF bridge, execution recording |
| `apecsLayer.ts` | ~400 | ✅ Complete | Async processing layer |
| `ringBuffer.ts` | 228 | ✅ Complete | SPSC lock-free ring buffer |
| `orbital.ts` | 95 | ⚠️ Stub | Satellite data feeds |
| `plasma.ts` | 53 | ⚠️ Stub | Plasma-ECS integration |
| `threat_intel.ts` | 53 | ⚠️ Stub | Threat intelligence feeds |

#### Graph Library (`src/lib/graph/`)

| File | LOC | Status | Description |
|------|-----|--------|-------------|
| `GraphNode.ts` | 240 | ✅ Complete | Node types, styles, ACOG support |
| `GraphEdge.ts` | 215 | ✅ Complete | Edge types, markers, animations |
| `networking.ts` | 410 | ✅ Complete | Network scan visualization |
| `constants.ts` | 256 | ✅ Complete | Colors, shapes, layouts |
| `examples.ts` | 405 | ✅ Complete | Demo graph data |

#### TypeScript Fixes Pending

```
~40 errors total (see TYPESCRIPT_FIXES_PENDING.md):
- Path alias configuration (@/*)
- Missing type properties (opacity, type, node_type)
- Optional property access fixes
- Backend files exclusion (glaf-intel.ts, neon-graph-api.ts)
```

### 6.3 sx9-glaf-core (Rust Crate)

**Status:** 85% Implemented

| Module | LOC | Status | Description |
|--------|-----|--------|-------------|
| `convergence.rs` | 172 | ✅ Complete | H1/H2 dual convergence, RFC-9024/9025 |
| `hawkes.rs` | 57 | ✅ Complete | Hawkes process intensity λ(t) |
| `hmm.rs` | 100 | ✅ Complete | Phase detection (Recon→Staging→Exec→Exfil) |
| `teth.rs` | 79 | ✅ Complete | Graph entropy H(G) |
| `matroid.rs` | 83 | ⚠️ Basic | Latent matroid rank (needs full impl) |
| `glaf_core.rs` | 84 | ✅ Complete | GLAFCore engine, Node/Edge management |
| `types.rs` | ~100 | ✅ Complete | Node, Edge, NodeChange types |
| `graph.rs` | ~150 | ✅ Complete | Graph operations |
| `trivariate.rs` | ~80 | ⚠️ Basic | Hash stub (impl in sx9-hashing-engine) |

#### Key Algorithms Implemented

**Dual Convergence (RFC-9024/9025):**
```rust
// Simple: (h1 + h2) / 2.0
// Weighted: 0.6 * h1 + 0.4 * h2
// If delta > 0.1, weighted catches edge cases
```

**Hawkes Process:**
```rust
// λ(t) = μ + Σ α × e^(-β(t-tᵢ))
// Default: μ=0.1, α=0.5, β=1.0
```

**HMM Phase Transitions:**
```
[Recon] → [Staging] → [Execution] → [Exfil]
   0.6       0.5         0.6         0.9  (stay probability)
```

---

## 7. UI System Status (sx9-forge)

### 7.1 Architecture

**Location:** `sx9-forge/` (Tauri + React + Vite)

| Component | Status | Description |
|-----------|--------|-------------|
| PromptForgeScreen | ✅ 1783 LOC | Main UI screen |
| Tauri Commands | ✅ Implemented | save_prompt, create_linear_issue, notify_slack |
| Redux Store | ✅ Connected | Leptose, ChromaDB status |
| YAML Generation | ✅ Working | Prompt template output |
| Template Loading | ✅ Working | Load/save YAML prompts |

### 7.2 UI Layout

```
┌─────────────────────────────────────────────────────────────┐
│ HEADER: Title | RFC | Phase | [Copy] [Generate]             │
├─────────────────────────────────────────────────────────────┤
│ LEFT RAIL  │        CENTER         │      RIGHT RAIL        │
│ (Actions)  │     (YAML Editor)     │      (Context)         │
│            │                       │                        │
│ • harness  │   1│ prompt:          │  Tabs:                 │
│ • persona  │   2│   title: "..."   │  • intel               │
│ • agents   │   3│   rfc: RFC-XXXX  │  • tools               │
│ • linear   │   4│   phase: IMPL    │  • threats             │
│ • slack    │   5│   harness: ...   │  • qa                  │
│ • context  │   6│   persona: FORGE │                        │
│            │   7│   agents: [...]  │  Leptose: [status]     │
│            │                       │  ChromaDB: [status]    │
├─────────────────────────────────────────────────────────────┤
│ STATUS BAR: Feedback message | Timestamp                    │
└─────────────────────────────────────────────────────────────┘
```

### 7.3 CLSGS Agents (Annex A.2)

| Agent | Domain | Tools |
|-------|--------|-------|
| FORGE | Code Generation | Filesystem, CI/CD, MCP |
| AXIOM | Analysis | Math reasoning, Figma |
| VECTOR | Architecture | Read-only audits, deps |
| SENTINEL | Security | MITRE ATT&CK, vuln scan |
| GUARDIAN | QA | Test coverage, gates |
| ORACLE | Research | Web search, synthesis |
| SCRIBE | Documentation | RFC generation |
| RELAY | Integration | API bridges |
| ARBITER | Decision | Conflict resolution |
| WEAVER | Orchestration | Multi-agent coordination |

### 7.4 Workflow Status

**Current Flow:**
1. ✅ New/Edit prompt via UI
2. ✅ Configure harness mode (Build/Review/Deploy)
3. ✅ Select persona (CLSGS agents)
4. ✅ Set Linear team and Slack channel
5. ✅ Generate YAML output
6. ✅ Save to disk via Tauri command
7. ✅ Create Linear issue (optional)
8. ✅ Notify Slack (optional)

**Gaps:**
- ⚠️ Schedule execution (placeholder only)
- ⚠️ Direct agent dispatch (manual via YAML)
- ⚠️ Real-time harness status (mocked)

---

## 8. graph-db Integration Points

### 8.1 Connection to sx9

| graph-db | sx9 Equivalent | Status |
|----------|----------------|--------|
| `src/lib/gateway.ts` | `sx9-gateway-primary` | 🔗 Via NeuralMux |
| `src/lib/vaultClient.ts` | `sx9-foundation-core/keyvault` | 🔗 Via /vault/* |
| `src/lib/glaf/legionHotPath.ts` | `sx9-glaf-core` | ⚠️ Port to Rust |
| `src/lib/workflow/executor.ts` | `sx9-harness` | ⚠️ Needs bridge |
| `src/components/` | `sx9-ops-main/glaf/` | 🔄 Merge required |

### 8.2 Recommended Consolidation

1. **Phase 1:** Port `legionHotPath.ts` → Rust in sx9-glaf-core
2. **Phase 2:** Wire workflow executor → sx9-harness agents
3. **Phase 3:** Merge graph-db components → sx9-ops-main/glaf/
4. **Phase 4:** Unify persistence (Sled → SledIS multi-realm)

---

## 9. Summary: All Implementation Gaps

| Area | Component | Gap | Priority |
|------|-----------|-----|----------|
| Geometry | `earth.rs` | Full WGS84 impl | HIGH |
| Geometry | `enu.rs` | Local tangent plane | HIGH |
| Geometry | `intercept.rs` | Trajectory intersection | MEDIUM |
| Selection | `partition.rs` | Partition matroid | HIGH |
| Selection | `laminar.rs` | Laminar matroid | MEDIUM |
| Selection | `greedy.rs` | Deterministic greedy | HIGH |
| GLAF | `matroid.rs` | Full LatentMatroid | MEDIUM |
| GLAF | `trivariate.rs` | Wire to hashing-engine | LOW |
| graph-db | TypeScript | 40 type errors | HIGH |
| graph-db | orbital.ts | Satellite feeds | LOW |
| UI | Schedule | Cron execution | LOW |
| UI | Agent dispatch | Direct invocation | MEDIUM |
