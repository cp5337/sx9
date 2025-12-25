//! SX9 Licensing & Subscription Model
//!
//! Provides feature gating, subscription management, and tier-based access control.
//! The gateway acts as the central yes/no filter for all components and features.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// License tiers - progressive unlocking ("level ascension")
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LicenseTier {
    /// Free tier - basic components, community support
    Free = 0,
    /// Pro tier - advanced components, priority support
    Pro = 1,
    /// Enterprise tier - all components, dedicated support, custom integrations
    Enterprise = 2,
    /// Government tier - full access, no restrictions, compliance features
    Government = 3,
}

impl LicenseTier {
    /// Check if this tier can access a required tier level
    pub fn can_access(&self, required: LicenseTier) -> bool {
        *self >= required
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            LicenseTier::Free => "Free",
            LicenseTier::Pro => "Pro",
            LicenseTier::Enterprise => "Enterprise",
            LicenseTier::Government => "Government",
        }
    }

    /// Get tier color for UI
    pub fn color(&self) -> &'static str {
        match self {
            LicenseTier::Free => "#6b7280",      // gray
            LicenseTier::Pro => "#3b82f6",       // blue
            LicenseTier::Enterprise => "#8b5cf6", // purple
            LicenseTier::Government => "#10b981", // green
        }
    }
}

/// Subscription status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Unique subscription ID
    pub id: String,
    /// Organization/user ID
    pub org_id: String,
    /// Current license tier
    pub tier: LicenseTier,
    /// Subscription start date (Unix timestamp)
    pub started_at: u64,
    /// Subscription expiration date (Unix timestamp, None = never expires)
    pub expires_at: Option<u64>,
    /// Is subscription currently active
    pub active: bool,
    /// Seats/users allowed (None = unlimited)
    pub seats: Option<u32>,
    /// Custom feature overrides (can grant specific features regardless of tier)
    pub feature_overrides: HashSet<String>,
    /// Custom component overrides (can grant specific components regardless of tier)
    pub component_overrides: HashSet<String>,
}

impl Subscription {
    /// Check if subscription is valid (active and not expired)
    pub fn is_valid(&self) -> bool {
        if !self.active {
            return false;
        }

        if let Some(expires) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            return now < expires;
        }

        true
    }

    /// Days until expiration (None if never expires or already expired)
    pub fn days_remaining(&self) -> Option<i64> {
        self.expires_at.map(|exp| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            ((exp as i64) - (now as i64)) / 86400
        })
    }

    /// Check if user can access a specific tier level
    pub fn can_access_tier(&self, required: LicenseTier) -> bool {
        self.is_valid() && self.tier.can_access(required)
    }

    /// Check if user can access a specific feature
    pub fn can_access_feature(&self, feature_id: &str, required_tier: LicenseTier) -> bool {
        if !self.is_valid() {
            return false;
        }

        // Check feature overrides first
        if self.feature_overrides.contains(feature_id) {
            return true;
        }

        // Fall back to tier check
        self.tier.can_access(required_tier)
    }

    /// Check if user can access a specific component
    pub fn can_access_component(&self, component_id: &str, required_tier: LicenseTier) -> bool {
        if !self.is_valid() {
            return false;
        }

        // Check component overrides first
        if self.component_overrides.contains(component_id) {
            return true;
        }

        // Fall back to tier check
        self.tier.can_access(required_tier)
    }
}

/// Component metadata for the marketplace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMeta {
    /// Unique component ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Description
    pub description: String,
    /// Category (toolbars, infrastructure, redteam, etc.)
    pub category: String,
    /// Minimum tier required to access
    pub required_tier: LicenseTier,
    /// Version string
    pub version: String,
    /// WASM bundle size in bytes (if applicable)
    pub wasm_size: Option<u64>,
    /// Requires dual heartbeat validation
    pub requires_heartbeat: bool,
    /// Icon name (lucide icon)
    pub icon: String,
    /// Capabilities/features provided
    pub capabilities: Vec<String>,
}

/// Feature flag definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlag {
    /// Feature ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Description
    pub description: String,
    /// Minimum tier required
    pub required_tier: LicenseTier,
    /// Is this a beta feature
    pub beta: bool,
}

/// License validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseValidation {
    /// Is the license valid
    pub valid: bool,
    /// Current tier
    pub tier: LicenseTier,
    /// Days remaining (None if never expires)
    pub days_remaining: Option<i64>,
    /// List of accessible component IDs
    pub accessible_components: Vec<String>,
    /// List of accessible feature IDs
    pub accessible_features: Vec<String>,
    /// Warning message (e.g., "Expires in 7 days")
    pub warning: Option<String>,
    /// Error message (if invalid)
    pub error: Option<String>,
}

/// License check request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseCheckRequest {
    /// Organization/API key
    pub api_key: String,
    /// Optional: specific component to check
    pub component_id: Option<String>,
    /// Optional: specific feature to check
    pub feature_id: Option<String>,
}

