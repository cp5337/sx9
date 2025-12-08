//! L2 Tool Chain Orchestration Module
//!
//! Pure Layer-2 chain execution:
//! - Chain definitions encoded as SCH hashes
//! - Steps dispatched via ToolTrigger unicode runes
//! - No userspace processing - all L2 eBPF maps
//! - Results flow through ring buffer → CDN tunnel

use anyhow::{Result, bail};

// Chain definition encoded as L2 SCH sequence for eBPF dispatch

/// L2 ToolTrigger runes (from ebpf-tools/common)
/// Maps to Unicode Private Use Area U+EE00+
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum L2Trigger {
    /// nmap SYN scan
    NmapSyn = 0x10,
    /// masscan TCP scan
    MasscanTcp = 0x20,
    /// nuclei template scan
    NucleiTemplate = 0x30,
    /// sqlmap detection
    SqlmapDetect = 0x40,
    /// hydra SSH bruteforce
    HydraSsh = 0x50,
    /// metasploit exploit
    MsfExploit = 0x60,
    /// responder LLMNR
    ResponderLlmnr = 0x70,
    /// impacket SMB
    ImpacketSmb = 0x80,
    /// bloodhound collection
    BloodhoundCollect = 0x90,
    /// crackmapexec SMB
    CmeSmb = 0xA0,
}

impl L2Trigger {
    /// Convert from tool name to L2 trigger byte
    pub fn from_tool(name: &str) -> Option<Self> {
        match name {
            "nmap" => Some(Self::NmapSyn),
            "masscan" => Some(Self::MasscanTcp),
            "nuclei" => Some(Self::NucleiTemplate),
            "sqlmap" => Some(Self::SqlmapDetect),
            "hydra" => Some(Self::HydraSsh),
            "metasploit" | "msf" => Some(Self::MsfExploit),
            "responder" => Some(Self::ResponderLlmnr),
            "impacket" => Some(Self::ImpacketSmb),
            "bloodhound" => Some(Self::BloodhoundCollect),
            "crackmapexec" | "cme" => Some(Self::CmeSmb),
            _ => None,
        }
    }

    /// Get Unicode rune for this trigger (Private Use Area)
    pub fn to_rune(self) -> char {
        char::from_u32(0xEE00 + self as u32).unwrap_or('\u{FFFD}')
    }
}

/// HD4 Kill Chain phase byte encoding
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hd4Phase {
    Hunt = 0x01,
    Detect = 0x02,
    Disrupt = 0x03,
    Disable = 0x04,
    Dominate = 0x05,
}

/// L2 Chain Step - encoded as 32-byte BPF map entry
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct L2ChainStep {
    /// Tool trigger byte
    pub trigger: u8,
    /// HD4 phase
    pub hd4_phase: u8,
    /// Timeout in seconds (max 255)
    pub timeout_secs: u8,
    /// Flags: [0]=conditional, [1]=rollback, [2-7]=reserved
    pub flags: u8,
    /// Target hash (SCH format - 8 bytes)
    pub target_sch: [u8; 8],
    /// Port/range specification (4 bytes)
    pub port_spec: [u8; 4],
    /// Next step index (0xFF = terminal)
    pub next_step: u8,
    /// Rollback step index (0xFF = none)
    pub rollback_step: u8,
    /// Reserved for alignment
    pub _reserved: [u8; 14],
}

impl L2ChainStep {
    /// Size in bytes for BPF map
    pub const SIZE: usize = 32;

    /// Create empty step
    pub fn empty() -> Self {
        Self {
            trigger: 0,
            hd4_phase: 0,
            timeout_secs: 60,
            flags: 0,
            target_sch: [0; 8],
            port_spec: [0; 4],
            next_step: 0xFF,
            rollback_step: 0xFF,
            _reserved: [0; 14],
        }
    }

