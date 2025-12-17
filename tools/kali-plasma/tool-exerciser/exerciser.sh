#!/bin/bash
# SX9 Tool Exerciser - Cycles through tools and captures output
# Outputs are hashed and formatted for the SX9 pipeline

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Directories
OUTPUT_DIR="/sx9/output"
CORPUS_DIR="${CORPUS_DIR:-/sx9/corpus}"
RAW_DIR="$OUTPUT_DIR/raw"
PARSED_DIR="$OUTPUT_DIR/parsed"
HASHED_DIR="$OUTPUT_DIR/hashed"
LOG_DIR="/sx9/logs"
CONFIG_DIR="/sx9/config"

# Config
PROFILES_FILE="$CONFIG_DIR/tool_profiles.toml"
MANIFEST_FILE="$OUTPUT_DIR/manifest.json"

# Runtime
TIER="${TIER:-0}"
DRY_RUN="${DRY_RUN:-false}"
VERBOSE="${VERBOSE:-false}"
NATS_URL="${NATS_URL:-}"
SEQUENCE=0

# ============================================================================
# FUNCTIONS
# ============================================================================

log() {
    echo -e "${CYAN}[$(date '+%Y-%m-%d %H:%M:%S')]${NC} $1"
}

log_success() {
    echo -e "${GREEN}✓${NC} $1"
}

log_error() {
    echo -e "${RED}✗${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}⚠${NC} $1"
}

show_help() {
    cat << 'EOF'
SX9 Tool Exerciser - Cycle through Kali tools and capture outputs

USAGE:
    exerciser.sh [OPTIONS] [COMMAND]

COMMANDS:
    run         Run the exerciser (default)
    list        List available tools by tier
    single      Run a single tool
    manifest    Show output manifest

OPTIONS:
    -t, --tier TIER     Tool tier (0=help, 1=localhost, 2=passive, 3=synthetic, 4=password)
    -d, --dry-run       Show what would run without executing
    -v, --verbose       Verbose output
    -n, --nats URL      NATS server URL for streaming results
    -h, --help          Show this help

ENVIRONMENT:
    TIER        Default tier (0-4)
    DRY_RUN     Set to 'true' for dry run
    NATS_URL    NATS server URL

EXAMPLES:
    # Run tier 0 (help only - safest)
    exerciser.sh -t 0 run

    # Run tier 1 (localhost/scanme.nmap.org)
    exerciser.sh -t 1 run

    # Dry run tier 2
    exerciser.sh -t 2 --dry-run run

    # Single tool
    exerciser.sh single nmap "nmap -sV scanme.nmap.org"

    # Stream to NATS
    exerciser.sh -n nats://localhost:4222 -t 1 run

EOF
}

# Initialize output directories
init_output() {
    mkdir -p "$RAW_DIR" "$PARSED_DIR" "$HASHED_DIR" "$LOG_DIR"
    
    # Initialize corpus directories if mounted
    if [ -d "$CORPUS_DIR" ]; then
        mkdir -p "$CORPUS_DIR/raw" "$CORPUS_DIR/parsed" "$CORPUS_DIR/hashed" "$CORPUS_DIR/index"
        log "Corpus directory: $CORPUS_DIR"
    fi
    
    # Initialize manifest
    cat > "$MANIFEST_FILE" << EOF
{
  "version": "1.0.0",
  "started": "$(date -Iseconds)",
  "tier": $TIER,
  "outputs": []
}
EOF
    
    log "Initialized output directories"
}

# Generate short code for tool output
generate_short_code() {
    local tool_name="$1"
    local hash_prefix="$2"
    
    # Get tool prefix (first 3 chars uppercase)
    local prefix
    case "$tool_name" in
        nmap*) prefix="NMP" ;;
        masscan*) prefix="MSC" ;;
        nikto*) prefix="NKT" ;;
        nuclei*) prefix="NUC" ;;
        sqlmap*) prefix="SQL" ;;
        john*) prefix="JHN" ;;
        hashcat*) prefix="HSH" ;;
        gobuster*) prefix="GOB" ;;
        amass*) prefix="AMS" ;;
        subfinder*) prefix="SBF" ;;
        harvester*|theharvester*) prefix="HRV" ;;
        whatweb*) prefix="WWB" ;;
        *) prefix=$(echo "$tool_name" | cut -c1-3 | tr '[:lower:]' '[:upper:]') ;;
    esac
    
    # Combine with hash suffix
    echo "${prefix}${hash_prefix}"
}

