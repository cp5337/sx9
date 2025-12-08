use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhiModelConfig {
    pub model_path: String,
    pub max_length: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub device: String,
}

impl Default for PhiModelConfig {
    fn default() -> Self {
        Self {
            model_path: "http://localhost:11434".to_string(), // Ollama endpoint
            max_length: 2048,
            temperature: 0.7,
            top_p: 0.9,
            device: "cpu".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhiRequest {
    pub prompt: String,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
    pub system_prompt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhiResponse {
    pub generated_text: String,
    pub tokens_used: usize,
    pub processing_time_ms: u64,
    pub model_version: String,
}

pub struct PhiService {
    config: PhiModelConfig,
    model_loaded: Arc<RwLock<bool>>,
}

impl PhiService {
    pub fn new(config: PhiModelConfig) -> Self {
        Self {
            config,
            model_loaded: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing Ollama Phi model service...");

        // Test Ollama connection
        let client = reqwest::Client::new();
        let ollama_url = format!("{}/api/tags", self.config.model_path);

        match client.get(&ollama_url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    info!("✅ Ollama connection successful");
                    let mut loaded = self.model_loaded.write().await;
                    *loaded = true;
                    info!("🧠 Phi model ready via Ollama");
                    Ok(())
                } else {
                    warn!("⚠️ Ollama not responding, falling back to simulation mode");
                    Ok(())
                }
            }
            Err(_) => {
                warn!("⚠️ Ollama not available, using simulation mode");
                Ok(())
            }
        }
    }

    pub async fn generate(&self, request: PhiRequest) -> Result<PhiResponse> {
        let start_time = std::time::Instant::now();

        // Check if model is loaded (Ollama available)
        let loaded = {
            let loaded = self.model_loaded.read().await;
            *loaded
        };

        if loaded {
            // Use real Ollama inference
            match self.call_ollama(&request).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    warn!("Ollama call failed, falling back to simulation: {}", e);
                }
            }
        }

        // Fallback to simulation mode
        let full_prompt = if let Some(system) = &request.system_prompt {
            format!("System: {}\nUser: {}\nAssistant:", system, request.prompt)
        } else {
            format!("User: {}\nAssistant:", request.prompt)
        };

        let response_text = self.simulate_phi_generation(&full_prompt, &request).await?;
        let processing_time = start_time.elapsed();

        Ok(PhiResponse {
            generated_text: response_text,
            tokens_used: self.estimate_tokens(&request.prompt),
            processing_time_ms: processing_time.as_millis() as u64,
            model_version: "Phi-simulation".to_string(),
        })
    }

    async fn simulate_phi_generation(&self, prompt: &str, request: &PhiRequest) -> Result<String> {
        // In production, this would interface with the actual Phi model
        // For now, we'll simulate intelligent responses based on prompt patterns

        let response = if prompt.to_lowercase().contains("statistical") {
            "Based on the statistical analysis, I can provide insights into the data patterns. The key metrics show significant correlation between variables with a confidence interval of 95%."
        } else if prompt.to_lowercase().contains("ctas") {
            "CTAS 7.0 represents a revolutionary advancement in tactical analysis systems. The foundation architecture provides unprecedented scalability and performance."
        } else if prompt.to_lowercase().contains("code") || prompt.to_lowercase().contains("rust") {
            "Here's a Rust implementation that follows best practices:\n\n```rust\nuse tokio::sync::RwLock;\nuse std::sync::Arc;\n\n// Implementation follows CTAS patterns\n```"
        } else {
            "I understand your request. Let me provide a comprehensive response based on the available data and analysis capabilities of the CTAS 7.0 statistical engine."
        };

        // Apply length constraints
        let max_len = request.max_tokens.unwrap_or(self.config.max_length);
        let truncated = if response.len() > max_len {
            &response[..max_len]
        } else {
            response
        };

        Ok(truncated.to_string())
    }

    async fn call_ollama(&self, request: &PhiRequest) -> Result<PhiResponse> {
        let start_time = std::time::Instant::now();
        let client = reqwest::Client::new();

        // Build the full prompt
        let full_prompt = if let Some(system) = &request.system_prompt {
            format!("{}\n\n{}", system, request.prompt)
        } else {
            request.prompt.clone()
        };

        // Ollama API request
        let ollama_request = serde_json::json!({
            "model": "phi",
            "prompt": full_prompt,
            "stream": false,
            "options": {
                "temperature": request.temperature.unwrap_or(self.config.temperature),
                "num_ctx": request.max_tokens.unwrap_or(self.config.max_length)
            }
        });

        let ollama_url = format!("{}/api/generate", self.config.model_path);
        let response = client
            .post(&ollama_url)
            .json(&ollama_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Ollama API error: {}", response.status()));
        }

        let ollama_response: serde_json::Value = response.json().await?;
        let generated_text = ollama_response["response"]
            .as_str()
            .unwrap_or("Error: No response from Ollama")
            .to_string();

        let processing_time = start_time.elapsed();

        Ok(PhiResponse {
            generated_text,
            tokens_used: self.estimate_tokens(&request.prompt),
            processing_time_ms: processing_time.as_millis() as u64,
            model_version: "Phi-via-Ollama".to_string(),
        })
    }

    fn estimate_tokens(&self, text: &str) -> usize {
        // Rough estimation: ~4 characters per token
        (text.len() + 3) / 4
    }

    pub async fn health_check(&self) -> bool {
        let loaded = self.model_loaded.read().await;
        *loaded
    }

    pub fn get_model_info(&self) -> serde_json::Value {
        serde_json::json!({
            "model_name": "Phi-3.5-mini-instruct",
            "model_size": "4B parameters",
            "capabilities": [
                "Text generation",
                "Code completion",
                "Statistical analysis",
                "CTAS integration"
            ],
            "max_context_length": self.config.max_length,
            "device": self.config.device
        })
    }
}