    /// Encode as bytes for BPF map
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];
        bytes[0] = self.trigger;
        bytes[1] = self.hd4_phase;
        bytes[2] = self.timeout_secs;
        bytes[3] = self.flags;
        bytes[4..12].copy_from_slice(&self.target_sch);
        bytes[12..16].copy_from_slice(&self.port_spec);
        bytes[16] = self.next_step;
        bytes[17] = self.rollback_step;
        bytes
    }

    /// Decode from BPF map bytes
    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        let mut target_sch = [0u8; 8];
        let mut port_spec = [0u8; 4];
        target_sch.copy_from_slice(&bytes[4..12]);
        port_spec.copy_from_slice(&bytes[12..16]);

        Self {
            trigger: bytes[0],
            hd4_phase: bytes[1],
            timeout_secs: bytes[2],
            flags: bytes[3],
            target_sch,
            port_spec,
            next_step: bytes[16],
            rollback_step: bytes[17],
            _reserved: [0; 14],
        }
    }
}

/// L2 Chain Header - first entry in BPF chain map
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct L2ChainHeader {
    /// Magic bytes: 0xC4A1 (CHAIN)
    pub magic: u16,
    /// Chain ID (CUID format - 6 bytes)
    pub chain_id: [u8; 6],
    /// Number of steps
    pub step_count: u8,
    /// Current step index
    pub current_step: u8,
    /// Chain status: 0=pending, 1=running, 2=done, 3=fail, 4=rollback
    pub status: u8,
    /// GLAF node reference (4 bytes)
    pub glaf_ref: [u8; 4],
    /// Minimum entropy bits * 10 (e.g., 25 = 2.5 bits)
    pub min_entropy_x10: u8,
    /// HD4 phase for overall chain
    pub chain_hd4: u8,
    /// Reserved
    pub _reserved: [u8; 14],
}

impl L2ChainHeader {
    /// Magic bytes for validation
    pub const MAGIC: u16 = 0xC4A1;
    /// Size in bytes
    pub const SIZE: usize = 32;

    /// Create new chain header
    pub fn new(chain_id: [u8; 6], step_count: u8) -> Self {
        Self {
            magic: Self::MAGIC,
            chain_id,
            step_count,
            current_step: 0,
            status: 0,
            glaf_ref: [0; 4],
            min_entropy_x10: 25, // 2.5 bits default
            chain_hd4: Hd4Phase::Hunt as u8,
            _reserved: [0; 14],
        }
    }

    /// Encode as bytes for BPF map
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];
        bytes[0..2].copy_from_slice(&self.magic.to_le_bytes());
        bytes[2..8].copy_from_slice(&self.chain_id);
        bytes[8] = self.step_count;
        bytes[9] = self.current_step;
        bytes[10] = self.status;
        bytes[11..15].copy_from_slice(&self.glaf_ref);
        bytes[15] = self.min_entropy_x10;
        bytes[16] = self.chain_hd4;
        bytes
    }
}

// L2 Chain orchestrator writes directly to BPF maps

/// L2 Chain - complete chain definition for BPF execution
pub struct L2Chain {
    /// Chain header
    pub header: L2ChainHeader,
    /// Chain steps (max 16)
    pub steps: Vec<L2ChainStep>,
}

impl L2Chain {
    /// Maximum steps per chain
    pub const MAX_STEPS: usize = 16;

    /// Create new L2 chain from builder pattern
    pub fn builder(name: &str) -> L2ChainBuilder {
        L2ChainBuilder::new(name)
    }

    /// Encode entire chain as bytes for BPF map array
    /// Format: [header:32][step0:32][step1:32]...
    pub fn to_bpf_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(32 + self.steps.len() * 32);
        bytes.extend_from_slice(&self.header.to_bytes());
        for step in &self.steps {
            bytes.extend_from_slice(&step.to_bytes());
        }
        bytes
    }

    /// Get L2 trigger payload for emission to ring bus
    pub fn trigger_payload(&self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(16);

        // Chain ID (6 bytes)
        payload.extend_from_slice(&self.header.chain_id);

        // Step count
        payload.push(self.header.step_count);

        // First step trigger
        if let Some(step) = self.steps.first() {
            payload.push(step.trigger);
        } else {
            payload.push(0);
        }

        // GLAF reference
        payload.extend_from_slice(&self.header.glaf_ref);

        // HD4 phase
        payload.push(self.header.chain_hd4);

        payload
    }
}

