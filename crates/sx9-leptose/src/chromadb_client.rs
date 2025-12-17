//! ChromaDB Client - Query existing vector collections
//!
//! Connects to the existing ChromaDB instance populated by:
//! - `add_to_chromadb_with_unicode.py`
//! - `threat_vector_pipeline.py`
//!
//! Collections:
//! - `tools` - Threat intel tools with Unicode ops
//! - `ctas_tasks` - CTAS tasks with Unicode ops
//! - `ptcc_configs` - PTCC configurations
//! - `tool_chains` - Tool chains
//! - `threat_content` - Vectorized threat intel (MITRE, Atomic, Sigma, etc.)

use crate::{LeptoseError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// ChromaDB collection names
pub mod collections {
    pub const TOOLS: &str = "tools";
    pub const CTAS_TASKS: &str = "ctas_tasks";
    pub const PTCC_CONFIGS: &str = "ptcc_configs";
    pub const TOOL_CHAINS: &str = "tool_chains";
    pub const THREAT_CONTENT: &str = "threat_content";
}

/// Query result from ChromaDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub id: String,
    pub document: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub distance: f32,
}

/// ChromaDB client configuration
#[derive(Debug, Clone)]
pub struct ChromaDbConfig {
    /// Path to ChromaDB persistent storage
    pub db_path: PathBuf,
    /// HTTP endpoint (if using server mode)
    pub http_endpoint: Option<String>,
    /// Default number of results
    pub default_n_results: usize,
}

impl Default for ChromaDbConfig {
    fn default() -> Self {
        Self {
            db_path: PathBuf::from(
                "tools/abe/iac/04-abe-iac/node-interview-generator/output/vectors/chromadb",
            ),
            http_endpoint: None,
            default_n_results: 10,
        }
    }
}

/// ChromaDB client for querying existing collections
///
/// Note: This is a lightweight client that shells out to Python
/// for actual ChromaDB operations. For high-performance use cases,
/// consider running ChromaDB in server mode with HTTP API.
pub struct ChromaDbClient {
    config: ChromaDbConfig,
}

impl ChromaDbClient {
    pub fn new(config: ChromaDbConfig) -> Self {
        Self { config }
    }

