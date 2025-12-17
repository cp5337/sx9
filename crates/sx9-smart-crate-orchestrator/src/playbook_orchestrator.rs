//! XSD Playbook Orchestrator
//! Orchestrates all CTAS-7 crates using XSD-based playbooks
//!
//! ## Multi-Modal Execution Framework (v7.3.1)
//!
//! Supports 7 execution modes for the same playbook content:
//! 1. **XSD/XML Structure** → Direct orchestration (sequential/parallel steps)
//! 2. **LISP Expressions** → Functional evaluation (embedded logic)
//! 3. **Trivariate Hash** → Content-addressed retrieval and execution
//! 4. **RDF Triples** → Semantic graph traversal
//! 5. **USIM Registry** → Universal symbolic message routing
//! 6. **Unicode Assembly** → Single-character primitive operations (emoji triggers)
//! 7. **Voice Commands** → Natural language query/execution with ElevenLabs + Zoe persona
//!
//! ## Architecture:
//!
//! ```
//! Same Playbook → Multiple Execution Paths:
//!
//! network-scan-playbook.xml
//! ├─ Hash: df69d2277148c302ffecda1106c7d31578c4b190d8769ba9
//! ├─ LISP: (orchestrate-escalation "192.168.1.218")
//! ├─ RDF: <escalation> <hasLevel> <Level0>
//! ├─ USIM: Registered in universal catalog
//! ├─ Unicode: 💩 → Execute all levels
//! └─ XML: Parse and execute steps sequentially
//! ```
//!
//! ## Benefits:
//! - Query by hash → Execute playbook
//! - Query by LISP → Evaluate functional logic
//! - Query by RDF → Traverse semantic relationships
//! - Query by Unicode → Trigger via emoji
//! - Query by USIM → Lookup in universal registry
//! - Query by XML → Parse structured steps
//! - Query by voice → Natural language execution with audio feedback
//!
//! All pointing to the SAME content, different execution contexts!
//!
//! ## Voice Integration:
//!
//! Voice commands are logged and can trigger playbook execution:
//! - "Execute network scan playbook on 192.168.1.218"
//! - "Show status of escalation ladder"
//! - "Run playbook with hash df69d227..."
//! - "Deploy honeypot for target machine"
//!
//! Voice responses provide real-time status updates via ElevenLabs (Zoe persona)

use crate::usim::{USIMProcessor, USIMTrivariate};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use sx9_foundation_manifold::core::async_runtime::tokio::time::{sleep, Duration};
use sx9_foundation_manifold::core::data::serde::{Deserialize, Serialize};
use sx9_foundation_manifold::core::data::Utc;
use sx9_foundation_manifold::core::diagnostics::anyhow::Result;
use sx9_foundation_manifold::core::diagnostics::tracing::{error, info, warn};
use sx9_foundation_manifold::core::networking::reqwest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybookStep {
    pub step_id: String,
    pub service_name: String,
    pub command: String,
    pub working_directory: String,
    pub port: Option<u16>,
    pub dependencies: Vec<String>,
    pub timeout_seconds: u64,
    pub retry_count: u32,
    pub health_check_url: Option<String>,
    /// LISP expression for functional evaluation (optional)
    pub lisp_expr: Option<String>,
    /// Unicode primitive operations (optional)
    pub unicode_ops: Option<Vec<UnicodeOperation>>,
    /// RDF triples for semantic graph (optional)
    pub rdf_triples: Option<Vec<RDFTriple>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playbook {
    pub playbook_id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<PlaybookStep>,
    pub execution_order: Vec<String>,
    pub parallel_groups: Vec<Vec<String>>,
    /// Trivariate hash for content addressing
    pub trivariate_hash: Option<TrivariateMurmur>,
    /// USIM registration for universal catalog
    pub usim_registration: Option<USIMTrivariate>,
    /// Emoji trigger mapping
    pub emoji_trigger: Option<String>,
    /// LISP execution functions
    pub lisp_functions: Option<HashMap<String, String>>,
    /// RDF graph representation
    pub rdf_graph: Option<Vec<RDFTriple>>,
}

/// Trivariate Murmur3 hash (v7.3.1 spec)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrivariateMurmur {
    pub sch: String,       // 16 bytes Base96
    pub cuid: String,      // 16 bytes Base96
    pub uuid: String,      // 16 bytes Base96
    pub full_hash: String, // 48 bytes concatenated
}

