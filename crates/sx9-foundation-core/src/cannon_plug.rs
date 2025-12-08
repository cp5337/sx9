use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CannonPlug {
    pub id: String,
    pub name: String,
    pub plug_type: PlugType,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub status: PlugStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlugType {
    IOS,
    Web,
    API,
    Service,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlugStatus {
    Connected,
    Disconnected,
    Error,
    Pending,
}

#[derive(Debug)]
pub struct CannonPlugManager {
    plugs: HashMap<String, CannonPlug>,
}

impl CannonPlugManager {
    pub fn new() -> Self {
        Self {
            plugs: HashMap::new(),
        }
    }

    pub fn register_plug(&mut self, plug: CannonPlug) -> String {
        let plug_id = plug.id.clone();
        info!("🔌 Registering cannon plug: {} ({})", plug.name, plug_id);
        self.plugs.insert(plug_id.clone(), plug);
        plug_id
    }

    pub fn get_plug(&self, plug_id: &str) -> Option<&CannonPlug> {
        self.plugs.get(plug_id)
    }

    pub fn get_all_plugs(&self) -> Vec<&CannonPlug> {
        self.plugs.values().collect()
    }

    pub fn connect_plug(&mut self, plug_id: &str) -> Result<(), String> {
        if let Some(plug) = self.plugs.get_mut(plug_id) {
            plug.status = PlugStatus::Connected;
            plug.last_activity = chrono::Utc::now();
            info!("🔗 Cannon plug connected: {}", plug.name);
            Ok(())
        } else {
            Err(format!("Plug {} not found", plug_id))
        }
    }

    pub fn disconnect_plug(&mut self, plug_id: &str) -> Result<(), String> {
        if let Some(plug) = self.plugs.get_mut(plug_id) {
            plug.status = PlugStatus::Disconnected;
            info!("🔌 Cannon plug disconnected: {}", plug.name);
            Ok(())
        } else {
            Err(format!("Plug {} not found", plug_id))
        }
    }

    pub fn get_plugs_by_type(&self, plug_type: &PlugType) -> Vec<&CannonPlug> {
        self.plugs.values()
            .filter(|plug| std::mem::discriminant(&plug.plug_type) == std::mem::discriminant(plug_type))
            .collect()
    }

    pub fn get_connected_plugs(&self) -> Vec<&CannonPlug> {
        self.plugs.values()
            .filter(|plug| matches!(plug.status, PlugStatus::Connected))
            .collect()
    }
}

impl Default for CannonPlugManager {
    fn default() -> Self {
        Self::new()
    }
}

