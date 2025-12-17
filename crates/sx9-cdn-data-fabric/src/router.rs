//! Query Router
//!
//! Routes queries to appropriate database adapters based on:
//! - Target database
//! - Query language (SQL, SurrealQL, Cypher)
//! - Capabilities required

use crate::registry::DatabaseInfo;
use serde_json::Value;

/// Determine query language from query string
pub fn detect_query_language(query: &str) -> QueryLanguage {
    let query_upper = query.to_uppercase();

    if query_upper.contains("MATCH") && query_upper.contains("RETURN") {
        QueryLanguage::Cypher
    } else if query_upper.contains("FETCH")
        || query_upper.contains("->")
        || query_upper.contains("<-")
    {
        QueryLanguage::SurrealQL
    } else if query_upper.starts_with("SELECT") || query_upper.starts_with("INSERT") {
        // Could be SQL or SurrealQL - check for SurrealDB-specific syntax
        if query_upper.contains("FROM ") && !query_upper.contains("->") {
            QueryLanguage::SQL
        } else {
            QueryLanguage::SurrealQL
        }
    } else {
        QueryLanguage::Unknown
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QueryLanguage {
    SQL,
    SurrealQL,
    Cypher,
    Gremlin,
    Unknown,
}

/// Find best database for a query
pub fn find_best_database(
    databases: &[DatabaseInfo],
    query_lang: &QueryLanguage,
    required_capabilities: &[String],
) -> Option<DatabaseInfo> {
    databases
        .iter()
        .filter(|db| {
            // Check capabilities
            required_capabilities
                .iter()
                .all(|cap| db.capabilities.contains(cap))
        })
        .filter(|db| {
            // Check query language support
            match query_lang {
                QueryLanguage::Cypher => db.db_type == "neo4j",
                QueryLanguage::SQL => db.db_type == "postgres" || db.db_type == "surrealdb",
                QueryLanguage::SurrealQL => db.db_type == "surrealdb",
                QueryLanguage::Gremlin => false, // Not supported yet
                QueryLanguage::Unknown => true,
            }
        })
        .next()
        .cloned()
}

/// Route configuration for a query
#[derive(Debug)]
pub struct QueryRoute {
    pub database: DatabaseInfo,
    pub query_language: QueryLanguage,
    pub transformed_query: String,
}

/// Plan query routing
pub fn plan_route(
    databases: &[DatabaseInfo],
    target_db: Option<&str>,
    query: &str,
) -> Option<QueryRoute> {
    let query_lang = detect_query_language(query);

    // If specific database requested, use it
    if let Some(db_id) = target_db {
        let db = databases.iter().find(|d| d.db_id == db_id)?;

        // Transform query if needed
        let transformed = match (&query_lang, db.db_type.as_str()) {
            (QueryLanguage::Cypher, "surrealdb") => crate::transform::cypher_to_surql(query),
            (QueryLanguage::SurrealQL, "neo4j") => crate::transform::surql_to_cypher(query),
            _ => query.to_string(),
        };

        return Some(QueryRoute {
            database: db.clone(),
            query_language: query_lang,
            transformed_query: transformed,
        });
    }

    // Auto-select database
    let db = find_best_database(databases, &query_lang, &[])?;

    Some(QueryRoute {
        database: db,
        query_language: query_lang,
        transformed_query: query.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_cypher() {
        let query = "MATCH (n:ThreatActor) RETURN n";
        assert_eq!(detect_query_language(query), QueryLanguage::Cypher);
    }

    #[test]
    fn test_detect_surql() {
        let query = "SELECT * FROM threat_actor->uses->technique FETCH technique";
        assert_eq!(detect_query_language(query), QueryLanguage::SurrealQL);
    }

    #[test]
    fn test_detect_sql() {
        let query = "SELECT * FROM users WHERE id = 1";
        assert_eq!(detect_query_language(query), QueryLanguage::SQL);
    }
}
