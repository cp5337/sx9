//! CONVERGE Event Window
//!
//! RFC Compliance:
//! - RFC-93X1 CONVERGE Core - Window constraints
//!
//! Deterministic windowing for action events:
//! - max_window_ms: 300000 (5 minutes)
//! - max_events_per_window: 4096
//!
//! No raw event persistence - set-based reasoning only.

use serde::{Deserialize, Serialize};
use sx9_foundation_core::data::{DateTime, Utc, Uuid};
use std::collections::VecDeque;

use crate::model::{ActionEvent, ConvergeError, HD4Phase, TacticalProfile};

/// Window configuration from smartcrate.toml
pub const MAX_WINDOW_MS: u64 = 300_000;      // 5 minutes
pub const MAX_EVENTS_PER_WINDOW: usize = 4096;

/// Event window for convergence detection
#[derive(Debug, Clone)]
pub struct EventWindow {
    /// Events in the window (FIFO)
    events: VecDeque<ActionEvent>,
    /// Window start time
    window_start: DateTime<Utc>,
    /// Maximum window duration in milliseconds
    max_duration_ms: u64,
    /// Maximum events allowed
    max_events: usize,
    /// Current HD4 phase
    hd4_phase: HD4Phase,
}

impl Default for EventWindow {
    fn default() -> Self {
        Self::new(MAX_WINDOW_MS, MAX_EVENTS_PER_WINDOW)
    }
}

impl EventWindow {
    pub fn new(max_duration_ms: u64, max_events: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(max_events),
            window_start: Utc::now(),
            max_duration_ms,
            max_events,
            hd4_phase: HD4Phase::Detect,
        }
    }

    /// Push event into window, enforcing constraints
    pub fn push(&mut self, event: ActionEvent) -> Result<(), ConvergeError> {
        // Expire old events first
        self.expire_old_events();

        // Check capacity
        if self.events.len() >= self.max_events {
            return Err(ConvergeError::WindowOverflow(self.events.len()));
        }

        self.events.push_back(event);
        Ok(())
    }

    /// Expire events older than window duration
    fn expire_old_events(&mut self) {
        let now = Utc::now();
        let cutoff_ns = (now.timestamp_nanos_opt().unwrap_or(0) as u64)
            .saturating_sub(self.max_duration_ms * 1_000_000);

        while let Some(front) = self.events.front() {
            if front.timestamp_ns < cutoff_ns {
                self.events.pop_front();
            } else {
                break;
            }
        }
    }

    /// Get current event count
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Check if window is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Reset window
    pub fn reset(&mut self) {
        self.events.clear();
        self.window_start = Utc::now();
    }

    /// Set HD4 phase (market state)
    pub fn set_hd4_phase(&mut self, phase: HD4Phase) {
        self.hd4_phase = phase;
    }

    /// Get current HD4 phase
    pub fn hd4_phase(&self) -> HD4Phase {
        self.hd4_phase
    }

    /// Get all events (for set-based reasoning)
    pub fn events(&self) -> &VecDeque<ActionEvent> {
        &self.events
    }

    /// Drain events by entity ID
    pub fn drain_by_entity(&mut self, entity_id: Uuid) -> Vec<ActionEvent> {
        let mut drained = Vec::new();
        self.events.retain(|e| {
            if e.entity_id == Some(entity_id) {
                drained.push(e.clone());
                false
            } else {
                true
            }
        });
        drained
    }

    /// Get aggregate tactical profile for window
    pub fn aggregate_tactical(&self) -> TacticalProfile {
        if self.events.is_empty() {
            return TacticalProfile::default();
        }

        let count = self.events.len() as f64;
        let (sum_p, sum_t, sum_h) = self.events.iter().fold(
            (0.0, 0.0, 0.0),
            |(p, t, h), e| (p + e.tactical.p, t + e.tactical.t, h + e.tactical.h),
        );

        TacticalProfile {
            p: sum_p / count,
            t: sum_t / count,
            h: sum_h / count,
        }
    }

    /// Get events by source (kali, wazuh, sensor, sim)
    pub fn events_by_source(&self, source: &str) -> Vec<&ActionEvent> {
        self.events.iter().filter(|e| e.source == source).collect()
    }
}

