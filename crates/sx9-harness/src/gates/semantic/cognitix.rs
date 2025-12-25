//! Cognitix Lifetime Metrics
//!
//! Cumulative QA metrics for sales and marketing purposes.
//! Tracks total heartbeat checks, testing hours, and certifications
//! across the lifetime of an installation.
//!
//! # Marketing Value
//!
//! Cognitix badges provide social proof of quality:
//! - "1M+ heartbeat checks"
//! - "10K hours of testing"
//! - "Peak: Diamond certified"
//!
//! These metrics cannot be reset and accumulate over time.

use serde::{Deserialize, Serialize};
use sx9_foundation_core::data::{DateTime, Utc};

use super::certification::{CertificationLevel, QualityCertification};

// ============================================================================
// CERTIFICATION COUNTS
// ============================================================================

/// Count of certifications by level
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CertificationCounts {
    /// Bronze certifications issued
    pub bronze: u32,
    /// Silver certifications issued
    pub silver: u32,
    /// Gold certifications issued
    pub gold: u32,
    /// Platinum certifications issued
    pub platinum: u32,
    /// Diamond certifications issued
    pub diamond: u32,
}

// ============================================================================
// COGNITIX LIFETIME METRICS
// ============================================================================

/// Cumulative certification metrics for sales benefits
///
/// "Cognitix Certified - 1M+ heartbeat checks, 10K hours of testing"
///
/// These metrics accumulate over the lifetime of the installation
/// and provide social proof of quality assurance rigor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitixLifetimeMetrics {
    /// Total dual heartbeat checks performed
    pub total_heartbeat_checks: u64,

    /// Total hours of QA testing
    pub total_testing_hours: f64,

    /// Total lines of code analyzed
    pub total_lines_analyzed: u64,

    /// Total drift signals detected and resolved
    pub total_drift_resolved: u64,

    /// Total certifications issued
    pub total_certifications: u64,

    /// Certifications by level
    pub certifications_by_level: CertificationCounts,

    /// Highest certification achieved
    pub peak_certification: CertificationLevel,

    /// Current streak (consecutive successful certifications)
    pub current_streak: u32,

    /// Longest streak ever
    pub longest_streak: u32,

    /// First certification date
    pub member_since: Option<DateTime<Utc>>,

    /// Last certification date
    pub last_certified: Option<DateTime<Utc>>,
}

impl Default for CognitixLifetimeMetrics {
    fn default() -> Self {
        Self {
            total_heartbeat_checks: 0,
            total_testing_hours: 0.0,
            total_lines_analyzed: 0,
            total_drift_resolved: 0,
            total_certifications: 0,
            certifications_by_level: CertificationCounts::default(),
            peak_certification: CertificationLevel::None,
            current_streak: 0,
            longest_streak: 0,
            member_since: None,
            last_certified: None,
        }
    }
}

impl CognitixLifetimeMetrics {
    /// Record a new certification
    ///
    /// Updates all relevant counters and streak tracking.
    pub fn record_certification(&mut self, cert: &QualityCertification) {
        let now = Utc::now();

        // Update totals
        self.total_certifications += 1;
        if cert.gates.heartbeat_passed {
            self.total_heartbeat_checks += 1;
        }

        // Update level counts
        match cert.level {
            CertificationLevel::Bronze => self.certifications_by_level.bronze += 1,
            CertificationLevel::Silver => self.certifications_by_level.silver += 1,
            CertificationLevel::Gold => self.certifications_by_level.gold += 1,
            CertificationLevel::Platinum => self.certifications_by_level.platinum += 1,
            CertificationLevel::Diamond => self.certifications_by_level.diamond += 1,
            CertificationLevel::None => {}
        }

        // Update peak
        if cert.level > self.peak_certification {
            self.peak_certification = cert.level;
        }

        // Update streak (Silver or above maintains streak)
        if cert.level >= CertificationLevel::Silver {
            self.current_streak += 1;
            if self.current_streak > self.longest_streak {
                self.longest_streak = self.current_streak;
            }
        } else {
            self.current_streak = 0;
        }

        // Update timestamps
        if self.member_since.is_none() {
            self.member_since = Some(now);
        }
        self.last_certified = Some(now);
    }

    /// Generate marketing summary
    ///
    /// Returns a concise string suitable for badges and marketing materials.
    pub fn marketing_summary(&self) -> String {
        let checks = format_large_number(self.total_heartbeat_checks);
        let hours = format_hours(self.total_testing_hours);

        format!(
            "Cognitix Certified | {} heartbeat checks | {} hours testing | {} certifications | Peak: {}",
            checks,
            hours,
            self.total_certifications,
            self.peak_certification.badge()
        )
    }

    /// Generate badge for display
    pub fn cognitix_badge(&self) -> CognitixBadge {
        CognitixBadge {
            level: self.peak_certification,
            heartbeat_checks: self.total_heartbeat_checks,
            testing_hours: self.total_testing_hours,
            streak: self.current_streak,
            member_since: self.member_since,
        }
    }
}

// ============================================================================
// COGNITIX BADGE
// ============================================================================

/// Displayable Cognitix badge
///
/// Embeddable badge for README files, websites, and marketing materials.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitixBadge {
    /// Highest certification level achieved
    pub level: CertificationLevel,
    /// Total heartbeat checks performed
    pub heartbeat_checks: u64,
    /// Total testing hours
    pub testing_hours: f64,
    /// Current certification streak
    pub streak: u32,
    /// Member since date
    pub member_since: Option<DateTime<Utc>>,
}

impl CognitixBadge {
    /// Format as embeddable badge text
    pub fn as_text(&self) -> String {
        format!(
            "🛡️ COGNITIX {} | {}+ checks | Streak: {}",
            self.level.badge(),
            self.format_checks(),
            self.streak
        )
    }

    /// Format as markdown badge
    pub fn as_markdown(&self) -> String {
        format!(
            "![Cognitix {}](https://cognitix.synaptix9.com/badge/{}/{})",
            self.level.badge(),
            format!("{:?}", self.level).to_lowercase(),
            self.heartbeat_checks
        )
    }

    /// Format checks for display
    fn format_checks(&self) -> String {
        format_large_number(self.heartbeat_checks)
    }
}

// ============================================================================
// FORMATTING HELPERS
// ============================================================================

/// Format large numbers for marketing display
fn format_large_number(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M+", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{}K+", n / 1_000)
    } else {
        format!("{}", n)
    }
}

/// Format hours for marketing display
fn format_hours(hours: f64) -> String {
    if hours >= 1000.0 {
        format!("{:.1}K", hours / 1000.0)
    } else {
        format!("{:.0}", hours)
    }
}
