#!/bin/bash

# CTAS Group Operations Executor
# Execute cargo operations on grouped crates using XSD orchestration

set -e

echo "🏗️ CTAS Group Operations Executor"
echo "=================================="
echo "Mode: XSD-driven group operations"
echo "Timestamp: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
echo ""

# Configuration
OPERATIONS_DIR="./group-operations"
RESULTS_DIR="./operation-results"
LOGS_DIR="./operation-logs"

# Create directories
mkdir -p "$OPERATIONS_DIR" "$RESULTS_DIR" "$LOGS_DIR"

# Group definitions
declare -A GROUPS=(
    ["foundation"]="ctas-core ctas-tie ctas-port-manager ctas-hook-system ctas-qa-system ctas-standards-enforcement ctas-xsd-environment ctas-forge-hashing ctas-genetic-hash ctas-hashflow ctas-hashing-engine ctas-hash-affixation"
    ["intelligence"]="ctas-intelligence-hub ctas-document-intel ctas-threat-vector-db cyber-intelligence-platform ctas-gnn-standalone ctas-gnn-core ctas-gnn-utils ctas-gnn-models ctas-gnn-training ctas-gnn-inference ctas-gnn-visualization ctas-gnn-integration"
    ["operations"]="ctas-tie ctas-port-manager ctas-hook-system ctas-persona-dashboard ctas-honeypot-engine ctas-fratricide-detection Deception-Platform Financial-Intelligence-Blockchain Financial-Warfare-Detection ctas-scenario-engine ctas-mission-planner ctas-tactical-coordinator"
    ["specialized"]="ctas-gis-engine ctas-geospatial-analysis ctas-map-integration ctas-location-services ctas-spatial-intelligence ctas-financial-intelligence ctas-blockchain-analysis ctas-cryptocurrency-tracking ctas-economic-warfare ctas-deception-engine ctas-honeypot-manager ctas-threat-deception"
    ["testing"]="ctas-qa-system ctas-standards-enforcement ctas-xsd-environment integration_test ctas-test-framework ctas-integration-tests ctas-unit-tests ctas-performance-tests ctas-security-tests"
    ["ai-cli"]="AI-CLI ctas-ai-engine ctas-cli-interface ctas-automation-engine ctas-ai-coordinator ctas-intelligent-automation"
    ["tools"]="tools ctas-utility-tools ctas-development-tools ctas-helper-functions ctas-build-tools ctas-deployment-tools"
    ["infrastructure"]="ctas-frontend-port-block ctas-repo-prompt ctas-document-intel ctas-unified-intelligence ctas-threat-vector-db cyber-intelligence-platform ctas-persona-dashboard ctas-honeypot-engine ctas-fratricide-detection Deception-Platform Financial-Intelligence-Blockchain Financial-Warfare-Detection"
)

# Operation types
declare -A OPERATIONS=(
    ["check"]="cargo check"
    ["build"]="cargo build"
    ["test"]="cargo test"
    ["clippy"]="cargo clippy"
    ["doc"]="cargo doc"
    ["clean"]="cargo clean"
    ["update"]="cargo update"
)

