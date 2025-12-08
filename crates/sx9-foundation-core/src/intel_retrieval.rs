use crate::usim_header::{UsimHeader, IntelSource, ThreatCategory};
use crate::hash_engine::Hasher as Blake3Hasher;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, warn, error};

/// AI/Internet Intelligence Retrieval System
/// Fetches threat intelligence from multiple sources for hash analysis
pub struct IntelligenceRetrieval {
    http_client: Client,
    ai_endpoint: String,
    threat_feeds: Vec<ThreatFeed>,
    cache: Arc<RwLock<HashMap<[u8; 32], IntelligenceData>>>,
    statistics: Arc<RwLock<RetrievalStats>>,
}

/// Threat intelligence feed configuration
#[derive(Debug, Clone)]
pub struct ThreatFeed {
    pub name: String,
    pub url: String,
    pub api_key: Option<String>,
    pub feed_type: FeedType,
    pub refresh_interval: Duration,
    pub last_update: Option<Instant>,
}

/// Types of threat intelligence feeds
#[derive(Debug, Clone)]
pub enum FeedType {
    MalwareBazaar,      // MalwareBazaar API
    VirusTotal,         // VirusTotal API
    AlienVault,         // AlienVault OTX
    ThreatFox,          // Abuse.ch ThreatFox
    URLVoid,            // URLVoid API
    Hybrid,             // Hybrid Analysis
    Custom,             // Custom API endpoint
}

/// Intelligence data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceData {
    pub hash: [u8; 32],
    pub threat_category: ThreatCategory,
    pub confidence_score: f32,
    pub sources: Vec<String>,
    pub mitre_tactics: Vec<String>,
    pub first_seen: u64,
    pub last_seen: u64,
    pub detection_count: u32,
    pub family_name: Option<String>,
    pub description: Option<String>,
    pub external_refs: Vec<String>,
}

/// Retrieval statistics for analysis
#[derive(Debug, Default, Clone)]
pub struct RetrievalStats {
    pub total_queries: u64,
    pub cache_hits: u64,
    pub ai_queries: u64,
    pub internet_queries: u64,
    pub avg_response_time_ms: f64,
    pub error_count: u64,
    pub feeds_updated: HashMap<String, u64>,
}

/// Test documents for hash intelligence evaluation
pub struct TestDocuments {
    pub malware_samples: Vec<TestDocument>,
    pub benign_samples: Vec<TestDocument>,
    pub phishing_samples: Vec<TestDocument>,
    pub unknown_samples: Vec<TestDocument>,
}

#[derive(Debug, Clone)]
pub struct TestDocument {
    pub name: String,
    pub content: Vec<u8>,
    pub expected_category: ThreatCategory,
    pub hash: [u8; 32],
    pub source: String,
}

impl IntelligenceRetrieval {
    /// Create new intelligence retrieval system
    pub fn new(ai_endpoint: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("CTAS-7.0 Intelligence Engine")
            .build()
            .unwrap();

        let threat_feeds = Self::setup_default_feeds();

        Self {
            http_client,
            ai_endpoint,
            threat_feeds,
            cache: Arc::new(RwLock::new(HashMap::with_capacity(100_000))),
            statistics: Arc::new(RwLock::new(RetrievalStats::default())),
        }
    }

    /// Setup default threat intelligence feeds
    fn setup_default_feeds() -> Vec<ThreatFeed> {
        vec![
            ThreatFeed {
                name: "MalwareBazaar".to_string(),
                url: "https://mb-api.abuse.ch/api/v1/".to_string(),
                api_key: None,
                feed_type: FeedType::MalwareBazaar,
                refresh_interval: Duration::from_secs(3600), // 1 hour
                last_update: None,
            },
            ThreatFeed {
                name: "ThreatFox".to_string(),
                url: "https://threatfox-api.abuse.ch/api/v1/".to_string(),
                api_key: None,
                feed_type: FeedType::ThreatFox,
                refresh_interval: Duration::from_secs(1800), // 30 minutes
                last_update: None,
            },
            ThreatFeed {
                name: "VirusTotal".to_string(),
                url: "https://www.virustotal.com/vtapi/v2/".to_string(),
                api_key: std::env::var("VT_API_KEY").ok(),
                feed_type: FeedType::VirusTotal,
                refresh_interval: Duration::from_secs(300), // 5 minutes
                last_update: None,
            },
        ]
    }

