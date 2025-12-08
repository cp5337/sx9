//! Swift bridge for iOS interface integration
//!
//! Enables SwiftUI tactical planning interface with iTunes-style mission orchestration
//! Provides hash-driven mission execution from native iOS applications

use serde::{Deserialize, Serialize};
use crate::{TacticalResult, TacticalError};

/// Swift bridge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwiftBridgeConfig {
    pub bridge_port: u16,
    pub security_token: String,
    pub haptic_feedback_enabled: bool,
    pub physics_validation_enabled: bool,
}

impl Default for SwiftBridgeConfig {
    fn default() -> Self {
        Self {
            bridge_port: 18101,
            security_token: "ctas7-swift-bridge-token".to_string(),
            haptic_feedback_enabled: true,
            physics_validation_enabled: true,
        }
    }
}

/// iTunes-style tactical tool representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalTool {
    pub tool_id: String,
    pub tool_name: String,
    pub tool_category: ToolCategory,
    pub execution_hash: String,
    pub attraction_physics: AttractionPhysics,
    pub haptic_properties: HapticProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToolCategory {
    Reconnaissance,
    Exploitation,
    PostExploitation,
    Persistence,
    Defense,
    Analysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttractionPhysics {
    pub base_attraction: f32,
    pub constraint_repulsion: f32,
    pub compatibility_bonus: f32,
    pub environmental_modifier: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HapticProperties {
    pub vibration_intensity: f32,
    pub vibration_pattern: String,
    pub attraction_feedback: bool,
    pub validation_feedback: bool,
}

/// Mission playlist (iTunes-style)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalPlaylist {
    pub playlist_id: String,
    pub playlist_name: String,
    pub tools: Vec<TacticalTool>,
    pub execution_sequence: Vec<String>,
    pub environmental_constraints: Vec<EnvironmentalConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalConstraint {
    pub constraint_type: ConstraintType,
    pub constraint_value: f64,
    pub validation_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    METOC { wind_speed: f32, weather_condition: String },
    Policy { policy_id: String, restriction_level: u8 },
    Resources { resource_type: String, available_amount: f64 },
    OrderOfBattle { friendly_forces: Vec<String>, threat_forces: Vec<String> },
}

/// Swift bridge service
pub struct SwiftBridge {
    config: SwiftBridgeConfig,
}

impl SwiftBridge {
    pub fn new(config: SwiftBridgeConfig) -> Self {
        Self { config }
    }

    /// Validate tool combination with physics
    pub async fn validate_tool_combination(
        &self,
        tools: &[TacticalTool],
        constraints: &[EnvironmentalConstraint],
    ) -> TacticalResult<ValidationResult> {
        let start = std::time::Instant::now();
        
        let mut total_attraction = 0.0f64;
        let mut constraint_violations = Vec::new();
        
        // Physics-based validation
        for tool in tools {
            let mut tool_attraction = tool.attraction_physics.base_attraction as f64;
            
            // Apply environmental constraints
            for constraint in constraints {
                match &constraint.constraint_type {
                    ConstraintType::METOC { wind_speed, .. } => {
                        if *wind_speed > 30.0 && tool.tool_category == ToolCategory::Reconnaissance {
                            tool_attraction += tool.attraction_physics.constraint_repulsion as f64;
                            constraint_violations.push(format!(
                                "High wind speed ({} knots) incompatible with {}",
                                wind_speed, tool.tool_name
                            ));
                        }
                    },
                    ConstraintType::Policy { restriction_level, .. } => {
                        if *restriction_level > 7 && matches!(tool.tool_category, ToolCategory::Exploitation) {
                            tool_attraction += tool.attraction_physics.constraint_repulsion as f64;
                            constraint_violations.push(format!(
                                "Policy restriction level {} blocks {}",
                                restriction_level, tool.tool_name
                            ));
                        }
                    },
                    _ => {} // Handle other constraint types
                }
            }
            
            total_attraction += tool_attraction;
        }
        
        let validation_result = ValidationResult {
            is_valid: constraint_violations.is_empty(),
            total_attraction_score: total_attraction,
            constraint_violations,
            haptic_feedback: if self.config.haptic_feedback_enabled {
                Some(generate_haptic_feedback(total_attraction))
            } else {
                None
            },
        };

        TacticalResult::success(
            validation_result,
            start.elapsed().as_millis() as f64
        )
    }

    /// Execute tactical playlist
    pub async fn execute_playlist(&self, playlist: &TacticalPlaylist) -> TacticalResult<serde_json::Value> {
        let start = std::time::Instant::now();
        
        // Generate execution hashes for the playlist
        let execution_hashes: Vec<String> = playlist.tools
            .iter()
            .map(|tool| tool.execution_hash.clone())
            .collect();
        
        TacticalResult::success(
            serde_json::json!({
                "playlist_id": playlist.playlist_id,
                "execution_hashes": execution_hashes,
                "status": "queued_for_execution"
            }),
            start.elapsed().as_millis() as f64
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub total_attraction_score: f64,
    pub constraint_violations: Vec<String>,
    pub haptic_feedback: Option<HapticFeedback>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HapticFeedback {
    pub feedback_type: HapticFeedbackType,
    pub intensity: f32,
    pub duration_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HapticFeedbackType {
    Attraction,
    Repulsion,
    Validation,
    Warning,
}

fn generate_haptic_feedback(attraction_score: f64) -> HapticFeedback {
    if attraction_score > 0.0 {
        HapticFeedback {
            feedback_type: HapticFeedbackType::Attraction,
            intensity: (attraction_score / 10.0).min(1.0) as f32,
            duration_ms: 200,
        }
    } else {
        HapticFeedback {
            feedback_type: HapticFeedbackType::Repulsion,
            intensity: ((-attraction_score) / 10.0).min(1.0) as f32,
            duration_ms: 300,
        }
    }
}