//! DSL Core Types and Operations
//!
//! Re-exports and adapts DSL modules from sx9-foundation-daemon

use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// DSL operation types (simplified from foundation-daemon)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DSLOperation {
    HashTrigger {
        sch: String,
        operation: String,
        metadata: HashMap<String, String>,
    },
    IntelCollection {
        hash: Option<String>,
        tool: String,
        timeout: String,
    },
    KaliTool {
        tool: String,
        args: Vec<String>,
    },
    Workflow {
        steps: Vec<DSLOperation>,
    },
    Parallel {
        operations: Vec<DSLOperation>,
    },
}

/// DSL execution result
pub type DSLResult<T> = Result<T, DSLError>;

/// DSL error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DSLError {
    ParseError(String),
    ExecutionError(String),
    WasmError(String),
    Timeout,
}

impl std::fmt::Display for DSLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DSLError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DSLError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            DSLError::WasmError(msg) => write!(f, "WASM error: {}", msg),
            DSLError::Timeout => write!(f, "Operation timeout"),
        }
    }
}

impl std::error::Error for DSLError {}

/// DSL Engine
pub struct DSLEngine {
    wasm_enabled: bool,
}

impl DSLEngine {
    pub fn new(wasm_enabled: bool) -> Self {
        Self { wasm_enabled }
    }

    pub async fn parse(&self, input: &str) -> DSLResult<DSLOperation> {
        // Simple parser - in production would use proper DSL parser
        if input.starts_with("hash_trigger!") {
            // Parse hash trigger operation
            Ok(DSLOperation::HashTrigger {
                sch: "default".to_string(),
                operation: input.to_string(),
                metadata: HashMap::new(),
            })
        } else if input.starts_with("kali_tool!") {
            Ok(DSLOperation::KaliTool {
                tool: "nmap".to_string(),
                args: vec![],
            })
        } else {
            Err(DSLError::ParseError("Unknown operation".to_string()))
        }
    }

    pub async fn execute(&self, operation: &DSLOperation) -> DSLResult<String> {
        match operation {
            DSLOperation::HashTrigger { .. } => {
                Ok("Hash trigger executed".to_string())
            }
            DSLOperation::IntelCollection { .. } => {
                Ok("Intel collection executed".to_string())
            }
            DSLOperation::KaliTool { tool, .. } => {
                Ok(format!("Kali tool {} executed", tool))
            }
            DSLOperation::Workflow { steps } => {
                for step in steps {
                    self.execute(step).await?;
                }
                Ok("Workflow executed".to_string())
            }
            DSLOperation::Parallel { operations } => {
                // Execute in parallel
                let mut handles = Vec::new();
                for op in operations {
                    let engine = self;
                    let op_clone = op.clone();
                    handles.push(sx9_foundation_core::async_runtime::tokio::spawn(async move {
                        engine.execute(&op_clone).await
                    }));
                }
                
                for handle in handles {
                    handle.await.map_err(|e| DSLError::ExecutionError(e.to_string()))??;
                }
                Ok("Parallel operations executed".to_string())
            }
        }
    }
}

