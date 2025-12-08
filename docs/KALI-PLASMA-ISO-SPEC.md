# Kali Plasma ISO Specification

## Overview

Kali Plasma is a hardened, biometric-bound Kali Linux derivative that implements SDT (Software-Defined Thyristor) at Layer 2 using Rust eBPF. All offensive tools are invisible to userspace — only SDT commands flow through the polycrystal plasma backplane.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        KALI PLASMA ARCHITECTURE                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │                      USERSPACE (INVISIBLE)                       │   │
│   │                                                                  │   │
│   │   [No shells]  [No logs]  [No processes]  [No network sockets]  │   │
│   │                                                                  │   │
│   │   Only: plasma-agent (single process, biometric-gated)          │   │
│   │                                                                  │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                 │                                        │
│                                 │ SDT frames only                        │
│                                 ▼                                        │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │                      KERNEL (eBPF/XDP)                           │   │
│   │                                                                  │   │
│   │   ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐       │   │
│   │   │ SDT Gate │  │ Polycrys │  │ Tool     │  │ Entropy  │       │   │
│   │   │ Evaluator│  │ Resonator│  │ Executor │  │ Harvester│       │   │
│   │   └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘       │   │
│   │        │             │             │             │              │   │
│   │        └─────────────┴─────────────┴─────────────┘              │   │
│   │                          │                                       │   │
│   │                    PLASMA_STATE map                              │   │
│   │                    (survives reboot)                             │   │
│   │                                                                  │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                 │                                        │
│                                 │ EtherType 0xSD77                       │
│                                 ▼                                        │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │                         NIC (XDP)                                │   │
│   │                                                                  │   │
│   │   Line-rate SDT processing @ 5-12 ns                            │   │
│   │   Zero-copy, zero-allocation                                    │   │
│   │   Tools never touch userspace                                   │   │
│   │                                                                  │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

## Build Requirements

### Biometric Binding

The ISO will not boot without operator biometric verification:

```rust
/// Biometric verification at boot
pub struct BiometricGate {
    /// Fingerprint template hash (stored in TPM)
    fingerprint_hash: [u8; 32],
    /// Face encoding hash (stored in TPM)
    face_hash: [u8; 32],
    /// Voice print hash (stored in TPM)
    voice_hash: [u8; 32],
    /// Hardware token ID (YubiKey, etc.)
    hwtoken_id: [u8; 20],
}

impl BiometricGate {
    /// All three factors required to unlock
    pub fn verify(&self, fp: &[u8], face: &[u8], voice: &[u8]) -> bool {
        let fp_match = constant_time_eq(&hash(fp), &self.fingerprint_hash);
        let face_match = constant_time_eq(&hash(face), &self.face_hash);
        let voice_match = constant_time_eq(&hash(voice), &self.voice_hash);
        
        fp_match && face_match && voice_match
    }
}
```

### Build-Time Binding

```bash
# ISO is compiled with operator's biometrics baked in
# Cannot be transferred, cannot be cloned, cannot be shared

$ kali-plasma-build \
    --fingerprint /dev/fingerprint0 \
    --face /dev/video0 \
    --voice /dev/audio0 \
    --hwtoken /dev/hidraw0 \
    --output kali-plasma-$(whoami).iso
```

## ISO Components

### 1. Minimal Base System

```
kali-plasma/
├── boot/
│   ├── vmlinuz-plasma          # Custom kernel (no modules, no loadable drivers)
│   ├── initramfs-plasma.img    # Biometric verification in initramfs
│   └── grub/
│       └── grub.cfg            # Encrypted, signed
├── lib/
│   └── firmware/               # Only required NIC firmware
├── usr/
│   └── lib/
│       └── plasma/
│           ├── plasma-agent    # Single userspace binary
│           ├── libsdt.so       # SDT frame construction
│           └── libcrystal.so   # Polycrystal resonance
└── etc/
    ├── plasma/
    │   ├── crystals.toml       # Crystal family configs
    │   ├── gates.toml          # SDT gate definitions
    │   └── tools.toml          # Tool chain mappings
    └── nats/
        └── nats.conf           # Tunnel to CDN
```

