use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrateStatus {
    Production,      // ✅ Production ready
    Staging,         // 🚧 In staging/testing
    Development,     // 🔧 Active development
    Experimental,    // 🧪 Experimental/proof of concept
    Deprecated,      // ⚠️ Deprecated/legacy
    Broken,          // ❌ Broken/shit code
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrateLocation {
    Foundation,      // Foundation crates (core system)
    Candidate,       // Candidate crates (staging)
    Shipyard,        // Shipyard (development)
    Legacy,          // Legacy/archived
    External,        // External dependencies
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateMetrics {
    pub lines_of_code: u32,
    pub quality_score: f64,  // 0-100
    pub test_coverage: f64,  // 0-100
    pub documentation: f64,  // 0-100
    pub complexity: u32,
    pub last_updated: DateTime<Utc>,
    pub commit_frequency: u32, // commits per month
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateInfo {
    pub name: String,
    pub status: CrateStatus,
    pub location: CrateLocation,
    pub metrics: CrateMetrics,
    pub description: String,
    pub maintainer: String,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
    pub issues: Vec<String>,
    pub achievements: Vec<String>,
}

pub struct CrateLeaderboard {
    crates: Vec<CrateInfo>,
    last_updated: DateTime<Utc>,
}

impl CrateLeaderboard {
    pub fn new() -> Self {
        let mut leaderboard = Self {
            crates: Vec::new(),
            last_updated: Utc::now(),
        };
        leaderboard.initialize_ctas_crates();
        leaderboard
    }

    fn initialize_ctas_crates(&mut self) {
        // Foundation Crates (Production Ready)
        self.add_crate(CrateInfo {
            name: "ctas7-core-foundation".to_string(),
            status: CrateStatus::Production,
            location: CrateLocation::Foundation,
            metrics: CrateMetrics {
                lines_of_code: 15420,
                quality_score: 95.0,
                test_coverage: 89.0,
                documentation: 92.0,
                complexity: 345,
                last_updated: Utc::now(),
                commit_frequency: 23,
            },
            description: "Core algorithms and shared functionality".to_string(),
            maintainer: "CTAS Engineering".to_string(),
            dependencies: vec!["tokio".to_string(), "serde".to_string()],
            dependents: vec!["ctas7-interface-foundation".to_string()],
            issues: vec![],
            achievements: vec!["🏆 Zero critical bugs".to_string(), "⚡ Sub-10ms response time".to_string()],
        });

        self.add_crate(CrateInfo {
            name: "ctas7-interface-foundation".to_string(),
            status: CrateStatus::Production,
            location: CrateLocation::Foundation,
            metrics: CrateMetrics {
                lines_of_code: 8930,
                quality_score: 92.0,
                test_coverage: 85.0,
                documentation: 88.0,
                complexity: 234,
                last_updated: Utc::now(),
                commit_frequency: 18,
            },
            description: "API interfaces and HTTP servers".to_string(),
            maintainer: "CTAS Engineering".to_string(),
            dependencies: vec!["axum".to_string(), "ctas7-core-foundation".to_string()],
            dependents: vec!["ctas7-statistical-analysis-cdn".to_string()],
            issues: vec![],
            achievements: vec!["🌐 High throughput API".to_string(), "🔒 Security hardened".to_string()],
        });

        // Candidate Crates (Active Development)
        self.add_crate(CrateInfo {
            name: "ctas7-statistical-analysis-cdn".to_string(),
            status: CrateStatus::Staging,
            location: CrateLocation::Candidate,
            metrics: CrateMetrics {
                lines_of_code: 41741,
                quality_score: 88.0,
                test_coverage: 75.0,
                documentation: 82.0,
                complexity: 567,
                last_updated: Utc::now(),
                commit_frequency: 45,
            },
            description: "Enhanced statistical analysis with AI integration".to_string(),
            maintainer: "CTAS AI Team".to_string(),
            dependencies: vec!["axum".to_string(), "tokio".to_string(), "petgraph".to_string()],
            dependents: vec![],
            issues: vec!["Need Ollama integration testing".to_string()],
            achievements: vec!["🧠 Phi + GNN integration".to_string(), "📊 Academic-grade analysis".to_string()],
        });

        self.add_crate(CrateInfo {
            name: "ctas-hashing-engine".to_string(),
            status: CrateStatus::Production,
            location: CrateLocation::Candidate,
            metrics: CrateMetrics {
                lines_of_code: 5234,
                quality_score: 98.0,
                test_coverage: 95.0,
                documentation: 90.0,
                complexity: 123,
                last_updated: Utc::now(),
                commit_frequency: 12,
            },
            description: "Genetic hash algorithm with 1,146x compression".to_string(),
            maintainer: "CTAS Crypto Team".to_string(),
            dependencies: vec!["blake3".to_string()],
            dependents: vec!["ctas7-statistical-analysis-cdn".to_string()],
            issues: vec![],
            achievements: vec!["🚀 1,146x compression ratio".to_string(), "⚡ Ultra-fast hashing".to_string()],
        });

        self.add_crate(CrateInfo {
            name: "ctas7-real-port-manager".to_string(),
            status: CrateStatus::Development,
            location: CrateLocation::Candidate,
            metrics: CrateMetrics {
                lines_of_code: 12450,
                quality_score: 72.0,
                test_coverage: 68.0,
                documentation: 65.0,
                complexity: 456,
                last_updated: Utc::now(),
                commit_frequency: 34,
            },
            description: "Port management service (18103)".to_string(),
            maintainer: "CTAS Network Team".to_string(),
            dependencies: vec!["tokio".to_string(), "axum".to_string()],
            dependents: vec![],
            issues: vec!["Health check endpoint failing".to_string(), "Memory leaks in connection pool".to_string()],
            achievements: vec!["🌐 Multi-port support".to_string()],
        });

        // Experimental/Shit Code
        self.add_crate(CrateInfo {
            name: "ctas-qa-system".to_string(),
            status: CrateStatus::Broken,
            location: CrateLocation::Candidate,
            metrics: CrateMetrics {
                lines_of_code: 8920,
                quality_score: 35.0,
                test_coverage: 25.0,
                documentation: 15.0,
                complexity: 789,
                last_updated: Utc::now(),
                commit_frequency: 5,
            },
            description: "Quality assurance automation (BROKEN)".to_string(),
            maintainer: "NEEDS OWNER".to_string(),
            dependencies: vec!["old_deps".to_string()],
            dependents: vec![],
            issues: vec![
                "❌ Compilation failures".to_string(),
                "❌ No tests passing".to_string(),
                "❌ Deprecated dependencies".to_string(),
                "❌ No documentation".to_string()
            ],
            achievements: vec![],
        });

        // Legacy Archive
        self.add_crate(CrateInfo {
            name: "ctas-6-6-mono-hashing-engine".to_string(),
            status: CrateStatus::Deprecated,
            location: CrateLocation::Legacy,
            metrics: CrateMetrics {
                lines_of_code: 3456,
                quality_score: 60.0,
                test_coverage: 45.0,
                documentation: 70.0,
                complexity: 234,
                last_updated: Utc::now(),
                commit_frequency: 0,
            },
            description: "Legacy hashing engine (replaced by genetic hash)".to_string(),
            maintainer: "ARCHIVED".to_string(),
            dependencies: vec!["old_crypto".to_string()],
            dependents: vec![],
            issues: vec!["⚠️ Deprecated - use ctas-hashing-engine instead".to_string()],
            achievements: vec!["🏛️ Foundation for genetic hash algorithm".to_string()],
        });
    }

    fn add_crate(&mut self, crate_info: CrateInfo) {
        self.crates.push(crate_info);
    }

    pub fn display_leaderboard(&self) {
        self.print_header();
        self.print_production_crates();
        self.print_development_crates();
        self.print_problem_crates();
        self.print_legacy_crates();
        self.print_summary_stats();
    }

    fn print_header(&self) {
        println!("╔═══════════════════════════════════════════════════════════════════════════════╗");
        println!("║                         🏆 CTAS CRATE LEADERBOARD                             ║");
        println!("║                        Last Updated: {}                ║",
                 self.last_updated.format("%Y-%m-%d %H:%M UTC"));
        println!("╚═══════════════════════════════════════════════════════════════════════════════╝");
        println!();
    }

    fn print_production_crates(&self) {
        println!("🏆 PRODUCTION READY CRATES");
        println!("═══════════════════════════════════════════════════════════════════════════════");

        let production_crates: Vec<_> = self.crates.iter()
            .filter(|c| matches!(c.status, CrateStatus::Production))
            .collect();

        for crate_info in production_crates {
            self.print_crate_row(crate_info, "✅");
        }
        println!();
    }

    fn print_development_crates(&self) {
        println!("🚧 DEVELOPMENT & STAGING CRATES");
        println!("═══════════════════════════════════════════════════════════════════════════════");

        let dev_crates: Vec<_> = self.crates.iter()
            .filter(|c| matches!(c.status, CrateStatus::Development | CrateStatus::Staging | CrateStatus::Experimental))
            .collect();

        for crate_info in dev_crates {
            let icon = match crate_info.status {
                CrateStatus::Staging => "🚧",
                CrateStatus::Development => "🔧",
                CrateStatus::Experimental => "🧪",
                _ => "⚠️",
            };
            self.print_crate_row(crate_info, icon);
        }
        println!();
    }

    fn print_problem_crates(&self) {
        println!("❌ PROBLEM CRATES (SHIT CODE)");
        println!("═══════════════════════════════════════════════════════════════════════════════");

        let problem_crates: Vec<_> = self.crates.iter()
            .filter(|c| matches!(c.status, CrateStatus::Broken))
            .collect();

        for crate_info in problem_crates {
            self.print_crate_row(crate_info, "❌");
            for issue in &crate_info.issues {
                println!("    └─ {}", issue);
            }
        }
        println!();
    }

    fn print_legacy_crates(&self) {
        println!("🏛️ LEGACY/ARCHIVED CRATES");
        println!("═══════════════════════════════════════════════════════════════════════════════");

        let legacy_crates: Vec<_> = self.crates.iter()
            .filter(|c| matches!(c.status, CrateStatus::Deprecated))
            .collect();

        for crate_info in legacy_crates {
            self.print_crate_row(crate_info, "⚠️");
        }
        println!();
    }

    fn print_crate_row(&self, crate_info: &CrateInfo, icon: &str) {
        let quality_bar = self.create_quality_bar(crate_info.metrics.quality_score);
        let location_badge = match crate_info.location {
            CrateLocation::Foundation => "FOUNDATION",
            CrateLocation::Candidate => "CANDIDATE",
            CrateLocation::Shipyard => "SHIPYARD",
            CrateLocation::Legacy => "LEGACY",
            CrateLocation::External => "EXTERNAL",
        };

        println!("{} {:35} │ {} │ {:>8} │ {:>6} LOC │ {}",
            icon,
            truncate_string(&crate_info.name, 35),
            quality_bar,
            location_badge,
            crate_info.metrics.lines_of_code,
            truncate_string(&crate_info.description, 30)
        );

        if !crate_info.achievements.is_empty() {
            let achievements = crate_info.achievements.join(" ");
            println!("    └─ {}", truncate_string(&achievements, 70));
        }
    }

    fn create_quality_bar(&self, score: f64) -> String {
        let width = 10;
        let filled = (score / 100.0 * width as f64) as usize;
        let char = if score >= 90.0 { "█" } else if score >= 70.0 { "▓" } else { "░" };
        let bar = char.repeat(filled) + &"░".repeat(width - filled);
        format!("{} {:>3.0}%", bar, score)
    }

    fn print_summary_stats(&self) {
        let total_crates = self.crates.len();
        let production_count = self.crates.iter().filter(|c| matches!(c.status, CrateStatus::Production)).count();
        let broken_count = self.crates.iter().filter(|c| matches!(c.status, CrateStatus::Broken)).count();
        let total_loc: u32 = self.crates.iter().map(|c| c.metrics.lines_of_code).sum();
        let avg_quality: f64 = self.crates.iter().map(|c| c.metrics.quality_score).sum::<f64>() / total_crates as f64;

        println!("📊 SUMMARY STATISTICS");
        println!("═══════════════════════════════════════════════════════════════════════════════");
        println!("  Total Crates: {}  │  Production Ready: {}  │  Broken: {}  │  Success Rate: {:.1}%",
            total_crates, production_count, broken_count,
            (production_count as f64 / total_crates as f64) * 100.0);
        println!("  Total Lines of Code: {:,}  │  Average Quality Score: {:.1}/100",
            total_loc, avg_quality);
        println!("  🎯 CTAS-7 Codebase Health: {} {}",
            if avg_quality >= 80.0 { "EXCELLENT" } else if avg_quality >= 60.0 { "GOOD" } else { "NEEDS WORK" },
            if avg_quality >= 80.0 { "🌟" } else if avg_quality >= 60.0 { "✅" } else { "⚠️" }
        );
        println!();
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len-3])
    } else {
        s.to_string()
    }
}