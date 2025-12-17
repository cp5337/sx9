//! CTAS-7 Resource Monitor
//! 
//! System resource monitoring and performance tracking.

use std::collections::HashMap;
use tracing::{info, warn, error};
use chrono::Utc;
use sysinfo::{System, Cpu, Disk, NetworkData};

/// Resource Monitor
#[derive(Debug)]
pub struct ResourceMonitor {
    pub system_info: System,
    pub resource_history: HashMap<String, Vec<ResourceMetric>>,
    pub alert_thresholds: HashMap<String, f64>,
}

/// Resource Metric
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResourceMetric {
    pub metric_name: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub threshold: Option<f64>,
    pub status: ResourceStatus,
}

/// Resource Status
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ResourceStatus {
    Normal,
    Warning,
    Critical,
}

impl ResourceMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        let mut monitor = Self {
            system_info: system,
            resource_history: HashMap::new(),
            alert_thresholds: HashMap::new(),
        };
        
        // Set default thresholds
        monitor.alert_thresholds.insert("cpu_usage".to_string(), 80.0);
        monitor.alert_thresholds.insert("memory_usage".to_string(), 85.0);
        monitor.alert_thresholds.insert("disk_usage".to_string(), 90.0);
        
        monitor
    }
    
    pub async fn collect_metrics(&mut self) -> Vec<ResourceMetric> {
        self.system_info.refresh_all();
        
        let mut metrics = Vec::new();
        
        // CPU Usage
        let cpu_usage = self.system_info.global_cpu_info().cpu_usage() as f64;
        let cpu_metric = ResourceMetric {
            metric_name: "cpu_usage".to_string(),
            value: cpu_usage,
            unit: "percent".to_string(),
            timestamp: Utc::now(),
            threshold: self.alert_thresholds.get("cpu_usage").copied(),
            status: self.get_resource_status("cpu_usage", cpu_usage),
        };
        metrics.push(cpu_metric);
        
        // Memory Usage
        let total_memory = self.system_info.total_memory();
        let used_memory = self.system_info.used_memory();
        let memory_usage = (used_memory as f64 / total_memory as f64) * 100.0;
        let memory_metric = ResourceMetric {
            metric_name: "memory_usage".to_string(),
            value: memory_usage,
            unit: "percent".to_string(),
            timestamp: Utc::now(),
            threshold: self.alert_thresholds.get("memory_usage").copied(),
            status: self.get_resource_status("memory_usage", memory_usage),
        };
        metrics.push(memory_metric);
        
        // Disk Usage - Basic implementation
        // Note: sysinfo API varies by version, using basic metrics for now
        
        // Network I/O - Basic implementation
        // Note: sysinfo API varies by version, using basic metrics for now
        
        // Store metrics in history
        for metric in &metrics {
            self.resource_history
                .entry(metric.metric_name.clone())
                .or_insert_with(Vec::new)
                .push(metric.clone());
        }
        
        info!("📊 Collected {} resource metrics", metrics.len());
        metrics
    }
    
    fn get_resource_status(&self, metric_name: &str, value: f64) -> ResourceStatus {
        if let Some(threshold) = self.alert_thresholds.get(metric_name) {
            if value >= *threshold {
                ResourceStatus::Critical
            } else if value >= *threshold * 0.8 {
                ResourceStatus::Warning
            } else {
                ResourceStatus::Normal
            }
        } else {
            ResourceStatus::Normal
        }
    }
    
    pub fn get_resource_summary(&self) -> HashMap<String, f64> {
        let mut summary = HashMap::new();
        
        // CPU
        summary.insert("cpu_usage".to_string(), self.system_info.global_cpu_info().cpu_usage() as f64);
        
        // Memory
        let total_memory = self.system_info.total_memory();
        let used_memory = self.system_info.used_memory();
        let memory_usage = (used_memory as f64 / total_memory as f64) * 100.0;
        summary.insert("memory_usage".to_string(), memory_usage);
        
        // Disk - Basic implementation
        // Note: sysinfo API varies by version, using basic metrics for now
        
        summary
    }
}