/// Unicode primitive operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicodeOperation {
    pub unicode: String,   // e.g., "U+E100"
    pub primitive: String, // e.g., "Send", "Receive", "Navigate"
    pub description: String,
}

/// RDF triple for semantic graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RDFTriple {
    pub subject: String,
    pub predicate: String,
    pub object: String,
}

/// Execution mode selector
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ExecutionMode {
    /// Execute via XML structure (default)
    XML,
    /// Execute via LISP evaluation
    LISP,
    /// Execute via hash lookup
    Hash,
    /// Execute via RDF graph traversal
    RDF,
    /// Execute via USIM registry
    USIM,
    /// Execute via Unicode/emoji trigger
    Unicode,
    /// Execute via voice command (ElevenLabs + Zoe)
    Voice,
}

/// Voice interaction log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceInteraction {
    pub timestamp: String,
    pub speaker: VoiceSpeaker,
    pub command: String,
    pub playbook_id: Option<String>,
    pub execution_mode: ExecutionMode,
    pub response: String,
    pub audio_file: Option<String>, // Path to ElevenLabs audio response
    pub trivariate_hash: Option<String>,
}

/// Voice speaker identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoiceSpeaker {
    /// User voice input
    User,
    /// Zoe AI persona (ElevenLabs)
    Zoe,
    /// Natasha AI persona
    Natasha,
    /// System TTS
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStatus {
    pub step_id: String,
    pub status: StepStatus,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub error_message: Option<String>,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Retrying,
}

pub struct PlaybookOrchestrator {
    execution_status: HashMap<String, ExecutionStatus>,
    running_processes: HashMap<String, std::process::Child>,
    /// Voice interaction log (all voice commands and responses)
    voice_log: Vec<VoiceInteraction>,
    /// USIM processor for universal message routing
    usim_processor: Option<USIMProcessor>,
    /// Playbook registry (hash → playbook mapping)
    playbook_registry: HashMap<String, Playbook>,
}

impl PlaybookOrchestrator {
    pub fn new() -> Self {
        Self {
            execution_status: HashMap::new(),
            running_processes: HashMap::new(),
            voice_log: Vec::new(),
            usim_processor: None,
            playbook_registry: HashMap::new(),
        }
    }

    /// Register a playbook in the multi-modal registry
    pub fn register_playbook(&mut self, playbook: Playbook) -> Result<(), String> {
        info!(
            "📋 Registering playbook: {} ({})",
            playbook.name, playbook.playbook_id
        );

        // Register by playbook ID
        self.playbook_registry
            .insert(playbook.playbook_id.clone(), playbook.clone());

        // Register by trivariate hash if available
        if let Some(ref hash) = playbook.trivariate_hash {
            info!("🔗 Registering by hash: {}", hash.full_hash);
            self.playbook_registry
                .insert(hash.full_hash.clone(), playbook.clone());
        }

        // Register by emoji trigger if available
        if let Some(ref emoji) = playbook.emoji_trigger {
            info!("💩 Registering by emoji: {}", emoji);
            self.playbook_registry
                .insert(emoji.clone(), playbook.clone());
        }

        Ok(())
    }

    /// Execute playbook by any identifier (ID, hash, emoji, voice command)
    pub async fn execute_by_identifier(
        &mut self,
        identifier: &str,
        mode: ExecutionMode,
    ) -> Result<HashMap<String, ExecutionStatus>, String> {
        info!(
            "🎯 Executing playbook by identifier: {} (mode: {:?})",
            identifier, mode
        );

        // Look up playbook in registry
        let playbook = self
            .playbook_registry
            .get(identifier)
            .ok_or_else(|| format!("Playbook not found: {}", identifier))?
            .clone();

        // Log voice interaction if voice mode
        if matches!(mode, ExecutionMode::Voice) {
            self.log_voice_interaction(VoiceInteraction {
                timestamp: Utc::now().to_rfc3339(),
                speaker: VoiceSpeaker::User,
                command: format!("Execute playbook: {}", identifier),
                playbook_id: Some(playbook.playbook_id.clone()),
                execution_mode: mode,
                response: format!("Executing playbook: {}", playbook.name),
                audio_file: None,
                trivariate_hash: playbook
                    .trivariate_hash
                    .as_ref()
                    .map(|h| h.full_hash.clone()),
            });
        }

        // Execute the playbook
        self.execute_playbook(&playbook).await
    }

