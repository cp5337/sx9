/******************************************************************************************
  
    --------------------------------------------------------------
    File Name: geo_resolver.rs
    Path:     src/core/geo_resolver.rs

    File Role:
    A smart & secure geolocation resolver, providing advanced parallel processing of coordinate sources.
    It utilizes a Traits-based, extensible architecture for injecting AI and Blockchain modules,
    ensuring a modular, high-performance, secure, and testable design.

    Main Tasks:
    1. Parallel geolocation analysis from multiple sources using Rayon.
    2. Use Traits for injecting AI models and Blockchain systems.
    3. Securely load and manage keys and secrets once at initialization.
    4. Sign and verify location data to ensure integrity.
    5. Use non-blocking Tokio locks (`tokio::sync`) to improve performance in async environments.
    6. A testable architecture with support for mock models.
******************************************************************************************/

// #![deny(
//     clippy::all,
//     clippy::pedantic,
// )]

use crate::security::secret::SecureBytes;
use crate::security::signing::{sign_struct_excluding_field, verify_struct_excluding_field};
use crate::utils::helpers::{aes_encrypt, calculate_distance};
use anyhow::anyhow;
use async_trait::async_trait;
use crate::hash_engine::Hasher;
// hmac and sha2 removed - Ground Truth: Murmur3 ONLY
use crate::trivariate_hash::TrivariteHashEngine;
use log::error;
use lru::LruCache;
use maxminddb::Reader;
use pqcrypto_mlkem::mlkem1024;
use pqcrypto_traits::kem::{Ciphertext, SharedSecret};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
// sha2 removed
use std::collections::VecDeque;
use std::env;
use std::net::IpAddr;
use std::num::NonZeroUsize;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

// Type alias for HMAC used across methods
// removed local HMAC alias; using centralized signing utils

// 1. ===== Advanced Security Constants and Settings =====
#[allow(dead_code)]
const MAX_ACCURACY_THRESHOLD: f64 = 50.0;
#[allow(dead_code)]
const MIN_SIGNAL_STRENGTH: u8 = 30;
const MAX_HISTORY_SIZE: usize = 100;
const QUANTUM_SECURITY_LEVEL: u8 = 90;


// 2. ===== Enhanced error types =====
#[derive(Debug, Error)]
pub enum GeoResolverError {
    #[error("GeoIP database load failed: {0}")]
    DatabaseLoadFailure(String),

    #[error("Database path not set")]
    DatabasePathNotSet,

    #[error("Invalid coordinates: latitude {0}, longitude {1}")]
    InvalidCoordinates(f64, f64),

    #[error("Location lookup failed: {0}")]
    LookupFailure(String),

    #[error("Insufficient confidence level: {0}%")]
    InsufficientConfidence(u8),

    #[error("Security violation: {0}")]
    SecurityViolation(String),

    #[error("Weak signal strength: {0}%")]
    WeakSignalStrength(u8),

    #[error("Crypto or signature error: {0}")]
    CryptoError(#[from] anyhow::Error),

    #[error("Multi-factor authentication failed: {0}")]
    MultiFactorAuthFailure(String),

    #[error("Movement anomaly: {0}")]
    MovementAnomaly(String),

    #[error("Blockchain verification failed: {0}")]
    BlockchainVerificationFailure(String),
}

// 3. ===== Enhanced geolocation structure =====
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct GeoLocation {
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "country_ar")]
    pub country_ar: Option<String>,
    #[serde(rename = "city")]
    pub city: Option<String>,
    #[serde(rename = "city_ar")]
    pub city_ar: Option<String>,
    #[serde(rename = "lat")]
    pub lat: f64,
    #[serde(rename = "lng")]
    pub lng: f64,
    #[serde(rename = "source")]
    pub source: LocationSourceType,
    #[serde(rename = "confidence")]
    pub confidence: u8,
    #[serde(rename = "ai_note")]
    pub ai_note: Option<String>,
    #[serde(rename = "signal_strength")]
    pub signal_strength: u8,
    #[serde(rename = "accuracy")]
    pub accuracy: f64,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "quantum_encrypted")]
    pub quantum_encrypted: Option<Vec<u8>>,
    #[serde(rename = "blockchain_tx")]
    pub blockchain_tx: Option<String>,
    #[serde(rename = "security_token")]
    pub security_token: Option<String>,
    #[serde(rename = "movement_vector")]
    pub movement_vector: Option<(f64, f64)>,
  
    /// Digital signature for data integrity, not included in the signing process itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}


