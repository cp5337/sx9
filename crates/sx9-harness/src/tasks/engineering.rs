//! Cognitive Computing Factory Tasks
//!
//! V-N-N pattern (subject Forge implicit)
//! AI-first software factory workflow

use serde::{Serialize, Deserialize};

/// Factory task identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FactoryTaskId {
    // === INGEST ===
    ReceiveMission,
    ParseIntent,
    LoadContext,

    // === REASON ===
    SelectModel,
    BuildPrompt,
    ExecuteInference,

    // === PRODUCE ===
    GenerateArtifact,
    ValidateOutput,
    RefineResult,

    // === INTEGRATE ===
    ApplyChanges,
    VerifyIntegration,
    PublishResult,

    // === LEARN ===
    RecordOutcome,
    UpdateKnowledge,
    EmitTelemetry,
}

/// Factory task definition
#[derive(Debug, Clone)]
pub struct FactoryTask {
    pub id: FactoryTaskId,
    pub verb: &'static str,
    pub object: &'static str,
    pub context: &'static str,
    pub inputs: &'static [&'static str],
    pub outputs: &'static [&'static str],
    pub requires: &'static [FactoryTaskId],
}

/// Factory task registry
pub const FACTORY_TASKS: &[FactoryTask] = &[
    // === INGEST ===
    FactoryTask {
        id: FactoryTaskId::ReceiveMission,
        verb: "receive",
        object: "mission",
        context: "queue",
        inputs: &["mission_payload"],
        outputs: &["mission"],
        requires: &[],
    },
    FactoryTask {
        id: FactoryTaskId::ParseIntent,
        verb: "parse",
        object: "intent",
        context: "mission",
        inputs: &["mission"],
        outputs: &["intent"],
        requires: &[FactoryTaskId::ReceiveMission],
    },
    FactoryTask {
        id: FactoryTaskId::LoadContext,
        verb: "load",
        object: "context",
        context: "memory",
        inputs: &["intent"],
        outputs: &["context_bundle"],
        requires: &[FactoryTaskId::ParseIntent],
    },

    // === REASON ===
    FactoryTask {
        id: FactoryTaskId::SelectModel,
        verb: "select",
        object: "model",
        context: "capability",
        inputs: &["intent", "context_bundle"],
        outputs: &["model_config"],
        requires: &[FactoryTaskId::LoadContext],
    },
    FactoryTask {
        id: FactoryTaskId::BuildPrompt,
        verb: "build",
        object: "prompt",
        context: "template",
        inputs: &["intent", "context_bundle"],
        outputs: &["prompt"],
        requires: &[FactoryTaskId::LoadContext],
    },
    FactoryTask {
        id: FactoryTaskId::ExecuteInference,
        verb: "execute",
        object: "inference",
        context: "model",
        inputs: &["prompt", "model_config"],
        outputs: &["inference_result"],
        requires: &[FactoryTaskId::SelectModel, FactoryTaskId::BuildPrompt],
    },

    // === PRODUCE ===
    FactoryTask {
        id: FactoryTaskId::GenerateArtifact,
        verb: "generate",
        object: "artifact",
        context: "output",
        inputs: &["inference_result"],
        outputs: &["artifact"],
        requires: &[FactoryTaskId::ExecuteInference],
    },
    FactoryTask {
        id: FactoryTaskId::ValidateOutput,
        verb: "validate",
        object: "output",
        context: "schema",
        inputs: &["artifact"],
        outputs: &["validation_result"],
        requires: &[FactoryTaskId::GenerateArtifact],
    },
    FactoryTask {
        id: FactoryTaskId::RefineResult,
        verb: "refine",
        object: "result",
        context: "feedback",
        inputs: &["artifact", "validation_result"],
        outputs: &["refined_artifact"],
        requires: &[FactoryTaskId::ValidateOutput],
    },

    // === INTEGRATE ===
    FactoryTask {
        id: FactoryTaskId::ApplyChanges,
        verb: "apply",
        object: "changes",
        context: "target",
        inputs: &["refined_artifact"],
        outputs: &["applied_changes"],
        requires: &[FactoryTaskId::RefineResult],
    },
    FactoryTask {
        id: FactoryTaskId::VerifyIntegration,
        verb: "verify",
        object: "integration",
        context: "system",
        inputs: &["applied_changes"],
        outputs: &["integration_status"],
        requires: &[FactoryTaskId::ApplyChanges],
    },
    FactoryTask {
        id: FactoryTaskId::PublishResult,
        verb: "publish",
        object: "result",
        context: "channel",
        inputs: &["integration_status", "refined_artifact"],
        outputs: &["publication"],
        requires: &[FactoryTaskId::VerifyIntegration],
    },

    // === LEARN ===
    FactoryTask {
        id: FactoryTaskId::RecordOutcome,
        verb: "record",
        object: "outcome",
        context: "history",
        inputs: &["mission", "publication"],
        outputs: &["outcome_record"],
        requires: &[FactoryTaskId::PublishResult],
    },
    FactoryTask {
        id: FactoryTaskId::UpdateKnowledge,
        verb: "update",
        object: "knowledge",
        context: "memory",
        inputs: &["outcome_record"],
        outputs: &["knowledge_delta"],
        requires: &[FactoryTaskId::RecordOutcome],
    },
    FactoryTask {
        id: FactoryTaskId::EmitTelemetry,
        verb: "emit",
        object: "telemetry",
        context: "observability",
        inputs: &["outcome_record"],
        outputs: &["telemetry_event"],
        requires: &[FactoryTaskId::RecordOutcome],
    },
];

/// Get factory task by ID
pub fn get_factory_task(id: FactoryTaskId) -> Option<&'static FactoryTask> {
    FACTORY_TASKS.iter().find(|t| t.id == id)
}

/// Get factory task display string
pub fn factory_task_display(task: &FactoryTask) -> String {
    format!("{} {} {}", task.verb, task.object, task.context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_task_count() {
        assert_eq!(FACTORY_TASKS.len(), 15);
    }

    #[test]
    fn test_mission_flow() {
        let start = get_factory_task(FactoryTaskId::ReceiveMission).unwrap();
        assert!(start.requires.is_empty());

        let end = get_factory_task(FactoryTaskId::UpdateKnowledge).unwrap();
        assert!(end.requires.contains(&FactoryTaskId::RecordOutcome));
    }

    #[test]
    fn test_factory_task_display() {
        let task = get_factory_task(FactoryTaskId::ExecuteInference).unwrap();
        assert_eq!(factory_task_display(task), "execute inference model");
    }
}
