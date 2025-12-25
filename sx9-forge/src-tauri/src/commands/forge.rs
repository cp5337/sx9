//! SX9 Prompt Forge - Tauri Commands
//!
//! These commands are invoked from the TypeScript frontend via `invoke()`.
//! They handle:
//! - File operations (save prompts)
//! - Linear API integration
//! - Slack notifications
//! - Clipboard operations

use serde::Serialize;
use std::fs;
use std::path::PathBuf;

// ============================================================================
// RESPONSE TYPES
// ============================================================================

#[derive(Debug, Serialize)]
pub struct SavePromptResult {
    pub success: bool,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LinearIssueResult {
    pub success: bool,
    pub issue_id: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SlackNotifyResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// ============================================================================
// COMMANDS
// ============================================================================

/// Save a prompt YAML file to disk
#[tauri::command]
pub async fn save_prompt(
    filename: String,
    content: String,
    workdir: String,
) -> Result<SavePromptResult, String> {
    // Get the prompts directory
    let base_path = if workdir.starts_with('/') || workdir.starts_with('~') {
        // Expand tilde manually
        let expanded = if workdir.starts_with("~/") {
            if let Some(home) = std::env::var("HOME").ok() {
                PathBuf::from(home).join(&workdir[2..])
            } else {
                PathBuf::from(&workdir)
            }
        } else {
            PathBuf::from(&workdir)
        };
        expanded
    } else {
        // Default to current directory + workdir
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(&workdir)
    };

    // Ensure directory exists
    if let Err(e) = fs::create_dir_all(&base_path) {
        return Ok(SavePromptResult {
            success: false,
            path: String::new(),
            error: Some(format!("Failed to create directory: {}", e)),
        });
    }

    let file_path = base_path.join(&filename);

    // Write the file
    match fs::write(&file_path, &content) {
        Ok(_) => Ok(SavePromptResult {
            success: true,
            path: file_path.to_string_lossy().to_string(),
            error: None,
        }),
        Err(e) => Ok(SavePromptResult {
            success: false,
            path: String::new(),
            error: Some(format!("Failed to write file: {}", e)),
        }),
    }
}

/// Create a Linear issue via their GraphQL API
#[tauri::command]
pub async fn create_linear_issue_forge(
    title: String,
    description: String,
    team_id: String,
) -> Result<LinearIssueResult, String> {
    use sx9_foundation_interface::Client;
    
    // Get Linear API key from environment
    let api_key = match std::env::var("LINEAR_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Ok(LinearIssueResult {
                success: false,
                issue_id: String::new(),
                url: String::new(),
                error: Some("LINEAR_API_KEY not set".to_string()),
            });
        }
    };

    // Build the GraphQL mutation
    let query = r#"
        mutation CreateIssue($title: String!, $description: String, $teamId: String!) {
            issueCreate(input: {
                title: $title
                description: $description
                teamId: $teamId
            }) {
                success
                issue {
                    id
                    identifier
                    url
                }
            }
        }
    "#;

    let variables = serde_json::json!({
        "title": title,
        "description": description,
        "teamId": team_id,
    });

    let payload = serde_json::json!({
        "query": query,
        "variables": variables,
    });

    // Make the request using foundation interface
    let client = Client::new();
    let response = client
        .post("https://api.linear.app/graphql")
        .header("Authorization", api_key)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                let body: serde_json::Value = res.json().await.unwrap_or_default();
                
                if let Some(issue) = body
                    .get("data")
                    .and_then(|d| d.get("issueCreate"))
                    .and_then(|ic| ic.get("issue"))
                {
                    Ok(LinearIssueResult {
                        success: true,
                        issue_id: issue
                            .get("identifier")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        url: issue
                            .get("url")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        error: None,
                    })
                } else {
                    Ok(LinearIssueResult {
                        success: false,
                        issue_id: String::new(),
                        url: String::new(),
                        error: Some("Failed to parse Linear response".to_string()),
                    })
                }
            } else {
                Ok(LinearIssueResult {
                    success: false,
                    issue_id: String::new(),
                    url: String::new(),
                    error: Some(format!("Linear API error: {}", res.status())),
                })
            }
        }
        Err(e) => Ok(LinearIssueResult {
            success: false,
            issue_id: String::new(),
            url: String::new(),
            error: Some(format!("Request failed: {}", e)),
        }),
    }
}

/// Send a notification to Slack
#[tauri::command]
pub async fn notify_slack(
    channel: String,
    message: String,
) -> Result<SlackNotifyResult, String> {
    use sx9_foundation_interface::Client;
    
    // Get Slack webhook URL from environment
    let webhook_url = match std::env::var("SLACK_WEBHOOK_URL") {
        Ok(url) => url,
        Err(_) => {
            return Ok(SlackNotifyResult {
                success: false,
                error: Some("SLACK_WEBHOOK_URL not set".to_string()),
            });
        }
    };

    let payload = serde_json::json!({
        "channel": channel,
        "text": message,
        "unfurl_links": false,
    });

    let client = Client::new();
    let response = client
        .post(&webhook_url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                Ok(SlackNotifyResult {
                    success: true,
                    error: None,
                })
            } else {
                Ok(SlackNotifyResult {
                    success: false,
                    error: Some(format!("Slack API error: {}", res.status())),
                })
            }
        }
        Err(e) => Ok(SlackNotifyResult {
            success: false,
            error: Some(format!("Request failed: {}", e)),
        }),
    }
}

