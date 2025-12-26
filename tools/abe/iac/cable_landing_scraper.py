#!/usr/bin/env python3
"""
Cable Landing Scraper for CTAS-7 Ground Station Network
========================================================

Scrapes submarine cable landing points from TeleGeography's submarinecablemap.com
to seed the 257 LaserLight FSO ground station network.

Data Flow:
    submarinecablemap.com API → cable_landings.json → Supabase ground_stations table

Output Format (matches ground_stations.rs schema):
    {
        "station_id": "GS-001-NYC",
        "name": "New York Landing",
        "latitude": 40.7128,
        "longitude": -74.0060,
        "country": "United States",
        "cable_names": ["TAT-14", "AC-1"],
        "fso_suitability_score": 0.0  # Populated by GEE weather analysis later
    }

Usage:
    python cable_landing_scraper.py --fetch          # Fetch from API
    python cable_landing_scraper.py --parse-local    # Parse local cache
    python cable_landing_scraper.py --export-json    # Export to JSON
    python cable_landing_scraper.py --load-supabase  # Load to Supabase
    python cable_landing_scraper.py --fix-inversion  # Fix lat/lon swap

Note: Original data had lat/lon inversion - use --fix-inversion to correct.
"""

import json
import argparse
import logging
from pathlib import Path
from datetime import datetime
from typing import List, Dict, Optional, Any
import os

# Optional imports
try:
    import requests
    HAS_REQUESTS = True
except ImportError:
    HAS_REQUESTS = False

try:
    from supabase import create_client, Client
    HAS_SUPABASE = True
except ImportError:
    HAS_SUPABASE = False

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

# ============================================================================
# Configuration
# ============================================================================

OUTPUT_DIR = Path(__file__).parent / "output" / "ground_stations"
CACHE_FILE = OUTPUT_DIR / "cable_landings_raw.json"
STATIONS_FILE = OUTPUT_DIR / "ground_stations.json"
GEOJSON_FILE = OUTPUT_DIR / "ground_stations.geojson"

# TeleGeography API (public endpoints)
TELEGEOGRAPHY_API = "https://www.submarinecablemap.com/api/v3"
LANDING_POINTS_ENDPOINT = f"{TELEGEOGRAPHY_API}/landing-point/all.json"
CABLES_ENDPOINT = f"{TELEGEOGRAPHY_API}/cable/all.json"

# Supabase config (from environment)
SUPABASE_URL = os.environ.get("SUPABASE_URL", "")
SUPABASE_KEY = os.environ.get("SUPABASE_ANON_KEY", "")

# ============================================================================
# Data Models
# ============================================================================

def create_ground_station(
    station_id: str,
    name: str,
    latitude: float,
    longitude: float,
    country: str,
    cable_names: List[str],
    region: str = "",
    city: str = ""
) -> Dict[str, Any]:
    """Create a ground station record matching the Rust schema."""
    return {
        "station_id": station_id,
        "name": name,
        "latitude": latitude,
        "longitude": longitude,
        "elevation_m": 0.0,
        "country": country,
        "region": region,
        "city": city or name,
        "cable_landing_id": station_id,
        "cable_names": cable_names,
        "fso_suitability_score": 0.0,
        "clear_sky_days_per_year": 0,
        "avg_cloud_cover_pct": 0.0,
        "avg_water_vapor_mm": 0.0,
        "cluster_id": assign_cluster(longitude),
        "trivariate_hash": None,
        "operational": False,
        "wasm_sensor_deployed": False,
        "last_ping_ms": None,
        "last_updated": datetime.utcnow().isoformat() + "Z"
    }


def assign_cluster(longitude: float) -> int:
    """Assign to one of 10 clusters based on longitude bands."""
    # -180 to 180 divided into 10 bands of 36 degrees each
    cluster = int((longitude + 180.0) / 36.0)
    return min(max(cluster, 0), 9)


