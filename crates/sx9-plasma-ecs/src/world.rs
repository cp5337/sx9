//! Plasma ECS World
//!
//! Unified world interface for both Legion and apecs

use crate::components::*;
use crate::legion_layer::LegionPlasmaWorld;
use crate::apecs_layer::ApecsPlasmaWorld;
use anyhow::Result;
use std::sync::Arc;

/// Unified Plasma ECS World
pub struct PlasmaWorld {
    legion_world: Option<Arc<LegionPlasmaWorld>>,
    apecs_world: Option<Arc<ApecsPlasmaWorld>>,
}

impl PlasmaWorld {
    pub fn new(use_legion: bool, use_apecs: bool) -> Result<Self> {
        Ok(Self {
            legion_world: if use_legion {
                Some(Arc::new(LegionPlasmaWorld::new()?))
            } else {
                None
            },
            apecs_world: if use_apecs {
                Some(Arc::new(ApecsPlasmaWorld::new()?))
            } else {
                None
            },
        })
    }

    /// Add plasma entity to world
    pub async fn add_plasma_entity(&self, component: PlasmaComponent) -> Result<u64> {
        if let Some(ref legion) = self.legion_world {
            return legion.add_plasma_entity(component).await;
        }
        if let Some(ref apecs) = self.apecs_world {
            return apecs.add_plasma_entity(component).await;
        }
        anyhow::bail!("No ECS world available");
    }

    /// Update plasma entity
    pub async fn update_plasma_entity(&self, entity_id: u64, component: PlasmaComponent) -> Result<()> {
        if let Some(ref legion) = self.legion_world {
            return legion.update_plasma_entity(entity_id, component).await;
        }
        if let Some(ref apecs) = self.apecs_world {
            return apecs.update_plasma_entity(entity_id, component).await;
        }
        anyhow::bail!("No ECS world available");
    }

    /// Get plasma entity
    pub async fn get_plasma_entity(&self, entity_id: u64) -> Result<Option<PlasmaComponent>> {
        if let Some(ref legion) = self.legion_world {
            return legion.get_plasma_entity(entity_id).await;
        }
        if let Some(ref apecs) = self.apecs_world {
            return apecs.get_plasma_entity(entity_id).await;
        }
        anyhow::bail!("No ECS world available");
    }

    /// Run ECS systems
    pub async fn run_systems(&self) -> Result<()> {
        if let Some(ref legion) = self.legion_world {
            legion.run_systems().await?;
        }
        if let Some(ref apecs) = self.apecs_world {
            apecs.run_systems().await?;
        }
        Ok(())
    }
}

