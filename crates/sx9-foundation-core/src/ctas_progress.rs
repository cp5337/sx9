//! CTAS Progress Indicator Module
//!
//! Provides branded, user-friendly progress indicators (steps, spinners, tables)
//! for all CTAS tools and QA modules. Uses `indicatif` for terminal output.
//! Emits structured events for future TTS/logging integration.

use crate::meta_enrollable::{MetaEnrollable, MetaEnrollment};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc};
use std::time::Duration;

#[derive(Clone, Debug)]
pub enum ProgressStatus {
    Start,
    Update,
    Success,
    Error,
    Finish,
}

#[derive(Clone, Debug)]
pub struct ProgressEvent {
    pub step: usize,
    pub total_steps: usize,
    pub name: String,
    pub status: ProgressStatus,
    pub message: String,
    pub module: String,
}

pub type ProgressCallback = Arc<dyn Fn(&ProgressEvent) + Send + Sync>;

/// Step-based progress indicator with branding and event/callback support
pub struct ProgressStep {
    pb: Arc<ProgressBar>,
    step_name: String,
    step: usize,
    total_steps: usize,
    module: String,
    callback: Option<ProgressCallback>,
}

impl ProgressStep {
    pub fn with_steps(module: &str, step_name: &str, total_steps: usize) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]));
        pb.enable_steady_tick(Duration::from_millis(80));
        pb.set_message(format!("[CTAS:{}] {}: starting...", module, step_name));
        Self {
            pb: Arc::new(pb),
            step_name: step_name.to_string(),
            step: 0,
            total_steps,
            module: module.to_string(),
            callback: None,
        }
    }
    pub fn set_callback(&mut self, cb: ProgressCallback) {
        self.callback = Some(cb);
    }
    fn emit(&self, status: ProgressStatus, msg: &str) {
        if let Some(cb) = &self.callback {
            let event = ProgressEvent {
                step: self.step,
                total_steps: self.total_steps,
                name: self.step_name.clone(),
                status,
                message: msg.to_string(),
                module: self.module.clone(),
            };
            cb(&event);
        }
    }
    pub fn start_step(&mut self, step: usize, msg: &str) {
        self.step = step;
        let m = format!("[CTAS:{}] 🔄 Step {}/{}: {} - {}", self.module, step, self.total_steps, self.step_name, msg);
        self.pb.set_message(&m);
        self.emit(ProgressStatus::Start, msg);
    }
    pub fn update(&self, msg: &str) {
        let m = format!("[CTAS:{}] 🔄 Step {}/{}: {} - {}", self.module, self.step, self.total_steps, self.step_name, msg);
        self.pb.set_message(&m);
        self.emit(ProgressStatus::Update, msg);
    }
    pub fn success(&self, msg: &str) {
        let m = format!("[CTAS:{}] ✅ Step {}/{}: {} - {}", self.module, self.step, self.total_steps, self.step_name, msg);
        self.pb.finish_with_message(&m);
        self.emit(ProgressStatus::Success, msg);
    }
    pub fn error(&self, msg: &str) {
        let m = format!("[CTAS:{}] ❌ Step {}/{}: {} - {}", self.module, self.step, self.total_steps, self.step_name, msg);
        self.pb.abandon_with_message(&m);
        self.emit(ProgressStatus::Error, msg);
    }
    pub fn finish(&self, msg: &str) {
        let m = format!("[CTAS:{}] {}: {}", self.module, self.step_name, msg);
        self.pb.finish_with_message(&m);
        self.emit(ProgressStatus::Finish, msg);
    }
    pub fn initial_rx_tx_messages() -> (String, String) {
        (
            "[CTAS:Progress] RX: Progress indicator initialized and ready to receive events.".to_string(),
            "[CTAS:Progress] TX: Progress indicator ready to transmit status updates and meta events.".to_string()
        )
    }
}

impl MetaEnrollable for ProgressStep {
    fn enroll_meta(&self) -> MetaEnrollment {
        MetaEnrollment {
            traits: vec!["ProgressIndicator".to_string(), "MetaStatusEmitter".to_string()],
            action_codes: vec!["progress_start".to_string(), "progress_update".to_string(), "progress_success".to_string(), "progress_error".to_string(), "progress_finish".to_string()],
            xsd_path: "xsd/progress_indicator.xsd".to_string(),
        }
    }
}

/// Indeterminate spinner for async tasks (with branding and event support)
pub struct ProgressSpinner {
    pb: Arc<ProgressBar>,
    module: String,
    callback: Option<ProgressCallback>,
}