# Murmur3-64 hash function (portable implementation)
# Uses xxhash as fallback since murmur3 may not be available
murmur3_64() {
    local data="$1"
    local seed="$2"
    
    # Try xxhash first (common on Linux), fallback to md5 truncated
    if command -v xxh64sum &> /dev/null; then
        echo -n "$data" | xxh64sum | cut -c1-16
    elif command -v xxhsum &> /dev/null; then
        echo -n "$data" | xxhsum -H64 | cut -c1-16
    else
        # Fallback: MD5 truncated to 16 hex chars (64 bits)
        echo -n "${seed}${data}" | md5sum | cut -c1-16
    fi
}

# Generate dual trivariate hash (RFC-9001 compliant)
# Returns: h1 (semantic) and h2 (operational) as hex strings
generate_trivariate_hash() {
    local tool_name="$1"
    local output_file="$2"
    local parser="$3"
    local request_id="$4"
    local timestamp="$5"
    
    # h1: Semantic hash (SCH) - what the output MEANS
    # Seed: 0xC7A50000
    local semantic_input="${tool_name}|${parser}|$(head -c 256 "$output_file" | base64 -w0 2>/dev/null || base64)"
    local h1
    h1=$(murmur3_64 "$semantic_input" "C7A50000")
    
    # h2: Operational hash (CUID) - what TRIGGERED it  
    # Seed: 0xC7A50001
    local operational_input="${request_id}|${timestamp}|${SEQUENCE}"
    local h2
    h2=$(murmur3_64 "$operational_input" "C7A50001")
    
    echo "${h1}|${h2}"
}

# Hash output and create metadata
hash_output() {
    local tool_name="$1"
    local output_file="$2"
    local parser="$3"
    local duration="$4"
    local exit_code="$5"
    
    # Generate request ID (would come from trigger in real system)
    local request_id="REQ$(date +%s%N | cut -c1-8)"
    local timestamp
    timestamp=$(date -Iseconds)
    
    # Generate dual trivariate hash (h1|h2)
    local trivariate
    trivariate=$(generate_trivariate_hash "$tool_name" "$output_file" "$parser" "$request_id" "$timestamp")
    local h1="${trivariate%%|*}"
    local h2="${trivariate##*|}"
    
    # Also keep content hash for dedup
    local content_hash
    content_hash=$(sha256sum "$output_file" 2>/dev/null | cut -c1-64 || shasum -a 256 "$output_file" | cut -c1-64)
    
    # Short code from h1 (semantic hash)
    local hash_prefix
    hash_prefix=$(echo "$h1" | cut -c1-5 | tr '[:lower:]' '[:upper:]')
    
    # Generate short code
    local short_code
    short_code=$(generate_short_code "$tool_name" "$hash_prefix")
    
    # File size
    local file_size
    file_size=$(stat -c%s "$output_file" 2>/dev/null || stat -f%z "$output_file")
    
    # Size class rune
    local size_rune
    if [ "$file_size" -lt 256 ]; then
        size_rune="0xE910"  # tiny
    elif [ "$file_size" -lt 4096 ]; then
        size_rune="0xE911"  # small
    elif [ "$file_size" -lt 65536 ]; then
        size_rune="0xE912"  # medium
    elif [ "$file_size" -lt 1048576 ]; then
        size_rune="0xE913"  # large
    else
        size_rune="0xE914"  # xlarge
    fi
    
    # Sequence number
    SEQUENCE=$((SEQUENCE + 1))
    
    # Get parser rune from tool
    local parser_rune="0xE92F"  # default: generic
    case "$parser" in
        nmap_xml) parser_rune="0xE920" ;;
        nmap_grep) parser_rune="0xE921" ;;
        masscan_json) parser_rune="0xE922" ;;
        nuclei_json) parser_rune="0xE923" ;;
        nikto_json) parser_rune="0xE924" ;;
        sqlmap_json) parser_rune="0xE925" ;;
        hashcat_pot) parser_rune="0xE926" ;;
        john_pot) parser_rune="0xE927" ;;
        harvester_json) parser_rune="0xE928" ;;
        help) parser_rune="0xE908" ;;  # stdout
    esac
    
    # Get trigger rune from tool
    local trigger_rune="0xE000"  # default: nmap
    case "$tool_name" in
        nmap*) trigger_rune="0xE000" ;;
        masscan*) trigger_rune="0xE001" ;;
        nikto*) trigger_rune="0xE010" ;;
        nuclei*) trigger_rune="0xE011" ;;
        sqlmap*) trigger_rune="0xE020" ;;
        john*) trigger_rune="0xE030" ;;
        hashcat*) trigger_rune="0xE031" ;;
        gobuster*) trigger_rune="0xE012" ;;
        amass*) trigger_rune="0xE040" ;;
        subfinder*) trigger_rune="0xE041" ;;
        theharvester*|harvester*) trigger_rune="0xE042" ;;
    esac

    # Create h1 semantic file (TOML - what it MEANS)
    local h1_file="$HASHED_DIR/${short_code}.h1.toml"
    cat > "$h1_file" << EOF