/// Window set for multi-source convergence
#[derive(Debug, Default)]
pub struct WindowSet {
    /// Blue book events (OSSEC/Wazuh defenders)
    pub blue: EventWindow,
    /// Red book events (Kali/TETH attackers)
    pub red: EventWindow,
    /// Sensor events (passive observation)
    pub sensor: EventWindow,
    /// Simulation events (training/testing)
    pub sim: EventWindow,
}

impl WindowSet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Route event to appropriate window by source
    pub fn route_event(&mut self, event: ActionEvent) -> Result<(), ConvergeError> {
        match event.source.as_str() {
            "wazuh" | "ossec" | "blue" => self.blue.push(event),
            "kali" | "teth" | "red" => self.red.push(event),
            "sensor" => self.sensor.push(event),
            "sim" => self.sim.push(event),
            _ => self.sensor.push(event), // Default to sensor
        }
    }

    /// Set HD4 phase across all windows
    pub fn set_hd4_phase(&mut self, phase: HD4Phase) {
        self.blue.set_hd4_phase(phase);
        self.red.set_hd4_phase(phase);
        self.sensor.set_hd4_phase(phase);
        self.sim.set_hd4_phase(phase);
    }

    /// Get total event count
    pub fn total_events(&self) -> usize {
        self.blue.len() + self.red.len() + self.sensor.len() + self.sim.len()
    }

    /// Reset all windows
    pub fn reset_all(&mut self) {
        self.blue.reset();
        self.red.reset();
        self.sensor.reset();
        self.sim.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sx9_foundation_core::data::serde_json::json;

    fn make_event(source: &str, entity_id: Option<Uuid>) -> ActionEvent {
        ActionEvent {
            id: Uuid::new_v4(),
            source: source.to_string(),
            event_type: "test".to_string(),
            entity_id,
            timestamp_ns: Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            payload: json!({}),
            tactical: TacticalProfile::default(),
        }
    }

    #[test]
    fn test_window_push() {
        let mut window = EventWindow::default();
        let event = make_event("wazuh", None);
        assert!(window.push(event).is_ok());
        assert_eq!(window.len(), 1);
    }

    #[test]
    fn test_window_overflow() {
        let mut window = EventWindow::new(300_000, 2);
        assert!(window.push(make_event("wazuh", None)).is_ok());
        assert!(window.push(make_event("wazuh", None)).is_ok());
        assert!(matches!(
            window.push(make_event("wazuh", None)),
            Err(ConvergeError::WindowOverflow(_))
        ));
    }

    #[test]
    fn test_window_set_routing() {
        let mut ws = WindowSet::new();
        ws.route_event(make_event("wazuh", None)).unwrap();
        ws.route_event(make_event("kali", None)).unwrap();
        ws.route_event(make_event("sensor", None)).unwrap();

        assert_eq!(ws.blue.len(), 1);
        assert_eq!(ws.red.len(), 1);
        assert_eq!(ws.sensor.len(), 1);
        assert_eq!(ws.total_events(), 3);
    }

    #[test]
    fn test_aggregate_tactical() {
        let mut window = EventWindow::default();
        let mut e1 = make_event("test", None);
        e1.tactical = TacticalProfile { p: 0.8, t: 0.6, h: 0.4 };
        let mut e2 = make_event("test", None);
        e2.tactical = TacticalProfile { p: 0.6, t: 0.8, h: 0.6 };

        window.push(e1).unwrap();
        window.push(e2).unwrap();

        let agg = window.aggregate_tactical();
        assert!((agg.p - 0.7).abs() < 0.001);
        assert!((agg.t - 0.7).abs() < 0.001);
        assert!((agg.h - 0.5).abs() < 0.001);
    }
}
