//! Voice Hooks for Smart Crates
//!
//! Enables Smart Crates to emit voice notifications and respond to voice commands.
//! Integrates with ctas7-foundation-voice for ElevenLabs synthesis and thalamic filtering.
//!
//! ## v7.3.1 Hallmark Feature
//!
//! ## Critical Principle: Voice → Lowest Deterministic Level
//!
//! Voice commands ALWAYS start at the lowest level tool (script) and escalate
//! only when necessary. This ensures:
//! - Maximum efficiency (don't use nmap when ping suffices)
//! - Cost optimization (scripts are free, containers cost resources)
//! - Deterministic execution (scripts are predictable)
//! - Resource consciousness (hourglass principle)
//!
//! Example:
//! ```
//! Voice: "Scan 192.168.1.218"
//!   → Level 0: ping.sh (100ms, 1KB)
//!   → Level 1: firefly-microkernel.wasm (if host alive)
//!   → Level 2: scorpion-recon.wasm (if ports detected)
//!   → Level 3: rustscan (if services found)
//!   → Level 4: nmap container (if full scan needed)
//!   → Level 5: honeypot (if offensive testing authorized)
//! ```

use sx9_foundation_manifold::core::diagnostics::anyhow::{Result, Context};
use sx9_foundation_manifold::core::data::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use sx9_foundation_manifold::core::async_runtime::tokio::process::Command;
use sx9_foundation_manifold::core::diagnostics::tracing::{info, warn, debug};

// Import from foundation-voice
// use foundation_voice::{VoiceLogger, VoiceInteraction, VoiceSpeaker, VoicePriority};

/// Voice hooks configuration from Cargo.toml metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceHooksConfig {
    /// Enable voice hooks
    pub enabled: bool,
    
    /// Voice persona to use (e.g., "zoe_technical", "natasha")
    pub persona: String,
    
    /// Default priority for voice notifications
    pub priority: VoicePriority,
    
    /// Apply thalamic filtering
    pub thalamic_filter: bool,
    
    /// Voice hooks for lifecycle events
    pub events: VoiceHooksEvents,
    
    /// Voice commands with escalation ladder
    pub commands: HashMap<String, VoiceCommandConfig>,
}

/// Voice command configuration with escalation ladder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCommandConfig {
    /// Command phrase (e.g., "scan target", "build crate")
    pub phrase: String,
    
    /// Escalation ladder (ordered from lowest to highest level)
    pub escalation_ladder: Vec<EscalationLevel>,
    
    /// Auto-escalate based on results
    pub auto_escalate: bool,
    
    /// Maximum escalation level (0-5, or None for unlimited)
    pub max_level: Option<u8>,
    
    /// Orchestration mode override (can be set via voice command)
    pub orchestration_mode: Option<OrchestrationMode>,
}

/// Orchestration mode - controls which levels are allowed
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrchestrationMode {
    /// Only scripts (Level 0)
    ScriptsOnly,
    /// Scripts + Microkernel (Levels 0-1)
    LimitToMicrokernel,
    /// Scripts + Microkernel + WASM (Levels 0-2)
    LimitToWASM,
    /// Scripts + Microkernel + WASM + Binaries (Levels 0-3)
    LimitToBinaries,
    /// All levels except VMs (Levels 0-4)
    NoVMs,
    /// All levels allowed (Levels 0-5)
    FullEscalation,
}

/// Escalation level in the ladder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationLevel {
    /// Level number (0 = script, 1 = microkernel, 2 = WASM, 3 = binary, 4 = container, 5 = VM)
    pub level: u8,
    
    /// Tool/script to execute
    pub tool: String,
    
    /// Estimated resource usage
    pub resources: ResourceEstimate,
    
    /// Condition to escalate to next level (optional)
    pub escalate_if: Option<String>,
    
    /// Success message template
    pub on_success: Option<String>,
    
    /// Failure message template
    pub on_failure: Option<String>,
}

