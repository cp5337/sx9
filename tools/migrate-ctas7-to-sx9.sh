#!/bin/bash
#
# CTAS7 → Synaptix9 Migration Script
# Renames and migrates the CTAS7 command center to SX9 core backend
#
# Usage: ./migrate-ctas7-to-sx9.sh [--dry-run] [--source /path/to/ctas7] [--dest /path/to/sx9]
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default paths
SOURCE_DIR="${SOURCE_DIR:-/Users/cp5337/Developer/ctas7-command-center}"
DEST_DIR="${DEST_DIR:-/Users/cp5337/Developer/synaptix9-workflow-system/packages/core/src}"
DRY_RUN=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --source)
            SOURCE_DIR="$2"
            shift 2
            ;;
        --dest)
            DEST_DIR="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     CTAS7 → Synaptix9 Migration Script                     ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "Source: ${YELLOW}$SOURCE_DIR${NC}"
echo -e "Dest:   ${YELLOW}$DEST_DIR${NC}"
echo -e "Mode:   ${YELLOW}$([ "$DRY_RUN" = true ] && echo "DRY RUN" || echo "LIVE")${NC}"
echo ""

# ============================================================================
# PHASE 1: Validate source directory
# ============================================================================
echo -e "${BLUE}[Phase 1]${NC} Validating source directory..."

if [ ! -d "$SOURCE_DIR/src" ]; then
    echo -e "${RED}ERROR: Source directory not found or missing src/: $SOURCE_DIR${NC}"
    exit 1
fi

if [ ! -f "$SOURCE_DIR/src/services/CTASOrchestrator.ts" ]; then
    echo -e "${RED}ERROR: CTASOrchestrator.ts not found - is this the right codebase?${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Source validated${NC}"

# ============================================================================
# PHASE 2: Create destination structure
# ============================================================================
echo -e "${BLUE}[Phase 2]${NC} Creating destination structure..."

DIRS_TO_CREATE=(
    "$DEST_DIR"
    "$DEST_DIR/services"
    "$DEST_DIR/hooks"
    "$DEST_DIR/components"
    "$DEST_DIR/types"
    "$DEST_DIR/utils"
    "$DEST_DIR/lib"
    "$DEST_DIR/core"
    "$DEST_DIR/engines"
)

for dir in "${DIRS_TO_CREATE[@]}"; do
    if [ "$DRY_RUN" = true ]; then
        echo "  [DRY] Would create: $dir"
    else
        mkdir -p "$dir"
        echo -e "  ${GREEN}✓${NC} Created: $dir"
    fi
done

# ============================================================================
# PHASE 3: Define rename mappings
# ============================================================================
echo -e "${BLUE}[Phase 3]${NC} Preparing rename mappings..."

# Function to get renamed filename
get_renamed_file() {
    local filename="$1"
    case "$filename" in
        "CTASOrchestrator.ts") echo "SX9Orchestrator.ts" ;;
        "CTASOntologyManager.tsx") echo "SX9OntologyManager.tsx" ;;
        "CTASCommandCenter.tsx") echo "SX9CommandCenter.tsx" ;;
        "CTASCrateManagement.tsx") echo "SX9CrateManagement.tsx" ;;
        "ctasService.ts") echo "sx9Service.ts" ;;
        *) echo "$filename" ;;
    esac
}

# String replacements (in order of specificity - longest first)
REPLACEMENTS=(
    "CTASOrchestrator:SX9Orchestrator"
    "CTASOntologyManager:SX9OntologyManager"
    "CTASCommandCenter:SX9CommandCenter"
    "CTASCrateManagement:SX9CrateManagement"
    "CTASService:SX9Service"
    "CTASSystemStatus:SX9SystemStatus"
    "CTASEcosystemStatus:SX9EcosystemStatus"
    "CTASTask:SX9Task"
    "CTASReporter:SX9Reporter"
    "ctasService:sx9Service"
    "ctasIssues:sx9Issues"
    "CTAS_BASE:SX9_BASE"
    "CTAS_DB_URL:SX9_DB_URL"
    "CTAS_SCHEMA_AUDIT_STATUS:SX9_SCHEMA_AUDIT_STATUS"
    "CTAS-7:Synaptix9"
    "CTAS7:Synaptix9"
    "ctas7:sx9"
    "ctas-7:sx9"
    "CTAS:SX9"
)

