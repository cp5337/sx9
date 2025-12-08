//! Protocol buffer definitions for CTAS v7.0
//! 
//! Symbolic messaging protocol for agent communication

use prost::Message;
use serde::{Deserialize, Serialize};

/// Symbolic message structure for V7 agent communication
#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct SymbolicMessageV7 {
    #[prost(string, tag = "1")]
    pub cuid: String,
    
    #[prost(string, tag = "2")]
    pub persona: String,
    
    #[prost(string, tag = "3")]
    pub content: String,
    
    #[prost(string, tag = "4")]
    pub entropy: String,
    
    #[prost(string, tag = "5")]
    pub ttl: String,
    
    #[prost(string, tag = "6")]
    pub sch: String,
}

/// Acknowledgment response for V7 messages
#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AckV7 {
    #[prost(string, tag = "1")]
    pub status: String,
}

/// gRPC service definition for CTAS Agent V7
pub mod ctas_agent_v7 {
    tonic::include_proto!("ctas_agent_v7");
}

