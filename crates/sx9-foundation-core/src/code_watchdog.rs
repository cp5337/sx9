//! Code Watchdog - Monitors runaway code and code size changes
//!
//! Tiny watchdog system to detect code bloat, runaway functions,
//! and track changes in code size across the foundation system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Code Watchdog system
#[derive(Debug, Clone)]
pub struct CodeWatchdog {
    pub function_monitors: HashMap<String, FunctionMonitor>,
    pub file_size_monitors: HashMap<String, FileSizeMonitor>,
    pub alert_thresholds: WatchdogThresholds,
    pub active: bool,
}

/// Monitor for individual function execution
#[derive(Debug, Clone)]
pub struct FunctionMonitor {
    pub function_name: String,
    pub start_time: Option<Instant>,
    pub execution_count: u64,
    pub total_duration: Duration,
    pub max_duration: Duration,
    pub runaway_alerts: u32,
}

/// Monitor for file size changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSizeMonitor {
    pub file_path: String,
    pub initial_size_bytes: u64,
    pub current_size_bytes: u64,
    pub size_growth_percent: f64,
    pub lines_added: u32,
    pub last_check: Option<std::time::SystemTime>,
}

/// Watchdog alert thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchdogThresholds {
    pub max_function_duration_ms: u64,
    pub max_file_size_growth_percent: f64,
    pub max_lines_per_function: u32,
    pub alert_after_n_runaways: u32,
}

impl CodeWatchdog {
    pub fn new() -> Self {
        Self {
            function_monitors: HashMap::new(),
            file_size_monitors: HashMap::new(),
            alert_thresholds: WatchdogThresholds::default(),
            active: true,
        }
    }

    /// Start monitoring a function
    pub fn start_function_monitor(&mut self, function_name: &str) {
        if !self.active { return; }

        let monitor = self.function_monitors.entry(function_name.to_string())
            .or_insert_with(|| FunctionMonitor::new(function_name));

        monitor.start_time = Some(Instant::now());
        monitor.execution_count += 1;
    }

    /// Stop monitoring a function and check for runaway
    pub fn stop_function_monitor(&mut self, function_name: &str) -> Option<Duration> {
        if !self.active { return None; }

        if let Some(monitor) = self.function_monitors.get_mut(function_name) {
            if let Some(start_time) = monitor.start_time.take() {
                let duration = start_time.elapsed();
                monitor.total_duration += duration;

                if duration > monitor.max_duration {
                    monitor.max_duration = duration;
                }

                // Check for runaway
                if duration.as_millis() > self.alert_thresholds.max_function_duration_ms as u128 {
                    monitor.runaway_alerts += 1;
                    println!("🚨 WATCHDOG: Runaway function detected: {} ({}ms)",
                             function_name, duration.as_millis());

                    if monitor.runaway_alerts >= self.alert_thresholds.alert_after_n_runaways {
                        println!("⚠️ WATCHDOG: Function {} has {} runaway alerts!",
                                 function_name, monitor.runaway_alerts);
                    }
                }

                return Some(duration);
            }
        }
        None
    }

    /// Monitor file size changes
    pub fn monitor_file_size(&mut self, file_path: &str, current_size: u64) {
        if !self.active { return; }

        let monitor = self.file_size_monitors.entry(file_path.to_string())
            .or_insert_with(|| FileSizeMonitor::new(file_path, current_size));

        if monitor.initial_size_bytes == 0 {
            monitor.initial_size_bytes = current_size;
        }

        monitor.current_size_bytes = current_size;
        monitor.last_check = Some(std::time::SystemTime::now());

        // Calculate growth
        if monitor.initial_size_bytes > 0 {
            monitor.size_growth_percent =
                ((current_size as f64 - monitor.initial_size_bytes as f64) /
                 monitor.initial_size_bytes as f64) * 100.0;

            // Alert on excessive growth
            if monitor.size_growth_percent > self.alert_thresholds.max_file_size_growth_percent {
                println!("📈 WATCHDOG: File size growth alert: {} (+{:.1}%)",
                         file_path, monitor.size_growth_percent);
            }
        }
    }

