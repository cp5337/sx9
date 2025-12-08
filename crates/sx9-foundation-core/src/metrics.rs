//! CTAS-7 Line Analyzer Metrics Module
//! Real-time metrics tracking and quality scoring
//! Follows CTAS-7 standards: ≤200 LOC

use serde::{Deserialize, Serialize};
use super::core::{LineType, HalsteadContribution};

/// Real-time metrics during analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeMetrics {
    pub functions_found: usize,
    pub control_structures: usize,
    pub assignments: usize,
    pub warnings: usize,
    pub quality_score: f64,
}

impl RealTimeMetrics {
    /// Create new metrics tracker
    pub fn new() -> Self {
        Self {
            functions_found: 0,
            control_structures: 0,
            assignments: 0,
            warnings: 0,
            quality_score: 100.0,
        }
    }

    /// Update metrics based on line analysis
    pub fn update(&mut self, line_type: &LineType, complexity: usize, halstead: &HalsteadContribution) {
        match line_type {
            LineType::Function => self.functions_found += 1,
            LineType::Control => self.control_structures += 1,
            LineType::Assignment => self.assignments += 1,
            _ => {}
        }

        if complexity > 2 {
            self.warnings += 1;
            self.quality_score -= 2.0;
        }

        if halstead.operators.len() > 5 {
            self.quality_score -= 1.0;
        }

        self.quality_score = self.quality_score.max(0.0);
    }

    /// Calculate final quality grade
    pub fn get_quality_grade(&self) -> &'static str {
        if self.quality_score >= 95.0 {
            "A+"
        } else if self.quality_score >= 90.0 {
            "A"
        } else if self.quality_score >= 85.0 {
            "B+"
        } else if self.quality_score >= 80.0 {
            "B"
        } else if self.quality_score >= 75.0 {
            "C+"
        } else if self.quality_score >= 70.0 {
            "C"
        } else if self.quality_score >= 65.0 {
            "D+"
        } else if self.quality_score >= 60.0 {
            "D"
        } else {
            "F"
        }
    }

    /// Calculate Tesla engineering phase
    pub fn get_tesla_phase(&self) -> &'static str {
        let total_complexity = self.control_structures + self.functions_found;

        if total_complexity <= 10 {
            "Foundation"
        } else if total_complexity <= 30 {
            "Engine"
        } else if total_complexity <= 50 {
            "Integration"
        } else {
            "Application"
        }
    }

    /// Check if meets Tesla standards
    pub fn meets_tesla_standards(&self) -> bool {
        self.quality_score >= 85.0 && self.warnings <= 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_quality_grading() {
        let mut metrics = RealTimeMetrics::new();
        assert_eq!(metrics.get_quality_grade(), "A+");

        metrics.quality_score = 75.5;
        assert_eq!(metrics.get_quality_grade(), "C+");
    }

    #[test]
    fn test_tesla_phase_calculation() {
        let mut metrics = RealTimeMetrics::new();
        assert_eq!(metrics.get_tesla_phase(), "Foundation");

        metrics.functions_found = 15;
        assert_eq!(metrics.get_tesla_phase(), "Engine");
    }

    #[test]
    fn test_metrics_update() {
        let mut metrics = RealTimeMetrics::new();
        let mut operators = HashMap::new();
        operators.insert("+".to_string(), 3);

        let halstead = HalsteadContribution {
            operators,
            operands: HashMap::new(),
        };

        metrics.update(&LineType::Function, 1, &halstead);
        assert_eq!(metrics.functions_found, 1);
        assert_eq!(metrics.quality_score, 100.0);

        metrics.update(&LineType::Control, 5, &halstead);
        assert_eq!(metrics.control_structures, 1);
        assert!(metrics.quality_score < 100.0);
    }
}