// ═══════════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS - CODE QUALITY (LIGHTNING QA)
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LightningQaResult {
    pub success: bool,
    pub output: String,
    pub dashboard_path: String,
}

#[tauri::command]
pub async fn run_lightning_qa(target_path: String) -> Result<LightningQaResult, String> {
    use std::process::Command;
    
    // Locate the QA script relative to the repo root
    // Assuming invoke from dev-forge which is in repo root
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let script_path = root.join("../tools/lightning-qa/sx9-lightning-qa.py");
    
    // Execute Python script
    // Use system python3
    let output = Command::new("python3")
        .arg(&script_path)
        .arg(&target_path)
        .output()
        .map_err(|e| format!("Failed to execute QA script: {}", e))?;
        
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    if !output.status.success() {
        return Err(format!("QA execution failed: {}\n{}", stdout, stderr));
    }
    
    Ok(LightningQaResult {
        success: true,
        output: stdout,
        dashboard_path: format!("{}/sx9-qa-results/dashboard.md", target_path),
    })
}
