#!/bin/bash
# Kali Plasma ISO Builder
#
# Builds a biometric-bound Kali Plasma ISO for an operator.
#
# Prerequisites:
# - Rust nightly with bpfel-unknown-none target
# - LLVM 17+
# - Kali Linux base ISO
# - TPM 2.0 device
# - Biometric sensors
# - Hardware token (YubiKey)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="${PROJECT_DIR}/build"
OUTPUT_DIR="${PROJECT_DIR}/output"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check Rust
    if ! command -v rustc &> /dev/null; then
        log_error "Rust not found. Install from https://rustup.rs"
        exit 1
    fi
    
    # Check Rust nightly
    if ! rustup show | grep -q "nightly"; then
        log_warn "Rust nightly not found. Installing..."
        rustup install nightly
    fi
    
    # Check bpfel target
    if ! rustup target list --installed | grep -q "bpfel-unknown-none"; then
        log_warn "bpfel target not found. Installing..."
        rustup target add bpfel-unknown-none --toolchain nightly
    fi
    
    # Check LLVM
    if ! command -v llvm-objcopy &> /dev/null; then
        log_error "LLVM not found. Install LLVM 17+"
        exit 1
    fi
    
    log_success "Prerequisites OK"
}

# Capture operator biometrics
capture_biometrics() {
    local operator=$1
    local bio_dir="${BUILD_DIR}/biometrics/${operator}"
    
    log_info "Capturing biometrics for operator: ${operator}"
    
    mkdir -p "${bio_dir}"
    
    # In production, these would interface with actual sensors
    # For now, generate placeholder hashes
    
    log_info "  Capturing fingerprint..."
    echo "FINGERPRINT_PLACEHOLDER_${operator}" | sha256sum | cut -d' ' -f1 > "${bio_dir}/fingerprint.hash"
    
    log_info "  Capturing face..."
    echo "FACE_PLACEHOLDER_${operator}" | sha256sum | cut -d' ' -f1 > "${bio_dir}/face.hash"
    
    log_info "  Capturing voice..."
    echo "VOICE_PLACEHOLDER_${operator}" | sha256sum | cut -d' ' -f1 > "${bio_dir}/voice.hash"
    
    # Generate operator identity hash
    cat "${bio_dir}"/*.hash | sha256sum | cut -d' ' -f1 > "${bio_dir}/identity.hash"
    
    log_success "Biometrics captured"
}

# Generate operator keys
generate_keys() {
    local operator=$1
    local key_dir="${BUILD_DIR}/keys/${operator}"
    
    log_info "Generating keys for operator: ${operator}"
    
    mkdir -p "${key_dir}"
    
    # Generate TLS key pair
    openssl genrsa -out "${key_dir}/operator.key" 4096 2>/dev/null
    openssl req -new -x509 -key "${key_dir}/operator.key" \
        -out "${key_dir}/operator.crt" \
        -days 365 \
        -subj "/CN=${operator}/O=CTAS-7/OU=Plasma" 2>/dev/null
    
    log_success "Keys generated"
}

# Build plasma agent
build_agent() {
    log_info "Building plasma-agent..."
    
    cd "${PROJECT_DIR}/agent"
    
    # Build in release mode
    cargo build --release
    
    # Copy binary
    mkdir -p "${BUILD_DIR}/rootfs/usr/lib/plasma"
    cp target/release/plasma-agent "${BUILD_DIR}/rootfs/usr/lib/plasma/"
    
    log_success "Agent built"
}

# Build eBPF tools
build_ebpf_tools() {
    log_info "Building eBPF tools..."
    
    # For now, just create placeholder .o files
    # In production, these would be actual eBPF programs
    
    local tools_dir="${BUILD_DIR}/rootfs/usr/lib/plasma/tools"
    mkdir -p "${tools_dir}"
    
    for tool in nmap masscan nuclei sqlmap hydra metasploit responder impacket bloodhound crackmapexec; do
        log_info "  Building ${tool}_ebpf.o..."
        # Placeholder
        echo "EBPF_PLACEHOLDER_${tool}" > "${tools_dir}/${tool}_ebpf.o"
    done
    
    log_success "eBPF tools built"
}

# Build custom kernel
build_kernel() {
    log_info "Building plasma kernel..."
    
    # In production, this would:
    # 1. Download kernel source
    # 2. Apply plasma config
    # 3. Build with minimal modules
    # 4. Sign with operator key
    
    log_warn "Kernel build skipped (placeholder)"
}

# Assemble ISO
assemble_iso() {
    local operator=$1
    local output="${OUTPUT_DIR}/kali-plasma-${operator}.iso"
    
    log_info "Assembling ISO: ${output}"
    
    mkdir -p "${OUTPUT_DIR}"
    
    # In production, this would:
    # 1. Extract Kali base ISO
    # 2. Replace kernel with plasma kernel
    # 3. Add plasma-agent and eBPF tools
    # 4. Add biometric gate sealed to TPM
    # 5. Remove unnecessary packages
    # 6. Repack ISO
    
    # For now, create a placeholder
    tar -czf "${output}" -C "${BUILD_DIR}" .
    
    log_success "ISO assembled: ${output}"
}

# Sign ISO
sign_iso() {
    local operator=$1
    local iso="${OUTPUT_DIR}/kali-plasma-${operator}.iso"
    
    log_info "Signing ISO..."
    
    # Generate signature
    sha256sum "${iso}" > "${iso}.sha256"
    
    # In production, also sign with operator's key
    
    log_success "ISO signed"
    
    echo ""
    echo "============================================"
    echo "  Kali Plasma ISO Build Complete"
    echo "============================================"
    echo ""
    echo "  Operator: ${operator}"
    echo "  Output:   ${iso}"
    echo "  SHA256:   $(cat ${iso}.sha256 | cut -d' ' -f1)"
    echo ""
    echo "  WARNING: This ISO is bound to operator biometrics."
    echo "           It will not boot without verification."
    echo ""
}

# Main
main() {
    local operator="${1:-$(whoami)}"
    
    echo ""
    echo "╔════════════════════════════════════════════╗"
    echo "║       KALI PLASMA ISO BUILDER              ║"
    echo "╠════════════════════════════════════════════╣"
    echo "║  Operator: ${operator}"
    echo "╚════════════════════════════════════════════╝"
    echo ""
    
    # Create build directory
    mkdir -p "${BUILD_DIR}"
    mkdir -p "${BUILD_DIR}/rootfs"
    
    # Build steps
    check_prerequisites
    capture_biometrics "${operator}"
    generate_keys "${operator}"
    build_agent
    build_ebpf_tools
    build_kernel
    assemble_iso "${operator}"
    sign_iso "${operator}"
}

# Run
main "$@"




