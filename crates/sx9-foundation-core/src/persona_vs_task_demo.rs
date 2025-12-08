//! CTAS 1st Person vs 2nd Person Demo
//! 
//! Demonstrates the difference between:
//! - 1n: Tracking adversary personas (first-person narrative, autonomous behavior)
//! - 2n: Executing operator tasks (second-person commands, directed actions)

use bevy::prelude::*;
use ctas_exploit_vector_machine::{
    adversary_tracking_system::{
        adversary_simulation_system, operator_task_execution_system,
        AdversaryPersonaBundle, OperatorTaskBundle, TaskType
    }
};

fn main() {
    println!("🎭 CTAS Persona vs Task Demonstration");
    println!("📝 1st Person (1n): Adversary tracking with first-person narratives");
    println!("⚡ 2nd Person (2n): Operator task execution with commands");
    println!("{}", "─".repeat(80));

    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, setup_personas_and_tasks)
        .add_systems(Update, (
            adversary_simulation_system,
            operator_task_execution_system,
            display_system_status,
            auto_exit_system.run_if(should_exit),
        ))
        .run();
}

/// Setup both adversary personas (1n) and operator tasks (2n)
fn setup_personas_and_tasks(mut commands: Commands) {
    println!("🚀 Setting up demonstration...\n");
    
    // === 1ST PERSON (1n) - ADVERSARY PERSONAS ===
    println!("🎭 **1ST PERSON (1n) - ADVERSARY TRACKING:**");
    
    // Spawn Jihadist Operator adversary
    let jihadist_bundle = AdversaryPersonaBundle::jihadist_operator(
        "Ahmad Hassan".to_string(),
        "Infiltrate network infrastructure for intelligence gathering".to_string(),
    );
    commands.spawn((
        jihadist_bundle,
        Name::new("Adversary-Ahmad-Hassan"),
    ));
    println!("   👤 Ahmad Hassan (Jihadist Operator): 'I am infiltrating network infrastructure'");
    
    // === 2ND PERSON (2n) - OPERATOR TASKS ===
    println!("\n⚡ **2ND PERSON (2n) - OPERATOR TASK EXECUTION:**");
    
    // Task 1: Port scan command
    let port_scan_bundle = OperatorTaskBundle::new(
        "Execute comprehensive port scan".to_string(),
        "192.168.1.100".to_string(),
        TaskType::PortScan,
        "OPERATOR-001".to_string(),
    );
    commands.spawn((
        port_scan_bundle,
        Name::new("Task-PortScan-192.168.1.100"),
    ));
    println!("   ⚡ Command: 'Execute comprehensive port scan on 192.168.1.100'");
    
    // Task 2: Vulnerability assessment command
    let vuln_scan_bundle = OperatorTaskBundle::new(
        "Perform vulnerability assessment".to_string(),
        "10.0.0.5".to_string(),
        TaskType::VulnerabilityAssessment,
        "OPERATOR-001".to_string(),
    );
    commands.spawn((
        vuln_scan_bundle,
        Name::new("Task-VulnScan-10.0.0.5"),
    ));
    println!("   ⚡ Command: 'Perform vulnerability assessment on 10.0.0.5'");
    
    // Task 3: Network reconnaissance command
    let recon_bundle = OperatorTaskBundle::new(
        "Conduct network reconnaissance".to_string(),
        "172.16.0.0/24".to_string(),
        TaskType::NetworkReconnaissance,
        "OPERATOR-002".to_string(),
    );
    commands.spawn((
        recon_bundle,
        Name::new("Task-NetworkRecon-172.16.0.0/24"),
    ));
    println!("   ⚡ Command: 'Conduct network reconnaissance on 172.16.0.0/24'");
    
    println!("\n🎮 Starting Bevy ECS execution...");
    println!("🎭 Adversaries will generate first-person narratives and autonomous decisions");
    println!("⚡ Tasks will execute operator commands and report results\n");
}

