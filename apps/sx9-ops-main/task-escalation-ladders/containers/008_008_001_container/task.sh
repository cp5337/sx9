#!/bin/bash
# Container execution for: Destroying Forensic Evidence

TARGET="${1:-localhost}"

echo "🐳 Container Execution"
echo "🎯 Task: Destroying Forensic Evidence"
echo "📍 Target: $TARGET"

# Execute all tools in sequence
echo "🔧 Running nmap..."
nmap $TARGET


echo "✅ Container execution complete"
