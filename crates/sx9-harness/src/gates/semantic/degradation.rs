//! Certification Degradation Tracking
//!
//! Tracks certification changes when updates are installed.
//! Provides accountability for quality changes over time.
//!
//! Per user requirement: "old installations hanging on is an
//! invitation to vulnerabilities" - degradation tracking forces
//! updates by surfacing quality decline.
//!
//! # Degradation Causes
//!
//! | Cause              | Description                          |
//! |--------------------|--------------------------------------|
//! | NewDrift           | New drift detected in update         |
//! | DriftWorsened      | Existing drift score increased       |
//! | StaticFailure      | Static QA gate failed                |
//! | HeartbeatFailure   | Dual heartbeat check failed          |
//! | AnnotationLoss     | N-V-N-N annotation coverage dropped  |
//! | ArchViolation      | Architecture rule violated           |
//! | PatternViolation   | Canonical pattern violated           |
//! | DependencyRisk     | Vulnerable dependency introduced     |
//! | Expired            | Certification expired without renewal|

use serde::{Deserialize, Serialize};
use sx9_foundation_core::data::{DateTime, Utc};

use super::certification::{CertificationLevel, QualityCertification};

// ============================================================================
// DELTA DIRECTION
// ============================================================================

/// Direction of certification change
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeltaDirection {
    /// Certification improved
    Upgraded,
    /// Certification stayed same
    Maintained,
    /// Certification dropped
    Degraded,
    /// Certification revoked entirely
    Revoked,
}

// ============================================================================
// DEGRADATION CAUSE
// ============================================================================

/// Cause of certification degradation
///
/// Each cause includes relevant details for diagnostics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradationCause {
    /// New drift detected in update
    NewDrift { vector: String, score: f32 },
    /// Existing drift worsened
    DriftWorsened { vector: String, delta: f32 },
    /// Static QA failure
    StaticFailure { findings: u32 },
    /// Heartbeat check failed
    HeartbeatFailure,
    /// Annotation coverage dropped
    AnnotationLoss { previous: u8, current: u8 },
    /// Architecture violation introduced
    ArchViolation { rule: String },
    /// Pattern violation
    PatternViolation { pattern: String },
    /// Dependency introduced vulnerability
    DependencyRisk { package: String, severity: String },
    /// Certification expired without renewal
    Expired,
}

// ============================================================================
// UPDATE TRIGGER
// ============================================================================

/// What triggered the certification check
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateTrigger {
    /// New code commit
    Commit { sha: String, author: String },
    /// Dependency update
    DependencyUpdate { packages: Vec<String> },
    /// Configuration change
    ConfigChange { file: String },
    /// Scheduled re-certification
    Scheduled,
    /// Manual re-check requested
    Manual { requester: String },
    /// CI/CD pipeline
    Pipeline { run_id: String },
}

// ============================================================================
// CERTIFICATION DELTA
// ============================================================================

/// Tracks certification changes when updates are installed
///
/// Compares before/after certification and identifies causes of any degradation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificationDelta {
    /// Previous certification level
    pub previous_level: CertificationLevel,
    /// New certification level after update
    pub new_level: CertificationLevel,
    /// Direction of change
    pub direction: DeltaDirection,
    /// Score change (positive = improvement)
    pub score_delta: i8,
    /// Specific degradation causes
    pub causes: Vec<DegradationCause>,
    /// Update that triggered the change
    pub trigger: UpdateTrigger,
    /// Timestamp
    pub occurred_at: DateTime<Utc>,
}

impl CertificationDelta {
    /// Create delta from before/after certification
    pub fn compare(
        previous: &QualityCertification,
        current: &QualityCertification,
        trigger: UpdateTrigger,
    ) -> Self {
        let direction = if current.level > previous.level {
            DeltaDirection::Upgraded
        } else if current.level < previous.level {
            if current.level == CertificationLevel::None {
                DeltaDirection::Revoked
            } else {
                DeltaDirection::Degraded
            }
        } else {
            DeltaDirection::Maintained
        };

        let score_delta = current.score as i8 - previous.score as i8;

        // Analyze causes of degradation
        let causes = Self::analyze_causes(previous, current);

        Self {
            previous_level: previous.level,
            new_level: current.level,
            direction,
            score_delta,
            causes,
            trigger,
            occurred_at: Utc::now(),
        }
    }

