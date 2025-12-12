# Google Earth Engine & Ground Stations - CTAS v7.3.1

**Purpose:** Geospatial intelligence extraction and optimal ground station placement

**Generated:** $(date)
**Status:** GIS Integration Architecture

---

## Overview

CTAS v7.3.1 uses **Google Earth Engine (GEE)** for large-scale geospatial analysis to:
1. Select optimal locations for **257 LaserLight FSO ground stations**
2. Extract critical infrastructure layers (power grid, submarine cables, internet backbone)
3. Perform atmospheric analysis for Free Space Optical (FSO) links
4. Generate KMZ layers for Mapbox/Cesium/GEE universal viewing

---

## Google Earth Engine Integration

### Purpose

GEE provides **planetary-scale geospatial analysis** for:
- **Atmospheric conditions** - Days of sunlight, cloud cover, humidity for FSO links
- **Critical infrastructure** - Power grids, telecom hubs, data centers
- **Submarine cable landings** - Global internet backbone entry points
- **Population density** - Urban vs rural deployment strategies
- **Terrain analysis** - Line-of-sight for FSO links

### Cost Analysis

**GEE Pricing:**
- **Free Tier:** 10,000 requests/day, 250GB storage
- **Commercial:** $0.10 per 1000 requests, $0.02/GB storage
- **6 Hours Heavy Lifting:** ~$50-100 (estimated for 257 station analysis)

**Use Case:** One-time analysis for ground station placement, not continuous monitoring

### API Access

```python
import ee

# Authenticate
ee.Authenticate()
ee.Initialize(project='CTAS7-AI-Studio-Project-ONE')

# Example: Get atmospheric data for FSO analysis
def analyze_fso_conditions(lat, lon, start_date, end_date):
    point = ee.Geometry.Point([lon, lat])
    
    # Cloud cover from MODIS
    modis = ee.ImageCollection('MODIS/006/MOD09GA') \
        .filterDate(start_date, end_date) \
        .filterBounds(point)
    
    # Calculate clear sky days
    clear_days = modis.map(lambda img: 
        img.select('state_1km').bitwiseAnd(1).eq(0)
    ).mean()
    
    # Atmospheric water vapor
    water_vapor = ee.ImageCollection('MODIS/006/MOD05_L2') \
        .filterDate(start_date, end_date) \
        .filterBounds(point) \
        .select('Water_Vapor_Near_Infrared') \
        .mean()
    
    return {
        'clear_sky_percentage': clear_days.reduceRegion(
            reducer=ee.Reducer.mean(),
            geometry=point,
            scale=1000
        ).getInfo(),
        'avg_water_vapor': water_vapor.reduceRegion(
            reducer=ee.Reducer.mean(),
            geometry=point,
            scale=1000
        ).getInfo()
    }
```

---

## LaserLight FSO Ground Stations

### Overview

**Total Stations:** 257 (updated from original 247)
**Constellation:** 12 LaserLight MEO satellites in Walker Delta configuration
**Technology:** Free Space Optical (FSO) communication
**Purpose:** High-bandwidth, low-latency global connectivity

### Selection Criteria

Ground stations selected based on:

1. **Atmospheric Conditions** (GEE Analysis)
   - Days of sunlight per year (>250 preferred)
   - Low cloud cover (<30% annual average)
   - Low humidity (<60% annual average)
   - Minimal precipitation

2. **Proximity to Critical Infrastructure**
   - Submarine cable landing stations
   - Major internet exchange points (IXPs)
   - Data centers and cloud on-ramps
   - Power grid interconnections

3. **Strategic Value**
   - Geographic diversity (global coverage)
   - Redundancy (multiple stations per region)
   - Access to multiple submarine cables
   - Proximity to major cities/population centers

4. **Technical Feasibility**
   - Line-of-sight to MEO satellites
   - Minimal atmospheric interference
   - Available land/rooftop space
   - Power and fiber connectivity

### Ground Station Roles

Each ground station serves multiple functions:

1. **OSINT Collection Node** (247 original stations)
   - WASM microkernel deployment
   - Distributed intelligence gathering
   - Feed 165-node graph detector
   - Node interview EEI collection

2. **FSO Communication Hub** (257 total)
   - Satellite uplink/downlink
   - Mesh network connectivity
   - Redundant routing
   - High-bandwidth data transfer