/// License check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseCheckResponse {
    /// Is access granted
    pub granted: bool,
    /// Reason (if denied)
    pub reason: Option<String>,
    /// Tier required for access
    pub required_tier: Option<LicenseTier>,
    /// User's current tier
    pub current_tier: LicenseTier,
    /// Upgrade URL (if denied due to tier)
    pub upgrade_url: Option<String>,
}

/// Default components by tier
pub fn default_components() -> Vec<ComponentMeta> {
    vec![
        // Free tier components
        ComponentMeta {
            id: "filter-panel".to_string(),
            name: "Filter Panel".to_string(),
            description: "Advanced filtering with sectors/phases".to_string(),
            category: "toolbars".to_string(),
            required_tier: LicenseTier::Free,
            version: "1.0.0".to_string(),
            wasm_size: None,
            requires_heartbeat: false,
            icon: "Search".to_string(),
            capabilities: vec!["filtering".to_string(), "search".to_string()],
        },
        ComponentMeta {
            id: "db-panel".to_string(),
            name: "Database Connection Panel".to_string(),
            description: "Unified database management".to_string(),
            category: "infrastructure".to_string(),
            required_tier: LicenseTier::Free,
            version: "1.0.0".to_string(),
            wasm_size: None,
            requires_heartbeat: false,
            icon: "Database".to_string(),
            capabilities: vec!["supabase".to_string(), "neon".to_string(), "sled".to_string()],
        },

        // Pro tier components
        ComponentMeta {
            id: "cognigraph".to_string(),
            name: "Cognigraph".to_string(),
            description: "Cognitive graph visualization".to_string(),
            category: "visualization".to_string(),
            required_tier: LicenseTier::Pro,
            version: "2.0.0".to_string(),
            wasm_size: Some(2_300_000),
            requires_heartbeat: true,
            icon: "Brain".to_string(),
            capabilities: vec!["graph".to_string(), "visualization".to_string(), "glaf".to_string()],
        },
        ComponentMeta {
            id: "hash-composer".to_string(),
            name: "Hash Composer".to_string(),
            description: "Trivariate hash composition & analysis".to_string(),
            category: "analysis".to_string(),
            required_tier: LicenseTier::Pro,
            version: "1.5.0".to_string(),
            wasm_size: Some(1_200_000),
            requires_heartbeat: true,
            icon: "Terminal".to_string(),
            capabilities: vec!["hashing".to_string(), "analysis".to_string(), "trivariate".to_string()],
        },

        // Enterprise tier components
        ComponentMeta {
            id: "redteam-runner".to_string(),
            name: "Red Team Runner".to_string(),
            description: "Automated red team operations".to_string(),
            category: "redteam".to_string(),
            required_tier: LicenseTier::Enterprise,
            version: "3.0.0".to_string(),
            wasm_size: Some(5_600_000),
            requires_heartbeat: true,
            icon: "Target".to_string(),
            capabilities: vec!["redteam".to_string(), "automation".to_string(), "atomic".to_string()],
        },
        ComponentMeta {
            id: "kali-tools".to_string(),
            name: "Kali Tools Integration".to_string(),
            description: "Full Kali Linux tool launcher".to_string(),
            category: "ops".to_string(),
            required_tier: LicenseTier::Enterprise,
            version: "2.1.0".to_string(),
            wasm_size: None,
            requires_heartbeat: true,
            icon: "Server".to_string(),
            capabilities: vec!["kali".to_string(), "pentest".to_string(), "tools".to_string()],
        },

        // Government tier (all features, compliance)
        ComponentMeta {
            id: "threat-intel-classified".to_string(),
            name: "Classified Threat Intel".to_string(),
            description: "Government-grade threat intelligence feeds".to_string(),
            category: "intel".to_string(),
            required_tier: LicenseTier::Government,
            version: "1.0.0".to_string(),
            wasm_size: None,
            requires_heartbeat: true,
            icon: "Shield".to_string(),
            capabilities: vec!["classified".to_string(), "intel".to_string(), "compliance".to_string()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_ordering() {
        assert!(LicenseTier::Government > LicenseTier::Enterprise);
        assert!(LicenseTier::Enterprise > LicenseTier::Pro);
        assert!(LicenseTier::Pro > LicenseTier::Free);
    }

    #[test]
    fn test_tier_access() {
        assert!(LicenseTier::Government.can_access(LicenseTier::Free));
        assert!(LicenseTier::Pro.can_access(LicenseTier::Free));
        assert!(!LicenseTier::Free.can_access(LicenseTier::Pro));
    }

    #[test]
    fn test_subscription_expiry() {
        let mut sub = Subscription {
            id: "test".to_string(),
            org_id: "org1".to_string(),
            tier: LicenseTier::Pro,
            started_at: 0,
            expires_at: Some(0), // Already expired
            active: true,
            seats: None,
            feature_overrides: HashSet::new(),
            component_overrides: HashSet::new(),
        };

        assert!(!sub.is_valid());

        // Set to future
        sub.expires_at = Some(u64::MAX);
        assert!(sub.is_valid());

        // Never expires
        sub.expires_at = None;
        assert!(sub.is_valid());
    }
}
