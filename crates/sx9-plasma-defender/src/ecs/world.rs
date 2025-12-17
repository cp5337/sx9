//! Plasma Defender ECS World
//!
//! Unified world interface for both Legion (hot-path) and apecs (cold-path)

use crate::ecs::apecs_layer::DefenderApecsWorld;
use crate::ecs::components::*;
use crate::ecs::legion_layer::DefenderLegionWorld;
use crate::ecs::systems::*;
use anyhow::Result;
use std::sync::Arc;

/// Unified Defender ECS World
///
/// Provides a single interface to both Legion (hot-path) and apecs (cold-path) layers.
/// - Hot operations go to Legion for deterministic batch processing
/// - Cold operations go to apecs for async I/O
pub struct DefenderWorld {
    /// Legion world for hot-path operations
    pub legion: Arc<DefenderLegionWorld>,
    /// apecs world for cold-path async operations
    pub apecs: Arc<DefenderApecsWorld>,
    /// Configuration
    config: DefenderWorldConfig,
}

/// Configuration for DefenderWorld
#[derive(Debug, Clone)]
pub struct DefenderWorldConfig {
    /// Enable Legion layer
    pub use_legion: bool,
    /// Enable apecs layer
    pub use_apecs: bool,
    /// Hot-path threshold (entities above this go to Legion)
    pub hot_threshold: u8,
    /// Auto-persist interval (ticks)
    pub persist_interval: u64,
}

impl Default for DefenderWorldConfig {
    fn default() -> Self {
        Self {
            use_legion: true,
            use_apecs: true,
            hot_threshold: 1, // speed_class 0 or 1 = hot
            persist_interval: 100,
        }
    }
}

impl DefenderWorld {
    /// Create new DefenderWorld with default config
    pub fn new() -> Self {
        Self::with_config(DefenderWorldConfig::default())
    }

    /// Create new DefenderWorld with custom config
    pub fn with_config(config: DefenderWorldConfig) -> Self {
        Self {
            legion: Arc::new(DefenderLegionWorld::new().expect("Failed to create Legion world")),
            apecs: Arc::new(DefenderApecsWorld::new().expect("Failed to create apecs world")),
            config,
        }
    }

    // =========================================================================
    // THREAT ENTITY OPERATIONS
    // =========================================================================

    /// Add threat entity (routes to Legion for hot-path)
    pub async fn add_threat(&self, threat: ThreatEntityComponent) -> Result<u64> {
        // All new threats go to Legion first
        let entity_id = self.legion.add_threat_entity(threat.clone()).await?;

        // If cold-path, also persist to apecs
        if threat.speed_class > self.config.hot_threshold {
            self.apecs.persist_threat(entity_id, threat).await?;
        }

        Ok(entity_id)
    }

    /// Add threat from OSSEC alert
    pub async fn add_threat_from_ossec(&self, alert: OssecAlertComponent) -> Result<u64> {
        let entity_id = self.legion.add_threat_from_ossec(alert.clone()).await?;

        // Persist alert to apecs for async correlation
        self.apecs.persist_ossec_alert(entity_id, alert).await?;

        Ok(entity_id)
    }

    /// Add threat from tool output
    pub async fn add_threat_from_tool(&self, tool: ToolOutputComponent) -> Result<u64> {
        let entity_id = self.legion.add_threat_from_tool(tool.clone()).await?;

        // Persist tool output to apecs for async processing
        self.apecs.persist_tool_output(entity_id, tool).await?;

        Ok(entity_id)
    }

    /// Get threat entity
    pub async fn get_threat(&self, entity_id: u64) -> Result<Option<ThreatEntityComponent>> {
        // Try Legion first (hot-path)
        if let Some(threat) = self.legion.get_threat_entity(entity_id).await? {
            return Ok(Some(threat));
        }

        // Fall back to apecs (cold-path)
        self.apecs.load_threat(entity_id).await
    }

