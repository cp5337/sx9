//! Core FORGE Mission Tasks
//!
//! V-N-N pattern (subject Forge implicit)
//! Verb - Object - Context

use serde::{Serialize, Deserialize};

/// Task identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskId {
    // === SESSION ===
    StartSession,
    EndSession,
    CheckpointSession,

    // === MEMORY ===
    ReadKnowledge,
    WriteKnowledge,
    ReadProgress,
    WriteProgress,
    ReadFocus,
    WriteFocus,

    // === CONTEXT ===
    GatherContext,
    SummarizeContext,
    PruneContext,

    // === LINEAR ===
    FetchIssue,
    UpdateIssue,
    CreateIssue,
    CloseIssue,

    // === GIT ===
    ReadStatus,
    CreateBranch,
    StageChanges,
    CreateCommit,
    CreateCheckpoint,
    PushBranch,

    // === CODE ===
    ReadFile,
    WriteFile,
    EditFile,
    DeleteFile,
    GenerateCode,
    RefactorCode,

    // === QA ===
    RunTests,
    RunStaticGate,
    RunArchGate,
    RunPatternGate,
    GenerateReport,

    // === NATS ===
    PublishEvent,
    SubscribeTopic,
    RequestReply,

    // === AGENT ===
    DispatchTask,
    ReceiveResult,
    HandoffTask,
}

/// Task definition with V-N-N components
#[derive(Debug, Clone)]
pub struct Task {
    pub id: TaskId,
    pub verb: &'static str,
    pub object: &'static str,
    pub context: &'static str,
    pub inputs: &'static [&'static str],
    pub outputs: &'static [&'static str],
    pub requires: &'static [TaskId],
}

