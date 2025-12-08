//! # Open CTI (Cyber Threat Intelligence) Media Ingestion System
//!
//! This system provides real-time ingestion and processing of cyber threat intelligence
//! from multiple open sources including news feeds, security blogs, vulnerability databases,
//! social media, and government advisories.
//!
//! Feeds the EEI Decision Engine with structured threat intelligence for:
//! - Real-time threat landscape assessment
//! - Attack pattern recognition and analysis
//! - Vulnerability intelligence and impact assessment
//! - Geopolitical event correlation with cyber threats
//! - Social engineering and disinformation campaign detection

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::eei_decision_engine::{EeiDecisionEngine, NyxTraceIntelligence};

/// Open CTI ingestion engine for real-time threat intelligence
#[derive(Debug)]
pub struct OpenCtiIngestionEngine {
    /// RSS/Atom feed processors for security news and advisories
    feed_processors: Arc<RwLock<HashMap<String, FeedProcessor>>>,
    /// Social media intelligence collectors
    social_intel_collectors: Arc<RwLock<HashMap<String, SocialIntelCollector>>>,
    /// Government advisory processors (CISA, NIST, etc.)
    gov_advisory_processors: Arc<RwLock<HashMap<String, GovAdvisoryProcessor>>>,
    /// Vulnerability database processors (CVE, NVD, etc.)
    vuln_db_processors: Arc<RwLock<HashMap<String, VulnDbProcessor>>>,
    /// Dark web monitoring systems
    darkweb_monitors: Arc<RwLock<HashMap<String, DarkwebMonitor>>>,
    /// Intelligence analysis engine for processing raw data
    intel_analyzer: Arc<IntelligenceAnalyzer>,
    /// EEI decision engine integration
    eei_engine: Arc<EeiDecisionEngine>,
    /// Pub/sub channel for real-time intelligence distribution
    intelligence_channel: mpsc::UnboundedSender<ProcessedIntelligence>,
    /// Configuration for ingestion sources
    ingestion_config: Arc<RwLock<IngestionConfig>>,
}

/// Configuration for CTI ingestion sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionConfig {
    /// RSS/Atom feeds to monitor
    pub feed_sources: HashMap<String, FeedSource>,
    /// Social media sources and keywords
    pub social_sources: HashMap<String, SocialSource>,
    /// Government advisory sources
    pub government_sources: HashMap<String, GovernmentSource>,
    /// Vulnerability database sources
    pub vulnerability_sources: HashMap<String, VulnerabilitySource>,
    /// Dark web monitoring configuration
    pub darkweb_sources: HashMap<String, DarkwebSource>,
    /// Processing intervals and thresholds
    pub processing_config: ProcessingConfig,
}

/// RSS/Atom feed source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedSource {
    pub url: String,
    pub source_type: FeedSourceType,
    pub credibility_score: f64,
    pub update_interval_minutes: u64,
    pub keywords: Vec<String>,
    pub language: String,
    pub region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedSourceType {
    SecurityNews,
    VendorAdvisory,
    ResearchBlog,
    ThreatIntelligence,
    IncidentResponse,
    GovernmentAlert,
}

/// Social media intelligence source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSource {
    pub platform: SocialPlatform,
    pub keywords: Vec<String>,
    pub hashtags: Vec<String>,
    pub accounts_to_monitor: Vec<String>,
    pub sentiment_analysis: bool,
    pub geolocation_tracking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialPlatform {
    Twitter,
    Reddit,
    Telegram,
    Discord,
    LinkedIn,
    GitHub,
    GitLab,
}

/// Government advisory source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentSource {
    pub agency: GovernmentAgency,
    pub api_endpoint: String,
    pub classification_levels: Vec<String>,
    pub sectors: Vec<String>,
    pub auto_download: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernmentAgency {
    CISA,
    NIST,
    NSA,
    FBI,
    DHS,
    ENISA,     // European Union
    NCSC,      // UK
    ANSSI,     // France
    BSI,       // Germany
    JPCERT,    // Japan
    CERT_AU,   // Australia
}

/// Vulnerability database source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilitySource {
    pub database: VulnerabilityDatabase,
    pub api_endpoint: String,
    pub cvss_threshold: f64,
    pub product_filters: Vec<String>,
    pub vendor_filters: Vec<String>,
    pub auto_correlation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VulnerabilityDatabase {
    NVD,           // National Vulnerability Database
    MITRE_CVE,     // MITRE CVE Database
    ExploitDB,     // Exploit Database
    VulDB,         // VulnDB Commercial Database
    CERT_VU,       // CERT Vulnerability Notes
    FIRST_CVSS,    // FIRST CVSS Database
}