3. **Threat Intelligence Sensor**
   - Network traffic analysis
   - GeoIP threat mapping
   - Malicious IP tracking
   - Anomaly detection

4. **Legion ECS Entity** (Graph Node)
   - Part of geospatial world
   - Relationships to cables, IXPs, infrastructure
   - SlotGraph coordination
   - Real-time status tracking

---

## GEE Data Extraction

### Layers Extracted

#### 1. Submarine Cable Landings

**Source:** TeleGeography Submarine Cable Map 2025
**Format:** KMZ (Keyhole Markup Language Zipped)
**Data:**
- Cable landing points (lat/lon)
- Cable names and operators
- Capacity (Tbps)
- Operational status
- Year deployed

**GEE Processing:**
```python
# Extract cable landing points
cable_landings = ee.FeatureCollection('users/ctas7/submarine_cables')
cable_points = cable_landings.filterBounds(region)

# Export as KMZ
task = ee.batch.Export.table.toDrive(
    collection=cable_points,
    description='submarine_cable_landings',
    fileFormat='KMZ'
)
task.start()
```

#### 2. Critical Infrastructure

**Power Grid:**
- Transmission lines (>230kV)
- Substations and transformers
- Generation facilities
- Grid interconnections

**Internet Backbone:**
- Internet Exchange Points (IXPs)
- Major fiber routes
- Data centers
- Cloud on-ramps (AWS, Azure, GCP)

**Telecommunications:**
- Cell towers and base stations
- Microwave links
- Satellite ground stations
- Network operations centers (NOCs)

#### 3. Atmospheric Analysis for FSO

**Datasets Used:**
- **MODIS** - Cloud cover, water vapor
- **ERA5** - Temperature, humidity, wind
- **GOES** - Real-time weather
- **Landsat** - Land use, terrain

**Analysis:**
```python
def fso_suitability_score(lat, lon):
    """
    Calculate FSO link suitability (0-100)
    Higher score = better conditions
    """
    point = ee.Geometry.Point([lon, lat])
    
    # Cloud cover (lower is better)
    cloud_score = 100 - get_cloud_cover_percentage(point)
    
    # Clear sky days (higher is better)
    clear_days_score = get_clear_sky_days(point) / 365 * 100
    
    # Water vapor (lower is better)
    wv_score = 100 - (get_water_vapor(point) / 50 * 100)
    
    # Weighted average
    return (cloud_score * 0.4 + clear_days_score * 0.4 + wv_score * 0.2)
```

#### 4. Geospatial Intelligence Layers

**WMD Proliferation:**
- Nuclear facilities
- Chemical plants
- Biological research labs
- Dual-use facilities

**Conflict Zones:**
- Military installations
- Conflict areas
- Refugee camps
- Humanitarian corridors

**Environmental:**
- Natural disasters
- Climate patterns
- Resource scarcity
- Migration routes

---

## Ground Station Database

### Schema

```sql
-- SurrealDB schema
DEFINE TABLE ground_stations SCHEMAFULL;

DEFINE FIELD id ON ground_stations TYPE string;
DEFINE FIELD name ON ground_stations TYPE string;
DEFINE FIELD location ON ground_stations TYPE geometry<point>;
DEFINE FIELD latitude ON ground_stations TYPE float;
DEFINE FIELD longitude ON ground_stations TYPE float;
DEFINE FIELD elevation ON ground_stations TYPE float;

-- FSO characteristics
DEFINE FIELD fso_suitability_score ON ground_stations TYPE float;
DEFINE FIELD clear_sky_days ON ground_stations TYPE int;
DEFINE FIELD avg_cloud_cover ON ground_stations TYPE float;
DEFINE FIELD avg_water_vapor ON ground_stations TYPE float;

-- Infrastructure proximity
DEFINE FIELD nearest_cable_landing ON ground_stations TYPE record<cable_landings>;
DEFINE FIELD distance_to_cable ON ground_stations TYPE float;
DEFINE FIELD nearest_ixp ON ground_stations TYPE record<ixps>;
DEFINE FIELD distance_to_ixp ON ground_stations TYPE float;

-- OSINT capabilities
DEFINE FIELD wasm_microkernel_deployed ON ground_stations TYPE bool;
DEFINE FIELD collection_capabilities ON ground_stations TYPE array<string>;
DEFINE FIELD node_interviews_supported ON ground_stations TYPE array<string>;

-- Status
DEFINE FIELD operational_status ON ground_stations TYPE string;
DEFINE FIELD last_heartbeat ON ground_stations TYPE datetime;
DEFINE FIELD uptime_percentage ON ground_stations TYPE float;

-- Relationships (SlotGraph)
DEFINE FIELD connects_to ON ground_stations TYPE array<record<ground_stations>>;
DEFINE FIELD serves_region ON ground_stations TYPE string;
```

