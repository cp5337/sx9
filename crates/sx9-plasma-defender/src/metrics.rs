//! Metrics Collection

use sx9_atlas_bus::PlasmaState;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Metrics {
    pub delta_angle: u16,
    pub entropy: u32,
    pub excited: bool,
    pub ring_strength: f32,
    pub trigger_count: u32,
    pub supersession_count: u32,
}

pub struct MetricsCollector {
    plasma: Arc<PlasmaState>,
}

impl MetricsCollector {
    pub fn new(plasma: Arc<PlasmaState>) -> Self {
        Self { plasma }
    }
    
    pub fn collect(&self) -> Metrics {
        Metrics {
            delta_angle: self.plasma.delta_angle_raw(),
            entropy: self.plasma.entropy(),
            excited: self.plasma.is_excited(),
            ring_strength: self.plasma.last_ring_strength(),
            trigger_count: self.plasma.trigger_count(),
            supersession_count: self.plasma.supersession_count(),
        }
    }
}