impl ProgressSpinner {
    pub fn start(module: &str, msg: &str) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .unwrap());
        pb.enable_steady_tick(Duration::from_millis(80));
        pb.set_message(format!("[CTAS:{}] {}", module, msg));
        Self { pb: Arc::new(pb), module: module.to_string(), callback: None }
    }
    pub fn set_callback(&mut self, cb: ProgressCallback) {
        self.callback = Some(cb);
    }
    fn emit(&self, status: ProgressStatus, msg: &str) {
        if let Some(cb) = &self.callback {
            let event = ProgressEvent {
                step: 0,
                total_steps: 0,
                name: "Spinner".to_string(),
                status,
                message: msg.to_string(),
                module: self.module.clone(),
            };
            cb(&event);
        }
    }
    pub fn update(&self, msg: &str) {
        self.pb.set_message(format!("[CTAS:{}] {}", self.module, msg));
        self.emit(ProgressStatus::Update, msg);
    }
    pub fn success(&self, msg: &str) {
        self.pb.finish_with_message(format!("[CTAS:{}] ✅ {}", self.module, msg));
        self.emit(ProgressStatus::Success, msg);
    }
    pub fn error(&self, msg: &str) {
        self.pb.abandon_with_message(format!("[CTAS:{}] ❌ {}", self.module, msg));
        self.emit(ProgressStatus::Error, msg);
    }
    pub fn finish(&self, msg: &str) {
        self.pb.finish_with_message(format!("[CTAS:{}] {}", self.module, msg));
        self.emit(ProgressStatus::Finish, msg);
    }
    pub fn initial_rx_tx_messages() -> (String, String) {
        (
            "[CTAS:Spinner] RX: Spinner initialized and ready to receive events.".to_string(),
            "[CTAS:Spinner] TX: Spinner ready to transmit status updates and meta events.".to_string()
        )
    }
}

impl MetaEnrollable for ProgressSpinner {
    fn enroll_meta(&self) -> MetaEnrollment {
        MetaEnrollment {
            traits: vec!["ProgressSpinner".to_string(), "MetaStatusEmitter".to_string()],
            action_codes: vec!["spinner_update".to_string(), "spinner_success".to_string(), "spinner_error".to_string(), "spinner_finish".to_string()],
            xsd_path: "xsd/progress_indicator.xsd".to_string(),
        }
    }
}

/// Render a branded Markdown QA status table with color-coded status and total file count
pub fn render_qa_status_markdown(module: &str, results: &[crate::levels::QAResult], total_files: usize) -> String {
    fn status_color(status: &crate::levels::QAStatus) -> &'static str {
        match status {
            crate::levels::QAStatus::Success => "<span style=\"color:green\"><b>✅ Success</b></span>",
            crate::levels::QAStatus::Warning => "<span style=\"color:orange\"><b>⚠️ Warning</b></span>",
            crate::levels::QAStatus::Error => "<span style=\"color:red\"><b>❌ Error</b></span>",
            crate::levels::QAStatus::Critical => "<span style=\"color:darkred\"><b>🛑 Critical</b></span>",
            crate::levels::QAStatus::Skipped => "<span style=\"color:gray\"><b>⏭️ Skipped</b></span>",
        }
    }
    let mut md = String::new();
    md.push_str(&format!("<span style=\"color:#0074D9\"><b>[CTAS {}]</b></span>  ", module));
    md.push_str(&format!("\n_Total files analyzed: **{}**_\n\n", total_files));
    md.push_str("| QA Level | Status | Score | Issues | Critical | Errors | Warnings |\n");
    md.push_str("|----------|--------|-------|--------|----------|--------|----------|\n");
    for result in results {
        let (crit, err, warn) = result.issues.iter().fold((0,0,0), |(c,e,w), issue| match issue.severity {
            crate::levels::IssueSeverity::Critical => (c+1,e,w),
            crate::levels::IssueSeverity::Error => (c,e+1,w),
            crate::levels::IssueSeverity::Warning => (c,e,w+1),
            _ => (c,e,w),
        });
        let status = status_color(&result.status);
        md.push_str(&format!(
            "| {} | {} | {:.1} | {} | {} | {} | {} |\n",
            result.level.name(),
            status,
            result.score,
            result.issues.len(),
            crit, err, warn
        ));
    }
    md
}

/// Policy: Show table at the end of a major run or on user request
pub fn should_show_table(at_end_of_run: bool, user_requested: bool) -> bool {
    at_end_of_run || user_requested
}

// Usage: import and use ProgressStep or ProgressSpinner in any CTAS module for consistent, branded progress reporting and future TTS/logging integration.
