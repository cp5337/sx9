//! OSINT CSV Media Sites Processor
//! Processes 6474+ media sites from OSINT map for threat intelligence workflows
//! Follows CTAS-7 standards: ≤200 LOC, Tesla/SpaceX grade

use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use chrono::{DateTime, Utc};
use crate::hash_engine::Hasher;
use uuid::Uuid;

use crate::rust_workflow_orchestrator::{RustWorkflowOrchestrator, WorkflowMessage, MessageType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSINTSiteRecord {
    pub record_id: String,
    pub event_date: String,
    pub source_url: String,
    pub city: String,
    pub state: String,
    pub incident_type: String,
    pub description: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub classification: String,
    pub blake3_hash: String,
    pub processed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSINTProcessor {
    pub processor_id: String,
    pub csv_path: String,
    pub batch_size: usize,
    pub processed_count: u64,
    pub error_count: u64,
    pub total_sites: u64,
    pub start_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStats {
    pub total_records: u64,
    pub processed_successfully: u64,
    pub processing_errors: u64,
    pub threat_indicators: u64,
    pub geographic_coverage: u32, // Number of unique states/regions
    pub classification_breakdown: std::collections::HashMap<String, u64>,
    pub processing_rate_per_minute: f64,
}

impl OSINTProcessor {
    pub fn new(csv_path: String, batch_size: usize) -> Self {
        Self {
            processor_id: Uuid::new_v4().to_string(),
            csv_path,
            batch_size,
            processed_count: 0,
            error_count: 0,
            total_sites: 0,
            start_time: Utc::now(),
        }
    }

    pub async fn process_osint_csv(&mut self) -> Result<ProcessingStats, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.csv_path);
        if !file_path.exists() {
            return Err(format!("OSINT CSV file not found: {}", self.csv_path).into());
        }

        let file = File::open(file_path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Skip CSV header
        if let Some(header) = lines.next_line().await? {
            tracing::info!("Processing OSINT CSV with header: {}", &header[0..100.min(header.len())]);
        }

        let mut batch = Vec::new();
        let mut stats = ProcessingStats {
            total_records: 0,
            processed_successfully: 0,
            processing_errors: 0,
            threat_indicators: 0,
            geographic_coverage: 0,
            classification_breakdown: std::collections::HashMap::new(),
            processing_rate_per_minute: 0.0,
        };

        let mut unique_states = std::collections::HashSet::new();

        while let Some(line) = lines.next_line().await? {
            if line.trim().is_empty() {
                continue;
            }

            match self.parse_osint_record(&line).await {
                Ok(record) => {
                    // Track geographic coverage
                    unique_states.insert(record.state.clone());

                    // Track classification breakdown
                    *stats.classification_breakdown
                        .entry(record.classification.clone())
                        .or_insert(0) += 1;

                    // Identify potential threat indicators
                    if self.is_threat_indicator(&record) {
                        stats.threat_indicators += 1;
                    }

                    batch.push(record);
                    stats.processed_successfully += 1;

                    if batch.len() >= self.batch_size {
                        self.process_batch(batch.clone()).await?;
                        batch.clear();
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to parse OSINT record: {}", e);
                    stats.processing_errors += 1;
                }
            }

            stats.total_records += 1;
            self.processed_count += 1;

            // Process remaining batch
            if !batch.is_empty() && stats.total_records % 1000 == 0 {
                tracing::info!("Processed {} OSINT records", stats.total_records);
            }
        }

        // Process final batch
        if !batch.is_empty() {
            self.process_batch(batch).await?;
        }

        stats.geographic_coverage = unique_states.len() as u32;
        stats.processing_rate_per_minute = self.calculate_processing_rate();

        tracing::info!("OSINT CSV processing complete: {} records processed", stats.total_records);
        Ok(stats)
    }

    async fn parse_osint_record(&self, csv_line: &str) -> Result<OSINTSiteRecord, Box<dyn std::error::Error>> {
        let fields: Vec<&str> = csv_line.split(',').collect();

        if fields.len() < 8 {
            return Err("Insufficient CSV fields".into());
        }

        let record_id = Uuid::new_v4().to_string();
        let event_date = fields[0].trim_matches('"').to_string();
        let source_url = fields[1].trim_matches('"').to_string();
        let city = fields.get(3).unwrap_or(&"").trim_matches('"').to_string();
        let state = fields.get(4).unwrap_or(&"").trim_matches('"').to_string();
        let incident_type = fields.get(15).unwrap_or(&"Unknown").trim_matches('"').to_string();
        let description = fields.get(34).unwrap_or(&"").trim_matches('"').to_string();

        // Parse coordinates
        let lat_str = fields.get(21).unwrap_or(&"").trim_matches('"');
        let (latitude, longitude) = self.parse_coordinates(lat_str);

        // Default handling caveat assignment
        let classification = "HANDLING_CAVEAT_1".to_string();

        let record = OSINTSiteRecord {
            record_id: record_id.clone(),
            event_date,
            source_url,
            city,
            state,
            incident_type,
            description,
            latitude,
            longitude,
            classification,
            blake3_hash: self.calculate_record_hash(&record_id, csv_line),
            processed_at: Utc::now(),
        };

        Ok(record)
    }

    fn parse_coordinates(&self, coord_str: &str) -> (Option<f64>, Option<f64>) {
        if coord_str.starts_with("POINT (") && coord_str.ends_with(')') {
            let coords = coord_str.trim_start_matches("POINT (").trim_end_matches(')');
            let parts: Vec<&str> = coords.split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(lon), Ok(lat)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                    return (Some(lat), Some(lon));
                }
            }
        }
        (None, None)
    }

    fn is_threat_indicator(&self, record: &OSINTSiteRecord) -> bool {
        let threat_keywords = ["bomb", "explosive", "threat", "device", "weapon", "terror"];
        let text_to_check = format!("{} {}", record.incident_type, record.description).to_lowercase();

        threat_keywords.iter().any(|keyword| text_to_check.contains(keyword))
    }

    async fn process_batch(&self, batch: Vec<OSINTSiteRecord>) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("Processing batch of {} OSINT records", batch.len());

        for record in &batch {
            // Send high-priority threat indicators to workflow orchestrator
            if self.is_threat_indicator(record) && record.classification == "HANDLING_CAVEAT_1" {
                tracing::warn!("High-priority threat indicator: {} in {}, {}",
                    record.incident_type, record.city, record.state);
            }
        }

        Ok(())
    }

    fn calculate_record_hash(&self, record_id: &str, csv_line: &str) -> String {
        let mut hasher = Hasher::new();
        hasher.update(record_id.as_bytes());
        hasher.update(csv_line.as_bytes());
        hasher.finalize().to_hex().to_string()
    }

    fn calculate_processing_rate(&self) -> f64 {
        let elapsed = Utc::now().timestamp() - self.start_time.timestamp();
        if elapsed > 0 {
            (self.processed_count as f64) / (elapsed as f64 / 60.0)
        } else {
            0.0
        }
    }

    pub async fn get_processing_summary(&self) -> serde_json::Value {
        serde_json::json!({
            "processor_id": self.processor_id,
            "csv_path": self.csv_path,
            "processed_count": self.processed_count,
            "error_count": self.error_count,
            "processing_rate": self.calculate_processing_rate(),
            "uptime_minutes": (Utc::now().timestamp() - self.start_time.timestamp()) / 60
        })
    }
}