# Function to execute operation on a group
execute_group_operation() {
    local group_name="$1"
    local operation="$2"
    local parallel="$3"
    
    echo "🎯 Executing $operation on $group_name group..."
    
    local crates="${GROUPS[$group_name]}"
    local operation_cmd="${OPERATIONS[$operation]}"
    local log_file="$LOGS_DIR/${group_name}_${operation}.log"
    local result_file="$RESULTS_DIR/${group_name}_${operation}.json"
    
    # Create result structure
    cat > "$result_file" << EOF
{
  "group_name": "$group_name",
  "operation": "$operation",
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "parallel": $parallel,
  "crates": [],
  "summary": {
    "total_crates": 0,
    "successful": 0,
    "failed": 0,
    "skipped": 0
  }
}
EOF
    
    local total_crates=0
    local successful=0
    local failed=0
    local skipped=0
    
    # Execute operation on each crate
    for crate in $crates; do
        total_crates=$((total_crates + 1))
        
        if [ -d "$crate" ] && [ -f "$crate/Cargo.toml" ]; then
            echo "  📦 Processing $crate..."
            
            if [ "$parallel" = "true" ]; then
                # Parallel execution
                (
                    cd "$crate"
                    if $operation_cmd > "$log_file.$crate" 2>&1; then
                        echo "    ✅ $crate: SUCCESS"
                        echo "{\"crate\": \"$crate\", \"status\": \"success\", \"log\": \"$log_file.$crate\"}" >> "$result_file.tmp"
                        successful=$((successful + 1))
                    else
                        echo "    ❌ $crate: FAILED"
                        echo "{\"crate\": \"$crate\", \"status\": \"failed\", \"log\": \"$log_file.$crate\"}" >> "$result_file.tmp"
                        failed=$((failed + 1))
                    fi
                ) &
            else
                # Sequential execution
                (
                    cd "$crate"
                    if $operation_cmd > "$log_file.$crate" 2>&1; then
                        echo "    ✅ $crate: SUCCESS"
                        echo "{\"crate\": \"$crate\", \"status\": \"success\", \"log\": \"$log_file.$crate\"}" >> "$result_file.tmp"
                        successful=$((successful + 1))
                    else
                        echo "    ❌ $crate: FAILED"
                        echo "{\"crate\": \"$crate\", \"status\": \"failed\", \"log\": \"$log_file.$crate\"}" >> "$result_file.tmp"
                        failed=$((failed + 1))
                    fi
                )
            fi
        else
            echo "  ⚠️  Skipping $crate (not found)"
            skipped=$((skipped + 1))
        fi
    done
    
    # Wait for parallel operations to complete
    if [ "$parallel" = "true" ]; then
        wait
    fi
    
    # Update result file with summary
    cat > "$result_file" << EOF
{
  "group_name": "$group_name",
  "operation": "$operation",
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "parallel": $parallel,
  "summary": {
    "total_crates": $total_crates,
    "successful": $successful,
    "failed": $failed,
    "skipped": $skipped
  },
  "crates": [
EOF
    
    if [ -f "$result_file.tmp" ]; then
        cat "$result_file.tmp" >> "$result_file"
        rm "$result_file.tmp"
    fi
    
    echo "  ]" >> "$result_file"
    echo "}" >> "$result_file"
    
    echo "📊 $group_name group $operation complete: $successful/$total_crates successful"
}

# Function to execute operation on all groups
execute_all_groups_operation() {
    local operation="$1"
    local parallel="$2"
    
    echo "🚀 Executing $operation on all groups..."
    
    local all_results_file="$RESULTS_DIR/all_groups_${operation}.json"
    
    # Create all groups result structure
    cat > "$all_results_file" << EOF
{
  "operation": "$operation",
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "parallel": $parallel,
  "groups": []
}
EOF
    
    local total_groups=0
    local total_crates=0
    local total_successful=0
    local total_failed=0
    
    # Execute on each group
    for group in "${!GROUPS[@]}"; do
        total_groups=$((total_groups + 1))
        echo ""
        execute_group_operation "$group" "$operation" "$parallel"
        
        # Read group results
        local group_result_file="$RESULTS_DIR/${group}_${operation}.json"
        if [ -f "$group_result_file" ]; then
            local group_successful=$(jq -r '.summary.successful' "$group_result_file")
            local group_failed=$(jq -r '.summary.failed' "$group_result_file")
            local group_total=$(jq -r '.summary.total_crates' "$group_result_file")
            
            total_crates=$((total_crates + group_total))
            total_successful=$((total_successful + group_successful))
            total_failed=$((total_failed + group_failed))
        fi
    done
    
    # Update all groups result
    cat > "$all_results_file" << EOF
{
  "operation": "$operation",
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "parallel": $parallel,
  "summary": {
    "total_groups": $total_groups,
    "total_crates": $total_crates,
    "total_successful": $total_successful,
    "total_failed": $total_failed
  },
  "groups": [
EOF
    
    for group in "${!GROUPS[@]}"; do
        local group_result_file="$RESULTS_DIR/${group}_${operation}.json"
        if [ -f "$group_result_file" ]; then
            cat "$group_result_file" >> "$all_results_file"
            echo "," >> "$all_results_file"
        fi
    done
    
    # Remove trailing comma and close
    sed -i '' '$ s/,$//' "$all_results_file"
    echo "  ]" >> "$all_results_file"
    echo "}" >> "$all_results_file"
    
    echo ""
    echo "🎉 All groups $operation complete!"
    echo "📊 Summary: $total_successful/$total_crates crates successful across $total_groups groups"
}

# Function to show available operations
show_operations() {
    echo "Available operations:"
    for op in "${!OPERATIONS[@]}"; do
        echo "  - $op: ${OPERATIONS[$op]}"
    done
    echo ""
    echo "Available groups:"
    for group in "${!GROUPS[@]}"; do
        local crate_count=$(echo "${GROUPS[$group]}" | wc -w)
        echo "  - $group ($crate_count crates)"
    done
}

# Function to generate operation report
generate_report() {
    local operation="$1"
    local report_file="$RESULTS_DIR/${operation}_report.md"
    
    echo "📊 Generating $operation report..."
    
    cat > "$report_file" << EOF
# CTAS Group Operations Report - $operation

**Generated:** $(date -u +"%Y-%m-%dT%H:%M:%SZ")  
**Operation:** $operation  
**Mode:** XSD-driven group operations  

## Executive Summary

EOF
    
    # Read all groups result
    local all_results_file="$RESULTS_DIR/all_groups_${operation}.json"
    if [ -f "$all_results_file" ]; then
        local total_groups=$(jq -r '.summary.total_groups' "$all_results_file")
        local total_crates=$(jq -r '.summary.total_crates' "$all_results_file")
        local total_successful=$(jq -r '.summary.total_successful' "$all_results_file")
        local total_failed=$(jq -r '.summary.total_failed' "$all_results_file")
        
        cat >> "$report_file" << EOF
- **Total Groups:** $total_groups
- **Total Crates:** $total_crates
- **Successful:** $total_successful
- **Failed:** $total_failed
- **Success Rate:** $((total_successful * 100 / total_crates))%

## Group Results

EOF
        
        for group in "${!GROUPS[@]}"; do
            local group_result_file="$RESULTS_DIR/${group}_${operation}.json"
            if [ -f "$group_result_file" ]; then
                local group_successful=$(jq -r '.summary.successful' "$group_result_file")
                local group_failed=$(jq -r '.summary.failed' "$group_result_file")
                local group_total=$(jq -r '.summary.total_crates' "$group_result_file")
                local success_rate=$((group_successful * 100 / group_total))
                
                cat >> "$report_file" << EOF
### $group Group
- **Crates:** $group_total
- **Successful:** $group_successful
- **Failed:** $group_failed
- **Success Rate:** $success_rate%

EOF
            fi
        done
    fi
    
    cat >> "$report_file" << EOF

## Recommendations

1. **Review Failed Crates:** Investigate crates that failed the operation
2. **Optimize Dependencies:** Check for dependency conflicts
3. **Update Documentation:** Ensure all crates have proper documentation
4. **Run Tests:** Execute comprehensive testing on successful builds

## Next Steps

1. **Fix Compilation Issues:** Address any compilation errors
2. **Run Integration Tests:** Test group interactions
3. **Performance Analysis:** Analyze build and test performance
4. **Deployment Preparation:** Prepare for production deployment
EOF
    
    echo "✅ Report generated: $report_file"
}

# Main execution logic
case "${1:-help}" in
    "check"|"build"|"test"|"clippy"|"doc"|"clean"|"update")
        operation="$1"
        group="${2:-all}"
        parallel="${3:-false}"
        
        if [ "$group" = "all" ]; then
            execute_all_groups_operation "$operation" "$parallel"
        else
            if [ -n "${GROUPS[$group]}" ]; then
                execute_group_operation "$group" "$operation" "$parallel"
            else
                echo "❌ Unknown group: $group"
                echo "Available groups: ${!GROUPS[*]}"
                exit 1
            fi
        fi
        
        # Generate report
        generate_report "$operation"
        ;;
    "list")
        show_operations
        ;;
    "report")
        operation="${2:-check}"
        generate_report "$operation"
        ;;
    "help"|*)
        echo "Usage: $0 <operation> [group] [parallel]"
        echo ""
        echo "Operations:"
        show_operations
        echo ""
        echo "Examples:"
        echo "  $0 check foundation false    # Run cargo check on foundation group sequentially"
        echo "  $0 build intelligence true   # Run cargo build on intelligence group in parallel"
        echo "  $0 test all true             # Run cargo test on all groups in parallel"
        echo "  $0 list                      # Show available operations and groups"
        echo "  $0 report check              # Generate report for check operation"
        ;;
esac

echo ""
echo "✅ Group operations complete!"
echo "📂 Results saved to: $RESULTS_DIR"
echo "📋 Logs saved to: $LOGS_DIR"
