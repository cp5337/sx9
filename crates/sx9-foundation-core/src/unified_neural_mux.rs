//! Unified Neural Mux Integration for CTAS-7 Foundation Core
//!
//! Extends the neural_mux module with unified operations across the foundation
//! Provides consolidated routing for multi-system neural operations

use crate::data::{DateTime, Deserialize, Serialize, Utc};
use crate::neural_mux::{NeuralMuxConfig, OperationRoute, Priority};
use std::collections::HashMap;

/// Unified Neural Mux for cross-system operations
#[derive(Debug)]
pub struct UnifiedNeuralMux {
    /// Individual neural mux configurations for different systems
    pub system_configs: HashMap<String, NeuralMuxConfig>,
    /// Global priority overrides
    pub global_priorities: HashMap<String, Priority>,
    /// Cross-system routing rules
    pub routing_rules: Vec<UnifiedRoutingRule>,
    /// Last synchronized timestamp
    pub last_sync: DateTime<Utc>,
}

/// Cross-system routing rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedRoutingRule {
    /// Source system identifier
    pub source_system: String,
    /// Target system identifier
    pub target_system: String,
    /// Operation type pattern
    pub operation_pattern: String,
    /// Priority mapping
    pub priority_mapping: Priority,
    /// Enabled flag
    pub enabled: bool,
}

impl UnifiedNeuralMux {
    /// Create a new unified neural mux
    pub fn new() -> Self {
        Self {
            system_configs: HashMap::new(),
            global_priorities: HashMap::new(),
            routing_rules: Vec::new(),
            last_sync: Utc::now(),
        }
    }

    /// Add a system configuration
    pub fn add_system_config(&mut self, system_id: String, config: NeuralMuxConfig) {
        self.system_configs.insert(system_id, config);
        self.last_sync = Utc::now();
    }

    /// Route operation across systems
    pub async fn route_unified_operation(
        &self,
        operation: &str,
        source_system: &str,
        priority: Priority,
    ) -> Result<Vec<OperationRoute>, String> {
        let mut routes = Vec::new();

        // Apply global priority overrides
        let effective_priority = self
            .global_priorities
            .get(operation)
            .copied()
            .unwrap_or(priority);

        // Find matching routing rules
        for rule in &self.routing_rules {
            if rule.enabled
                && rule.source_system == source_system
                && operation.contains(&rule.operation_pattern)
            {
                // Create route for target system
                if let Some(_target_config) = self.system_configs.get(&rule.target_system) {
                    let route = OperationRoute {
                        unicode_range: (0xE000, 0xE0FF), // Default range for unified operations
                        target_processor: rule.target_system.clone(),
                        priority: rule.priority_mapping,
                        transport_profile: crate::neural_mux::TransportProfile::Internal,
                        context_awareness: true,
                    };
                    routes.push(route);
                }
            }
        }

        Ok(routes)
    }

    /// Synchronize all system muxes
    pub async fn synchronize_systems(&mut self) -> Result<(), String> {
        // Basic synchronization - update timestamp
        self.last_sync = Utc::now();

        // TODO: Implement cross-system state synchronization
        // This would involve coordinating priorities and routes across all systems

        Ok(())
    }

    /// Add global priority override
    pub fn set_global_priority(&mut self, operation: String, priority: Priority) {
        self.global_priorities.insert(operation, priority);
        self.last_sync = Utc::now();
    }

    /// Add unified routing rule
    pub fn add_routing_rule(&mut self, rule: UnifiedRoutingRule) {
        self.routing_rules.push(rule);
        self.last_sync = Utc::now();
    }
}

impl Default for UnifiedNeuralMux {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a basic cross-system routing rule
pub fn create_routing_rule(
    source: &str,
    target: &str,
    pattern: &str,
    priority: Priority,
) -> UnifiedRoutingRule {
    UnifiedRoutingRule {
        source_system: source.to_string(),
        target_system: target.to_string(),
        operation_pattern: pattern.to_string(),
        priority_mapping: priority,
        enabled: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_unified_neural_mux_creation() {
        let mux = UnifiedNeuralMux::new();
        assert!(mux.system_configs.is_empty());
        assert!(mux.global_priorities.is_empty());
        assert!(mux.routing_rules.is_empty());
    }

    #[tokio::test]
    async fn test_routing_rule_creation() {
        let rule = create_routing_rule("system1", "system2", "operation_*", Priority::High);
        assert_eq!(rule.source_system, "system1");
        assert_eq!(rule.target_system, "system2");
        assert_eq!(rule.operation_pattern, "operation_*");
        assert_eq!(rule.priority_mapping, Priority::High);
        assert!(rule.enabled);
    }
}
