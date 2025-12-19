//! SX9 Dev Forge - Tauri Backend
//!
//! Prompt engineering, IDE bootstrap, and mission management.

mod ide;
mod linear;
mod missions;
mod rfc;
mod vault;
pub mod atomic_clipboard;
pub mod clipboard_commands;
pub mod file_index;  // NEW: File indexing for AI agent discovery
mod thalmic_filter;  // NEW: Plain language intent parser
mod key_onboarder;  // NEW: File-based key import

use ide::{BootstrapConfig, BootstrapConstraints, BootstrapContext, IdeBootstrap, IdeType};
use linear::{CreateIssueInput, LinearClient};
use missions::{Mission, MissionStore};
use rfc::RfcLoader;
use vault::{global_vault, KeyEntrySummary, VaultStats};
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
    pub mission_store: Mutex<MissionStore>,
    pub linear_api_key: Mutex<Option<String>>,
    pub rfc_base_path: Mutex<Option<PathBuf>>,
    pub atomic_clipboard: Mutex<AtomicClipboard>,
}

impl Default for AppState {
    fn default() -> Self {
        let store_path = missions::default_store_path();
        let mission_store = MissionStore::load(store_path).unwrap_or_else(|_| {
            MissionStore::new(missions::default_store_path())
        });

        // Try to load Linear API key from vault
        let linear_api_key = if let Ok(vault) = global_vault() {
            // Use tokio runtime to get the key
            let rt = tokio::runtime::Runtime::new().ok();
            rt.and_then(|rt| {
                rt.block_on(async {
                    vault.get(vault::keys::LINEAR).await
                })
            })
        } else {
            None
        };

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize vault early
    match global_vault() {
        Ok(vault) => {
            // Log vault status
            let rt = tokio::runtime::Runtime::new().unwrap();
            let stats = rt.block_on(vault.stats());
            println!("🔐 KeyVault loaded: {} keys ({} active)", stats.total, stats.active);
            println!("   Path: {}", stats.vault_path);
        }
        Err(e) => {
            eprintln!("⚠️ KeyVault failed to load: {}", e);
        }
    }

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
            // IDE
            bootstrap_ide,
            open_in_ide,
            // Atomic Clipboard
            clipboard_commands::clipboard_push,
            clipboard_commands::clipboard_list,
            clipboard_commands::clipboard_get,
            clipboard_commands::clipboard_search_by_tag,
            clipboard_commands::clipboard_search_by_source,
            clipboard_commands::clipboard_stats,
            clipboard_commands::clipboard_clear,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
