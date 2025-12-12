#!/bin/bash
# Container execution for: UAV Comms Exploitation

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: UAV Comms Exploitation"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
