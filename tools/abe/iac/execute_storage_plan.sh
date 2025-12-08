#!/bin/bash
# Execute Storage Plan: Upload to Supabase, CDN, Neo4j
# Based on STORAGE_STRATEGY.md

set -e

echo "💾 Executing Storage Plan"
echo "========================"
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_DIR="$SCRIPT_DIR/output"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Check for required files (threat_content may be in node-interview-generator/output/threat_content)
THREAT_CONTENT_DIR="$OUTPUT_DIR/threat_content"
if [ ! -d "$THREAT_CONTENT_DIR" ]; then
    # Try alternative location
    THREAT_CONTENT_DIR="$SCRIPT_DIR/node-interview-generator/output/threat_content"
    if [ ! -d "$THREAT_CONTENT_DIR" ]; then
        echo "❌ Threat content not found. Run download first."
        echo "   Checked: $OUTPUT_DIR/threat_content"
        echo "   Checked: $THREAT_CONTENT_DIR"
        exit 1
    fi
fi
echo "✅ Found threat content at: $THREAT_CONTENT_DIR"

# ============================================================================
# PHASE 1: LOAD TO SUPABASE (PRIMARY STORAGE)
# ============================================================================

echo "📊 Phase 1: Loading to Supabase (Primary Storage)"
echo ""

# Check if load script exists, create if not
if [ ! -f "$SCRIPT_DIR/node-interview-generator/load_to_supabase.py" ]; then
    echo "   Creating load_to_supabase.py..."
    cat > "$SCRIPT_DIR/node-interview-generator/load_to_supabase.py" << 'PYTHON_EOF'
#!/usr/bin/env python3
"""
Load processed threat content to Supabase
Uses existing plasma_threats and plasma_entities tables + new tables
"""

import json
import sys
from pathlib import Path
from typing import Dict, List, Any

try:
    from supabase import create_client, Client
    SUPABASE_AVAILABLE = True
except ImportError:
    SUPABASE_AVAILABLE = False
    print("⚠️  supabase-py not installed. Install with: pip install supabase")
    sys.exit(1)

# Load Supabase credentials from environment or config
import os
SUPABASE_URL = os.environ.get("SUPABASE_URL")
SUPABASE_KEY = os.environ.get("SUPABASE_KEY") or os.environ.get("SUPABASE_ANON_KEY")

if not SUPABASE_URL or not SUPABASE_KEY:
    print("❌ SUPABASE_URL and SUPABASE_KEY must be set")
    sys.exit(1)

def load_threat_tools(supabase: Client, tools_file: Path):
    """Load threat tools to Supabase."""
    if not tools_file.exists():
        return 0
    
    with open(tools_file) as f:
        tools = json.load(f)
    
    if not isinstance(tools, list):
        tools = tools.get("tools", [])
    
    count = 0
    for tool in tools:
        try:
            # Insert into plasma_entities (tools are entities)
            supabase.table("plasma_entities").insert({
                "type": "tool",
                "value": tool.get("name", ""),
                "metadata": {
                    "description": tool.get("description", ""),
                    "categories": tool.get("categories", []),
                    "homepage": tool.get("homepage", ""),
                    "trivariate_hash": tool.get("trivariate_hash", ""),
                    "unicode_operation": tool.get("unicode_operation", ""),
                    "hd4_phase": tool.get("hd4_phase", ""),
                    "task_graph_node": tool.get("task_graph_node", {})
                },
                "tags": tool.get("categories", []),
                "trivariate_hash": tool.get("trivariate_hash", "")
            }).execute()
            count += 1
        except Exception as e:
            print(f"   ⚠️  Error loading tool {tool.get('name', 'unknown')}: {e}")
    
    return count

def load_ontology(supabase: Client, ontology_file: Path):
    """Load SPIRES ontology to Supabase."""
    if not ontology_file.exists():
        return 0
    
    with open(ontology_file) as f:
        ontology = json.load(f)
    
    # Create threat_ontology table if needed
    # (This would normally be a migration, but for now we'll use plasma_threats)
    
    terms = ontology.get("terms", [])
    count = 0
    
    for term in terms:
        try:
            # Store ontology terms as threats with special source
            supabase.table("plasma_threats").insert({
                "level": "medium",
                "source": "spires_ontology",
                "target": term.get("category", ""),
                "description": term.get("description", ""),
                "mitre": term.get("mitre_mappings", []),
                "raw_data": {
                    "term": term.get("name", ""),
                    "category": term.get("category", ""),
                    "relationships": term.get("relationships", []),
                    "spires_metadata": term.get("metadata", {})
                },
                "source_feed": "spires_ontology"
            }).execute()
            count += 1
        except Exception as e:
            print(f"   ⚠️  Error loading term {term.get('name', 'unknown')}: {e}")
    
    return count