echo -e "${GREEN}✓ 5 file renames configured${NC}"
echo -e "${GREEN}✓ ${#REPLACEMENTS[@]} string replacements configured${NC}"

# Function to apply replacements to a file
apply_replacements() {
    local file="$1"
    for replacement in "${REPLACEMENTS[@]}"; do
        old="${replacement%%:*}"
        new="${replacement##*:}"
        sed -i '' "s/$old/$new/g" "$file" 2>/dev/null || true
    done
}

# ============================================================================
# PHASE 4: Copy and transform services (CORE BACKEND)
# ============================================================================
echo -e "${BLUE}[Phase 4]${NC} Migrating core services..."

SERVICES=(
    "CTASOrchestrator.ts"
    "ScriptExecutionCoordinator.ts"
    "LegionExecutionEngine.ts"
    "HashingEngineConnector.ts"
    "SlotGraphQueryEngine.ts"
    "LegionSlotGraphSchema.ts"
    "ctasService.ts"
    "SledKVStore.ts"
)

for service in "${SERVICES[@]}"; do
    src_file="$SOURCE_DIR/src/services/$service"
    dest_name=$(get_renamed_file "$service")
    dest_file="$DEST_DIR/services/$dest_name"
    
    if [ -f "$src_file" ]; then
        if [ "$DRY_RUN" = true ]; then
            echo "  [DRY] Would copy: $service → $dest_name"
        else
            cp "$src_file" "$dest_file"
            apply_replacements "$dest_file"
            echo -e "  ${GREEN}✓${NC} $service → $dest_name"
        fi
    else
        echo -e "  ${YELLOW}⚠${NC} Not found: $service"
    fi
done

# ============================================================================
# PHASE 5: Migrate hooks
# ============================================================================
echo -e "${BLUE}[Phase 5]${NC} Migrating hooks..."

HOOKS=(
    "useWebSocket.ts"
    "useVoiceConversation.ts"
    "useSystemDiagnostics.ts"
    "useSupabaseData.ts"
    "useBeamTelemetry.ts"
    "useBeamPatterns.ts"
    "useRealtimeTelemetry.ts"
)

for hook in "${HOOKS[@]}"; do
    src_file="$SOURCE_DIR/src/hooks/$hook"
    dest_file="$DEST_DIR/hooks/$hook"
    
    if [ -f "$src_file" ]; then
        if [ "$DRY_RUN" = true ]; then
            echo "  [DRY] Would copy: $hook"
        else
            cp "$src_file" "$dest_file"
            apply_replacements "$dest_file"
            echo -e "  ${GREEN}✓${NC} $hook"
        fi
    else
        echo -e "  ${YELLOW}⚠${NC} Not found: $hook"
    fi
done

# ============================================================================
# PHASE 6: Migrate types
# ============================================================================
echo -e "${BLUE}[Phase 6]${NC} Migrating types..."

if [ -f "$SOURCE_DIR/src/types.ts" ]; then
    if [ "$DRY_RUN" = true ]; then
        echo "  [DRY] Would copy: types.ts"
    else
        cp "$SOURCE_DIR/src/types.ts" "$DEST_DIR/types/index.ts"
        apply_replacements "$DEST_DIR/types/index.ts"
        echo -e "  ${GREEN}✓${NC} types.ts → types/index.ts"
    fi
fi