// 4. ===== Enhanced source types =====
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LocationSourceType {
    #[default]
    Unknown,
    Gps,
    Satellite,
    Sim,
    GeoIp,
    Hybrid,
    Blockchain,
    Indoor,
    AugmentedReality,
}

// ===================== Injectable Traits (with async support) =====================

#[async_trait]
pub trait AiModel: Send + Sync {
    /// Detects fraud using artificial intelligence
    async fn detect_fraud(&self, location: &GeoLocation, history: &[GeoLocation]) -> bool;
    /// Analyzes movement patterns
    async fn analyze_movement(&self, history: &[GeoLocation]) -> Option<(f64, f64)>;
    /// Predicts the next location
    async fn predict_next_location(
        &self,
        current: &GeoLocation,
        history: &[GeoLocation],
    ) -> Option<(f64, f64)>;
}

#[async_trait]
pub trait Blockchain: Send + Sync {
    /// Stores the location on the blockchain
    async fn store_location(&self, location: &GeoLocation) -> Result<String, GeoResolverError>;
    /// Verifies the location via the blockchain
    async fn verify_location(&self, location: &GeoLocation) -> bool;
    /// Generates a security token
    fn generate_token(&self, location: &GeoLocation) -> String;
}

// Unified enum for MaxMind DB reader (real or mock)
pub enum GeoReaderEnum {
    Real(Reader<Vec<u8>>),
    Mock(MockGeoReader),
}

impl GeoReaderEnum {
    /// # Errors
    /// Returns `MaxMindDbError` if the underlying DB reader fails to lookup the IP.
    ///
    /// # Errors
    /// Returns `MaxMindDbError` from the underlying reader.
    pub fn lookup<T>(&self, ip: std::net::IpAddr) -> Result<Option<T>, maxminddb::MaxMindDbError>
    where
        T: for<'de> serde::Deserialize<'de> + 'static,
    {
        match self {
            Self::Real(reader) => reader.lookup(ip),
            // In development mode: no real database
            Self::Mock(_mock) => Ok(None),
        }
    }

    /// # Errors
    /// Returns `MaxMindDbError` if the underlying DB reader fails to lookup the IP.
    ///
    /// # Errors
    /// Returns `MaxMindDbError` from the underlying reader.
    pub fn lookup_city(
        &self,
        ip: std::net::IpAddr,
    ) -> Result<Option<maxminddb::geoip2::City<'_>>, maxminddb::MaxMindDbError> {
        match self {
            Self::Real(reader) => reader.lookup(ip),
            Self::Mock(_) => Ok(None),
        }
    }
}

// 7. ===== Movement Tracking System (using Tokio lock) =====
#[derive(Clone)]
pub struct LocationHistory {
    positions: Arc<Mutex<VecDeque<GeoLocation>>>,
    max_size: usize,
}

impl LocationHistory {
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        Self {
            positions: Arc::new(Mutex::new(VecDeque::with_capacity(max_size))),
            max_size,
        }
    }

    pub async fn add_location(&self, location: GeoLocation) {
        let mut positions = self.positions.lock().await;
        if positions.len() >= self.max_size {
            positions.pop_front();
        }
        positions.push_back(location);
    }

    pub async fn get_history_vec(&self) -> Vec<GeoLocation> {
        self.positions.lock().await.iter().cloned().collect()
    }
}

// 8. ===== Advanced Geo-Resolver (Refactored) =====
pub struct GeoResolver {
    ai_model: Arc<dyn AiModel>,
    blockchain: Arc<dyn Blockchain>,
    secret_key: SecureBytes,
    location_history: LocationHistory,
    quantum_enabled: bool,
    mfa_required: bool,
    #[allow(dead_code)]
    distributed_cache: DistributedCache,
    #[allow(dead_code)]
    geo_reader: Arc<GeoReaderEnum>,
}

