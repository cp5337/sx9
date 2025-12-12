#!/bin/bash
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
