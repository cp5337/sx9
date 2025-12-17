//! Legion ECS Layer (Layer 2: Deterministic Batch Processing)
//!
//! Hot-path operations for threat entity management.
//! Adapted from sx9-plasma-ecs for Plasma-Defender's threat domain.

use crate::ecs::components::*;
use anyhow::Result;
use legion::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Legion-based Defender world for threat entities
pub struct DefenderLegionWorld {
    world: Arc<RwLock<World>>,
    /// Entity ID to Legion Entity mapping
    entities: Arc<RwLock<HashMap<u64, Entity>>>,
    /// Next entity ID
    next_id: Arc<RwLock<u64>>,
    /// Tick counter
    tick_count: Arc<RwLock<u64>>,
}

impl DefenderLegionWorld {
    pub fn new() -> Result<Self> {
        let world = World::default();
        Ok(Self {
            world: Arc::new(RwLock::new(world)),
            entities: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(1)),
            tick_count: Arc::new(RwLock::new(0)),
        })
    }

    // =========================================================================
    // THREAT ENTITY OPERATIONS
    // =========================================================================

    /// Add a new threat entity
    pub async fn add_threat_entity(&self, threat: ThreatEntityComponent) -> Result<u64> {
        let mut world = self.world.write().await;
        let mut entities = self.entities.write().await;
        let mut next_id = self.next_id.write().await;

        let entity_id = *next_id;
        *next_id += 1;

        // Create entity with threat component and default supporting components
        let entity = world.push((
            ThreatEntityComponent {
                entity_id,
                ..threat
            },
            Hd4PhaseComponent::default(),
            CrystalEvalComponent::default(),
            SdtGateComponent::default(),
            ThreatObserverComponent::default(),
        ));

        entities.insert(entity_id, entity);
        tracing::debug!("Created threat entity {}", entity_id);

        Ok(entity_id)
    }

    /// Add threat entity from OSSEC alert
    pub async fn add_threat_from_ossec(&self, alert: OssecAlertComponent) -> Result<u64> {
        let mut world = self.world.write().await;
        let mut entities = self.entities.write().await;
        let mut next_id = self.next_id.write().await;

        let entity_id = *next_id;
        *next_id += 1;

        // Determine initial HD4 phase based on alert level
        let initial_phase = match alert.level {
            0..=3 => Hd4Phase::Hunt,
            4..=7 => Hd4Phase::Detect,
            8..=10 => Hd4Phase::Disrupt,
            11..=13 => Hd4Phase::Disable,
            _ => Hd4Phase::Dominate,
        };

        // Create threat entity from alert
        let threat = ThreatEntityComponent {
            entity_id,
            threat_hash: alert.rule_id as u64, // Use rule_id as initial hash
            confidence: alert.level as f32 / 15.0,
            first_seen_ns: alert.timestamp,
            last_seen_ns: alert.timestamp,
            speed_class: if alert.level >= 10 { 0 } else { 1 }, // Hot for critical
            ..Default::default()
        };

        let hd4 = Hd4PhaseComponent {
            phase: initial_phase,
            entered_at: alert.timestamp,
            ..Default::default()
        };

        let entity = world.push((
            threat,
            alert,
            hd4,
            CrystalEvalComponent::default(),
            SdtGateComponent::default(),
            ThreatObserverComponent::default(),
        ));

        entities.insert(entity_id, entity);
        tracing::info!("Created threat entity {} from OSSEC alert", entity_id);

        Ok(entity_id)
    }

    /// Add threat entity from tool output
    pub async fn add_threat_from_tool(&self, tool: ToolOutputComponent) -> Result<u64> {
        let mut world = self.world.write().await;
        let mut entities = self.entities.write().await;
        let mut next_id = self.next_id.write().await;

        let entity_id = *next_id;
        *next_id += 1;

        let threat = ThreatEntityComponent {
            entity_id,
            threat_hash: tool.tool_hash,
            first_seen_ns: tool.processed_at,
            last_seen_ns: tool.processed_at,
            speed_class: 1, // Warm - needs evaluation
            ..Default::default()
        };

        let entity = world.push((
            threat,
            tool,
            Hd4PhaseComponent::default(),
            CrystalEvalComponent::default(),
            SdtGateComponent::default(),
            ThreatObserverComponent::default(),
        ));

        entities.insert(entity_id, entity);
        tracing::debug!("Created threat entity {} from tool output", entity_id);

        Ok(entity_id)
    }

    /// Update threat entity
    pub async fn update_threat_entity(
        &self,
        entity_id: u64,
        threat: ThreatEntityComponent,
    ) -> Result<()> {
        let mut world = self.world.write().await;
        let entities = self.entities.read().await;

        if let Some(&entity) = entities.get(&entity_id) {
            if let Some(mut entry) = world.entry(entity) {
                if let Ok(comp) = entry.get_component_mut::<ThreatEntityComponent>() {
                    *comp = threat;
                }
            }
        }

        Ok(())
    }

    /// Get threat entity
    pub async fn get_threat_entity(&self, entity_id: u64) -> Result<Option<ThreatEntityComponent>> {
        let mut world = self.world.write().await;
        let entities = self.entities.read().await;

        if let Some(&entity) = entities.get(&entity_id) {
            if let Some(entry) = world.entry(entity) {
                if let Ok(component) = entry.get_component::<ThreatEntityComponent>() {
                    return Ok(Some(component.clone()));
                }
            }
        }

        Ok(None)
    }

    /// Get all threat entities
    pub async fn get_all_threats(&self) -> Result<Vec<(u64, ThreatEntityComponent)>> {
        let world = self.world.read().await;
        let mut threats = Vec::new();

        let mut query = <&ThreatEntityComponent>::query();
        for threat in query.iter(&*world) {
            threats.push((threat.entity_id, threat.clone()));
        }

        Ok(threats)
    }

    /// Get threats by HD4 phase
    pub async fn get_threats_by_phase(&self, phase: Hd4Phase) -> Result<Vec<u64>> {
        let world = self.world.read().await;
        let mut entity_ids = Vec::new();

        let mut query = <(&ThreatEntityComponent, &Hd4PhaseComponent)>::query();
        for (threat, hd4) in query.iter(&*world) {
            if hd4.phase == phase {
                entity_ids.push(threat.entity_id);
            }
        }

        Ok(entity_ids)
    }

    /// Get hot-path threats (speed_class == 0)
    pub async fn get_hot_threats(&self) -> Result<Vec<u64>> {
        let world = self.world.read().await;
        let mut entity_ids = Vec::new();

        let mut query = <&ThreatEntityComponent>::query();
        for threat in query.iter(&*world) {
            if threat.speed_class == 0 {
                entity_ids.push(threat.entity_id);
            }
        }

        Ok(entity_ids)
    }

    // =========================================================================
    // HD4 PHASE OPERATIONS
    // =========================================================================

    /// Update HD4 phase for entity
    pub async fn update_hd4_phase(&self, entity_id: u64, phase: Hd4Phase) -> Result<()> {
        let mut world = self.world.write().await;
        let entities = self.entities.read().await;

        if let Some(&entity) = entities.get(&entity_id) {
            if let Some(mut entry) = world.entry(entity) {
                if let Ok(hd4) = entry.get_component_mut::<Hd4PhaseComponent>() {
                    hd4.previous_phase = Some(hd4.phase);
                    hd4.phase = phase;
                    hd4.entered_at = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as u64;
                    hd4.ticks_in_phase = 0;
                    tracing::info!("Entity {} transitioned to HD4 phase {:?}", entity_id, phase);
                }
            }
        }

        Ok(())
    }

    // =========================================================================
    // CRYSTAL OPERATIONS
    // =========================================================================

    /// Update crystal evaluation for entity
    pub async fn update_crystal_eval(
        &self,
        entity_id: u64,
        ring_strength: f32,
        family: DefensiveCrystalFamily,
    ) -> Result<()> {
        let mut world = self.world.write().await;
        let entities = self.entities.read().await;

        if let Some(&entity) = entities.get(&entity_id) {
            if let Some(mut entry) = world.entry(entity) {
                if let Ok(crystal) = entry.get_component_mut::<CrystalEvalComponent>() {
                    crystal.ring_strength = ring_strength;
                    crystal.family = family;
                    crystal.last_eval_ns = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as u64;
                    crystal.eval_count += 1;
                }
            }
        }

        Ok(())
    }

    // =========================================================================
    // SDT GATE OPERATIONS
    // =========================================================================

    /// Trigger SDT gate for entity
    pub async fn trigger_sdt_gate(&self, entity_id: u64, value: f32) -> Result<bool> {
        let mut world = self.world.write().await;
        let entities = self.entities.read().await;
        let tick = *self.tick_count.read().await;

        if let Some(&entity) = entities.get(&entity_id) {
            if let Some(mut entry) = world.entry(entity) {
                if let Ok(sdt) = entry.get_component_mut::<SdtGateComponent>() {
                    sdt.current_value = value;

                    // Check if threshold exceeded
                    if value >= sdt.threshold && sdt.state != SdtState::Latched {
                        sdt.state = SdtState::Latched;
                        sdt.last_trigger_tick = tick;
                        sdt.trigger_count += 1;
                        tracing::warn!("SDT gate {} LATCHED for entity {}", sdt.gate_id, entity_id);
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    // =========================================================================
    // TICK OPERATIONS
    // =========================================================================

    /// Increment tick counter
    pub async fn tick(&self) -> u64 {
        let mut tick = self.tick_count.write().await;
        *tick += 1;
        *tick
    }

    /// Get current tick
    pub async fn current_tick(&self) -> u64 {
        *self.tick_count.read().await
    }

    /// Get entity count
    pub async fn entity_count(&self) -> usize {
        self.entities.read().await.len()
    }
}

impl Default for DefenderLegionWorld {
    fn default() -> Self {
        Self::new().expect("Failed to create DefenderLegionWorld")
    }
}
