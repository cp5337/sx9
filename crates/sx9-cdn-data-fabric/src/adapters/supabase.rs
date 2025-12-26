//! Supabase Adapter
//!
//! Handles communication with Supabase instances via PostgREST API.
//! Replaces SurrealDB adapter (100 crate deps) with lightweight HTTP client.
//!
//! RFC-9005: Canonical storage architecture

use crate::registry::DatabaseInfo;
use reqwest::Client;
use serde_json::Value;

/// Supabase client for PostgREST queries
pub struct SupabaseClient {
    client: Client,
    base_url: String,
    anon_key: String,
}

impl SupabaseClient {
    pub fn new(base_url: &str, anon_key: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            anon_key: anon_key.to_string(),
        }
    }

    pub fn from_db_info(info: &DatabaseInfo) -> Self {
        let base_url = format!("{}:{}", info.host, info.port);
        let anon_key = info.namespace.clone().unwrap_or_default();
        Self::new(&base_url, &anon_key)
    }
}

/// Execute a PostgREST query against Supabase
/// Query format: "table_name?select=*&column=eq.value"
pub async fn execute(info: &DatabaseInfo, query: &str) -> anyhow::Result<Vec<Value>> {
    let client = SupabaseClient::from_db_info(info);

    // Parse query - expected format: "table_name?params" or just "table_name"
    let (table, params) = if query.contains('?') {
        let parts: Vec<&str> = query.splitn(2, '?').collect();
        (parts[0], Some(parts[1]))
    } else {
        (query.trim(), None)
    };

    let url = match params {
        Some(p) => format!("{}/rest/v1/{}?{}", client.base_url, table, p),
        None => format!("{}/rest/v1/{}?select=*", client.base_url, table),
    };

    let response = client
        .client
        .get(&url)
        .header("apikey", &client.anon_key)
        .header("Authorization", format!("Bearer {}", client.anon_key))
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!(
            "Supabase query failed: {} - {}",
            status,
            error_body
        ));
    }

    let result: Vec<Value> = response.json().await?;
    Ok(result)
}

/// Insert data into Supabase table
pub async fn insert(info: &DatabaseInfo, table: &str, data: &Value) -> anyhow::Result<Value> {
    let client = SupabaseClient::from_db_info(info);
    let url = format!("{}/rest/v1/{}", client.base_url, table);

    let response = client
        .client
        .post(&url)
        .header("apikey", &client.anon_key)
        .header("Authorization", format!("Bearer {}", client.anon_key))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .json(data)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!(
            "Supabase insert failed: {} - {}",
            status,
            error_body
        ));
    }

    let result: Value = response.json().await?;
    Ok(result)
}

/// Update data in Supabase table
pub async fn update(
    info: &DatabaseInfo,
    table: &str,
    filter: &str,
    data: &Value,
) -> anyhow::Result<Value> {
    let client = SupabaseClient::from_db_info(info);
    let url = format!("{}/rest/v1/{}?{}", client.base_url, table, filter);

    let response = client
        .client
        .patch(&url)
        .header("apikey", &client.anon_key)
        .header("Authorization", format!("Bearer {}", client.anon_key))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .json(data)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!(
            "Supabase update failed: {} - {}",
            status,
            error_body
        ));
    }

    let result: Value = response.json().await?;
    Ok(result)
}

/// Get schema from Supabase (via OpenAPI spec)
pub async fn get_schema(info: &DatabaseInfo) -> anyhow::Result<Value> {
    let client = SupabaseClient::from_db_info(info);
    let url = format!("{}/rest/v1/", client.base_url);

    let response = client
        .client
        .get(&url)
        .header("apikey", &client.anon_key)
        .header("Authorization", format!("Bearer {}", client.anon_key))
        .send()
        .await?;

    Ok(serde_json::json!({
        "db_type": "supabase",
        "endpoint": url,
        "status": response.status().as_u16()
    }))
}

/// Check health of Supabase instance
pub async fn health_check(info: &DatabaseInfo) -> bool {
    let client = SupabaseClient::from_db_info(info);
    let url = format!("{}/rest/v1/", client.base_url);

    match client
        .client
        .get(&url)
        .header("apikey", &client.anon_key)
        .send()
        .await
    {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_parsing() {
        let query = "node_interviews?select=*&type=eq.crate";
        let (table, params) = if query.contains('?') {
            let parts: Vec<&str> = query.splitn(2, '?').collect();
            (parts[0], Some(parts[1]))
        } else {
            (query, None)
        };

        assert_eq!(table, "node_interviews");
        assert_eq!(params, Some("select=*&type=eq.crate"));
    }
}