    /// Get all threats
    pub async fn get_all_threats(&self) -> Result<Vec<(u64, ThreatEntityComponent)>> {
        self.legion.get_all_threats().await
    }

    /// Get threats by HD4 phase
    pub async fn get_threats_by_phase(&self, phase: Hd4Phase) -> Result<Vec<u64>> {
        self.legion.get_threats_by_phase(phase).await
    }

    /// Get hot-path threats
    pub async fn get_hot_threats(&self) -> Result<Vec<u64>> {
        self.legion.get_hot_threats().await
    }

    // =========================================================================
    // HD4 PHASE OPERATIONS
    // =========================================================================

    /// Update HD4 phase for entity
    pub async fn update_phase(&self, entity_id: u64, phase: Hd4Phase) -> Result<()> {
        self.legion.update_hd4_phase(entity_id, phase).await
    }

    // =========================================================================
    // CRYSTAL OPERATIONS
    // =========================================================================

    /// Update crystal evaluation
    pub async fn update_crystal(
        &self,
        entity_id: u64,
        ring_strength: f32,
        family: DefensiveCrystalFamily,
    ) -> Result<()> {
        self.legion
            .update_crystal_eval(entity_id, ring_strength, family)
            .await
    }

    // =========================================================================
    // SDT GATE OPERATIONS
    // =========================================================================

    /// Trigger SDT gate
    pub async fn trigger_gate(&self, entity_id: u64, value: f32) -> Result<bool> {
        self.legion.trigger_sdt_gate(entity_id, value).await
    }

    // =========================================================================
    // EEI CORRELATION
    // =========================================================================

    /// Correlate threat with EEI system
    pub async fn correlate_eei(
        &self,
        entity_id: u64,
        query: &str,
    ) -> Result<Option<EeiCorrelationComponent>> {
        self.apecs.correlate_with_eei(entity_id, query).await
    }

    /// Store EEI correlation
    pub async fn store_correlation(
        &self,
        entity_id: u64,
        correlation: EeiCorrelationComponent,
    ) -> Result<()> {
        self.apecs
            .store_eei_correlation(entity_id, correlation)
            .await
    }

    // =========================================================================
    // QUERY OPERATIONS
    // =========================================================================

    /// Get threats by MITRE technique
    pub async fn get_by_mitre(&self, technique: &str) -> Result<Vec<u64>> {
        self.apecs.get_threats_by_mitre(technique).await
    }

    /// Get threats in time window
    pub async fn get_in_window(&self, start_ns: u64, end_ns: u64) -> Result<Vec<u64>> {
        self.apecs.get_threats_in_window(start_ns, end_ns).await
    }

    // =========================================================================
    // TICK OPERATIONS
    // =========================================================================

    /// Process one tick (runs all systems)
    pub async fn tick(&self) -> Result<TickResult> {
        let tick = self.legion.tick().await;

        // Run Legion systems (hot-path)
        // TODO: Extract components and run systems

        // Periodically run apecs systems (cold-path)
        if tick % self.config.persist_interval == 0 {
            self.apecs.run_systems().await?;
        }

        Ok(TickResult {
            tick,
            legion_entities: self.legion.entity_count().await,
            apecs_entities: self.apecs.entity_count().await,
        })
    }

    /// Get current tick
    pub async fn current_tick(&self) -> u64 {
        self.legion.current_tick().await
    }

    // =========================================================================
    // STATS
    // =========================================================================

    /// Get world statistics
    pub async fn stats(&self) -> WorldStats {
        WorldStats {
            legion_entities: self.legion.entity_count().await,
            apecs_entities: self.apecs.entity_count().await,
            current_tick: self.legion.current_tick().await,
        }
    }
}

impl Default for DefenderWorld {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a tick operation
#[derive(Debug, Clone)]
pub struct TickResult {
    pub tick: u64,
    pub legion_entities: usize,
    pub apecs_entities: usize,
}

/// World statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorldStats {
    pub legion_entities: usize,
    pub apecs_entities: usize,
    pub current_tick: u64,
}
