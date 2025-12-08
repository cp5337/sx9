//! Mission Load Sets - In-App Purchase Tool Chains
//!
//! Mission Loads are curated combinations of tools that provide
//! force multiplication for operators. Available in tiers:
//! - Free: Basic hunt/detect capabilities
//! - Commercial: Premium tool chains
//! - Enterprise: Full spectrum operations

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::nonagon::{NonagonCell, HD4Phase};

/// Clearance levels for Mission Loads
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClearanceLevel {
    /// Public - no restrictions
    Public,
    /// Commercial - requires license
    Commercial,
    /// Restricted - enterprise only
    Restricted,
    /// Classified - government/military
    Classified,
}

impl Default for ClearanceLevel {
    fn default() -> Self {
        ClearanceLevel::Public
    }
}

/// Primitive operations available in tool chains
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum Primitive {
    // Data Operations
    Read = 0x0001,
    Write = 0x0002,
    Transform = 0x0004,
    Filter = 0x0008,

    // Security Operations
    Encrypt = 0x0010,
    Decrypt = 0x0020,
    Authenticate = 0x0040,
    Authorize = 0x0080,
    Validate = 0x0100,

    // Network Operations
    Route = 0x0200,
    Buffer = 0x0400,
    Queue = 0x0800,
    Synchronize = 0x1000,
    Replicate = 0x2000,

    // Analysis Operations
    Observe = 0x4000,
    Cache = 0x8000,
    Execute = 0x10000,

    // Threat Operations (restricted)
    Reconnaissance = 0x20000,
    CommandControl = 0x40000,
    Install = 0x80000,
}

impl Primitive {
    /// Get bitfield value
    pub fn bitfield(self) -> u32 {
        self as u32
    }

    /// Check if primitive is restricted
    pub fn is_restricted(self) -> bool {
        matches!(self, Primitive::Reconnaissance | Primitive::CommandControl | Primitive::Install)
    }

    /// Get all primitives from bitfield
    pub fn from_bitfield(bits: u32) -> Vec<Primitive> {
        let all = [
            Primitive::Read, Primitive::Write, Primitive::Transform, Primitive::Filter,
            Primitive::Encrypt, Primitive::Decrypt, Primitive::Authenticate,
            Primitive::Authorize, Primitive::Validate, Primitive::Route,
            Primitive::Buffer, Primitive::Queue, Primitive::Synchronize,
            Primitive::Replicate, Primitive::Observe, Primitive::Cache,
            Primitive::Execute, Primitive::Reconnaissance, Primitive::CommandControl,
            Primitive::Install,
        ];
        all.iter()
            .filter(|p| (bits & p.bitfield()) != 0)
            .copied()
            .collect()
    }
}

/// Mission Load Set - purchasable tool chain package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionLoadSet {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Description
    pub description: String,
    /// HD4 phase this load targets
    pub hd4_phase: HD4Phase,
    /// Required clearance level
    pub clearance: ClearanceLevel,
    /// Primitive operations included
    pub primitives: Vec<Primitive>,
    /// Primitive bitfield (for fast lookup)
    pub primitive_bitfield: u32,
    /// Associated nonagon cell
    pub nonagon: NonagonCell,
    /// Tool IDs included in this load
    pub tool_ids: Vec<String>,
    /// Price in credits (0 = free tier)
    pub price_credits: u64,
    /// Whether this is active/purchasable
    pub active: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified
    pub updated_at: DateTime<Utc>,
}

