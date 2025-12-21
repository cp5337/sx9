//! SX9 Dev Forge - Tauri Backend
//!
//! Prompt engineering, IDE bootstrap, and mission management.

mod ide;
mod linear;
pub mod missions;
mod rfc;
pub mod vault;
pub mod atomic_clipboard;
pub mod clipboard_commands;
pub mod file_index;  // NEW: File indexing for AI agent discovery
mod thalmic_filter;  // NEW: Plain language intent parser
mod key_onboarder;  // NEW: File-based key import
mod voice;  // NEW: Voice integration with ElevenLabs
mod qa; // NEW: Code Quality Integration

use ide::{BootstrapConfig, BootstrapConstraints, BootstrapContext, IdeBootstrap, IdeType};
use linear::{CreateIssueInput, LinearClient};
use missions::{Mission, MissionStore};
use rfc::RfcLoader;
use vault::{global_vault, KeyEntrySummary, VaultStats, KeyVaultExt};
use atomic_clipboard::AtomicClipboard;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::sync::Mutex;
use tauri::State;
use file_index::{FileIndex, FileEntry, IndexStats}; // NEW

// ═══════════════════════════════════════════════════════════════════════════════
// APP STATE
// ═══════════════════════════════════════════════════════════════════════════════

pub struct AppState {
    pub mission_store: Mutex<missions::MissionStore>,
    pub linear_api_key: Mutex<Option<String>>,
    pub rfc_base_path: Mutex<Option<std::path::PathBuf>>,
    pub atomic_clipboard: Mutex<atomic_clipboard::AtomicClipboard>,
    pub elevenlabs_api_key: Mutex<Option<String>>,
}

