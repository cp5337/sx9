//! OSSEC TOML Rule Loader - Converts OSSEC rules to tool chains
//!
//! Loads RFC-9302 validated OSSEC TOML rules and converts them
//! into ForgeGraph nodes and tool chain definitions.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{info, warn, debug};
use tokio::fs;

use crate::nonagon::{NonagonCell, AlphaTrivariate, BetaTrivariate, GammaTrivariate, HD4Phase};
use crate::graph::{ForgeNode, ForgeEdge, NodeType, EdgeType, ForgeGraph};
use crate::mission_load::Primitive;
use crate::tool_gen::{GeneratedTool, ToolChain};

/// OSSEC TOML rule structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssecRule {
    /// Rule section
    pub rule: RuleSection,
    /// 1NF indicators
    #[serde(rename = "1nf")]
    pub indicators: Option<IndicatorsSection>,
    /// 2NF evasion tactics
    #[serde(rename = "2nf")]
    pub evasion: Option<EvasionSection>,
    /// Nine-sided nonagon analytics
    pub nine_sided: Option<NineSidedSection>,
}

/// Rule section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSection {
    /// Rule ID (60000-60699)
    pub id: u32,
    /// Severity level (1-15)
    pub level: u8,
    /// Rule description
    pub description: String,
    /// Primitive operation
    pub primitive: String,
    /// Unicode trigger
    pub unicode_trigger: String,
    /// SCH hash ID
    pub sch_id: String,
    /// Active response configuration
    pub active_response: Option<ActiveResponse>,
}

/// Active response configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveResponse {
    /// Command to execute
    pub command: String,
    /// Execution location
    pub location: String,
    /// Response level
    pub level: u8,
    /// Timeout in seconds
    pub timeout: u32,
}

/// 1NF Indicators section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorsSection {
    /// Plasma indicators
    pub indicators: Option<PlasmaIndicators>,
}

/// Plasma indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlasmaIndicators {
    /// Detection regex
    pub regex: Option<String>,
    /// Countermeasures
    pub countermeasures: Option<Vec<String>>,
}

/// 2NF Evasion section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvasionSection {
    /// Evasion tactics
    pub evasion: Option<EvasionTactics>,
}

/// Evasion tactics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvasionTactics {
    /// Tactic list
    pub tactics: Option<Vec<String>>,
}

/// Nine-sided nonagon analytics section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NineSidedSection {
    // Alpha trivariate
    pub alpha_x_context: f64,
    pub alpha_y_meaning: f64,
    pub alpha_z_intent: f64,
    // Beta trivariate
    pub beta_x_phase: f64,
    pub beta_y_intensity: f64,
    pub beta_z_duration: f64,
    // Gamma trivariate
    pub gamma_x_historical: f64,
    pub gamma_y_current: f64,
    pub gamma_z_predictive: f64,
    // Computed values
    pub center: f64,
    pub confidence: f64,
    pub vertices: Vec<f64>,
    // Trivariate hashes
    pub sch_primary: Option<String>,
    pub cuid_primary: Option<String>,
    pub uuid_primary: Option<String>,
}

impl NineSidedSection {
    /// Convert to NonagonCell
    pub fn to_nonagon_cell(&self, id: impl Into<String>) -> NonagonCell {
        NonagonCell::with_trivariates(
            id,
            AlphaTrivariate {
                context: self.alpha_x_context,
                meaning: self.alpha_y_meaning,
                intent: self.alpha_z_intent,
            },
            BetaTrivariate {
                phase: self.beta_x_phase,
                intensity: self.beta_y_intensity,
                duration: self.beta_z_duration,
            },
            GammaTrivariate {
                historical: self.gamma_x_historical,
                current: self.gamma_y_current,
                predictive: self.gamma_z_predictive,
            },
        )
    }
}

/// OSSEC Rule Loader
pub struct OssecLoader {
    /// Rules directory
    rules_path: PathBuf,
    /// Loaded rules by ID
    rules: HashMap<u32, OssecRule>,
    /// Rules by primitive
    by_primitive: HashMap<String, Vec<u32>>,
    /// Rules by level
    by_level: HashMap<u8, Vec<u32>>,
}

impl OssecLoader {
    /// Create new loader
    pub fn new(rules_path: impl Into<PathBuf>) -> Self {
        Self {
            rules_path: rules_path.into(),
            rules: HashMap::new(),
            by_primitive: HashMap::new(),
            by_level: HashMap::new(),
        }
    }

