//! Universal Symbolic Information Message (USIM)
//!
//! CTAS-7.3.1 Canonical Implementation
//! Ground Truth Data Structure for the Foundation
//!
//! Compliant with RFC-9001 (Trivariate Hashing)

use crate::data::{DateTime, Deserialize, Serialize, Utc};
use crate::trivariate_hash_v731::{ContextFrame, ExecEnv, ExecState, TrivariateHashEngineV731};
use std::collections::HashMap;

/// Universal Symbolic Information Message (USIM)
/// The fundamental atom of information in the CTAS-7 ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalSymbolicInformationMessage {
    /// Canonical Trivariate Hash (`SCH_CUID_UUID`)
    pub trivariate_hash: String,

    /// LISP Operator (S-Expression)
    pub lisp_operator: String,

    /// UTF-8 Compressed Symbol
    pub utf8_symbol: String,

    /// Contextual Metadata
    pub context: Context,

    /// Motion State
    pub motion_state: MotionState,

    /// MARC Record (Bibliographic Control)
    pub marc_record: HashMap<String, String>,

    /// PGP/GPG Public Key Block
    pub pgp_key: String,

    /// Biometric Hash (Base96)
    pub biometric_hash: String,

    /// Integrity Hash (Trivariate Base96, formerly Blake3)
    pub integrity_hash: String,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Raw Payload
    pub payload: String,
}

impl UniversalSymbolicInformationMessage {
    #[must_use]
    pub fn get_sch(&self) -> String {
        // triv:SCH_CUID_UUID -> split by _, index 0, remove "triv:"
        if let Some(rest) = self.trivariate_hash.strip_prefix("triv:") {
            let parts: Vec<&str> = rest.split('_').collect();
            if !parts.is_empty() {
                return parts[0].to_string();
            }
        }
        "UNKNOWN".to_string()
    }

    #[must_use]
    pub fn get_cuid(&self) -> String {
        if let Some(rest) = self.trivariate_hash.strip_prefix("triv:") {
            let parts: Vec<&str> = rest.split('_').collect();
            if parts.len() > 1 {
                return parts[1].to_string();
            }
        }
        "UNKNOWN".to_string()
    }

    #[must_use]
    pub fn get_uuid(&self) -> String {
        if let Some(rest) = self.trivariate_hash.strip_prefix("triv:") {
            let parts: Vec<&str> = rest.split('_').collect();
            if parts.len() > 2 {
                return parts[2].to_string();
            }
        }
        "UNKNOWN".to_string()
    }

    #[must_use]
    pub fn verify_integrity(&self) -> bool {
        // Re-calculate integrity hash
        let calc = Self::calculate_integrity_hash(
            &self.payload,
            &self.trivariate_hash,
            &self.lisp_operator,
        );
        calc == self.integrity_hash
    }

    #[must_use]
    pub fn calculate_integrity_hash(payload: &str, trivariate: &str, lisp: &str) -> String {
        let engine = TrivariateHashEngineV731::new();
        let mut data = Vec::new();
        data.extend_from_slice(payload.as_bytes());
        data.extend_from_slice(trivariate.as_bytes());
        data.extend_from_slice(lisp.as_bytes());
        engine.generate_hash_from_bytes(&data)
    }

    #[must_use]
    pub fn to_toml(&self) -> String {
        toml::to_string(self).unwrap_or_else(|_| "error: toml serialization failed".to_string())
    }

    #[must_use]
    pub fn generate_position_tails(&self) -> HashMap<String, String> {
        let mut tails = HashMap::new();
        // Generate pseudo-tails from CUID
        let cuid = self.get_cuid();
        if cuid.len() >= 16 {
            tails.insert("T1".to_string(), cuid[0..4].to_string());
            tails.insert("E1".to_string(), cuid[4..7].to_string());
            tails.insert("A1".to_string(), cuid[7..9].to_string());
        }
        tails
    }
}