### 2. Kernel Configuration

```
# Disabled (invisible)
CONFIG_MODULES=n                 # No loadable modules
CONFIG_PROC_FS=n                 # No /proc
CONFIG_SYSFS=n                   # No /sys  
CONFIG_DEVTMPFS=n                # No /dev (except essentials)
CONFIG_AUDIT=n                   # No audit
CONFIG_FTRACE=n                  # No tracing
CONFIG_KPROBES=n                 # No kprobes
CONFIG_PRINTK=n                  # No kernel messages
CONFIG_BUG=n                     # No bug splats
CONFIG_KALLSYMS=n                # No symbol table

# Enabled (required)
CONFIG_BPF=y                     # eBPF support
CONFIG_BPF_SYSCALL=y             # BPF syscall
CONFIG_XDP_SOCKETS=y             # XDP support
CONFIG_NET_CLS_BPF=y             # BPF classifier
CONFIG_NET_ACT_BPF=y             # BPF actions
CONFIG_CRYPTO_USER=y             # Crypto for SDT
CONFIG_TCG_TPM=y                 # TPM for biometrics
```

### 3. eBPF Tool Wrappers

All Kali tools are wrapped in Rust eBPF — they never execute in userspace:

```rust
//! Tool wrapper for nmap
//! Executes entirely in eBPF, results flow through SDT

#![no_std]
#![no_main]

use aya_bpf::{
    macros::{map, xdp},
    maps::{HashMap, RingBuf},
    programs::XdpContext,
};

use sx9_atlas_bus::{Command, CommandKind, Polycrystal};

/// Tool state (persisted in BPF map)
#[map]
static TOOL_STATE: HashMap<u32, ToolState> = HashMap::with_max_entries(256, 0);

/// Results ring buffer (read by plasma-agent)
#[map]
static RESULTS: RingBuf = RingBuf::with_byte_size(1024 * 1024, 0);

/// SDT trigger for nmap
#[xdp]
pub fn nmap_trigger(ctx: XdpContext) -> u32 {
    match handle_nmap(&ctx) {
        Ok(action) => action,
        Err(_) => xdp_action::XDP_PASS,
    }
}

fn handle_nmap(ctx: &XdpContext) -> Result<u32, ()> {
    let sdt = parse_sdt_frame(ctx)?;
    
    // Verify crystal resonance
    let poly = Polycrystal::tripwire();
    let result = poly.resonate_payload(&sdt.payload, sdt.delta_angle);
    
    if !result.passed {
        return Ok(xdp_action::XDP_DROP);
    }
    
    // Execute nmap logic entirely in eBPF
    match sdt.payload_type {
        0x10 => syn_scan(ctx, &sdt),      // SYN scan
        0x11 => udp_scan(ctx, &sdt),      // UDP scan
        0x12 => version_detect(ctx, &sdt), // Version detection
        0x13 => os_fingerprint(ctx, &sdt), // OS fingerprinting
        _ => Ok(xdp_action::XDP_PASS),
    }
}

/// SYN scan implementation (pure eBPF)
fn syn_scan(ctx: &XdpContext, sdt: &SdtFrame) -> Result<u32, ()> {
    let target = parse_target(&sdt.payload)?;
    
    // Craft SYN packet
    let syn = craft_syn_packet(target.ip, target.port);
    
    // Send via XDP_TX (never touches userspace)
    inject_packet(ctx, &syn)?;
    
    // Store pending scan in map
    TOOL_STATE.insert(&target.id, &ToolState::Scanning, 0)?;
    
    Ok(xdp_action::XDP_TX)
}
```

### 4. Wrapped Tools