# h1 - Semantic Hash (SCH)
# What the output MEANS - design time / configuration
# Seed: 0xC7A50000

[semantic]
short_code = "$short_code"
hash = "$h1"
seed = "0xC7A50000"

[tool]
name = "$tool_name"
trigger_rune = "$trigger_rune"
parser = "$parser"
parser_rune = "$parser_rune"

[content]
format_rune = "0xE908"
size_rune = "$size_rune"
size_bytes = $file_size
content_hash = "$content_hash"
EOF

    # Create h2 operational file (JSON - what TRIGGERED it)
    local h2_file="$HASHED_DIR/${short_code}.h2.json"
    cat > "$h2_file" << EOF
{
  "operational": {
    "short_code": "$short_code",
    "hash": "$h2",
    "seed": "0xC7A50001"
  },
  "request": {
    "id": "$request_id",
    "timestamp": "$timestamp",
    "sequence": $SEQUENCE
  },
  "execution": {
    "duration_ms": $duration,
    "exit_code": $exit_code,
    "env": "kali-plasma"
  },
  "raw_file": "$(basename "$output_file")"
}
EOF

    # Create combined hash metadata (references both)
    local hash_file="$HASHED_DIR/${short_code}.json"
    cat > "$hash_file" << EOF
{
  "short_code": "$short_code",
  "tool": "$tool_name",
  "hash_ref": {
    "h1_semantic": "$h1",
    "h1_format": "toml",
    "h1_file": "${short_code}.h1.toml",
    "h2_operational": "$h2",
    "h2_format": "json",
    "h2_file": "${short_code}.h2.json"
  },
  "heredity": {
    "operator": "0xE801",
    "operator_name": "cons",
    "expr": "(cons $h1 $h2)",
    "comment": "h1=TOML(semantic) . h2=JSON(operational)"
  }
}
EOF
    
    # Append to manifest
    local manifest_entry
    manifest_entry=$(cat << EOF
    {
      "short_code": "$short_code",
      "tool": "$tool_name",
      "hash": "$content_hash",
      "size": $file_size
    }
EOF
)
    
    # Update manifest (append to outputs array)
    # Using temp file to avoid jq dependency issues
    echo "$manifest_entry" >> "$OUTPUT_DIR/outputs.jsonl"
    
    # Copy to corpus if mounted
    if [ -d "$CORPUS_DIR" ]; then
        # Create tool subdirectory
        local tool_dir="$CORPUS_DIR/raw/$tool_name"
        mkdir -p "$tool_dir"
        
        # Copy raw output (compress if large)
        if [ "$file_size" -gt 4096 ]; then
            # Compress with zstd if available, else gzip
            if command -v zstd &> /dev/null; then
                zstd -q "$output_file" -o "$tool_dir/${short_code}.zst" 2>/dev/null || cp "$output_file" "$tool_dir/${short_code}.raw"
            else
                gzip -c "$output_file" > "$tool_dir/${short_code}.gz" 2>/dev/null || cp "$output_file" "$tool_dir/${short_code}.raw"
            fi
        else
            cp "$output_file" "$tool_dir/${short_code}.raw"
        fi
        
        # Copy hash metadata
        cp "$hash_file" "$CORPUS_DIR/hashed/"
    fi
    
    echo "$short_code"
}