/// Copy text to system clipboard
#[tauri::command]
pub async fn copy_to_clipboard(
    app: tauri::AppHandle,
    content: String,
) -> Result<(), String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;

    app.clipboard()
        .write_text(&content)
        .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;

    Ok(())
}

// ============================================================================
// PLUGIN REGISTRATION
// ============================================================================

// ============================================================================
// SERVICE STATUS CHECKS
// ============================================================================

#[derive(Debug, Serialize)]
pub struct ServiceCheckResult {
    pub ready: bool,
}

/// Check if Leptose inference service is available
#[tauri::command]
pub async fn check_leptose() -> Result<ServiceCheckResult, String> {
    use sx9_foundation_interface::Client;
    use std::time::Duration;

    // Get Leptose URL from env or use default
    let leptose_url = std::env::var("LEPTOSE_URL")
        .unwrap_or_else(|_| "http://localhost:11434".to_string());

    let client = Client::new();
    let health_url = format!("{}/api/tags", leptose_url);

    // Attempt health check with 2 second timeout
    match tokio::time::timeout(
        Duration::from_secs(2),
        client.get(&health_url).send(),
    )
    .await
    {
        Ok(Ok(response)) => Ok(ServiceCheckResult {
            ready: response.status().is_success(),
        }),
        _ => Ok(ServiceCheckResult { ready: false }),
    }
}

/// Check if ChromaDB vector database is available
#[tauri::command]
pub async fn check_chroma() -> Result<ServiceCheckResult, String> {
    use sx9_foundation_interface::Client;
    use std::time::Duration;

    // Get ChromaDB URL from env or use default
    let chroma_url = std::env::var("CHROMADB_URL")
        .unwrap_or_else(|_| "http://localhost:8000".to_string());

    let client = Client::new();
    let health_url = format!("{}/api/v1/heartbeat", chroma_url);

    // Attempt health check with 2 second timeout
    match tokio::time::timeout(
        Duration::from_secs(2),
        client.get(&health_url).send(),
    )
    .await
    {
        Ok(Ok(response)) => Ok(ServiceCheckResult {
            ready: response.status().is_success(),
        }),
        _ => Ok(ServiceCheckResult { ready: false }),
    }
}

// ============================================================================
// FILE DIALOG AND TEMPLATES
// ============================================================================

#[derive(Debug, Serialize)]
pub struct OpenFileResult {
    pub success: bool,
    pub path: Option<String>,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Open a file dialog to select a YAML prompt file
#[tauri::command]
pub async fn open_file_dialog(
    app: tauri::AppHandle,
    filters: Vec<String>,
) -> Result<OpenFileResult, String> {
    use tauri_plugin_dialog::DialogExt;

    // Build file filter from extensions
    let filter_name = "Prompt Files";
    let extensions: Vec<&str> = filters.iter().map(|s| s.as_str()).collect();

    let file_path = app
        .dialog()
        .file()
        .add_filter(filter_name, &extensions)
        .blocking_pick_file();

    match file_path {
        Some(path) => {
            let path_str = path.to_string();
            // Read the file content
            match std::fs::read_to_string(&path_str) {
                Ok(content) => Ok(OpenFileResult {
                    success: true,
                    path: Some(path_str),
                    content: Some(content),
                    error: None,
                }),
                Err(e) => Ok(OpenFileResult {
                    success: false,
                    path: Some(path_str),
                    content: None,
                    error: Some(format!("Failed to read file: {}", e)),
                }),
            }
        }
        None => Ok(OpenFileResult {
            success: false,
            path: None,
            content: None,
            error: Some("No file selected".to_string()),
        }),
    }
}

#[derive(Debug, Serialize)]
pub struct TemplateInfo {
    pub name: String,
    pub path: String,
    pub modified: u64,
}

#[derive(Debug, Serialize)]
pub struct ListTemplatesResult {
    pub success: bool,
    pub templates: Vec<TemplateInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// List available prompt templates from the templates directory
#[tauri::command]
pub async fn list_templates() -> Result<ListTemplatesResult, String> {
    use std::time::UNIX_EPOCH;

    // Get templates directory (~/Documents/CX9/templates or ~/CX9/templates)
    let templates_dir = if let Some(docs) = dirs::document_dir() {
        docs.join("CX9").join("templates")
    } else if let Some(home) = dirs::home_dir() {
        home.join("CX9").join("templates")
    } else {
        return Ok(ListTemplatesResult {
            success: false,
            templates: vec![],
            error: Some("Could not determine templates directory".to_string()),
        });
    };

    // Create directory if it doesn't exist
    if !templates_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&templates_dir) {
            return Ok(ListTemplatesResult {
                success: false,
                templates: vec![],
                error: Some(format!("Failed to create templates directory: {}", e)),
            });
        }
    }

    // Read directory entries
    let entries = match std::fs::read_dir(&templates_dir) {
        Ok(entries) => entries,
        Err(e) => {
            return Ok(ListTemplatesResult {
                success: false,
                templates: vec![],
                error: Some(format!("Failed to read templates directory: {}", e)),
            });
        }
    };

    let mut templates: Vec<TemplateInfo> = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();

        // Only include .yaml and .yml files
        if let Some(ext) = path.extension() {
            if ext == "yaml" || ext == "yml" {
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let modified = entry
                    .metadata()
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

                templates.push(TemplateInfo {
                    name,
                    path: path.to_string_lossy().to_string(),
                    modified,
                });
            }
        }
    }

    // Sort by modified time (newest first)
    templates.sort_by(|a, b| b.modified.cmp(&a.modified));

    Ok(ListTemplatesResult {
        success: true,
        templates,
        error: None,
    })
}
