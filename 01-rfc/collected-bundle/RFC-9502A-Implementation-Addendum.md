# RFC-9502A: Walker Delta Implementation Addendum

## Purpose

This addendum specifies the Rust backend implementation requirements for RFC-9502 (Walker Delta Constellation). It defines the Legion ECS components, systems, APECS handlers, and ring buffer events required to support orbital operations within the GLAF architecture.

**Target:** AntiGravity / Rust backend implementation  
**Prerequisite:** RFC-9500 (GLAF), RFC-9501 (Integration), RFC-9502 (Walker Delta)  
**Author:** Charles E. Payne  
**Date:** December 2025

---

## 1. Legion ECS Components

All components must be `#[repr(C)]` for cache alignment and FFI compatibility.

### 1.1 OrbitalIdentity Component

```rust
/// Identifies a satellite in the Walker constellation
#[derive(Clone, Copy, Debug, Component)]
#[repr(C)]
pub struct SatelliteIdentity {
    /// Unicode codepoint E300-E30B
    pub unicode: u32,
    /// Walker plane (1-3)
    pub plane: u8,
    /// Slot within plane (1-4)
    pub slot: u8,
    /// Right Ascension of Ascending Node (degrees)
    pub raan_deg: f32,
    /// Initial mean anomaly (degrees)
    pub mean_anomaly_deg: f32,
}

impl SatelliteIdentity {
    pub const UNICODE_BASE: u32 = 0xE300;
    pub const UNICODE_MAX: u32 = 0xE30B;
    
    pub fn from_unicode(unicode: u32) -> Option<Self> {
        if unicode < Self::UNICODE_BASE || unicode > Self::UNICODE_MAX {
            return None;
        }
        
        let index = (unicode - Self::UNICODE_BASE) as u8;
        let plane = (index / 4) + 1;  // 1, 2, or 3
        let slot = (index % 4) + 1;   // 1, 2, 3, or 4
        let raan_deg = ((plane - 1) as f32) * 120.0;
        let mean_anomaly_deg = ((slot - 1) as f32) * 90.0 + ((plane - 1) as f32) * 30.0;
        
        Some(Self {
            unicode,
            plane,
            slot,
            raan_deg,
            mean_anomaly_deg,
        })
    }
}
```

### 1.2 GroundStationIdentity Component

```rust
/// Identifies a ground station
#[derive(Clone, Copy, Debug, Component)]
#[repr(C)]
pub struct GroundStationIdentity {
    /// Unicode codepoint E500-E5FF
    pub unicode: u32,
    /// Station code (3-letter)
    pub code: [u8; 4],  // null-terminated
    /// Latitude (degrees, -90 to +90)
    pub lat_deg: f32,
    /// Longitude (degrees, -180 to +180)
    pub lon_deg: f32,
    /// Altitude above sea level (meters)
    pub alt_m: f32,
    /// Minimum elevation angle (degrees)
    pub min_elevation_deg: f32,
    /// Capabilities bitfield
    pub capabilities: GroundStationCapabilities,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct GroundStationCapabilities: u16 {
        const FSO_UPLINK   = 0b00000001;
        const FSO_DOWNLINK = 0b00000010;
        const RF_BACKUP    = 0b00000100;
        const TTC          = 0b00001000;  // Telemetry, Tracking, Command
        const GATEWAY      = 0b00010000;
    }
}
```

### 1.3 OrbitalState Component

