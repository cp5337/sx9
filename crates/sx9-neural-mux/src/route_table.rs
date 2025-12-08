//! Route Table - Lock-free concurrent route storage
//!
//! Uses DashMap for O(1) concurrent access without blocking

use std::time::{Duration, Instant};
use dashmap::DashMap;
use anyhow::{Result, anyhow};

use crate::router::{HashValue, RouteDestination};

/// A single route entry in the table
#[derive(Debug, Clone)]
pub struct RouteEntry {
    /// Primary key: SCH-T hash
    pub sch_t: HashValue,
    /// Secondary validation: CUID-T hash
    pub cuid_t: HashValue,
    /// Route destination
    pub destination: RouteDestination,
    /// Route priority (higher = preferred)
    pub priority: u8,
    /// Time-to-live
    pub ttl: Duration,
    /// Creation timestamp
    pub created_at: Instant,
    /// Last access timestamp
    pub last_accessed: Instant,
}

impl RouteEntry {
    /// Check if route has expired
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }

    /// Update last accessed time
    pub fn touch(&mut self) {
        self.last_accessed = Instant::now();
    }
}

/// Lock-free route table using DashMap
pub struct RouteTable {
    /// Primary index: SCH-T -> RouteEntry
    primary: DashMap<HashValue, RouteEntry>,
    /// Maximum capacity
    max_capacity: usize,
}

impl RouteTable {
    /// Create a new route table with given capacity
    pub fn new(max_capacity: usize) -> Self {
        Self {
            primary: DashMap::with_capacity(max_capacity / 4),
            max_capacity,
        }
    }

    /// O(1) route lookup
    #[inline]
    pub fn lookup(&self, sch_t: HashValue, _cuid_t: HashValue) -> Option<RouteEntry> {
        // Primary lookup by SCH-T
        self.primary.get(&sch_t).map(|entry| {
            // Clone to avoid holding the lock
            entry.value().clone()
        })
    }

    /// Insert a route entry
    pub fn insert(&self, entry: RouteEntry) -> Result<()> {
        if self.primary.len() >= self.max_capacity {
            return Err(anyhow!("Route table at capacity: {}", self.max_capacity));
        }

        self.primary.insert(entry.sch_t, entry);
        Ok(())
    }

    /// Remove a route by SCH-T
    pub fn remove(&self, sch_t: HashValue) -> Option<RouteEntry> {
        self.primary.remove(&sch_t).map(|(_, v)| v)
    }

    /// Get current table size
    pub fn len(&self) -> usize {
        self.primary.len()
    }

    /// Check if table is empty
    pub fn is_empty(&self) -> bool {
        self.primary.is_empty()
    }

    /// Evict expired routes
    pub fn evict_expired(&self) -> usize {
        let before = self.primary.len();
        self.primary.retain(|_, entry| !entry.is_expired());
        before - self.primary.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_table_insert_lookup() {
        let table = RouteTable::new(1000);

        let entry = RouteEntry {
            sch_t: 0x12345678,
            cuid_t: 0xABCDEF00,
            destination: RouteDestination::Local("test_handler".to_string()),
            priority: 10,
            ttl: Duration::from_secs(3600),
            created_at: Instant::now(),
            last_accessed: Instant::now(),
        };

        table.insert(entry.clone()).unwrap();
        assert_eq!(table.len(), 1);

        let result = table.lookup(0x12345678, 0xABCDEF00);
        assert!(result.is_some());
    }
}