def fix_coordinate_inversion(station: Dict[str, Any]) -> Dict[str, Any]:
    """Fix lat/lon inversion if detected."""
    lat = station["latitude"]
    lon = station["longitude"]

    # Detect inversion: latitude should be -90 to 90
    if abs(lat) > 90 and abs(lon) <= 90:
        logger.warning(f"Fixing inverted coords for {station['station_id']}: {lat}, {lon} -> {lon}, {lat}")
        station["latitude"] = lon
        station["longitude"] = lat
        station["cluster_id"] = assign_cluster(station["longitude"])

    return station


def validate_coordinates(station: Dict[str, Any]) -> bool:
    """Validate coordinates are in valid ranges."""
    lat = station["latitude"]
    lon = station["longitude"]

    if lat < -90 or lat > 90:
        logger.error(f"Invalid latitude {lat} for {station['station_id']}")
        return False
    if lon < -180 or lon > 180:
        logger.error(f"Invalid longitude {lon} for {station['station_id']}")
        return False
    return True


# ============================================================================
# Scraping Functions
# ============================================================================

def fetch_landing_points() -> List[Dict[str, Any]]:
    """Fetch landing points from TeleGeography API."""
    if not HAS_REQUESTS:
        logger.error("requests library not installed. Run: pip install requests")
        return []

    logger.info(f"Fetching landing points from {LANDING_POINTS_ENDPOINT}")

    try:
        response = requests.get(LANDING_POINTS_ENDPOINT, timeout=30)
        response.raise_for_status()
        data = response.json()
        logger.info(f"Fetched {len(data)} landing points")
        return data
    except requests.RequestException as e:
        logger.error(f"Failed to fetch landing points: {e}")
        return []


def fetch_cables() -> List[Dict[str, Any]]:
    """Fetch cable data from TeleGeography API."""
    if not HAS_REQUESTS:
        logger.error("requests library not installed. Run: pip install requests")
        return []

    logger.info(f"Fetching cables from {CABLES_ENDPOINT}")

    try:
        response = requests.get(CABLES_ENDPOINT, timeout=30)
        response.raise_for_status()
        data = response.json()
        logger.info(f"Fetched {len(data)} cables")
        return data
    except requests.RequestException as e:
        logger.error(f"Failed to fetch cables: {e}")
        return []


def parse_landing_points(raw_data: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    """Parse raw landing point data into ground station format."""
    stations = []

    for i, point in enumerate(raw_data):
        try:
            # Extract coordinates
            lat = float(point.get("latitude", 0))
            lon = float(point.get("longitude", 0))

            # Skip invalid coordinates
            if lat == 0 and lon == 0:
                continue

            # Extract metadata
            name = point.get("name", f"Landing-{i}")
            country = point.get("country", "Unknown")

            # Get cable names if available
            cable_names = []
            if "cables" in point:
                cable_names = [c.get("name", "") for c in point["cables"] if c.get("name")]

            # Create station ID
            station_id = f"GS-{i:03d}-{name[:3].upper()}"

            station = create_ground_station(
                station_id=station_id,
                name=name,
                latitude=lat,
                longitude=lon,
                country=country,
                cable_names=cable_names,
                city=name
            )

            stations.append(station)

        except (ValueError, KeyError) as e:
            logger.warning(f"Failed to parse landing point {i}: {e}")
            continue

    logger.info(f"Parsed {len(stations)} ground stations from {len(raw_data)} landing points")
    return stations


# ============================================================================
# Export Functions
# ============================================================================

def export_to_json(stations: List[Dict[str, Any]], filepath: Path) -> None:
    """Export stations to JSON file."""
    filepath.parent.mkdir(parents=True, exist_ok=True)
    with open(filepath, 'w') as f:
        json.dump(stations, f, indent=2)
    logger.info(f"Exported {len(stations)} stations to {filepath}")


def export_to_geojson(stations: List[Dict[str, Any]], filepath: Path) -> None:
    """Export stations to GeoJSON for visualization."""
    features = []
    for s in stations:
        feature = {
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [s["longitude"], s["latitude"]]
            },
            "properties": {
                "station_id": s["station_id"],
                "name": s["name"],
                "country": s["country"],
                "cluster_id": s["cluster_id"],
                "cables": s["cable_names"]
            }
        }
        features.append(feature)

    geojson = {
        "type": "FeatureCollection",
        "features": features
    }

    filepath.parent.mkdir(parents=True, exist_ok=True)
    with open(filepath, 'w') as f:
        json.dump(geojson, f, indent=2)
    logger.info(f"Exported GeoJSON to {filepath}")


