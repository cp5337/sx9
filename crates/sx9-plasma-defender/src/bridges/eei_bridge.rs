//! EEI (Essential Elements of Information) Bridge
//!
//! Connects Plasma-Defender to the Leptose intelligence engine
//! for EEI queries and correlation.
//!
//! NATS Subjects:
//! - `eei.query` - Outbound queries to Leptose
//! - `eei.answer` - Inbound responses from Leptose
//! - `sx9.defender.eei.query` - JetStream persistence

use crate::ecs::components::EeiCorrelationComponent;
use crate::ring_bus::{EeiQuery, EeiResponse, RingBusNode};
use anyhow::Result;
use async_nats::Client;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};

// =============================================================================
// CONSTANTS
// =============================================================================

/// NATS subjects for EEI communication
pub mod subjects {
    /// Query to Leptose EEI engine
    pub const EEI_QUERY: &str = "eei.query";
    /// Response from Leptose EEI engine
    pub const EEI_ANSWER: &str = "eei.answer";
    /// Defender-specific EEI queries (JetStream)
    pub const DEFENDER_EEI_QUERY: &str = "sx9.defender.eei.query";
    /// Defender-specific EEI responses (JetStream)
    pub const DEFENDER_EEI_RESPONSE: &str = "sx9.defender.eei.response";
}

// =============================================================================
// REQUEST/RESPONSE TYPES
// =============================================================================

/// EEI query request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EeiQueryRequest {
    /// Unique query ID
    pub query_id: u64,
    /// Entity ID making the query
    pub entity_id: u64,
    /// Threat hash for correlation
    pub threat_hash: u64,
    /// Keywords to search
    pub keywords: Vec<String>,
    /// MITRE techniques to correlate
    pub mitre_techniques: Vec<String>,
    /// Time window start (nanos since epoch)
    pub window_start_ns: Option<u64>,
    /// Time window end (nanos since epoch)
    pub window_end_ns: Option<u64>,
    /// Maximum results
    pub max_results: Option<u32>,
    /// Requester identifier
    pub requester: String,
}

impl EeiQueryRequest {
    /// Create new EEI query
    pub fn new(entity_id: u64, threat_hash: u64) -> Self {
        Self {
            query_id: Self::generate_query_id(),
            entity_id,
            threat_hash,
            keywords: Vec::new(),
            mitre_techniques: Vec::new(),
            window_start_ns: None,
            window_end_ns: None,
            max_results: Some(10),
            requester: "plasma-defender".to_string(),
        }
    }

    /// Add keywords
    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = keywords;
        self
    }

    /// Add MITRE techniques
    pub fn with_mitre(mut self, techniques: Vec<String>) -> Self {
        self.mitre_techniques = techniques;
        self
    }

    /// Set time window
    pub fn with_window(mut self, start_ns: u64, end_ns: u64) -> Self {
        self.window_start_ns = Some(start_ns);
        self.window_end_ns = Some(end_ns);
        self
    }

    /// Generate unique query ID
    fn generate_query_id() -> u64 {
        use std::time::SystemTime;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        now ^ (rand::random::<u64>() & 0xFFFF)
    }
}

/// EEI query response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EeiQueryResponse {
    /// Query ID this responds to
    pub query_id: u64,
    /// Entity ID that made the query
    pub entity_id: u64,
    /// Success flag
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Matched EEI IDs
    pub matched_eeis: Vec<u64>,
    /// Correlation scores for each match
    pub correlation_scores: Vec<f32>,
    /// Overall correlation score
    pub overall_score: f32,
    /// Time-of-Value remaining (ms)
    pub tov_remaining_ms: u64,
    /// Matched content snippets
    pub snippets: Vec<String>,
    /// Response timestamp
    pub timestamp_ns: u64,
}

impl EeiQueryResponse {
    /// Create empty response (no matches)
    pub fn empty(query_id: u64, entity_id: u64) -> Self {
        Self {
            query_id,
            entity_id,
            success: true,
            error: None,
            matched_eeis: Vec::new(),
            correlation_scores: Vec::new(),
            overall_score: 0.0,
            tov_remaining_ms: 0,
            snippets: Vec::new(),
            timestamp_ns: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        }
    }

    /// Create error response
    pub fn error(query_id: u64, entity_id: u64, error: impl Into<String>) -> Self {
        Self {
            query_id,
            entity_id,
            success: false,
            error: Some(error.into()),
            matched_eeis: Vec::new(),
            correlation_scores: Vec::new(),
            overall_score: 0.0,
            tov_remaining_ms: 0,
            snippets: Vec::new(),
            timestamp_ns: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        }
    }

    /// Convert to ECS component
    pub fn to_component(&self) -> EeiCorrelationComponent {
        EeiCorrelationComponent {
            eei_id: self.matched_eeis.first().copied(),
            correlation_score: self.overall_score,
            matched_keywords: self.snippets.clone(),
            tov_remaining_ms: self.tov_remaining_ms,
            last_correlation_ns: self.timestamp_ns,
        }
    }
}

// =============================================================================
// EEI BRIDGE
// =============================================================================

/// EEI Bridge - connects to Leptose EEI engine
pub struct EeiBridge {
    /// NATS client
    client: Client,
    /// Ring Bus for JetStream persistence
    ring_bus: Option<Arc<RingBusNode>>,
    /// Pending queries awaiting responses
    pending: Arc<RwLock<HashMap<u64, mpsc::Sender<EeiQueryResponse>>>>,
    /// Query timeout
    timeout: Duration,
}