if [ -d "$SOURCE_DIR/src/types" ]; then
    for type_file in "$SOURCE_DIR/src/types"/*.ts; do
        if [ -f "$type_file" ]; then
            filename=$(basename "$type_file")
            if [ "$DRY_RUN" = true ]; then
                echo "  [DRY] Would copy: types/$filename"
            else
                cp "$type_file" "$DEST_DIR/types/$filename"
                apply_replacements "$DEST_DIR/types/$filename"
                echo -e "  ${GREEN}✓${NC} types/$filename"
            fi
        fi
    done
fi

# ============================================================================
# PHASE 7: Migrate lib utilities
# ============================================================================
echo -e "${BLUE}[Phase 7]${NC} Migrating lib utilities..."

if [ -d "$SOURCE_DIR/src/lib" ]; then
    for lib_file in "$SOURCE_DIR/src/lib"/*.ts; do
        if [ -f "$lib_file" ]; then
            filename=$(basename "$lib_file")
            if [ "$DRY_RUN" = true ]; then
                echo "  [DRY] Would copy: lib/$filename"
            else
                cp "$lib_file" "$DEST_DIR/lib/$filename"
                apply_replacements "$DEST_DIR/lib/$filename"
                echo -e "  ${GREEN}✓${NC} lib/$filename"
            fi
        fi
    done
fi

# ============================================================================
# PHASE 8: Create barrel exports
# ============================================================================
echo -e "${BLUE}[Phase 8]${NC} Creating barrel exports..."

if [ "$DRY_RUN" = false ]; then

# Main index.ts
cat > "$DEST_DIR/index.ts" << 'EOF'
/**
 * Synaptix9 Core Engine
 * Migrated from CTAS7 Command Center
 * 
 * This module provides the core orchestration, execution, and coordination
 * capabilities for Synaptix9 implementations.
 */

// Core Orchestrator
export { SX9Orchestrator } from './services/SX9Orchestrator';
export type { SX9SystemStatus, ConnectionTarget } from './services/SX9Orchestrator';

// Execution Engines
export { LegionExecutionEngine } from './services/LegionExecutionEngine';
export type { LegionTask, ExecutionContext } from './services/LegionExecutionEngine';

export { ScriptExecutionCoordinator } from './services/ScriptExecutionCoordinator';
export type { ScriptExecution, CoordinationPlan } from './services/ScriptExecutionCoordinator';

// Query Engines
export { SlotGraphQueryEngine, slotGraphQueryEngine } from './services/SlotGraphQueryEngine';

// Connectors
export { HashingEngineConnector } from './services/HashingEngineConnector';
export type { HashRequest, HashResponse, BatchHashRequest, BatchHashResponse } from './services/HashingEngineConnector';

// Hooks
export { useWebSocket } from './hooks/useWebSocket';
export { useVoiceConversation } from './hooks/useVoiceConversation';
export { useSystemDiagnostics } from './hooks/useSystemDiagnostics';

// Types
export * from './types';

// Version
export const SX9_CORE_VERSION = '1.0.0';
EOF

echo -e "  ${GREEN}✓${NC} Created index.ts"

# Services barrel
cat > "$DEST_DIR/services/index.ts" << 'EOF'
export { SX9Orchestrator } from './SX9Orchestrator';
export { LegionExecutionEngine } from './LegionExecutionEngine';
export { ScriptExecutionCoordinator } from './ScriptExecutionCoordinator';
export { SlotGraphQueryEngine, slotGraphQueryEngine } from './SlotGraphQueryEngine';
export { HashingEngineConnector } from './HashingEngineConnector';
export { SledKVStore } from './SledKVStore';
EOF

echo -e "  ${GREEN}✓${NC} Created services/index.ts"

# Hooks barrel
cat > "$DEST_DIR/hooks/index.ts" << 'EOF'
export { useWebSocket } from './useWebSocket';
export { useVoiceConversation } from './useVoiceConversation';
export { useSystemDiagnostics } from './useSystemDiagnostics';
export { useSupabaseData } from './useSupabaseData';
export { useBeamTelemetry } from './useBeamTelemetry';
export { useBeamPatterns } from './useBeamPatterns';
export { useRealtimeTelemetry } from './useRealtimeTelemetry';
EOF

