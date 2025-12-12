#!/usr/bin/env python3
"""
CTAS v7.3.1 Task Escalation Ladder Builder
Generates escalation ladders for all 164 tasks from Supabase
Each task gets: Script → Microkernel → Binary → Container
"""

import json
import os
from pathlib import Path
from supabase import create_client, Client

# Configuration
SUPABASE_URL = 'https://kxabqezjpglbbrjdpdmv.supabase.co'
OUTPUT_DIR = Path('/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas6-reference/task-escalation-ladders')
SCRIPTS_DIR = OUTPUT_DIR / 'scripts'
MICROKERNEL_DIR = OUTPUT_DIR / 'microkernel'
BINARIES_DIR = OUTPUT_DIR / 'binaries'
CONTAINERS_DIR = OUTPUT_DIR / 'containers'

# Load Supabase key
def get_supabase_key():
    env_file = Path('/Users/cp5337/Developer/ctas7-command-center/.env')
    if env_file.exists():
        with open(env_file, 'r') as f:
            for line in f:
                if 'SUPABASE_KEY' in line or 'SUPABASE_ANON_KEY' in line or 'REACT_APP_SUPABASE_ANON_KEY' in line:
                    key = line.split('=')[1].strip().strip('"').strip("'")
                    if key and len(key) > 20:
                        return key
    return None

# Tool to script mapping
TOOL_SCRIPTS = {
    'nmap': {
        'script': 'nmap -sn {target}',
        'description': 'Network reconnaissance and port scanning',
        'escalation': ['nmap', 'rustscan', 'masscan']
    },
    'masscan': {
        'script': 'masscan {target} -p1-65535 --rate=1000',
        'description': 'Fast port scanner',
        'escalation': ['masscan']
    },
    'rustscan': {
        'script': 'rustscan -a {target}',
        'description': 'Modern port scanner',
        'escalation': ['rustscan']
    },
    'metasploit': {
        'script': 'msfconsole -q -x "use auxiliary/scanner/portscan/tcp; set RHOSTS {target}; run"',
        'description': 'Penetration testing framework',
        'escalation': ['msfconsole']
    },
    'wireshark': {
        'script': 'tshark -i eth0 -c 100',
        'description': 'Network protocol analyzer',
        'escalation': ['tshark', 'wireshark']
    },
    'burpsuite': {
        'script': 'burpsuite --project-file={project}',
        'description': 'Web application security testing',
        'escalation': ['burpsuite']
    },
    'sqlmap': {
        'script': 'sqlmap -u {url} --batch',
        'description': 'SQL injection tool',
        'escalation': ['sqlmap']
    },
    'nikto': {
        'script': 'nikto -h {target}',
        'description': 'Web server scanner',
        'escalation': ['nikto']
    },
    'aircrack-ng': {
        'script': 'aircrack-ng -w {wordlist} {capture}',
        'description': 'Wireless security auditing',
        'escalation': ['aircrack-ng', 'airodump-ng']
    },
    'john': {
        'script': 'john --wordlist={wordlist} {hashfile}',
        'description': 'Password cracker',
        'escalation': ['john']
    },
}

def create_directories():
    """Create output directory structure"""
    for dir_path in [SCRIPTS_DIR, MICROKERNEL_DIR, BINARIES_DIR, CONTAINERS_DIR]:
        dir_path.mkdir(parents=True, exist_ok=True)
    print(f"✅ Created directory structure at {OUTPUT_DIR}")

def generate_script_level(task):
    """Generate Level 1: Shell Script"""
    task_id = task['task_id'].replace('uuid-', '').replace('-', '_')
    script_name = f"{task_id}_{task['task_name'].lower().replace(' ', '_')}.sh"
    script_path = SCRIPTS_DIR / script_name
    
    # Determine primary tool
    tools = task.get('kali_tools', [])
    primary_tool = tools[0] if tools else 'nmap'
    tool_config = TOOL_SCRIPTS.get(primary_tool, TOOL_SCRIPTS['nmap'])
    
    script_content = f"""#!/bin/bash
# CTAS Task: {task['task_name']}
# Task ID: {task['task_id']}
# Category: {task['category']}
# HD4 Phase: {task['hd4_phase']}
# Description: {task['description']}

set -e

# Configuration
TASK_ID="{task['task_id']}"
TASK_NAME="{task['task_name']}"
TARGET="${{1:-localhost}}"
LOG_FILE="/tmp/ctas_{task_id}.log"

# Logging
log() {{
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}}

log "🎯 Starting CTAS Task: $TASK_NAME"
log "📍 Target: $TARGET"

# Check if tool is available
if ! command -v {primary_tool} &> /dev/null; then
    log "❌ {primary_tool} not found. Escalating to microkernel..."
    exec ./microkernel/{task_id}_microkernel "$TARGET"
fi

# Execute primary tool
log "🔧 Executing {primary_tool}..."
{tool_config['script']}

# Check exit status
if [ $? -eq 0 ]; then
    log "✅ Task completed successfully"
else
    log "⚠️  Task failed, escalating to microkernel..."
    exec ./microkernel/{task_id}_microkernel "$TARGET"
fi
"""
    
    with open(script_path, 'w') as f:
        f.write(script_content)
    
    os.chmod(script_path, 0o755)
    return script_name