/// Dark web monitoring source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkwebSource {
    pub monitoring_type: DarkwebMonitoringType,
    pub keywords: Vec<String>,
    pub markets_to_monitor: Vec<String>,
    pub forums_to_monitor: Vec<String>,
    pub risk_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DarkwebMonitoringType {
    MarketplaceMonitoring,
    ForumDiscussions,
    DataLeaks,
    RansomwareNegotiations,
    ExploitSales,
    CredentialDumps,
}

/// Processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingConfig {
    pub max_concurrent_processors: usize,
    pub intelligence_retention_days: u64,
    pub duplicate_detection_window_hours: u64,
    pub threat_scoring_model: String,
    pub auto_enrichment: bool,
    pub real_time_alerting: bool,
}

/// Processed intelligence ready for EEI integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedIntelligence {
    pub intelligence_id: Uuid,
    pub source_type: IntelligenceSourceType,
    pub source_name: String,
    pub raw_content: String,
    pub processed_content: IntelligenceContent,
    pub threat_indicators: Vec<ThreatIndicator>,
    pub attribution: Option<ThreatAttribution>,
    pub relevance_score: f64,
    pub confidence_score: f64,
    pub urgency_level: UrgencyLevel,
    pub geographic_relevance: Vec<String>,
    pub sector_relevance: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub expiration: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntelligenceSourceType {
    OpenSource,
    SocialMedia,
    GovernmentAdvisory,
    VulnerabilityDatabase,
    DarkwebIntelligence,
    CommercialFeed,
}