    /// Get watchdog status report
    pub fn get_status_report(&self) -> String {
        let mut report = String::from("🐕 Code Watchdog Status Report:\n");

        report.push_str(&format!("Active: {}\n", if self.active { "YES" } else { "NO" }));
        report.push_str(&format!("Functions Monitored: {}\n", self.function_monitors.len()));
        report.push_str(&format!("Files Monitored: {}\n", self.file_size_monitors.len()));

        // Runaway function alerts
        let runaway_functions: Vec<_> = self.function_monitors.iter()
            .filter(|(_, m)| m.runaway_alerts > 0)
            .collect();

        if !runaway_functions.is_empty() {
            report.push_str(&format!("Runaway Functions: {}\n", runaway_functions.len()));
            for (name, monitor) in runaway_functions {
                report.push_str(&format!("  {} - {} alerts (max: {}ms)\n",
                                       name, monitor.runaway_alerts, monitor.max_duration.as_millis()));
            }
        }

        // File size alerts
        let growing_files: Vec<_> = self.file_size_monitors.iter()
            .filter(|(_, m)| m.size_growth_percent > 10.0)
            .collect();

        if !growing_files.is_empty() {
            report.push_str(&format!("Growing Files: {}\n", growing_files.len()));
            for (path, monitor) in growing_files {
                report.push_str(&format!("  {} - +{:.1}%\n",
                                       path, monitor.size_growth_percent));
            }
        }

        report
    }
}

impl FunctionMonitor {
    fn new(function_name: &str) -> Self {
        Self {
            function_name: function_name.to_string(),
            start_time: None,
            execution_count: 0,
            total_duration: Duration::new(0, 0),
            max_duration: Duration::new(0, 0),
            runaway_alerts: 0,
        }
    }
}

impl FileSizeMonitor {
    fn new(file_path: &str, initial_size: u64) -> Self {
        Self {
            file_path: file_path.to_string(),
            initial_size_bytes: initial_size,
            current_size_bytes: initial_size,
            size_growth_percent: 0.0,
            lines_added: 0,
            last_check: None,
        }
    }
}

impl WatchdogThresholds {
    fn default() -> Self {
        Self {
            max_function_duration_ms: 5000,    // 5 seconds
            max_file_size_growth_percent: 50.0, // 50% growth
            max_lines_per_function: 100,        // 100 lines
            alert_after_n_runaways: 3,          // 3 runaway alerts
        }
    }
}

impl Default for CodeWatchdog {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro for easy function monitoring
#[macro_export]
macro_rules! watch_function {
    ($watchdog:expr, $func_name:expr, $code:block) => {
        {
            $watchdog.start_function_monitor($func_name);
            let result = $code;
            $watchdog.stop_function_monitor($func_name);
            result
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_watchdog_creation() {
        let watchdog = CodeWatchdog::new();
        assert!(watchdog.active);
        assert_eq!(watchdog.function_monitors.len(), 0);
    }

    #[test]
    fn test_function_monitoring() {
        let mut watchdog = CodeWatchdog::new();

        watchdog.start_function_monitor("test_function");
        thread::sleep(Duration::from_millis(10));
        let duration = watchdog.stop_function_monitor("test_function");

        assert!(duration.is_some());
        assert!(duration.unwrap().as_millis() >= 10);
    }

    #[test]
    fn test_file_size_monitoring() {
        let mut watchdog = CodeWatchdog::new();

        watchdog.monitor_file_size("test.rs", 1000);
        watchdog.monitor_file_size("test.rs", 1500);

        let monitor = watchdog.file_size_monitors.get("test.rs").unwrap();
        assert_eq!(monitor.size_growth_percent, 50.0);
    }
}