/// Resource estimate for a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceEstimate {
    pub disk: String,      // e.g., "1 KB", "50 MB"
    pub memory: String,    // e.g., "1 MB", "100 MB"
    pub cpu: String,       // e.g., "<1%", "25%"
    pub time: String,      // e.g., "100ms", "30s"
}

/// Result of executing a voice command with escalation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCommandResult {
    pub crate_name: String,
    pub command_phrase: String,
    pub levels_executed: Vec<EscalationLevelResult>,
    pub final_level: u8,
    pub total_duration_ms: u64,
}

/// Result of executing a single escalation level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationLevelResult {
    pub level: u8,
    pub tool: String,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
    pub resources: ResourceEstimate,
}

/// Voice hooks for crate lifecycle events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceHooksEvents {
    pub on_build_start: Option<String>,
    pub on_build_success: Option<String>,
    pub on_build_failure: Option<String>,
    pub on_test_start: Option<String>,
    pub on_test_success: Option<String>,
    pub on_test_failure: Option<String>,
    pub on_deploy_start: Option<String>,
    pub on_deploy_success: Option<String>,
    pub on_deploy_failure: Option<String>,
    pub on_qa_start: Option<String>,
    pub on_qa_complete: Option<String>,
    pub on_health_check: Option<String>,
    pub on_error: Option<String>,
}

/// Voice priority levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VoicePriority {
    Low,
    Normal,
    High,
    Critical,
    Emergency,
}

/// Voice hook event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceHookEvent {
    BuildStart,
    BuildSuccess,
    BuildFailure,
    TestStart,
    TestSuccess,
    TestFailure,
    DeployStart,
    DeploySuccess,
    DeployFailure,
    QAStart,
    QAComplete,
    HealthCheck,
    Error,
}

/// Voice hook context (variables for template substitution)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceHookContext {
    pub crate_name: String,
    pub version: String,
    pub duration: Option<String>,
    pub error: Option<String>,
    pub failed_count: Option<u32>,
    pub score: Option<u32>,
    pub environment: Option<String>,
    pub custom: HashMap<String, String>,
}

/// Voice hooks manager for Smart Crates
pub struct VoiceHooksManager {
    /// Voice logger (from foundation-voice)
    // voice_logger: Arc<VoiceLogger>,
    
    /// Active voice hooks configurations (crate_name -> config)
    hooks: HashMap<String, VoiceHooksConfig>,
}

impl VoiceHooksManager {
    /// Create a new voice hooks manager
    pub fn new() -> Self {
        info!("Initializing Voice Hooks Manager (v7.3.1)");
        
        Self {
            hooks: HashMap::new(),
        }
    }

    /// Register voice hooks for a crate
    pub fn register_crate(&mut self, crate_name: &str, config: VoiceHooksConfig) -> Result<()> {
        if config.enabled {
            info!("Registering voice hooks for crate: {} (persona: {})", crate_name, config.persona);
            self.hooks.insert(crate_name.to_string(), config);
        } else {
            debug!("Voice hooks disabled for crate: {}", crate_name);
        }
        
        Ok(())
    }

    /// Load voice hooks from Cargo.toml metadata
    pub async fn load_from_cargo_toml(&mut self, cargo_toml_path: &Path) -> Result<()> {
        let content = sx9_foundation_manifold::core::async_runtime::tokio::fs::read_to_string(cargo_toml_path).await
            .context("Failed to read Cargo.toml")?;
        
        let cargo_toml: toml::Value = toml::from_str(&content)
            .context("Failed to parse Cargo.toml")?;
        
        // Extract package name
        let crate_name = cargo_toml
            .get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .ok_or_else(|| anyhow::anyhow!("No package name in Cargo.toml"))?;
        
        // Check for voice_hooks metadata
        if let Some(metadata) = cargo_toml
            .get("package")
            .and_then(|p| p.get("metadata"))
            .and_then(|m| m.get("voice_hooks"))
        {
            let config: VoiceHooksConfig = toml::from_str(&metadata.to_string())
                .context("Failed to parse voice_hooks metadata")?;
            
            self.register_crate(crate_name, config)?;
        }
        
        Ok(())
    }

