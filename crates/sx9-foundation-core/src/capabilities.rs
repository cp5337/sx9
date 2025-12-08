//! Move, Shoot, Communicate Capabilities Validation

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitCapabilities {
    pub movement: MovementCapability,
    pub fires: FiresCapability,
    pub last_validated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementCapability {
    pub operational: bool,
    pub mobility_status: MobilityStatus,
    pub vehicles_available: u32,
    pub vehicles_operational: u32,
    pub fuel_status: FuelStatus,
    pub route_planning: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MobilityStatus {
    FullyMobile,
    LimitedMobility,
    Immobilized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuelStatus {
    pub current_level: f64,
    pub consumption_rate: f64,
    pub range_km: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiresCapability {
    pub operational: bool,
    pub weapons_systems: Vec<WeaponSystem>,
    pub ammunition_status: AmmunitionStatus,
    pub fire_control: bool,
    pub target_acquisition: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponSystem {
    pub name: String,
    pub system_type: String,
    pub operational: bool,
    pub effective_range: u32,
    pub ammunition_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmmunitionStatus {
    pub total_rounds: u32,
    pub by_type: std::collections::HashMap<String, u32>,
    pub resupply_needed: bool,
}

impl UnitCapabilities {
    pub fn new() -> Self {
        Self {
            movement: MovementCapability {
                operational: true,
                mobility_status: MobilityStatus::FullyMobile,
                vehicles_available: 4,
                vehicles_operational: 3,
                fuel_status: FuelStatus {
                    current_level: 75.0,
                    consumption_rate: 2.5,
                    range_km: 300.0,
                },
                route_planning: true,
            },
            fires: FiresCapability {
                operational: true,
                weapons_systems: vec![
                    WeaponSystem {
                        name: "M4A1".to_string(),
                        system_type: "Individual Weapon".to_string(),
                        operational: true,
                        effective_range: 500,
                        ammunition_count: 210,
                    },
                    WeaponSystem {
                        name: "M240B".to_string(),
                        system_type: "Machine Gun".to_string(),
                        operational: true,
                        effective_range: 1800,
                        ammunition_count: 800,
                    },
                ],
                ammunition_status: AmmunitionStatus {
                    total_rounds: 1010,
                    by_type: {
                        let mut ammo = std::collections::HashMap::new();
                        ammo.insert("5.56mm".to_string(), 210);
                        ammo.insert("7.62mm".to_string(), 800);
                        ammo
                    },
                    resupply_needed: false,
                },
                fire_control: true,
                target_acquisition: true,
            },
            last_validated: chrono::Utc::now(),
        }
    }

    pub fn validate_movement(&mut self) -> bool {
        let vehicle_readiness = self.movement.vehicles_operational as f64
            / self.movement.vehicles_available as f64;
        let fuel_adequate = self.movement.fuel_status.current_level > 25.0;

        self.movement.operational = vehicle_readiness >= 0.75 && fuel_adequate;
        self.last_validated = chrono::Utc::now();

        self.movement.operational
    }

    pub fn validate_fires(&mut self) -> bool {
        let operational_weapons = self.fires.weapons_systems.iter()
            .filter(|w| w.operational).count();
        let weapons_ready = operational_weapons > 0;
        let ammo_adequate = self.fires.ammunition_status.total_rounds > 100;

        self.fires.operational = weapons_ready && ammo_adequate
            && self.fires.fire_control && self.fires.target_acquisition;
        self.last_validated = chrono::Utc::now();

        self.fires.operational
    }

    pub fn validate_all(&mut self) -> (bool, bool) {
        let movement_ok = self.validate_movement();
        let fires_ok = self.validate_fires();
        (movement_ok, fires_ok)
    }
}