impl EeiBridge {
    /// Create new EEI bridge
    pub async fn new(nats_url: &str) -> Result<Self> {
        let client = async_nats::connect(nats_url).await?;

        Ok(Self {
            client,
            ring_bus: None,
            pending: Arc::new(RwLock::new(HashMap::new())),
            timeout: Duration::from_secs(5),
        })
    }

    /// Connect to Ring Bus for JetStream persistence
    pub fn with_ring_bus(mut self, ring_bus: Arc<RingBusNode>) -> Self {
        self.ring_bus = Some(ring_bus);
        self
    }

    /// Set query timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Query EEI system synchronously
    pub async fn query(&self, request: &EeiQueryRequest) -> Result<EeiQueryResponse> {
        let payload = serde_json::to_vec(request)?;

        // Publish to JetStream if available
        if let Some(ref ring_bus) = self.ring_bus {
            let eei_query = EeiQuery {
                query_id: request.query_id,
                keywords: request.keywords.clone(),
                mitre_techniques: request.mitre_techniques.clone(),
                window_start_ns: request.window_start_ns.unwrap_or(0),
                window_end_ns: request.window_end_ns.unwrap_or(u64::MAX),
                requester: request.requester.clone(),
            };
            ring_bus.publish_eei_query(&eei_query).await?;
        }

        // Send request and wait for response
        let response = tokio::time::timeout(
            self.timeout,
            self.client
                .request(subjects::EEI_QUERY.to_string(), payload.into()),
        )
        .await;

        match response {
            Ok(Ok(msg)) => {
                let response: EeiQueryResponse = serde_json::from_slice(&msg.payload)?;
                Ok(response)
            }
            Ok(Err(e)) => Ok(EeiQueryResponse::error(
                request.query_id,
                request.entity_id,
                format!("NATS error: {}", e),
            )),
            Err(_) => Ok(EeiQueryResponse::error(
                request.query_id,
                request.entity_id,
                "Query timeout",
            )),
        }
    }

    /// Query EEI system asynchronously (non-blocking)
    pub async fn query_async(
        &self,
        request: EeiQueryRequest,
    ) -> Result<mpsc::Receiver<EeiQueryResponse>> {
        let (tx, rx) = mpsc::channel(1);

        // Store pending query
        self.pending.write().await.insert(request.query_id, tx);

        // Publish query
        let payload = serde_json::to_vec(&request)?;
        self.client
            .publish(subjects::EEI_QUERY.to_string(), payload.into())
            .await?;

        Ok(rx)
    }

    /// Start listening for responses
    pub async fn start_listener(&self) -> Result<()> {
        let mut subscriber = self
            .client
            .subscribe(subjects::EEI_ANSWER.to_string())
            .await?;
        let pending = self.pending.clone();

        tokio::spawn(async move {
            while let Some(msg) = subscriber.next().await {
                if let Ok(response) = serde_json::from_slice::<EeiQueryResponse>(&msg.payload) {
                    let mut pending = pending.write().await;
                    if let Some(tx) = pending.remove(&response.query_id) {
                        let _ = tx.send(response).await;
                    }
                }
            }
        });

        Ok(())
    }

    /// Query by MITRE technique
    pub async fn query_by_mitre(
        &self,
        entity_id: u64,
        threat_hash: u64,
        technique: &str,
    ) -> Result<EeiQueryResponse> {
        let request =
            EeiQueryRequest::new(entity_id, threat_hash).with_mitre(vec![technique.to_string()]);
        self.query(&request).await
    }

    /// Query by keywords
    pub async fn query_by_keywords(
        &self,
        entity_id: u64,
        threat_hash: u64,
        keywords: Vec<String>,
    ) -> Result<EeiQueryResponse> {
        let request = EeiQueryRequest::new(entity_id, threat_hash).with_keywords(keywords);
        self.query(&request).await
    }

    /// Correlate threat with EEI and update component
    pub async fn correlate_threat(
        &self,
        entity_id: u64,
        threat_hash: u64,
        technique: Option<&str>,
        keywords: Vec<String>,
    ) -> Result<EeiCorrelationComponent> {
        let mut request = EeiQueryRequest::new(entity_id, threat_hash).with_keywords(keywords);

        if let Some(tech) = technique {
            request = request.with_mitre(vec![tech.to_string()]);
        }

        let response = self.query(&request).await?;
        Ok(response.to_component())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eei_query_request() {
        let request = EeiQueryRequest::new(123, 456)
            .with_keywords(vec!["powershell".to_string()])
            .with_mitre(vec!["T1059.001".to_string()]);

        assert_eq!(request.entity_id, 123);
        assert_eq!(request.threat_hash, 456);
        assert!(request.keywords.contains(&"powershell".to_string()));
        assert!(request.mitre_techniques.contains(&"T1059.001".to_string()));
    }

    #[test]
    fn test_eei_response_to_component() {
        let response = EeiQueryResponse {
            query_id: 1,
            entity_id: 2,
            success: true,
            error: None,
            matched_eeis: vec![100, 101],
            correlation_scores: vec![0.9, 0.8],
            overall_score: 0.85,
            tov_remaining_ms: 3600000,
            snippets: vec!["match1".to_string()],
            timestamp_ns: 0,
        };

        let component = response.to_component();
        assert_eq!(component.eei_id, Some(100));
        assert_eq!(component.correlation_score, 0.85);
    }
}
