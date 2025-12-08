//! Unicode Playbook Executor
//!
//! Executes Unicode playbooks through Neural Mux with 7-tier escalation support.
//! Maintains phase sequencing and dependencies.

use crate::dsl::playbook_unicode::*;
use crate::dsl::unicode_bridge::UnicodeEmitter;
use crate::dsl::{DSLError, DSLResult};
use std::collections::{HashMap, HashSet, VecDeque};

/// Playbook execution status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

/// Playbook executor
pub struct PlaybookExecutor {
    emitter: UnicodeEmitter,
    /// Step execution status
    step_status: HashMap<String, ExecutionStatus>,
    /// Step execution order (topological sort)
    execution_order: Vec<String>,
}

impl PlaybookExecutor {
    /// Create new executor
    pub fn new() -> Self {
        Self {
            emitter: UnicodeEmitter::new(),
            step_status: HashMap::new(),
            execution_order: Vec::new(),
        }
    }
    
    /// Execute playbook
    pub async fn execute(&mut self, playbook: &UnicodePlaybook) -> DSLResult<HashMap<String, ExecutionStatus>> {
        // Validate playbook
        playbook.validate()?;
        
        // Initialize step statuses
        for step in &playbook.steps {
            self.step_status.insert(step.name.clone(), ExecutionStatus::Pending);
        }
        
        // Topological sort to determine execution order
        self.execution_order = self.topological_sort(playbook)?;
        
        // Execute steps in order
        for step_name in &self.execution_order {
            let step = playbook.steps.iter()
                .find(|s| s.name == *step_name)
                .ok_or_else(|| DSLError::ExecutionFailed(format!("Step not found: {}", step_name)))?;
            
            // Check dependencies
            if !self.check_dependencies(step, playbook)? {
                self.step_status.insert(step_name.clone(), ExecutionStatus::Skipped);
                continue;
            }
            
            // Execute step
            self.step_status.insert(step_name.clone(), ExecutionStatus::Running);
            
            match self.execute_step(step).await {
                Ok(_) => {
                    self.step_status.insert(step_name.clone(), ExecutionStatus::Completed);
                }
                Err(e) => {
                    self.step_status.insert(step_name.clone(), ExecutionStatus::Failed);
                    return Err(e);
                }
            }
        }
        
        Ok(self.step_status.clone())
    }
    
    /// Execute a single step
    async fn execute_step(&self, step: &UnicodePlaybookStep) -> DSLResult<()> {
        // Emit Unicode operation for Neural Mux routing
        let unicode_ops = vec![step.unicode_op];
        
        // TODO: Route through Neural Mux
        // For now, just validate the operation
        if (step.unicode_op as u32) < 0xE000 || (step.unicode_op as u32) > 0xE9FF {
            return Err(DSLError::InvalidParameters(
                format!("Invalid Unicode operation: U+{:04X}", step.unicode_op as u32)
            ));
        }
        
        // TODO: Execute based on tier
        match step.tier {
            EscalationTier::Wasm => {
                // Execute WASM microkernel
            }
            EscalationTier::Microkernel => {
                // Execute microkernel
            }
            EscalationTier::KernelCrate => {
                // Execute kernel crate
            }
            EscalationTier::MultiCrates => {
                // Execute multi-crates
            }
            EscalationTier::Containers => {
                // Execute containers
            }
            EscalationTier::Firefly => {
                // Execute Firefly
            }
            EscalationTier::Orb => {
                // Execute Orb
            }
        }
        
        Ok(())
    }
    
    /// Check if step dependencies are satisfied
    fn check_dependencies(&self, step: &UnicodePlaybookStep, playbook: &UnicodePlaybook) -> DSLResult<bool> {
        for dep_name in &step.depends_on {
            let status = self.step_status.get(dep_name)
                .ok_or_else(|| DSLError::ExecutionFailed(format!("Dependency status not found: {}", dep_name)))?;
            
            match status {
                ExecutionStatus::Completed => continue,
                ExecutionStatus::Failed => {
                    return Err(DSLError::ExecutionFailed(format!("Dependency failed: {}", dep_name)));
                }
                _ => return Ok(false), // Dependency not ready
            }
        }
        Ok(true)
    }
    
    /// Topological sort of steps based on dependencies
    fn topological_sort(&self, playbook: &UnicodePlaybook) -> DSLResult<Vec<String>> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        
        // Initialize in-degree and graph
        for step in &playbook.steps {
            in_degree.insert(step.name.clone(), 0);
            graph.insert(step.name.clone(), Vec::new());
        }
        
        // Build graph and calculate in-degrees
        for step in &playbook.steps {
            for dep in &step.depends_on {
                graph.get_mut(dep)
                    .ok_or_else(|| DSLError::ExecutionFailed(format!("Dependency not found: {}", dep)))?
                    .push(step.name.clone());
                
                *in_degree.get_mut(&step.name)
                    .ok_or_else(|| DSLError::ExecutionFailed(format!("Step not found in in-degree: {}", step.name)))? += 1;
            }
        }
        
        // Kahn's algorithm
        let mut queue: VecDeque<String> = VecDeque::new();
        for (step_name, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(step_name.clone());
            }
        }
        
        let mut result = Vec::new();
        while let Some(step_name) = queue.pop_front() {
            result.push(step_name.clone());
            
            if let Some(dependents) = graph.get(&step_name) {
                for dependent in dependents {
                    let degree = in_degree.get_mut(dependent)
                        .ok_or_else(|| DSLError::ExecutionFailed(format!("Dependent not found: {}", dependent)))?;
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(dependent.clone());
                    }
                }
            }
        }
        
        // Check for cycles
        if result.len() != playbook.steps.len() {
            return Err(DSLError::ExecutionFailed("Circular dependency detected".to_string()));
        }
        
        Ok(result)
    }
    
    /// Get execution status for a step
    pub fn get_step_status(&self, step_name: &str) -> Option<&ExecutionStatus> {
        self.step_status.get(step_name)
    }
    
    /// Get all step statuses
    pub fn get_all_statuses(&self) -> &HashMap<String, ExecutionStatus> {
        &self.step_status
    }
}

impl Default for PlaybookExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_topological_sort() {
        let executor = PlaybookExecutor::new();
        
        let mut playbook = UnicodePlaybook::new("test".to_string(), "1.0".to_string());
        
        let step1 = UnicodePlaybookStep {
            name: "step1".to_string(),
            tier: EscalationTier::Wasm,
            unicode_op: '\u{E900}',
            tool: None,
            target: None,
            depends_on: Vec::new(),
            metadata: HashMap::new(),
        };
        
        let step2 = UnicodePlaybookStep {
            name: "step2".to_string(),
            tier: EscalationTier::Microkernel,
            unicode_op: '\u{E901}',
            tool: None,
            target: None,
            depends_on: vec!["step1".to_string()],
            metadata: HashMap::new(),
        };
        
        playbook.add_step(step1);
        playbook.add_step(step2);
        
        let order = executor.topological_sort(&playbook).unwrap();
        assert_eq!(order[0], "step1");
        assert_eq!(order[1], "step2");
    }
}