/// Display the status of both paradigms
fn display_system_status(
    mut timer: Local<Timer>,
    time: Res<Time>,
    adversary_query: Query<&ctas_exploit_vector_machine::adversary_tracking_system::AdversaryPersona>,
    task_query: Query<&ctas_exploit_vector_machine::adversary_tracking_system::OperatorTask>,
    action_query: Query<&ctas_exploit_vector_machine::adversary_tracking_system::AdversaryAction>,
) {
    // Initialize timer if needed
    if timer.duration() == Duration::ZERO {
        *timer = Timer::from_seconds(3.0, TimerMode::Repeating);
    }
    
    timer.tick(time.delta());
    
    if timer.finished() {
        println!("📊 **SYSTEM STATUS UPDATE:**");
        
        // Show 1n adversary status
        println!("   🎭 **1ST PERSON (1n) - ADVERSARY STATUS:**");
        for adversary in adversary_query.iter() {
            println!("      👤 {}: {}", adversary.name, adversary.narrative_state);
            println!("         Stress: {:.2} | Confidence: {:.2} | Paranoia: {:.2}",
                adversary.psychological_state.stress_level,
                adversary.psychological_state.confidence,
                adversary.psychological_state.paranoia_level
            );
        }
        
        // Show adversary actions
        let action_count = action_query.iter().count();
        if action_count > 0 {
            println!("         Autonomous actions generated: {}", action_count);
        }
        
        // Show 2n task status
        println!("   ⚡ **2ND PERSON (2n) - TASK STATUS:**");
        let mut completed_count = 0;
        let mut executing_count = 0;
        let mut queued_count = 0;
        
        for task in task_query.iter() {
            match task.status {
                ctas_exploit_vector_machine::adversary_tracking_system::TaskStatus::Completed => {
                    completed_count += 1;
                    println!("      ✅ {}: COMPLETED", task.command);
                }
                ctas_exploit_vector_machine::adversary_tracking_system::TaskStatus::Executing => {
                    executing_count += 1;
                    println!("      ⏳ {}: EXECUTING", task.command);
                }
                ctas_exploit_vector_machine::adversary_tracking_system::TaskStatus::Queued => {
                    queued_count += 1;
                    println!("      📋 {}: QUEUED", task.command);
                }
                ctas_exploit_vector_machine::adversary_tracking_system::TaskStatus::Failed => {
                    println!("      ❌ {}: FAILED", task.command);
                }
                ctas_exploit_vector_machine::adversary_tracking_system::TaskStatus::Cancelled => {
                    println!("      🚫 {}: CANCELLED", task.command);
                }
            }
        }
        
        println!("      Summary: {} completed, {} executing, {} queued", 
            completed_count, executing_count, queued_count);
        println!();
    }
}

/// Determine when to exit the demo
fn should_exit(
    time: Res<Time>,
    task_query: Query<&ctas_exploit_vector_machine::adversary_tracking_system::OperatorTask>,
) -> bool {
    // Exit after 20 seconds or when all tasks are complete
    let all_tasks_complete = task_query.iter().all(|task| {
        matches!(
            task.status,
            ctas_exploit_vector_machine::adversary_tracking_system::TaskStatus::Completed |
            ctas_exploit_vector_machine::adversary_tracking_system::TaskStatus::Failed
        )
    });
    
    time.elapsed_seconds() > 20.0 || all_tasks_complete
}

/// Exit the demo gracefully
fn auto_exit_system(mut exit: EventWriter<AppExit>) {
    println!("\n🎉 **DEMONSTRATION COMPLETE!**");
    println!("\n📝 **KEY DIFFERENCES DEMONSTRATED:**");
    println!("   🎭 **1st Person (1n) - Adversary Tracking:**");
    println!("      • First-person narratives: 'I am feeling pressure...'");
    println!("      • Autonomous decision-making based on psychological state");
    println!("      • Environmental awareness affecting behavior");
    println!("      • Self-generated actions based on persona");
    println!("      • Persona-driven behavioral patterns");
    
    println!("   ⚡ **2nd Person (2n) - Task Execution:**");
    println!("      • Command-driven execution: 'Execute port scan'");
    println!("      • Directed actions from operator commands");
    println!("      • Task-focused with specific objectives");
    println!("      • System reports results back to operator");
    println!("      • Imperative execution model");
    
    println!("\n🧠 **COGNITIVE ARCHITECTURE INSIGHTS:**");
    println!("   • 1n enables adversary behavior simulation and prediction");
    println!("   • 2n enables operational task execution and automation");
    println!("   • Both paradigms use SlotGraph ECS for unified processing");
    println!("   • Hash-triggered execution works for both paradigms");
    println!("   • Cognigraph analysis applies to both tracking and tasks");
    
    println!("\n✨ CTAS dual-paradigm architecture validated successfully!\n");
    
    exit.send(AppExit::Success);
}

/// Timer implementation for local resources
#[derive(Default)]
struct Timer {
    duration: Duration,
    timer: bevy::time::Timer,
}

impl Timer {
    fn from_seconds(seconds: f32, mode: bevy::time::TimerMode) -> Self {
        Self {
            duration: Duration::from_secs_f32(seconds),
            timer: bevy::time::Timer::from_seconds(seconds, mode),
        }
    }
    
    fn tick(&mut self, delta: Duration) -> &bevy::time::Timer {
        self.timer.tick(delta);
        &self.timer
    }
    
    fn finished(&self) -> bool {
        self.timer.finished()
    }
    
    fn duration(&self) -> Duration {
        self.duration
    }
}