use crate::{atomic_clipboard, AppState};
use tauri::State;

// ═══════════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS - ATOMIC CLIPBOARD
// ═══════════════════════════════════════════════════════════════════════════════

#[tauri::command]
pub async fn clipboard_push(
    content: String,
    source: String,
    tags: Vec<String>,
    metadata: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let entry = atomic_clipboard::ClipboardEntry {
        id: uuid::Uuid::new_v4().to_string(),
        content,
        source,
        tags,
        created_at: chrono::Utc::now(),
        metadata,
    };
    
    let clipboard = state.atomic_clipboard.lock().await;
    clipboard.push(entry).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn clipboard_list(
    limit: usize,
    state: State<'_, AppState>,
) -> Result<Vec<atomic_clipboard::ClipboardEntry>, String> {
    let clipboard = state.atomic_clipboard.lock().await;
    clipboard.list(limit).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn clipboard_get(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<atomic_clipboard::ClipboardEntry>, String> {
    let clipboard = state.atomic_clipboard.lock().await;
    clipboard.get(&id).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn clipboard_search_by_tag(
    tag: String,
    state: State<'_, AppState>,
) -> Result<Vec<atomic_clipboard::ClipboardEntry>, String> {
    let clipboard = state.atomic_clipboard.lock().await;
    clipboard.search_by_tag(&tag).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn clipboard_search_by_source(
    source: String,
    state: State<'_, AppState>,
) -> Result<Vec<atomic_clipboard::ClipboardEntry>, String> {
    let clipboard = state.atomic_clipboard.lock().await;
    clipboard.search_by_source(&source).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn clipboard_stats(
    state: State<'_, AppState>,
) -> Result<atomic_clipboard::ClipboardStats, String> {
    let clipboard = state.atomic_clipboard.lock().await;
    Ok(clipboard.stats().await)
}

#[tauri::command]
pub async fn clipboard_clear(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let clipboard = state.atomic_clipboard.lock().await;
    clipboard.clear().await
        .map_err(|e| e.to_string())
}
