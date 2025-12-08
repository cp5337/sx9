/*!
Phi-3 AI Integration with Legion ECS
Professional analytics with Microsoft Phi-3 small language model
*/

use candle_core::{Device, Result as CandleResult, Tensor};
use candle_transformers::models::phi3::{Config, Model};
use legion::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn, error};
use uuid::Uuid;

/// Phi-3 AI Component for Legion entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phi3Component {
    pub entity_id: Uuid,
    pub model_path: String,
    pub context_window: usize,
    pub temperature: f32,
    pub active: bool,
}

/// AI Analysis Results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAnalysisResult {
    pub entity_id: Uuid,
    pub input_text: String,
    pub generated_response: String,
    pub confidence_score: f32,
    pub processing_time_ms: f64,
    pub token_count: usize,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Phi-3 Model Manager for Legion ECS
pub struct Phi3Manager {
    device: Device,
    model: Option<Model>,
    config: Config,
    analysis_cache: HashMap<String, AIAnalysisResult>,
}

impl Phi3Manager {
    pub fn new() -> CandleResult<Self> {
        info!("🧠 Initializing Phi-3 AI integration with Legion ECS");
        
        let device = Device::Cpu; // Use CPU for containerized deployment
        let config = Config::default();
        
        Ok(Self {
            device,
            model: None,
            config,
            analysis_cache: HashMap::new(),
        })
    }
    
    pub async fn load_model(&mut self, model_path: &str) -> CandleResult<()> {
        info!("📥 Loading Phi-3 model from: {}", model_path);
        
        // In production, this would load the actual Phi-3 model
        // For playground, we'll simulate the model loading
        info!("✅ Phi-3 model loaded successfully (simulated)");
        Ok(())
    }
    
    pub async fn analyze_entity_data(
        &mut self, 
        entity_id: Uuid,
        input_text: &str
    ) -> CandleResult<AIAnalysisResult> {
        let start_time = std::time::Instant::now();
        
        // Check cache first
        if let Some(cached_result) = self.analysis_cache.get(input_text) {
            info!("📋 Using cached AI analysis for entity {}", entity_id);
            return Ok(cached_result.clone());
        }
        
        // Simulate Phi-3 inference
        let generated_response = self.simulate_phi3_inference(input_text).await?;
        
        let processing_time = start_time.elapsed().as_millis() as f64;
        
        let result = AIAnalysisResult {
            entity_id,
            input_text: input_text.to_string(),
            generated_response,
            confidence_score: 0.87, // Simulated confidence
            processing_time_ms: processing_time,
            token_count: input_text.split_whitespace().count(),
            timestamp: chrono::Utc::now(),
        };
        
        // Cache the result
        self.analysis_cache.insert(input_text.to_string(), result.clone());
        
        info!("🧠 Phi-3 analysis complete for entity {} in {:.2}ms", 
              entity_id, processing_time);
        
        Ok(result)
    }
    
    async fn simulate_phi3_inference(&self, input: &str) -> CandleResult<String> {
        // Simulate Phi-3 processing with realistic responses
        let response = match input.to_lowercase().as_str() {
            text if text.contains("threat") => {
                "Analyzing threat patterns: Medium risk detected. Recommend enhanced monitoring and defensive posture adjustments."
            },
            text if text.contains("task") => {
                "Task analysis: Optimal execution path identified. Resource allocation efficient. Estimated completion: 85% probability within timeline."
            },
            text if text.contains("agent") => {
                "Agent behavioral analysis: Performance metrics nominal. Capability utilization at 73%. Recommend task diversification."
            },
            text if text.contains("network") => {
                "Network topology analysis: 47 nodes identified. 3 potential vulnerabilities detected. Security recommendations generated."
            },
            _ => {
                "Professional analytical assessment complete. Data patterns analyzed. Actionable intelligence generated for operational use."
            }
        };
        
        // Simulate processing delay (realistic for Phi-3)
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        
        Ok(response.to_string())
    }
    
    pub fn get_analysis_cache(&self) -> &HashMap<String, AIAnalysisResult> {
        &self.analysis_cache
    }
    
    pub fn clear_cache(&mut self) {
        self.analysis_cache.clear();
        info!("🗑️ Phi-3 analysis cache cleared");
    }
}

/// Legion ECS System for Phi-3 AI Processing
pub fn phi3_analysis_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("phi3_analysis_system")
        .write_resource::<Phi3Manager>()
        .with_query(<(Entity, &Phi3Component, &crate::Task)>::query())
        .build(|_, world, resources, query| {
            let mut phi3_manager = resources.get_mut::<Phi3Manager>().unwrap();
            
            for (entity, phi3_comp, task) in query.iter(world) {
                if phi3_comp.active {
                    // Prepare input text for AI analysis
                    let input_text = format!(
                        "Analyze task: {} with priority {:?} and status {:?}", 
                        task.name, task.priority, task.status
                    );
                    
                    // In a real system, you'd spawn async tasks or use a queue
                    info!("🧠 Scheduling Phi-3 analysis for entity {:?}", entity);
                    info!("📝 Input: {}", input_text);
                    
                    // This would typically be handled asynchronously
                    // For demonstration, we log the scheduled analysis
                }
            }
        })
}

/// System for AI-driven task optimization
pub fn ai_task_optimization_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("ai_task_optimization")
        .read_resource::<Phi3Manager>()
        .with_query(<(Entity, &mut crate::Task, &Phi3Component)>::query())
        .build(|_, world, resources, query| {
            let phi3_manager = resources.get::<Phi3Manager>().unwrap();
            
            for (entity, mut task, phi3_comp) in query.iter_mut(world) {
                if phi3_comp.active && phi3_manager.analysis_cache.len() > 0 {
                    // AI-driven task priority optimization
                    match task.priority {
                        crate::TaskPriority::Low => {
                            if task.name.contains("critical") || task.name.contains("urgent") {
                                task.priority = crate::TaskPriority::High;
                                info!("🧠 AI upgraded task '{}' priority to High", task.name);
                            }
                        },
                        crate::TaskPriority::Medium => {
                            if task.name.contains("emergency") {
                                task.priority = crate::TaskPriority::Emergency;
                                info!("🚨 AI escalated task '{}' to Emergency", task.name);
                            }
                        },
                        _ => {}
                    }
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_phi3_manager_creation() {
        let manager = Phi3Manager::new();
        assert!(manager.is_ok());
    }
    
    #[tokio::test]
    async fn test_ai_analysis_simulation() {
        let mut manager = Phi3Manager::new().unwrap();
        let entity_id = Uuid::new_v4();
        
        let result = manager
            .analyze_entity_data(entity_id, "threat analysis needed")
            .await
            .unwrap();
        
        assert_eq!(result.entity_id, entity_id);
        assert!(result.generated_response.contains("threat"));
        assert!(result.confidence_score > 0.0);
    }
    
    #[tokio::test]
    async fn test_analysis_caching() {
        let mut manager = Phi3Manager::new().unwrap();
        let entity_id = Uuid::new_v4();
        let input = "test input";
        
        // First analysis
        let result1 = manager.analyze_entity_data(entity_id, input).await.unwrap();
        
        // Second analysis should use cache
        let result2 = manager.analyze_entity_data(entity_id, input).await.unwrap();
        
        assert_eq!(result1.generated_response, result2.generated_response);
        assert_eq!(manager.analysis_cache.len(), 1);
    }
}