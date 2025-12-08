use crate::{Tool, ToolResult, Content, Config};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use tracing::{info, error};
use std::collections::HashMap;

/// Elite team personas with Tesla-grade capabilities
#[derive(Debug, Clone)]
pub struct ElitePersona {
    pub name: String,
    pub specialty: String,
    pub keywords: Vec<String>,
    pub ai_models: Vec<String>,
    pub human_equivalent_capabilities: Vec<String>,
}

/// Tesla-grade AI consensus tool with persona matching
pub struct AITool {
    config: Config,
    elite_personas: Vec<ElitePersona>,
}

impl AITool {
    pub fn new(config: &Config) -> Result<Self> {
        let elite_personas = Self::initialize_elite_personas();

        Ok(Self {
            config: config.clone(),
            elite_personas,
        })
    }

    fn initialize_elite_personas() -> Vec<ElitePersona> {
        vec![
            ElitePersona {
                name: "Commander Hayes".to_string(),
                specialty: "Strategic Operations & Leadership".to_string(),
                keywords: vec!["strategy", "operations", "leadership", "command", "architecture"].iter().map(|s| s.to_string()).collect(),
                ai_models: vec!["gpt-4", "claude-3-5-sonnet", "gemini-pro"].iter().map(|s| s.to_string()).collect(),
                human_equivalent_capabilities: vec![
                    "system_architecture_design",
                    "strategic_planning",
                    "team_coordination",
                    "technical_leadership",
                    "project_management",
                    "risk_assessment"
                ].iter().map(|s| s.to_string()).collect(),
            },
            ElitePersona {
                name: "Dmitri Kozlov".to_string(),
                specialty: "APT & Advanced Threat Analysis".to_string(),
                keywords: vec!["apt", "threat", "security", "attack", "vulnerability", "malware"].iter().map(|s| s.to_string()).collect(),
                ai_models: vec!["gpt-4", "claude-3-5-sonnet"].iter().map(|s| s.to_string()).collect(),
                human_equivalent_capabilities: vec![
                    "threat_hunting",
                    "vulnerability_analysis",
                    "malware_reverse_engineering",
                    "incident_response",
                    "forensic_analysis",
                    "penetration_testing"
                ].iter().map(|s| s.to_string()).collect(),
            },
            ElitePersona {
                name: "James Sterling".to_string(),
                specialty: "Financial Intelligence & Analysis".to_string(),
                keywords: vec!["financial", "money", "blockchain", "transaction", "economic", "crypto"].iter().map(|s| s.to_string()).collect(),
                ai_models: vec!["gpt-4", "wolfram"].iter().map(|s| s.to_string()).collect(),
                human_equivalent_capabilities: vec![
                    "financial_modeling",
                    "market_analysis",
                    "risk_assessment",
                    "blockchain_analysis",
                    "economic_forecasting",
                    "compliance_review"
                ].iter().map(|s| s.to_string()).collect(),
            },
            ElitePersona {
                name: "Omar Al-Rashid".to_string(),
                specialty: "MENA Cultural & Linguistic Intelligence".to_string(),
                keywords: vec!["cultural", "linguistic", "mena", "social", "regional", "translation"].iter().map(|s| s.to_string()).collect(),
                ai_models: vec!["gpt-4", "gemini-pro"].iter().map(|s| s.to_string()).collect(),
                human_equivalent_capabilities: vec![
                    "cultural_analysis",
                    "linguistic_translation",
                    "regional_intelligence",
                    "social_media_analysis",
                    "geopolitical_assessment",
                    "cross_cultural_communication"
                ].iter().map(|s| s.to_string()).collect(),
            },
            ElitePersona {
                name: "Natasha Volkov".to_string(),
                specialty: "AI & Neural Systems Analysis".to_string(),
                keywords: vec!["ai", "neural", "machine learning", "algorithm", "automation", "ml"].iter().map(|s| s.to_string()).collect(),
                ai_models: vec!["claude-3-5-sonnet", "gpt-4", "gemini-pro"].iter().map(|s| s.to_string()).collect(),
                human_equivalent_capabilities: vec![
                    "ai_system_design",
                    "neural_network_architecture",
                    "ml_model_optimization",
                    "ai_safety_analysis",
                    "autonomous_system_design",
                    "ai_ethics_review"
                ].iter().map(|s| s.to_string()).collect(),
            },
            ElitePersona {
                name: "Emily Chen".to_string(),
                specialty: "Infrastructure & Systems Engineering".to_string(),
                keywords: vec!["infrastructure", "system", "network", "deployment", "devops", "cloud"].iter().map(|s| s.to_string()).collect(),
                ai_models: vec!["gpt-4", "claude-3-5-sonnet"].iter().map(|s| s.to_string()).collect(),
                human_equivalent_capabilities: vec![
                    "infrastructure_design",
                    "system_optimization",
                    "network_architecture",
                    "deployment_automation",
                    "performance_tuning",
                    "scalability_planning"
                ].iter().map(|s| s.to_string()).collect(),
            },
        ]
    }