/// Builder for L2 chains
pub struct L2ChainBuilder {
    /// Chain name (hashed to CUID)
    name: String,
    /// Steps to add
    steps: Vec<L2ChainStep>,
    /// GLAF node reference
    glaf_ref: Option<[u8; 4]>,
    /// HD4 phase
    hd4_phase: Hd4Phase,
    /// Minimum entropy
    min_entropy_x10: u8,
}

impl L2ChainBuilder {
    /// Create new builder
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            steps: Vec::new(),
            glaf_ref: None,
            hd4_phase: Hd4Phase::Hunt,
            min_entropy_x10: 25,
        }
    }

    /// Add a step to the chain
    pub fn step(mut self, trigger: L2Trigger, target_sch: [u8; 8]) -> Self {
        let idx = self.steps.len();
        let mut step = L2ChainStep::empty();
        step.trigger = trigger as u8;
        step.hd4_phase = self.hd4_phase as u8;
        step.target_sch = target_sch;
        step.next_step = if idx < L2Chain::MAX_STEPS - 1 {
            (idx + 1) as u8
        } else {
            0xFF
        };
        self.steps.push(step);
        self
    }

    /// Set port specification for last step
    pub fn ports(mut self, start: u16, end: u16) -> Self {
        if let Some(step) = self.steps.last_mut() {
            step.port_spec[0..2].copy_from_slice(&start.to_le_bytes());
            step.port_spec[2..4].copy_from_slice(&end.to_le_bytes());
        }
        self
    }

    /// Set timeout for last step
    pub fn timeout(mut self, secs: u8) -> Self {
        if let Some(step) = self.steps.last_mut() {
            step.timeout_secs = secs;
        }
        self
    }

    /// Set HD4 phase
    pub fn hd4(mut self, phase: Hd4Phase) -> Self {
        self.hd4_phase = phase;
        self
    }

    /// Link to GLAF node
    pub fn glaf(mut self, node_ref: [u8; 4]) -> Self {
        self.glaf_ref = Some(node_ref);
        self
    }

    /// Set minimum entropy requirement
    pub fn min_entropy(mut self, bits: f32) -> Self {
        self.min_entropy_x10 = (bits * 10.0) as u8;
        self
    }

    /// Build the L2 chain
    pub fn build(mut self) -> Result<L2Chain> {
        if self.steps.is_empty() {
            bail!("Chain must have at least one step");
        }
        if self.steps.len() > L2Chain::MAX_STEPS {
            bail!("Chain exceeds maximum {} steps", L2Chain::MAX_STEPS);
        }

        // Mark last step as terminal
        if let Some(step) = self.steps.last_mut() {
            step.next_step = 0xFF;
        }

        // Generate CUID from name (simple hash for now)
        let chain_id = hash_to_cuid(&self.name);

        let mut header = L2ChainHeader::new(chain_id, self.steps.len() as u8);
        header.chain_hd4 = self.hd4_phase as u8;
        header.min_entropy_x10 = self.min_entropy_x10;
        if let Some(glaf) = self.glaf_ref {
            header.glaf_ref = glaf;
        }

        Ok(L2Chain {
            header,
            steps: self.steps,
        })
    }
}

/// Hash name to 6-byte CUID
fn hash_to_cuid(name: &str) -> [u8; 6] {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    let result = hasher.finalize();

    let mut cuid = [0u8; 6];
    cuid.copy_from_slice(&result[0..6]);
    cuid
}

/// Compute SCH (Semantic Content Hash) from target specification
pub fn target_to_sch(target: &str) -> [u8; 8] {
    use blake3::Hasher;
    let mut hasher = Hasher::new();
    hasher.update(target.as_bytes());
    let result = hasher.finalize();

    let mut sch = [0u8; 8];
    sch.copy_from_slice(&result.as_bytes()[0..8]);
    sch
}

// Predefined chain templates for common operations

/// Create reconnaissance chain (Hunt phase)
pub fn recon_chain(target: &str) -> Result<L2Chain> {
    let target_sch = target_to_sch(target);

    L2Chain::builder("recon-basic")
        .hd4(Hd4Phase::Hunt)
        .step(L2Trigger::NmapSyn, target_sch)
        .ports(1, 1000)
        .timeout(120)
        .step(L2Trigger::MasscanTcp, target_sch)
        .ports(1, 65535)
        .timeout(60)
        .step(L2Trigger::NucleiTemplate, target_sch)
        .timeout(180)
        .build()
}

