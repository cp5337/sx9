//! CTAS 7.0 Tactical Foundation
//! 
//! Steve Jobs-level tactical computing foundation bridging TypeScript frontend,
//! SwiftUI native interface, and Universal Cognigraph mathematics.

pub mod cognigraph;
pub mod cdn_bridge;
pub mod hash_missions;
pub mod swift_bridge;
pub mod haptic_physics;

// Re-export core interfaces for tactical operations
pub use cognigraph::*;
pub use cdn_bridge::*;
pub use hash_missions::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Universal tactical operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub execution_time_ms: f64,
    pub mission_hash: Option<String>,
}

/// Tactical operation error types
#[derive(Debug, thiserror::Error)]
pub enum TacticalError {
    #[error("CDN bridge error: {0}")]
    CdnBridge(String),
    
    #[error("Hash mission execution failed: {0}")]
    HashMission(String),
    
    #[error("Cognigraph validation failed: {0}")]
    CognigraphValidation(String),
    
    #[error("Haptic feedback error: {0}")]
    HapticFeedback(String),
    
    #[error("Swift bridge error: {0}")]
    SwiftBridge(String),
}

/// Domain-agnostic execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainContext {
    NationalSecurity {
        clearance_level: String,
        operation_name: String,
        rules_of_engagement: Vec<String>,
    },
    Healthcare {
        facility_id: String,
        patient_privacy_level: String,
        regulatory_compliance: Vec<String>,
    },
    Manufacturing {
        facility_id: String,
        safety_protocols: Vec<String>,
        quality_standards: Vec<String>,
    },
    Restaurant {
        location_id: String,
        health_regulations: Vec<String>,
        service_standards: Vec<String>,
    },
}

impl<T> TacticalResult<T> {
    pub fn success(data: T, execution_time_ms: f64) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            execution_time_ms,
            mission_hash: None,
        }
    }
    
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            execution_time_ms: 0.0,
            mission_hash: None,
        }
    }
    
    pub fn with_mission_hash(mut self, hash: String) -> Self {
        self.mission_hash = Some(hash);
        self
    }
}
// CTAS-7 Gold Disk Retrofit Integration
pub mod foundation_integration;