    /// Analyze what caused the degradation
    fn analyze_causes(
        previous: &QualityCertification,
        current: &QualityCertification,
    ) -> Vec<DegradationCause> {
        let mut causes = Vec::new();

        // Check for new drift
        if current.drift_metrics.total_signals > previous.drift_metrics.total_signals {
            causes.push(DegradationCause::NewDrift {
                vector: current.drift_metrics.vectors_detected.first()
                    .cloned()
                    .unwrap_or_else(|| "unknown".to_string()),
                score: current.drift_metrics.max_score,
            });
        }

        // Check for drift worsening
        if current.drift_metrics.max_score > previous.drift_metrics.max_score {
            let delta = current.drift_metrics.max_score - previous.drift_metrics.max_score;
            if delta > 0.1 {
                causes.push(DegradationCause::DriftWorsened {
                    vector: "aggregate".to_string(),
                    delta,
                });
            }
        }

        // Check static QA
        if !current.gates.static_passed && previous.gates.static_passed {
            causes.push(DegradationCause::StaticFailure { findings: 1 });
        }

        // Check heartbeat
        if !current.gates.heartbeat_passed && previous.gates.heartbeat_passed {
            causes.push(DegradationCause::HeartbeatFailure);
        }

        // Check annotation coverage
        if current.drift_metrics.annotation_coverage < previous.drift_metrics.annotation_coverage {
            causes.push(DegradationCause::AnnotationLoss {
                previous: previous.drift_metrics.annotation_coverage,
                current: current.drift_metrics.annotation_coverage,
            });
        }

        // Check arch gate
        if !current.gates.arch_passed && previous.gates.arch_passed {
            causes.push(DegradationCause::ArchViolation {
                rule: "unknown".to_string(),
            });
        }

        causes
    }

    /// Check if this is a degradation
    pub fn is_degradation(&self) -> bool {
        matches!(self.direction, DeltaDirection::Degraded | DeltaDirection::Revoked)
    }

    /// Generate alert message for degradation
    ///
    /// Returns None if not a degradation.
    pub fn alert_message(&self) -> Option<String> {
        if !self.is_degradation() {
            return None;
        }

        let causes_text: Vec<String> = self.causes.iter().map(|c| {
            match c {
                DegradationCause::NewDrift { vector, score } =>
                    format!("New {} drift (score: {:.0}%)", vector, score * 100.0),
                DegradationCause::DriftWorsened { vector, delta } =>
                    format!("{} drift worsened by {:.0}%", vector, delta * 100.0),
                DegradationCause::StaticFailure { findings } =>
                    format!("{} static analysis findings", findings),
                DegradationCause::HeartbeatFailure =>
                    "Dual heartbeat check failed".to_string(),
                DegradationCause::AnnotationLoss { previous, current } =>
                    format!("Annotation coverage dropped {}% → {}%", previous, current),
                DegradationCause::ArchViolation { rule } =>
                    format!("Architecture violation: {}", rule),
                DegradationCause::PatternViolation { pattern } =>
                    format!("Pattern violation: {}", pattern),
                DegradationCause::DependencyRisk { package, severity } =>
                    format!("Dependency risk in {}: {}", package, severity),
                DegradationCause::Expired =>
                    "Certification expired".to_string(),
            }
        }).collect();

        Some(format!(
            "⚠️ CERTIFICATION DEGRADED: {} → {} | Causes: {}",
            self.previous_level.badge(),
            self.new_level.badge(),
            causes_text.join(", ")
        ))
    }

