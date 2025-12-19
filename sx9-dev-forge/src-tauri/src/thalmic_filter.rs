//! Thalmic Filter - Plain Language Intent Parser with Adaptive Learning
//! 
//! Translates natural language commands into deterministic intent structures
//! Performance target: <1ms for cached patterns, <5ms for new patterns
//! Learns from usage to create personal shortcuts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Action {
    Search,
    Create,
    Audit,
    Explain,
    Execute,
    Start,
    Stop,
    Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub action: Action,
    pub targets: Vec<String>,
    pub agents: Vec<String>,
    pub context: HashMap<String, String>,
    pub raw_input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CustomPattern {
    input: String,
    intent: Intent,
    uses: u32,
    created_at: DateTime<Utc>,
    last_used: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct AdaptivePatterns {
    custom_patterns: Vec<CustomPattern>,
    aliases: HashMap<String, Vec<String>>,
}

pub struct ThalmicFilter {
    patterns: AdaptivePatterns,
    patterns_path: PathBuf,
}

impl ThalmicFilter {
    /// Create new Thalmic Filter with adaptive learning
    pub fn new() -> Self {
        let patterns_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".sx9")
            .join("thalmic_patterns.json");
        
        let patterns = Self::load_patterns(&patterns_path)
            .unwrap_or_default();
        
        Self {
            patterns,
            patterns_path,
        }
    }
    
    fn load_patterns(path: &PathBuf) -> Result<AdaptivePatterns, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let patterns = serde_json::from_str(&content)?;
        Ok(patterns)
    }
    
    fn save_patterns(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(&self.patterns)?;
        fs::write(&self.patterns_path, content)?;
        Ok(())
    }
    
    /// Parse plain language input with adaptive learning
    pub fn parse(&mut self, input: &str) -> Intent {
        let lower = input.to_lowercase();
        
        // 1. Try custom patterns first (fastest - your learned shortcuts)
        let custom_match = self.patterns.custom_patterns
            .iter()
            .find(|p| p.input == lower)
            .map(|p| p.intent.clone());
        
        if let Some(intent) = custom_match {
            self.record_use(&lower, &intent);
            return intent;
        }
        
        // 2. Try standard pattern matching
        let intent = Self::parse_standard(&lower, input);
        
        // 3. Record usage for learning
        self.record_use(&lower, &intent);
        
        intent
    }
    
    fn record_use(&mut self, input: &str, intent: &Intent) {
        // Find existing pattern or create new
        if let Some(pattern) = self.patterns.custom_patterns
            .iter_mut()
            .find(|p| p.input == input)
        {
            // Update existing pattern
            pattern.uses += 1;
            pattern.last_used = Utc::now();
        } else {
            // Track new pattern - after 3 uses, it becomes a custom pattern
            let uses = self.count_similar_uses(input);
            if uses >= 2 {  // This is the 3rd use
                self.patterns.custom_patterns.push(CustomPattern {
                    input: input.to_string(),
                    intent: intent.clone(),
                    uses: 3,
                    created_at: Utc::now(),
                    last_used: Utc::now(),
                });
            }
        }
        
        // Save patterns periodically (every 5 uses)
        if self.patterns.custom_patterns.iter().map(|p| p.uses).sum::<u32>() % 5 == 0 {
            let _ = self.save_patterns();
        }
    }
    
    fn count_similar_uses(&self, _input: &str) -> u32 {
        // TODO: Implement fuzzy matching for similar inputs
        0
    }
    
    /// Standard pattern matching (non-adaptive)
    fn parse_standard(lower: &str, original: &str) -> Intent {
        
        // Detect action
        let action = Self::detect_action(&lower);
        
        // Extract targets (files, modules, components)
        let targets = Self::extract_targets(&lower, original);
        
        // Detect agents/personas
        let agents = Self::detect_agents(&lower);
        
        // Extract additional context
        let context = Self::extract_context(&lower);
        
        Intent {
            action,
            targets,
            agents,
            context,
            raw_input: original.to_string(),
        }
    }
    
    fn detect_action(input: &str) -> Action {
        // Search/Find
        if input.contains("search") || input.contains("find") || input.contains("locate") {
            return Action::Search;
        }
        
        // Create/Make/Build
        if input.contains("create") || input.contains("make") || input.contains("build") {
            return Action::Create;
        }
        
        // Audit/Check/Review
        if input.contains("audit") || input.contains("check") || input.contains("review") {
            return Action::Audit;
        }
        
        // Explain/Describe/Tell
        if input.contains("explain") || input.contains("describe") || input.contains("tell") {
            return Action::Explain;
        }
        
        // Start
        if input.contains("start") || input.contains("launch") || input.contains("run") {
            return Action::Start;
        }
        
        // Stop
        if input.contains("stop") || input.contains("kill") || input.contains("terminate") {
            return Action::Stop;
        }
        
        // Status
        if input.contains("status") || input.contains("health") || input.contains("show") {
            return Action::Status;
        }
        
        // Default: Execute
        Action::Execute
    }
    
    fn extract_targets(lower: &str, original: &str) -> Vec<String> {
        let mut targets = Vec::new();
        
        // File patterns
        if lower.contains("auth") {
            targets.push("auth".to_string());
        }
        if lower.contains("api") {
            targets.push("api".to_string());
        }
        if lower.contains("clipboard") {
            targets.push("clipboard".to_string());
        }
        if lower.contains("mission") {
            targets.push("mission".to_string());
        }
        if lower.contains("vault") {
            targets.push("vault".to_string());
        }
        
        // Extract quoted strings as targets
        if let Some(start) = original.find('"') {
            if let Some(end) = original[start + 1..].find('"') {
                let quoted = &original[start + 1..start + 1 + end];
                targets.push(quoted.to_string());
            }
        }
        
        targets
    }
    
    fn detect_agents(input: &str) -> Vec<String> {
        let mut agents = Vec::new();
        
        // Elite dev personas
        if input.contains("forge") { agents.push("FORGE".to_string()); }
        if input.contains("axiom") { agents.push("AXIOM".to_string()); }
        if input.contains("vector") { agents.push("VECTOR".to_string()); }
        if input.contains("sentinel") { agents.push("SENTINEL".to_string()); }
        if input.contains("nexus") { agents.push("NEXUS".to_string()); }
        if input.contains("cipher") { agents.push("CIPHER".to_string()); }
        if input.contains("scribe") { agents.push("SCRIBE".to_string()); }
        
        // Team-based detection
        if input.contains("security team") || input.contains("security") {
            agents.extend(vec![
                "VECTOR".to_string(),
                "CIPHER".to_string(),
                "SENTINEL".to_string(),
            ]);
        }
        
        if input.contains("dev team") || input.contains("developers") {
            agents.extend(vec![
                "FORGE".to_string(),
                "AXIOM".to_string(),
                "NEXUS".to_string(),
            ]);
        }
        
        // All agents
        if input.contains("all") || input.contains("everyone") {
            agents.push("all".to_string());
        }
        
        agents
    }
    
    fn extract_context(input: &str) -> HashMap<String, String> {
        let mut context = HashMap::new();
        
        // Priority detection
        if input.contains("urgent") || input.contains("critical") {
            context.insert("priority".to_string(), "high".to_string());
        } else if input.contains("low priority") {
            context.insert("priority".to_string(), "low".to_string());
        }
        
        // Scope detection
        if input.contains("quick") || input.contains("fast") {
            context.insert("scope".to_string(), "quick".to_string());
        } else if input.contains("thorough") || input.contains("detailed") {
            context.insert("scope".to_string(), "thorough".to_string());
        }
        
        context
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_intent() {
        let intent = ThalmicFilter::parse("search for authentication files");
        assert_eq!(intent.action, Action::Search);
        assert!(intent.targets.contains(&"auth".to_string()));
    }

    #[test]
    fn test_audit_with_team() {
        let intent = ThalmicFilter::parse("have the security team audit the auth module");
        assert_eq!(intent.action, Action::Audit);
        assert!(intent.targets.contains(&"auth".to_string()));
        assert!(intent.agents.contains(&"VECTOR".to_string()));
        assert!(intent.agents.contains(&"CIPHER".to_string()));
    }

    #[test]
    fn test_create_mission() {
        let intent = ThalmicFilter::parse("create a mission for FORGE to build the API");
        assert_eq!(intent.action, Action::Create);
        assert!(intent.targets.contains(&"api".to_string()));
        assert!(intent.agents.contains(&"FORGE".to_string()));
    }

    #[test]
    fn test_explain_with_priority() {
        let intent = ThalmicFilter::parse("urgent: explain the clipboard system");
        assert_eq!(intent.action, Action::Explain);
        assert!(intent.targets.contains(&"clipboard".to_string()));
        assert_eq!(intent.context.get("priority"), Some(&"high".to_string()));
    }
}