impl MissionLoadSet {
    /// Create new mission load set
    pub fn new(
        name: impl Into<String>,
        hd4_phase: HD4Phase,
        clearance: ClearanceLevel,
    ) -> Self {
        let name = name.into();
        let id = format!("ml-{}", Uuid::new_v4());
        let now = Utc::now();

        let mut nonagon = NonagonCell::new(&id);
        nonagon.set_hd4_phase(hd4_phase);

        Self {
            id,
            name,
            description: String::new(),
            hd4_phase,
            clearance,
            primitives: Vec::new(),
            primitive_bitfield: 0,
            nonagon,
            tool_ids: Vec::new(),
            price_credits: 0,
            active: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Add primitive to this load
    pub fn add_primitive(&mut self, primitive: Primitive) {
        if !self.primitives.contains(&primitive) {
            self.primitives.push(primitive);
            self.primitive_bitfield |= primitive.bitfield();
        }
    }

    /// Add multiple primitives
    pub fn add_primitives(&mut self, primitives: &[Primitive]) {
        for p in primitives {
            self.add_primitive(*p);
        }
    }

    /// Check if load has primitive
    pub fn has_primitive(&self, primitive: Primitive) -> bool {
        (self.primitive_bitfield & primitive.bitfield()) != 0
    }

    /// Add tool ID
    pub fn add_tool(&mut self, tool_id: impl Into<String>) {
        self.tool_ids.push(tool_id.into());
        self.updated_at = Utc::now();
    }

    /// Check if load contains restricted primitives
    pub fn has_restricted_primitives(&self) -> bool {
        self.primitives.iter().any(|p| p.is_restricted())
    }

    /// Validate clearance matches primitive restrictions
    pub fn validate_clearance(&self) -> bool {
        if self.has_restricted_primitives() {
            matches!(self.clearance, ClearanceLevel::Restricted | ClearanceLevel::Classified)
        } else {
            true
        }
    }
}

/// Mission Load Catalog - manages all available loads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionLoadCatalog {
    /// All mission loads by ID
    loads: HashMap<String, MissionLoadSet>,
    /// Free tier load IDs
    free_tier: Vec<String>,
    /// Commercial tier load IDs
    commercial_tier: Vec<String>,
    /// Enterprise tier load IDs
    enterprise_tier: Vec<String>,
}

impl MissionLoadCatalog {
    /// Create empty catalog
    pub fn new() -> Self {
        Self {
            loads: HashMap::new(),
            free_tier: Vec::new(),
            commercial_tier: Vec::new(),
            enterprise_tier: Vec::new(),
        }
    }

    /// Create catalog with default mission loads
    pub fn new_with_defaults() -> Self {
        let mut catalog = Self::new();

        // Free tier - basic hunt/detect
        let mut hunt_basic = MissionLoadSet::new("Hunt Basic", HD4Phase::Hunt, ClearanceLevel::Public);
        hunt_basic.description = "Basic threat hunting with OSINT tools".into();
        hunt_basic.add_primitives(&[Primitive::Read, Primitive::Observe, Primitive::Filter]);
        catalog.add_load(hunt_basic, Tier::Free);

        let mut detect_basic = MissionLoadSet::new("Detect Basic", HD4Phase::Detect, ClearanceLevel::Public);
        detect_basic.description = "Basic threat detection with signature matching".into();
        detect_basic.add_primitives(&[Primitive::Read, Primitive::Validate, Primitive::Cache]);
        catalog.add_load(detect_basic, Tier::Free);

        // Commercial tier - premium tools
        let mut hunt_premium = MissionLoadSet::new("Hunt Premium", HD4Phase::Hunt, ClearanceLevel::Commercial);
        hunt_premium.description = "Advanced threat hunting with ML-powered analysis".into();
        hunt_premium.price_credits = 1000;
        hunt_premium.add_primitives(&[
            Primitive::Read, Primitive::Observe, Primitive::Filter,
            Primitive::Transform, Primitive::Cache, Primitive::Execute,
        ]);
        catalog.add_load(hunt_premium, Tier::Commercial);

        let mut detect_premium = MissionLoadSet::new("Detect Premium", HD4Phase::Detect, ClearanceLevel::Commercial);
        detect_premium.description = "Advanced detection with behavioral analysis".into();
        detect_premium.price_credits = 1500;
        detect_premium.add_primitives(&[
            Primitive::Read, Primitive::Validate, Primitive::Cache,
            Primitive::Observe, Primitive::Transform, Primitive::Queue,
        ]);
        catalog.add_load(detect_premium, Tier::Commercial);

        let mut disrupt_pro = MissionLoadSet::new("Disrupt Professional", HD4Phase::Disrupt, ClearanceLevel::Commercial);
        disrupt_pro.description = "Professional disruption toolkit".into();
        disrupt_pro.price_credits = 2500;
        disrupt_pro.add_primitives(&[
            Primitive::Read, Primitive::Write, Primitive::Execute,
            Primitive::Route, Primitive::Buffer, Primitive::Synchronize,
        ]);
        catalog.add_load(disrupt_pro, Tier::Commercial);

        // Enterprise tier - full spectrum
        let mut disable_ent = MissionLoadSet::new("Disable Enterprise", HD4Phase::Disable, ClearanceLevel::Restricted);
        disable_ent.description = "Enterprise-grade disable operations".into();
        disable_ent.price_credits = 10000;
        disable_ent.add_primitives(&[
            Primitive::Read, Primitive::Write, Primitive::Execute,
            Primitive::Encrypt, Primitive::Decrypt, Primitive::Authenticate,
            Primitive::Route, Primitive::CommandControl,
        ]);
        catalog.add_load(disable_ent, Tier::Enterprise);

        let mut dominate_rst = MissionLoadSet::new("Dominate Restricted", HD4Phase::Dominate, ClearanceLevel::Restricted);
        dominate_rst.description = "Full spectrum dominance toolkit".into();
        dominate_rst.price_credits = 25000;
        dominate_rst.add_primitives(&[
            Primitive::Read, Primitive::Write, Primitive::Execute,
            Primitive::Encrypt, Primitive::Decrypt, Primitive::Authenticate,
            Primitive::Authorize, Primitive::Route, Primitive::Replicate,
            Primitive::CommandControl, Primitive::Install,
        ]);
        catalog.add_load(dominate_rst, Tier::Enterprise);

        catalog
    }