    /// Trigger a voice hook event
    pub async fn trigger_event(
        &self,
        crate_name: &str,
        event: VoiceHookEvent,
        context: VoiceHookContext,
    ) -> Result<()> {
        // Get hooks config for this crate
        let config = match self.hooks.get(crate_name) {
            Some(c) if c.enabled => c,
            _ => {
                debug!("Voice hooks not enabled for crate: {}", crate_name);
                return Ok(());
            }
        };
        
        // Get the message template for this event
        let template = match event {
            VoiceHookEvent::BuildStart => config.events.on_build_start.as_ref(),
            VoiceHookEvent::BuildSuccess => config.events.on_build_success.as_ref(),
            VoiceHookEvent::BuildFailure => config.events.on_build_failure.as_ref(),
            VoiceHookEvent::TestStart => config.events.on_test_start.as_ref(),
            VoiceHookEvent::TestSuccess => config.events.on_test_success.as_ref(),
            VoiceHookEvent::TestFailure => config.events.on_test_failure.as_ref(),
            VoiceHookEvent::DeployStart => config.events.on_deploy_start.as_ref(),
            VoiceHookEvent::DeploySuccess => config.events.on_deploy_success.as_ref(),
            VoiceHookEvent::DeployFailure => config.events.on_deploy_failure.as_ref(),
            VoiceHookEvent::QAStart => config.events.on_qa_start.as_ref(),
            VoiceHookEvent::QAComplete => config.events.on_qa_complete.as_ref(),
            VoiceHookEvent::HealthCheck => config.events.on_health_check.as_ref(),
            VoiceHookEvent::Error => config.events.on_error.as_ref(),
        };
        
        if let Some(template) = template {
            // Substitute variables in template
            let message = self.substitute_template(template, &context);
            
            info!("Voice hook triggered: {} - {}", crate_name, message);
            
            // TODO: Integrate with foundation-voice VoiceLogger
            // self.voice_logger.log_update(
            //     VoiceSpeaker::from_persona(&config.persona),
            //     &message
            // ).await?;
            
            // For now, just log it
            println!("🔊 [{}] {}", config.persona, message);
        }
        
        Ok(())
    }