impl Default for AppState {
    fn default() -> Self {
        let store_path = missions::default_store_path();
        let mission_store = MissionStore::load(store_path).unwrap_or_else(|_| {
            MissionStore::new(missions::default_store_path())
        });

        // Try to load Linear API key from vault
        // NOTE: Disabled to avoid tokio runtime panic on startup
        // Key will be loaded lazily when needed via set_linear_api_key command
        let linear_api_key = None;

        if linear_api_key.is_some() {
            tracing::info!("Loaded Linear API key from vault");
        }

        // Initialize Atomic Clipboard
        let atomic_clipboard = AtomicClipboard::new()
            .unwrap_or_else(|e| {
                tracing::warn!("Failed to initialize Atomic Clipboard: {}", e);
                // Fallback to temp directory
                let temp_path = std::env::temp_dir().join("sx9-clipboard");
                AtomicClipboard::with_path(&temp_path)
                    .expect("Failed to create fallback clipboard")
            });

        tracing::info!("Atomic Clipboard initialized");

        Self {
            mission_store: Mutex::new(mission_store),
            linear_api_key: Mutex::new(linear_api_key),
            rfc_base_path: Mutex::new(None),
            atomic_clipboard: Mutex::new(atomic_clipboard),
            elevenlabs_api_key: Mutex::new(None),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS - VAULT
// ═══════════════════════════════════════════════════════════════════════════════

#[tauri::command]
async fn vault_list_keys() -> Result<Vec<String>, String> {
    let vault = global_vault().map_err(|e| e.to_string())?;
    Ok(vault.list_keys().await)
}

#[tauri::command]
async fn vault_list_entries() -> Result<Vec<KeyEntrySummary>, String> {
    let vault = global_vault().map_err(|e| e.to_string())?;
    Ok(vault.list_entries().await)
}

#[tauri::command]
async fn vault_get_key(name: String) -> Result<Option<String>, String> {
    let vault = global_vault().map_err(|e| e.to_string())?;
    Ok(vault.get(&name).await)
}

#[tauri::command]
async fn vault_set_key(name: String, value: String, service: String) -> Result<(), String> {
    let vault = global_vault().map_err(|e| e.to_string())?;
    vault.set(&name, &value, &service).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn vault_delete_key(name: String) -> Result<bool, String> {
    let vault = global_vault().map_err(|e| e.to_string())?;
    vault.delete(&name).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn vault_stats() -> Result<VaultStats, String> {
    let vault = global_vault().map_err(|e| e.to_string())?;
    Ok(vault.stats().await)
}

#[tauri::command]
async fn vault_activate_key(name: String) -> Result<bool, String> {
    let vault = global_vault().map_err(|e| e.to_string())?;
    vault.activate(&name).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn vault_deactivate_key(name: String) -> Result<bool, String> {
    let vault = global_vault().map_err(|e| e.to_string())?;
    vault.deactivate(&name).await.map_err(|e| e.to_string())
}

/// Get list of standard key names
#[tauri::command]
fn vault_standard_keys() -> Vec<&'static str> {
    vault::keys::ALL.to_vec()
}

// ═══════════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS - MISSIONS
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMissionInput {
    pub title: String,
    pub prompt_type: String,
    pub persona: String,
    pub harness: String,
    pub objective: String,
    pub hard_constraints: Vec<String>,
    pub soft_constraints: Vec<String>,
    pub deliverables: Vec<String>,
    pub rfcs: Vec<String>,
    pub priority: String,
    pub phase: String,
}

#[tauri::command]
async fn create_mission(
    input: CreateMissionInput,
    state: State<'_, AppState>,
) -> Result<Mission, String> {
    process_mission_creation(input, &state).await
}

pub async fn process_mission_creation(
    input: CreateMissionInput,
    state: &AppState,
) -> Result<Mission, String> {
    let mut mission = Mission::new(
        input.title.clone(),
        input.prompt_type.clone(),
        input.persona.clone(),
        input.harness.clone(),
    );
    mission.objective = input.objective.clone();
    mission.hard_constraints = input.hard_constraints.clone();
    mission.soft_constraints = input.soft_constraints.clone();
    mission.deliverables = input.deliverables.clone();
    mission.rfcs = input.rfcs.clone();
    mission.priority = input.priority.clone();
    mission.phase = input.phase.clone();

    let mut store = state.mission_store.lock().await;
    store.create(mission.clone()).map_err(|e| e.to_string())?;
    drop(store);

    // Auto-save to Atomic Clipboard
    let clipboard = state.atomic_clipboard.lock().await;
    let clipboard_entry = atomic_clipboard::ClipboardEntry {
        id: uuid::Uuid::new_v4().to_string(),
        content: format!(
            "Mission Created: {}\nPersona: {}\nPhase: {}\nObjective: {}",
            mission.title, mission.persona, mission.phase, mission.objective
        ),
        source: "mission".to_string(),
        tags: vec![
            "mission".to_string(),
            mission.phase.clone(),
            mission.persona.clone(),
        ],
        created_at: chrono::Utc::now(),
        metadata: serde_json::json!({
            "mission_id": mission.id,
            "title": mission.title,
            "persona": mission.persona,
            "phase": mission.phase,
            "priority": mission.priority,
        }),
    };
    
    clipboard.push(clipboard_entry).await
        .map_err(|e| format!("Failed to save to clipboard: {}", e))?;

    Ok(mission)
}

#[tauri::command]
async fn list_missions(state: State<'_, AppState>) -> Result<Vec<Mission>, String> {
    let store = state.mission_store.lock().await;
    Ok(store.list().into_iter().cloned().collect())
}

#[tauri::command]
async fn get_mission(id: String, state: State<'_, AppState>) -> Result<Option<Mission>, String> {
    let store = state.mission_store.lock().await;
    Ok(store.get(&id).cloned())
}

#[tauri::command]
async fn start_mission(id: String, state: State<'_, AppState>) -> Result<Mission, String> {
    let mut store = state.mission_store.lock().await;
    let mission = store.get_mut(&id).ok_or("Mission not found")?;
    mission.start();
    let result = mission.clone();
    store.save().map_err(|e| e.to_string())?;
    Ok(result)
}

#[tauri::command]
async fn checkpoint_mission(
    id: String,
    message: String,
    state: State<'_, AppState>,
) -> Result<Mission, String> {
    let mut store = state.mission_store.lock().await;
    let mission = store.get_mut(&id).ok_or("Mission not found")?;
    mission.checkpoint(message);
    let result = mission.clone();
    store.save().map_err(|e| e.to_string())?;
    Ok(result)
}

#[tauri::command]
async fn complete_mission(id: String, state: State<'_, AppState>) -> Result<Mission, String> {
    let mut store = state.mission_store.lock().await;
    let mission = store.get_mut(&id).ok_or("Mission not found")?;
    mission.complete();
    let result = mission.clone();
    store.save().map_err(|e| e.to_string())?;
    Ok(result)
}

#[tauri::command]
async fn fail_mission(
    id: String,
    reason: String,
    state: State<'_, AppState>,
) -> Result<Mission, String> {
    let mut store = state.mission_store.lock().await;
    let mission = store.get_mut(&id).ok_or("Mission not found")?;
    mission.fail(reason);
    let result = mission.clone();
    store.save().map_err(|e| e.to_string())?;
    Ok(result)
}

// ═══════════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS - LINEAR
// ═══════════════════════════════════════════════════════════════════════════════

#[tauri::command]
async fn set_linear_api_key(api_key: String, state: State<'_, AppState>) -> Result<(), String> {
    // Store in vault for persistence
    let vault = global_vault().map_err(|e| e.to_string())?;
    vault.set(vault::keys::LINEAR, &api_key, "api").await.map_err(|e| e.to_string())?;
    
    // Update app state
    let mut key = state.linear_api_key.lock().await;
    *key = Some(api_key);
    Ok(())
}

#[tauri::command]
async fn get_linear_api_key_status(state: State<'_, AppState>) -> Result<bool, String> {
    let key = state.linear_api_key.lock().await;
    Ok(key.is_some())
}

#[tauri::command]
async fn list_linear_teams(state: State<'_, AppState>) -> Result<Vec<linear::LinearTeam>, String> {
    let key = state.linear_api_key.lock().await;
    let api_key = key.clone().ok_or("Linear API key not set")?;
    drop(key);

    let client = LinearClient::new(api_key);
    client.list_teams().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_linear_projects(
    team_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<linear::LinearProject>, String> {
    let key = state.linear_api_key.lock().await;
    let api_key = key.clone().ok_or("Linear API key not set")?;
    drop(key);

    let client = LinearClient::new(api_key);
    client.list_projects(&team_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_linear_issue(
    title: String,
    description: String,
    team_id: String,
    project_id: Option<String>,
    labels: Vec<String>,
    state: State<'_, AppState>,
) -> Result<linear::LinearIssue, String> {
    let key = state.linear_api_key.lock().await;
    let api_key = key.clone().ok_or("Linear API key not set")?;
    drop(key);

    let client = LinearClient::new(api_key);
    let input = CreateIssueInput {
        title,
        description: Some(description),
        team_id,
        project_id,
        priority: None,
        labels,
    };
    client.create_issue(input).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_linear_issue_from_mission(
    mission_id: String,
    team_id: String,
    state: State<'_, AppState>,
) -> Result<linear::LinearIssue, String> {
    // Get mission data
    let mission_store = state.mission_store.lock().await;
    let mission = mission_store
        .get(&mission_id)
        .ok_or("Mission not found")?
        .clone();
    drop(mission_store);
    
    // Check if already linked
    if mission.linear_issue_id.is_some() {
        return Err("Mission already linked to Linear issue".to_string());
    }
    
    // Auto-generate labels based on persona, phase, priority
    let mut labels = Vec::new();
    
    // Persona → Label mapping
    match mission.persona.as_str() {
        "FORGE" => labels.push("dev".to_string()),
        "VECTOR" => labels.push("security".to_string()),
        "CIPHER" => labels.push("crypto".to_string()),
        "SENTINEL" => labels.push("monitoring".to_string()),
        "ATLAS" => labels.push("architecture".to_string()),
        _ => {}
    }
    
    // Phase → Label
    match mission.phase.as_str() {
        "PLANNING" => labels.push("planning".to_string()),
        "IMPLEMENTATION" => labels.push("in-progress".to_string()),
        "TESTING" => labels.push("qa".to_string()),
        "DEPLOYMENT" => labels.push("deploy".to_string()),
        _ => {}
    }
    
    // Priority → Linear priority level
    let priority = match mission.priority.as_str() {
        "CRITICAL" => Some(1), // Urgent
        "HIGH" => Some(2),     // High
        "MEDIUM" => Some(3),   // Normal
        "LOW" => Some(4),      // Low
        _ => None,
    };
    
    // Build description with mission context
    let description = format!(
        "## Mission Objective\n{}\n\n## Assigned Persona\n{}\n\n## Current Phase\n{}\n\n## Test Harness\n{}\n\n---\n*Auto-created from SX9 Dev Forge*",
        mission.objective,
        mission.persona,
        mission.phase,
        mission.harness
    );
    
    // Create Linear issue
    let key = state.linear_api_key.lock().await;
    let api_key = key.clone().ok_or("Linear API key not set")?;
    drop(key);
    
    let client = LinearClient::new(api_key);
    let input = CreateIssueInput {
        title: mission.title.clone(),
        description: Some(description),
        team_id,
        project_id: None,
        priority,
        labels,
    };
    
    let issue = client.create_issue(input).await.map_err(|e| e.to_string())?;
    
    // Update mission with Linear issue info
    let mut mission_store = state.mission_store.lock().await;
    if let Some(mission) = mission_store.get(&mission_id).cloned() {
        let mut updated_mission = mission;
        updated_mission.linear_issue_id = Some(issue.id.clone());
        updated_mission.linear_issue_url = Some(issue.url.clone());
        mission_store.update(updated_mission).map_err(|e| e.to_string())?;
    }
    drop(mission_store);
    
    // Save to Atomic Clipboard
    let clipboard_entry = atomic_clipboard::ClipboardEntry {
        id: uuid::Uuid::new_v4().to_string(),
        content: serde_json::to_string(&issue).unwrap_or_default(),
        source: "linear".to_string(),
        tags: vec!["linear".to_string(), "issue".to_string(), mission_id],
        created_at: chrono::Utc::now(),
        metadata: serde_json::json!({
            "issue_id": issue.id,
            "issue_url": issue.url,
            "identifier": issue.identifier,
        }),
    };
    
    if let Ok(clipboard) = atomic_clipboard::AtomicClipboard::new() {
        let _ = clipboard.push(clipboard_entry).await;
    }
    
    Ok(issue)
}

// ═══════════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS - VOICE & IDEATION
// ═══════════════════════════════════════════════════════════════════════════════

#[tauri::command]
async fn text_to_speech(
    text: String,
    state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    let key = state.elevenlabs_api_key.lock().await;
    let api_key = key.clone().ok_or("ElevenLabs API key not set")?;
    drop(key);

    let config = voice::VoiceConfig::default();
    let client = voice::VoiceClient::new(api_key, config);
    
    client.text_to_speech(&text, None).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_voices(state: State<'_, AppState>) -> Result<Vec<voice::Voice>, String> {
    let key = state.elevenlabs_api_key.lock().await;
    let api_key = key.clone().ok_or("ElevenLabs API key not set")?;
    drop(key);

    let config = voice::VoiceConfig::default();
    let client = voice::VoiceClient::new(api_key, config);
    
    client.list_voices().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn process_voice_idea(
    transcript: String,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    // Run through Thalmic Filter to extract structured data
    let mut filter = thalmic_filter::ThalmicFilter::new();
    let intent = filter.parse(&transcript);
    
    // Extract mission components
    let concept = serde_json::json!({
        "raw_idea": transcript,
        "intent": intent,
        "objective": extract_objective(&transcript),
        "potential_personas": extract_personas(&transcript),
        "constraints": extract_constraints(&transcript),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    // Save to Atomic Clipboard
    let clipboard_entry = atomic_clipboard::ClipboardEntry {
        id: uuid::Uuid::new_v4().to_string(),
        content: serde_json::to_string(&concept).unwrap_or_default(),
        source: "voice-ideation".to_string(),
        tags: vec!["ideation".to_string(), "voice".to_string()],
        created_at: chrono::Utc::now(),
        metadata: concept.clone(),
    };
    
    if let Ok(clipboard) = atomic_clipboard::AtomicClipboard::new() {
        let _ = clipboard.push(clipboard_entry).await;
    }
    
    Ok(concept)
}

// Helper functions for voice idea processing
fn extract_objective(text: &str) -> String {
    // Simple extraction - can be enhanced with NLP
    let lower = text.to_lowercase();
    if lower.contains("build") || lower.contains("create") || lower.contains("implement") {
        text.to_string()
    } else {
        format!("Implement: {}", text)
    }
}

fn extract_personas(text: &str) -> Vec<String> {
    let lower = text.to_lowercase();
    let mut personas = Vec::new();
    
    if lower.contains("security") || lower.contains("threat") {
        personas.push("VECTOR".to_string());
    }
    if lower.contains("architecture") || lower.contains("design") {
        personas.push("ATLAS".to_string());
    }
    if lower.contains("crypto") || lower.contains("encryption") {
        personas.push("CIPHER".to_string());
    }
    if lower.contains("monitor") || lower.contains("observability") {
        personas.push("SENTINEL".to_string());
    }
    
    if personas.is_empty() {
        personas.push("FORGE".to_string());
    }
    
    personas
}

fn extract_constraints(text: &str) -> Vec<String> {
    let mut constraints = Vec::new();
    let lower = text.to_lowercase();
    
    if lower.contains("fast") || lower.contains("performance") {
        constraints.push("High performance required".to_string());
    }
    if lower.contains("secure") || lower.contains("safe") {
        constraints.push("Security critical".to_string());
    }
    if lower.contains("simple") || lower.contains("minimal") {
        constraints.push("Keep implementation simple".to_string());
    }
    
    constraints
}

// ═══════════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS - KEYVAULT DIAGNOSTICS
// ═══════════════════════════════════════════════════════════════════════════════

#[tauri::command]
async fn diagnose_keyvault() -> Result<String, String> {
    let vault_path = vault::KeyVault::default_vault_dir();
    let db_path = vault_path.join("keys.sled");
    
    let mut report = String::new();
    report.push_str(&format!("🔍 KeyVault Diagnostic Report\n\n"));
    report.push_str(&format!("Path: {:?}\n", vault_path));
    report.push_str(&format!("Database: {:?}\n", db_path));
    report.push_str(&format!("Exists: {}\n\n", db_path.exists()));
    
    if !db_path.exists() {
        return Ok(report + "❌ Database does not exist");
    }
    
    match sled::open(&db_path) {
        Ok(db) => {
            report.push_str(&format!("✅ Database opened successfully\n"));
            report.push_str(&format!("Total entries: {}\n\n", db.len()));
            
            let mut count = 0;
            for item in db.iter().take(5) {
                if let Ok((key, value)) = item {
                    count += 1;
                    let key_str = String::from_utf8_lossy(&key);
                    report.push_str(&format!("Entry #{}:\n", count));
                    report.push_str(&format!("  Key: {}\n", key_str));
                    report.push_str(&format!("  Value size: {} bytes\n", value.len()));
                    
                    // Try to parse as KeyEntry
                    match serde_json::from_slice::<vault::KeyEntry>(&value) {
                        Ok(entry) => {
                            report.push_str("  ✅ Valid KeyEntry\n");
                            report.push_str(&format!("     Name: {}\n", entry.name));
                            report.push_str(&format!("     Service: {}\n", entry.service));
                            report.push_str(&format!("     Active: {}\n", entry.active));
                        }
                        Err(e) => {
                            report.push_str(&format!("  ❌ Parse error: {}\n", e));
                            // Show first 200 chars of value
                            if let Ok(json_str) = String::from_utf8(value.to_vec()) {
                                let preview = &json_str[..json_str.len().min(200)];
                                report.push_str(&format!("     Raw: {}...\n", preview));
                            }
                        }
                    }
                    report.push('\n');
                }
            }
            
            if db.len() > 5 {
                report.push_str(&format!("... and {} more entries\n", db.len() - 5));
            }
        }
        Err(e) => {
            report.push_str(&format!("❌ Failed to open database: {}\n", e));
        }
    }
    
    Ok(report)
}

// ═══════════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS - RFC
// ═══════════════════════════════════════════════════════════════════════════════

#[tauri::command]
async fn set_rfc_base_path(path: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut base_path = state.rfc_base_path.lock().await;
    *base_path = Some(PathBuf::from(path));
    Ok(())
}

#[tauri::command]
async fn load_rfc_index(state: State<'_, AppState>) -> Result<rfc::RfcIndex, String> {
    let base_path = state.rfc_base_path.lock().await;
    let path = base_path.clone().ok_or("RFC base path not set")?;
    drop(base_path);

    let loader = RfcLoader::new(path);
    loader.load_all().map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_rfcs(query: String, state: State<'_, AppState>) -> Result<Vec<rfc::RfcMeta>, String> {
    let base_path = state.rfc_base_path.lock().await;
    let path = base_path.clone().ok_or("RFC base path not set")?;
    drop(base_path);

    let loader = RfcLoader::new(path);
    let index = loader.load_all().map_err(|e| e.to_string())?;
    Ok(index.search(&query).into_iter().cloned().collect())
}

#[tauri::command]
async fn get_rfc_content(rfc_id: String, state: State<'_, AppState>) -> Result<String, String> {
    let base_path = state.rfc_base_path.lock().await;
    let path = base_path.clone().ok_or("RFC base path not set")?;
    drop(base_path);

    let loader = RfcLoader::new(path);
    loader.read_content(&rfc_id).map_err(|e| e.to_string())
}

// ═══════════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS - FILE INDEX
// ═══════════════════════════════════════════════════════════════════════════════

#[tauri::command]
async fn file_index_build() -> Result<usize, String> {
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let index = FileIndex::new(root).map_err(|e| e.to_string())?;
    index.index_workspace().map_err(|e| e.to_string())
}

#[tauri::command]
async fn file_index_search_tags(tags: Vec<String>) -> Result<Vec<FileEntry>, String> {
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let index = FileIndex::new(root).map_err(|e| e.to_string())?;
    index.search_by_tags(&tags).map_err(|e| e.to_string())
}

#[tauri::command]
async fn file_index_search_path(query: String) -> Result<Vec<FileEntry>, String> {
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let index = FileIndex::new(root).map_err(|e| e.to_string())?;
    index.search_by_path(&query).map_err(|e| e.to_string())
}

#[tauri::command]
async fn file_index_recent(limit: usize) -> Result<Vec<FileEntry>, String> {
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let index = FileIndex::new(root).map_err(|e| e.to_string())?;
    index.recent(limit).map_err(|e| e.to_string())
}

#[tauri::command]
async fn file_index_stats() -> Result<IndexStats, String> {
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let index = FileIndex::new(root).map_err(|e| e.to_string())?;
    index.stats().map_err(|e| e.to_string())
}

#[tauri::command]
async fn file_index_tag(relative_path: String, tag: String) -> Result<(), String> {
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let index = FileIndex::new(root).map_err(|e| e.to_string())?;
    index.tag_file(&relative_path, tag).map_err(|e| e.to_string())
}

// ═══════════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS - KEY ONBOARDING
// ═══════════════════════════════════════════════════════════════════════════════

#[tauri::command]
async fn scan_and_import_keys() -> Result<Vec<String>, String> {
    let onboarder = key_onboarder::KeyOnboarder::new();
    onboarder.scan_and_import().await
}

#[tauri::command]
fn get_keys_directory() -> String {
    let onboarder = key_onboarder::KeyOnboarder::new();
    onboarder.keys_dir_path()
}

// ═══════════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS - IDE BOOTSTRAP
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapIdeInput {
    pub ide: String,
    pub project_path: String,
    pub persona: String,
    pub harness: String,
    pub mission_title: String,
    pub mission_id: String,
    pub hard_constraints: Vec<String>,
    pub soft_constraints: Vec<String>,
    pub forbidden_paths: Vec<String>,
    pub max_file_lines: Option<u32>,
    pub rfcs: Vec<String>,
    pub linear_issue: Option<String>,
    pub working_directory: String,
}

#[tauri::command]
fn bootstrap_ide(input: BootstrapIdeInput) -> Result<Vec<String>, String> {
    let ide = match input.ide.to_lowercase().as_str() {
        "cursor" => IdeType::Cursor,
        "vscode" | "code" => IdeType::Vscode,
        "antigravity" => IdeType::Antigravity,
        _ => return Err(format!("Unsupported IDE: {}", input.ide)),
    };

    let config = BootstrapConfig {
        ide,
        project_path: PathBuf::from(&input.project_path),
        persona: input.persona,
        harness: input.harness,
        mission_title: input.mission_title,
        mission_id: input.mission_id,
        constraints: BootstrapConstraints {
            hard: input.hard_constraints,
            soft: input.soft_constraints,
            forbidden_paths: input.forbidden_paths,
            max_file_lines: input.max_file_lines,
        },
        context: BootstrapContext {
            rfcs: input.rfcs,
            linear_issue: input.linear_issue,
            working_directory: input.working_directory,
        },
    };

    let bootstrap = IdeBootstrap::new(config);
    let written = bootstrap.write_files().map_err(|e| e.to_string())?;
    
    Ok(written.into_iter().map(|p| p.display().to_string()).collect())
}

#[tauri::command]
fn open_in_ide(ide: String, project_path: String) -> Result<(), String> {
    let ide_type = match ide.to_lowercase().as_str() {
        "cursor" => IdeType::Cursor,
        "vscode" | "code" => IdeType::Vscode,
        "antigravity" => IdeType::Antigravity,
        _ => return Err(format!("Unsupported IDE: {}", ide)),
    };

    ide::open_ide(ide_type, &PathBuf::from(project_path)).map_err(|e| e.to_string())
}

// ═══════════════════════════════════════════════════════════════════════════════
// APP INITIALIZATION
// ═══════════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS - NOTIFICATIONS (SLACK)
// ═══════════════════════════════════════════════════════════════════════════════

#[tauri::command]
async fn send_slack(channel: Option<String>, message: String) -> Result<String, String> {
    use reqwest::Client;
    use serde_json::json;

    let token = std::env::var("SLACK_BOT_TOKEN")
        .map_err(|_| "SLACK_BOT_TOKEN not found in environment".to_string())?;

    let client = Client::new();
    let res = client.post("https://slack.com/api/chat.postMessage")
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "channel": channel.unwrap_or_else(|| "#general".to_string()),
            "text": message
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Slack API Error: {}", res.status()));
    }

    let body = res.text().await.map_err(|e| e.to_string())?;
    Ok(body)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize vault early
    /* 
    match global_vault() {
        Ok(vault) => {
            // Log vault status
            // let rt = tokio::runtime::Runtime::new().unwrap();
            // let stats = rt.block_on(vault.stats());
            // println!("🔐 KeyVault loaded: {} keys ({} active)", stats.total, stats.active);
            println!("🔐 KeyVault loaded (stats disabled to avoid tokio panic)");
        }
        Err(e) => {
            eprintln!("⚠️ KeyVault failed to load: {}", e);
        }
    }
    */

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            // Vault
            vault_list_keys,
            vault_list_entries,
            vault_get_key,
            vault_set_key,
            vault_delete_key,
            vault_stats,
            vault_activate_key,
            vault_deactivate_key,
            vault_standard_keys,
            // Missions
            create_mission,
            list_missions,
            get_mission,
            start_mission,
            checkpoint_mission,
            complete_mission,
            fail_mission,
            // Linear
            set_linear_api_key,
            get_linear_api_key_status,
            list_linear_teams,
            list_linear_projects,
            create_linear_issue,
            create_linear_issue_from_mission,
            // RFC
            set_rfc_base_path,
            load_rfc_index,
            search_rfcs,
            get_rfc_content,
            // File Index
            file_index_build,
            file_index_search_tags,
            file_index_search_path,
            file_index_recent,
            file_index_stats,
            file_index_tag,
            // Key Onboarding
            scan_and_import_keys,
            get_keys_directory,
            // KeyVault Diagnostics
            diagnose_keyvault,
            // IDE
            bootstrap_ide,
            open_in_ide,
            // Notifications
            send_slack,
            // Atomic Clipboard
            clipboard_commands::clipboard_push,
            clipboard_commands::clipboard_list,
            clipboard_commands::clipboard_get,
            clipboard_commands::clipboard_search_by_tag,
            clipboard_commands::clipboard_search_by_source,
            clipboard_commands::clipboard_stats,
            clipboard_commands::clipboard_clear,
            
            // QA
            qa::run_lightning_qa
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
