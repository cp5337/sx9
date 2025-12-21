//! SX9 MCP Server for Claude Desktop & Grok
//! 
//! Exposes:
//! - Atomic Clipboard (read/write)
//! - File Index (search repos)
//! - Artifacts (Claude brain directory access)

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use tokio::runtime::Runtime;

// Import parent modules
use sx9_dev_forge_lib::atomic_clipboard::{AtomicClipboard, ClipboardEntry};
use sx9_dev_forge_lib::file_index::FileIndex;

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Value,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

fn main() {
    let rt = Runtime::new().unwrap();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    eprintln!("🚀 SX9 MCP Server starting...");
    eprintln!("📋 Atomic Clipboard: Ready");
    eprintln!("📁 File Index: Ready");
    eprintln!("🧠 Artifacts: Ready");
    
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("❌ Error reading stdin: {}", e);
                continue;
            }
        };
        
        if line.trim().is_empty() {
            continue;
        }
        
        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("❌ Error parsing request: {}", e);
                continue;
            }
        };
        
        let response = rt.block_on(handle_request(request));
        let response_json = serde_json::to_string(&response).unwrap();
        writeln!(stdout, "{}", response_json).unwrap();
        stdout.flush().unwrap();
    }
}

async fn handle_request(req: JsonRpcRequest) -> JsonRpcResponse {
    match req.method.as_str() {
        "initialize" => handle_initialize(req.id),
        "tools/list" => handle_tools_list(req.id),
        "tools/call" => handle_tool_call(req.id, req.params).await,
        "resources/list" => handle_resources_list(req.id),
        "resources/read" => handle_resource_read(req.id, req.params).await,
        _ => error_response(req.id, -32601, "Method not found"),
    }
}

fn handle_initialize(id: Value) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {},
                "resources": {}
            },
            "serverInfo": {
                "name": "sx9-mcp-server",
                "version": "0.1.0"
            }
        })),
        error: None,
    }
}

fn handle_tools_list(id: Value) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(json!({
            "tools": [
                {
                    "name": "clipboard_read",
                    "description": "Read entries from Atomic Clipboard (shared memory across all IDEs and AI agents)",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "tags": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "Filter by tags (optional)"
                            },
                            "limit": {
                                "type": "number",
                                "description": "Max entries to return (default: 10)"
                            }
                        }
                    }
                },
                {
                    "name": "clipboard_write",
                    "description": "Write entry to Atomic Clipboard (accessible by all IDEs and AI agents)",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "content": {
                                "type": "string",
                                "description": "Content to save"
                            },
                            "tags": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "Tags for the entry"
                            },
                            "source": {
                                "type": "string",
                                "description": "Source identifier (default: mcp_client)"
                            }
                        },
                        "required": ["content"]
                    }
                },
                {
                    "name": "file_search",
                    "description": "Search file index across all repositories by tags or path",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "tags": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "Search by tags (e.g., ['rust', 'api'])"
                            },
                            "path_query": {
                                "type": "string",
                                "description": "Search by path text (e.g., 'auth')"
                            },
                            "workspace": {
                                "type": "string",
                                "description": "Workspace to search (default: ~/Developer/sx9)"
                            }
                        }
                    }
                },
                {
                    "name": "read_artifact",
                    "description": "Read artifact from Claude brain directory (plans, walkthroughs, tasks)",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "filename": {
                                "type": "string",
                                "description": "Artifact filename (e.g., 'task.md', 'implementation_plan.md')"
                            }
                        },
                        "required": ["filename"]
                    }
                },
                {
                    "name": "list_artifacts",
                    "description": "List all artifacts in Claude brain directory",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                }
            ]
        })),
        error: None,
    }
}

async fn handle_tool_call(id: Value, params: Option<Value>) -> JsonRpcResponse {
    let params = match params {
        Some(p) => p,
        None => return error_response(id, -32602, "Missing params"),
    };
    
    let tool_name = match params.get("name").and_then(|v| v.as_str()) {
        Some(n) => n,
        None => return error_response(id, -32602, "Missing tool name"),
    };
    
    let args = params.get("arguments").cloned().unwrap_or(json!({}));
    
    match tool_name {
        "clipboard_read" => handle_clipboard_read(id, &args).await,
        "clipboard_write" => handle_clipboard_write(id, &args).await,
        "file_search" => handle_file_search(id, &args).await,
        "read_artifact" => handle_read_artifact(id, &args).await,
        "list_artifacts" => handle_list_artifacts(id).await,
        _ => error_response(id, -32602, "Unknown tool"),
    }
}

async fn handle_clipboard_read(id: Value, args: &Value) -> JsonRpcResponse {
    match AtomicClipboard::new() {
        Ok(clipboard) => {
            let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
            match clipboard.list(limit).await {
                Ok(mut entries) => {
                    if let Some(tags) = args.get("tags").and_then(|v| v.as_array()) {
                        let tag_strings: Vec<String> = tags.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect();
                        
                        if !tag_strings.is_empty() {
                            entries.retain(|e| e.tags.iter().any(|t| tag_strings.contains(t)));
                        }
                    }
                    
                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: Some(json!({"content": entries})),
                        error: None,
                    }
                }
                Err(e) => error_response(id, -32603, &format!("Failed to read clipboard: {}", e)),
            }
        }
        Err(e) => error_response(id, -32603, &format!("Failed to initialize clipboard: {}", e)),
    }
}