def generate_microkernel_level(task):
    """Generate Level 2: WASM Microkernel"""
    task_id = task['task_id'].replace('uuid-', '').replace('-', '_')
    microkernel_name = f"{task_id}_microkernel.wat"
    microkernel_path = MICROKERNEL_DIR / microkernel_name
    
    # WebAssembly Text format (WAT)
    wat_content = f"""(module
  ;; CTAS Task Microkernel: {task['task_name']}
  ;; Task ID: {task['task_id']}
  ;; Lightweight WASM execution for resource-constrained environments
  
  (import "env" "log" (func $log (param i32 i32)))
  (import "env" "execute_tool" (func $execute_tool (param i32) (result i32)))
  
  (memory (export "memory") 1)
  
  ;; Task metadata
  (data (i32.const 0) "{task['task_id']}")
  (data (i32.const 100) "{task['task_name']}")
  
  ;; Main execution function
  (func (export "execute") (param $target i32) (result i32)
    (local $result i32)
    
    ;; Log start
    (call $log (i32.const 100) (i32.const 50))
    
    ;; Execute tool
    (local.set $result (call $execute_tool (local.get $target)))
    
    ;; Check result
    (if (i32.eq (local.get $result) (i32.const 0))
      (then
        ;; Success
        (return (i32.const 0))
      )
      (else
        ;; Failure - escalate to binary
        (return (i32.const 1))
      )
    )
  )
  
  ;; Resource check function
  (func (export "check_resources") (result i32)
    ;; Returns 1 if sufficient resources, 0 if need to escalate
    (i32.const 1)
  )
)
"""
    
    with open(microkernel_path, 'w') as f:
        f.write(wat_content)
    
    return microkernel_name

def generate_binary_level(task):
    """Generate Level 3: Rust Binary"""
    task_id = task['task_id'].replace('uuid-', '').replace('-', '_')
    binary_name = f"{task_id}_binary"
    binary_dir = BINARIES_DIR / binary_name
    binary_dir.mkdir(exist_ok=True)
    
    # Cargo.toml
    cargo_toml = f"""[package]
name = "{binary_name}"
version = "7.3.1"
edition = "2021"

[dependencies]
tokio = {{ version = "1.41", features = ["full"] }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
log = "0.4"
env_logger = "0.11"

# CTAS Foundation
ctas7-foundation-core = {{ path = "../../../../ctas7-foundation-core" }}
"""
    
    # main.rs
    main_rs = f"""use std::process::Command;
use log::{{info, error}};

/// CTAS Task Binary: {task['task_name']}
/// Task ID: {task['task_id']}
/// Category: {task['category']}
/// HD4 Phase: {task['hd4_phase']}

#[tokio::main]
async fn main() {{
    env_logger::init();
    
    let target = std::env::args().nth(1).unwrap_or_else(|| "localhost".to_string());
    
    info!("🎯 CTAS Task: {task['task_name']}");
    info!("📍 Target: {{}}", target);
    
    match execute_task(&target).await {{
        Ok(_) => {{
            info!("✅ Task completed successfully");
            std::process::exit(0);
        }}
        Err(e) => {{
            error!("❌ Task failed: {{}}", e);
            info!("⚠️  Escalating to container...");
            escalate_to_container(&target);
        }}
    }}
}}

async fn execute_task(target: &str) -> Result<(), Box<dyn std::error::Error>> {{
    // Execute primary tool
    let output = Command::new("{task.get('kali_tools', ['nmap'])[0] if task.get('kali_tools') else 'nmap'}")
        .arg(target)
        .output()?;
    
    if output.status.success() {{
        Ok(())
    }} else {{
        Err("Tool execution failed".into())
    }}
}}

fn escalate_to_container(target: &str) {{
    info!("🐳 Starting container execution...");
    let _ = Command::new("docker")
        .args(&["run", "--rm", "ctas7/kali-tools:7.3.1", "{task['task_id']}", target])
        .spawn();
}}
"""
    
    with open(binary_dir / 'Cargo.toml', 'w') as f:
        f.write(cargo_toml)
    
    src_dir = binary_dir / 'src'
    src_dir.mkdir(exist_ok=True)
    with open(src_dir / 'main.rs', 'w') as f:
        f.write(main_rs)
    
    return binary_name