def load_to_supabase(stations: List[Dict[str, Any]]) -> int:
    """Load stations to Supabase ground_stations table."""
    if not HAS_SUPABASE:
        logger.error("supabase library not installed. Run: pip install supabase")
        return 0

    if not SUPABASE_URL or not SUPABASE_KEY:
        logger.error("SUPABASE_URL and SUPABASE_ANON_KEY environment variables required")
        return 0

    try:
        client: Client = create_client(SUPABASE_URL, SUPABASE_KEY)

        # Upsert in batches
        batch_size = 50
        loaded = 0

        for i in range(0, len(stations), batch_size):
            batch = stations[i:i + batch_size]
            result = client.table("ground_stations").upsert(batch).execute()
            loaded += len(batch)
            logger.info(f"Loaded batch {i // batch_size + 1}: {len(batch)} stations")

        logger.info(f"Loaded {loaded} stations to Supabase")
        return loaded

    except Exception as e:
        logger.error(f"Failed to load to Supabase: {e}")
        return 0


# ============================================================================
# Main CLI
# ============================================================================

def main():
    parser = argparse.ArgumentParser(description="Cable Landing Scraper for CTAS-7")
    parser.add_argument("--fetch", action="store_true", help="Fetch from TeleGeography API")
    parser.add_argument("--parse-local", action="store_true", help="Parse local cache file")
    parser.add_argument("--export-json", action="store_true", help="Export to JSON")
    parser.add_argument("--export-geojson", action="store_true", help="Export to GeoJSON")
    parser.add_argument("--load-supabase", action="store_true", help="Load to Supabase")
    parser.add_argument("--fix-inversion", action="store_true", help="Fix lat/lon inversion")
    parser.add_argument("--all", action="store_true", help="Run full pipeline")
    args = parser.parse_args()

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    stations = []

    # Fetch from API
    if args.fetch or args.all:
        raw_data = fetch_landing_points()
        if raw_data:
            # Cache raw data
            with open(CACHE_FILE, 'w') as f:
                json.dump(raw_data, f, indent=2)
            logger.info(f"Cached raw data to {CACHE_FILE}")
            stations = parse_landing_points(raw_data)

    # Parse local cache
    if args.parse_local:
        if CACHE_FILE.exists():
            with open(CACHE_FILE, 'r') as f:
                raw_data = json.load(f)
            stations = parse_landing_points(raw_data)
        else:
            logger.error(f"Cache file not found: {CACHE_FILE}")
            return

    # Load existing stations file
    if not stations and STATIONS_FILE.exists():
        with open(STATIONS_FILE, 'r') as f:
            stations = json.load(f)
        logger.info(f"Loaded {len(stations)} stations from {STATIONS_FILE}")

    # Fix coordinate inversion
    if args.fix_inversion and stations:
        stations = [fix_coordinate_inversion(s) for s in stations]
        # Validate
        valid_stations = [s for s in stations if validate_coordinates(s)]
        logger.info(f"Valid stations after fix: {len(valid_stations)}/{len(stations)}")
        stations = valid_stations

    # Export to JSON
    if (args.export_json or args.all) and stations:
        export_to_json(stations, STATIONS_FILE)

    # Export to GeoJSON
    if (args.export_geojson or args.all) and stations:
        export_to_geojson(stations, GEOJSON_FILE)

    # Load to Supabase
    if args.load_supabase and stations:
        load_to_supabase(stations)

    # Summary
    if stations:
        print(f"\n=== Ground Station Summary ===")
        print(f"Total stations: {len(stations)}")
        print(f"Countries: {len(set(s['country'] for s in stations))}")
        print(f"Clusters: {sorted(set(s['cluster_id'] for s in stations))}")

        # Show sample
        print(f"\nSample station:")
        print(json.dumps(stations[0], indent=2))


if __name__ == "__main__":
    main()
