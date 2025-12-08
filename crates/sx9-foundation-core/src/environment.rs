//! XSD Environment Annotation Types
//! Core types for contextual operational awareness

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OSILayer {
    Physical,    // L1 - Infrastructure monitoring
    DataLink,    // L2 - Switch/VLAN awareness
    Network,     // L3 - Routing, IP intelligence
    Transport,   // L4 - TCP/UDP port context
    Session,     // L5 - Connection state management
    Presentation,// L6 - Encryption, compression context
    Application, // L7 - Full application awareness
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OperationalContext {
    Utility,        // Basic infrastructure services
    DevOps,         // CI/CD, deployment, monitoring
    DeceptionOps,   // Honeypots, threat hunting
    Communication,  // Email, chat, collaboration
    LLMOps,         // AI/ML services, model serving
    Research,       // Data analysis, experimentation
    Production,     // Live customer-facing services
    Staging,        // Pre-production testing
    Emergency,      // Incident response mode
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XSDEnvironmentAnnotation {
    pub osi_layer: OSILayer,
    pub operational_context: OperationalContext,
    pub service_classification: ServiceClassification,
    pub security_posture: SecurityPosture,
    pub intelligence_level: IntelligenceLevel,
    pub automation_capability: AutomationCapability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceClassification {
    pub primary_function: String,
    pub data_sensitivity: DataSensitivity,
    pub availability_requirement: AvailabilityRequirement,
    pub compliance_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSensitivity {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AvailabilityRequirement {
    BestEffort,     // Dev, testing
    HighAvailability, // 99.9%
    MissionCritical,  // 99.99%
    AlwaysOn,        // 99.999%
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityPosture {
    Open,           // Public facing, minimal security
    Authenticated,  // Requires auth but standard security
    Hardened,       // Enhanced security measures
    Paranoid,       // Maximum security, assume breach
    Deception,      // Honeypot mode - log everything
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntelligenceLevel {
    Basic,          // Standard logging
    Enhanced,       // Pattern analysis
    Advanced,       // ML-based analysis
    Consciousness,  // XSD-level awareness and adaptation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationCapability {
    Manual,         // Human intervention required
    Scripted,       // Predefined automation
    Adaptive,       // Context-aware automation
    Autonomous,     // Self-managing with XSD consciousness
}

impl XSDEnvironmentAnnotation {
    /// Create a new environment annotation with sensible defaults
    pub fn new(context: OperationalContext, layer: OSILayer) -> Self {
        Self {
            osi_layer: layer,
            operational_context: context,
            service_classification: ServiceClassification::default(),
            security_posture: SecurityPosture::Authenticated,
            intelligence_level: IntelligenceLevel::Basic,
            automation_capability: AutomationCapability::Scripted,
        }
    }

    /// Create a deception ops annotation
    pub fn deception_ops() -> Self {
        Self {
            osi_layer: OSILayer::Application,
            operational_context: OperationalContext::DeceptionOps,
            service_classification: ServiceClassification {
                primary_function: "Honeypot".to_string(),
                data_sensitivity: DataSensitivity::Public,
                availability_requirement: AvailabilityRequirement::BestEffort,
                compliance_requirements: vec![],
            },
            security_posture: SecurityPosture::Deception,
            intelligence_level: IntelligenceLevel::Consciousness,
            automation_capability: AutomationCapability::Autonomous,
        }
    }

    /// Create an LLM ops annotation
    pub fn llm_ops() -> Self {
        Self {
            osi_layer: OSILayer::Application,
            operational_context: OperationalContext::LLMOps,
            service_classification: ServiceClassification {
                primary_function: "AI Model Serving".to_string(),
                data_sensitivity: DataSensitivity::Internal,
                availability_requirement: AvailabilityRequirement::HighAvailability,
                compliance_requirements: vec!["AI_ETHICS".to_string()],
            },
            security_posture: SecurityPosture::Hardened,
            intelligence_level: IntelligenceLevel::Advanced,
            automation_capability: AutomationCapability::Adaptive,
        }
    }

    /// Create a dev ops annotation
    pub fn dev_ops() -> Self {
        Self {
            osi_layer: OSILayer::Application,
            operational_context: OperationalContext::DevOps,
            service_classification: ServiceClassification {
                primary_function: "CI/CD Pipeline".to_string(),
                data_sensitivity: DataSensitivity::Internal,
                availability_requirement: AvailabilityRequirement::HighAvailability,
                compliance_requirements: vec!["SOC2".to_string()],
            },
            security_posture: SecurityPosture::Authenticated,
            intelligence_level: IntelligenceLevel::Enhanced,
            automation_capability: AutomationCapability::Adaptive,
        }
    }
}

impl Default for ServiceClassification {
    fn default() -> Self {
        Self {
            primary_function: "Generic Service".to_string(),
            data_sensitivity: DataSensitivity::Internal,
            availability_requirement: AvailabilityRequirement::BestEffort,
            compliance_requirements: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_annotation_creation() {
        let annotation = XSDEnvironmentAnnotation::new(
            OperationalContext::DevOps,
            OSILayer::Application
        );

        assert!(matches!(annotation.operational_context, OperationalContext::DevOps));
        assert!(matches!(annotation.osi_layer, OSILayer::Application));
    }

    #[test]
    fn test_deception_ops_preset() {
        let annotation = XSDEnvironmentAnnotation::deception_ops();

        assert!(matches!(annotation.operational_context, OperationalContext::DeceptionOps));
        assert!(matches!(annotation.security_posture, SecurityPosture::Deception));
        assert!(matches!(annotation.intelligence_level, IntelligenceLevel::Consciousness));
    }

    #[test]
    fn test_llm_ops_preset() {
        let annotation = XSDEnvironmentAnnotation::llm_ops();

        assert!(matches!(annotation.operational_context, OperationalContext::LLMOps));
        assert!(matches!(annotation.security_posture, SecurityPosture::Hardened));
        assert_eq!(annotation.service_classification.primary_function, "AI Model Serving");
    }
}
