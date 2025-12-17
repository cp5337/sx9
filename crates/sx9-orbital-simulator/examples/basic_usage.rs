//! Basic usage example for CTAS-7 orbital mechanics crate

use chrono::{Duration, Utc};
use ctas7_orbital_mechanics::*;

fn main() -> Result<()> {
    println!("=== CTAS-7 Orbital Mechanics Example ===");

    // Example 1: Create LaserLight FSO constellation
    println!("\n1. Creating LaserLight FSO MEO constellation...");
    let engine = create_laserlight_constellation()?;
    println!(
        "✓ Created constellation with {} satellites",
        engine.constellation().satellite_count()
    );

    // Example 2: Create custom MEO constellation
    println!("\n2. Creating custom MEO constellation...");
    let custom_engine = create_custom_meo_constellation(8, 10000.0, 60.0, 2)?;
    println!(
        "✓ Created custom constellation with {} satellites",
        custom_engine.constellation().satellite_count()
    );

    // Example 3: Load configuration from file
    println!("\n3. Loading constellation from configuration file...");
    let config = load_constellation_config("examples/laserlight_constellation.json")?;
    let configured_engine = OrbitalMechanicsEngine::with_config(config)?;
    println!(
        "✓ Loaded constellation: {}",
        configured_engine
            .constellation()
            .satellites()
            .next()
            .unwrap()
            .name
    );

    // Example 4: Calculate satellite positions
    println!("\n4. Calculating satellite positions...");
    let now = Utc::now();
    for satellite in engine.constellation().satellites().take(3) {
        let position = engine.satellite_position(&satellite.satellite_id, now)?;
        println!(
            "   {}: Lat {:.3}°, Lon {:.3}°, Alt {:.1} km",
            satellite.satellite_id,
            position.geodetic.latitude_deg,
            position.geodetic.longitude_deg,
            position.geodetic.altitude_km
        );
    }

    // Example 5: Calculate visibility windows
    println!("\n5. Calculating visibility windows...");
    let mut engine_with_station = engine;

    // Add a ground station
    let station = GroundStation {
        station_id: "GS-DEMO".to_string(),
        name: "Demo Ground Station".to_string(),
        position: ground_station::StationPosition {
            latitude_deg: 40.0,
            longitude_deg: -105.0,
            elevation_m: 1600.0,
        },
    };
    engine_with_station.add_ground_station(station);

    let windows = engine_with_station.calculate_all_visibility_windows(now, 24.0)?;
    println!(
        "✓ Found {} visibility windows in next 24 hours",
        windows.len()
    );

    if let Some(window) = windows.first() {
        println!(
            "   Next pass: {} - {} ({:.1} min, max elev {:.1}°)",
            window.start_time.format("%H:%M:%S"),
            window.end_time.format("%H:%M:%S"),
            window.duration_seconds / 60.0,
            window.max_elevation_deg
        );
    }

    // Example 6: FSO link analysis
    println!("\n6. Analyzing FSO link quality...");
    if let Some(satellite) = engine_with_station.constellation().satellites().next() {
        if let Some(station) = engine_with_station.ground_stations().stations().next() {
            if let Ok(Some(link_quality)) = engine_with_station.analyze_fso_link(
                &satellite.satellite_id,
                &station.station_id,
                now,
            ) {
                println!(
                    "   FSO Link: {:.1} Gbps, {:.1} dB margin, {:.1}° elevation",
                    link_quality.estimated_throughput_gbps,
                    link_quality.link_margin_db,
                    link_quality.elevation_angle_deg
                );
            } else {
                println!("   FSO Link: Not visible at current time");
            }
        }
    }

    // Example 7: Generate constellation report
    println!("\n7. Generating constellation status report...");
    let report = engine_with_station.constellation_report(now)?;
    println!("{}", report);

    // Example 8: Orbital mechanics validation
    println!("\n8. Validating orbital mechanics...");
    let coverage = engine_with_station.constellation().coverage_statistics();
    println!("   Coverage Statistics:");
    println!("   - Satellites: {}", coverage.satellite_count);
    println!(
        "   - Average inclination: {:.1}°",
        coverage.average_inclination_deg
    );
    println!(
        "   - Altitude range: {:.1} - {:.1} km",
        coverage.altitude_range_km.0, coverage.altitude_range_km.1
    );
    println!(
        "   - Global coverage: {:.1}%",
        coverage.latitude_coverage.global_coverage_percent
    );

    println!("\n✅ All examples completed successfully!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_functionality() {
        let result = main();
        assert!(
            result.is_ok(),
            "Basic usage example should complete without errors"
        );
    }
}
