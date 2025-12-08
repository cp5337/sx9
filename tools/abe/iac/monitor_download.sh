#!/bin/bash
# Monitor Threat Content Download Progress
# Shows real-time progress, file counts, and cost tracking

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/node-interview-generator"

LOG_FILE="${LOG_FILE:-/tmp/threat_download.log}"
OUTPUT_DIR="output/threat_content"
ONTOLOGY_DIR="../output/ontology"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Cost tracking
START_TIME=$(date +%s)
COST_PER_HOUR=0.10  # Preemptible CPU instance cost
TOTAL_COST=0

echo "📊 Threat Content Download Monitor"
echo "=================================="
echo ""
echo "📋 Configuration:"
echo "   Log File: $LOG_FILE"
echo "   Output Dir: $OUTPUT_DIR"
echo "   Ontology Dir: $ONTOLOGY_DIR"
echo ""
echo "💰 Cost Estimate:"
echo "   • Data Download: FREE (ingress is free)"
echo "   • If on GCP Preemptible: ~\$0.10/hour"
echo "   • Estimated Total: ~\$0.40-0.60"
echo "   • Data Size: ~27 GB (compressed), ~50-75 GB (uncompressed)"
echo ""
echo "📥 Starting download with progress monitoring..."
echo ""

# Start download in background
python3 threat_content_fetcher.py --all --no-training 2>&1 | tee "$LOG_FILE" &
DOWNLOAD_PID=$!

echo "✅ Download started (PID: $DOWNLOAD_PID)"
echo "   Monitor: tail -f $LOG_FILE"
echo ""

# Progress monitoring loop
while kill -0 $DOWNLOAD_PID 2>/dev/null; do
    clear
    echo "📊 Threat Content Download Progress"
    echo "===================================="
    echo ""
    
    # Calculate elapsed time and cost
    CURRENT_TIME=$(date +%s)
    ELAPSED=$((CURRENT_TIME - START_TIME))
    ELAPSED_HOURS=$(awk "BEGIN {printf \"%.2f\", $ELAPSED / 3600}")
    ESTIMATED_COST=$(awk "BEGIN {printf \"%.4f\", $ELAPSED_HOURS * $COST_PER_HOUR}")
    
    echo "⏱️  Elapsed Time: ${ELAPSED}s (${ELAPSED_HOURS}h)"
    echo "💰 Estimated Cost: \$${ESTIMATED_COST}"
    echo ""
    
    # Check output directory
    if [ -d "$OUTPUT_DIR" ]; then
        FILE_COUNT=$(find "$OUTPUT_DIR" -type f 2>/dev/null | wc -l | tr -d ' ')
        TOTAL_SIZE=$(du -sh "$OUTPUT_DIR" 2>/dev/null | cut -f1)
        echo "📁 Output Directory:"
        echo "   Files: ${GREEN}${FILE_COUNT}${NC}"
        echo "   Size: ${GREEN}${TOTAL_SIZE}${NC}"
    else
        echo "📁 Output Directory: ${YELLOW}Not created yet${NC}"
    fi
    
    echo ""
    echo "📋 Source Status:"
    
    # Check each source
    sources=(
        "MITRE ATT&CK Enterprise:mitre_attack_enterprise"
        "MITRE ATT&CK ICS:mitre_attack_ics"
        "MITRE ATT&CK Mobile:mitre_attack_mobile"
        "MITRE D3FEND:mitre_d3fend"
        "MITRE CAR:mitre_car"
        "MITRE ENGAGE:mitre_engage"
        "MITRE ATLAS:mitre_atlas"
        "Atomic Red Team:atomic_red_team"
        "Nuclei Templates:nuclei_templates"
        "Sigma Rules:sigma_rules"
        "YARA Rules:yara_rules"
        "Wazuh Rules:wazuh_rules"
        "LOLBAS:lolbas"
        "GTFOBins:gtfobins"
        "OSINT Framework:osint_framework"
    )
    
    for source_info in "${sources[@]}"; do
        IFS=':' read -r name dir <<< "$source_info"
        if [ -d "$OUTPUT_DIR/$dir" ]; then
            count=$(find "$OUTPUT_DIR/$dir" -type f 2>/dev/null | wc -l | tr -d ' ')
            if [ "$count" -gt 0 ]; then
                echo "   ${GREEN}✓${NC} $name: $count files"
            else
                echo "   ${YELLOW}○${NC} $name: 0 files (downloading...)"
            fi
        else
            echo "   ${YELLOW}○${NC} $name: Not started"
        fi
    done
    
    echo ""
    echo "🧠 SPIRES Ontology:"
    if [ -f "$ONTOLOGY_DIR/ontology_raw.json" ]; then
        if command -v python3 &> /dev/null; then
            term_count=$(python3 -c "import json; f=open('$ONTOLOGY_DIR/ontology_raw.json'); d=json.load(f); print(len(d.get('terms', [])))" 2>/dev/null || echo "?")
            rel_count=$(python3 -c "import json; f=open('$ONTOLOGY_DIR/ontology_raw.json'); d=json.load(f); print(len(d.get('relations', [])))" 2>/dev/null || echo "?")
            echo "   ${GREEN}✓${NC} Terms: $term_count"
            echo "   ${GREEN}✓${NC} Relations: $rel_count"
        else
            echo "   ${GREEN}✓${NC} Generated"
        fi
    else
        echo "   ${YELLOW}○${NC} Not generated yet"
    fi
    
    echo ""
    echo "📝 Recent Log Activity:"
    if [ -f "$LOG_FILE" ]; then
        tail -n 5 "$LOG_FILE" | sed 's/^/   /'
    else
        echo "   ${YELLOW}Log file not created yet${NC}"
    fi
    
    echo ""
    echo "Press Ctrl+C to stop monitoring (download will continue)"
    
    sleep 5
done

# Wait for download to complete
wait $DOWNLOAD_PID
EXIT_CODE=$?

clear
echo "📊 Download Complete!"
echo "===================="
echo ""

if [ $EXIT_CODE -eq 0 ]; then
    echo "${GREEN}✅ Download successful!${NC}"
else
    echo "${RED}❌ Download failed (exit code: $EXIT_CODE)${NC}"
fi

# Final summary
FINAL_TIME=$(date +%s)
TOTAL_ELAPSED=$((FINAL_TIME - START_TIME))
TOTAL_HOURS=$(awk "BEGIN {printf \"%.2f\", $TOTAL_ELAPSED / 3600}")
FINAL_COST=$(awk "BEGIN {printf \"%.4f\", $TOTAL_HOURS * $COST_PER_HOUR}")

echo ""
echo "⏱️  Total Time: ${TOTAL_ELAPSED}s (${TOTAL_HOURS}h)"
echo "💰 Total Cost: \$${FINAL_COST}"
echo ""

if [ -d "$OUTPUT_DIR" ]; then
    FILE_COUNT=$(find "$OUTPUT_DIR" -type f 2>/dev/null | wc -l | tr -d ' ')
    TOTAL_SIZE=$(du -sh "$OUTPUT_DIR" 2>/dev/null | cut -f1)
    echo "📁 Final Output:"
    echo "   Files: ${GREEN}${FILE_COUNT}${NC}"
    echo "   Size: ${GREEN}${TOTAL_SIZE}${NC}"
fi

echo ""
echo "📋 Check log for details: $LOG_FILE"