# Execute a single tool and capture output
execute_tool() {
    local name="$1"
    local cmd="$2"
    local parser="${3:-generic}"
    local timeout="${4:-60}"
    
    log "Executing: ${BLUE}$name${NC}"
    
    if [ "$VERBOSE" = "true" ]; then
        log "  Command: $cmd"
        log "  Parser: $parser"
        log "  Timeout: ${timeout}s"
    fi
    
    if [ "$DRY_RUN" = "true" ]; then
        log_warn "  [DRY RUN] Would execute: $cmd"
        return 0
    fi
    
    # Output file
    local output_file="$RAW_DIR/${name}_$(date +%s).out"
    local log_file="$LOG_DIR/${name}_$(date +%s).log"
    
    # Execute with timeout
    local start_time
    start_time=$(date +%s%3N)
    local exit_code=0
    
    if timeout "$timeout" bash -c "$cmd" > "$output_file" 2> "$log_file"; then
        exit_code=0
    else
        exit_code=$?
    fi
    
    local end_time
    end_time=$(date +%s%3N)
    local duration=$((end_time - start_time))
    
    # Check if we got output
    if [ ! -s "$output_file" ]; then
        # Try stderr as output
        if [ -s "$log_file" ]; then
            mv "$log_file" "$output_file"
        else
            echo "No output captured" > "$output_file"
        fi
    fi
    
    # Hash and create metadata
    local short_code
    short_code=$(hash_output "$name" "$output_file" "$parser" "$duration" "$exit_code")
    
    if [ "$exit_code" -eq 0 ]; then
        log_success "  → $short_code (${duration}ms, $(stat -c%s "$output_file" 2>/dev/null || stat -f%z "$output_file") bytes)"
    else
        log_error "  → $short_code (exit: $exit_code, ${duration}ms)"
    fi
    
    # Stream to NATS if configured
    if [ -n "$NATS_URL" ]; then
        stream_to_nats "$short_code" "$output_file"
    fi
    
    return $exit_code
}

# Stream result to NATS
stream_to_nats() {
    local short_code="$1"
    local output_file="$2"
    
    if command -v nats &> /dev/null; then
        # Publish to NATS
        local subject="sx9.tool.response.${short_code}"
        cat "$HASHED_DIR/${short_code}.json" | nats pub "$subject" --server="$NATS_URL" 2>/dev/null || true
        
        if [ "$VERBOSE" = "true" ]; then
            log "  → Published to NATS: $subject"
        fi
    fi
}

# Run tier 0: Help only
run_tier_0() {
    log "Running Tier 0: Help/Version outputs"
    
    local tools=(
        "nmap:nmap --help"
        "masscan:masscan --help"
        "nikto:nikto -H"
        "gobuster:gobuster --help"
        "ffuf:ffuf -h"
        "nuclei:nuclei -h"
        "amass:amass -h"
        "subfinder:subfinder -h"
        "whatweb:whatweb --help"
        "dirb:dirb"
        "whois:whois --help"
        "dig:dig -h"
    )
    
    local success=0
    local failed=0
    
    for tool_spec in "${tools[@]}"; do
        local name="${tool_spec%%:*}"
        local cmd="${tool_spec#*:}"
        
        if execute_tool "$name" "$cmd" "help" 10; then
            ((success++))
        else
            ((failed++))
        fi
    done
    
    log "Tier 0 complete: ${GREEN}$success success${NC}, ${RED}$failed failed${NC}"
}

# Run tier 1: Localhost/scanme.nmap.org
run_tier_1() {
    log "Running Tier 1: Localhost and scanme.nmap.org"
    
    local tools=(
        "nmap_version:nmap -sV -p22,80,443 scanme.nmap.org -oX -:nmap_xml:60"
        "nmap_quick:nmap -F scanme.nmap.org -oG -:nmap_grep:30"
        "nmap_localhost:nmap -sT -p1-100 127.0.0.1 -oX -:nmap_xml:30"
        "ping:ping -c 3 scanme.nmap.org:ping:15"
        "dig:dig scanme.nmap.org ANY +noall +answer:dns:10"
        "whois:whois nmap.org:whois:30"
    )
    
    local success=0
    local failed=0
    
    for tool_spec in "${tools[@]}"; do
        IFS=':' read -r name cmd parser timeout <<< "$tool_spec"
        
        if execute_tool "$name" "$cmd" "$parser" "$timeout"; then
            ((success++))
        else
            ((failed++))
        fi
    done
    
    log "Tier 1 complete: ${GREEN}$success success${NC}, ${RED}$failed failed${NC}"
}

