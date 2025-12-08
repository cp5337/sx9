//! Operations Order (OPORD) Management
//! Five-paragraph format tactical orders

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OPORD {
    pub id: String,
    pub operation_name: String,
    pub dtg: DateTime<Utc>,
    pub situation: Situation,
    pub mission: String,
    pub execution: Execution,
    pub service_support: ServiceSupport,
    pub command_signal: CommandSignal,
    pub status: OPORDStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Situation {
    pub enemy_forces: String,
    pub friendly_forces: String,
    pub terrain_weather: String,
    pub civil_considerations: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
    pub commander_intent: String,
    pub concept_of_operations: String,
    pub tasks: Vec<Task>,
    pub coordinating_instructions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub unit: String,
    pub task_description: String,
    pub purpose: String,
    pub priority: TaskPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Priority1,
    Priority2,
    Priority3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSupport {
    pub logistics: String,
    pub medical: String,
    pub transport: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandSignal {
    pub command_posts: Vec<CommandPost>,
    pub frequencies: Vec<Frequency>,
    pub signals: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPost {
    pub name: String,
    pub location: String,
    pub primary_freq: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frequency {
    pub name: String,
    pub frequency: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OPORDStatus {
    Draft,
    Review,
    Approved,
    Executing,
    Complete,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOPORDRequest {
    pub operation_name: String,
    pub mission_statement: String,
    pub commander_intent: String,
}

impl OPORD {
    pub fn new(
        operation_name: String,
        mission: String,
        commander_intent: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            operation_name,
            dtg: Utc::now(),
            situation: Situation {
                enemy_forces: "TBD".to_string(),
                friendly_forces: "TBD".to_string(),
                terrain_weather: "TBD".to_string(),
                civil_considerations: "TBD".to_string(),
            },
            mission,
            execution: Execution {
                commander_intent,
                concept_of_operations: "TBD".to_string(),
                tasks: Vec::new(),
                coordinating_instructions: Vec::new(),
            },
            service_support: ServiceSupport {
                logistics: "TBD".to_string(),
                medical: "TBD".to_string(),
                transport: "TBD".to_string(),
            },
            command_signal: CommandSignal {
                command_posts: Vec::new(),
                frequencies: Vec::new(),
                signals: "TBD".to_string(),
            },
            status: OPORDStatus::Draft,
            created_at: Utc::now(),
        }
    }

    pub fn add_task(&mut self, unit: String, task: String, purpose: String) {
        self.execution.tasks.push(Task {
            unit,
            task_description: task,
            purpose,
            priority: TaskPriority::Priority2,
        });
    }

    pub fn update_status(&mut self, status: OPORDStatus) {
        self.status = status;
    }
}