/// Create SMB enumeration chain (Hunt phase)
pub fn smb_enum_chain(target: &str) -> Result<L2Chain> {
    let target_sch = target_to_sch(target);

    L2Chain::builder("smb-enum")
        .hd4(Hd4Phase::Hunt)
        .step(L2Trigger::NmapSyn, target_sch)
        .ports(139, 445)
        .timeout(30)
        .step(L2Trigger::CmeSmb, target_sch)
        .timeout(60)
        .step(L2Trigger::ImpacketSmb, target_sch)
        .timeout(90)
        .build()
}

/// Create AD enumeration chain (Hunt phase)
pub fn ad_enum_chain(target: &str) -> Result<L2Chain> {
    let target_sch = target_to_sch(target);

    L2Chain::builder("ad-enum")
        .hd4(Hd4Phase::Hunt)
        .step(L2Trigger::BloodhoundCollect, target_sch)
        .timeout(180)
        .step(L2Trigger::CmeSmb, target_sch)
        .timeout(120)
        .build()
}

/// Create exploitation chain (Disrupt phase)
pub fn exploit_chain(target: &str, glaf_threat_id: [u8; 4]) -> Result<L2Chain> {
    let target_sch = target_to_sch(target);

    L2Chain::builder("exploit-threat")
        .hd4(Hd4Phase::Disrupt)
        .glaf(glaf_threat_id)
        .min_entropy(3.0)
        .step(L2Trigger::SqlmapDetect, target_sch)
        .timeout(120)
        .step(L2Trigger::MsfExploit, target_sch)
        .timeout(300)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l2_trigger_rune() {
        assert_eq!(L2Trigger::NmapSyn.to_rune(), '\u{EE10}');
        assert_eq!(L2Trigger::MasscanTcp.to_rune(), '\u{EE20}');
    }

    #[test]
    fn test_chain_builder() {
        let target_sch = target_to_sch("192.168.1.1");

        let chain = L2Chain::builder("test-chain")
            .hd4(Hd4Phase::Hunt)
            .step(L2Trigger::NmapSyn, target_sch)
            .ports(1, 1000)
            .step(L2Trigger::MasscanTcp, target_sch)
            .build()
            .unwrap();

        assert_eq!(chain.steps.len(), 2);
        assert_eq!(chain.header.step_count, 2);
        assert_eq!(chain.steps[0].next_step, 1);
        assert_eq!(chain.steps[1].next_step, 0xFF);
    }

    #[test]
    fn test_step_encoding() {
        let mut step = L2ChainStep::empty();
        step.trigger = L2Trigger::NmapSyn as u8;
        step.hd4_phase = Hd4Phase::Hunt as u8;
        step.timeout_secs = 60;

        let bytes = step.to_bytes();
        let decoded = L2ChainStep::from_bytes(&bytes);

        assert_eq!(decoded.trigger, step.trigger);
        assert_eq!(decoded.hd4_phase, step.hd4_phase);
        assert_eq!(decoded.timeout_secs, step.timeout_secs);
    }

    #[test]
    fn test_bpf_bytes() {
        let chain = recon_chain("192.168.1.1").unwrap();
        let bytes = chain.to_bpf_bytes();

        // Header (32) + 3 steps (32 each) = 128 bytes
        assert_eq!(bytes.len(), 32 + 3 * 32);

        // Verify magic
        let magic = u16::from_le_bytes([bytes[0], bytes[1]]);
        assert_eq!(magic, L2ChainHeader::MAGIC);
    }

    #[test]
    fn test_trigger_payload() {
        let chain = recon_chain("192.168.1.1").unwrap();
        let payload = chain.trigger_payload();

        // Should have chain_id(6) + step_count(1) + first_trigger(1) + glaf(4) + hd4(1) = 13
        assert_eq!(payload.len(), 13);
        assert_eq!(payload[7], L2Trigger::NmapSyn as u8);
    }
}
