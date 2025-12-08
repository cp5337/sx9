//! Legion ECS Layer (Layer 2: Deterministic Batch Processing)

use crate::components::PlasmaComponent;
use anyhow::Result;
use legion::*;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Legion-based Plasma world
pub struct LegionPlasmaWorld {
    world: Arc<RwLock<World>>,
    entities: Arc<RwLock<std::collections::HashMap<u64, Entity>>>,
    next_id: Arc<RwLock<u64>>,
}

impl LegionPlasmaWorld {
    pub fn new() -> Result<Self> {
        let world = World::default();
        Ok(Self {
            world: Arc::new(RwLock::new(world)),
            entities: Arc::new(RwLock::new(std::collections::HashMap::new())),
            next_id: Arc::new(RwLock::new(1)),
        })
    }

    pub async fn add_plasma_entity(&self, component: PlasmaComponent) -> Result<u64> {
        let mut world = self.world.write().await;
        let mut entities = self.entities.write().await;
        let mut next_id = self.next_id.write().await;

        let entity_id = *next_id;
        *next_id += 1;

        let entity = world.push((component,));
        entities.insert(entity_id, entity);

        Ok(entity_id)
    }

    pub async fn update_plasma_entity(&self, entity_id: u64, component: PlasmaComponent) -> Result<()> {
        let mut world = self.world.write().await;
        let entities = self.entities.read().await;

        if let Some(&entity) = entities.get(&entity_id) {
            if let Some(mut entry) = world.entry(entity) {
                if let Ok(comp) = entry.get_component_mut::<PlasmaComponent>() {
                    *comp = component;
                }
            }
        }

        Ok(())
    }

    pub async fn get_plasma_entity(&self, entity_id: u64) -> Result<Option<PlasmaComponent>> {
        let mut world = self.world.write().await;
        let entities = self.entities.read().await;

        if let Some(&entity) = entities.get(&entity_id) {
            if let Some(entry) = world.entry(entity) {
                if let Ok(component) = entry.get_component::<PlasmaComponent>() {
                    return Ok(Some(component.clone()));
                }
            }
        }

        Ok(None)
    }

    pub async fn run_systems(&self) -> Result<()> {
        // Run Legion systems (batch processing)
        let world = self.world.read().await;
        // Systems would be registered and executed here
        Ok(())
    }
}