/// Structured intelligence content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceContent {
    pub title: String,
    pub summary: String,
    pub threat_type: ThreatType,
    pub attack_vectors: Vec<AttackVector>,
    pub affected_technologies: Vec<String>,
    pub mitigation_strategies: Vec<String>,
    pub related_cves: Vec<String>,
    pub related_campaigns: Vec<String>,
    pub timeline: Option<ThreatTimeline>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatType {
    Malware,
    Ransomware,
    Phishing,
    DataBreach,
    VulnerabilityExploit,
    APT,
    Botnet,
    DDoS,
    InsiderThreat,
    SupplyChainAttack,
    Cryptojacking,
    Disinformation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttackVector {
    Email,
    Web,
    Network,
    USBMedia,
    SocialEngineering,
    PhysicalAccess,
    SupplyChain,
    CloudServices,
    MobileDevices,
    IoTDevices,
}

/// Threat indicator (IOC)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    pub indicator_type: IndicatorType,
    pub value: String,
    pub confidence: f64,
    pub source: String,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndicatorType {
    IPv4,
    IPv6,
    Domain,
    URL,
    EmailAddress,
    FileHash,
    FileName,
    RegistryKey,
    UserAgent,
    Certificate,
    ASN,
    CIDR,
}

/// Threat attribution information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAttribution {
    pub threat_actor: Option<String>,
    pub campaign_name: Option<String>,
    pub country_origin: Option<String>,
    pub motivation: Vec<ThreatMotivation>,
    pub sophistication_level: SophisticationLevel,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatMotivation {
    Financial,
    Espionage,
    Sabotage,
    Activism,
    Terrorism,
    StateSponsor,
    PersonalGain,
    Revenge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SophisticationLevel {
    Basic,
    Intermediate,
    Advanced,
    Expert,
    StateLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UrgencyLevel {
    Critical,    // Immediate response required
    High,        // Response within hours
    Medium,      // Response within days
    Low,         // Monitor and track
    Info,        // Information only
}

/// Threat timeline for tracking campaign evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatTimeline {
    pub initial_discovery: DateTime<Utc>,
    pub first_attacks: Option<DateTime<Utc>>,
    pub peak_activity: Option<DateTime<Utc>>,
    pub current_status: ThreatStatus,
    pub predicted_evolution: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatStatus {
    Emerging,
    Active,
    Declining,
    Dormant,
    Resolved,
}

impl OpenCtiIngestionEngine {
    /// Initialize the CTI ingestion engine
    pub async fn new(eei_engine: Arc<EeiDecisionEngine>) -> Self {
        let (intelligence_tx, mut intelligence_rx) = mpsc::unbounded_channel();

        // Clone the EEI engine for the intelligence processor
        let eei_engine_clone = eei_engine.clone();

        // Spawn background intelligence processor
        tokio::spawn(async move {
            while let Some(intelligence) = intelligence_rx.recv().await {
                if let Err(e) = eei_engine_clone.integrate_nyx_trace(
                    Self::convert_to_nyx_trace_intelligence(&intelligence)
                ).await {
                    eprintln!("Failed to integrate intelligence into EEI engine: {:?}", e);
                }
            }
        });

        Self {
            feed_processors: Arc::new(RwLock::new(HashMap::new())),
            social_intel_collectors: Arc::new(RwLock::new(HashMap::new())),
            gov_advisory_processors: Arc::new(RwLock::new(HashMap::new())),
            vuln_db_processors: Arc::new(RwLock::new(HashMap::new())),
            darkweb_monitors: Arc::new(RwLock::new(HashMap::new())),
            intel_analyzer: Arc::new(IntelligenceAnalyzer::new()),
            eei_engine,
            intelligence_channel: intelligence_tx,
            ingestion_config: Arc::new(RwLock::new(IngestionConfig::default())),
        }
    }

    /// Start all CTI ingestion processes
    pub async fn start_ingestion(&self) -> Result<(), CtiError> {
        // Start RSS/Atom feed processors
        self.start_feed_processors().await?;

        // Start social media intelligence collectors
        self.start_social_collectors().await?;

        // Start government advisory processors
        self.start_gov_processors().await?;

        // Start vulnerability database processors
        self.start_vuln_processors().await?;

        // Start dark web monitors (if configured)
        self.start_darkweb_monitors().await?;

        println!("🌍 Open CTI ingestion engine started - monitoring global threat landscape");
        Ok(())
    }

    /// Add new RSS/Atom feed source
    pub async fn add_feed_source(&self, name: String, source: FeedSource) -> Result<(), CtiError> {
        let mut config = self.ingestion_config.write().await;
        config.feed_sources.insert(name.clone(), source.clone());

        // Create and start feed processor
        let processor = FeedProcessor::new(name.clone(), source);
        let mut processors = self.feed_processors.write().await;
        processors.insert(name.clone(), processor);

        // Start processing in background
        self.spawn_feed_processor(name).await?;

        Ok(())
    }

    /// Process and analyze incoming intelligence
    pub async fn process_intelligence(&self, raw_intel: RawIntelligence) -> Result<ProcessedIntelligence, CtiError> {
        // Analyze and structure the raw intelligence
        let processed = self.intel_analyzer.analyze(raw_intel).await?;

        // Send to EEI engine via pub/sub channel
        if let Err(e) = self.intelligence_channel.send(processed.clone()) {
            eprintln!("Failed to send intelligence to EEI engine: {:?}", e);
        }

        Ok(processed)
    }

    /// Get real-time threat landscape summary
    pub async fn get_threat_landscape(&self) -> ThreatLandscapeSummary {
        // Aggregate intelligence from all sources
        ThreatLandscapeSummary {
            active_campaigns: self.count_active_campaigns().await,
            new_vulnerabilities_24h: self.count_new_vulnerabilities().await,
            threat_level: self.assess_global_threat_level().await,
            trending_attacks: self.get_trending_attacks().await,
            geographic_hotspots: self.identify_geographic_hotspots().await,
            sector_targeting: self.analyze_sector_targeting().await,
        }
    }

    /// Configure specific intelligence sources
    pub async fn configure_sources(&self, config: IngestionConfig) -> Result<(), CtiError> {
        let mut current_config = self.ingestion_config.write().await;
        *current_config = config;
        Ok(())
    }

    // Helper method to convert ProcessedIntelligence to NyxTraceIntelligence
    fn convert_to_nyx_trace_intelligence(intelligence: &ProcessedIntelligence) -> NyxTraceIntelligence {
        // This would map the processed intelligence to the format expected by EEI engine
        NyxTraceIntelligence {}
    }

    // Private implementation methods
    async fn start_feed_processors(&self) -> Result<(), CtiError> {
        // Implementation for starting RSS/Atom processors
        Ok(())
    }

    async fn start_social_collectors(&self) -> Result<(), CtiError> {
        // Implementation for starting social media collectors
        Ok(())
    }

    async fn start_gov_processors(&self) -> Result<(), CtiError> {
        // Implementation for starting government advisory processors
        Ok(())
    }

    async fn start_vuln_processors(&self) -> Result<(), CtiError> {
        // Implementation for starting vulnerability database processors
        Ok(())
    }

    async fn start_darkweb_monitors(&self) -> Result<(), CtiError> {
        // Implementation for starting dark web monitors
        Ok(())
    }

    async fn spawn_feed_processor(&self, _name: String) -> Result<(), CtiError> {
        // Implementation for spawning individual feed processor
        Ok(())
    }

    async fn count_active_campaigns(&self) -> u64 { 0 }
    async fn count_new_vulnerabilities(&self) -> u64 { 0 }
    async fn assess_global_threat_level(&self) -> String { "Medium".to_string() }
    async fn get_trending_attacks(&self) -> Vec<String> { vec![] }
    async fn identify_geographic_hotspots(&self) -> Vec<String> { vec![] }
    async fn analyze_sector_targeting(&self) -> HashMap<String, u64> { HashMap::new() }
}

/// Default CTI sources configuration
impl Default for IngestionConfig {
    fn default() -> Self {
        let mut feed_sources = HashMap::new();

        // Add default security news feeds
        feed_sources.insert("krebs-security".to_string(), FeedSource {
            url: "https://krebsonsecurity.com/feed/".to_string(),
            source_type: FeedSourceType::SecurityNews,
            credibility_score: 0.95,
            update_interval_minutes: 30,
            keywords: vec!["breach".to_string(), "malware".to_string(), "ransomware".to_string()],
            language: "en".to_string(),
            region: Some("US".to_string()),
        });

        feed_sources.insert("threat-post".to_string(), FeedSource {
            url: "https://threatpost.com/feed/".to_string(),
            source_type: FeedSourceType::ThreatIntelligence,
            credibility_score: 0.90,
            update_interval_minutes: 30,
            keywords: vec!["apt".to_string(), "zero-day".to_string(), "exploit".to_string()],
            language: "en".to_string(),
            region: None,
        });

        let mut government_sources = HashMap::new();
        government_sources.insert("cisa-alerts".to_string(), GovernmentSource {
            agency: GovernmentAgency::CISA,
            api_endpoint: "https://api.cisa.gov/alerts".to_string(),
            classification_levels: vec!["PUBLIC".to_string()],
            sectors: vec!["ALL".to_string()],
            auto_download: true,
        });

        let mut vulnerability_sources = HashMap::new();
        vulnerability_sources.insert("nvd".to_string(), VulnerabilitySource {
            database: VulnerabilityDatabase::NVD,
            api_endpoint: "https://services.nvd.nist.gov/rest/json/cves/2.0".to_string(),
            cvss_threshold: 7.0,
            product_filters: vec![],
            vendor_filters: vec![],
            auto_correlation: true,
        });

        Self {
            feed_sources,
            social_sources: HashMap::new(),
            government_sources,
            vulnerability_sources,
            darkweb_sources: HashMap::new(),
            processing_config: ProcessingConfig {
                max_concurrent_processors: 10,
                intelligence_retention_days: 365,
                duplicate_detection_window_hours: 24,
                threat_scoring_model: "ctas_v1".to_string(),
                auto_enrichment: true,
                real_time_alerting: true,
            },
        }
    }
}

// Supporting types and structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawIntelligence {
    pub source: String,
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatLandscapeSummary {
    pub active_campaigns: u64,
    pub new_vulnerabilities_24h: u64,
    pub threat_level: String,
    pub trending_attacks: Vec<String>,
    pub geographic_hotspots: Vec<String>,
    pub sector_targeting: HashMap<String, u64>,
}

// Placeholder processor types
#[derive(Debug)] pub struct FeedProcessor {}
#[derive(Debug)] pub struct SocialIntelCollector {}
#[derive(Debug)] pub struct GovAdvisoryProcessor {}
#[derive(Debug)] pub struct VulnDbProcessor {}
#[derive(Debug)] pub struct DarkwebMonitor {}
#[derive(Debug)] pub struct IntelligenceAnalyzer {}

impl FeedProcessor {
    fn new(_name: String, _source: FeedSource) -> Self { Self {} }
}

impl IntelligenceAnalyzer {
    fn new() -> Self { Self {} }
    async fn analyze(&self, _raw: RawIntelligence) -> Result<ProcessedIntelligence, CtiError> {
        Ok(ProcessedIntelligence {
            intelligence_id: Uuid::new_v4(),
            source_type: IntelligenceSourceType::OpenSource,
            source_name: "test".to_string(),
            raw_content: "test".to_string(),
            processed_content: IntelligenceContent {
                title: "test".to_string(),
                summary: "test".to_string(),
                threat_type: ThreatType::Malware,
                attack_vectors: vec![],
                affected_technologies: vec![],
                mitigation_strategies: vec![],
                related_cves: vec![],
                related_campaigns: vec![],
                timeline: None,
            },
            threat_indicators: vec![],
            attribution: None,
            relevance_score: 0.5,
            confidence_score: 0.5,
            urgency_level: UrgencyLevel::Medium,
            geographic_relevance: vec![],
            sector_relevance: vec![],
            timestamp: Utc::now(),
            expiration: None,
        })
    }
}

// Error handling
#[derive(Debug)]
pub enum CtiError {
    IngestionError(String),
    ProcessingError(String),
    ConfigurationError(String),
    NetworkError(String),
}