```rust
/// Current orbital state (updated by PropagationSystem)
#[derive(Clone, Copy, Debug, Component)]
#[repr(C, align(64))]  // Cache-aligned for hot path
pub struct OrbitalState {
    /// Timestamp (Unix milliseconds)
    pub timestamp_ms: i64,
    
    /// Position in ECI frame (km)
    pub position_eci: Vec3,
    
    /// Velocity in ECI frame (km/s)
    pub velocity_eci: Vec3,
    
    /// Geodetic coordinates (for display)
    pub lat_deg: f32,
    pub lon_deg: f32,
    pub alt_km: f32,
    
    /// Orbital elements (for propagation)
    pub semi_major_axis_km: f32,
    pub eccentricity: f32,
    pub inclination_deg: f32,
    pub raan_deg: f32,
    pub arg_perigee_deg: f32,
    pub true_anomaly_deg: f32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

### 1.4 LinkState Component

```rust
/// State of a satellite-to-ground-station link
#[derive(Clone, Copy, Debug, Component)]
#[repr(C)]
pub struct LinkState {
    /// Satellite unicode (E300-E30B)
    pub satellite: u32,
    /// Ground station unicode (E500-E5FF)
    pub ground_station: u32,
    /// Current link status
    pub status: LinkStatus,
    /// Link metrics
    pub metrics: LinkMetrics,
    /// Last update timestamp
    pub updated_ms: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LinkStatus {
    /// No line of sight
    NotVisible = 0,
    /// Line of sight but link not established
    Visible = 1,
    /// Acquiring beacon
    Acquiring = 2,
    /// FSO link locked
    Locked = 3,
    /// Link degraded (weather, pointing)
    Degraded = 4,
    /// Using RF backup
    RFBackup = 5,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct LinkMetrics {
    /// Elevation angle from ground station (degrees)
    pub elevation_deg: f32,
    /// Azimuth angle from ground station (degrees)
    pub azimuth_deg: f32,
    /// Slant range (km)
    pub range_km: f32,
    /// Received signal power (dBm)
    pub signal_dbm: f32,
    /// Signal-to-noise ratio (dB)
    pub snr_db: f32,
    /// Current bit error rate
    pub ber: f64,
    /// Effective data rate (Gbps)
    pub data_rate_gbps: f32,
    /// Link margin (dB)
    pub margin_db: f32,
}
```

### 1.5 VisibilityCache Component

```rust
/// Cached visibility data (updated by VisibilitySystem)
#[derive(Clone, Debug, Component)]
pub struct VisibilityCache {
    /// Bitmap of visible ground stations (indexed by GS unicode - 0xE500)
    pub visible_stations: BitSet<256>,
    /// Next AOS (Acquisition of Signal) times per station
    pub next_aos: [Option<i64>; 16],  // Top 16 stations
    /// Next LOS (Loss of Signal) times per station
    pub next_los: [Option<i64>; 16],
    /// Cache validity timestamp
    pub valid_until_ms: i64,
}
```

---

## 2. Legion Systems

Systems execute in the hot path and must maintain <1µs latency for query routing.

### 2.1 PropagationSystem

Updates satellite positions using SGP4/SDP4 or Kepler propagation.

```rust
/// Propagates orbital states forward in time
#[system(for_each)]
pub fn propagation_system(
    identity: &SatelliteIdentity,
    state: &mut OrbitalState,
    #[resource] clock: &SimulationClock,
    #[resource] propagator: &OrbitalPropagator,
) {
    let dt_sec = (clock.now_ms - state.timestamp_ms) as f64 / 1000.0;
    
    if dt_sec < 0.1 {
        return;  // Skip if <100ms since last update
    }
    
    // Propagate orbital elements
    let new_state = propagator.propagate(
        state.semi_major_axis_km,
        state.eccentricity,
        state.inclination_deg,
        state.raan_deg,
        state.arg_perigee_deg,
        state.true_anomaly_deg,
        dt_sec,
    );
    
    // Update ECI position/velocity
    state.position_eci = new_state.position;
    state.velocity_eci = new_state.velocity;
    
    // Convert to geodetic
    let lla = eci_to_lla(&new_state.position, clock.now_ms);
    state.lat_deg = lla.lat as f32;
    state.lon_deg = lla.lon as f32;
    state.alt_km = lla.alt as f32;
    
    // Update elements
    state.true_anomaly_deg = new_state.true_anomaly;
    state.raan_deg = new_state.raan;  // Includes J2 drift
    state.timestamp_ms = clock.now_ms;
}
```

### 2.2 VisibilitySystem

Computes satellite-ground station visibility.

```rust
/// Computes visibility between satellites and ground stations
#[system]
pub fn visibility_system(
    world: &mut SubWorld,
    #[resource] gs_registry: &GroundStationRegistry,
    #[resource] clock: &SimulationClock,
) {
    // Query all satellites
    let mut sat_query = <(&SatelliteIdentity, &OrbitalState, &mut VisibilityCache)>::query();
    
    for (sat_id, sat_state, vis_cache) in sat_query.iter_mut(world) {
        // Skip if cache still valid
        if vis_cache.valid_until_ms > clock.now_ms {
            continue;
        }
        
        vis_cache.visible_stations.clear();
        
        for gs in gs_registry.iter() {
            let elevation = compute_elevation(
                sat_state.position_eci,
                gs.lat_deg,
                gs.lon_deg,
                gs.alt_m,
                clock.now_ms,
            );
            
            if elevation >= gs.min_elevation_deg {
                let gs_index = (gs.unicode - 0xE500) as usize;
                vis_cache.visible_stations.insert(gs_index);
            }
        }
        
        // Cache valid for 10 seconds
        vis_cache.valid_until_ms = clock.now_ms + 10_000;
    }
}

fn compute_elevation(
    sat_eci: Vec3,
    gs_lat: f32,
    gs_lon: f32,
    gs_alt: f32,
    time_ms: i64,
) -> f32 {
    // Convert ground station to ECI
    let gs_eci = lla_to_eci(gs_lat as f64, gs_lon as f64, gs_alt as f64 / 1000.0, time_ms);
    
    // Vector from GS to satellite
    let to_sat = Vec3 {
        x: sat_eci.x - gs_eci.x,
        y: sat_eci.y - gs_eci.y,
        z: sat_eci.z - gs_eci.z,
    };
    
    // Local Up vector at ground station
    let up = gs_eci.normalize();
    
    // Elevation = 90° - angle between to_sat and up
    let cos_angle = to_sat.normalize().dot(&up);
    90.0 - cos_angle.acos().to_degrees() as f32
}
```

### 2.3 LinkBudgetSystem

Calculates real-time link margins.

```rust
/// Updates link metrics for all active links
#[system(for_each)]
pub fn link_budget_system(
    link: &mut LinkState,
    #[resource] sat_states: &SatelliteStateMap,
    #[resource] gs_registry: &GroundStationRegistry,
    #[resource] weather: &WeatherData,
    #[resource] clock: &SimulationClock,
) {
    let sat_state = match sat_states.get(link.satellite) {
        Some(s) => s,
        None => return,
    };
    
    let gs = match gs_registry.get(link.ground_station) {
        Some(g) => g,
        None => return,
    };
    
    // Compute geometry
    let (elevation, azimuth, range) = compute_look_angles(
        sat_state.position_eci,
        gs.lat_deg, gs.lon_deg, gs.alt_m,
        clock.now_ms,
    );
    
    link.metrics.elevation_deg = elevation;
    link.metrics.azimuth_deg = azimuth;
    link.metrics.range_km = range;
    
    // Check visibility
    if elevation < gs.min_elevation_deg {
        link.status = LinkStatus::NotVisible;
        return;
    }
    
    // Link budget calculation (dB)
    let tx_power_dbm = 40.0;  // 10W
    let tx_gain_dbi = 109.2;  // 10cm aperture at 1550nm
    let rx_gain_dbi = 117.1;  // 25cm aperture
    let fspl_db = 20.0 * (range * 1e6).log10() + 20.0 * (193.4e12_f64).log10() - 147.55;
    
    // Atmospheric loss based on elevation and weather
    let atm_loss_db = weather.get_attenuation(gs.unicode, elevation);
    
    // Pointing loss (assume 1 dB nominal)
    let pointing_loss_db = 1.0;
    
    // Received power
    let rx_power_dbm = tx_power_dbm + tx_gain_dbi + rx_gain_dbi 
        - fspl_db as f32 - atm_loss_db - pointing_loss_db - 2.5;  // Optical losses
    
    link.metrics.signal_dbm = rx_power_dbm;
    
    // Required power for 10 Gbps at BER 1e-9
    let required_dbm = -36.0;
    link.metrics.margin_db = rx_power_dbm - required_dbm;
    
    // Update status based on margin
    link.status = if link.metrics.margin_db > 6.0 {
        LinkStatus::Locked
    } else if link.metrics.margin_db > 3.0 {
        LinkStatus::Degraded
    } else if link.metrics.margin_db > 0.0 {
        LinkStatus::RFBackup
    } else {
        LinkStatus::Visible  // Can see but can't link
    };
    
    // Update data rate based on margin
    link.metrics.data_rate_gbps = if link.metrics.margin_db > 6.0 {
        10.0
    } else if link.metrics.margin_db > 3.0 {
        5.0
    } else if link.metrics.margin_db > 0.0 {
        1.0  // RF backup rate
    } else {
        0.0
    };
    
    link.updated_ms = clock.now_ms;
}
```

### 2.4 OrbitalQueryRouter

Routes GLAF queries for orbital assets.

```rust
/// Routes queries for orbital domain (nonagon nodes 2-3)
#[system(for_each)]
pub fn orbital_query_router(
    query: &IncomingQuery,
    #[resource] ring_buffer: &RingBuffer,
    #[resource] sat_registry: &SatelliteRegistry,
    #[resource] gs_registry: &GroundStationRegistry,
) {
    // Check if query targets orbital domain
    if !query.domains.intersects(NonagonDomains::KINETIC) {
        return;
    }
    
    // Route based on unicode range
    let results = match query.target_unicode {
        0xE300..=0xE30B => {
            // Satellite query
            sat_registry.query_with_crystal(query.crystal, query.filters)
        }
        0xE500..=0xE5FF => {
            // Ground station query
            gs_registry.query_with_crystal(query.crystal, query.filters)
        }
        _ => return,
    };
    
    // Push results to ring buffer for async processing
    ring_buffer.produce(&EventSlot {
        sequence: query.sequence,
        unicode: query.target_unicode,
        event_type: EventType::OrbitalQueryResult,
        priority: query.priority,
        payload: results.into_payload(),
    });
}
```

---

## 3. Ring Buffer Event Types

Add these variants to the existing `EventType` enum:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EventType {
    // Existing types (RFC-9500)
    Update = 0,
    Execute = 1,
    Complete = 2,
    Error = 3,
    
    // Orbital types (RFC-9502A)
    OrbitalPositionUpdate = 16,
    OrbitalLinkStateChange = 17,
    OrbitalPassScheduled = 18,
    OrbitalPassStarted = 19,
    OrbitalPassEnded = 20,
    OrbitalQueryResult = 21,
    OrbitalHealthUpdate = 22,
    OrbitalWeatherUpdate = 23,
}
```

### Event Payloads

```rust
/// Position update payload (fits in 48 bytes)
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct PositionUpdatePayload {
    pub lat_deg: f32,      // 4
    pub lon_deg: f32,      // 4
    pub alt_km: f32,       // 4
    pub vx_kms: f32,       // 4
    pub vy_kms: f32,       // 4
    pub vz_kms: f32,       // 4
    pub heading_deg: f32,  // 4
    pub speed_kms: f32,    // 4
    pub _reserved: [u8; 16], // 16
}  // Total: 48 bytes

/// Link state change payload (fits in 48 bytes)
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct LinkStatePayload {
    pub satellite: u32,       // 4
    pub ground_station: u32,  // 4
    pub old_status: u8,       // 1
    pub new_status: u8,       // 1
    pub elevation_deg: f32,   // 4
    pub range_km: f32,        // 4
    pub margin_db: f32,       // 4
    pub data_rate_gbps: f32,  // 4
    pub _reserved: [u8; 22],  // 22
}  // Total: 48 bytes
```

---

## 4. APECS Async Handlers

The APECS layer processes ring buffer events and interfaces with external systems.

### 4.1 OrbitalEventHandler

```rust
/// Handles orbital events from ring buffer
pub struct OrbitalEventHandler {
    jetstream: JetStreamContext,
    temporal: TemporalClient,
    metrics: MetricsCollector,
}

impl OrbitalEventHandler {
    pub async fn handle(&self, event: EventSlot) -> Result<()> {
        match event.event_type {
            EventType::OrbitalPositionUpdate => {
                self.handle_position_update(event).await
            }
            EventType::OrbitalLinkStateChange => {
                self.handle_link_state_change(event).await
            }
            EventType::OrbitalPassScheduled => {
                self.handle_pass_scheduled(event).await
            }
            _ => Ok(()),
        }
    }
    
    async fn handle_position_update(&self, event: EventSlot) -> Result<()> {
        let payload: PositionUpdatePayload = event.decode_payload();
        
        // Publish to JetStream at 1 Hz
        let subject = format!("glaf.orbital.position.{:04X}", event.unicode);
        self.jetstream.publish(subject, &payload).await?;
        
        // Update metrics
        self.metrics.record_position_update(event.unicode);
        
        Ok(())
    }
    
    async fn handle_link_state_change(&self, event: EventSlot) -> Result<()> {
        let payload: LinkStatePayload = event.decode_payload();
        
        // Publish to JetStream
        let subject = format!(
            "glaf.orbital.link.{:04X}.{:04X}",
            payload.satellite, payload.ground_station
        );
        self.jetstream.publish(subject, &payload).await?;
        
        // If link just locked, might trigger data transfer workflow
        if payload.new_status == LinkStatus::Locked as u8 
            && payload.old_status != LinkStatus::Locked as u8 
        {
            self.temporal.signal_workflow(
                "pending-transfers",
                "link-available",
                &LinkAvailableSignal {
                    satellite: payload.satellite,
                    ground_station: payload.ground_station,
                    data_rate_gbps: payload.data_rate_gbps,
                },
            ).await?;
        }
        
        Ok(())
    }
    
    async fn handle_pass_scheduled(&self, event: EventSlot) -> Result<()> {
        let payload: PassScheduledPayload = event.decode_payload();
        
        // Start Temporal workflow for pass execution
        self.temporal.start_workflow(
            "PassExecutionWorkflow",
            &PassExecutionInput {
                satellite: payload.satellite,
                ground_station: payload.ground_station,
                aos_time: payload.aos_time,
                los_time: payload.los_time,
                max_elevation: payload.max_elevation,
            },
        ).await?;
        
        Ok(())
    }
}
```

### 4.2 EphemerisPublisher

```rust
/// Publishes satellite ephemeris to JetStream at configured rate
pub struct EphemerisPublisher {
    jetstream: JetStreamContext,
    publish_rate_hz: f32,
    last_publish: HashMap<u32, Instant>,
}

impl EphemerisPublisher {
    pub async fn publish_if_due(&self, unicode: u32, state: &OrbitalState) -> Result<()> {
        let interval = Duration::from_secs_f32(1.0 / self.publish_rate_hz);
        let last = self.last_publish.get(&unicode).copied().unwrap_or(Instant::now() - interval);
        
        if last.elapsed() < interval {
            return Ok(());
        }
        
        let event = PositionEvent {
            unicode,
            timestamp: state.timestamp_ms,
            position: EciPosition {
                x_km: state.position_eci.x,
                y_km: state.position_eci.y,
                z_km: state.position_eci.z,
            },
            velocity: EciVelocity {
                vx: state.velocity_eci.x,
                vy: state.velocity_eci.y,
                vz: state.velocity_eci.z,
            },
            geodetic: LLA {
                lat_deg: state.lat_deg as f64,
                lon_deg: state.lon_deg as f64,
                alt_km: state.alt_km as f64,
            },
        };
        
        let subject = format!("glaf.orbital.position.{:04X}", unicode);
        self.jetstream.publish(subject, &event).await?;
        
        self.last_publish.insert(unicode, Instant::now());
        
        Ok(())
    }
}
```

### 4.3 PassScheduler

```rust
/// Monitors upcoming passes and schedules workflows
pub struct PassScheduler {
    temporal: TemporalClient,
    schedule_horizon: Duration,  // How far ahead to schedule
    scheduled_passes: HashSet<PassId>,
}

impl PassScheduler {
    pub async fn check_upcoming_passes(
        &mut self,
        satellites: &[SatelliteState],
        ground_stations: &[GroundStation],
    ) -> Result<()> {
        let now = Utc::now();
        let horizon = now + chrono::Duration::from_std(self.schedule_horizon)?;
        
        for sat in satellites {
            for gs in ground_stations {
                // Predict next pass
                let pass = predict_next_pass(
                    &sat.orbital_state,
                    gs.lat_deg, gs.lon_deg, gs.alt_m,
                    gs.min_elevation_deg,
                    now,
                    horizon,
                );
                
                if let Some(pass) = pass {
                    let pass_id = PassId::new(sat.unicode, gs.unicode, pass.aos);
                    
                    if !self.scheduled_passes.contains(&pass_id) {
                        // Schedule Temporal workflow
                        self.temporal.start_workflow(
                            "PassSchedulingWorkflow",
                            &PassRequest {
                                satellite: sat.unicode,
                                ground_station: gs.unicode,
                                time_window: TimeWindow {
                                    start: pass.aos,
                                    end: pass.los,
                                },
                                max_elevation: pass.max_elevation,
                            },
                        ).await?;
                        
                        self.scheduled_passes.insert(pass_id);
                    }
                }
            }
        }
        
        // Clean up old scheduled passes
        self.scheduled_passes.retain(|p| p.aos > now);
        
        Ok(())
    }
}
```

### 4.4 WeatherIntegration

```rust
/// Ingests weather data and updates link predictions
pub struct WeatherIntegration {
    weather_api: WeatherApiClient,
    cache: HashMap<u32, WeatherCondition>,
    cache_ttl: Duration,
}

impl WeatherIntegration {
    pub async fn update_ground_station_weather(&mut self, gs_unicode: u32) -> Result<()> {
        let gs = self.gs_registry.get(gs_unicode)?;
        
        let weather = self.weather_api.get_conditions(gs.lat_deg, gs.lon_deg).await?;
        
        self.cache.insert(gs_unicode, WeatherCondition {
            cloud_cover_pct: weather.cloud_cover,
            visibility_km: weather.visibility,
            rain_rate_mmh: weather.precipitation_rate,
            attenuation_db: self.calculate_attenuation(&weather),
            updated: Instant::now(),
        });
        
        // Publish weather update
        self.jetstream.publish(
            format!("glaf.orbital.weather.{:04X}", gs_unicode),
            &self.cache[&gs_unicode],
        ).await?;
        
        Ok(())
    }
    
    fn calculate_attenuation(&self, weather: &WeatherData) -> f32 {
        // FSO attenuation model
        let cloud_atten = match weather.cloud_cover {
            0..=10 => 0.2,
            11..=30 => 0.5,
            31..=60 => 2.0,
            61..=80 => 5.0,
            _ => 15.0,
        };
        
        let rain_atten = weather.precipitation_rate * 0.5;  // ~0.5 dB per mm/hr
        
        cloud_atten + rain_atten
    }
    
    pub fn get_attenuation(&self, gs_unicode: u32, elevation_deg: f32) -> f32 {
        let base_atten = self.cache.get(&gs_unicode)
            .map(|w| w.attenuation_db)
            .unwrap_or(0.5);
        
        // Scale by airmass (secant of zenith angle)
        let zenith_deg = 90.0 - elevation_deg;
        let airmass = 1.0 / zenith_deg.to_radians().cos().max(0.1);
        
        base_atten * airmass.min(10.0)
    }
}
```

---

## 5. World Initialization

```rust
/// Initialize Legion world with orbital entities
pub fn initialize_orbital_world(world: &mut World, resources: &mut Resources) {
    // Register orbital components
    world.register::<SatelliteIdentity>();
    world.register::<GroundStationIdentity>();
    world.register::<OrbitalState>();
    world.register::<LinkState>();
    world.register::<VisibilityCache>();
    
    // Create satellites (Walker 12/3/1)
    for unicode in 0xE300..=0xE30Bu32 {
        let identity = SatelliteIdentity::from_unicode(unicode).unwrap();
        let initial_state = compute_initial_state(&identity);
        
        world.push((
            identity,
            initial_state,
            VisibilityCache::default(),
        ));
    }
    
    // Create ground stations
    let ground_stations = vec![
        (0xE500, "SVB", 78.2, 15.6, 450.0, GroundStationCapabilities::TTC | GroundStationCapabilities::FSO_UPLINK | GroundStationCapabilities::FSO_DOWNLINK),
        (0xE501, "FAI", 64.8, -147.7, 135.0, GroundStationCapabilities::TTC | GroundStationCapabilities::FSO_UPLINK | GroundStationCapabilities::FSO_DOWNLINK),
        (0xE502, "SCL", -33.4, -70.6, 520.0, GroundStationCapabilities::TTC | GroundStationCapabilities::FSO_UPLINK | GroundStationCapabilities::FSO_DOWNLINK),
        (0xE503, "PER", -31.9, 115.9, 25.0, GroundStationCapabilities::TTC | GroundStationCapabilities::FSO_UPLINK | GroundStationCapabilities::FSO_DOWNLINK),
        (0xE504, "HNL", 21.3, -157.8, 5.0, GroundStationCapabilities::GATEWAY | GroundStationCapabilities::FSO_UPLINK | GroundStationCapabilities::FSO_DOWNLINK),
        (0xE505, "GUM", 13.4, 144.8, 80.0, GroundStationCapabilities::GATEWAY | GroundStationCapabilities::FSO_UPLINK | GroundStationCapabilities::FSO_DOWNLINK),
        (0xE506, "SIN", 1.3, 103.8, 15.0, GroundStationCapabilities::GATEWAY | GroundStationCapabilities::FSO_UPLINK | GroundStationCapabilities::FSO_DOWNLINK),
        (0xE507, "DXB", 25.3, 55.3, 5.0, GroundStationCapabilities::GATEWAY | GroundStationCapabilities::FSO_UPLINK | GroundStationCapabilities::FSO_DOWNLINK),
        (0xE508, "PDL", 37.7, -25.7, 35.0, GroundStationCapabilities::GATEWAY | GroundStationCapabilities::FSO_UPLINK | GroundStationCapabilities::FSO_DOWNLINK),
        (0xE509, "CPT", -33.9, 18.4, 45.0, GroundStationCapabilities::GATEWAY | GroundStationCapabilities::FSO_UPLINK | GroundStationCapabilities::FSO_DOWNLINK),
    ];
    
    for (unicode, code, lat, lon, alt, caps) in ground_stations {
        let mut code_bytes = [0u8; 4];
        code_bytes[..3].copy_from_slice(code.as_bytes());
        
        world.push((
            GroundStationIdentity {
                unicode,
                code: code_bytes,
                lat_deg: lat,
                lon_deg: lon,
                alt_m: alt,
                min_elevation_deg: 10.0,
                capabilities: caps,
            },
        ));
    }
    
    // Create link state entities for all sat-GS pairs
    for sat_unicode in 0xE300..=0xE30Bu32 {
        for gs_unicode in 0xE500..=0xE509u32 {
            world.push((
                LinkState {
                    satellite: sat_unicode,
                    ground_station: gs_unicode,
                    status: LinkStatus::NotVisible,
                    metrics: LinkMetrics::default(),
                    updated_ms: 0,
                },
            ));
        }
    }
    
    // Initialize resources
    resources.insert(SimulationClock::new());
    resources.insert(OrbitalPropagator::sgp4());
    resources.insert(SatelliteRegistry::new());
    resources.insert(GroundStationRegistry::new());
    resources.insert(WeatherData::default());
}
```

---

## 6. System Schedule

Add orbital systems to the Legion schedule:

```rust
pub fn build_orbital_schedule() -> Schedule {
    Schedule::builder()
        // Hot path systems (every tick, <1µs each)
        .add_system(propagation_system_system())
        .add_system(visibility_system_system())
        .add_system(link_budget_system_system())
        .add_system(orbital_query_router_system())
        
        // Flush to ring buffer
        .flush()
        
        .build()
}
```

---

## 7. APECS Integration

Wire up async handlers in the APECS consumer loop:

```rust
pub async fn orbital_apecs_loop(
    ring_buffer: Arc<RingBuffer>,
    jetstream: JetStreamContext,
    temporal: TemporalClient,
) {
    let orbital_handler = OrbitalEventHandler::new(jetstream.clone(), temporal.clone());
    let ephemeris_pub = EphemerisPublisher::new(jetstream.clone(), 1.0);  // 1 Hz
    let mut pass_scheduler = PassScheduler::new(temporal.clone(), Duration::from_secs(3600));
    let mut weather = WeatherIntegration::new(jetstream.clone());
    
    let mut interval = tokio::time::interval(Duration::from_millis(100));
    
    loop {
        interval.tick().await;
        
        // Process ring buffer events
        while let Some(event) = ring_buffer.consume() {
            if event.event_type as u8 >= 16 {  // Orbital event types
                orbital_handler.handle(event).await.ok();
            }
        }
        
        // Periodic tasks
        pass_scheduler.check_upcoming_passes(&satellites, &ground_stations).await.ok();
        
        // Update weather every 5 minutes
        for gs_unicode in 0xE500..=0xE509u32 {
            weather.update_ground_station_weather(gs_unicode).await.ok();
        }
    }
}
```

---

## 8. File Structure

```
src/
├── orbital/
│   ├── mod.rs
│   ├── components.rs       # SatelliteIdentity, OrbitalState, LinkState, etc.
│   ├── systems.rs          # PropagationSystem, VisibilitySystem, LinkBudgetSystem
│   ├── propagator.rs       # SGP4/Kepler propagation
│   ├── geometry.rs         # ECI/LLA conversions, look angle calculations
│   ├── link_budget.rs      # FSO link budget calculations
│   └── init.rs             # World initialization
├── apecs/
│   ├── mod.rs
│   ├── orbital_handler.rs  # OrbitalEventHandler
│   ├── ephemeris.rs        # EphemerisPublisher
│   ├── pass_scheduler.rs   # PassScheduler
│   └── weather.rs          # WeatherIntegration
└── events/
    └── orbital.rs          # Event types and payloads
```

---

## 9. Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
# Existing (RFC-9500)
legion = "0.4"
bitflags = "2.0"

# Orbital mechanics
sgp4 = "0.8"           # SGP4/SDP4 propagator
nalgebra = "0.32"      # Linear algebra
chrono = "0.4"         # Time handling

# Async
tokio = { version = "1", features = ["full"] }
async-nats = "0.33"    # JetStream

# Serialization
serde = { version = "1", features = ["derive"] }
bincode = "1.3"        # Binary serialization for payloads
```

---

## 10. Testing Requirements

1. **Unit Tests:**
   - `SatelliteIdentity::from_unicode()` produces correct plane/slot/RAAN/MA
   - Propagation matches reference ephemeris within 100m over 24h
   - Visibility calculation matches STK/GMAT reference
   - Link budget matches hand calculations within 0.5 dB

2. **Integration Tests:**
   - Full pass simulation from AOS to LOS
   - Ring buffer event flow to JetStream
   - Temporal workflow execution for pass scheduling

3. **Performance Tests:**
   - PropagationSystem: <100ns per satellite
   - VisibilitySystem: <500ns per satellite
   - LinkBudgetSystem: <200ns per link
   - Total orbital tick: <10µs for full constellation

---

## Summary

This addendum specifies:

- **5 Legion Components:** SatelliteIdentity, GroundStationIdentity, OrbitalState, LinkState, VisibilityCache
- **4 Legion Systems:** PropagationSystem, VisibilitySystem, LinkBudgetSystem, OrbitalQueryRouter
- **8 Event Types:** Position, link state, pass scheduling, health, weather
- **4 APECS Handlers:** OrbitalEventHandler, EphemerisPublisher, PassScheduler, WeatherIntegration
- **World Initialization:** 12 satellites, 10 ground stations, 120 link state entities

Implement in order: Components → Systems → Events → APECS handlers → Integration tests.

---

**END OF RFC-9502A**
