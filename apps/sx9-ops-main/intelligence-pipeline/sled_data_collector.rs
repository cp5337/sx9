// Sled-based data collector for CTAS tool intelligence
// Fast, embedded database for tool execution data

use sled::{Db, Tree};
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use blake3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecutionRecord {
    pub execution_id: String,
    pub tool_name: String,
    pub command: String,
    pub parameters: Vec<String>,
    pub timestamp: u64,
    pub duration_ms: u64,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub output_files: Vec<String>,
    pub success: bool,
    pub primitive_type: String,
    pub resource_usage: ResourceUsage,
    pub network_activity: Vec<NetworkConnection>,
    pub file_operations: Vec<FileOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_percent: f64,
    pub memory_mb: u64,
    pub disk_io_bytes: u64,
    pub network_io_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    pub protocol: String,
    pub local_addr: String,
    pub remote_addr: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperation {
    pub operation: String, // "read", "write", "create", "delete"
    pub file_path: String,
    pub size_bytes: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolProfile {
    pub tool_name: String,
    pub category: String,
    pub primitive_types: Vec<String>,
    pub typical_usage: Vec<String>,
    pub success_indicators: Vec<String>,
    pub common_outputs: Vec<String>,
    pub detection_signatures: Vec<String>,
    pub last_updated: u64,
}

pub struct SledToolCollector {
    db: Db,
    executions: Tree,
    profiles: Tree,
    analytics: Tree,
    indices: Tree,
}

impl SledToolCollector {
    pub fn new(db_path: &str) -> Result<Self> {
        let db = sled::open(db_path)
            .context("Failed to open Sled database")?;
        
        let executions = db.open_tree("tool_executions")
            .context("Failed to open executions tree")?;
        
        let profiles = db.open_tree("tool_profiles")
            .context("Failed to open profiles tree")?;
        
        let analytics = db.open_tree("analytics")
            .context("Failed to open analytics tree")?;
        
        let indices = db.open_tree("indices")
            .context("Failed to open indices tree")?;
        
        Ok(Self {
            db,
            executions,
            profiles,
            analytics,
            indices,
        })
    }
    
    pub fn store_execution(&self, record: &ToolExecutionRecord) -> Result<()> {
        // Serialize record
        let serialized = bincode::serialize(record)
            .context("Failed to serialize execution record")?;
        
        // Store with execution_id as key
        self.executions.insert(record.execution_id.as_bytes(), serialized)
            .context("Failed to store execution record")?;
        
        // Update indices for fast lookups
        self.update_indices(record)?;
        
        // Update tool profile
        self.update_tool_profile(record)?;
        
        self.db.flush()?;
        Ok(())
    }
    
    fn update_indices(&self, record: &ToolExecutionRecord) -> Result<()> {
        let timestamp_key = format!("timestamp:{}", record.timestamp);
        self.indices.insert(timestamp_key.as_bytes(), record.execution_id.as_bytes())?;
        
        let tool_key = format!("tool:{}:{}", record.tool_name, record.timestamp);
        self.indices.insert(tool_key.as_bytes(), record.execution_id.as_bytes())?;
        
        let primitive_key = format!("primitive:{}:{}", record.primitive_type, record.timestamp);
        self.indices.insert(primitive_key.as_bytes(), record.execution_id.as_bytes())?;
        
        let success_key = format!("success:{}:{}", record.success, record.timestamp);
        self.indices.insert(success_key.as_bytes(), record.execution_id.as_bytes())?;
        
        Ok(())
    }
    
    fn update_tool_profile(&self, record: &ToolExecutionRecord) -> Result<()> {
        let profile_key = format!("profile:{}", record.tool_name);
        
        // Get existing profile or create new one
        let mut profile = if let Some(existing) = self.profiles.get(profile_key.as_bytes())? {
            bincode::deserialize::<ToolProfile>(&existing)
                .context("Failed to deserialize existing profile")?
        } else {
            ToolProfile {
                tool_name: record.tool_name.clone(),
                category: "unknown".to_string(),
                primitive_types: vec![record.primitive_type.clone()],
                typical_usage: vec![record.command.clone()],
                success_indicators: vec![],
                common_outputs: record.output_files.clone(),
                detection_signatures: vec![],
                last_updated: record.timestamp,
            }
        };
        
        // Update profile with new execution data
        if !profile.primitive_types.contains(&record.primitive_type) {
            profile.primitive_types.push(record.primitive_type.clone());
        }
        
        if !profile.typical_usage.contains(&record.command) {
            profile.typical_usage.push(record.command.clone());
        }
        
        // Add new output files
        for output_file in &record.output_files {
            if !profile.common_outputs.contains(output_file) {
                profile.common_outputs.push(output_file.clone());
            }
        }
        
        profile.last_updated = record.timestamp;
        
        // Store updated profile
        let serialized = bincode::serialize(&profile)
            .context("Failed to serialize profile")?;
        self.profiles.insert(profile_key.as_bytes(), serialized)?;
        
        Ok(())
    }
    
    pub fn get_execution(&self, execution_id: &str) -> Result<Option<ToolExecutionRecord>> {
        if let Some(data) = self.executions.get(execution_id.as_bytes())? {
            let record = bincode::deserialize::<ToolExecutionRecord>(&data)
                .context("Failed to deserialize execution record")?;
            Ok(Some(record))
        } else {
            Ok(None)
        }
    }
    
    pub fn get_tool_profile(&self, tool_name: &str) -> Result<Option<ToolProfile>> {
        let profile_key = format!("profile:{}", tool_name);
        
        if let Some(data) = self.profiles.get(profile_key.as_bytes())? {
            let profile = bincode::deserialize::<ToolProfile>(&data)
                .context("Failed to deserialize tool profile")?;
            Ok(Some(profile))
        } else {
            Ok(None)
        }
    }
    
    pub fn get_executions_by_tool(&self, tool_name: &str, limit: usize) -> Result<Vec<ToolExecutionRecord>> {
        let prefix = format!("tool:{}", tool_name);
        let mut executions = Vec::new();
        
        for result in self.indices.scan_prefix(prefix.as_bytes()).take(limit) {
            let (_, execution_id_bytes) = result?;
            let execution_id = String::from_utf8_lossy(&execution_id_bytes);
            
            if let Some(record) = self.get_execution(&execution_id)? {
                executions.push(record);
            }
        }
        
        // Sort by timestamp (newest first)
        executions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(executions)
    }
    
    pub fn get_executions_by_primitive(&self, primitive_type: &str, limit: usize) -> Result<Vec<ToolExecutionRecord>> {
        let prefix = format!("primitive:{}", primitive_type);
        let mut executions = Vec::new();
        
        for result in self.indices.scan_prefix(prefix.as_bytes()).take(limit) {
            let (_, execution_id_bytes) = result?;
            let execution_id = String::from_utf8_lossy(&execution_id_bytes);
            
            if let Some(record) = self.get_execution(&execution_id)? {
                executions.push(record);
            }
        }
        
        executions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(executions)
    }
    
    pub fn get_recent_executions(&self, limit: usize) -> Result<Vec<ToolExecutionRecord>> {
        let mut executions = Vec::new();
        
        // Scan timestamp index in reverse order (newest first)
        for result in self.indices.scan_prefix("timestamp:".as_bytes()).rev().take(limit) {
            let (_, execution_id_bytes) = result?;
            let execution_id = String::from_utf8_lossy(&execution_id_bytes);
            
            if let Some(record) = self.get_execution(&execution_id)? {
                executions.push(record);
            }
        }
        
        Ok(executions)
    }
    
    pub fn generate_analytics(&self) -> Result<ToolAnalytics> {
        let mut tool_counts = std::collections::HashMap::new();
        let mut primitive_counts = std::collections::HashMap::new();
        let mut success_rate_by_tool = std::collections::HashMap::new();
        let mut total_executions = 0;
        let mut successful_executions = 0;
        
        // Scan all executions for analytics
        for result in self.executions.iter() {
            let (_, data) = result?;
            let record: ToolExecutionRecord = bincode::deserialize(&data)
                .context("Failed to deserialize execution record")?;
            
            total_executions += 1;
            if record.success {
                successful_executions += 1;
            }
            
            // Tool counts
            *tool_counts.entry(record.tool_name.clone()).or_insert(0) += 1;
            
            // Primitive counts
            *primitive_counts.entry(record.primitive_type.clone()).or_insert(0) += 1;
            
            // Success rates by tool
            let tool_stats = success_rate_by_tool.entry(record.tool_name.clone())
                .or_insert((0, 0)); // (successful, total)
            tool_stats.1 += 1;
            if record.success {
                tool_stats.0 += 1;
            }
        }
        
        // Calculate success rates
        let mut tool_success_rates = std::collections::HashMap::new();
        for (tool, (successful, total)) in success_rate_by_tool {
            let rate = if total > 0 { successful as f64 / total as f64 } else { 0.0 };
            tool_success_rates.insert(tool, rate);
        }
        
        let analytics = ToolAnalytics {
            total_executions,
            successful_executions,
            overall_success_rate: if total_executions > 0 { 
                successful_executions as f64 / total_executions as f64 
            } else { 0.0 },
            tool_counts,
            primitive_counts,
            tool_success_rates,
            generated_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        };
        
        // Store analytics
        let serialized = bincode::serialize(&analytics)?;
        self.analytics.insert("latest".as_bytes(), serialized)?;
        self.db.flush()?;
        
        Ok(analytics)
    }
    
    pub fn get_tool_list(&self) -> Result<Vec<String>> {
        let mut tools = std::collections::HashSet::new();
        
        for result in self.profiles.iter() {
            let (key, _) = result?;
            let key_str = String::from_utf8_lossy(&key);
            if let Some(tool_name) = key_str.strip_prefix("profile:") {
                tools.insert(tool_name.to_string());
            }
        }
        
        Ok(tools.into_iter().collect())
    }
    
    pub fn export_to_json(&self, output_path: &str) -> Result<()> {
        let analytics = self.generate_analytics()?;
        let recent_executions = self.get_recent_executions(100)?;
        let tool_list = self.get_tool_list()?;
        
        let export_data = serde_json::json!({
            "analytics": analytics,
            "recent_executions": recent_executions,
            "tool_list": tool_list,
            "export_timestamp": SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
        });
        
        std::fs::write(output_path, serde_json::to_string_pretty(&export_data)?)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolAnalytics {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub overall_success_rate: f64,
    pub tool_counts: std::collections::HashMap<String, u64>,
    pub primitive_counts: std::collections::HashMap<String, u64>,
    pub tool_success_rates: std::collections::HashMap<String, f64>,
    pub generated_at: u64,
}

// Helper function to generate Blake3 hash for execution ID
pub fn generate_execution_id(tool_name: &str, timestamp: u64) -> String {
    let input = format!("{}:{}", tool_name, timestamp);
    let hash = blake3::hash(input.as_bytes());
    format!("exec_{}", hex::encode(&hash.as_bytes()[..8]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_store_and_retrieve_execution() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let collector = SledToolCollector::new(db_path.to_str().unwrap()).unwrap();
        
        let record = ToolExecutionRecord {
            execution_id: "test_exec_1".to_string(),
            tool_name: "nmap".to_string(),
            command: "nmap -sV 127.0.0.1".to_string(),
            parameters: vec!["-sV".to_string(), "127.0.0.1".to_string()],
            timestamp: 1640995200,
            duration_ms: 5000,
            exit_code: 0,
            stdout: "Nmap scan report...".to_string(),
            stderr: "".to_string(),
            output_files: vec!["/tmp/scan_results.xml".to_string()],
            success: true,
            primitive_type: "SENSE".to_string(),
            resource_usage: ResourceUsage {
                cpu_percent: 15.5,
                memory_mb: 128,
                disk_io_bytes: 1024,
                network_io_bytes: 2048,
            },
            network_activity: vec![],
            file_operations: vec![],
        };
        
        collector.store_execution(&record).unwrap();
        
        let retrieved = collector.get_execution("test_exec_1").unwrap().unwrap();
        assert_eq!(retrieved.tool_name, "nmap");
        assert_eq!(retrieved.success, true);
    }
    
    #[test]
    fn test_analytics_generation() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let collector = SledToolCollector::new(db_path.to_str().unwrap()).unwrap();
        
        // Add sample executions
        for i in 0..5 {
            let record = ToolExecutionRecord {
                execution_id: format!("test_exec_{}", i),
                tool_name: "nmap".to_string(),
                command: "nmap -sV 127.0.0.1".to_string(),
                parameters: vec![],
                timestamp: 1640995200 + i,
                duration_ms: 1000,
                exit_code: if i % 2 == 0 { 0 } else { 1 },
                stdout: "".to_string(),
                stderr: "".to_string(),
                output_files: vec![],
                success: i % 2 == 0,
                primitive_type: "SENSE".to_string(),
                resource_usage: ResourceUsage {
                    cpu_percent: 10.0,
                    memory_mb: 64,
                    disk_io_bytes: 512,
                    network_io_bytes: 1024,
                },
                network_activity: vec![],
                file_operations: vec![],
            };
            collector.store_execution(&record).unwrap();
        }
        
        let analytics = collector.generate_analytics().unwrap();
        assert_eq!(analytics.total_executions, 5);
        assert_eq!(analytics.successful_executions, 3);
        assert_eq!(analytics.tool_counts.get("nmap").unwrap(), &5);
    }
}
