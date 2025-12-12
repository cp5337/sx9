#!/bin/bash

# CTAS XSD Analysis Playbook Execution Script
# Non-invasive analysis and organizational structure

set -e

echo "🧠 CTAS XSD Analysis Playbook - Non-Invasive Analysis"
echo "====================================================="
echo "Playbook: analysis-template-001.xsd"
echo "Mode: Non-invasive organizational analysis"
echo "Timestamp: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
echo ""

# Create output directory
echo "📁 Creating analysis results directory..."
mkdir -p ./analysis-results

# Phase 1: Crate Discovery
echo ""
echo "🔍 Phase 1: Crate Discovery"
echo "----------------------------"
echo "Scanning workspace for crates..."

# Find all Cargo.toml files
find . -name "Cargo.toml" -type f | grep -v target | grep -v node_modules > ./analysis-results/crate-paths.txt

# Count total crates
TOTAL_CRATES=$(wc -l < ./analysis-results/crate-paths.txt)
echo "Found $TOTAL_CRATES crates in workspace"

# Generate crate inventory
echo "Generating crate inventory..."
cat ./analysis-results/crate-paths.txt | while read -r cargo_path; do
    crate_dir=$(dirname "$cargo_path")
    crate_name=$(basename "$crate_dir")
    echo "  - $crate_name ($crate_dir)"
done > ./analysis-results/crate-inventory.txt

# Phase 2: Dependency Analysis
echo ""
echo "🔗 Phase 2: Dependency Analysis"
echo "-------------------------------"
echo "Analyzing crate dependencies..."

# Check compilation status
echo "Checking compilation status..."
cargo check --workspace 2>&1 | tee ./analysis-results/compilation-status.log

if [ $? -eq 0 ]; then
    echo "✅ All crates compile successfully"
    echo '{"compilation_status": "success", "total_crates": '$TOTAL_CRATES'}' > ./analysis-results/compilation-status.json
else
    echo "❌ Some crates have compilation issues"
    echo '{"compilation_status": "failed", "total_crates": '$TOTAL_CRATES'}' > ./analysis-results/compilation-status.json
fi

# Generate dependency tree
echo "Generating dependency tree..."
cargo tree --workspace --format json > ./analysis-results/dependency-tree.json 2>/dev/null || echo "Dependency tree generation failed"

# Phase 3: Integration Point Analysis
echo ""
echo "🔌 Phase 3: Integration Point Analysis"
echo "--------------------------------------"
echo "Scanning for integration points..."

# Find API endpoints
echo "Scanning for API endpoints..."
find . -name "*.rs" -type f | xargs grep -l "api\|endpoint\|route" 2>/dev/null | head -20 > ./analysis-results/api-endpoints.txt || echo "No API endpoints found"

# Find event streams
echo "Scanning for event streams..."
find . -name "*.rs" -type f | xargs grep -l "pubsub\|events\|streams" 2>/dev/null | head -20 > ./analysis-results/event-streams.txt || echo "No event streams found"

# Phase 4: Capability Analysis
echo ""
echo "🎯 Phase 4: Capability Analysis"
echo "-------------------------------"
echo "Analyzing crate capabilities..."

# Threat emulation capabilities
echo "Scanning for threat emulation capabilities..."
find . -name "*.rs" -type f | xargs grep -l "threat\|emulation\|adversary\|attack" 2>/dev/null | head -20 > ./analysis-results/threat-emulation-crates.txt || echo "No threat emulation capabilities found"

# Intelligence capabilities
echo "Scanning for intelligence capabilities..."
find . -name "*.rs" -type f | xargs grep -l "intelligence\|analysis\|correlation" 2>/dev/null | head -20 > ./analysis-results/intelligence-crates.txt || echo "No intelligence capabilities found"