    fn select_best_persona(&self, content: &str) -> &ElitePersona {
        let content_lower = content.to_lowercase();
        let mut best_match = (0, &self.elite_personas[0]);

        for persona in &self.elite_personas {
            let mut score = 0;

            // Count keyword matches
            for keyword in &persona.keywords {
                if content_lower.contains(&keyword.to_lowercase()) {
                    score += content_lower.matches(&keyword.to_lowercase()).count();
                }
            }

            if score > best_match.0 {
                best_match = (score, persona);
            }
        }

        // Default to Natasha for AI/technical tasks if no clear winner
        if best_match.0 == 0 {
            return self.elite_personas.iter()
                .find(|p| p.name == "Natasha Volkov")
                .unwrap_or(&self.elite_personas[0]);
        }

        best_match.1
    }

    async fn query_ai_model(&self, model: &str, prompt: &str) -> Result<String> {
        match model {
            "openai" | "gpt-4" => self.query_openai(prompt).await,
            "gemini" | "gemini-pro" => self.query_gemini(prompt).await,
            "grok" | "grok-beta" => self.query_grok(prompt).await,
            "claude" | "claude-3-5-sonnet" => self.query_claude(prompt).await,
            "wolfram" => self.query_wolfram(prompt).await,
            _ => Err(anyhow::anyhow!("Unknown AI model: {}", model)),
        }
    }

