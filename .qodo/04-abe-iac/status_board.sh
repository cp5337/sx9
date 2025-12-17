#!/bin/bash
# CTAS-7 System Status Board
# Quick ASCII dashboard for system status

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# Status symbols
CHECK="${GREEN}✓${NC}"
CROSS="${RED}✗${NC}"
WARN="${YELLOW}⚠${NC}"
INFO="${BLUE}ℹ${NC}"

echo ""
echo "╔══════════════════════════════════════════════════════════════════════════════╗"
echo "║                                                                              ║"
echo -e "║${CYAN}     ███████╗██╗  ██╗███╗   ██╗ █████╗ ██████╗ ████████╗██╗██╗  ██╗${NC}     ║"
echo -e "║${CYAN}     ██╔════╝╚██╗██╔╝████╗  ██║██╔══██╗██╔══██╗╚══██╔══╝██║╚██╗██╔╝${NC}     ║"
echo -e "║${CYAN}     ███████╗ ╚███╔╝ ██╔██╗ ██║███████║██████╔╝   ██║   ██║ ╚███╔╝ ${NC}     ║"
echo -e "║${CYAN}     ╚════██║ ██╔██╗ ██║╚██╗██║██╔══██║██╔══██╗   ██║   ██║ ██╔██╗ ${NC}     ║"
echo -e "║${CYAN}     ███████║██╔╝ ██╗██║ ╚████║██║  ██║██║  ██║   ██║   ██║██╔╝ ██╗${NC}     ║"
echo -e "║${CYAN}     ╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   ╚═╝╚═╝  ╚═╝${NC}     ║"
echo "║                                                                              ║"
echo -e "║${MAGENTA}                    SYSTEM STATUS BOARD${NC}                                  ║"
echo "╚══════════════════════════════════════════════════════════════════════════════╝"
echo ""

# ============================================================================
# PIPELINE STATUS
# ============================================================================
echo "┌─ PIPELINE STATUS ───────────────────────────────────────────────────────────┐"

# Check if pipeline is running
if [ -f "logs/pipeline.pid" ]; then
    PID=$(cat logs/pipeline.pid 2>/dev/null || echo "")
    if [ -n "$PID" ] && ps -p "$PID" > /dev/null 2>&1; then
        echo "│ Pipeline:        ${GREEN}RUNNING${NC} (PID: $PID)"
    else
        echo "│ Pipeline:        ${YELLOW}COMPLETED${NC} (process finished)"
    fi
else
    echo "│ Pipeline:        ${YELLOW}NOT RUNNING${NC}"
fi

