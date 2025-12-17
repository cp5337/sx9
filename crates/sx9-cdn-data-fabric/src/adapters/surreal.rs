//! SurrealDB Adapter
//!
//! Handles communication with SurrealDB instances.

use crate::registry::DatabaseInfo;
use serde_json::Value;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

/// Execute a SurrealQL query
pub async fn execute(info: &DatabaseInfo, query: &str) -> anyhow::Result<Vec<Value>> {
    let url = format!("{}:{}", info.host, info.port);

    let db: Surreal<Client> = Surreal::new::<Ws>(&url).await?;

    // Authenticate
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select namespace and database
    if let (Some(ns), Some(database)) = (&info.namespace, &info.database) {
        db.use_ns(ns).use_db(database).await?;
    }

    // Execute query
    let result: Vec<Value> = db.query(query).await?.take(0)?;

    Ok(result)
}

/// Get schema from SurrealDB
pub async fn get_schema(info: &DatabaseInfo) -> anyhow::Result<Value> {
    let url = format!("{}:{}", info.host, info.port);

    let db: Surreal<Client> = Surreal::new::<Ws>(&url).await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    if let (Some(ns), Some(database)) = (&info.namespace, &info.database) {
        db.use_ns(ns).use_db(database).await?;
    }

    // Get table info
    let tables: Vec<Value> = db.query("INFO FOR DB").await?.take(0)?;

    Ok(serde_json::json!({
        "db_type": "surrealdb",
        "tables": tables
    }))
}

/// Check health of SurrealDB instance
pub async fn health_check(info: &DatabaseInfo) -> bool {
    let url = format!("{}:{}", info.host, info.port);

    match Surreal::new::<Ws>(&url).await {
        Ok(db) => {
            match db
                .signin(Root {
                    username: "root",
                    password: "root",
                })
                .await
            {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}