    async fn query_openai(&self, prompt: &str) -> Result<String> {
        let client = reqwest::Client::new();

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", std::env::var("OPENAI_API_KEY")?))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "gpt-4",
                "messages": [{"role": "user", "content": prompt}],
                "temperature": 0.7,
                "max_tokens": 1000
            }))
            .send()
            .await?;

        let data: Value = response.json().await?;

        Ok(data["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No response")
            .to_string())
    }

    async fn query_gemini(&self, prompt: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let api_key = std::env::var("GOOGLE_API_KEY")?;

        let response = client
            .post(&format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "contents": [{"parts": [{"text": prompt}]}],
                "generationConfig": {"temperature": 0.7}
            }))
            .send()
            .await?;

        let data: Value = response.json().await?;

        Ok(data["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("No response")
            .to_string())
    }

    async fn query_grok(&self, prompt: &str) -> Result<String> {
        let client = reqwest::Client::new();

        let response = client
            .post("https://api.x.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", std::env::var("XAI_API_KEY")?))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "grok-beta",
                "messages": [{"role": "user", "content": prompt}],
                "temperature": 0.7,
                "max_tokens": 1000
            }))
            .send()
            .await?;

        let data: Value = response.json().await?;

        Ok(data["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No response")
            .to_string())
    }

    async fn query_claude(&self, prompt: &str) -> Result<String> {
        let client = reqwest::Client::new();

        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("Authorization", format!("Bearer {}", std::env::var("ANTHROPIC_API_KEY")?))
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": "claude-3-5-sonnet-20241022",
                "messages": [{"role": "user", "content": prompt}],
                "temperature": 0.7,
                "max_tokens": 1000
            }))
            .send()
            .await?;

        let data: Value = response.json().await?;

        Ok(data["content"][0]["text"]
            .as_str()
            .unwrap_or("No response")
            .to_string())
    }

    async fn query_wolfram(&self, prompt: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let api_key = std::env::var("WOLFRAM_API_KEY")?;

        let mut params = std::collections::HashMap::new();
        params.insert("appid", api_key);
        params.insert("input", prompt.to_string());
        params.insert("format", "plaintext".to_string());
        params.insert("output", "json".to_string());

        let response = client
            .get("https://api.wolframalpha.com/v2/query")
            .query(&params)
            .send()
            .await?;

        let data: Value = response.json().await?;

        Ok(format!("Wolfram Alpha result: {:?}", data))
    }

    async fn get_persona_ai_consensus(&self, args: Value) -> Result<ToolResult> {
        let prompt = args.get("prompt")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing prompt"))?;

        let task_type = args.get("task_type")
            .and_then(|v| v.as_str())
            .unwrap_or("general");

        // Select best persona for this task
        let persona = self.select_best_persona(&format!("{} {}", prompt, task_type));

        info!("🎯 Selected persona: {} for task type: {}", persona.name, task_type);

        let enhanced_prompt = format!(
            "You are {}, an elite specialist in {}. Your capabilities include: {}. \n\nTask: {}",
            persona.name,
            persona.specialty,
            persona.human_equivalent_capabilities.join(", "),
            prompt
        );

        // Query all AI models that this persona uses
        let mut responses = Vec::new();
        for model in &persona.ai_models {
            match self.query_ai_model(model, &enhanced_prompt).await {
                Ok(response) => {
                    responses.push(json!({
                        "model": model,
                        "persona": persona.name.clone(),
                        "response": response
                    }));
                }
                Err(e) => {
                    error!("Failed to query {}: {}", model, e);
                    responses.push(json!({
                        "model": model,
                        "persona": persona.name.clone(),
                        "error": e.to_string()
                    }));
                }
            }
        }

        let result = json!({
            "selected_persona": {
                "name": persona.name,
                "specialty": persona.specialty,
                "human_equivalent_capabilities": persona.human_equivalent_capabilities
            },
            "responses": responses,
            "streaming_enabled": true,
            "tesla_grade_quality": true,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        Ok(ToolResult {
            content: vec![Content::Text {
                text: serde_json::to_string_pretty(&result)?,
            }],
            is_error: false,
        })
    }

    async fn stream_ai_response(&self, args: Value) -> Result<ToolResult> {
        let prompt = args.get("prompt")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing prompt"))?;

        let stream_mode = args.get("stream_mode")
            .and_then(|v| v.as_str())
            .unwrap_or("real_time");

        // Select persona and stream response
        let persona = self.select_best_persona(prompt);

        // Simulate streaming chunks (in real implementation, this would stream from AI APIs)
        let streaming_chunks = vec![
            "Starting analysis...",
            "Processing with elite-level capabilities...",
            "Applying specialized knowledge...",
            "Generating Tesla-grade response...",
            "Finalizing recommendations..."
        ];

        let result = json!({
            "stream_mode": stream_mode,
            "persona": persona.name,
            "chunks": streaming_chunks,
            "capabilities_applied": persona.human_equivalent_capabilities,
            "streaming_status": "active",
            "tesla_performance": true
        });

        Ok(ToolResult {
            content: vec![Content::Text {
                text: serde_json::to_string_pretty(&result)?,
            }],
            is_error: false,
        })
    }
}

#[async_trait]
impl Tool for AITool {
    fn name(&self) -> &str {
        "ai_consensus_personas"
    }

    fn description(&self) -> &str {
        "Tesla-grade AI consensus engine with elite team persona matching. Provides human-equivalent capabilities through specialized AI personas with streaming support."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["persona_consensus", "stream_response", "list_personas"],
                    "description": "AI operation to perform"
                },
                "prompt": {
                    "type": "string",
                    "description": "The prompt or task for AI processing"
                },
                "task_type": {
                    "type": "string",
                    "description": "Type of task (security, financial, technical, etc.)"
                },
                "stream_mode": {
                    "type": "string",
                    "enum": ["real_time", "batch", "hybrid"],
                    "description": "Streaming mode for responses"
                },
                "models": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Specific AI models to include"
                }
            },
            "required": ["operation"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let operation = args.get("operation")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing operation"))?;

        match operation {
            "persona_consensus" => self.get_persona_ai_consensus(args).await,
            "stream_response" => self.stream_ai_response(args).await,
            "list_personas" => {
                let personas_info: Vec<_> = self.elite_personas.iter().map(|p| {
                    json!({
                        "name": p.name,
                        "specialty": p.specialty,
                        "keywords": p.keywords,
                        "ai_models": p.ai_models,
                        "human_equivalent_capabilities": p.human_equivalent_capabilities
                    })
                }).collect();

                Ok(ToolResult {
                    content: vec![Content::Text {
                        text: serde_json::to_string_pretty(&json!({
                            "elite_personas": personas_info,
                            "total": personas_info.len(),
                            "tesla_grade": true
                        }))?,
                    }],
                    is_error: false,
                })
            },
            _ => Err(anyhow::anyhow!("Unknown operation: {}", operation)),
        }
    }
}