| Tool | SDT Type | eBPF Program | Description |
|------|----------|--------------|-------------|
| nmap | 0x10-0x1F | `nmap_ebpf.o` | Port scanning, OS detection |
| masscan | 0x20-0x2F | `masscan_ebpf.o` | Mass port scanning |
| nuclei | 0x30-0x3F | `nuclei_ebpf.o` | Vulnerability scanning |
| sqlmap | 0x40-0x4F | `sqlmap_ebpf.o` | SQL injection |
| hydra | 0x50-0x5F | `hydra_ebpf.o` | Brute force |
| metasploit | 0x60-0x6F | `msf_ebpf.o` | Exploitation framework |
| responder | 0x70-0x7F | `responder_ebpf.o` | LLMNR/NBT-NS poisoning |
| impacket | 0x80-0x8F | `impacket_ebpf.o` | SMB/WMI/DCE-RPC |
| bloodhound | 0x90-0x9F | `bloodhound_ebpf.o` | AD enumeration |
| crackmapexec | 0xA0-0xAF | `cme_ebpf.o` | Network pentesting |

### 5. Plasma Agent

The single userspace process:

```rust
//! plasma-agent: The only visible process on Kali Plasma

use sx9_atlas_bus::{AtlasBus, Polycrystal, ThyristorConfig};
use tokio::sync::mpsc;

/// Plasma agent - biometric-gated, single process
#[tokio::main]
async fn main() -> Result<()> {
    // Step 1: Biometric verification (blocks until verified)
    let operator = verify_biometrics().await?;
    
    // Step 2: Initialize SDT bus
    static BUS: AtlasBus = AtlasBus::new();
    BUS.plasma().prime();
    
    // Step 3: Load eBPF programs
    load_ebpf_tools(&operator)?;
    
    // Step 4: Connect to CDN tunnel
    let tunnel = connect_cdn_tunnel(&operator).await?;
    
    // Step 5: Main loop - read commands from tunnel, dispatch to eBPF
    loop {
        tokio::select! {
            // Incoming command from CDN
            cmd = tunnel.recv() => {
                let cmd = cmd?;
                
                // Resonate through polycrystal
                let (passed, _) = BUS.plasma().resonate_poly(
                    &operator.polycrystal,
                    &cmd.payload,
                    tick(),
                    &ThyristorConfig::STRICT,
                );
                
                if passed {
                    // Dispatch to eBPF tool
                    dispatch_to_ebpf(&cmd)?;
                }
            }
            
            // Results from eBPF ring buffer
            result = read_ebpf_results() => {
                let result = result?;
                
                // Filter through DistilBERT (in-process, no external calls)
                let filtered = filter_result(&result)?;
                
                // Send back through tunnel
                tunnel.send(filtered).await?;
            }
            
            // Entropy harvest from NIC
            entropy = harvest_entropy() => {
                BUS.plasma().add_entropy(entropy);
            }
        }
    }
}

/// Verify operator biometrics
async fn verify_biometrics() -> Result<Operator> {
    // Read from TPM-sealed storage
    let gate = BiometricGate::load_from_tpm()?;
    
    // Capture biometrics
    let fp = capture_fingerprint().await?;
    let face = capture_face().await?;
    let voice = capture_voice().await?;
    
    // Verify
    if !gate.verify(&fp, &face, &voice) {
        // Self-destruct: wipe RAM, halt
        secure_wipe();
        std::process::exit(1);
    }
    
    // Load operator config
    Operator::load(&gate)
}
```

## CDN Tunnel

### Connection Flow

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         CDN TUNNEL ARCHITECTURE                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   Kali Plasma ──► Tailscale/WireGuard ──► CDN Edge ──► CTAS Main       │
│        │              (encrypted)           (filter)     (orchestrator) │
│        │                                                                 │
│   [plasma-agent]                                                        │
│        │                                                                 │
│        ├──► sx9.tool.nmap.cmd      (outbound commands)                  │
│        ├──► sx9.tool.nmap.result   (inbound results)                    │
│        ├──► sx9.plasma.entropy     (entropy harvest)                    │
│        └──► sx9.sdt.canary         (trip alerts)                        │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### NATS Configuration