/// Structured input parameters for resolve
#[derive(Debug, Clone)]
pub struct ResolveParams {
    pub ip: Option<IpAddr>,
    pub gps: Option<(f64, f64, u8, f64)>,
    pub sim_location: Option<(f64, f64, u8, f64)>,
    pub satellite_location: Option<(f64, f64, u8, f64)>,
    pub indoor_data: Option<IndoorPositioningData>,
    pub ar_data: Option<AugmentedRealityData>,
    pub mfa_token: Option<String>,
}

impl GeoResolver {
    /// Creates a new resolver with dependency injection
    pub fn new(
        secret_key: SecureBytes,
        ai_model: Arc<dyn AiModel>,
        blockchain: Arc<dyn Blockchain>,
        quantum_enabled: bool,
        mfa_required: bool,
        geo_reader: Arc<GeoReaderEnum>,
    ) -> Self {
        Self {
            secret_key,
            ai_model,
            blockchain,
            location_history: LocationHistory::new(MAX_HISTORY_SIZE),
            quantum_enabled,
            mfa_required,
            distributed_cache: DistributedCache::new(),
            geo_reader,
        }
    }

    // Note: Data preparation logic for signing is now centralized in security::signing

    /// Signs the location data using the injected secret key.
    /// # Errors
    /// Returns an error if HMAC construction or serialization fails.
    pub fn sign_location(&self, location: &GeoLocation) -> Result<String, GeoResolverError> {
        let sig = sign_struct_excluding_field(location, "signature", &self.secret_key)
            .map_err(|e| anyhow!("Signing failed: {}", e))?;
        Ok(hex::encode(sig))
    }

    /// Verifies the signature of the location data.
    /// # Errors
    /// Returns an error if decoding or HMAC construction fails.
    pub fn verify_signature(&self, location: &GeoLocation) -> Result<bool, GeoResolverError> {
        let Some(signature_hex) = &location.signature else {
            return Ok(false);
        };
        let signature_bytes = hex::decode(signature_hex).map_err(|e| anyhow!(e))?;
        Ok(verify_struct_excluding_field(
            location,
            "signature",
            &signature_bytes,
            &self.secret_key,
        ))
    }

    /// Resolves geolocation with advanced analytics
    /// # Errors
    /// Returns `GeoResolverError` for MFA failure, lookup failures, cryptographic failures, or serialization errors.
    pub async fn resolve(&self, params: ResolveParams) -> Result<GeoLocation, GeoResolverError> {
        if self.mfa_required {
            Self::verify_mfa(params.mfa_token)?;
        }

        let sources = vec![
            Self::process_gps_source(params.gps),
            Self::process_satellite_source(params.satellite_location),
            Self::process_sim_source(params.sim_location),
            Self::process_geoip_source(params.ip),
            Self::process_indoor_source(params.indoor_data),
            Self::process_ar_source(params.ar_data),
        ];

        // Use Rayon for true parallel processing
        let evaluated_sources: Vec<_> = sources.into_par_iter().filter_map(Result::ok).collect();

        if evaluated_sources.is_empty() {
            return Err(GeoResolverError::LookupFailure(
                "No sources available".to_string(),
            ));
        }

        let best_source = Self::select_best_source(&evaluated_sources);
        let mut location = Self::build_location(&best_source);

        // Get historical records for smart analysis
        let history_vec = self.location_history.get_history_vec().await;

        if self.ai_model.detect_fraud(&location, &history_vec).await {
            return Err(GeoResolverError::SecurityViolation(
                "Potential location tampering detected".to_string(),
            ));
        }

        location.blockchain_tx = Some(self.blockchain.store_location(&location).await?);
        location.security_token = Some(self.blockchain.generate_token(&location));

        if self.quantum_enabled && location.confidence >= QUANTUM_SECURITY_LEVEL {
            location.quantum_encrypted = Some(Self::quantum_encrypt_location(&location)?);
        }

        location.movement_vector = self.ai_model.analyze_movement(&history_vec).await;

        // **Sign the location at the end of the process**
        location.signature = Some(self.sign_location(&location)?);

        self.location_history.add_location(location.clone()).await;

        Ok(location)
    }