    /// Query a collection by text similarity
    pub async fn query(
        &self,
        collection: &str,
        query_text: &str,
        n_results: Option<usize>,
        where_filter: Option<HashMap<String, String>>,
    ) -> Result<Vec<QueryResult>> {
        let n = n_results.unwrap_or(self.config.default_n_results);

        // Build Python query script
        let filter_json = where_filter
            .map(|f| serde_json::to_string(&f).unwrap_or_default())
            .unwrap_or_else(|| "null".to_string());

        let script = format!(
            r#"
import chromadb
import json
from chromadb.config import Settings

client = chromadb.PersistentClient(
    path="{db_path}",
    settings=Settings(anonymized_telemetry=False)
)

try:
    collection = client.get_collection("{collection}")
    
    where_filter = {filter_json}
    
    results = collection.query(
        query_texts=["{query_text}"],
        n_results={n},
        where=where_filter if where_filter else None,
        include=["documents", "metadatas", "distances"]
    )
    
    output = []
    if results and results['ids'] and results['ids'][0]:
        for i, doc_id in enumerate(results['ids'][0]):
            output.append({{
                "id": doc_id,
                "document": results['documents'][0][i] if results['documents'] else "",
                "metadata": results['metadatas'][0][i] if results['metadatas'] else {{}},
                "distance": results['distances'][0][i] if results['distances'] else 0.0
            }})
    
    print(json.dumps(output))
except Exception as e:
    print(json.dumps({{"error": str(e)}}))
"#,
            db_path = self.config.db_path.display(),
            collection = collection,
            query_text = query_text.replace('"', r#"\""#),
            filter_json = filter_json,
            n = n,
        );

        // Execute Python script
        let output = tokio::process::Command::new("python3")
            .arg("-c")
            .arg(&script)
            .output()
            .await
            .map_err(|e| {
                LeptoseError::VectorError(format!("Failed to execute ChromaDB query: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(LeptoseError::VectorError(format!(
                "ChromaDB query failed: {}",
                stderr
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let results: Vec<QueryResult> = serde_json::from_str(&stdout).map_err(|e| {
            LeptoseError::VectorError(format!("Failed to parse ChromaDB results: {}", e))
        })?;

        Ok(results)
    }

    /// Query tools collection
    pub async fn query_tools(&self, query: &str, n: usize) -> Result<Vec<QueryResult>> {
        self.query(collections::TOOLS, query, Some(n), None).await
    }

    /// Query CTAS tasks collection
    pub async fn query_tasks(&self, query: &str, n: usize) -> Result<Vec<QueryResult>> {
        self.query(collections::CTAS_TASKS, query, Some(n), None)
            .await
    }

    /// Query PTCC configs collection
    pub async fn query_ptcc(&self, query: &str, n: usize) -> Result<Vec<QueryResult>> {
        self.query(collections::PTCC_CONFIGS, query, Some(n), None)
            .await
    }

    /// Query threat content collection
    pub async fn query_threats(&self, query: &str, n: usize) -> Result<Vec<QueryResult>> {
        self.query(collections::THREAT_CONTENT, query, Some(n), None)
            .await
    }

    /// Find tools for a specific MITRE technique
    pub async fn find_tools_for_technique(&self, technique_id: &str) -> Result<Vec<QueryResult>> {
        let mut filter = HashMap::new();
        filter.insert("mitre_techniques".to_string(), technique_id.to_string());
        self.query(collections::TOOLS, technique_id, Some(20), Some(filter))
            .await
    }

    /// Find EEI satisfiers - tools/tasks that can answer an EEI
    pub async fn find_eei_satisfiers(&self, eei_question: &str) -> Result<EeiSatisfiers> {
        // Query across multiple collections
        let tools = self.query_tools(eei_question, 5).await?;
        let tasks = self.query_tasks(eei_question, 5).await?;
        let threats = self.query_threats(eei_question, 5).await?;

        Ok(EeiSatisfiers {
            tools,
            tasks,
            threat_intel: threats,
        })
    }

    /// Get collection statistics
    pub async fn get_stats(&self) -> Result<ChromaDbStats> {
        let script = format!(
            r#"
import chromadb
import json
from chromadb.config import Settings

client = chromadb.PersistentClient(
    path="{db_path}",
    settings=Settings(anonymized_telemetry=False)
)

stats = {{}}
for name in ["tools", "ctas_tasks", "ptcc_configs", "tool_chains", "threat_content"]:
    try:
        coll = client.get_collection(name)
        stats[name] = coll.count()
    except:
        stats[name] = 0

print(json.dumps(stats))
"#,
            db_path = self.config.db_path.display(),
        );

        let output = tokio::process::Command::new("python3")
            .arg("-c")
            .arg(&script)
            .output()
            .await
            .map_err(|e| LeptoseError::VectorError(format!("Failed to get stats: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let counts: HashMap<String, usize> = serde_json::from_str(&stdout)
            .map_err(|e| LeptoseError::VectorError(format!("Failed to parse stats: {}", e)))?;

        Ok(ChromaDbStats {
            tools_count: *counts.get("tools").unwrap_or(&0),
            tasks_count: *counts.get("ctas_tasks").unwrap_or(&0),
            ptcc_count: *counts.get("ptcc_configs").unwrap_or(&0),
            chains_count: *counts.get("tool_chains").unwrap_or(&0),
            threat_count: *counts.get("threat_content").unwrap_or(&0),
        })
    }
}

/// EEI satisfiers from multiple collections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EeiSatisfiers {
    pub tools: Vec<QueryResult>,
    pub tasks: Vec<QueryResult>,
    pub threat_intel: Vec<QueryResult>,
}

/// ChromaDB statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromaDbStats {
    pub tools_count: usize,
    pub tasks_count: usize,
    pub ptcc_count: usize,
    pub chains_count: usize,
    pub threat_count: usize,
}

impl ChromaDbStats {
    pub fn total(&self) -> usize {
        self.tools_count
            + self.tasks_count
            + self.ptcc_count
            + self.chains_count
            + self.threat_count
    }
}

impl Default for ChromaDbClient {
    fn default() -> Self {
        Self::new(ChromaDbConfig::default())
    }
}