```toml
# /etc/nats/nats.conf

# Tunnel through CDN
connect_urls = [
    "nats://cdn-edge-1.sx9.io:4222",
    "nats://cdn-edge-2.sx9.io:4222",
]

# TLS with operator cert
tls {
    cert_file = "/etc/plasma/operator.crt"
    key_file = "/etc/plasma/operator.key"
    ca_file = "/etc/plasma/ca.crt"
}

# JetStream for persistence
jetstream {
    store_dir = "/dev/shm/nats"  # RAM only, no disk
    max_memory_store = 128MB
}
```

## Security Features

### 1. No Forensic Artifacts

| Artifact | Status | Reason |
|----------|--------|--------|
| Shell history | ❌ None | No shell |
| Process list | ❌ None | No /proc |
| Network sockets | ❌ None | XDP only |
| Log files | ❌ None | No syslog |
| Disk writes | ❌ None | RAM-only |
| Core dumps | ❌ None | Disabled |
| Swap | ❌ None | Disabled |

### 2. Self-Destruct Triggers

```rust
/// Conditions that trigger secure wipe
pub enum SelfDestructTrigger {
    /// Biometric verification failed 3 times
    BiometricFail,
    /// Hardware token removed
    TokenRemoved,
    /// Canary tripped
    CanaryTrip,
    /// Entropy drought (no entropy for 60s)
    EntropyDrought,
    /// SDT gate forced open
    GateTamper,
    /// Manual trigger (panic button)
    ManualTrigger,
}

/// Secure wipe procedure
pub fn self_destruct(reason: SelfDestructTrigger) -> ! {
    // 1. Wipe all BPF maps
    wipe_bpf_maps();
    
    // 2. Overwrite RAM
    for page in ram_pages() {
        unsafe { volatile_memset(page, 0xFF, PAGE_SIZE); }
        unsafe { volatile_memset(page, 0x00, PAGE_SIZE); }
        unsafe { volatile_memset(page, 0xAA, PAGE_SIZE); }
    }
    
    // 3. Clear TPM
    tpm_clear();
    
    // 4. Halt (not reboot)
    unsafe { asm!("cli; hlt"); }
    
    loop {}
}
```

### 3. Anti-Tampering

```rust
/// Runtime integrity checks
pub fn integrity_loop() {
    loop {
        // Check eBPF program hashes
        for prog in ebpf_programs() {
            let hash = hash_program(prog);
            if hash != prog.expected_hash {
                self_destruct(SelfDestructTrigger::GateTamper);
            }
        }
        
        // Check plasma state
        if !plasma_state_valid() {
            self_destruct(SelfDestructTrigger::GateTamper);
        }
        
        // Check crystal resonance
        let result = POLYCRYSTAL.resonate_payload(b"integrity", 0);
        if !result.passed {
            self_destruct(SelfDestructTrigger::CanaryTrip);
        }
        
        sleep(Duration::from_millis(100));
    }
}
```

## Build Process

### Prerequisites

```bash
# Build machine requirements
- Rust nightly (for eBPF)
- LLVM 17+ (for BPF target)
- Kali Linux base ISO
- TPM 2.0 device
- Biometric sensors (fingerprint, camera, mic)
- YubiKey or similar hardware token
```

### Build Script