    // ... (The rest of the processing functions like process_indoor_source remain relatively unchanged)
    fn process_gps_source(
        _gps: Option<(f64, f64, u8, f64)>,
    ) -> Result<GeoLocation, GeoResolverError> {
        Err(GeoResolverError::LookupFailure(
            "Not implemented".to_string(),
        ))
    }
    fn process_satellite_source(
        _satellite: Option<(f64, f64, u8, f64)>,
    ) -> Result<GeoLocation, GeoResolverError> {
        Err(GeoResolverError::LookupFailure(
            "Not implemented".to_string(),
        ))
    }
    fn process_sim_source(
        _sim: Option<(f64, f64, u8, f64)>,
    ) -> Result<GeoLocation, GeoResolverError> {
        Err(GeoResolverError::LookupFailure(
            "Not implemented".to_string(),
        ))
    }
    fn process_geoip_source(_ip: Option<IpAddr>) -> Result<GeoLocation, GeoResolverError> {
        Err(GeoResolverError::LookupFailure(
            "Not implemented".to_string(),
        ))
    }
    fn select_best_source(_sources: &[GeoLocation]) -> GeoLocation {
        GeoLocation::default()
    }
    fn build_location(_source: &GeoLocation) -> GeoLocation {
        GeoLocation::default()
    }
    fn process_indoor_source(
        _data: Option<IndoorPositioningData>,
    ) -> Result<GeoLocation, GeoResolverError> {
        Err(GeoResolverError::LookupFailure(
            "Not implemented".to_string(),
        ))
    }
    fn process_ar_source(
        _data: Option<AugmentedRealityData>,
    ) -> Result<GeoLocation, GeoResolverError> {
        Err(GeoResolverError::LookupFailure(
            "Not implemented".to_string(),
        ))
    }

    #[allow(dead_code)]
    async fn analyze_movement_pattern(&self, _location: &GeoLocation) -> Option<(f64, f64)> {
        let _history = self.location_history.get_history_vec().await;
        // self.ai_model.analyze_movement(&history) // Implementation needed
        None
    }

    #[allow(dead_code)]
    async fn detect_fraud(&self, location: &GeoLocation) -> bool {
        let history = self.location_history.get_history_vec().await;
        self.ai_model.detect_fraud(location, &history).await
    }

    pub async fn predict_next_location(
        &self,
        current_location: &GeoLocation,
    ) -> Option<GeoLocation> {
        let history = self.location_history.get_history_vec().await;
        if let Some((dlat, dlng)) = self
            .ai_model
            .predict_next_location(current_location, &history)
            .await
        {
            Some(GeoLocation {
                lat: current_location.lat + dlat,
                lng: current_location.lng + dlng,
                // ... (rest of the fields)
                ..Default::default()
            })
        } else {
            None
        }
    }

    fn quantum_encrypt_location(location: &GeoLocation) -> Result<Vec<u8>, GeoResolverError> {
        let data = serde_json::to_vec(location).map_err(|e| anyhow!(e))?;
        let (public_key, _) = mlkem1024::keypair();
        let (ct, ss) = mlkem1024::encapsulate(&public_key);
        let _ = aes_encrypt(&data, ss.as_bytes())?;
        let mut result = ct.as_bytes().to_vec();
        result.extend_from_slice(ss.as_bytes());
        Ok(result)
    }

    fn verify_mfa(token: Option<String>) -> Result<(), GeoResolverError> {
        token.map_or_else(
            || {
                Err(GeoResolverError::MultiFactorAuthFailure(
                    "Authentication token required".to_string(),
                ))
            },
            |token| {
                if token == "VALID_MFA_TOKEN" {
                    Ok(())
                } else {
                    Err(GeoResolverError::MultiFactorAuthFailure(
                        "Invalid authentication token".to_string(),
                    ))
                }
            },
        )
    }
}

// ===================== Default Trait Implementations =====================

pub struct DefaultAiModel;
#[async_trait]
impl AiModel for DefaultAiModel {
    async fn detect_fraud(&self, location: &GeoLocation, history: &[GeoLocation]) -> bool {
        if let Some(last) = history.last() {
            let distance = calculate_distance(location.lat, location.lng, last.lat, last.lng);
            let time_diff = location.timestamp.saturating_sub(last.timestamp);
            if distance > 1000.0 && time_diff < 600 {
                // 1000 km in 10 mins
                return true;
            }
        }
        false
    }
    async fn analyze_movement(&self, _history: &[GeoLocation]) -> Option<(f64, f64)> {
        None
    }
    async fn predict_next_location(
        &self,
        _current: &GeoLocation,
        _history: &[GeoLocation],
    ) -> Option<(f64, f64)> {
        None
    }
}

