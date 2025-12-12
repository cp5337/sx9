//! Haptic physics engine for tactical validation
//!
//! Steve Jobs-level physics-based tactical planning with haptic feedback
//! Implements attraction/repulsion mechanics for tool and mission validation

use crate::TacticalResult;
use serde::{Deserialize, Serialize};

/// Physics engine for haptic tactical validation
pub struct HapticPhysicsEngine {
    gravity_constant: f64,
    friction_coefficient: f64,
    electromagnetic_strength: f64,
}

impl Default for HapticPhysicsEngine {
    fn default() -> Self {
        Self {
            gravity_constant: 9.81,
            friction_coefficient: 0.1,
            electromagnetic_strength: 1.0,
        }
    }
}

/// Physical properties of tactical elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalPhysics {
    pub mass: f64,
    pub charge: f64,
    pub position: Vector3D,
    pub velocity: Vector3D,
    pub constraints: Vec<PhysicsConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsConstraint {
    pub constraint_type: ConstraintType,
    pub force_magnitude: f64,
    pub direction: Vector3D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    Environmental,
    Policy,
    Resource,
    Temporal,
    Security,
}

/// Force calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceResult {
    pub net_force: Vector3D,
    pub attraction_strength: f64,
    pub repulsion_strength: f64,
    pub validation_score: f64,
}

