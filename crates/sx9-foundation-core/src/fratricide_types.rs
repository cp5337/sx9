//! Data structures and enums for Fratricide Prevention

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub name: String,
    pub file_path: String,
    pub line_number: u32,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInfo {
    pub name: String,
    pub path: String,
    pub purpose: String,
    pub dependencies: Vec<String>,
    pub exports: Vec<String>,
    pub mutex_locks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictRule {
    pub rule_type: ConflictType,
    pub pattern: String,
    pub severity: ConflictSeverity,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    FunctionDuplication,
    MutexDeadlock,
    ResourceContention,
    NamespaceCollision,
    ComponentOverlap,
    PortConflict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FratricideAnalysis {
    pub conflicts_detected: Vec<ConflictDetection>,
    pub duplication_score: f64,
    pub mutex_analysis: MutexAnalysis,
    pub component_overlaps: Vec<ComponentOverlap>,
    pub recommendations: Vec<String>,
    pub risk_level: ConflictSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictDetection {
    pub conflict_type: ConflictType,
    pub severity: ConflictSeverity,
    pub description: String,
    pub affected_files: Vec<String>,
    pub suggested_resolution: String,
    pub auto_fixable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutexAnalysis {
    pub potential_deadlocks: Vec<DeadlockRisk>,
    pub lock_contention_points: Vec<ContentionPoint>,
    pub file_lock_conflicts: Vec<FileLockConflict>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDuressPattern {
    pub pattern_type: String,
    pub description: String,
    pub original_file: String,
    pub duplicate_file: String,
    pub confidence: f64,
    pub recommendation: String,
    pub duress_indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentOverlap {
    pub component1: String,
    pub component2: String,
    pub overlap_type: String,
    pub severity: ConflictSeverity,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlockRisk {
    pub risk_type: String,
    pub description: String,
    pub affected_files: Vec<String>,
    pub severity: ConflictSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentionPoint {
    pub resource: String,
    pub contention_type: String,
    pub affected_files: Vec<String>,
    pub severity: ConflictSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLockConflict {
    pub file_path: String,
    pub lock_type: String,
    pub conflict_description: String,
    pub severity: ConflictSeverity,
}