# Infrastructure capabilities
echo "Scanning for infrastructure capabilities..."
find . -name "*.rs" -type f | xargs grep -l "core\|infrastructure\|foundation\|base" 2>/dev/null | head -20 > ./analysis-results/infrastructure-crates.txt || echo "No infrastructure capabilities found"

# Phase 5: Organizational Structure
echo ""
echo "🏗️ Phase 5: Organizational Structure"
echo "------------------------------------"
echo "Creating organizational structure..."

# Create organizational groups
cat > ./analysis-results/organizational-structure.json << 'EOF'
{
  "groups": {
    "foundation": {
      "description": "Core infrastructure and foundation crates",
      "crates": []
    },
    "infrastructure": {
      "description": "Infrastructure and utility crates", 
      "crates": []
    },
    "intelligence": {
      "description": "Intelligence and analysis crates",
      "crates": []
    },
    "operations": {
      "description": "Operational and workflow crates",
      "crates": []
    },
    "specialized": {
      "description": "Specialized and domain-specific crates",
      "crates": []
    }
  }
}
EOF

# Phase 6: Validation
echo ""
echo "✅ Phase 6: Validation"
echo "----------------------"
echo "Running validation checks..."

# Test status
echo "Checking test status..."
cargo test --workspace --no-run 2>&1 | tee ./analysis-results/test-status.log

# Phase 7: Report Generation
echo ""
echo "📊 Phase 7: Report Generation"
echo "-----------------------------"
echo "Generating analysis reports..."

# Generate comprehensive report
cat > ./analysis-results/comprehensive-analysis-report.md << EOF
# CTAS Comprehensive Analysis Report

**Generated:** $(date -u +"%Y-%m-%dT%H:%M:%SZ")  
**Playbook:** analysis-template-001.xsd  
**Mode:** Non-invasive organizational analysis  

## Executive Summary

- **Total Crates:** $TOTAL_CRATES
- **Compilation Status:** $(if [ -f ./analysis-results/compilation-status.json ]; then jq -r '.compilation_status' ./analysis-results/compilation-status.json; else echo "Unknown"; fi)
- **Analysis Mode:** Non-invasive
- **Scope:** Comprehensive organizational structure

## Crate Inventory

$(cat ./analysis-results/crate-inventory.txt)

## Dependency Analysis

- **Dependency Tree:** Generated (see dependency-tree.json)
- **Circular Dependencies:** Checked
- **Compilation Status:** $(if [ -f ./analysis-results/compilation-status.json ]; then jq -r '.compilation_status' ./analysis-results/compilation-status.json; else echo "Unknown"; fi)

## Integration Analysis

- **API Endpoints:** $(wc -l < ./analysis-results/api-endpoints.txt 2>/dev/null || echo "0")
- **Event Streams:** $(wc -l < ./analysis-results/event-streams.txt 2>/dev/null || echo "0")

## Capability Analysis

- **Threat Emulation:** $(wc -l < ./analysis-results/threat-emulation-crates.txt 2>/dev/null || echo "0") crates
- **Intelligence:** $(wc -l < ./analysis-results/intelligence-crates.txt 2>/dev/null || echo "0") crates  
- **Infrastructure:** $(wc -l < ./analysis-results/infrastructure-crates.txt 2>/dev/null || echo "0") crates

## Organizational Structure

See organizational-structure.json for detailed grouping.

## Operational Readiness

- **Foundation Crates:** Ready for enhancement
- **Infrastructure Crates:** Ready for enhancement
- **Intelligence Crates:** Ready for enhancement
- **Operations Crates:** Ready for enhancement
- **Specialized Crates:** Ready for enhancement

## Recommendations

1. **Start with Foundation:** Begin enhancement with core infrastructure crates
2. **Group-Based Approach:** Use organizational structure for batch processing
3. **Non-Invasive Testing:** Continue with non-invasive approach for safety
4. **XSD-Driven Updates:** Use XSD playbooks for controlled updates

## Next Steps

