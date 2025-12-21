#!/bin/bash
# Quick script to run tool discovery in Kali container

echo "🚀 Running Kali Tool Discovery..."

# Install Python if needed
apt-get update -qq && apt-get install -y python3 > /dev/null 2>&1

# Run discovery script
python3 /attack-scripts/kali-tool-discovery.py

echo ""
echo "📊 Results:"
echo "  - Tool help outputs: /results/tool-discovery/*.txt"
echo "  - Summary JSON: /results/tool-discovery/tool_discovery_summary.json"
echo "  - Automation report: /results/tool-discovery/automation_report.md"