    /// Load all TOML rules from directory
    pub async fn load_all(&mut self) -> anyhow::Result<usize> {
        info!("Loading OSSEC TOML rules from {:?}", self.rules_path);

        if !self.rules_path.exists() {
            warn!("OSSEC rules path does not exist: {:?}", self.rules_path);
            return Ok(0);
        }

        let mut count = 0;
        let mut entries = fs::read_dir(&self.rules_path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "toml") {
                match self.load_rule(&path).await {
                    Ok(rule) => {
                        let id = rule.rule.id;
                        let primitive = rule.rule.primitive.clone();
                        let level = rule.rule.level;

                        // Index by primitive
                        self.by_primitive
                            .entry(primitive)
                            .or_default()
                            .push(id);

                        // Index by level
                        self.by_level
                            .entry(level)
                            .or_default()
                            .push(id);

                        self.rules.insert(id, rule);
                        count += 1;
                    }
                    Err(e) => {
                        warn!("Failed to load rule {:?}: {}", path, e);
                    }
                }
            }
        }

        info!("Loaded {} OSSEC rules", count);
        info!("  Primitives: {:?}", self.by_primitive.keys().collect::<Vec<_>>());
        Ok(count)
    }

    /// Load single rule from file
    async fn load_rule(&self, path: &Path) -> anyhow::Result<OssecRule> {
        let content = fs::read_to_string(path).await?;
        let rule: OssecRule = toml::from_str(&content)?;
        debug!("Loaded rule {}: {}", rule.rule.id, rule.rule.primitive);
        Ok(rule)
    }

    /// Get rule by ID
    pub fn get(&self, id: u32) -> Option<&OssecRule> {
        self.rules.get(&id)
    }

    /// Get rules by primitive
    pub fn get_by_primitive(&self, primitive: &str) -> Vec<&OssecRule> {
        self.by_primitive
            .get(primitive)
            .map(|ids| ids.iter().filter_map(|id| self.rules.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get rules by level
    pub fn get_by_level(&self, level: u8) -> Vec<&OssecRule> {
        self.by_level
            .get(&level)
            .map(|ids| ids.iter().filter_map(|id| self.rules.get(id)).collect())
            .unwrap_or_default()
    }

    /// Convert rule to ForgeGraph node
    pub fn rule_to_node(&self, rule: &OssecRule) -> ForgeNode {
        let mut node = ForgeNode::new(
            NodeType::OssecRule,
            format!("OSSEC-{}", rule.rule.id),
        );

        node.id = format!("ossec-{}", rule.rule.id);
        node.set_property("level", rule.rule.level.to_string());
        node.set_property("primitive", &rule.rule.primitive);
        node.set_property("unicode_trigger", &rule.rule.unicode_trigger);
        node.set_property("sch_id", &rule.rule.sch_id);

        // Set TETH entropy from nine_sided if available
        if let Some(ns) = &rule.nine_sided {
            let cell = ns.to_nonagon_cell(&node.id);
            node.teth_entropy = Some(cell.teth_entropy);
            node.set_property("confidence", ns.confidence.to_string());
            node.set_property("center", ns.center.to_string());
        }

        node
    }

    /// Add all rules to graph as nodes
    pub async fn populate_graph(&self, graph: &mut ForgeGraph) -> anyhow::Result<usize> {
        let mut count = 0;

        for rule in self.rules.values() {
            let node = self.rule_to_node(rule);
            graph.add_node(node).await?;
            count += 1;
        }

        info!("Added {} OSSEC rules to graph", count);
        Ok(count)
    }

    /// Create tool chain from rules matching criteria
    pub fn create_chain_from_rules(
        &self,
        name: impl Into<String>,
        rule_ids: &[u32],
    ) -> Option<ToolChain> {
        if rule_ids.is_empty() {
            return None;
        }

        let mut chain = ToolChain::new(name);
        chain.description = format!("Chain from {} OSSEC rules", rule_ids.len());

        // Compute combined nonagon from rules
        let mut combined_vertices = [0.0f64; 9];
        let mut valid_count = 0;

        for id in rule_ids {
            if let Some(rule) = self.rules.get(id) {
                chain.add_tool(format!("ossec-{}", id));

                if let Some(ns) = &rule.nine_sided {
                    for (i, v) in ns.vertices.iter().enumerate() {
                        if i < 9 {
                            combined_vertices[i] += v;
                        }
                    }
                    valid_count += 1;
                }
            }
        }

        // Average the vertices
        if valid_count > 0 {
            for v in &mut combined_vertices {
                *v /= valid_count as f64;
            }
        }

        // Create combined nonagon
        chain.nonagon = NonagonCell::with_trivariates(
            &chain.id,
            AlphaTrivariate {
                context: combined_vertices[0],
                meaning: combined_vertices[1],
                intent: combined_vertices[2],
            },
            BetaTrivariate {
                phase: combined_vertices[3],
                intensity: combined_vertices[4],
                duration: combined_vertices[5],
            },
            GammaTrivariate {
                historical: combined_vertices[6],
                current: combined_vertices[7],
                predictive: combined_vertices[8],
            },
        );

        Some(chain)
    }

    /// Create tool chains by primitive type
    pub fn create_chains_by_primitive(&self) -> Vec<ToolChain> {
        let mut chains = Vec::new();

        for (primitive, ids) in &self.by_primitive {
            if let Some(chain) = self.create_chain_from_rules(
                format!("{} Chain", primitive),
                ids,
            ) {
                chains.push(chain);
            }
        }

        chains
    }

    /// Create tool chains by severity level (high, medium, low)
    pub fn create_chains_by_severity(&self) -> Vec<ToolChain> {
        let mut chains = Vec::new();

        // High severity (level >= 10)
        let high_ids: Vec<u32> = self.by_level
            .iter()
            .filter(|(&level, _)| level >= 10)
            .flat_map(|(_, ids)| ids.clone())
            .collect();

        if let Some(chain) = self.create_chain_from_rules("High Severity Chain", &high_ids) {
            chains.push(chain);
        }

        // Medium severity (level 5-9)
        let medium_ids: Vec<u32> = self.by_level
            .iter()
            .filter(|(&level, _)| level >= 5 && level < 10)
            .flat_map(|(_, ids)| ids.clone())
            .collect();

        if let Some(chain) = self.create_chain_from_rules("Medium Severity Chain", &medium_ids) {
            chains.push(chain);
        }

        // Low severity (level < 5)
        let low_ids: Vec<u32> = self.by_level
            .iter()
            .filter(|(&level, _)| level < 5)
            .flat_map(|(_, ids)| ids.clone())
            .collect();

        if let Some(chain) = self.create_chain_from_rules("Low Severity Chain", &low_ids) {
            chains.push(chain);
        }

        chains
    }

    /// Map primitive string to Primitive enum
    pub fn primitive_from_str(s: &str) -> Option<Primitive> {
        match s.to_uppercase().as_str() {
            "READ" => Some(Primitive::Read),
            "WRITE" => Some(Primitive::Write),
            "TRANSFORM" => Some(Primitive::Transform),
            "FILTER" => Some(Primitive::Filter),
            "ENCRYPT" => Some(Primitive::Encrypt),
            "DECRYPT" => Some(Primitive::Decrypt),
            "AUTHENTICATE" => Some(Primitive::Authenticate),
            "AUTHORIZE" => Some(Primitive::Authorize),
            "VALIDATE" => Some(Primitive::Validate),
            "ROUTE" => Some(Primitive::Route),
            "BUFFER" => Some(Primitive::Buffer),
            "QUEUE" => Some(Primitive::Queue),
            "SYNCHRONIZE" => Some(Primitive::Synchronize),
            "REPLICATE" => Some(Primitive::Replicate),
            "OBSERVE" => Some(Primitive::Observe),
            "CACHE" => Some(Primitive::Cache),
            "EXECUTE" => Some(Primitive::Execute),
            "RECONNAISSANCE" => Some(Primitive::Reconnaissance),
            "COMMAND_CONTROL" => Some(Primitive::CommandControl),
            "INSTALL" => Some(Primitive::Install),
            _ => None,
        }
    }

    /// Get statistics
    pub fn stats(&self) -> LoaderStats {
        LoaderStats {
            total_rules: self.rules.len(),
            primitives: self.by_primitive.keys().cloned().collect(),
            by_primitive_count: self.by_primitive
                .iter()
                .map(|(k, v)| (k.clone(), v.len()))
                .collect(),
            levels: self.by_level.keys().copied().collect(),
        }
    }
}

/// Loader statistics
#[derive(Debug, Clone, Serialize)]
pub struct LoaderStats {
    /// Total rules loaded
    pub total_rules: usize,
    /// Unique primitives
    pub primitives: Vec<String>,
    /// Count by primitive
    pub by_primitive_count: HashMap<String, usize>,
    /// Unique levels
    pub levels: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_mapping() {
        assert_eq!(OssecLoader::primitive_from_str("TRANSFORM"), Some(Primitive::Transform));
        assert_eq!(OssecLoader::primitive_from_str("execute"), Some(Primitive::Execute));
        assert_eq!(OssecLoader::primitive_from_str("QUEUE"), Some(Primitive::Queue));
    }
}