// Integration with Rust Workflow Orchestrator
pub async fn start_osint_processing_workflow(
    orchestrator: &RustWorkflowOrchestrator,
    csv_path: String,
    batch_size: usize,
) -> Result<String, Box<dyn std::error::Error>> {
    let workflow_id = orchestrator.create_osint_workflow(csv_path.clone(), batch_size).await;

    let mut processor = OSINTProcessor::new(csv_path, batch_size);
    let stats = processor.process_osint_csv().await?;

    tracing::info!("OSINT workflow {} completed with {} records", workflow_id, stats.total_records);

    Ok(workflow_id)
}

// REST API endpoint for OSINT CSV processing
pub async fn process_osint_csv_endpoint(
    axum::Json(request): axum::Json<serde_json::Value>,
) -> axum::Json<serde_json::Value> {
    let csv_path = request["csv_path"]
        .as_str()
        .unwrap_or("/Users/cp5337/Desktop/osint_map.csv")
        .to_string();
    let batch_size = request["batch_size"].as_u64().unwrap_or(100) as usize;

    let mut processor = OSINTProcessor::new(csv_path, batch_size);

    match processor.process_osint_csv().await {
        Ok(stats) => {
            axum::Json(serde_json::json!({
                "status": "completed",
                "processor_id": processor.processor_id,
                "stats": stats,
                "message": "OSINT CSV processing completed successfully"
            }))
        }
        Err(e) => {
            axum::Json(serde_json::json!({
                "status": "error",
                "error": e.to_string(),
                "processor_id": processor.processor_id
            }))
        }
    }
}