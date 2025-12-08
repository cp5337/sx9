//! Origin server implementation

use std::time::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::types::OriginStatus;

/// Origin server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginServer {
    pub id: Uuid,
    pub url: String,
    pub health_check_url: String,
    pub status: OriginStatus,
    pub last_health_check: DateTime<Utc>,
    pub response_time: Duration,
}

impl OriginServer {
    pub fn new(url: String, health_check_url: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            url,
            health_check_url,
            status: OriginStatus::Unknown,
            last_health_check: Utc::now(),
            response_time: Duration::from_millis(0),
        }
    }

    pub fn is_healthy(&self) -> bool {
        matches!(self.status, OriginStatus::Healthy)
    }
}
