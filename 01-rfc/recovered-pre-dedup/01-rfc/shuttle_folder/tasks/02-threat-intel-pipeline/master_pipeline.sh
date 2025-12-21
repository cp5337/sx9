#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════
# SX9 Threat Intelligence Master Pipeline
# ═══════════════════════════════════════════════════════════════════════════
#
# Phases:
#   1. Download from 27 sources (30-60 min)
#   2. Normalize with RFC-9001 hashes  
#   3. Convert to RFC-9005 unified entities
#   4. Load to Neon PostgreSQL
#
# Usage:
#   ./master_pipeline.sh                    # Full pipeline
#   ./master_pipeline.sh --skip-download    # Skip download, normalize only
#   ./master_pipeline.sh --normalize-only   # Normalize + convert only
#   ./master_pipeline.sh --load-only        # Load existing to Neon
#   ./master_pipeline.sh --convert-only     # RFC-9005 convert only
#
# ═══════════════════════════════════════════════════════════════════════════

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Directories
DOWNLOAD_DIR="$SCRIPT_DIR/node-interview-generator/output/threat_content"
NORMALIZED_DIR="$SCRIPT_DIR/normalized"
NEON_DIR="$SCRIPT_DIR/neon_ready"
LOG_DIR="$SCRIPT_DIR/logs"

mkdir -p "$LOG_DIR"
mkdir -p "$NORMALIZED_DIR"
mkdir -p "$NEON_DIR"

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_FILE="$LOG_DIR/master_pipeline_${TIMESTAMP}.log"

# ═══════════════════════════════════════════════════════════════════════════
# Logging
# ═══════════════════════════════════════════════════════════════════════════

log() {
    echo -e "${GREEN}[$(date +%H:%M:%S)]${NC} $1" | tee -a "$LOG_FILE"
}

warn() {
    echo -e "${YELLOW}[$(date +%H:%M:%S)] ⚠️  $1${NC}" | tee -a "$LOG_FILE"
}

error() {
    echo -e "${RED}[$(date +%H:%M:%S)] ❌ $1${NC}" | tee -a "$LOG_FILE"
}

header() {
    echo -e "\n${BLUE}═══════════════════════════════════════════════════════════════${NC}" | tee -a "$LOG_FILE"
    echo -e "${CYAN} $1${NC}" | tee -a "$LOG_FILE"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}\n" | tee -a "$LOG_FILE"
}

# ═══════════════════════════════════════════════════════════════════════════
# Parse Arguments
# ═══════════════════════════════════════════════════════════════════════════

SKIP_DOWNLOAD=false
NORMALIZE_ONLY=false
LOAD_ONLY=false
CONVERT_ONLY=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-download)
            SKIP_DOWNLOAD=true
            shift
            ;;
        --normalize-only)
            NORMALIZE_ONLY=true
            shift
            ;;
        --load-only)
            LOAD_ONLY=true
            shift
            ;;
        --convert-only)
            CONVERT_ONLY=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# ═══════════════════════════════════════════════════════════════════════════
# Check Environment
# ═══════════════════════════════════════════════════════════════════════════

header "🔥 SX9 Threat Intelligence Master Pipeline"

log "Checking environment..."

# Check Python
if ! command -v python3 &> /dev/null; then
    error "Python3 not found"
    exit 1
fi
log "✅ Python3 available"

# Check for mmh3
if python3 -c "import mmh3" 2>/dev/null; then
    log "✅ mmh3 (Murmur3) available"
else
    warn "mmh3 not installed. Using fallback hashing."
fi

# Check for yaml
if python3 -c "import yaml" 2>/dev/null; then
    log "✅ PyYAML available"
else
    error "PyYAML not installed. Install with: pip install pyyaml"
    exit 1
fi

# Check Neon credentials
if [[ -z "$NEON_DATABASE_URL" ]] && [[ -z "$DATABASE_URL" ]]; then
    warn "NEON_DATABASE_URL or DATABASE_URL not set. Load step will require SQL file import."
    SKIP_LOAD=true
else
    log "✅ Database credentials found"
    SKIP_LOAD=false
    DB_URL="${NEON_DATABASE_URL:-$DATABASE_URL}"
fi

# ═══════════════════════════════════════════════════════════════════════════
# Phase 1: Download
# ═══════════════════════════════════════════════════════════════════════════