    /// Generate NATS subject for this delta
    pub fn nats_subject(&self) -> String {
        match self.direction {
            DeltaDirection::Upgraded => "sx9.qa.certification.upgraded".to_string(),
            DeltaDirection::Maintained => "sx9.qa.certification.maintained".to_string(),
            DeltaDirection::Degraded => "sx9.qa.certification.degraded".to_string(),
            DeltaDirection::Revoked => "sx9.qa.certification.revoked".to_string(),
        }
    }
}

// ============================================================================
// TREND HEALTH
// ============================================================================

/// Overall trend health
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrendHealth {
    /// Consistently high quality (<5% degradation rate)
    Excellent,
    /// Generally good, minor fluctuations (<15%)
    #[default]
    Good,
    /// Some concerns, needs attention (<30%)
    Warning,
    /// Frequent degradations, intervention needed (30%+)
    Critical,
}

/// Trend direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrendDirection {
    /// Quality improving
    Improving,
    /// Quality stable
    #[default]
    Stable,
    /// Quality declining
    Declining,
}

// ============================================================================
// CERTIFICATION TREND
// ============================================================================

/// Certification health trend over time
///
/// Tracks the last 30 deltas to identify patterns in quality.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CertificationTrend {
    /// Recent deltas (last 30 certifications)
    pub recent_deltas: Vec<CertificationDelta>,
    /// Degradation count in window
    pub degradations_in_window: u32,
    /// Upgrades in window
    pub upgrades_in_window: u32,
    /// Current health status
    pub health: TrendHealth,
    /// Trend direction
    pub direction: TrendDirection,
}

impl CertificationTrend {
    /// Add a new delta and recalculate trend
    pub fn record(&mut self, delta: CertificationDelta) {
        // Keep last 30 deltas
        if self.recent_deltas.len() >= 30 {
            self.recent_deltas.remove(0);
        }

        // Update counters
        match delta.direction {
            DeltaDirection::Degraded | DeltaDirection::Revoked => {
                self.degradations_in_window += 1;
            }
            DeltaDirection::Upgraded => {
                self.upgrades_in_window += 1;
            }
            _ => {}
        }

        self.recent_deltas.push(delta);

        // Recalculate health and direction
        self.recalculate();
    }

    /// Recalculate health and direction based on recent deltas
    fn recalculate(&mut self) {
        let total = self.recent_deltas.len() as u32;
        if total == 0 {
            return;
        }

        // Calculate health based on degradation rate
        let degradation_rate = self.degradations_in_window as f32 / total as f32;
        self.health = if degradation_rate < 0.05 {
            TrendHealth::Excellent
        } else if degradation_rate < 0.15 {
            TrendHealth::Good
        } else if degradation_rate < 0.30 {
            TrendHealth::Warning
        } else {
            TrendHealth::Critical
        };

        // Calculate direction from recent scores
        if self.recent_deltas.len() >= 5 {
            let recent_avg: i32 = self.recent_deltas.iter()
                .rev()
                .take(5)
                .map(|d| d.score_delta as i32)
                .sum::<i32>() / 5;

            self.direction = if recent_avg > 2 {
                TrendDirection::Improving
            } else if recent_avg < -2 {
                TrendDirection::Declining
            } else {
                TrendDirection::Stable
            };
        }
    }

    /// Get trend emoji
    pub fn emoji(&self) -> &'static str {
        match (&self.health, &self.direction) {
            (TrendHealth::Excellent, TrendDirection::Improving) => "🚀",
            (TrendHealth::Excellent, _) => "✨",
            (TrendHealth::Good, TrendDirection::Improving) => "📈",
            (TrendHealth::Good, TrendDirection::Declining) => "📉",
            (TrendHealth::Good, _) => "✅",
            (TrendHealth::Warning, TrendDirection::Improving) => "⚠️📈",
            (TrendHealth::Warning, _) => "⚠️",
            (TrendHealth::Critical, _) => "🚨",
        }
    }

    /// Generate trend summary
    pub fn summary(&self) -> String {
        format!(
            "{} {:?} | {} degradations / {} upgrades in last {} checks",
            self.emoji(),
            self.health,
            self.degradations_in_window,
            self.upgrades_in_window,
            self.recent_deltas.len()
        )
    }
}
