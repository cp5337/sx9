//! PostgreSQL Adapter
//!
//! Handles communication with PostgreSQL instances.

use crate::registry::DatabaseInfo;
use serde_json::{json, Value};
use tokio_postgres::{NoTls, Row};

/// Execute a SQL query
pub async fn execute(info: &DatabaseInfo, query: &str) -> anyhow::Result<Vec<Value>> {
    let conn_string = format!(
        "host={} port={} user=postgres password=postgres dbname={}",
        info.host,
        info.port,
        info.database.as_deref().unwrap_or("postgres")
    );

    let (client, connection) = tokio_postgres::connect(&conn_string, NoTls).await?;

    // Spawn connection handler
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("PostgreSQL connection error: {}", e);
        }
    });

    // Execute query
    let rows = client.query(query, &[]).await?;

    // Convert rows to JSON
    let results: Vec<Value> = rows.iter().map(|row| row_to_json(row)).collect();

    Ok(results)
}

/// Convert PostgreSQL row to JSON
fn row_to_json(row: &Row) -> Value {
    let mut obj = serde_json::Map::new();

    for (idx, column) in row.columns().iter().enumerate() {
        let name = column.name();
        let value = match column.type_().name() {
            "int4" | "int8" => {
                if let Ok(v) = row.try_get::<_, i64>(idx) {
                    json!(v)
                } else {
                    Value::Null
                }
            }
            "float4" | "float8" | "numeric" => {
                if let Ok(v) = row.try_get::<_, f64>(idx) {
                    json!(v)
                } else {
                    Value::Null
                }
            }
            "bool" => {
                if let Ok(v) = row.try_get::<_, bool>(idx) {
                    json!(v)
                } else {
                    Value::Null
                }
            }
            "text" | "varchar" | "char" | "name" => {
                if let Ok(v) = row.try_get::<_, String>(idx) {
                    json!(v)
                } else {
                    Value::Null
                }
            }
            "json" | "jsonb" => {
                if let Ok(v) = row.try_get::<_, Value>(idx) {
                    v
                } else {
                    Value::Null
                }
            }
            _ => {
                // Try as string fallback
                if let Ok(v) = row.try_get::<_, String>(idx) {
                    json!(v)
                } else {
                    Value::Null
                }
            }
        };
        obj.insert(name.to_string(), value);
    }

    Value::Object(obj)
}

/// Get schema from PostgreSQL
pub async fn get_schema(info: &DatabaseInfo) -> anyhow::Result<Value> {
    let query = r#"
        SELECT 
            table_name,
            column_name,
            data_type,
            is_nullable
        FROM information_schema.columns
        WHERE table_schema = 'public'
        ORDER BY table_name, ordinal_position
    "#;

    let results = execute(info, query).await?;

    Ok(json!({
        "db_type": "postgres",
        "columns": results
    }))
}

/// Check health of PostgreSQL instance
pub async fn health_check(info: &DatabaseInfo) -> bool {
    let conn_string = format!(
        "host={} port={} user=postgres password=postgres dbname={}",
        info.host,
        info.port,
        info.database.as_deref().unwrap_or("postgres")
    );

    match tokio_postgres::connect(&conn_string, NoTls).await {
        Ok((client, connection)) => {
            tokio::spawn(async move {
                let _ = connection.await;
            });

            match client.simple_query("SELECT 1").await {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}