    /// Add load to catalog
    pub fn add_load(&mut self, load: MissionLoadSet, tier: Tier) {
        let id = load.id.clone();
        self.loads.insert(id.clone(), load);

        match tier {
            Tier::Free => self.free_tier.push(id),
            Tier::Commercial => self.commercial_tier.push(id),
            Tier::Enterprise => self.enterprise_tier.push(id),
        }
    }

    /// Get load by ID
    pub fn get(&self, id: &str) -> Option<&MissionLoadSet> {
        self.loads.get(id)
    }

    /// Get mutable load by ID
    pub fn get_mut(&mut self, id: &str) -> Option<&mut MissionLoadSet> {
        self.loads.get_mut(id)
    }

    /// Get all loads in tier
    pub fn get_tier(&self, tier: Tier) -> Vec<&MissionLoadSet> {
        let ids = match tier {
            Tier::Free => &self.free_tier,
            Tier::Commercial => &self.commercial_tier,
            Tier::Enterprise => &self.enterprise_tier,
        };
        ids.iter().filter_map(|id| self.loads.get(id)).collect()
    }

    /// Get loads by HD4 phase
    pub fn get_by_phase(&self, phase: HD4Phase) -> Vec<&MissionLoadSet> {
        self.loads.values().filter(|l| l.hd4_phase == phase).collect()
    }

    /// Get loads by clearance level
    pub fn get_by_clearance(&self, clearance: ClearanceLevel) -> Vec<&MissionLoadSet> {
        self.loads.values().filter(|l| l.clearance == clearance).collect()
    }

    /// Get all load IDs
    pub fn all_ids(&self) -> Vec<&String> {
        self.loads.keys().collect()
    }

    /// Get total count
    pub fn len(&self) -> usize {
        self.loads.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.loads.is_empty()
    }
}

impl Default for MissionLoadCatalog {
    fn default() -> Self {
        Self::new_with_defaults()
    }
}

/// Pricing tier for mission loads
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Free,
    Commercial,
    Enterprise,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_bitfield() {
        let mut load = MissionLoadSet::new("Test", HD4Phase::Hunt, ClearanceLevel::Public);
        load.add_primitives(&[Primitive::Read, Primitive::Write, Primitive::Execute]);

        assert!(load.has_primitive(Primitive::Read));
        assert!(load.has_primitive(Primitive::Write));
        assert!(load.has_primitive(Primitive::Execute));
        assert!(!load.has_primitive(Primitive::Encrypt));
    }

    #[test]
    fn test_clearance_validation() {
        let mut load = MissionLoadSet::new("Test", HD4Phase::Dominate, ClearanceLevel::Public);
        load.add_primitive(Primitive::CommandControl);
        assert!(!load.validate_clearance());

        load.clearance = ClearanceLevel::Restricted;
        assert!(load.validate_clearance());
    }

    #[test]
    fn test_catalog_defaults() {
        let catalog = MissionLoadCatalog::new_with_defaults();
        assert!(!catalog.is_empty());
        assert_eq!(catalog.get_tier(Tier::Free).len(), 2);
    }
}