# Run tier 2: Passive OSINT
run_tier_2() {
    log "Running Tier 2: Passive OSINT"
    
    local tools=(
        "subfinder:subfinder -d nmap.org -silent:subfinder_json:60"
        "whatweb:whatweb -v scanme.nmap.org --log-json=-:whatweb_json:30"
    )
    
    local success=0
    local failed=0
    
    for tool_spec in "${tools[@]}"; do
        IFS=':' read -r name cmd parser timeout <<< "$tool_spec"
        
        if execute_tool "$name" "$cmd" "$parser" "$timeout"; then
            ((success++))
        else
            ((failed++))
        fi
    done
    
    log "Tier 2 complete: ${GREEN}$success success${NC}, ${RED}$failed failed${NC}"
}

# List available tools
list_tools() {
    echo ""
    echo "Available Tool Tiers:"
    echo "====================="
    echo ""
    echo "Tier 0 (Help Only - Safest):"
    echo "  nmap, masscan, nikto, gobuster, ffuf, nuclei, amass, subfinder, whatweb, dirb"
    echo ""
    echo "Tier 1 (Localhost/scanme.nmap.org):"
    echo "  nmap scans, ping, dig, whois"
    echo ""
    echo "Tier 2 (Passive OSINT):"
    echo "  subfinder, whatweb, theHarvester (passive sources)"
    echo ""
    echo "Tier 3 (Synthetic Targets - requires target container):"
    echo "  nikto, nuclei, gobuster, sqlmap"
    echo ""
    echo "Tier 4 (Password Cracking - offline only):"
    echo "  john, hashcat with synthetic hashes"
    echo ""
}

# Show manifest
show_manifest() {
    if [ -f "$MANIFEST_FILE" ]; then
        cat "$MANIFEST_FILE"
        echo ""
        echo "Outputs:"
        if [ -f "$OUTPUT_DIR/outputs.jsonl" ]; then
            cat "$OUTPUT_DIR/outputs.jsonl"
        fi
    else
        echo "No manifest found. Run exerciser first."
    fi
}

# Single tool execution
run_single() {
    local name="$1"
    local cmd="$2"
    local parser="${3:-generic}"
    local timeout="${4:-60}"
    
    init_output
    execute_tool "$name" "$cmd" "$parser" "$timeout"
}

# Main run
run_main() {
    init_output
    
    log "Starting Tool Exerciser - Tier $TIER"
    log "Output directory: $OUTPUT_DIR"
    echo ""
    
    case "$TIER" in
        0) run_tier_0 ;;
        1) run_tier_1 ;;
        2) run_tier_2 ;;
        3) 
            log_error "Tier 3 requires synthetic target container"
            log "Start with: docker run -d --name sx9-vuln-target vulnerables/web-dvwa"
            exit 1
            ;;
        4)
            log_error "Tier 4 requires synthetic hash files"
            exit 1
            ;;
        *)
            log_error "Unknown tier: $TIER"
            exit 1
            ;;
    esac
    
    echo ""
    log "Exercise complete!"
    log "Results in: $OUTPUT_DIR"
    log "Hashed outputs: $HASHED_DIR"
    
    # Summary
    local total_outputs
    total_outputs=$(ls -1 "$HASHED_DIR"/*.json 2>/dev/null | wc -l || echo 0)
    log "Total outputs captured: $total_outputs"
}

# ============================================================================
# MAIN
# ============================================================================

# Parse arguments
COMMAND="run"

while [[ $# -gt 0 ]]; do
    case $1 in
        -t|--tier)
            TIER="$2"
            shift 2
            ;;
        -d|--dry-run)
            DRY_RUN="true"
            shift
            ;;
        -v|--verbose)
            VERBOSE="true"
            shift
            ;;
        -n|--nats)
            NATS_URL="$2"
            shift 2
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        run|list|single|manifest)
            COMMAND="$1"
            shift
            break
            ;;
        *)
            log_error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Execute command
case "$COMMAND" in
    run)
        run_main
        ;;
    list)
        list_tools
        ;;
    single)
        if [ $# -lt 2 ]; then
            log_error "Usage: exerciser.sh single <name> <command> [parser] [timeout]"
            exit 1
        fi
        run_single "$@"
        ;;
    manifest)
        show_manifest
        ;;
    *)
        log_error "Unknown command: $COMMAND"
        show_help
        exit 1
        ;;
esac