def generate_container_level(task):
    """Generate Level 4: Docker Container"""
    task_id = task['task_id'].replace('uuid-', '').replace('-', '_')
    container_name = f"{task_id}_container"
    container_dir = CONTAINERS_DIR / container_name
    container_dir.mkdir(exist_ok=True)
    
    # Dockerfile
    dockerfile = f"""# CTAS Task Container: {task['task_name']}
# Task ID: {task['task_id']}
# Full-featured execution environment

FROM kalilinux/kali-rolling:latest

# Install required tools
RUN apt-get update && apt-get install -y \\
    {' '.join(task.get('kali_tools', ['nmap'])[:5])} \\
    python3 \\
    curl \\
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy task script
COPY task.sh /app/
RUN chmod +x /app/task.sh

# Task metadata
ENV TASK_ID="{task['task_id']}"
ENV TASK_NAME="{task['task_name']}"
ENV HD4_PHASE="{task['hd4_phase']}"

ENTRYPOINT ["/app/task.sh"]
"""
    
    # Task script
    task_script = f"""#!/bin/bash
# Container execution for: {task['task_name']}

TARGET="${{1:-localhost}}"

echo "🐳 Container Execution"
echo "🎯 Task: {task['task_name']}"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
{''.join([f'echo "🔧 Running {tool}..."\n{tool} $TARGET\n' for tool in task.get('kali_tools', ['nmap'])[:3]])}

echo "✅ Container execution complete"
"""
    
    with open(container_dir / 'Dockerfile', 'w') as f:
        f.write(dockerfile)
    
    with open(container_dir / 'task.sh', 'w') as f:
        f.write(task_script)
    
    return container_name

def generate_orchestrator(tasks):
    """Generate master orchestrator script"""
    orchestrator_path = OUTPUT_DIR / 'orchestrate-task.sh'
    
    orchestrator = """#!/bin/bash
# CTAS Task Escalation Orchestrator
# Executes tasks with automatic escalation

set -e

TASK_ID="$1"
TARGET="${2:-localhost}"

if [ -z "$TASK_ID" ]; then
    echo "Usage: $0 <task_id> [target]"
    exit 1
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎯 CTAS Task Escalation Ladder"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📋 Task ID: $TASK_ID"
echo "📍 Target: $TARGET"
echo ""

# Level 1: Try script
echo "🔹 Level 1: Shell Script"
if [ -f "scripts/${TASK_ID}*.sh" ]; then
    ./scripts/${TASK_ID}*.sh "$TARGET" && exit 0
fi

# Level 2: Try microkernel
echo "🔹 Level 2: WASM Microkernel"
if [ -f "microkernel/${TASK_ID}*.wat" ]; then
    wasmtime microkernel/${TASK_ID}*.wat "$TARGET" && exit 0
fi

# Level 3: Try binary
echo "🔹 Level 3: Rust Binary"
if [ -d "binaries/${TASK_ID}*" ]; then
    cd binaries/${TASK_ID}*
    cargo run --release -- "$TARGET" && exit 0
    cd ../..
fi

# Level 4: Container (last resort)
echo "🔹 Level 4: Docker Container"
if [ -d "containers/${TASK_ID}*" ]; then
    cd containers/${TASK_ID}*
    docker build -t ctas7-task-${TASK_ID}:latest .
    docker run --rm ctas7-task-${TASK_ID}:latest "$TARGET"
fi

echo ""
echo "✅ Task execution complete!"
"""
    
    with open(orchestrator_path, 'w') as f:
        f.write(orchestrator)
    
    os.chmod(orchestrator_path, 0o755)
    print(f"✅ Created orchestrator: {orchestrator_path}")

def main():
    print('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━')
    print('🎯 CTAS v7.3.1 Task Escalation Ladder Builder')
    print('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━')
    print('')
    
    # Create directories
    create_directories()
    
    # Load Supabase key
    supabase_key = get_supabase_key()
    if not supabase_key:
        print("❌ Supabase key not found")
        return
    
    # Connect to Supabase
    print("📥 Fetching tasks from Supabase...")
    supabase: Client = create_client(SUPABASE_URL, supabase_key)
    response = supabase.table('ctas_tasks').select('*').execute()
    tasks = response.data
    print(f"✅ Loaded {len(tasks)} tasks")
    print('')
    
    # Generate escalation ladders for each task
    stats = {
        'scripts': 0,
        'microkernels': 0,
        'binaries': 0,
        'containers': 0
    }
    
    for i, task in enumerate(tasks, 1):
        print(f"[{i}/{len(tasks)}] {task['task_name']}...")
        
        # Generate all 4 levels
        generate_script_level(task)
        stats['scripts'] += 1
        
        generate_microkernel_level(task)
        stats['microkernels'] += 1
        
        generate_binary_level(task)
        stats['binaries'] += 1
        
        generate_container_level(task)
        stats['containers'] += 1
    
    # Generate orchestrator
    generate_orchestrator(tasks)
    
    print('')
    print('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━')
    print('✅ ESCALATION LADDERS COMPLETE!')
    print('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━')
    print('')
    print(f"📊 Generated:")
    print(f"   • Scripts: {stats['scripts']}")
    print(f"   • Microkernels: {stats['microkernels']}")
    print(f"   • Binaries: {stats['binaries']}")
    print(f"   • Containers: {stats['containers']}")
    print('')
    print(f"📁 Output directory: {OUTPUT_DIR}")
    print('')
    print("🚀 Usage:")
    print(f"   cd {OUTPUT_DIR}")
    print("   ./orchestrate-task.sh <task_id> [target]")
    print('')

if __name__ == '__main__':
    main()