echo -e "  ${GREEN}✓${NC} Created hooks/index.ts"

fi

# ============================================================================
# PHASE 9: Generate migration report
# ============================================================================
echo ""
echo -e "${BLUE}[Phase 9]${NC} Migration Summary"
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"

if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}DRY RUN - No files were modified${NC}"
    echo ""
    echo "To execute migration, run without --dry-run flag"
else
    # Count migrated files
    SERVICE_COUNT=$(ls -1 "$DEST_DIR/services"/*.ts 2>/dev/null | wc -l | tr -d ' ')
    HOOK_COUNT=$(ls -1 "$DEST_DIR/hooks"/*.ts 2>/dev/null | wc -l | tr -d ' ')
    TYPE_COUNT=$(ls -1 "$DEST_DIR/types"/*.ts 2>/dev/null | wc -l | tr -d ' ')
    
    echo -e "${GREEN}Migration Complete!${NC}"
    echo ""
    echo "Files migrated:"
    echo -e "  Services: ${GREEN}$SERVICE_COUNT${NC}"
    echo -e "  Hooks:    ${GREEN}$HOOK_COUNT${NC}"
    echo -e "  Types:    ${GREEN}$TYPE_COUNT${NC}"
    echo ""
    echo "Destination: $DEST_DIR"
fi

echo ""
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"

# ============================================================================
# ORCHESTRATOR DOCUMENTATION
# ============================================================================
cat << 'DOCS'

## What the SX9Orchestrator Does

The SX9Orchestrator is the **central nervous system** for any Synaptix9 implementation.
It coordinates 4 major subsystems:

┌─────────────────────────────────────────────────────────────────┐
│                      SX9Orchestrator                            │
│  ┌──────────────────┐  ┌──────────────────┐                    │
│  │ ScriptExecution  │  │ LegionExecution  │                    │
│  │   Coordinator    │  │     Engine       │                    │
│  │                  │  │                  │                    │
│  │ • Dependency DAG │  │ • HD4 Phases     │                    │
│  │ • Plan execution │  │ • 1n/2n Forms    │                    │
│  │ • Script routing │  │ • World routing  │                    │
│  └────────┬─────────┘  └────────┬─────────┘                    │
│           │                     │                               │
│  ┌────────┴─────────┐  ┌────────┴─────────┐                    │
│  │  DatabaseMux     │  │ HashingEngine    │                    │
│  │   Connector      │  │   Connector      │                    │
│  │                  │  │                  │                    │
│  │ • SurrealDB      │  │ • Murmur3        │                    │
│  │ • Supabase       │  │ • Batch hashing  │                    │
│  │ • Sled KV        │  │ • Compression    │                    │
│  │ • SlotGraph      │  │ • Base96 encode  │                    │
│  └──────────────────┘  └──────────────────┘                    │
└─────────────────────────────────────────────────────────────────┘

### Key Capabilities:

1. **Connection Management**
   - Tracks 14 connection targets (DBs, APIs, frontend, backend)
   - Dependency-aware status propagation
   - Health checking with automatic status updates

2. **Coordination Plans**
   - HD4 Connection Plan: Dashboard → Tasks → Views → DB
   - Legion Coordination: 1n adversary ↔ 2n counter-adversary

3. **HD4 Phase Model**
   - Hunt → Detect → Disrupt → Disable → Dominate
   - Tasks tagged by phase for workflow orchestration

4. **4-World Model**
   - Cyber, Geographical, Space, Maritime domains
   - Each world has its own task set and capabilities

5. **Hashing Integration**
   - Threat intelligence indicator hashing
   - Document fingerprinting for USIM
   - Legion task script verification

### Usage:

```typescript
import { SX9Orchestrator } from '@synaptix9/core';

const orchestrator = new SX9Orchestrator();
await orchestrator.initialize();
await orchestrator.startSystemConnection();

const status = await orchestrator.getSystemStatus();
console.log(status.overall); // 'connected' | 'connecting' | 'error'
```

DOCS