    /// Retrieve intelligence for a hash with full source analysis
    pub async fn retrieve_intelligence(&self, hash: [u8; 32], header: &mut UsimHeader) -> Result<IntelligenceData, Box<dyn std::error::Error>> {
        let start_time = Instant::now();
        let mut stats = self.statistics.write().await;
        stats.total_queries += 1;
        drop(stats);

        // 1. Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(data) = cache.get(&hash) {
                let mut stats = self.statistics.write().await;
                stats.cache_hits += 1;
                drop(stats);

                header.add_intelligence(
                    IntelSource::Cache,
                    data.confidence_score,
                    data.threat_category.clone(),
                    data.mitre_tactics.clone(),
                    data.external_refs.clone(),
                );

                info!("Cache hit for hash: {:02x?}", &hash[0..8]);
                return Ok(data.clone());
            }
        }

        // 2. Try AI inference first (fastest)
        if let Ok(ai_data) = self.query_ai_inference(&hash).await {
            let mut stats = self.statistics.write().await;
            stats.ai_queries += 1;
            drop(stats);

            // Cache the result
            {
                let mut cache = self.cache.write().await;
                cache.insert(hash, ai_data.clone());
            }

            header.add_intelligence(
                IntelSource::AI,
                ai_data.confidence_score,
                ai_data.threat_category.clone(),
                ai_data.mitre_tactics.clone(),
                ai_data.external_refs.clone(),
            );

            info!("AI inference result for hash: {:02x?}", &hash[0..8]);
            return Ok(ai_data);
        }

        // 3. Query internet threat feeds
        let mut combined_data = None;
        let mut sources = Vec::new();

        for feed in &self.threat_feeds {
            if let Ok(feed_data) = self.query_threat_feed(feed, &hash).await {
                sources.push(feed.name.clone());

                match combined_data {
                    None => combined_data = Some(feed_data),
                    Some(ref mut existing) => {
                        // Merge data from multiple sources
                        existing.confidence_score = (existing.confidence_score + feed_data.confidence_score) / 2.0;
                        existing.sources.extend(feed_data.sources);
                        existing.mitre_tactics.extend(feed_data.mitre_tactics);
                        existing.external_refs.extend(feed_data.external_refs);
                        existing.detection_count += feed_data.detection_count;
                    }
                }
            }
        }

        let mut stats = self.statistics.write().await;
        stats.internet_queries += 1;
        let elapsed = start_time.elapsed().as_millis() as f64;
        stats.avg_response_time_ms = (stats.avg_response_time_ms * (stats.total_queries - 1) as f64 + elapsed) / stats.total_queries as f64;
        drop(stats);

        if let Some(mut data) = combined_data {
            data.sources = sources;

            // Cache the result
            {
                let mut cache = self.cache.write().await;
                cache.insert(hash, data.clone());
            }

            header.add_intelligence(
                IntelSource::Internet,
                data.confidence_score,
                data.threat_category.clone(),
                data.mitre_tactics.clone(),
                data.external_refs.clone(),
            );

            info!("Internet intelligence result for hash: {:02x?}", &hash[0..8]);
            Ok(data)
        } else {
            // No intelligence found
            let unknown_data = IntelligenceData {
                hash,
                threat_category: ThreatCategory::Unknown,
                confidence_score: 0.0,
                sources: vec!["None".to_string()],
                mitre_tactics: vec![],
                first_seen: 0,
                last_seen: 0,
                detection_count: 0,
                family_name: None,
                description: Some("No threat intelligence available".to_string()),
                external_refs: vec![],
            };

            header.add_intelligence(
                IntelSource::Internet,
                0.0,
                ThreatCategory::Unknown,
                vec![],
                vec![],
            );

            warn!("No intelligence found for hash: {:02x?}", &hash[0..8]);
            Ok(unknown_data)
        }
    }

    /// Query AI inference endpoint
    async fn query_ai_inference(&self, hash: &[u8; 32]) -> Result<IntelligenceData, Box<dyn std::error::Error>> {
        let hex_hash = hex::encode(hash);

        let request_body = serde_json::json!({
            "hash": hex_hash,
            "analysis_type": "threat_classification",
            "include_mitre": true,
            "confidence_threshold": 0.7
        });

        let response = self.http_client
            .post(&self.ai_endpoint)
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let ai_response: AIInferenceResponse = response.json().await?;

            Ok(IntelligenceData {
                hash: *hash,
                threat_category: self.parse_threat_category(&ai_response.category),
                confidence_score: ai_response.confidence,
                sources: vec!["AI-Model".to_string()],
                mitre_tactics: ai_response.mitre_tactics,
                first_seen: 0,
                last_seen: 0,
                detection_count: 1,
                family_name: ai_response.family,
                description: ai_response.description,
                external_refs: vec![],
            })
        } else {
            Err(format!("AI inference failed: {}", response.status()).into())
        }
    }

    /// Query specific threat feed
    async fn query_threat_feed(&self, feed: &ThreatFeed, hash: &[u8; 32]) -> Result<IntelligenceData, Box<dyn std::error::Error>> {
        let hex_hash = hex::encode(hash);

        match feed.feed_type {
            FeedType::MalwareBazaar => self.query_malware_bazaar(&hex_hash).await,
            FeedType::ThreatFox => self.query_threatfox(&hex_hash).await,
            FeedType::VirusTotal => self.query_virustotal(&hex_hash, feed.api_key.as_ref()).await,
            _ => Err("Feed type not implemented".into()),
        }
    }

    /// Query MalwareBazaar
    async fn query_malware_bazaar(&self, hex_hash: &str) -> Result<IntelligenceData, Box<dyn std::error::Error>> {
        let request_body = serde_json::json!({
            "query": "get_info",
            "hash": hex_hash
        });

        let response = self.http_client
            .post("https://mb-api.abuse.ch/api/v1/")
            .json(&request_body)
            .send()
            .await?;

        // Parse MalwareBazaar response and convert to IntelligenceData
        // Implementation depends on actual API response format
        Err("MalwareBazaar parsing not implemented".into())
    }

    /// Query ThreatFox
    async fn query_threatfox(&self, hex_hash: &str) -> Result<IntelligenceData, Box<dyn std::error::Error>> {
        // Similar implementation for ThreatFox API
        Err("ThreatFox parsing not implemented".into())
    }

    /// Query VirusTotal
    async fn query_virustotal(&self, hex_hash: &str, api_key: Option<&String>) -> Result<IntelligenceData, Box<dyn std::error::Error>> {
        let api_key = api_key.ok_or("VirusTotal API key required")?;

        let url = format!("https://www.virustotal.com/vtapi/v2/file/report?apikey={}&resource={}", api_key, hex_hash);

        let response = self.http_client.get(&url).send().await?;

        // Parse VirusTotal response
        Err("VirusTotal parsing not implemented".into())
    }

    /// Parse threat category from string
    fn parse_threat_category(&self, category: &str) -> ThreatCategory {
        match category.to_lowercase().as_str() {
            "malware" => ThreatCategory::Malware,
            "phishing" => ThreatCategory::Phishing,
            "c2" | "c&c" | "command-control" => ThreatCategory::C2Infrastructure,
            "exfiltration" => ThreatCategory::DataExfiltration,
            "reconnaissance" | "recon" => ThreatCategory::Reconnaissance,
            "lateral-movement" => ThreatCategory::LateralMovement,
            "persistence" => ThreatCategory::Persistence,
            "privilege-escalation" => ThreatCategory::PrivilegeEscalation,
            "defense-evasion" => ThreatCategory::DefenseEvasion,
            _ => ThreatCategory::Unknown,
        }
    }

    /// Generate test documents for evaluation
    pub fn generate_test_documents() -> TestDocuments {
        let mut documents = TestDocuments {
            malware_samples: Vec::new(),
            benign_samples: Vec::new(),
            phishing_samples: Vec::new(),
            unknown_samples: Vec::new(),
        };

        // Generate known malware samples
        documents.malware_samples.push(Self::create_test_document(
            "Zeus Banking Trojan",
            b"fake_zeus_malware_sample_for_testing",
            ThreatCategory::Malware,
            "Simulated Zeus sample",
        ));

        documents.malware_samples.push(Self::create_test_document(
            "Emotet Payload",
            b"fake_emotet_payload_simulation",
            ThreatCategory::Malware,
            "Simulated Emotet sample",
        ));

        // Generate phishing samples
        documents.phishing_samples.push(Self::create_test_document(
            "Fake PayPal Email",
            b"fake_paypal_phishing_email_content",
            ThreatCategory::Phishing,
            "Simulated phishing email",
        ));

        // Generate benign samples
        documents.benign_samples.push(Self::create_test_document(
            "System Binary",
            b"legitimate_windows_system32_binary",
            ThreatCategory::Unknown, // Benign
            "Legitimate system file",
        ));

        // Generate unknown samples
        documents.unknown_samples.push(Self::create_test_document(
            "Novel File",
            b"completely_unknown_file_content_sample",
            ThreatCategory::Unknown,
            "Unknown file type",
        ));

        documents
    }

    /// Create a test document with hash
    fn create_test_document(name: &str, content: &[u8], category: ThreatCategory, source: &str) -> TestDocument {
        let mut hasher = Blake3Hasher::new();
        hasher.update(content);
        let hash = hasher.finalize().into();

        TestDocument {
            name: name.to_string(),
            content: content.to_vec(),
            expected_category: category,
            hash,
            source: source.to_string(),
        }
    }

    /// Get retrieval statistics
    pub async fn get_statistics(&self) -> RetrievalStats {
        self.statistics.read().await.clone()
    }
}

/// AI inference response structure
#[derive(Debug, Deserialize)]
struct AIInferenceResponse {
    category: String,
    confidence: f32,
    mitre_tactics: Vec<String>,
    family: Option<String>,
    description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_generation() {
        let docs = IntelligenceRetrieval::generate_test_documents();
        assert!(!docs.malware_samples.is_empty());
        assert!(!docs.phishing_samples.is_empty());
        assert!(!docs.benign_samples.is_empty());
    }

    #[test]
    fn test_threat_category_parsing() {
        let intel = IntelligenceRetrieval::new("http://localhost:8080".to_string());
        assert_eq!(intel.parse_threat_category("malware"), ThreatCategory::Malware);
        assert_eq!(intel.parse_threat_category("phishing"), ThreatCategory::Phishing);
        assert_eq!(intel.parse_threat_category("unknown"), ThreatCategory::Unknown);
    }
}