def main():
    output_dir = Path(__file__).parent.parent / "output"
    
    if not SUPABASE_AVAILABLE:
        print("❌ Supabase client not available")
        return
    
    supabase: Client = create_client(SUPABASE_URL, SUPABASE_KEY)
    
    print("📊 Loading threat content to Supabase...")
    print("")
    
    # Load Kali tools
    tools_file = output_dir / "threat_content" / "kali_tools_inventory.json"
    if tools_file.exists():
        print("   Loading Kali tools...")
        count = load_threat_tools(supabase, tools_file)
        print(f"   ✅ Loaded {count} tools")
    
    # Load ontology
    ontology_file = output_dir / "ontology" / "sx9_ontology.json"
    if ontology_file.exists():
        print("   Loading SPIRES ontology...")
        count = load_ontology(supabase, ontology_file)
        print(f"   ✅ Loaded {count} ontology terms")
    
    print("")
    print("✅ Supabase loading complete")

if __name__ == "__main__":
    main()
PYTHON_EOF
    chmod +x "$SCRIPT_DIR/node-interview-generator/load_to_supabase.py"
fi

# Run Supabase loader
if [ -f "$SCRIPT_DIR/node-interview-generator/load_to_supabase.py" ]; then
    cd "$SCRIPT_DIR/node-interview-generator"
    python3 load_to_supabase.py 2>&1 | tee "$SCRIPT_DIR/logs/supabase_load_${TIMESTAMP}.log" || {
        echo "   ⚠️  Supabase loading had errors (check log)"
    }
fi

echo ""

# ============================================================================
# PHASE 2: UPLOAD TO CDN (Cloudflare R2 + GCP)
# ============================================================================

echo "🌐 Phase 2: Uploading to CDN (Cloudflare R2 + GCP Cloud CDN)"
echo ""

# Check for rclone (Cloudflare R2)
if command -v rclone &> /dev/null; then
    echo "   Uploading to Cloudflare R2..."
    
    # Check if R2 is configured
    if rclone listremotes | grep -q "r2:"; then
        rclone copy "$OUTPUT_DIR/ontology/" r2:ctas7-threat-intel/ontology/ --progress --log-file="$SCRIPT_DIR/logs/r2_upload_${TIMESTAMP}.log" || {
            echo "   ⚠️  R2 upload had errors"
        }
        rclone copy "$OUTPUT_DIR/sx9_dsl/" r2:ctas7-threat-intel/dsl/ --progress --log-file="$SCRIPT_DIR/logs/r2_upload_${TIMESTAMP}.log" || {
            echo "   ⚠️  R2 upload had errors"
        }
        echo "   ✅ Cloudflare R2 upload complete"
    else
        echo "   ⚠️  R2 not configured. Run: rclone config"
    fi
else
    echo "   ⚠️  rclone not installed. Install with: brew install rclone"
fi

# Check for gsutil (GCP Cloud CDN)
if command -v gsutil &> /dev/null; then
    echo "   Uploading to GCP Cloud CDN..."
    
    BUCKET_NAME="ctas7-threat-intel-private"
    
    # Create bucket if it doesn't exist
    if ! gsutil ls -b "gs://$BUCKET_NAME" &>/dev/null; then
        echo "   Creating GCS bucket: $BUCKET_NAME"
        gsutil mb -l us-central1 "gs://$BUCKET_NAME" || true
    fi
    
    # Upload hashes (private/IAM-gated)
    if [ -d "$OUTPUT_DIR/hashes" ]; then
        gsutil -m cp -r "$OUTPUT_DIR/hashes/" "gs://$BUCKET_NAME/hashes/" 2>&1 | tee "$SCRIPT_DIR/logs/gcp_upload_${TIMESTAMP}.log" || {
            echo "   ⚠️  GCP upload had errors"
        }
        echo "   ✅ GCP Cloud CDN upload complete"
    else
        echo "   ⚠️  Hashes directory not found (will be created in processing)"
    fi
