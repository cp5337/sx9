//! Economic Properties - Setup cost, maintenance cost, opportunity cost, depreciation
//! 
//! Core economic constraints for cognitive atoms in Universal Cognigraph.

use serde::{Deserialize, Serialize};

/// Economic Properties (from Universal Cognigraph)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicProperties {
    pub setup_cost: f32,        // c_s: Setup cost ∈ ℝ⁺
    pub maintenance_cost: f32,  // c_m: Maintenance cost rate ∈ ℝ⁺
    pub opportunity_cost: f32,  // c_o: Opportunity cost ∈ ℝ⁺
    pub depreciation_rate: f32, // c_d: Depreciation rate ∈ [0,1]
}

impl Default for EconomicProperties {
    fn default() -> Self {
        Self {
            setup_cost: 1.0,
            maintenance_cost: 0.1,
            opportunity_cost: 0.2,
            depreciation_rate: 0.05,
        }
    }
}

impl EconomicProperties {
    /// Calculate total cost over time period
    pub fn total_cost_over_time(&self, time_period: f32) -> f32 {
        self.setup_cost + 
        (self.maintenance_cost * time_period) +
        (self.opportunity_cost * time_period) +
        (self.depreciation_rate * time_period)
    }
}
