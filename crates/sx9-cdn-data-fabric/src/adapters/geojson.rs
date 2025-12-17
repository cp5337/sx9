//! GeoJSON Adapter
//!
//! Handles loading and querying GeoJSON files.

use crate::registry::DatabaseInfo;
use serde_json::{json, Value};
use std::path::Path;

/// Load a GeoJSON file
pub async fn load_file(path: &str) -> anyhow::Result<Value> {
    let content = tokio::fs::read_to_string(path).await?;
    let geojson: Value = serde_json::from_str(&content)?;
    Ok(geojson)
}

/// List available GeoJSON layers
pub async fn list_layers(base_path: &str) -> anyhow::Result<Vec<String>> {
    let mut layers = Vec::new();
    let mut entries = tokio::fs::read_dir(base_path).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().map(|e| e == "geojson").unwrap_or(false) {
            if let Some(stem) = path.file_stem() {
                layers.push(stem.to_string_lossy().to_string());
            }
        }
    }

    Ok(layers)
}

/// Query GeoJSON features by property
pub async fn query_features(path: &str, property: &str, value: &str) -> anyhow::Result<Vec<Value>> {
    let geojson = load_file(path).await?;

    let features = geojson
        .get("features")
        .and_then(|f| f.as_array())
        .map(|arr| {
            arr.iter()
                .filter(|feature| {
                    feature
                        .get("properties")
                        .and_then(|p| p.get(property))
                        .and_then(|v| v.as_str())
                        .map(|v| v.contains(value))
                        .unwrap_or(false)
                })
                .cloned()
                .collect()
        })
        .unwrap_or_default();

    Ok(features)
}

/// Get bounding box of GeoJSON
pub fn get_bbox(geojson: &Value) -> Option<[f64; 4]> {
    let features = geojson.get("features")?.as_array()?;

    let mut min_lon = f64::MAX;
    let mut min_lat = f64::MAX;
    let mut max_lon = f64::MIN;
    let mut max_lat = f64::MIN;

    for feature in features {
        if let Some(coords) = extract_coordinates(feature) {
            for coord in coords {
                if coord.len() >= 2 {
                    min_lon = min_lon.min(coord[0]);
                    max_lon = max_lon.max(coord[0]);
                    min_lat = min_lat.min(coord[1]);
                    max_lat = max_lat.max(coord[1]);
                }
            }
        }
    }

    if min_lon == f64::MAX {
        None
    } else {
        Some([min_lon, min_lat, max_lon, max_lat])
    }
}

/// Extract all coordinates from a feature
fn extract_coordinates(feature: &Value) -> Option<Vec<Vec<f64>>> {
    let geometry = feature.get("geometry")?;
    let geom_type = geometry.get("type")?.as_str()?;
    let coords = geometry.get("coordinates")?;

    match geom_type {
        "Point" => {
            let arr = coords.as_array()?;
            Some(vec![arr.iter().filter_map(|v| v.as_f64()).collect()])
        }
        "LineString" | "MultiPoint" => {
            let arr = coords.as_array()?;
            Some(
                arr.iter()
                    .filter_map(|c| {
                        c.as_array()
                            .map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
                    })
                    .collect(),
            )
        }
        "Polygon" | "MultiLineString" => {
            let arr = coords.as_array()?;
            Some(
                arr.iter()
                    .flat_map(|ring| {
                        ring.as_array()
                            .map(|r| {
                                r.iter()
                                    .filter_map(|c| {
                                        c.as_array()
                                            .map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default()
                    })
                    .collect(),
            )
        }
        _ => None,
    }
}

/// Convert graph data to GeoJSON (if nodes have lat/lon)
pub fn graph_to_geojson(nodes: &[Value], edges: &[Value]) -> Value {
    let mut features = Vec::new();

    // Add nodes as points
    for node in nodes {
        let props = node.get("properties").cloned().unwrap_or(json!({}));

        let lat = props
            .get("latitude")
            .or_else(|| props.get("lat"))
            .and_then(|v| v.as_f64());
        let lon = props
            .get("longitude")
            .or_else(|| props.get("lon"))
            .or_else(|| props.get("lng"))
            .and_then(|v| v.as_f64());

        if let (Some(lat), Some(lon)) = (lat, lon) {
            features.push(json!({
                "type": "Feature",
                "geometry": {
                    "type": "Point",
                    "coordinates": [lon, lat]
                },
                "properties": {
                    "id": node.get("id"),
                    "labels": node.get("labels"),
                    "data": props
                }
            }));
        }
    }

    // Add edges as lines (if both endpoints have coordinates)
    // This would require looking up node coordinates

    json!({
        "type": "FeatureCollection",
        "features": features
    })
}