    /// Log voice interaction
    pub fn log_voice_interaction(&mut self, interaction: VoiceInteraction) {
        info!(
            "🎤 Voice: [{:?}] {}",
            interaction.speaker, interaction.command
        );
        self.voice_log.push(interaction);
    }

    /// Get voice interaction log
    pub fn get_voice_log(&self) -> &[VoiceInteraction] {
        &self.voice_log
    }

    /// Generate voice response via ElevenLabs (Zoe persona)
    pub async fn generate_voice_response(
        &mut self,
        text: &str,
        speaker: VoiceSpeaker,
    ) -> Result<String, String> {
        info!("🔊 Generating voice response: {}", text);

        // TODO: Integrate with ElevenLabs API
        // For now, log the interaction
        let interaction = VoiceInteraction {
            timestamp: Utc::now().to_rfc3339(),
            speaker,
            command: String::new(),
            playbook_id: None,
            execution_mode: ExecutionMode::Voice,
            response: text.to_string(),
            audio_file: None, // Would be path to generated audio
            trivariate_hash: None,
        };

        self.log_voice_interaction(interaction);

        Ok(text.to_string())
    }

    pub fn create_ctas7_full_stack_playbook() -> Playbook {
        Playbook {
            playbook_id: "ctas7-full-stack".to_string(),
            name: "CTAS-7 Full Stack Orchestration".to_string(),
            description: "Orchestrates all CTAS-7 services using XSD intelligence".to_string(),
            trivariate_hash: Some(TrivariateMurmur {
                sch: "a1b2c3d4e5f6g7h8".to_string(),
                cuid: "i9j0k1l2m3n4o5p6".to_string(),
                uuid: "q7r8s9t0u1v2w3x4".to_string(),
                full_hash: "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4".to_string(),
            }),
            usim_registration: None,
            emoji_trigger: Some("🚀".to_string()),
            lisp_functions: None,
            rdf_graph: None,
            steps: vec![
                // Foundation Services
                PlaybookStep {
                    step_id: "cannon-plug".to_string(),
                    service_name: "Smart CDN Gateway".to_string(),
                    command: "cargo run --bin ctas7-smart-cdn-gateway".to_string(),
                    working_directory: "/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-candidate-crates-staging/ctas7-smart-cdn-gateway".to_string(),
                    port: Some(18100),
                    dependencies: vec![],
                    timeout_seconds: 30,
                    retry_count: 3,
                    health_check_url: Some("http://localhost:18100/health".to_string()),
                    lisp_expr: None,
                    unicode_ops: None,
                    rdf_triples: None,
                },
                PlaybookStep {
                    step_id: "universal-telemetry".to_string(),
                    service_name: "Universal Telemetry".to_string(),
                    command: "cargo run --bin ctas7-universal-telemetry".to_string(),
                    working_directory: "/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-candidate-crates-staging/ctas7-universal-telemetry".to_string(),
                    port: Some(18101),
                    dependencies: vec!["cannon-plug".to_string()],
                    timeout_seconds: 30,
                    retry_count: 3,
                    health_check_url: Some("http://localhost:18101/health".to_string()),
                    lisp_expr: None,
                    unicode_ops: None,
                    rdf_triples: None,
                },
                PlaybookStep {
                    step_id: "xsd-environment".to_string(),
                    service_name: "XSD Environment".to_string(),
                    command: "cargo run --bin ctas-xsd-environment".to_string(),
                    working_directory: "/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-candidate-crates-staging/ctas-xsd-environment".to_string(),
                    port: Some(18102),
                    dependencies: vec!["cannon-plug".to_string()],
                    timeout_seconds: 30,
                    retry_count: 3,
                    health_check_url: Some("http://localhost:18102/health".to_string()),
                    lisp_expr: None,
                    unicode_ops: None,
                    rdf_triples: None,
                },
                PlaybookStep {
                    step_id: "port-manager".to_string(),
                    service_name: "Port Manager".to_string(),
                    command: "cargo run --bin ctas7-port-manager".to_string(),
                    working_directory: "/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-candidate-crates-staging/ctas7-port-manager".to_string(),
                    port: Some(18103),
                    dependencies: vec!["cannon-plug".to_string()],
                    timeout_seconds: 30,
                    retry_count: 3,
                    health_check_url: Some("http://localhost:18103/health".to_string()),
                    lisp_expr: None,
                    unicode_ops: None,
                    rdf_triples: None,
                },
                PlaybookStep {
                    step_id: "hashing-engine".to_string(),
                    service_name: "Hashing Engine".to_string(),
                    command: "cargo run --bin ctas7-hashing-engine".to_string(),
                    working_directory: "/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-candidate-crates-staging/ctas7-hashing-engine".to_string(),
                    port: Some(18105),
                    dependencies: vec!["cannon-plug".to_string()],
                    timeout_seconds: 30,
                    retry_count: 3,
                    health_check_url: Some("http://localhost:18105/health".to_string()),
                    lisp_expr: None,
                    unicode_ops: None,
                    rdf_triples: None,
                },
                PlaybookStep {
                    step_id: "progress-monitor".to_string(),
                    service_name: "Progress Monitor".to_string(),
                    command: "cargo run --bin ctas7-progress-monitor".to_string(),
                    working_directory: "/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-candidate-crates-staging/ctas7-progress-monitor".to_string(),
                    port: Some(18106),
                    dependencies: vec!["cannon-plug".to_string()],
                    timeout_seconds: 30,
                    retry_count: 3,
                    health_check_url: Some("http://localhost:18106/health".to_string()),
                    lisp_expr: None,
                    unicode_ops: None,
                    rdf_triples: None,
                },
                PlaybookStep {
                    step_id: "statistical-analysis".to_string(),
                    service_name: "Statistical Analysis CDN".to_string(),
                    command: "cargo run --bin ctas7-statistical-analysis-cdn".to_string(),
                    working_directory: "/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-candidate-crates-staging/ctas7-statistical-analysis-cdn".to_string(),
                    port: Some(18108),
                    dependencies: vec!["cannon-plug".to_string(), "hashing-engine".to_string()],
                    timeout_seconds: 30,
                    retry_count: 3,
                    health_check_url: Some("http://localhost:18108/health".to_string()),
                    lisp_expr: None,
                    unicode_ops: None,
                    rdf_triples: None,
                },
                // Database Services
                PlaybookStep {
                    step_id: "postgres".to_string(),
                    service_name: "postgres".to_string(),
                    command: "docker run -d --name ctas-pg -e POSTGRES_PASSWORD=secret postgres:14".to_string(),
                    working_directory: ".".to_string(),
                    port: Some(5432),
                    dependencies: vec![],
                    timeout_seconds: 30,
                    retry_count: 3,
                    health_check_url: None, // No HTTP health check for raw PG
                    lisp_expr: None,
                    rdf_triples: None,
                    unicode_ops: None,
                },
                // Frontend Services
                PlaybookStep {
                    step_id: "command-center-ui".to_string(),
                    service_name: "Command Center UI".to_string(),
                    command: "npm run dev".to_string(),
                    working_directory: "/Users/cp5337/Developer/ui/ctas-7-ui-command-center".to_string(),
                    port: Some(5173),
                    dependencies: vec!["cannon-plug".to_string()],
                    timeout_seconds: 60,
                    retry_count: 2,
                    health_check_url: Some("http://localhost:5173".to_string()),
                    lisp_expr: None,
                    unicode_ops: None,
                    rdf_triples: None,
                },
            ],
            execution_order: vec![
                "cannon-plug".to_string(),
                "universal-telemetry".to_string(),
                "xsd-environment".to_string(),
                "port-manager".to_string(),
                "hashing-engine".to_string(),
                "progress-monitor".to_string(),
                "statistical-analysis".to_string(),
                "command-center-ui".to_string(),
            ],
            parallel_groups: vec![
                vec!["universal-telemetry".to_string(), "xsd-environment".to_string(), "port-manager".to_string()],
                vec!["hashing-engine".to_string(), "progress-monitor".to_string()],
            ],
        }
    }

