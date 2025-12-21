use sx9_dev_forge_lib::{atomic_clipboard, file_index, vault::{self, KeyVaultExt}};
use std::env;

use tokio::runtime::Runtime;

#[test]
fn test_feature_suite() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        println!("🚀 Starting Dev Forge Feature Verification Suite");
        
        // 1. Vault Identity Check
        println!("\n🔐 Testing Vault Identity...");
        match vault::global_vault() {
            Ok(vault) => {
                let stats = vault.stats().await;
                println!("   ✅ Vault accessible ({} keys)", stats.total);
                
                // Check specific keys
                if let Some(_) = vault.get("LINEAR_API_KEY").await {
                    println!("   ✅ LINEAR_API_KEY present");
                } else {
                    println!("   ⚠️ LINEAR_API_KEY missing (skipping ops test)");
                }
            },
            Err(e) => println!("   ❌ Vault Error: {}", e),
        }

        // 2. Atomic Clipboard Check
        println!("\n📋 Testing Atomic Clipboard...");
        let temp_dir = env::temp_dir().join("sx9-test-clipboard");
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir).ok();
        }
        
        match atomic_clipboard::AtomicClipboard::with_path(&temp_dir) {
            Ok(clipboard) => {
                let entry = atomic_clipboard::ClipboardEntry {
                    id: uuid::Uuid::new_v4().to_string(),
                    content: "Feature Verification Test".to_string(),
                    source: "test-suite".to_string(),
                    tags: vec!["test".to_string()],
                    created_at: chrono::Utc::now(),
                    metadata: serde_json::json!({"test": true}),
                };
                
                match clipboard.push(entry).await {
                    Ok(path) => println!("   ✅ Clipboard save successful: {:?}", path),
                    Err(e) => println!("   ❌ Clipboard save failed: {}", e),
                }
            },
            Err(e) => println!("   ❌ Clipboard init failed: {}", e),
        }

        // 3. File Index Check
        println!("\n📂 Testing File Index...");
        let cwd = env::current_dir().unwrap();
        match file_index::FileIndex::new(cwd.clone()) {
            Ok(index) => {
                match index.index_workspace() {
                    Ok(count) => println!("   ✅ Indexed {} files in workspace", count),
                    Err(e) => println!("   ❌ Indexing failed: {}", e),
                }
            },
            Err(e) => println!("   ❌ Index init failed: {}", e),
        }

        // 4. Mission Creation & Agentic Flow Verification
        println!("\n🤖 Testing Mission Creation (Agentic Flow)...");
        // We need to setup AppState manually for this test
        let clipboard_path = env::temp_dir().join("sx9-test-mission-clipboard");
        if clipboard_path.exists() { std::fs::remove_dir_all(&clipboard_path).ok(); }
        
        let mission_store_path = env::temp_dir().join("sx9-test-mission-store.json");
        if mission_store_path.exists() { std::fs::remove_file(&mission_store_path).ok(); }

        let atomic_clipboard = atomic_clipboard::AtomicClipboard::with_path(&clipboard_path).unwrap();
        let mission_store = sx9_dev_forge_lib::missions::MissionStore::new(mission_store_path.clone());
        
        let state = sx9_dev_forge_lib::AppState {
            mission_store: tokio::sync::Mutex::new(mission_store),
            linear_api_key: tokio::sync::Mutex::new(None),
            rfc_base_path: tokio::sync::Mutex::new(Some(cwd.clone())),
            atomic_clipboard: tokio::sync::Mutex::new(atomic_clipboard),
            elevenlabs_api_key: tokio::sync::Mutex::new(None),
        };
        


        let input = sx9_dev_forge_lib::CreateMissionInput {
            title: "Test Agentic Mission".to_string(),
            prompt_type: "security_audit".to_string(),
            persona: "AXIOM".to_string(),
            harness: "security".to_string(),
            objective: "Verify system integrity".to_string(),
            hard_constraints: vec!["No relentless loops".to_string()],
            soft_constraints: vec![].into(),
            deliverables: vec![].into(),
            rfcs: vec![].into(),
            priority: "P0".to_string(),
            phase: "execution".to_string(),
        };

        match sx9_dev_forge_lib::process_mission_creation(input, &state).await {
            Ok(mission) => {
                println!("   ✅ Mission created: {} (ID: {})", mission.title, mission.id);
                
                // Verify it hit the clipboard
                let clipboard = state.atomic_clipboard.lock().await;
                match clipboard.list(10).await {
                    Ok(entries) => {
                        if entries.iter().any(|e| e.content.contains("Mission Created")) {
                            println!("   ✅ Found mission in Atomic Clipboard");
                        } else {
                            println!("   ❌ Mission NOT found in Atomic Clipboard");
                        }
                    },
                    Err(e) => println!("   ❌ Clipboard list failed: {}", e),
                }
            },
            Err(e) => println!("   ❌ Mission creation failed: {}", e),
        }
        
        println!("\n✨ Feature Verification Complete");
    });
}
