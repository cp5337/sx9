//! Health Monitoring

use std::sync::Arc;
use sx9_atlas_bus::PlasmaState;

#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub status: String,
    pub sdt_state: String,
    pub excited: bool,
}

pub struct HealthMonitor {
    plasma: Arc<PlasmaState>,
}

impl HealthMonitor {
    pub fn new(plasma: Arc<PlasmaState>) -> Self {
        Self { plasma }
    }

    pub fn get_status(&self) -> HealthStatus {
        HealthStatus {
            status: "ok".to_string(),
            sdt_state: format!("{:?}", self.plasma.sdt_state()),
            excited: self.plasma.is_excited(),
        }
    }
}