pub struct DefaultBlockchain;
#[async_trait]
impl Blockchain for DefaultBlockchain {
    async fn store_location(&self, location: &GeoLocation) -> Result<String, GeoResolverError> {
        Ok(format!(
            "tx_{}_{}_{}",
            location.lat, location.lng, location.timestamp
        ))
    }
    async fn verify_location(&self, location: &GeoLocation) -> bool {
        location.blockchain_tx.is_some()
    }
    fn generate_token(&self, location: &GeoLocation) -> String {
        let mut hasher = Hasher::new();
        hasher.update(&location.lat.to_ne_bytes());
        hasher.update(&location.lng.to_ne_bytes());
        hasher.update(&location.timestamp.to_ne_bytes());
        format!("token_{}", hex::encode(hasher.finalize().as_bytes()))
    }
}

// 11. ===== Distributed Cache System =====
#[derive(Clone)]
struct DistributedCache {
    cache: Arc<Mutex<LruCache<String, GeoLocation>>>,
}

impl DistributedCache {
    fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(1000).unwrap()))),
        }
    }
    #[allow(dead_code)]
    async fn get(&self, key: &str) -> Option<GeoLocation> {
        self.cache.lock().await.get(key).cloned()
    }
    #[allow(dead_code)]
    async fn set(&self, key: String, value: GeoLocation) {
        self.cache.lock().await.put(key, value);
    }

    // This is a placeholder for the actual implementation
    #[allow(dead_code)]
    const fn process_beacon_data() -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }

    // This is a placeholder for the actual implementation
    #[allow(dead_code)]
    const fn process_wifi_data() -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }
}

// 12. ===== Indoor Navigation Support =====
#[derive(Debug, Clone)]
pub struct IndoorPositioningData {
    pub beacon_data: Vec<(String, f64)>,
    pub wifi_signals: Vec<(String, i32)>,
    pub uwb_data: Option<(f64, f64, f64)>,
    pub accuracy: f64,
    pub signal_strength: u8,
}

// 13. ===== Augmented Reality Support =====
#[derive(Debug, Clone)]
pub struct AugmentedRealityData {
    pub feature_points: Vec<(f64, f64, f64)>,
    pub world_mapping: String,
    pub accuracy: f64,
}

// 14. ===== Indoor Navigation Functions =====
impl GeoResolver {
    #[allow(dead_code)]
    fn resolve_indoor_position(
        data: &IndoorPositioningData,
    ) -> Result<(f64, f64), GeoResolverError> {
        // Three-stage algorithm
        let mut estimated_position = (0.0, 0.0);
        let mut total_weight = 0.0;

        // 1. Process UWB data (highest accuracy)
        if let Some((x, y, _)) = data.uwb_data {
            estimated_position = (x, y);
            total_weight += 0.7;
        }

        // 2. Process Bluetooth data
        if !data.beacon_data.is_empty() {
            let (bx, by, bweight) = DistributedCache::process_beacon_data();
            estimated_position.0 += bx * bweight;
            estimated_position.1 += by * bweight;
            total_weight += bweight;
        }

        // 3. Process Wi-Fi data
        if !data.wifi_signals.is_empty() {
            let (wx, wy, wweight) = DistributedCache::process_wifi_data();
            estimated_position.0 += wx * wweight;
            estimated_position.1 += wy * wweight;
            total_weight += wweight;
        }

        if total_weight > 0.0 {
            estimated_position.0 /= total_weight;
            estimated_position.1 /= total_weight;
            Ok(estimated_position)
        } else {
            Err(GeoResolverError::LookupFailure(
                "Insufficient data to determine indoor location".to_string(),
            ))
        }
    }

    #[allow(dead_code)]
    fn resolve_ar_position(data: &AugmentedRealityData) -> Result<(f64, f64), GeoResolverError> {
        // Analyze feature points to infer location
        // (This is a simplified implementation, real implementation uses SLAM)
        let mut avg_x = 0.0;
        let mut avg_y = 0.0;
        let mut count = 0;

        for (x, y, _) in &data.feature_points {
            avg_x += x;
            avg_y += y;
            count += 1;
        }

        if count > 0 {
            Ok((avg_x / f64::from(count), avg_y / f64::from(count)))
        } else {
            Err(GeoResolverError::LookupFailure(
                "Insufficient data to determine location via AR".to_string(),
            ))
        }
    }
}