    /// Execute a voice command for a crate (with escalation ladder)
    pub async fn execute_command(
        &self,
        crate_name: &str,
        command_phrase: &str,
        context: &VoiceHookContext,
        orchestration_override: Option<OrchestrationMode>,
    ) -> Result<VoiceCommandResult> {
        // Get hooks config for this crate
        let config = match self.hooks.get(crate_name) {
            Some(c) if c.enabled => c,
            _ => {
                return Err(anyhow::anyhow!("Voice hooks not enabled for crate: {}", crate_name));
            }
        };
        
        // Find matching command
        let command_config = config.commands.get(command_phrase)
            .ok_or_else(|| anyhow::anyhow!("Unknown voice command: {}", command_phrase))?;
        
        // Determine orchestration mode (override takes precedence)
        let orchestration_mode = orchestration_override
            .or(command_config.orchestration_mode)
            .unwrap_or(OrchestrationMode::FullEscalation);
        
        let max_allowed_level = Self::get_max_level_for_mode(orchestration_mode);
        
        info!("🎤 Voice command: '{}' for {} (mode: {:?}, max level: {})", 
            command_phrase, crate_name, orchestration_mode, max_allowed_level);
        
        // Execute escalation ladder
        let mut results = Vec::new();
        let mut current_level = 0;
        
        for level_config in &command_config.escalation_ladder {
            // Check orchestration mode limit
            if level_config.level > max_allowed_level {
                info!("⛔ Orchestration mode {:?} limits to level {}, stopping", 
                    orchestration_mode, max_allowed_level);
                break;
            }
            
            // Check if we should stop escalating (user-defined max)
            if let Some(max_level) = command_config.max_level {
                if level_config.level > max_level {
                    info!("⛔ Max escalation level {} reached, stopping", max_level);
                    break;
                }
            }
            
            info!("💩 Level {}: {} ({})", 
                level_config.level, 
                level_config.tool,
                level_config.resources.time
            );
            
            // Execute the tool at this level
            let start = std::time::Instant::now();
            let output = sx9_foundation_manifold::core::async_runtime::tokio::process::Command::new("sh")
                .arg("-c")
                .arg(&level_config.tool)
                .output()
                .await
                .context(format!("Failed to execute level {}", level_config.level))?;
            
            let duration = start.elapsed();
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            
            let level_result = EscalationLevelResult {
                level: level_config.level,
                tool: level_config.tool.clone(),
                success: output.status.success(),
                stdout: stdout.clone(),
                stderr: stderr.clone(),
                duration_ms: duration.as_millis() as u64,
                resources: level_config.resources.clone(),
            };
            
            if output.status.success() {
                if let Some(ref success_msg) = level_config.on_success {
                    let msg = self.substitute_template(success_msg, context);
                    info!("✅ {}", msg);
                }
                
                // Check if we should escalate to next level
                if command_config.auto_escalate {
                    if let Some(ref escalate_condition) = level_config.escalate_if {
                        // Simple condition check (e.g., "host is alive", "ports detected")
                        if stdout.contains(escalate_condition) {
                            info!("🔼 Escalating to next level (condition met: {})", escalate_condition);
                            results.push(level_result);
                            current_level += 1;
                            continue;
                        } else {
                            info!("✋ Stopping escalation (condition not met: {})", escalate_condition);
                            results.push(level_result);
                            break;
                        }
                    }
                }
                
                results.push(level_result);
                
                // If no auto-escalate or no condition, stop here
                if !command_config.auto_escalate {
                    break;
                }
            } else {
                if let Some(ref failure_msg) = level_config.on_failure {
                    let msg = self.substitute_template(failure_msg, context);
                    warn!("❌ {}", msg);
                }
                
                results.push(level_result);
                break;
            }
        }
        
        Ok(VoiceCommandResult {
            crate_name: crate_name.to_string(),
            command_phrase: command_phrase.to_string(),
            levels_executed: results,
            final_level: current_level,
            total_duration_ms: results.iter().map(|r| r.duration_ms).sum(),
        })
    }

    /// Substitute template variables
    fn substitute_template(&self, template: &str, context: &VoiceHookContext) -> String {
        let mut result = template.to_string();
        
        // Replace standard variables
        result = result.replace("{crate_name}", &context.crate_name);
        result = result.replace("{version}", &context.version);
        
        if let Some(ref duration) = context.duration {
            result = result.replace("{duration}", duration);
        }
        
        if let Some(ref error) = context.error {
            result = result.replace("{error}", error);
        }
        
        if let Some(failed_count) = context.failed_count {
            result = result.replace("{failed_count}", &failed_count.to_string());
        }
        
        if let Some(score) = context.score {
            result = result.replace("{score}", &score.to_string());
        }
        
        if let Some(ref environment) = context.environment {
            result = result.replace("{environment}", environment);
        }
        
        // Replace custom variables
        for (key, value) in &context.custom {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        
        result
    }

    /// Get all registered crates with voice hooks
    pub fn get_registered_crates(&self) -> Vec<String> {
        self.hooks.keys().cloned().collect()
    }

    /// Check if voice hooks are enabled for a crate
    pub fn is_enabled(&self, crate_name: &str) -> bool {
        self.hooks.get(crate_name).map(|c| c.enabled).unwrap_or(false)
    }

    /// Get maximum level allowed for orchestration mode
    fn get_max_level_for_mode(mode: OrchestrationMode) -> u8 {
        match mode {
            OrchestrationMode::ScriptsOnly => 0,
            OrchestrationMode::LimitToMicrokernel => 1,
            OrchestrationMode::LimitToWASM => 2,
            OrchestrationMode::LimitToBinaries => 3,
            OrchestrationMode::NoVMs => 4,
            OrchestrationMode::FullEscalation => 5,
        }
    }

    /// Parse orchestration mode from voice command
    pub fn parse_orchestration_command(command: &str) -> Option<OrchestrationMode> {
        let lower = command.to_lowercase();
        
        if lower.contains("scripts only") || lower.contains("script only") {
            Some(OrchestrationMode::ScriptsOnly)
        } else if lower.contains("limit to microkernel") || lower.contains("microkernel only") {
            Some(OrchestrationMode::LimitToMicrokernel)
        } else if lower.contains("limit to wasm") || lower.contains("wasm only") {
            Some(OrchestrationMode::LimitToWASM)
        } else if lower.contains("limit to binaries") || lower.contains("binaries only") {
            Some(OrchestrationMode::LimitToBinaries)
        } else if lower.contains("no vms") || lower.contains("no virtual machines") {
            Some(OrchestrationMode::NoVMs)
        } else if lower.contains("full escalation") || lower.contains("all levels") {
            Some(OrchestrationMode::FullEscalation)
        } else {
            None
        }
    }
}

impl Default for VoiceHooksManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Example usage in Smart Crate Orchestrator
pub mod example {
    use super::*;

