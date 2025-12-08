// ctas7-quality-kit/src/eei.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EeiStatus { Unfilled, Partial, Filled, Expired, Reattempt }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EeiLog {
    pub eei_id: String,
    pub window_start: DateTime<Utc>,
    pub window_end: DateTime<Utc>,
    pub status: EeiStatus,
    pub buffer_until: Option<DateTime<Utc>>,
    pub handoff: Option<String>,
    pub source_hash: String,
    pub summary: String,
}

impl EeiLog {
    pub fn expire_if_needed(mut self, now: DateTime<Utc>) -> Self {
        if now > self.window_end && matches!(self.status, EeiStatus::Unfilled) {
            self.status = EeiStatus::Expired;
        }
        self
    }
}

pub fn append(path: &str, entry: &EeiLog) -> anyhow::Result<()> {
    let mut v: Vec<EeiLog> = if Path::new(path).exists() {
        serde_json::from_slice(&fs::read(path)?)?
    } else { vec![] };
    v.push(entry.clone());
    fs::write(path, serde_json::to_vec_pretty(&v)?)?;
    Ok(())
}

pub fn next_actions(entries: &[EeiLog], now: DateTime<Utc>) -> Vec<String> {
    entries.iter().filter_map(|e| {
        let e2 = e.clone().expire_if_needed(now);
        match e2.status {
            EeiStatus::Expired => Some(format!("pitch {} to {}",
                e2.eei_id, e2.handoff.clone().unwrap_or("nyx".into()))),
            EeiStatus::Unfilled => Some(format!("buffer {}", e2.eei_id)),
            _ => None,
        }
    }).collect()
}