### Current Status

**Original Ground Stations CSV:** `ground_stations_*.csv`
**Issue:** Some locations in ocean (fucked up coordinates)
**Status:** Needs regeneration with GEE analysis

**Action Items:**
1. Run GEE analysis for 257 optimal locations
2. Validate coordinates (land-based, accessible)
3. Calculate FSO suitability scores
4. Map to submarine cable landings
5. Assign to regions for coverage
6. Generate new CSV with validated data

---

## Universal GIS Layering System

### Purpose

Single source of truth for GIS data, viewable across:
- **Mapbox GL** (CTAS Main Ops) - Street/building level detail
- **Cesium** (ctas7-command-center) - Orbital/3D visualization
- **Google Earth Engine** - Planetary-scale analysis

### Format: KMZ (Keyhole Markup Language)

**Why KMZ:**
- Universal format (Google Earth, Mapbox, Cesium)
- Compressed (smaller file sizes)
- Supports styles, icons, descriptions
- Hierarchical organization (folders)
- Hash-based versioning (USIM integration)

### Layer Structure

```
ctas-gis-layers/
├── infrastructure/
│   ├── submarine-cables.kmz
│   ├── power-grid.kmz
│   ├── internet-backbone.kmz
│   └── telecom-towers.kmz
├── ground-stations/
│   ├── laserlight-fso-257.kmz
│   ├── osint-nodes-247.kmz
│   └── coverage-zones.kmz
├── threats/
│   ├── geoip-malicious.kmz
│   ├── conflict-zones.kmz
│   └── wmd-facilities.kmz
├── scenarios/
│   ├── beslan-2004.kmz
│   ├── mumbai-2008.kmz
│   └── blue-dusk-black-sky.kmz
└── metadata/
    ├── layer-index.json
    └── usim-headers.json
```

### Hashing & Versioning

Each KMZ layer is hashed using **trivariate hashing**:

```
SCH (Semantic Content Hash) - Content of KMZ
CUID (Collision-resistant UID) - Generation timestamp + GEE job ID
UUID - Global uniqueness

Combined: 48-character Base96 hash
```

**USIM Header:**
```json
{
  "layer": "submarine-cables.kmz",
  "version": "7.3.1",
  "hash": "SCH-abc123...CUID-xyz789...UUID-def456",
  "generated": "2024-11-10T03:00:00Z",
  "source": "TeleGeography + GEE",
  "unicode": "🌐",
  "classification": "UNCLASSIFIED"
}
```

---

## Integration with CTAS Systems

### 1. Mapbox GL (CTAS Main Ops)

**Use Case:** Tactical operations, street-level detail

```javascript
// Load KMZ layer in Mapbox
import { KMZLoader } from '@loaders.gl/kml';

const kmzData = await load('submarine-cables.kmz', KMZLoader);

map.addSource('cables', {
  type: 'geojson',
  data: kmzData
});

map.addLayer({
  id: 'cable-lines',
  type: 'line',
  source: 'cables',
  paint: {
    'line-color': '#4ade80',
    'line-width': 2
  }
});
```

### 2. Cesium (Orbital View)

**Use Case:** Satellite tracking, global visualization

```javascript
// Load KMZ in Cesium
const viewer = new Cesium.Viewer('cesiumContainer');

const dataSource = await Cesium.KmlDataSource.load(
  'ground-stations/laserlight-fso-257.kmz'
);

viewer.dataSources.add(dataSource);
viewer.zoomTo(dataSource);
```

### 3. Legion ECS (Entity Management)

Ground stations as **Legion entities**:

```rust
// Legion ECS component
#[derive(Component)]
struct GroundStation {
    id: String,
    location: (f64, f64, f64), // lat, lon, elevation
    fso_score: f32,
    operational: bool,
    wasm_deployed: bool,
}

// Query ground stations in region
let stations_in_region: Vec<&GroundStation> = world
    .query::<&GroundStation>()
    .iter()
    .filter(|s| s.location.0 > min_lat && s.location.0 < max_lat)
    .collect();
```

