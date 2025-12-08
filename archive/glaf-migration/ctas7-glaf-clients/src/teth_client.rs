//! TETH Client - H2 Score retrieval from SurrealDB
//!
//! Zone C persistence layer for pre-computed convergence scores.
//! H2 is computed by GLAF Matroid Core and stored here for ATLAS retrieval.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::Surreal;
use tracing::info;

const TETH_ENDPOINT: &str = "http://glaf-teth-service:18301";
const TETH_NAMESPACE: &str = "ctas7";
const TETH_DATABASE: &str = "cognitive";
const TETH_TABLE: &str = "teth_entity_records";

/// H2 Convergence Score from TETH persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct H2Score {
    /// SCH prefix (first 16 Base96 chars of SCH-S)
    pub sch_prefix: String,
    /// Convergence value (0.0 - 1.0)
    pub value: f64,
    /// Confidence measure
    pub confidence: f64,
    /// Computation timestamp
    pub computed_at: String,
    /// Source matroid ID from Zone C
    pub matroid_id: String,
}

/// TETH Client for H2 score retrieval
pub struct TethClient {
    db: Surreal<Client>,
}

impl TethClient {
    /// Connect to TETH service at default endpoint
    pub async fn connect() -> Result<Self> {
        Self::connect_to(TETH_ENDPOINT).await
    }

    /// Connect to TETH service at specified endpoint
    pub async fn connect_to(endpoint: &str) -> Result<Self> {
        let db = Surreal::new::<Http>(endpoint).await?;
        db.signin(surrealdb::opt::auth::Root {
            username: "root",
            password: "root",
        })
        .await?;
        db.use_ns(TETH_NAMESPACE).use_db(TETH_DATABASE).await?;

        info!(endpoint = endpoint, "Connected to TETH service");
        Ok(Self { db })
    }

    /// Retrieve H2 score by SCH prefix
    ///
    /// Query: SELECT * FROM teth_entity_records WHERE sch_prefix = $prefix
    pub async fn get_h2_score(&self, sch_prefix: &str) -> Result<Option<H2Score>> {
        let table = TETH_TABLE.to_string();
        let prefix = sch_prefix.to_string();

        let mut response = self
            .db
            .query("SELECT * FROM type::table($table) WHERE sch_prefix = $prefix")
            .bind(("table", table))
            .bind(("prefix", prefix))
            .await?;

        let scores: Vec<H2Score> = response.take(0)?;
        Ok(scores.into_iter().next())
    }

    /// Retrieve multiple H2 scores by SCH prefixes
    pub async fn get_h2_scores(&self, sch_prefixes: &[&str]) -> Result<Vec<H2Score>> {
        let table = TETH_TABLE.to_string();
        let prefixes: Vec<String> = sch_prefixes.iter().map(|s| s.to_string()).collect();

        let mut response = self
            .db
            .query("SELECT * FROM type::table($table) WHERE sch_prefix IN $prefixes")
            .bind(("table", table))
            .bind(("prefixes", prefixes))
            .await?;

        let scores: Vec<H2Score> = response.take(0)?;
        Ok(scores)
    }

    /// Check if H2 score exists for entity
    pub async fn has_h2_score(&self, sch_prefix: &str) -> Result<bool> {
        let score = self.get_h2_score(sch_prefix).await?;
        Ok(score.is_some())
    }
}