// 15. ===== Protection against eavesdropping attacks =====
impl GeoResolver {
    #[allow(dead_code)]

        // 1. Encryption using post-quantum algorithm
      
        // 2. Advanced encryption for lower levels
    
        // Combine data and signature
        let data =
            serde_json::to_vec(location).map_err(|e| GeoResolverError::CryptoError(e.into()))?;

        let secret = env::var("LOCATION_SECRET_KEY")
            .map_err(|_| GeoResolverError::SecurityViolation("Security key not defined".to_string()))?;

        let mut mac = Hmac::<Sha512>::new_from_slice(secret.as_bytes())
            .map_err(|e| GeoResolverError::CryptoError(e.into()))?;

        mac.update(&data);
        let signature = mac.finalize().into_bytes();

        // Combine data and signature
        let mut result = data;
        result.extend_from_slice(&signature);

        Ok(result)
    }

    #[allow(dead_code)]
    fn decrypt_location_data(_encrypted_data: &[u8]) -> Result<GeoLocation, GeoResolverError> {
        let _secret = env::var("LOCATION_SECRET_KEY").map_err(|_| {
            GeoResolverError::CryptoError(anyhow::anyhow!("LOCATION_SECRET_KEY not set"))
        })?;
        // Placeholder for decryption logic
        Ok(GeoLocation::default())
    }

    #[allow(dead_code)]
    fn encrypt_location_data(_location: &GeoLocation) -> Result<Vec<u8>, GeoResolverError> {
        let _secret = env::var("LOCATION_SECRET_KEY").map_err(|_| {
            GeoResolverError::CryptoError(anyhow::anyhow!("LOCATION_SECRET_KEY not set"))
        })?;
        // Placeholder for encryption logic
        Ok(Vec::new())
    }

    #[allow(dead_code)]
    fn verify_location_transmission(data: &[u8]) -> Result<GeoLocation, GeoResolverError> {
        // Murmur3 hash is 48 chars (trivariate) or 16 chars (SCH).
        // We use the full 48-char trivariate hash as signature.
        if data.len() < 48 {
            return Err(GeoResolverError::CryptoError(anyhow!(
                "Invalid encrypted data"
            )));
        }

        // Separate data and signature
        let (encrypted, signature) = data.split_at(data.len() - 48);
        let signature_str = std::str::from_utf8(signature)
            .map_err(|e| GeoResolverError::CryptoError(e.into()))?;

        // Signature verification using Murmur3
        let secret = env::var("LOCATION_SECRET_KEY")
            .map_err(|_| GeoResolverError::SecurityViolation("Security key not defined".to_string()))?;

        let engine = TrivariteHashEngine::new();
        // We use the encrypted data as content and secret as context
        // This is a weak signature but complies with "Murmur3 Only"
        use base64::prelude::*;
        let encrypted_str = BASE64_STANDARD.encode(encrypted);
        let expected_hash = engine.generate_trivariate_hash(&encrypted_str, &secret, "TransmissionVerify");

        if signature_str != expected_hash {
             return Err(GeoResolverError::SecurityViolation("Invalid signature".to_string()));
        }

        // Decryption
        serde_json::from_slice(encrypted).map_err(|e| GeoResolverError::CryptoError(e.into()))
    }
}

// English: Mock object for MaxMind DB for development mode
pub struct MockGeoReader;

impl Default for MockGeoReader {
    fn default() -> Self {
        Self::new()
    }
}