async fn handle_clipboard_write(id: Value, args: &Value) -> JsonRpcResponse {
    let content = match args.get("content").and_then(|v| v.as_str()) {
        Some(c) => c,
        None => return error_response(id, -32602, "Missing content"),
    };
    
    let tags: Vec<String> = args.get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();
    
    let source = args.get("source").and_then(|v| v.as_str()).unwrap_or("mcp_client").to_string();
    
    match AtomicClipboard::new() {
        Ok(clipboard) => {
            let entry = ClipboardEntry {
                id: uuid::Uuid::new_v4().to_string(),
                content: content.to_string(),
                source,
                tags,
                created_at: chrono::Utc::now(),
                metadata: json!({}),
            };
            
            match clipboard.push(entry.clone()).await {
                Ok(_) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(json!({"success": true, "id": entry.id})),
                    error: None,
                },
                Err(e) => error_response(id, -32603, &format!("Failed to write clipboard: {}", e)),
            }
        }
        Err(e) => error_response(id, -32603, &format!("Failed to initialize clipboard: {}", e)),
    }
}

async fn handle_file_search(id: Value, args: &Value) -> JsonRpcResponse {
    let workspace = args.get("workspace").and_then(|v| v.as_str()).unwrap_or("~/Developer/sx9");
    let workspace_path = PathBuf::from(shellexpand::tilde(workspace).to_string());
    
    match FileIndex::new(workspace_path) {
        Ok(index) => {
            if let Err(e) = index.index_workspace() {
                return error_response(id, -32603, &format!("Failed to index workspace: {}", e));
            }
            
            let results = if let Some(tags) = args.get("tags").and_then(|v| v.as_array()) {
                let tag_strings: Vec<String> = tags.iter().filter_map(|v| v.as_str().map(String::from)).collect();
                index.search_by_tags(&tag_strings)
            } else if let Some(query) = args.get("path_query").and_then(|v| v.as_str()) {
                index.search_by_path(query)
            } else {
                index.recent(20)
            };
            
            match results {
                Ok(files) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(json!({"files": files})),
                    error: None,
                },
                Err(e) => error_response(id, -32603, &format!("Search failed: {}", e)),
            }
        }
        Err(e) => error_response(id, -32603, &format!("Failed to create index: {}", e)),
    }
}

async fn handle_read_artifact(id: Value, args: &Value) -> JsonRpcResponse {
    let filename = match args.get("filename").and_then(|v| v.as_str()) {
        Some(f) => f,
        None => return error_response(id, -32602, "Missing filename"),
    };
    
    let brain_dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".gemini/antigravity/brain");
    
    let mut conversations: Vec<_> = std::fs::read_dir(&brain_dir)
        .ok().into_iter().flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().ok().map(|t| t.is_dir()).unwrap_or(false))
        .collect();
    
    conversations.sort_by_key(|e| std::cmp::Reverse(e.metadata().ok().and_then(|m| m.modified().ok())));
    
    if let Some(latest_conv) = conversations.first() {
        let artifact_path = latest_conv.path().join(filename);
        match std::fs::read_to_string(&artifact_path) {
            Ok(content) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(json!({"content": content, "path": artifact_path.display().to_string()})),
                error: None,
            },
            Err(e) => error_response(id, -32603, &format!("Failed to read artifact: {}", e)),
        }
    } else {
        error_response(id, -32603, "No conversation directories found")
    }
}

async fn handle_list_artifacts(id: Value) -> JsonRpcResponse {
    let brain_dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".gemini/antigravity/brain");
    
    let mut conversations: Vec<_> = std::fs::read_dir(&brain_dir)
        .ok().into_iter().flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().ok().map(|t| t.is_dir()).unwrap_or(false))
        .collect();
    
    conversations.sort_by_key(|e| std::cmp::Reverse(e.metadata().ok().and_then(|m| m.modified().ok())));
    
    if let Some(latest_conv) = conversations.first() {
        let artifacts: Vec<String> = std::fs::read_dir(latest_conv.path())
            .ok().into_iter().flatten()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().ok().map(|t| t.is_file()).unwrap_or(false))
            .filter_map(|e| e.file_name().to_str().map(String::from))
            .filter(|name| name.ends_with(".md") || name.ends_with(".txt"))
            .collect();
        
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({"artifacts": artifacts, "conversation_path": latest_conv.path().display().to_string()})),
            error: None,
        }
    } else {
        error_response(id, -32603, "No conversation directories found")
    }
}

fn handle_resources_list(id: Value) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(json!({
            "resources": [
                {"uri": "clipboard://recent", "name": "Recent Clipboard Entries", "description": "Last 10 clipboard entries from Atomic Clipboard", "mimeType": "application/json"},
                {"uri": "artifacts://list", "name": "Claude Artifacts", "description": "All artifacts from current conversation", "mimeType": "application/json"}
            ]
        })),
        error: None,
    }
}

async fn handle_resource_read(id: Value, params: Option<Value>) -> JsonRpcResponse {
    let uri = params.as_ref()
        .and_then(|p| p.get("uri"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    
    match uri {
        "clipboard://recent" => handle_clipboard_read(id, &json!({"limit": 10})).await,
        "artifacts://list" => handle_list_artifacts(id).await,
        _ => error_response(id, -32602, "Unknown resource URI"),
    }
}

fn error_response(id: Value, code: i32, message: &str) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: None,
        error: Some(JsonRpcError {
            code,
            message: message.to_string(),
        }),
    }
}
