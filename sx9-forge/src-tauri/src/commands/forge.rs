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
    use sx9_foundation_interface::{Client, Result as InterfaceResult};
    
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
    _app: tauri::AppHandle,
    content: String,
) -> Result<(), String> {
    // TODO: Implement clipboard functionality with tauri-plugin-clipboard
    // For now, just log the content
    println!("Would copy to clipboard: {}", content);
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
    // TODO: Implement actual Leptose health check
    // For now, return disconnected (service not running)
    Ok(ServiceCheckResult { ready: false })
}

/// Check if ChromaDB vector database is available
#[tauri::command]
pub async fn check_chroma() -> Result<ServiceCheckResult, String> {
    // TODO: Implement actual ChromaDB health check
    // For now, return disconnected (service not running)
    Ok(ServiceCheckResult { ready: false })
}

