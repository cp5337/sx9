//! Data Format Transformation
//!
//! Transforms query results between formats:
//! - graph (nodes + edges for visualization)
//! - table (rows + columns for tabular view)
//! - json (raw JSON)
//! - geojson (geographic features)
//! - cypher (Neo4j query format)
//! - surql (SurrealDB query format)

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Transform result to specified format
pub fn to_format(result: &[Value], format: &str) -> Value {
    match format {
        "graph" => to_graph(result),
        "table" => to_table(result),
        "geojson" => to_geojson(result),
        _ => json!(result),
    }
}

/// Transform to graph format (nodes + edges)
pub fn to_graph(result: &[Value]) -> Value {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut node_ids = std::collections::HashSet::new();

    for (idx, record) in result.iter().enumerate() {
        // Extract ID
        let id = record
            .get("id")
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| format!("node_{}", idx));

        if node_ids.contains(&id) {
            continue;
        }
        node_ids.insert(id.clone());

        // Extract labels from table name in ID
        let labels = extract_labels(&id);

        // Build node
        let node = json!({
            "id": id,
            "labels": labels,
            "properties": record,
            "x": (idx % 10) as f64 * 100.0,
            "y": (idx / 10) as f64 * 100.0,
        });
        nodes.push(node);

        // Extract relationships from graph traversals
        if let Some(relations) = record.get("->") {
            if let Some(rel_map) = relations.as_object() {
                for (rel_type, targets) in rel_map {
                    if let Some(target_array) = targets.as_array() {
                        for target in target_array {
                            if let Some(target_id) = target.get("id").and_then(|v| v.as_str()) {
                                edges.push(json!({
                                    "id": format!("{}->{}:{}", id, rel_type, target_id),
                                    "source": id,
                                    "target": target_id,
                                    "type": rel_type,
                                    "properties": {}
                                }));
                            }
                        }
                    }
                }
            }
        }
    }

    json!({
        "nodes": nodes,
        "edges": edges,
        "stats": {
            "nodeCount": nodes.len(),
            "edgeCount": edges.len()
        }
    })
}

/// Transform to table format (rows + columns)
pub fn to_table(result: &[Value]) -> Value {
    if result.is_empty() {
        return json!({
            "columns": [],
            "rows": [],
            "stats": { "rowCount": 0 }
        });
    }

    // Extract columns from first record
    let columns: Vec<String> = result[0]
        .as_object()
        .map(|obj| obj.keys().cloned().collect())
        .unwrap_or_default();

    // Build rows
    let rows: Vec<Vec<Value>> = result
        .iter()
        .map(|record| {
            columns
                .iter()
                .map(|col| record.get(col).cloned().unwrap_or(Value::Null))
                .collect()
        })
        .collect();

    json!({
        "columns": columns,
        "rows": rows,
        "stats": {
            "rowCount": rows.len(),
            "columnCount": columns.len()
        }
    })
}

/// Transform to GeoJSON format
pub fn to_geojson(result: &[Value]) -> Value {
    let features: Vec<Value> = result
        .iter()
        .filter_map(|record| {
            // Try to extract coordinates
            let lat = record
                .get("latitude")
                .or_else(|| record.get("lat"))
                .and_then(|v| v.as_f64());
            let lon = record
                .get("longitude")
                .or_else(|| record.get("lon"))
                .or_else(|| record.get("lng"))
                .and_then(|v| v.as_f64());

            match (lat, lon) {
                (Some(lat), Some(lon)) => Some(json!({
                    "type": "Feature",
                    "geometry": {
                        "type": "Point",
                        "coordinates": [lon, lat]
                    },
                    "properties": record
                })),
                _ => {
                    // Check for existing geometry
                    if let Some(geom) = record.get("geometry") {
                        Some(json!({
                            "type": "Feature",
                            "geometry": geom,
                            "properties": record
                        }))
                    } else {
                        None
                    }
                }
            }
        })
        .collect();

    json!({
        "type": "FeatureCollection",
        "features": features
    })
}

/// Extract labels from SurrealDB ID
fn extract_labels(id: &str) -> Vec<String> {
    if let Some(table) = id.split(':').next() {
        vec![to_pascal_case(table)]
    } else {
        vec!["Node".to_string()]
    }
}

/// Convert snake_case to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

/// Convert Cypher query to SurrealQL
pub fn cypher_to_surql(cypher: &str) -> String {
    // Basic translation - this would need expansion for full support
    let surql = cypher
        .replace("MATCH", "SELECT * FROM")
        .replace("RETURN", "-- RETURN")
        .replace("WHERE", "WHERE")
        .replace("-[:USES]->", "->uses->")
        .replace("-[:CORRELATES]->", "->correlates->")
        .replace("(n:", "")
        .replace(")", "");

    surql
}

/// Convert SurrealQL query to Cypher
pub fn surql_to_cypher(surql: &str) -> String {
    // Basic translation
    let cypher = surql
        .replace("SELECT * FROM", "MATCH (n:")
        .replace("->uses->", "-[:USES]->")
        .replace("->correlates->", "-[:CORRELATES]->");

    format!("{}) RETURN n", cypher)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_graph() {
        let result = vec![
            json!({"id": "threat_actor:apt29", "name": "APT29"}),
            json!({"id": "technique:t1059", "name": "Command Line Interface"}),
        ];

        let graph = to_graph(&result);
        assert!(graph.get("nodes").unwrap().as_array().unwrap().len() == 2);
    }

    #[test]
    fn test_to_table() {
        let result = vec![
            json!({"id": "1", "name": "Alice"}),
            json!({"id": "2", "name": "Bob"}),
        ];

        let table = to_table(&result);
        assert!(table.get("rows").unwrap().as_array().unwrap().len() == 2);
    }

    #[test]
    fn test_to_geojson() {
        let result = vec![
            json!({"id": "1", "name": "Station A", "latitude": 40.7128, "longitude": -74.0060}),
        ];

        let geojson = to_geojson(&result);
        let features = geojson.get("features").unwrap().as_array().unwrap();
        assert_eq!(features.len(), 1);
    }
}
