//! PLASMA Delta Tuner Module
//!
//! Diagnostics and calibration system for Delta Operator.
//! Exposes delta measurements, logging, and feature flag control.

use super::delta_operator::{DeltaOperator, DeltaMeasurement};
use super::delta_gate::{DeltaGate, GatedPayload};
use crate::trivariate_hash_v731::{ContextFrame, TrivariateHash};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

/// Delta measurement log entry
#[derive(Debug, Clone)]
pub struct DeltaLogEntry {
    pub timestamp: u64,
    pub delta_angle: f32,
    pub entropy_drift: f32,
    pub semantic_drift: f32,
    pub noise_score: f32,
    pub gated_weight: Option<f32>,
    pub escalation_tier: String,
}

/// PLASMA Delta Tuner
pub struct DeltaTuner {
    operator: Arc<Mutex<DeltaOperator>>,
    log_buffer: Arc<Mutex<VecDeque<DeltaLogEntry>>>,
    max_log_entries: usize,
    enabled: bool,
}

impl DeltaTuner {
    /// Create new delta tuner
    pub fn new(enabled: bool) -> Self {
        Self {
            operator: Arc::new(Mutex::new(DeltaOperator::new(enabled))),
            log_buffer: Arc::new(Mutex::new(VecDeque::new())),
            max_log_entries: 1000,
            enabled,
        }
    }

    /// Measure delta and log result
    pub fn measure_and_log(
        &self,
        ctx1: &ContextFrame,
        ctx2: &ContextFrame,
        hash1: &TrivariateHash,
        hash2: &TrivariateHash,
        escalation_tier: &str,
    ) -> DeltaMeasurement {
        if !self.enabled {
            return DeltaMeasurement::new(0.0, 0.0, 0.0);
        }

        let operator = self.operator.lock().unwrap();
        let measurement = operator.measure_delta(ctx1, ctx2, hash1, hash2);

        // Log measurement
        self.log_measurement(&measurement, escalation_tier, None);

        measurement
    }

    /// Log delta measurement
    pub fn log_measurement(
        &self,
        measurement: &DeltaMeasurement,
        escalation_tier: &str,
        gated_weight: Option<f32>,
    ) {
        if !self.enabled {
            return;
        }

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let entry = DeltaLogEntry {
            timestamp,
            delta_angle: measurement.delta_angle,
            entropy_drift: measurement.entropy_drift,
            semantic_drift: measurement.semantic_drift,
            noise_score: measurement.noise_score,
            gated_weight,
            escalation_tier: escalation_tier.to_string(),
        };

        let mut log = self.log_buffer.lock().unwrap();
        log.push_back(entry);

        // Trim if over limit
        while log.len() > self.max_log_entries {
            log.pop_front();
        }
    }

    /// Get recent log entries
    pub fn get_recent_logs(&self, count: usize) -> Vec<DeltaLogEntry> {
        let log = self.log_buffer.lock().unwrap();
        log.iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    /// Get all log entries
    pub fn get_all_logs(&self) -> Vec<DeltaLogEntry> {
        let log = self.log_buffer.lock().unwrap();
        log.iter().cloned().collect()
    }

    /// Clear log buffer
    pub fn clear_logs(&self) {
        let mut log = self.log_buffer.lock().unwrap();
        log.clear();
    }

    /// Get statistics from logs
    pub fn get_statistics(&self) -> DeltaStatistics {
        let log = self.log_buffer.lock().unwrap();
        
        if log.is_empty() {
            return DeltaStatistics::default();
        }

        let mut stats = DeltaStatistics::default();
        let mut count = 0;

        for entry in log.iter() {
            stats.avg_delta_angle += entry.delta_angle;
            stats.avg_entropy_drift += entry.entropy_drift;
            stats.avg_semantic_drift += entry.semantic_drift;
            stats.avg_noise_score += entry.noise_score;
            
            if let Some(weight) = entry.gated_weight {
                stats.avg_gated_weight += weight;
                stats.gated_count += 1;
            }
            
            stats.max_delta_angle = stats.max_delta_angle.max(entry.delta_angle);
            stats.max_noise_score = stats.max_noise_score.max(entry.noise_score);
            
            count += 1;
        }

        if count > 0 {
            stats.avg_delta_angle /= count as f32;
            stats.avg_entropy_drift /= count as f32;
            stats.avg_semantic_drift /= count as f32;
            stats.avg_noise_score /= count as f32;
            
            if stats.gated_count > 0 {
                stats.avg_gated_weight /= stats.gated_count as f32;
            }
        }

        stats.total_measurements = count;
        stats
    }

    /// Enable/disable delta tuner
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        let mut operator = self.operator.lock().unwrap();
        *operator = DeltaOperator::new(enabled);
    }

    /// Check if tuner is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get operator reference (for direct use)
    pub fn operator(&self) -> Arc<Mutex<DeltaOperator>> {
        Arc::clone(&self.operator)
    }
}

/// Delta statistics
#[derive(Debug, Clone, Default)]
pub struct DeltaStatistics {
    pub total_measurements: usize,
    pub avg_delta_angle: f32,
    pub avg_entropy_drift: f32,
    pub avg_semantic_drift: f32,
    pub avg_noise_score: f32,
    pub avg_gated_weight: f32,
    pub max_delta_angle: f32,
    pub max_noise_score: f32,
    pub gated_count: usize,
}

impl Default for DeltaTuner {
    fn default() -> Self {
        Self::new(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trivariate_hash_v731::{ExecEnv, ExecState};

    #[test]
    fn test_measure_and_log() {
        let tuner = DeltaTuner::new(true);
        
        let ctx1 = ContextFrame::new(ExecEnv::Wasm, 1, ExecState::Hot);
        let ctx2 = ContextFrame::new(ExecEnv::Container, 2, ExecState::Warm);
        
        let hash1 = TrivariateHash::new(
            "aB7x9pQw2zRt4kMn".to_string(),
            "c5j8k3p2q7w1x9z".to_string(),
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        );
        
        let hash2 = TrivariateHash::new(
            "xY9mP4qR8sT2wN5k".to_string(),
            "d6k9l4p3q8w2x0z".to_string(),
            "660f9501-f3ac-52e5-b827-557766551111".to_string(),
        );

        let measurement = tuner.measure_and_log(&ctx1, &ctx2, &hash1, &hash2, "WASM->Microkernel");
        
        assert!(measurement.noise_score >= 0.0 && measurement.noise_score <= 1.0);
        
        let logs = tuner.get_recent_logs(10);
        assert!(!logs.is_empty());
        assert_eq!(logs[0].escalation_tier, "WASM->Microkernel");
    }

    #[test]
    fn test_statistics() {
        let tuner = DeltaTuner::new(true);
        
        // Add some synthetic measurements
        for i in 0..10 {
            let measurement = DeltaMeasurement::new(
                (i as f32) * 10.0,
                0.1 + (i as f32) * 0.05,
                0.2 + (i as f32) * 0.05,
            );
            tuner.log_measurement(&measurement, "test", Some(0.5));
        }

        let stats = tuner.get_statistics();
        assert_eq!(stats.total_measurements, 10);
        assert!(stats.avg_delta_angle > 0.0);
    }

    #[test]
    fn test_feature_flag() {
        let mut tuner = DeltaTuner::new(false);
        assert!(!tuner.is_enabled());
        
        tuner.set_enabled(true);
        assert!(tuner.is_enabled());
    }
}