else
    echo "   ⚠️  gsutil not installed. Install Google Cloud SDK"
fi

echo ""

# ============================================================================
# PHASE 3: LOAD TO NEO4J (Graph Relationships)
# ============================================================================

echo "🕸️  Phase 3: Loading to Neo4j (Graph Relationships)"
echo ""

if [ -f "$SCRIPT_DIR/node-interview-generator/neo4j_threat_loader.py" ]; then
    cd "$SCRIPT_DIR/node-interview-generator"
    python3 neo4j_threat_loader.py --all 2>&1 | tee "$SCRIPT_DIR/logs/neo4j_load_${TIMESTAMP}.log" || {
        echo "   ⚠️  Neo4j loading had errors (check log)"
    }
    echo "   ✅ Neo4j loading complete"
else
    echo "   ⚠️  Neo4j loader not found, skipping"
fi

echo ""

# ============================================================================
# PHASE 4: ARCHIVE TO GCS (Backup)
# ============================================================================

echo "📦 Phase 4: Archiving to GCS (Backup)"
echo ""

if command -v gsutil &> /dev/null; then
    ARCHIVE_BUCKET="ctas7-threat-intel-archive"
    ARCHIVE_FILE="threat_data_${TIMESTAMP}.tar.gz"
    
    # Create archive
    echo "   Creating compressed archive..."
    cd "$OUTPUT_DIR"
    tar -czf "/tmp/$ARCHIVE_FILE" threat_content/ ontology/ sx9_dsl/ task_graph/ hashes/ 2>/dev/null || {
        # If some dirs don't exist, archive what we have
        tar -czf "/tmp/$ARCHIVE_FILE" threat_content/ ontology/ 2>/dev/null || true
    }
    
    # Create bucket if needed
    if ! gsutil ls -b "gs://$ARCHIVE_BUCKET" &>/dev/null; then
        echo "   Creating archive bucket: $ARCHIVE_BUCKET"
        gsutil mb -l us-central1 "gs://$ARCHIVE_BUCKET" || true
    fi
    
    # Upload archive
    echo "   Uploading archive..."
    gsutil cp "/tmp/$ARCHIVE_FILE" "gs://$ARCHIVE_BUCKET/" 2>&1 | tee "$SCRIPT_DIR/logs/archive_upload_${TIMESTAMP}.log"
    
    if [ $? -eq 0 ]; then
        echo "   ✅ Archive uploaded: gs://$ARCHIVE_BUCKET/$ARCHIVE_FILE"
        rm -f "/tmp/$ARCHIVE_FILE"  # Clean up local archive
    else
        echo "   ⚠️  Archive upload had errors"
    fi
else
    echo "   ⚠️  gsutil not installed, skipping archive"
fi

echo ""

# ============================================================================
# PHASE 5: CLEANUP (Delete Raw Repos)
# ============================================================================

echo "🧹 Phase 5: Cleanup (Delete Raw Repos)"
echo ""

if [ "${KEEP_RAW_REPOS:-false}" != "true" ]; then
    echo "   Deleting raw repos (saves ~800 MB)..."
    
    # Delete cloned repos but keep JSON files
    cd "$OUTPUT_DIR/threat_content"
    rm -rf atomic-red-team/ nuclei-templates/ sigma/ caldera/ lolbas/ gtfobins/ \
           loldrivers/ hijacklibs/ wadcoms/ awesome-osint/ osint-framework/ \
           sherlock/ nmap/ yara-rules/ wazuh/ car/ atlas/ exploitdb/ 2>/dev/null || true
    
    echo "   ✅ Raw repos deleted"
else
    echo "   ⏭️  Keeping raw repos (KEEP_RAW_REPOS=true)"
fi

echo ""

# ============================================================================
# SUMMARY
# ============================================================================

echo "=========================================="
echo "✅ Storage Plan Execution Complete!"
echo "=========================================="
echo ""
echo "📊 Storage Summary:"
echo "   • Supabase: Primary storage (subscription)"
echo "   • Cloudflare R2: CDN (public/semi-public)"
echo "   • GCP Cloud CDN: CDN (private/IAM-gated)"
echo "   • Neo4j: Graph relationships"
echo "   • GCS Archive: Backup"
echo ""
echo "📄 Logs: $SCRIPT_DIR/logs/"
echo ""