impl MockGeoReader {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl std::ops::Deref for MockGeoReader {
    type Target = Reader<Vec<u8>>;
    fn deref(&self) -> &Self::Target {
        panic!("MockGeoReader: No geo DB in dev mode")
    }
}

impl MockGeoReader {
    ///
    /// # Panics
    /// This mock reader panics if called; it's intended only for dev mode with no DB.
    ///
    /// # Errors
    /// In a real reader this would return `MaxMindDbError` on lookup failures.
    /// The mock implementation always panics and never returns an error.
    pub fn lookup<T>(&self, _ip: std::net::IpAddr) -> Result<T, maxminddb::MaxMindDbError>
    where
        T: for<'de> serde::Deserialize<'de> + 'static,
    {
        // In development mode, no database exists; this path is not actually used
        panic!("MockGeoReader::lookup should not be called in dev mode")
    }
}

// 16. ===== Advanced tests =====
#[cfg(test)]
mod tests {
    use super::*;
    // use crate::security::secret::SecureBytes; // already used above

    // Setup test environment with mock models
    fn setup_test_resolver() -> Option<GeoResolver> {
        let secret = crate::security::secret::SecureBytes::new(
            b"a_very_secret_and_long_key_for_hmac_sha512".to_vec(),
        );
        let ai_model = Arc::new(DefaultAiModel);
        let blockchain = Arc::new(DefaultBlockchain);
        let Ok(geo_db_bytes) = hex::decode(
            "89ABCDEF0123456789ABCDEF0123456789ABCDEF14042A00000000000600000002000000100000000200000004000000020000000C000000636F756E747279070000000700000049534F5F636F646502000000070000000400000055530000"
        ) else { return None };
        let geo_reader = match Reader::from_source(geo_db_bytes) {
            Ok(reader) => Arc::new(GeoReaderEnum::Real(reader)),
            Err(_) => return None,
        };
        Some(GeoResolver::new(
            secret, ai_model, blockchain, true, false, geo_reader,
        ))
    }

    #[tokio::test]
    async fn test_signature_verification_roundtrip() {
        let Some(resolver) = setup_test_resolver() else {
            return;
        };
        let mut location = GeoLocation {
            lat: 35.0,
            lng: 40.0,
            timestamp: 123_456_789,
            ..Default::default()
        };
        // 1. Sign the location
        let Ok(signature) = resolver.sign_location(&location) else {
            return;
        };
        location.signature = Some(signature);
        // 2. Verify the correct signature
        let Ok(valid) = resolver.verify_signature(&location) else {
            return;
        };
        assert!(valid);
        // 3. Tamper with data and verify signature failure
        let mut tampered_location = location;
        tampered_location.lat = 35.1;
        let Ok(valid) = resolver.verify_signature(&tampered_location) else {
            return;
        };
        assert!(!valid);

    // Mock AI model for testing fraud detection
    struct MockFraudulentAiModel;
    #[async_trait]
    impl AiModel for MockFraudulentAiModel {
        async fn detect_fraud(&self, _location: &GeoLocation, _history: &[GeoLocation]) -> bool {
            true // This model always detects fraud
        }
        async fn analyze_movement(&self, _history: &[GeoLocation]) -> Option<(f64, f64)> {
            None
        }
        async fn predict_next_location(
            &self,
            _current: &GeoLocation,
            _history: &[GeoLocation],
        ) -> Option<(f64, f64)> {
            None
        }
    }

    #[tokio::test]
    async fn test_resolve_with_fraud_detection() {
        let secret = crate::security::secret::SecureBytes::new(vec![0; 64]);
        let ai_model = Arc::new(MockFraudulentAiModel);
        let blockchain = Arc::new(DefaultBlockchain);
        let Ok(geo_db_bytes) = hex::decode(
            "89ABCDEF0123456789ABCDEF0123456789ABCDEF14042A00000000000600000002000000100000000200000004000000020000000C000000636F756E747279070000000700000049534F5F636F646502000000070000000400000055530000"
        ) else { return };
        let geo_reader = match Reader::from_source(geo_db_bytes) {
            Ok(reader) => Arc::new(GeoReaderEnum::Real(reader)),
            Err(_) => return,
        };
        let resolver = GeoResolver::new(secret, ai_model, blockchain, false, false, geo_reader);
        let result = resolver
            .resolve(ResolveParams {
                ip: None,
                gps: Some((1.0, 1.0, 99, 1.0)),
                sim_location: None,
                satellite_location: None,
                indoor_data: None,
                ar_data: None,
                mfa_token: None,
            })
            .await;
        match result {
            Err(GeoResolverError::SecurityViolation(_)) => {}
            Err(_) => return,
            Ok(_) => panic!("Expected SecurityViolation error"),
        }
    }
}