/// USIM Builder for fluent construction
#[derive(Default)]
pub struct USIMBuilder {
    lisp_operator: Option<String>,
    utf8_symbol: Option<String>,
    payload: Option<String>,
    context: Option<Context>,
    motion_state: Option<MotionState>,
    marc_record: Option<HashMap<String, String>>,
    pgp_key: Option<String>,
    biometric: Option<String>,
}

impl USIMBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn lisp_operator(mut self, op: String) -> Self {
        self.lisp_operator = Some(op);
        self
    }

    #[must_use]
    pub fn utf8_symbol(mut self, sym: String) -> Self {
        self.utf8_symbol = Some(sym);
        self
    }

    #[must_use]
    pub fn payload(mut self, p: String) -> Self {
        self.payload = Some(p);
        self
    }

    #[must_use]
    pub fn context(mut self, ctx: Context) -> Self {
        self.context = Some(ctx);
        self
    }

    #[must_use]
    pub fn motion_state(mut self, state: MotionState) -> Self {
        self.motion_state = Some(state);
        self
    }

    #[must_use]
    pub fn marc_record(mut self, record: HashMap<String, String>) -> Self {
        self.marc_record = Some(record);
        self
    }

    #[must_use]
    pub fn pgp_key(mut self, key: String) -> Self {
        self.pgp_key = Some(key);
        self
    }

    #[must_use]
    pub fn biometric(mut self, hash: String) -> Self {
        self.biometric = Some(hash);
        self
    }

    pub fn build(self) -> Result<UniversalSymbolicInformationMessage, String> {
        let engine = TrivariateHashEngineV731::new();
        let ctx_frame = ContextFrame::new(ExecEnv::Native, 0, ExecState::Hot);

        let lisp = self.lisp_operator.ok_or("Missing lisp_operator")?;
        let symbol = self.utf8_symbol.ok_or("Missing utf8_symbol")?;
        let payload = self.payload.ok_or("Missing payload")?;
        let context = self.context.ok_or("Missing context")?;

        // Generate Trivariate Hash
        let triv_hash = engine
            .generate_trivariate(&payload, "USIM", "Core", "Gen", &ctx_frame)
            .to_canonical_format();

        // Calculate Integrity Hash (Replacing Blake3)
        let integrity = UniversalSymbolicInformationMessage::calculate_integrity_hash(
            &payload, &triv_hash, &lisp,
        );

        Ok(UniversalSymbolicInformationMessage {
            trivariate_hash: triv_hash,
            lisp_operator: lisp,
            utf8_symbol: symbol,
            context,
            motion_state: self.motion_state.unwrap_or(MotionState::Static),
            marc_record: self.marc_record.unwrap_or_default(),
            pgp_key: self.pgp_key.unwrap_or_default(),
            biometric_hash: self.biometric.unwrap_or_default(),
            integrity_hash: integrity,
            timestamp: Utc::now(),
            payload,
        })
    }
}

/// Contextual environment type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Context {
    Geospatial {
        lat: f64,
        lon: f64,
        region_code: String,
    },
    Network {
        ip_address: Option<String>,
        mac_address: Option<String>,
        hostname: Option<String>,
        grid_type: NetworkGridType,
    },
    Spectrum {
        frequency_mhz: f64,
        bandwidth_khz: f64,
        signal_strength_dbm: f64,
    },
    Incident {
        system_name: String,
        incident_id: String,
    },
    Logical {
        system_id: String,
        relative_position: String,
    },
    Cyber {
        vector: String,
        signature: String,
    },
}

/// Network Grid Type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NetworkGridType {
    LocalAreaNetwork,
    WideAreaNetwork,
    CloudOverlay,
    TacticalEdge,
    AirGapped,
}

/// Motion State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MotionState {
    Static,
    Moving { velocity: f64 },
    Accelerating { acceleration: f64 },
    Rotational { angular_velocity: f64 },
}