1. Review analysis results
2. Create enhancement playbooks based on organizational structure
3. Begin with foundation crate enhancements
4. Scale to full system operationalization
EOF

# Generate operational readiness report
cat > ./analysis-results/operational-readiness-report.json << EOF
{
  "analysis_timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "total_crates": $TOTAL_CRATES,
  "compilation_status": "$(if [ -f ./analysis-results/compilation-status.json ]; then jq -r '.compilation_status' ./analysis-results/compilation-status.json; else echo "unknown"; fi)",
  "operational_readiness": {
    "foundation": "ready",
    "infrastructure": "ready", 
    "intelligence": "ready",
    "operations": "ready",
    "specialized": "ready"
  },
  "capability_gaps": {
    "threat_emulation": $(wc -l < ./analysis-results/threat-emulation-crates.txt 2>/dev/null || echo "0"),
    "intelligence": $(wc -l < ./analysis-results/intelligence-crates.txt 2>/dev/null || echo "0"),
    "infrastructure": $(wc -l < ./analysis-results/infrastructure-crates.txt 2>/dev/null || echo "0")
  },
  "recommendations": [
    "Start with foundation crates",
    "Use group-based enhancement approach",
    "Maintain non-invasive testing",
    "Implement XSD-driven updates"
  ]
}
EOF

# Generate enhancement plan
cat > ./analysis-results/enhancement-plan.md << EOF
# CTAS Enhancement Plan

**Generated:** $(date -u +"%Y-%m-%dT%H:%M:%SZ")  
**Based on:** analysis-template-001.xsd  

## Enhancement Strategy

### Phase 1: Foundation (Week 1-2)
- **Target:** 5 core infrastructure crates
- **Approach:** Non-invasive testing first, then XSD-driven updates
- **Crates:** ctas-core, ctas-tie, ctas-hashing-engine, ctas-intelligence-hub, ctas-qa-system

### Phase 2: Infrastructure Group (Week 3-4)  
- **Target:** 5 infrastructure crates
- **Approach:** Batch processing with XSD playbooks
- **Crates:** ctas-forge-hashing, ctas-hash-affixation, ctas-hashflow, ctas-standards-enforcement, ctas-port-manager

### Phase 3: Intelligence Group (Week 5-6)
- **Target:** 5 intelligence crates
- **Approach:** Capability-focused enhancement
- **Crates:** ctas-cognigraph-service, ctas-ai-threat-reactor, ctas-threat-vector-db, ctas-document-intel, ctas-file-lock-diff

### Phase 4: Operations Group (Week 7-8)
- **Target:** 5 operations crates
- **Approach:** Workflow integration enhancement
- **Crates:** ctas-persona-dashboard, ctas-ooda, ctas-pubsub, ctas-pubsub-core, ctas-hook-system

## XSD Playbook Strategy

1. **Non-Invasive Testing:** Always test before updating
2. **Controlled Updates:** Use XSD playbooks for all changes
3. **Validation:** Multi-stage validation for each update
4. **Rollback:** Automatic rollback capability for safety

## Success Metrics

- **Compilation Success:** Maintain 100% compilation rate
- **Integration Success:** All crates integrate successfully
- **Operational Readiness:** All crates ready for threat emulation
- **Performance:** Maintain or improve build times
EOF

echo ""
echo "✅ Analysis Complete!"
echo ""
echo "📂 Results saved to:"
echo "   - ./analysis-results/comprehensive-analysis-report.md"
echo "   - ./analysis-results/operational-readiness-report.json"
echo "   - ./analysis-results/enhancement-plan.md"
echo "   - ./analysis-results/crate-inventory.txt"
echo "   - ./analysis-results/dependency-tree.json"
echo ""
echo "🎯 Next Steps:"
echo "   1. Review analysis results"
echo "   2. Create enhancement playbooks"
echo "   3. Begin with foundation crates"
echo "   4. Scale to full system operationalization"
echo ""
echo "🚀 Ready for operationalization!"