    pub async fn demonstrate_voice_hooks() -> Result<()> {
        let mut manager = VoiceHooksManager::new();
        
        // Register a crate with voice hooks
        let config = VoiceHooksConfig {
            enabled: true,
            persona: "zoe_technical".to_string(),
            priority: VoicePriority::Normal,
            thalamic_filter: true,
            events: VoiceHooksEvents {
                on_build_start: Some("Building {crate_name} version {version}".to_string()),
                on_build_success: Some("✅ Build complete for {crate_name} in {duration}".to_string()),
                on_build_failure: Some("❌ Build failed for {crate_name}: {error}".to_string()),
                on_test_success: Some("✅ All tests passed for {crate_name}".to_string()),
                on_qa_complete: Some("QA score: {score}/100 for {crate_name}".to_string()),
                on_test_start: None,
                on_test_failure: None,
                on_deploy_start: None,
                on_deploy_success: None,
                on_deploy_failure: None,
                on_qa_start: None,
                on_health_check: None,
                on_error: None,
            },
            commands: {
                let mut cmds = HashMap::new();
                cmds.insert("build this crate".to_string(), "cargo build --release".to_string());
                cmds.insert("run tests".to_string(), "cargo test".to_string());
                cmds
            },
        };
        
        manager.register_crate("my-smart-crate", config)?;
        
        // Trigger build start event
        let context = VoiceHookContext {
            crate_name: "my-smart-crate".to_string(),
            version: "7.3.1".to_string(),
            duration: None,
            error: None,
            failed_count: None,
            score: None,
            environment: None,
            custom: HashMap::new(),
        };
        
        manager.trigger_event("my-smart-crate", VoiceHookEvent::BuildStart, context).await?;
        
        // Execute a voice command
        let result = manager.execute_command("my-smart-crate", "build this crate").await?;
        println!("Command result: {}", result);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_voice_hooks_manager() {
        let manager = VoiceHooksManager::new();
        assert_eq!(manager.get_registered_crates().len(), 0);
    }

    #[test]
    fn test_template_substitution() {
        let manager = VoiceHooksManager::new();
        let context = VoiceHookContext {
            crate_name: "test-crate".to_string(),
            version: "1.0.0".to_string(),
            duration: Some("5s".to_string()),
            error: None,
            failed_count: None,
            score: Some(95),
            environment: None,
            custom: HashMap::new(),
        };
        
        let template = "Building {crate_name} v{version} took {duration}, score: {score}";
        let result = manager.substitute_template(template, &context);
        
        assert_eq!(result, "Building test-crate v1.0.0 took 5s, score: 95");
    }
}