### 4. SlotGraph (Coordination)

Ground stations coordinate via **SlotGraph**:

- **Slots:** Time windows for satellite passes
- **Graph:** Relationships between stations, cables, satellites
- **Coordination:** Optimal routing, redundancy, failover

---

## GEE Extraction Workflow

### Step 1: Define Analysis Region

```python
# Global analysis for 257 stations
region = ee.Geometry.Rectangle([-180, -90, 180, 90])

# Or specific region (e.g., North America)
region = ee.Geometry.Rectangle([-170, 15, -50, 75])
```

### Step 2: Extract Infrastructure

```python
# Submarine cables
cables = extract_submarine_cables(region)
export_kmz(cables, 'submarine-cables.kmz')

# Power grid
power_grid = extract_power_infrastructure(region)
export_kmz(power_grid, 'power-grid.kmz')

# Internet backbone
internet = extract_internet_infrastructure(region)
export_kmz(internet, 'internet-backbone.kmz')
```

### Step 3: Atmospheric Analysis

```python
# Analyze FSO conditions for candidate locations
candidates = generate_candidate_locations(region, density=1000)

fso_scores = []
for candidate in candidates:
    score = fso_suitability_score(candidate.lat, candidate.lon)
    fso_scores.append((candidate, score))

# Select top 257 locations
top_257 = sorted(fso_scores, key=lambda x: x[1], reverse=True)[:257]
```

### Step 4: Optimize Placement

```python
# Optimize for coverage + infrastructure proximity
optimized_stations = optimize_station_placement(
    candidates=top_257,
    cable_landings=cables,
    ixps=internet_exchanges,
    coverage_target=0.95,  # 95% global coverage
    redundancy_factor=1.5   # 1.5x redundancy
)
```

### Step 5: Generate KMZ

```python
# Create KMZ with all station data
stations_kmz = create_kmz(
    stations=optimized_stations,
    style={
        'icon': 'ground-station-icon.png',
        'color': '#4ade80',
        'scale': 1.5
    },
    metadata={
        'version': '7.3.1',
        'generated': datetime.now().isoformat(),
        'source': 'GEE + TeleGeography'
    }
)

export_kmz(stations_kmz, 'laserlight-fso-257.kmz')
```

### Step 6: Hash & Store

```bash
# Generate trivariate hash
HASH=$(./ctas7-hashing-engine hash-file laserlight-fso-257.kmz)

# Store in SurrealDB
curl -X POST http://localhost:8000/sql \
  -d "INSERT INTO gis_layers SET
      name = 'laserlight-fso-257',
      hash = '$HASH',
      format = 'kmz',
      version = '7.3.1',
      generated = time::now()"

# Store in Supabase (permanent record)
# Blockchain anchor for immutability
```

---

## Files & Locations

```
ctas7-command-center/
├── gis-layers/
│   ├── infrastructure/
│   ├── ground-stations/
│   └── threats/
└── scripts/
    ├── gee-extract-infrastructure.py
    ├── gee-analyze-fso-conditions.py
    └── gee-optimize-stations.py

ctas6-reference/
├── docs/architecture/
│   └── GEE-GROUND-STATIONS.md        # This document
└── public/
    └── gis/
        ├── submarine-cables.kmz
        ├── laserlight-fso-257.kmz
        └── layer-index.json
```

---

## Next Steps

1. **Run GEE Analysis**
   - Extract all infrastructure layers
   - Analyze FSO conditions globally
   - Optimize 257 station placement

2. **Generate KMZ Layers**
   - Submarine cables
   - Power grid
   - Internet backbone
   - Ground stations (validated coordinates)

3. **Hash & Store**
   - Trivariate hash all layers
   - Store in SurrealDB + Supabase
   - Blockchain anchor

4. **Integrate with CTAS**
   - Load layers in Mapbox (Hunt page)
   - Load layers in Cesium (Orbital view)
   - Deploy WASM microkernels to stations

5. **Deploy Ground Stations**
   - Physical deployment planning
   - FSO equipment installation
   - Network connectivity
   - WASM microkernel deployment

---

**Status:** Architecture defined, GEE extraction in progress
**Owner:** Marcus (Gemini 2M) - GIS & Infrastructure Architecture
**Cost:** ~$50-100 for 6-hour GEE analysis
**Related:** LaserLight FSO, TeleGeography, OSINT Nodes, Legion ECS

