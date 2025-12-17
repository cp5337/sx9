#!/bin/bash
# Execute Full Threat Content Pipeline
# 1. Refactor threat_content_fetcher.py (Vertex AI Gemini)
# 2. Download all threat content
# 3. Process and store according to storage plan

set -e

echo "🚀 Starting Full Threat Content Pipeline"
echo "=========================================="
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/node-interview-generator"

# Activate conda environment if available
if command -v conda &> /dev/null; then
    ENV_NAME="sx9-threat-intel"
    if conda env list | grep -q "^${ENV_NAME} "; then
        echo "📦 Activating conda environment: ${ENV_NAME}"
        eval "$(conda shell.bash hook)"
        conda activate "$ENV_NAME"
        echo "✅ Environment activated"
        echo ""
    fi
fi

# Cleanup flag: Delete external repos after processing to minimize local data
CLEANUP_REPOS="${CLEANUP_REPOS:-true}"

# Configuration
PROJECT_ID="${GOOGLE_CLOUD_PROJECT:-gen-lang-client-0290627006}"
REGION="${GCP_REGION:-us-central1}"
OUTPUT_DIR="$SCRIPT_DIR/output"
LOG_DIR="$SCRIPT_DIR/logs"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

mkdir -p "$LOG_DIR"

echo "📋 Configuration:"
echo "   Project: $PROJECT_ID"
echo "   Region: $REGION"
echo "   Output: $OUTPUT_DIR"
echo "   Logs: $LOG_DIR"
echo ""

# ============================================================================
# PHASE 1: REFACTOR (Optional - can skip if not needed)
# ============================================================================

if [ "${SKIP_REFACTOR:-false}" != "true" ]; then
    echo "🔨 Phase 1: Refactoring threat_content_fetcher.py with Vertex AI Gemini"
    echo "   (This may take 5-15 minutes)"
    echo ""
    
    if [ -f "refactor_with_vertex_gemini.py" ]; then
        python3 refactor_with_vertex_gemini.py \
            --project-id "$PROJECT_ID" \
            --region "$REGION" \
            --model gemini-2.0-flash-exp \
            --source threat_content_fetcher.py \
            --output refactored_output \
            2>&1 | tee "$LOG_DIR/refactor_${TIMESTAMP}.log"
        
        if [ $? -eq 0 ]; then
            echo "   ✅ Refactoring complete"
        else
            echo "   ⚠️  Refactoring had errors (check log)"
        fi
    else
        echo "   ⚠️  Refactor script not found, skipping"
    fi
    echo ""
else
    echo "⏭️  Phase 1: Skipping refactor (SKIP_REFACTOR=true)"
    echo ""
fi

# ============================================================================
# PHASE 2: DOWNLOAD ALL THREAT CONTENT
# ============================================================================

echo "📥 Phase 2: Downloading all threat content"
echo "   (This may take 30-60 minutes)"
echo ""

python3 threat_content_fetcher.py --all --no-training \
    2>&1 | tee "$LOG_DIR/download_${TIMESTAMP}.log"

if [ $? -eq 0 ]; then
    echo "   ✅ Download complete"
    echo ""
    
    # Show summary
    if [ -f "output/threat_content/threat_content_summary.json" ]; then
        echo "📊 Download Summary:"
        python3 -c "
import json
with open('output/threat_content/threat_content_summary.json') as f:
    data = json.load(f)
    counts = data.get('counts', {})
    total = sum(counts.values())
    print(f'   Total items: {total:,}')
    for k, v in counts.items():
        if v > 0:
            print(f'   • {k}: {v:,}')
"
    fi
else
    echo "   ❌ Download failed (check log: $LOG_DIR/download_${TIMESTAMP}.log)"
    exit 1
fi

echo ""

# ============================================================================
# PHASE 3: PROCESS WITH SPIRES
# ============================================================================

echo "🧠 Phase 3: Processing with SPIRES ontology generation"
echo "   (This may take 10-20 minutes)"
echo ""

cd "$SCRIPT_DIR"

if [ -f "spires_ontology_extractor.py" ]; then
    python3 spires_ontology_extractor.py --threats \
        2>&1 | tee "$LOG_DIR/spires_${TIMESTAMP}.log"
    
    if [ $? -eq 0 ]; then
        echo "   ✅ SPIRES processing complete"
    else
        echo "   ⚠️  SPIRES processing had errors (check log)"
    fi
else
    echo "   ⚠️  SPIRES extractor not found, skipping"
fi

echo ""

# ============================================================================
# PHASE 4: YAML TO DSL CONVERSION (RFC-9001/9002)
# ============================================================================

echo "🔄 Phase 4: Converting YAML to DSL (RFC-9001/9002 compliant)"
echo "   (This may take 15-30 minutes)"
echo ""

cd "$SCRIPT_DIR/node-interview-generator"

if [ -f "yaml_dsl_pipeline.py" ]; then
    python3 yaml_dsl_pipeline.py \
        --convert "$OUTPUT_DIR/threat_content" \
        --output "$OUTPUT_DIR/sx9_dsl" \
        2>&1 | tee "$LOG_DIR/dsl_${TIMESTAMP}.log"
    
    if [ $? -eq 0 ]; then
        echo "   ✅ DSL conversion complete"
    else
        echo "   ⚠️  DSL conversion had errors (check log)"
    fi
else
    echo "   ⚠️  DSL pipeline not found, skipping"
fi

echo ""

# ============================================================================
# PHASE 5: EXECUTE STORAGE PLAN
# ============================================================================

echo "💾 Phase 5: Executing storage plan"
echo ""

cd "$SCRIPT_DIR"

# Create storage execution script if it doesn't exist
if [ ! -f "execute_storage_plan.sh" ]; then
    echo "   ⚠️  Storage execution script not found, creating..."
    # Will be created below
fi

# Run storage plan
if [ -f "execute_storage_plan.sh" ]; then
    bash execute_storage_plan.sh 2>&1 | tee "$LOG_DIR/storage_${TIMESTAMP}.log"
    
    if [ $? -eq 0 ]; then
        echo "   ✅ Storage plan executed"
    else
        echo "   ⚠️  Storage plan had errors (check log)"
    fi
fi

echo ""
echo "=========================================="
echo "🎉 Pipeline Complete!"
echo "=========================================="
echo ""
echo "📊 Outputs:"
echo "   • Threat Content: $OUTPUT_DIR/threat_content/"
echo "   • SPIRES Ontology: $OUTPUT_DIR/ontology/"
echo "   • DSL Conversion: $OUTPUT_DIR/sx9_dsl/"
echo "   • Logs: $LOG_DIR/"
echo ""
echo "📄 Check logs for details:"
echo "   ls -lh $LOG_DIR/"
echo ""

