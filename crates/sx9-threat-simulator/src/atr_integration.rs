//! # Atomic Red Team Integration Module
//!
//! I integrate with Atomic Red Team for executing atomic tests
//! within CTAS threat emulation scenarios.

use crate::EmulationError;
use serde::{Deserialize, Serialize};

/// I integrate with Atomic Red Team
#[derive(Debug)]
pub struct AtrIntegration;

impl AtrIntegration {
    pub async fn new() -> Result<Self, EmulationError> {
        Ok(Self)
    }
}

/// I represent ATR atomic tests
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AtomicTest;
