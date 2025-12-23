#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use serde::{Deserialize, Serialize};
use reqwest;
use tokio;

mod commands;
use commands::forge::*;

#[tauri::command]
fn save_to_disk(content: String, filename: String) -> Result<String, String> {
    use std::fs;
    
    let app_dir = dirs::document_dir()
        .ok_or("Could not find documents directory")?;
    
    let cx9_dir = app_dir.join("CX9");
    fs::create_dir_all(&cx9_dir)
        .map_err(|e| format!("Failed to create directory: {}", e))?;
    
    let file_path = cx9_dir.join(&filename);
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    
    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
fn load_from_disk(filename: String) -> Result<String, String> {
    use std::fs;
    
    let app_dir = dirs::document_dir()
        .ok_or("Could not find documents directory")?;
    
    let file_path = app_dir.join("CX9").join(&filename);
    fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
fn list_files(directory: String) -> Result<Vec<String>, String> {
    use std::fs;
    
    let app_dir = dirs::document_dir()
        .ok_or("Could not find documents directory")?;
    
    let target_dir = app_dir.join("CX9").join(&directory);
    let entries = fs::read_dir(&target_dir)
        .map_err(|e| format!("Failed to read directory: {}", e))?;
    
    let files: Vec<String> = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.file_name().to_str().map(String::from)
            })
        })
        .collect();
    
    Ok(files)
}

#[tauri::command]
fn get_system_info() -> serde_json::Value {
    serde_json::json!({
        "platform": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "version": env!("CARGO_PKG_VERSION"),
    })
}

#[derive(Serialize, Deserialize)]
struct QAGateResult {
    passed: bool,
    message: String,
    details: Option<serde_json::Value>,
}

#[tauri::command]
fn execute_rust_pattern(pattern_name: String, input: String) -> Result<String, String> {
    // Placeholder for Rust pattern execution
    // This would call into your forge-unified-v5 Rust patterns
    match pattern_name.as_str() {
        "validate" => Ok(format!("Validated: {}", input)),
        "transform" => Ok(input.to_uppercase()),
        "analyze" => Ok(format!("Analysis: {} chars", input.len())),
        _ => Err(format!("Unknown pattern: {}", pattern_name)),
    }
}

#[tauri::command]
fn run_qa_gate(gate_name: String, data: serde_json::Value) -> Result<QAGateResult, String> {
    // Placeholder for QA gate execution
    // This would call into your forge-unified-v5 Python QA gates
    Ok(QAGateResult {
        passed: true,
        message: format!("QA gate '{}' passed", gate_name),
        details: Some(data),
    })
}

#[tauri::command]
async fn create_linear_issue(
    api_key: String,
    team_id: String,
    title: String,
    description: String,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    
    let query = format!(
        r#"mutation {{
            issueCreate(input: {{
                teamId: "{}",
                title: "{}",
                description: "{}"
            }}) {{
                success
                issue {{
                    id
                    identifier
                    url
                }}
            }}
        }}"#,
        team_id, title, description
    );
    
    let response = client
        .post("https://api.linear.app/graphql")
        .header("Authorization", api_key)
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await
        .map_err(|e| format!("Linear API error: {}", e))?;
    
    let result = response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    Ok(result)
}

#[tauri::command]
async fn send_slack_notification(
    webhook_url: String,
    channel: String,
    message: String,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    
    let payload = serde_json::json!({
        "channel": channel,
        "text": message,
    });
    
    client
        .post(&webhook_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Slack API error: {}", e))?;
    
    Ok("Notification sent".to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            save_to_disk,
            load_from_disk,
            list_files,
            get_system_info,
            execute_rust_pattern,
            run_qa_gate,
            create_linear_issue,
            send_slack_notification,
            // Forge commands
            save_prompt,
            create_linear_issue_forge,
            notify_slack,
            copy_to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