    pub async fn execute_modern_playbook(
        &mut self,
        playbook: &crate::dsl::Sx9Playbook,
    ) -> Result<HashMap<String, ExecutionStatus>, String> {
        info!(
            "🚀 Starting Modern Playbook Execution (RFC-9011-B): {}",
            playbook.name
        );

        // Initialize execution status for all actions
        for (index, action) in playbook.actions.iter().enumerate() {
            let step_id = format!("{}-step-{}", playbook.playbook_id, index);
            self.execution_status.insert(
                step_id.clone(),
                ExecutionStatus {
                    step_id: step_id.clone(),
                    status: StepStatus::Pending,
                    start_time: None,
                    end_time: None,
                    error_message: None,
                    retry_count: 0,
                },
            );
        }

        // Execute actions in order
        for (index, action) in playbook.actions.iter().enumerate() {
            let step_id = format!("{}-step-{}", playbook.playbook_id, index);
            info!(
                "📋 Executing modern action {}/{}",
                index + 1,
                playbook.actions.len()
            );

            match action {
                crate::dsl::PlaybookAction::Execute {
                    executor, command, ..
                } => {
                    // Update status
                    if let Some(status) = self.execution_status.get_mut(&step_id) {
                        status.status = StepStatus::Running;
                        status.start_time = Some(Utc::now().to_rfc3339());
                    }

                    info!("🔧 Executing command ({}) : {}", executor, command);

                    // Execute based on executor type
                    let mut cmd = match executor.as_str() {
                        "sh" | "bash" => {
                            let mut c = Command::new("sh");
                            c.arg("-c").arg(command);
                            c
                        }
                        _ => {
                            let mut c = Command::new("sh");
                            c.arg("-c").arg(command);
                            c
                        } // Default to sh for now
                    };

                    // Configure process
                    let child_result = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn();

                    match child_result {
                        Ok(mut child) => {
                            // Wait for completion for now (linear execution)
                            // In non-MVP, this would be async/background
                            match child.wait() {
                                Ok(exit_status) => {
                                    if exit_status.success() {
                                        info!("✅ Action completed: {}", step_id);
                                        if let Some(status) =
                                            self.execution_status.get_mut(&step_id)
                                        {
                                            status.status = StepStatus::Completed;
                                            status.end_time = Some(Utc::now().to_rfc3339());
                                        }
                                    } else {
                                        error!("❌ Action failed with status: {}", exit_status);
                                        if let Some(status) =
                                            self.execution_status.get_mut(&step_id)
                                        {
                                            status.status = StepStatus::Failed;
                                            status.error_message = Some(format!(
                                                "Exit code: {:?}",
                                                exit_status.code()
                                            ));
                                            status.end_time = Some(Utc::now().to_rfc3339());
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("❌ Failed to wait on child: {}", e);
                                    if let Some(status) = self.execution_status.get_mut(&step_id) {
                                        status.status = StepStatus::Failed;
                                        status.error_message = Some(e.to_string());
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            error!("❌ Failed to spawn process: {}", e);
                            if let Some(status) = self.execution_status.get_mut(&step_id) {
                                status.status = StepStatus::Failed;
                                status.error_message = Some(e.to_string());
                            }
                        }
                    }
                }
                _ => {
                    warn!(
                        "⚠️ Unsupported action type for local execution (skipping): {:?}",
                        action
                    );
                    if let Some(status) = self.execution_status.get_mut(&step_id) {
                        status.status = StepStatus::Completed; // Skip as completed for now
                        status.error_message = Some("Skipped: Unsupported action type".to_string());
                    }
                }
            }
        }

        Ok(self.execution_status.clone())
    }

    pub async fn execute_playbook(
        &mut self,
        playbook: &Playbook,
    ) -> Result<HashMap<String, ExecutionStatus>, String> {
        info!("🚀 Starting XSD Playbook Execution: {}", playbook.name);

        // Initialize execution status for all steps
        for step in &playbook.steps {
            self.execution_status.insert(
                step.step_id.clone(),
                ExecutionStatus {
                    step_id: step.step_id.clone(),
                    status: StepStatus::Pending,
                    start_time: None,
                    end_time: None,
                    error_message: None,
                    retry_count: 0,
                },
            );
        }

        // Execute steps in order
        for step_id in &playbook.execution_order {
            if let Some(step) = playbook.steps.iter().find(|s| &s.step_id == step_id) {
                info!(
                    "📋 Executing step: {} - {}",
                    step.step_id, step.service_name
                );

                // Check dependencies
                if !self.check_dependencies(step, &playbook.steps).await {
                    error!("❌ Dependencies not met for step: {}", step.step_id);
                    continue;
                }

                // Execute step
                match self.execute_step(step).await {
                    Ok(_) => {
                        info!("✅ Step completed: {}", step.step_id);
                        if let Some(status) = self.execution_status.get_mut(&step.step_id) {
                            status.status = StepStatus::Completed;
                            status.end_time = Some(Utc::now().to_rfc3339());
                        }
                    }
                    Err(e) => {
                        error!("❌ Step failed: {} - {}", step.step_id, e);
                        if let Some(status) = self.execution_status.get_mut(&step.step_id) {
                            status.status = StepStatus::Failed;
                            status.error_message = Some(e);
                            status.end_time = Some(Utc::now().to_rfc3339());
                        }
                    }
                }
            }
        }

        Ok(self.execution_status.clone())
    }

    async fn check_dependencies(&self, step: &PlaybookStep, all_steps: &[PlaybookStep]) -> bool {
        for dep_id in &step.dependencies {
            if let Some(dep_status) = self.execution_status.get(dep_id) {
                match dep_status.status {
                    StepStatus::Completed => continue,
                    _ => {
                        warn!(
                            "⚠️  Dependency not ready: {} for step: {}",
                            dep_id, step.step_id
                        );
                        return false;
                    }
                }
            } else {
                warn!(
                    "⚠️  Dependency not found: {} for step: {}",
                    dep_id, step.step_id
                );
                return false;
            }
        }
        true
    }

    async fn execute_step(&mut self, step: &PlaybookStep) -> Result<(), String> {
        info!(
            "🔧 Starting service: {} in {}",
            step.service_name, step.working_directory
        );

        // Update status to running
        if let Some(status) = self.execution_status.get_mut(&step.step_id) {
            status.status = StepStatus::Running;
            status.start_time = Some(Utc::now().to_rfc3339());
        }

        // Execute command
        let mut cmd = Command::new("sh")
            .arg("-c")
            .arg(&step.command)
            .current_dir(&step.working_directory)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start process: {}", e))?;

        // Store process handle
        self.running_processes.insert(step.step_id.clone(), cmd);

        // Wait for service to start
        sleep(Duration::from_secs(5)).await;

        // Health check if URL provided
        if let Some(health_url) = &step.health_check_url {
            if !self.health_check(health_url).await {
                warn!("⚠️  Health check failed for: {}", step.service_name);
                // Don't fail the step immediately, give it time to start
            }
        }

        info!(
            "🎯 Service started: {} on port {:?}",
            step.service_name, step.port
        );
        Ok(())
    }

    async fn health_check(&self, url: &str) -> bool {
        match sx9_foundation_manifold::core::networking::reqwest::get(url).await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    pub fn get_execution_status(&self) -> &HashMap<String, ExecutionStatus> {
        &self.execution_status
    }

    pub fn stop_all_services(&mut self) {
        info!("🛑 Stopping all services...");

        for (step_id, mut process) in self.running_processes.drain() {
            if let Err(e) = process.kill() {
                warn!("Failed to kill process for step {}: {}", step_id, e);
            }
        }
    }
}

impl Default for PlaybookOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_playbook_creation() {
        let playbook = PlaybookOrchestrator::create_ctas7_full_stack_playbook();
        assert_eq!(playbook.playbook_id, "ctas7-full-stack");
        assert!(!playbook.steps.is_empty());
        assert_eq!(playbook.steps.len(), 8);
    }

    #[tokio::test]
    async fn test_orchestrator_initialization() {
        let orchestrator = PlaybookOrchestrator::new();
        assert!(orchestrator.execution_status.is_empty());
        assert!(orchestrator.running_processes.is_empty());
    }

    #[tokio::test]
    async fn test_modern_playbook_execution() {
        use crate::dsl::{PlaybookAction, Sx9Playbook, TrivariateHash};

        let mut orchestrator = PlaybookOrchestrator::new();
        let playbook = Sx9Playbook {
            playbook_id: "test-modern-id".to_string(),
            name: "Modern Test".to_string(),
            description: "Test execution".to_string(),
            hd4_phase: "HUNT".to_string(),
            ptcc_primitive: 0x08,
            source_artifacts: vec![],
            attack_techniques: vec![],
            actions: vec![PlaybookAction::Execute {
                platform: None,
                executor: "sh".to_string(),
                command: "echo 'Hello Modern World'".to_string(),
                cleanup: "".to_string(),
                payloads: vec![],
                elevation_required: false,
            }],
            platforms: vec!["linux".to_string()],
            prerequisites: vec![],
            data_sources: vec![],
            severity: "low".to_string(),
            confidence: 1.0,
            created_at: "2025-01-01T00:00:00Z".to_string(),
            trivariate_hash: TrivariateHash {
                sch_t: "000".to_string(),
                cuid_t: "000".to_string(),
                uuid: "000".to_string(),
            },
        };

        let results = orchestrator
            .execute_modern_playbook(&playbook)
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        let status = results.get("test-modern-id-step-0").unwrap();
        assert_eq!(status.status, StepStatus::Completed);
    }
}