if [[ "$NORMALIZE_ONLY" == false ]] && [[ "$LOAD_ONLY" == false ]] && [[ "$CONVERT_ONLY" == false ]] && [[ "$SKIP_DOWNLOAD" == false ]]; then
    header "Phase 1/4: Download Threat Intelligence"
    
    log "Starting threat content download..."
    log "This will take 30-60 minutes."
    log "Output: $DOWNLOAD_DIR"
    
    cd "$SCRIPT_DIR/node-interview-generator"
    python3 threat_content_fetcher.py --all --no-training 2>&1 | tee -a "$LOG_FILE"
    
    if [[ $? -eq 0 ]]; then
        log "✅ Download complete"
    else
        error "Download failed"
        exit 1
    fi
    
    cd "$SCRIPT_DIR"
else
    if [[ "$SKIP_DOWNLOAD" == true ]]; then
        log "⏭️  Skipping download (--skip-download)"
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════
# Phase 2: Normalize (RFC-9001)
# ═══════════════════════════════════════════════════════════════════════════

if [[ "$LOAD_ONLY" == false ]] && [[ "$CONVERT_ONLY" == false ]]; then
    header "Phase 2/4: Normalize with RFC-9001 Hashes"
    
    log "Normalizing threat intelligence data..."
    log "Input: $DOWNLOAD_DIR"
    log "Output: $NORMALIZED_DIR"
    
    python3 normalize_threat_intel.py \
        --input "$DOWNLOAD_DIR" \
        --output "$NORMALIZED_DIR" \
        2>&1 | tee -a "$LOG_FILE"
    
    if [[ $? -eq 0 ]]; then
        log "✅ Normalization complete"
        ls -lh "$NORMALIZED_DIR"/*.json 2>/dev/null | head -5 | tee -a "$LOG_FILE"
    else
        error "Normalization failed"
        exit 1
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════
# Phase 3: Convert to RFC-9005 Unified Entities
# ═══════════════════════════════════════════════════════════════════════════

if [[ "$LOAD_ONLY" == false ]]; then
    header "Phase 3/4: Convert to RFC-9005 Unified Entities"
    
    log "Converting to RFC-9005 format..."
    log "Input: $NORMALIZED_DIR"
    log "Output: $NEON_DIR"
    
    python3 rfc9005_converter.py \
        --input "$NORMALIZED_DIR" \
        --output "$NEON_DIR" \
        2>&1 | tee -a "$LOG_FILE"
    
    if [[ $? -eq 0 ]]; then
        log "✅ RFC-9005 conversion complete"
        ls -lh "$NEON_DIR"/*.json "$NEON_DIR"/*.sql 2>/dev/null | tee -a "$LOG_FILE"
    else
        error "Conversion failed"
        exit 1
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════
# Phase 4: Load to Neon PostgreSQL
# ═══════════════════════════════════════════════════════════════════════════

if [[ "$NORMALIZE_ONLY" == false ]] && [[ "$CONVERT_ONLY" == false ]]; then
    header "Phase 4/4: Load to Neon PostgreSQL"
    
    if [[ "$SKIP_LOAD" == true ]]; then
        warn "Database URL not set. Skipping automatic load."
        log ""
        log "To load manually:"
        log "  1. Apply schema: psql \$DATABASE_URL -f neon/schema_rfc9005.sql"
        log "  2. Load data: psql \$DATABASE_URL -f neon_ready/neon_seed.sql"
    else
        log "Applying RFC-9005 schema..."
        psql "$DB_URL" -f "$SCRIPT_DIR/../neon/schema_rfc9005.sql" 2>&1 | tee -a "$LOG_FILE" || true
        
        log "Loading seed data..."
        psql "$DB_URL" -f "$NEON_DIR/neon_seed.sql" 2>&1 | tee -a "$LOG_FILE"
        
        if [[ $? -eq 0 ]]; then
            log "✅ Data loaded to Neon"
            psql "$DB_URL" -c "SELECT * FROM v_stats" 2>&1 | tee -a "$LOG_FILE" || true
        else
            warn "Load had some errors (check log)"
        fi
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════
# Summary
# ═══════════════════════════════════════════════════════════════════════════

header "🎯 Pipeline Complete"

log "Summary:"
log "  Log file: $LOG_FILE"
log "  Normalized: $NORMALIZED_DIR"
log "  Neon-ready: $NEON_DIR"

if [[ -f "$NEON_DIR/entities.json" ]]; then
    ENTITY_COUNT=$(python3 -c "import json; print(len(json.load(open('$NEON_DIR/entities.json'))))" 2>/dev/null || echo "0")
    REL_COUNT=$(python3 -c "import json; print(len(json.load(open('$NEON_DIR/relationships.json'))))" 2>/dev/null || echo "0")
    
    log ""
    log "  📊 Entities: $ENTITY_COUNT"
    log "  📊 Relationships: $REL_COUNT"
fi

log ""
log "🔥 Done!"