/// Core task registry
pub const TASKS: &[Task] = &[
    // === SESSION ===
    Task {
        id: TaskId::StartSession,
        verb: "start",
        object: "session",
        context: "workspace",
        inputs: &["workspace_path"],
        outputs: &["session_id"],
        requires: &[],
    },
    Task {
        id: TaskId::EndSession,
        verb: "end",
        object: "session",
        context: "workspace",
        inputs: &["session_id"],
        outputs: &["session_summary"],
        requires: &[TaskId::WriteProgress],
    },
    Task {
        id: TaskId::CheckpointSession,
        verb: "checkpoint",
        object: "session",
        context: "milestone",
        inputs: &["session_id", "milestone_name"],
        outputs: &["checkpoint_id"],
        requires: &[TaskId::CreateCommit],
    },

    // === MEMORY ===
    Task {
        id: TaskId::ReadKnowledge,
        verb: "read",
        object: "knowledge",
        context: "memory",
        inputs: &["memory_path"],
        outputs: &["knowledge_json"],
        requires: &[TaskId::StartSession],
    },
    Task {
        id: TaskId::WriteKnowledge,
        verb: "write",
        object: "knowledge",
        context: "memory",
        inputs: &["knowledge_json"],
        outputs: &["write_success"],
        requires: &[],
    },
    Task {
        id: TaskId::ReadProgress,
        verb: "read",
        object: "progress",
        context: "session",
        inputs: &["session_id"],
        outputs: &["progress_text"],
        requires: &[TaskId::StartSession],
    },
    Task {
        id: TaskId::WriteProgress,
        verb: "write",
        object: "progress",
        context: "session",
        inputs: &["session_id", "progress_entry"],
        outputs: &["write_success"],
        requires: &[],
    },
    Task {
        id: TaskId::ReadFocus,
        verb: "read",
        object: "focus",
        context: "session",
        inputs: &["session_id"],
        outputs: &["focus_json"],
        requires: &[TaskId::StartSession],
    },
    Task {
        id: TaskId::WriteFocus,
        verb: "write",
        object: "focus",
        context: "session",
        inputs: &["session_id", "focus_json"],
        outputs: &["write_success"],
        requires: &[],
    },

    // === CONTEXT ===
    Task {
        id: TaskId::GatherContext,
        verb: "gather",
        object: "context",
        context: "codebase",
        inputs: &["file_patterns", "query"],
        outputs: &["context_bundle"],
        requires: &[TaskId::ReadFocus],
    },
    Task {
        id: TaskId::SummarizeContext,
        verb: "summarize",
        object: "context",
        context: "session",
        inputs: &["context_bundle"],
        outputs: &["context_summary"],
        requires: &[TaskId::GatherContext],
    },
    Task {
        id: TaskId::PruneContext,
        verb: "prune",
        object: "context",
        context: "token_limit",
        inputs: &["context_bundle", "max_tokens"],
        outputs: &["pruned_context"],
        requires: &[TaskId::GatherContext],
    },

    // === LINEAR ===
    Task {
        id: TaskId::FetchIssue,
        verb: "fetch",
        object: "issue",
        context: "linear",
        inputs: &["issue_id"],
        outputs: &["issue_json"],
        requires: &[],
    },
    Task {
        id: TaskId::UpdateIssue,
        verb: "update",
        object: "issue",
        context: "linear",
        inputs: &["issue_id", "update_payload"],
        outputs: &["update_success"],
        requires: &[],
    },
    Task {
        id: TaskId::CreateIssue,
        verb: "create",
        object: "issue",
        context: "linear",
        inputs: &["issue_payload"],
        outputs: &["issue_id"],
        requires: &[],
    },
    Task {
        id: TaskId::CloseIssue,
        verb: "close",
        object: "issue",
        context: "linear",
        inputs: &["issue_id", "resolution"],
        outputs: &["close_success"],
        requires: &[],
    },

    // === GIT ===
    Task {
        id: TaskId::ReadStatus,
        verb: "read",
        object: "status",
        context: "git",
        inputs: &["repo_path"],
        outputs: &["git_status"],
        requires: &[],
    },
    Task {
        id: TaskId::CreateBranch,
        verb: "create",
        object: "branch",
        context: "git",
        inputs: &["branch_name", "base_ref"],
        outputs: &["branch_ref"],
        requires: &[TaskId::ReadStatus],
    },
    Task {
        id: TaskId::StageChanges,
        verb: "stage",
        object: "changes",
        context: "git",
        inputs: &["file_paths"],
        outputs: &["staged_files"],
        requires: &[],
    },
    Task {
        id: TaskId::CreateCommit,
        verb: "create",
        object: "commit",
        context: "git",
        inputs: &["message", "staged_files"],
        outputs: &["commit_sha"],
        requires: &[TaskId::StageChanges],
    },
    Task {
        id: TaskId::CreateCheckpoint,
        verb: "create",
        object: "checkpoint",
        context: "git",
        inputs: &["commit_sha", "checkpoint_name"],
        outputs: &["checkpoint_tag"],
        requires: &[TaskId::CreateCommit],
    },
    Task {
        id: TaskId::PushBranch,
        verb: "push",
        object: "branch",
        context: "remote",
        inputs: &["branch_ref", "remote_name"],
        outputs: &["push_success"],
        requires: &[TaskId::CreateCommit],
    },

    // === CODE ===
    Task {
        id: TaskId::ReadFile,
        verb: "read",
        object: "file",
        context: "filesystem",
        inputs: &["file_path"],
        outputs: &["file_content"],
        requires: &[],
    },
    Task {
        id: TaskId::WriteFile,
        verb: "write",
        object: "file",
        context: "filesystem",
        inputs: &["file_path", "content"],
        outputs: &["write_success"],
        requires: &[],
    },
    Task {
        id: TaskId::EditFile,
        verb: "edit",
        object: "file",
        context: "filesystem",
        inputs: &["file_path", "edits"],
        outputs: &["edit_success"],
        requires: &[TaskId::ReadFile],
    },
    Task {
        id: TaskId::DeleteFile,
        verb: "delete",
        object: "file",
        context: "filesystem",
        inputs: &["file_path"],
        outputs: &["delete_success"],
        requires: &[],
    },
    Task {
        id: TaskId::GenerateCode,
        verb: "generate",
        object: "code",
        context: "specification",
        inputs: &["spec", "context_bundle"],
        outputs: &["generated_files"],
        requires: &[TaskId::GatherContext],
    },
    Task {
        id: TaskId::RefactorCode,
        verb: "refactor",
        object: "code",
        context: "directive",
        inputs: &["file_path", "directive"],
        outputs: &["refactored_content"],
        requires: &[TaskId::ReadFile],
    },

    // === QA ===
    Task {
        id: TaskId::RunTests,
        verb: "run",
        object: "tests",
        context: "crate",
        inputs: &["crate_path", "test_filter"],
        outputs: &["test_results"],
        requires: &[],
    },
    Task {
        id: TaskId::RunStaticGate,
        verb: "run",
        object: "static_gate",
        context: "crate",
        inputs: &["crate_path"],
        outputs: &["static_report"],
        requires: &[],
    },
    Task {
        id: TaskId::RunArchGate,
        verb: "run",
        object: "arch_gate",
        context: "crate",
        inputs: &["crate_path"],
        outputs: &["arch_report"],
        requires: &[],
    },
    Task {
        id: TaskId::RunPatternGate,
        verb: "run",
        object: "pattern_gate",
        context: "crate",
        inputs: &["crate_path"],
        outputs: &["pattern_report"],
        requires: &[],
    },
    Task {
        id: TaskId::GenerateReport,
        verb: "generate",
        object: "report",
        context: "qa",
        inputs: &["static_report", "arch_report", "pattern_report"],
        outputs: &["qa_report"],
        requires: &[TaskId::RunStaticGate, TaskId::RunArchGate, TaskId::RunPatternGate],
    },

    // === NATS ===
    Task {
        id: TaskId::PublishEvent,
        verb: "publish",
        object: "event",
        context: "nats",
        inputs: &["subject", "payload"],
        outputs: &["publish_ack"],
        requires: &[],
    },
    Task {
        id: TaskId::SubscribeTopic,
        verb: "subscribe",
        object: "topic",
        context: "nats",
        inputs: &["subject_pattern"],
        outputs: &["subscription_id"],
        requires: &[],
    },
    Task {
        id: TaskId::RequestReply,
        verb: "request",
        object: "reply",
        context: "nats",
        inputs: &["subject", "request_payload"],
        outputs: &["response_payload"],
        requires: &[],
    },

    // === AGENT ===
    Task {
        id: TaskId::DispatchTask,
        verb: "dispatch",
        object: "task",
        context: "agent",
        inputs: &["agent_id", "task_payload"],
        outputs: &["dispatch_id"],
        requires: &[],
    },
    Task {
        id: TaskId::ReceiveResult,
        verb: "receive",
        object: "result",
        context: "agent",
        inputs: &["dispatch_id"],
        outputs: &["task_result"],
        requires: &[TaskId::DispatchTask],
    },
    Task {
        id: TaskId::HandoffTask,
        verb: "handoff",
        object: "task",
        context: "agent",
        inputs: &["task_id", "target_agent"],
        outputs: &["handoff_id"],
        requires: &[],
    },
];

/// Get task by ID
pub fn get_task(id: TaskId) -> Option<&'static Task> {
    TASKS.iter().find(|t| t.id == id)
}

/// Get all tasks that depend on a given task
pub fn get_dependents(id: TaskId) -> Vec<&'static Task> {
    TASKS.iter().filter(|t| t.requires.contains(&id)).collect()
}

/// Get task display string (V-N-N format)
pub fn task_display(task: &Task) -> String {
    format!("{} {} {}", task.verb, task.object, task.context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_count() {
        assert_eq!(TASKS.len(), 39);
    }

    #[test]
    fn test_get_task() {
        let task = get_task(TaskId::StartSession).unwrap();
        assert_eq!(task.verb, "start");
        assert_eq!(task.object, "session");
    }

    #[test]
    fn test_no_circular_deps() {
        for task in TASKS {
            assert!(!task.requires.contains(&task.id), "Task {:?} has circular dep", task.id);
        }
    }

    #[test]
    fn test_task_display() {
        let task = get_task(TaskId::GenerateCode).unwrap();
        assert_eq!(task_display(task), "generate code specification");
    }
}
