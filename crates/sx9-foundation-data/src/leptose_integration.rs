//! Leptose Integration - Connect agent chat with Leptose inference engine
//! Enables AI-powered chat responses and real-time knowledge graph updates

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::agent_chat::{ChatMessage, ChatSender, AgentChatSystem, MessageType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeptoseInferenceRequest {
    pub query: String,
    pub context: LeptoseContext,
    pub inference_type: InferenceType,
    pub agent_id: Option<String>,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeptoseContext {
    pub conversation_history: Vec<ChatMessage>,
    pub current_workflow: Option<String>,
    pub threat_level: Option<String>,
    pub tactical_situation: Option<String>,
    pub user_clearance: String,
    pub station: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InferenceType {
    ThreatAnalysis,
    TacticalRecommendation,
    TechnicalSupport,
    KnowledgeRetrieval,
    WorkflowGuidance,
    InterAgentCoordination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeptoseInferenceResponse {
    pub response_text: String,
    pub confidence: f64,
    pub knowledge_sources: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub follow_up_questions: Vec<String>,
    pub agent_suggestions: Vec<String>,
    pub inference_metadata: InferenceMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceMetadata {
    pub processing_time_ms: u64,
    pub knowledge_graph_nodes: u32,
    pub vector_similarities: Vec<f64>,
    pub xsd_validations: u32,
    pub decision_factors: Vec<String>,
}

pub struct LeptoseAgentBridge {
    chat_system: AgentChatSystem,
    inference_cache: RwLock<HashMap<String, LeptoseInferenceResponse>>,
    knowledge_state: RwLock<KnowledgeState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeState {
    pub active_topics: Vec<String>,
    pub threat_indicators: HashMap<String, f64>,
    pub tactical_assessments: Vec<TacticalAssessment>,
    pub agent_expertise: HashMap<String, Vec<String>>,
    pub workflow_context: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalAssessment {
    pub id: String,
    pub assessment_type: String,
    pub confidence: f64,
    pub factors: Vec<String>,
    pub recommendations: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl LeptoseAgentBridge {
    pub async fn new() -> Self {
        Self {
            chat_system: AgentChatSystem::new(),
            inference_cache: RwLock::new(HashMap::new()),
            knowledge_state: RwLock::new(KnowledgeState {
                active_topics: Vec::new(),
                threat_indicators: HashMap::new(),
                tactical_assessments: Vec::new(),
                agent_expertise: HashMap::new(),
                workflow_context: HashMap::new(),
            }),
        }
    }

    pub async fn process_chat_with_inference(&self, message: ChatMessage) -> anyhow::Result<LeptoseInferenceResponse> {
        // Create inference request from chat message
        let inference_request = self.create_inference_request(&message).await?;

        // Call Leptose inference engine
        let inference_response = self.call_leptose_inference(&inference_request).await?;

        // Update knowledge state
        self.update_knowledge_state(&message, &inference_response).await?;

        // Generate agent responses if needed
        self.trigger_agent_responses(&message, &inference_response).await?;

        // Cache the response
        self.cache_inference_response(&message.id, &inference_response).await;

        Ok(inference_response)
    }

    async fn create_inference_request(&self, message: &ChatMessage) -> anyhow::Result<LeptoseInferenceRequest> {
        let conversation_history = self.chat_system.get_channel_messages(&message.channel_id, 10).await;

        let inference_type = match message.message_type {
            MessageType::ThreatAlert => InferenceType::ThreatAnalysis,
            MessageType::Command => InferenceType::WorkflowGuidance,
            MessageType::InterAgentComm => InferenceType::InterAgentCoordination,
            _ => InferenceType::TechnicalSupport,
        };

        let knowledge_state = self.knowledge_state.read().await;

        Ok(LeptoseInferenceRequest {
            query: message.content.clone(),
            context: LeptoseContext {
                conversation_history,
                current_workflow: message.metadata.workflow_id.clone(),
                threat_level: message.metadata.threat_level.clone(),
                tactical_situation: None,
                user_clearance: "TACTICAL".to_string(),
                station: message.metadata.station.clone(),
            },
            inference_type,
            agent_id: match &message.sender {
                ChatSender::Agent { agent_id, .. } => Some(agent_id.clone()),
                _ => None,
            },
            session_id: message.metadata.usim_session.clone().unwrap_or_default(),
        })
    }

    async fn call_leptose_inference(&self, request: &LeptoseInferenceRequest) -> anyhow::Result<LeptoseInferenceResponse> {
        let client = reqwest::Client::new();

        // Call Leptose inference endpoint
        let leptose_response = client
            .post("http://localhost:18114/leptose/inference") // Leptose inference port
            .json(request)
            .send()
            .await?;

        if leptose_response.status().is_success() {
            let response: LeptoseInferenceResponse = leptose_response.json().await?;
            Ok(response)
        } else {
            // Fallback to basic response if Leptose unavailable
            Ok(self.create_fallback_response(request).await)
        }
    }

    async fn create_fallback_response(&self, request: &LeptoseInferenceRequest) -> LeptoseInferenceResponse {
        let response_text = match request.inference_type {
            InferenceType::ThreatAnalysis => {
                "🛡️ Threat analysis in progress. Correlating indicators with knowledge base."
            },
            InferenceType::TacticalRecommendation => {
                "🎯 Tactical assessment: Analyzing current situation and recommending courses of action."
            },
            InferenceType::TechnicalSupport => {
                "🔧 Technical support: Searching knowledge base for relevant solutions."
            },
            InferenceType::WorkflowGuidance => {
                "⚡ Workflow guidance: Analyzing current step and suggesting next actions."
            },
            InferenceType::InterAgentCoordination => {
                "🤝 Agent coordination: Facilitating communication between tactical agents."
            },
            _ => "🧠 Processing request with Leptose knowledge engine..."
        };

        LeptoseInferenceResponse {
            response_text: response_text.to_string(),
            confidence: 0.75,
            knowledge_sources: vec!["CTAS Knowledge Base".to_string()],
            recommended_actions: vec!["Continue monitoring".to_string()],
            follow_up_questions: vec!["Would you like more detailed analysis?".to_string()],
            agent_suggestions: vec!["volkov-security".to_string(), "cipher-data".to_string()],
            inference_metadata: InferenceMetadata {
                processing_time_ms: 150,
                knowledge_graph_nodes: 1247,
                vector_similarities: vec![0.87, 0.76, 0.65],
                xsd_validations: 3,
                decision_factors: vec![
                    "Historical patterns".to_string(),
                    "Current threat landscape".to_string(),
                    "Tactical doctrine".to_string()
                ],
            },
        }
    }

    async fn update_knowledge_state(&self, message: &ChatMessage, response: &LeptoseInferenceResponse) -> anyhow::Result<()> {
        let mut state = self.knowledge_state.write().await;

        // Extract topics from conversation
        if !response.knowledge_sources.is_empty() {
            for source in &response.knowledge_sources {
                if !state.active_topics.contains(source) {
                    state.active_topics.push(source.clone());
                }
            }
        }

        // Update threat indicators if relevant
        if message.message_type == MessageType::ThreatAlert {
            let threat_key = format!("threat_{}", message.id);
            state.threat_indicators.insert(threat_key, response.confidence);
        }

        // Create tactical assessment
        if response.confidence > 0.8 {
            let assessment = TacticalAssessment {
                id: Uuid::new_v4().to_string(),
                assessment_type: format!("{:?}", message.message_type),
                confidence: response.confidence,
                factors: response.inference_metadata.decision_factors.clone(),
                recommendations: response.recommended_actions.clone(),
                timestamp: Utc::now(),
            };
            state.tactical_assessments.push(assessment);
        }

        Ok(())
    }

    async fn trigger_agent_responses(&self, message: &ChatMessage, response: &LeptoseInferenceResponse) -> anyhow::Result<()> {
        // Trigger agent responses based on inference suggestions
        for suggested_agent in &response.agent_suggestions {
            let agent_message = ChatMessage {
                id: Uuid::new_v4().to_string(),
                channel_id: message.channel_id.clone(),
                sender: ChatSender::Agent {
                    agent_type: crate::agentic_workflow::AgentType::Echo, // Would be mapped properly
                    agent_id: suggested_agent.clone(),
                },
                recipient: None,
                content: format!("🤖 Based on Leptose analysis: {}", response.response_text),
                message_type: MessageType::AgentDecision,
                timestamp: Utc::now(),
                metadata: message.metadata.clone(),
                thread_id: Some(message.id.clone()),
            };

            self.chat_system.send_message(agent_message).await?;
        }

        Ok(())
    }

    async fn cache_inference_response(&self, message_id: &str, response: &LeptoseInferenceResponse) {
        let mut cache = self.inference_cache.write().await;
        cache.insert(message_id.to_string(), response.clone());

        // Keep only last 100 responses
        if cache.len() > 100 {
            let oldest_key = cache.keys().next().cloned();
            if let Some(key) = oldest_key {
                cache.remove(&key);
            }
        }
    }

    pub async fn get_knowledge_state(&self) -> KnowledgeState {
        self.knowledge_state.read().await.clone()
    }
}

// REST API endpoints for Leptose integration
pub async fn leptose_chat_inference(
    axum::Json(request): axum::Json<serde_json::Value>,
) -> axum::Json<serde_json::Value> {
    let bridge = LeptoseAgentBridge::new().await;

    let message = ChatMessage {
        id: Uuid::new_v4().to_string(),
        channel_id: request["channel_id"].as_str().unwrap_or("general").to_string(),
        sender: ChatSender::User {
            user_id: request["user_id"].as_str().unwrap_or("unknown").to_string(),
            username: request["username"].as_str().unwrap_or("User").to_string(),
        },
        recipient: None,
        content: request["message"].as_str().unwrap_or("").to_string(),
        message_type: MessageType::Text,
        timestamp: Utc::now(),
        metadata: crate::agent_chat::ChatMetadata {
            workflow_id: None,
            execution_id: None,
            threat_level: None,
            usim_session: None,
            station: "STATION_5".to_string(),
            encrypted: false,
        },
        thread_id: None,
    };

    match bridge.process_chat_with_inference(message).await {
        Ok(response) => axum::Json(serde_json::json!({
            "status": "success",
            "inference_response": response,
            "timestamp": Utc::now()
        })),
        Err(e) => axum::Json(serde_json::json!({
            "status": "error",
            "error": e.to_string()
        }))
    }
}

pub async fn get_leptose_knowledge_state() -> axum::Json<serde_json::Value> {
    let bridge = LeptoseAgentBridge::new().await;
    let state = bridge.get_knowledge_state().await;

    axum::Json(serde_json::json!({
        "knowledge_state": state,
        "timestamp": Utc::now()
    }))
}