impl HapticPhysicsEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate forces between tactical elements
    pub fn calculate_forces(&self, elements: &[TacticalPhysics]) -> Vec<ForceResult> {
        elements
            .iter()
            .map(|element| {
                let mut net_force = Vector3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                let mut attraction = 0.0;
                let mut repulsion = 0.0;

                // Calculate electromagnetic forces (attraction/repulsion)
                for other in elements {
                    if std::ptr::eq(element, other) {
                        continue;
                    }

                    let distance = self.calculate_distance(&element.position, &other.position);
                    if distance > 0.0 {
                        let force_magnitude = self.electromagnetic_strength
                            * (element.charge * other.charge)
                            / (distance * distance);

                        let direction = self.normalize_vector(
                            &self.subtract_vectors(&other.position, &element.position),
                        );

                        if force_magnitude > 0.0 {
                            attraction += force_magnitude;
                            net_force = self.add_vectors(
                                &net_force,
                                &self.scale_vector(&direction, force_magnitude),
                            );
                        } else {
                            repulsion += force_magnitude.abs();
                            net_force = self.subtract_vectors(
                                &net_force,
                                &self.scale_vector(&direction, force_magnitude.abs()),
                            );
                        }
                    }
                }

                // Apply constraint forces
                for constraint in &element.constraints {
                    let constraint_force =
                        self.scale_vector(&constraint.direction, constraint.force_magnitude);
                    net_force = self.add_vectors(&net_force, &constraint_force);

                    match constraint.constraint_type {
                        ConstraintType::Environmental
                        | ConstraintType::Policy
                        | ConstraintType::Security => {
                            repulsion += constraint.force_magnitude.abs();
                        }
                        ConstraintType::Resource | ConstraintType::Temporal => {
                            attraction += constraint.force_magnitude.max(0.0);
                        }
                    }
                }

                // Calculate validation score (higher attraction = better validation)
                let validation_score = if attraction > 0.0 && repulsion > 0.0 {
                    attraction / (attraction + repulsion)
                } else if attraction > 0.0 {
                    1.0
                } else {
                    0.0
                };

                ForceResult {
                    net_force,
                    attraction_strength: attraction,
                    repulsion_strength: repulsion,
                    validation_score,
                }
            })
            .collect()
    }

    /// Generate haptic feedback patterns
    pub async fn generate_haptic_pattern(
        &self,
        force_result: &ForceResult,
    ) -> TacticalResult<HapticPattern> {
        let start = std::time::Instant::now();

        let pattern = if force_result.validation_score > 0.7 {
            // Strong attraction - smooth, satisfying vibration
            HapticPattern {
                pattern_type: HapticPatternType::Attraction,
                intensity: (force_result.attraction_strength / 10.0).min(1.0) as f32,
                frequency: 120.0, // Smooth frequency
                duration_ms: 150,
                pulse_count: 1,
                fade_in_ms: 50,
                fade_out_ms: 100,
            }
        } else if force_result.validation_score < 0.3 {
            // Strong repulsion - sharp, warning vibration
            HapticPattern {
                pattern_type: HapticPatternType::Repulsion,
                intensity: (force_result.repulsion_strength / 10.0).min(1.0) as f32,
                frequency: 300.0, // Sharp, warning frequency
                duration_ms: 200,
                pulse_count: 3,
                fade_in_ms: 10,
                fade_out_ms: 50,
            }
        } else {
            // Neutral - gentle notification
            HapticPattern {
                pattern_type: HapticPatternType::Neutral,
                intensity: 0.3,
                frequency: 180.0,
                duration_ms: 100,
                pulse_count: 1,
                fade_in_ms: 30,
                fade_out_ms: 70,
            }
        };

        TacticalResult::success(pattern, start.elapsed().as_millis() as f64)
    }

    /// Validate tactical configuration with physics
    pub async fn validate_configuration(
        &self,
        elements: &[TacticalPhysics],
    ) -> TacticalResult<ValidationSummary> {
        let start = std::time::Instant::now();

        let force_results = self.calculate_forces(elements);

        let total_attraction: f64 = force_results.iter().map(|r| r.attraction_strength).sum();
        let total_repulsion: f64 = force_results.iter().map(|r| r.repulsion_strength).sum();
        let average_validation = force_results
            .iter()
            .map(|r| r.validation_score)
            .sum::<f64>()
            / force_results.len() as f64;

        let is_valid = average_validation > 0.5;
        let stability_factor = if total_attraction + total_repulsion > 0.0 {
            total_attraction / (total_attraction + total_repulsion)
        } else {
            0.5
        };

        let summary = ValidationSummary {
            is_valid,
            validation_score: average_validation,
            stability_factor,
            total_attraction,
            total_repulsion,
            element_count: elements.len(),
            recommendations: generate_recommendations(&force_results),
        };

        TacticalResult::success(summary, start.elapsed().as_millis() as f64)
    }

    // Vector math utility functions
    fn calculate_distance(&self, a: &Vector3D, b: &Vector3D) -> f64 {
        ((b.x - a.x).powi(2) + (b.y - a.y).powi(2) + (b.z - a.z).powi(2)).sqrt()
    }

    fn subtract_vectors(&self, a: &Vector3D, b: &Vector3D) -> Vector3D {
        Vector3D {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    }

    fn add_vectors(&self, a: &Vector3D, b: &Vector3D) -> Vector3D {
        Vector3D {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    }

    fn scale_vector(&self, v: &Vector3D, scale: f64) -> Vector3D {
        Vector3D {
            x: v.x * scale,
            y: v.y * scale,
            z: v.z * scale,
        }
    }

    fn normalize_vector(&self, v: &Vector3D) -> Vector3D {
        let magnitude = (v.x.powi(2) + v.y.powi(2) + v.z.powi(2)).sqrt();
        if magnitude > 0.0 {
            Vector3D {
                x: v.x / magnitude,
                y: v.y / magnitude,
                z: v.z / magnitude,
            }
        } else {
            Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HapticPattern {
    pub pattern_type: HapticPatternType,
    pub intensity: f32,
    pub frequency: f32,
    pub duration_ms: u32,
    pub pulse_count: u32,
    pub fade_in_ms: u32,
    pub fade_out_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HapticPatternType {
    Attraction,
    Repulsion,
    Neutral,
    Warning,
    Success,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub is_valid: bool,
    pub validation_score: f64,
    pub stability_factor: f64,
    pub total_attraction: f64,
    pub total_repulsion: f64,
    pub element_count: usize,
    pub recommendations: Vec<String>,
}

fn generate_recommendations(force_results: &[ForceResult]) -> Vec<String> {
    let mut recommendations = Vec::new();

    for (i, result) in force_results.iter().enumerate() {
        if result.validation_score < 0.3 {
            recommendations.push(format!(
                "Element {} has high repulsion - review constraints",
                i
            ));
        }
        if result.attraction_strength < 1.0 {
            recommendations.push(format!("Element {} needs stronger compatibility", i));
        }
    }

    if recommendations.is_empty() {
        recommendations.push("Configuration validated successfully".to_string());
    }

    recommendations
}
