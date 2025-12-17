#!/bin/bash
# Quick-start script for SX9 Tool Exerciser

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

show_help() {
    cat << 'EOF'
SX9 Tool Exerciser - Quick Start

USAGE:
    ./run.sh [TIER] [OPTIONS]

TIERS:
    0   Help only (safest, no network)
    1   Localhost + scanme.nmap.org
    2   Passive OSINT
    3   Synthetic targets (starts DVWA container)
    4   Password cracking (offline)

OPTIONS:
    --build     Rebuild the container
    --shell     Drop into exerciser shell
    --nats      Start with NATS streaming
    --viewer    Start result viewer
    --clean     Clean output directories
    --help      Show this help

EXAMPLES:
    ./run.sh 0              # Run tier 0 (help only)
    ./run.sh 1 --nats       # Run tier 1 with NATS streaming
    ./run.sh 3              # Run tier 3 (starts DVWA)
    ./run.sh --shell        # Interactive shell

EOF
}

# Parse args
TIER=0
BUILD=false
SHELL_MODE=false
WITH_NATS=false
WITH_VIEWER=false
CLEAN=false

while [[ $# -gt 0 ]]; do
    case $1 in
        0|1|2|3|4)
            TIER=$1
            shift
            ;;
        --build)
            BUILD=true
            shift
            ;;
        --shell)
            SHELL_MODE=true
            shift
            ;;
        --nats)
            WITH_NATS=true
            shift
            ;;
        --viewer)
            WITH_VIEWER=true
            shift
            ;;
        --clean)
            CLEAN=true
            shift
            ;;
        --help|-h)
            show_help
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Clean
if [ "$CLEAN" = true ]; then
    echo -e "${YELLOW}Cleaning output directories...${NC}"
    rm -rf output/* logs/*
    echo -e "${GREEN}Done${NC}"
    exit 0
fi

# Create directories
mkdir -p output/{raw,parsed,hashed} logs

# Build if requested or if image doesn't exist
if [ "$BUILD" = true ]; then
    echo -e "${BLUE}Building exerciser container...${NC}"
    docker compose build exerciser
fi

# Export tier
export TIER

# Determine compose profiles
PROFILES=""
if [ "$TIER" = "3" ]; then
    PROFILES="--profile tier3"
fi
if [ "$WITH_VIEWER" = true ]; then
    PROFILES="$PROFILES --profile viewer"
fi

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  SX9 Tool Exerciser - Tier $TIER${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

if [ "$SHELL_MODE" = true ]; then
    # Interactive shell
    echo -e "${BLUE}Starting interactive shell...${NC}"
    docker compose run --rm exerciser /bin/bash
else
    # Run exerciser
    echo -e "${BLUE}Starting exerciser...${NC}"
    echo ""
    
    if [ "$WITH_NATS" = true ]; then
        # Start NATS first
        docker compose up -d nats
        sleep 2
    fi
    
    # Run exerciser
    docker compose $PROFILES run --rm exerciser -t "$TIER" -v run
    
    echo ""
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}  Results${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo "Output files:"
    ls -la output/hashed/*.json 2>/dev/null | head -20 || echo "  No outputs yet"
    echo ""
    echo "Short codes:"
    for f in output/hashed/*.json; do
        if [ -f "$f" ]; then
            code=$(basename "$f" .json)
            tool=$(jq -r '.tool' "$f" 2>/dev/null || echo "unknown")
            size=$(jq -r '.size_bytes' "$f" 2>/dev/null || echo "0")
            echo "  $code → $tool ($size bytes)"
        fi
    done 2>/dev/null || true
fi