```bash
#!/bin/bash
# build-kali-plasma.sh

set -e

OPERATOR=$1
OUTPUT="kali-plasma-${OPERATOR}.iso"

echo "=== Kali Plasma ISO Builder ==="
echo "Operator: ${OPERATOR}"

# Step 1: Capture biometrics
echo "[1/7] Capturing biometrics..."
./capture-biometrics.sh ${OPERATOR}

# Step 2: Generate operator keys
echo "[2/7] Generating operator keys..."
./generate-keys.sh ${OPERATOR}

# Step 3: Build custom kernel
echo "[3/7] Building plasma kernel..."
cd kernel && make KCONFIG=plasma.config && cd ..

# Step 4: Build eBPF tools
echo "[4/7] Building eBPF tools..."
cd tools && cargo build --release --target bpfel-unknown-none && cd ..

# Step 5: Build plasma-agent
echo "[5/7] Building plasma-agent..."
cd agent && cargo build --release && cd ..

# Step 6: Assemble ISO
echo "[6/7] Assembling ISO..."
./assemble-iso.sh ${OPERATOR}

# Step 7: Sign and seal
echo "[7/7] Signing and sealing..."
./sign-iso.sh ${OUTPUT} ${OPERATOR}

echo "=== Build complete: ${OUTPUT} ==="
echo "SHA256: $(sha256sum ${OUTPUT})"
```

### Quantum-Ready Extensions

```rust
/// Future: Quantum key integration
pub struct QuantumKeyIntegration {
    /// Satellite bird for QKD
    qkd_bird_id: u16,
    
    /// Ground station for key reception
    ground_station: GroundStation,
    
    /// Hybrid key derivation (classical + quantum)
    hybrid_kdf: HybridKdf,
}

impl QuantumKeyIntegration {
    /// Derive session key from quantum entropy
    pub async fn derive_session_key(&self) -> SessionKey {
        // Receive ENTROPY_DRIP from satellite
        let quantum_bits = self.receive_entropy_drip().await;
        
        // Combine with classical entropy
        let classical_bits = harvest_local_entropy();
        
        // Hybrid KDF
        self.hybrid_kdf.derive(&quantum_bits, &classical_bits)
    }
}
```

## Deployment

### USB Boot

```bash
# Write ISO to USB
$ sudo dd if=kali-plasma-operator.iso of=/dev/sdb bs=4M status=progress

# Boot from USB, biometric verification required
# No installation, runs entirely in RAM
```

### Network Boot (PXE)

```bash
# TFTP server with signed boot images
# Requires hardware token present at boot
# Biometric verification over secure channel
```

### VM (Development Only)

```bash
# NOT recommended for operations
# TPM emulation required
# Biometric passthrough required
$ qemu-system-x86_64 \
    -enable-kvm \
    -m 4G \
    -cdrom kali-plasma-dev.iso \
    -device tpm-tis,tpmdev=tpm0 \
    -tpmdev emulator,id=tpm0,chardev=chrtpm \
    -usb -device usb-host,vendorid=0x1050,productid=0x0407  # YubiKey
```

## Operational Modes

### Mode 1: Reconnaissance

```toml
# crystals.toml
[mode.recon]
polycrystal = "tripwire"
tools = ["nmap", "masscan", "bloodhound"]
entropy_threshold = 0.3
```

### Mode 2: Exploitation

```toml
[mode.exploit]
polycrystal = "corporate_strict"
tools = ["metasploit", "sqlmap", "hydra"]
entropy_threshold = 0.7
```

### Mode 3: Persistence

```toml
[mode.persist]
polycrystal = "silent"
tools = ["impacket", "responder"]
entropy_threshold = 0.9
```

### Mode 4: Exfiltration

```toml
[mode.exfil]
polycrystal = "van_allen"
tools = ["crackmapexec"]
entropy_threshold = 0.95
tunnel = "priority"
```

---

## Summary

Kali Plasma is not a penetration testing distribution.

It's an **invisible operator platform** where:
- Tools never exist in userspace
- Commands flow through polycrystal resonance
- Results are filtered at line rate
- The operator is biometrically bound
- The system self-destructs on tampering

**The packet never existed if SDT says no.**
**The tool never ran if the crystal didn't ring.**
**The operator was never here if biometrics don't match.**

---

*"We don't run tools. The NIC runs tools. We just resonate."*