# Check pipeline phases
echo "│"
echo "│ Phases:"
if [ -d "node-interview-generator/output/threat_content" ] && [ "$(ls -A node-interview-generator/output/threat_content/*.json 2>/dev/null | wc -l)" -gt 0 ]; then
    COUNT=$(ls -1 node-interview-generator/output/threat_content/*.json 2>/dev/null | wc -l | tr -d ' ')
    SIZE=$(du -sh node-interview-generator/output/threat_content 2>/dev/null | cut -f1)
    echo "│   ${CHECK} Download:        $COUNT files ($SIZE)"
else
    echo "│   ${CROSS} Download:        Not found"
fi

if [ -d "output/ontology" ] && [ -f "output/ontology/ontology_raw.json" ]; then
    SIZE=$(du -sh output/ontology 2>/dev/null | cut -f1)
    echo "│   ${CHECK} SPIRES:          $SIZE"
else
    echo "│   ${CROSS} SPIRES:          Not found"
fi

if [ -d "output/sx9_dsl" ] && [ "$(ls -A output/sx9_dsl/*.json 2>/dev/null | wc -l)" -gt 0 ]; then
    COUNT=$(ls -1 output/sx9_dsl/*.json 2>/dev/null | wc -l | tr -d ' ')
    SIZE=$(du -sh output/sx9_dsl 2>/dev/null | cut -f1)
    echo "│   ${CHECK} DSL Conversion:  $COUNT files ($SIZE)"
else
    echo "│   ${CROSS} DSL Conversion:  Not found"
fi

if [ -f "logs/storage_*.log" ] && grep -q "✅" logs/storage_*.log 2>/dev/null; then
    echo "│   ${CHECK} Storage:         Executed"
else
    echo "│   ${CROSS} Storage:         Not executed"
fi

echo "└──────────────────────────────────────────────────────────────────────────────┘"
echo ""

# ============================================================================
# DATA STATUS
# ============================================================================
echo "┌─ DATA STATUS ────────────────────────────────────────────────────────────────┐"

# Threat Content
if [ -d "node-interview-generator/output/threat_content" ]; then
    THREAT_SIZE=$(du -sh node-interview-generator/output/threat_content 2>/dev/null | cut -f1)
    THREAT_COUNT=$(ls -1 node-interview-generator/output/threat_content/*.json 2>/dev/null | wc -l | tr -d ' ')
    echo "│ Threat Content:  ${GREEN}$THREAT_SIZE${NC} ($THREAT_COUNT files)"
else
    echo "│ Threat Content:  ${RED}Not found${NC}"
fi

# Ontology
if [ -d "output/ontology" ]; then
    ONT_SIZE=$(du -sh output/ontology 2>/dev/null | cut -f1)
    ONT_FILES=$(ls -1 output/ontology/*.json 2>/dev/null | wc -l | tr -d ' ')
    echo "│ Ontology:        ${GREEN}$ONT_SIZE${NC} ($ONT_FILES files)"
else
    echo "│ Ontology:        ${RED}Not found${NC}"
fi

# DSL
if [ -d "output/sx9_dsl" ]; then
    DSL_SIZE=$(du -sh output/sx9_dsl 2>/dev/null | cut -f1)
    DSL_COUNT=$(ls -1 output/sx9_dsl/*.json 2>/dev/null | wc -l | tr -d ' ')
    echo "│ DSL Conversion:  ${GREEN}$DSL_SIZE${NC} ($DSL_COUNT files)"
else
    echo "│ DSL Conversion:  ${YELLOW}Not generated${NC}"
fi

# Task Graph
if [ -d "output/task_graph" ]; then
    TG_SIZE=$(du -sh output/task_graph 2>/dev/null | cut -f1)
    TG_COUNT=$(ls -1 output/task_graph/*.json 2>/dev/null | wc -l | tr -d ' ')
    echo "│ Task Graph:       ${GREEN}$TG_SIZE${NC} ($TG_COUNT files)"
else
    echo "│ Task Graph:       ${YELLOW}Not generated${NC}"
fi

echo "└──────────────────────────────────────────────────────────────────────────────┘"
echo ""

# ============================================================================
# INTEGRATION STATUS
# ============================================================================
echo "┌─ INTEGRATION PLANS ──────────────────────────────────────────────────────────┐"

if [ -f "CTAS_INTEGRATION_PLANS.md" ]; then
    echo "│ ${CHECK} Unified Plan:     CTAS_INTEGRATION_PLANS.md"
else
    echo "│ ${CROSS} Unified Plan:     Not found"
fi

if [ -f "OSINT_CTAS_INTEGRATION.md" ]; then
    echo "│ ${CHECK} OSINT Plan:       OSINT_CTAS_INTEGRATION.md"
else
    echo "│ ${CROSS} OSINT Plan:       Not found"
fi

if [ -f "KALI_CTAS_INTEGRATION.md" ]; then
    echo "│ ${CHECK} Kali Plan:        KALI_CTAS_INTEGRATION.md"
else
    echo "│ ${CROSS} Kali Plan:        Not found"
fi

if [ -f "THREAT_INTEL_CTAS_INTEGRATION.md" ]; then
    echo "│ ${CHECK} Threat Intel:     THREAT_INTEL_CTAS_INTEGRATION.md"
else
    echo "│ ${CROSS} Threat Intel:     Not found"
fi

echo "└──────────────────────────────────────────────────────────────────────────────┘"
echo ""

# ============================================================================
# SERVICES STATUS (if running)
# ============================================================================
echo "┌─ SERVICES STATUS ────────────────────────────────────────────────────────────┐"

# Check Neo4j
if nc -z localhost 7687 2>/dev/null; then
    echo "│ Neo4j (Bolt):     ${GREEN}RUNNING${NC} (port 7687)"
else
    echo "│ Neo4j (Bolt):     ${YELLOW}NOT RUNNING${NC}"
fi

if nc -z localhost 7474 2>/dev/null; then
    echo "│ Neo4j (HTTP):     ${GREEN}RUNNING${NC} (port 7474)"
else
    echo "│ Neo4j (HTTP):     ${YELLOW}NOT RUNNING${NC}"
fi

# Check GLAF
if nc -z localhost 18050 2>/dev/null; then
    echo "│ GLAF:             ${GREEN}RUNNING${NC} (port 18050)"
else
    echo "│ GLAF:             ${YELLOW}NOT RUNNING${NC}"
fi

# Check Supabase (via environment or config)
if [ -n "$SUPABASE_URL" ] || [ -f "../sx9-ops-main-platform/.env.local" ]; then
    echo "│ Supabase:         ${GREEN}CONFIGURED${NC}"
else
    echo "│ Supabase:         ${YELLOW}NOT CONFIGURED${NC}"
fi

# Check Docker
if command -v docker &> /dev/null && docker ps &> /dev/null; then
    CONTAINERS=$(docker ps -q | wc -l | tr -d ' ')
    echo "│ Docker:           ${GREEN}RUNNING${NC} ($CONTAINERS containers)"
else
    echo "│ Docker:           ${YELLOW}NOT RUNNING${NC}"
fi

echo "└──────────────────────────────────────────────────────────────────────────────┘"
echo ""

# ============================================================================
# RECENT LOGS
# ============================================================================
echo "┌─ RECENT ACTIVITY ─────────────────────────────────────────────────────────────┐"

if [ -d "logs" ] && [ "$(ls -A logs/*.log 2>/dev/null)" ]; then
    LATEST_LOG=$(ls -t logs/*.log 2>/dev/null | head -1)
    if [ -n "$LATEST_LOG" ]; then
        echo "│ Latest log:      $(basename "$LATEST_LOG")"
        echo "│"
        # Show last 3 lines of latest log
        tail -3 "$LATEST_LOG" 2>/dev/null | while IFS= read -r line; do
            echo "│   $line"
        done
    else
        echo "│ No log files found"
    fi
else
    echo "│ No logs directory or log files"
fi

echo "└──────────────────────────────────────────────────────────────────────────────┘"
echo ""

# ============================================================================
# QUICK ACTIONS
# ============================================================================
echo "┌─ QUICK ACTIONS ───────────────────────────────────────────────────────────────┐"
echo "│"
echo "│   ${CYAN}./execute_full_pipeline.sh${NC}     Run full pipeline"
echo "│   ${CYAN}./execute_storage_plan.sh${NC}      Run storage plan only"
echo "│   ${CYAN}tail -f logs/pipeline_*.log${NC}    Watch pipeline logs"
echo "│   ${CYAN}./status_board.sh${NC}              Refresh this status board"
echo "│"
echo "└──────────────────────────────────────────────────────────────────────────────┘"
echo ""

# Footer
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
echo "Last updated: $TIMESTAMP"
echo ""

