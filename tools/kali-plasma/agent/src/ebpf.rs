//! eBPF tool management
//!
//! Loads, manages, and dispatches to eBPF tool programs.
//! All tools run entirely in eBPF - no userspace execution.

use anyhow::{Result, Context};
use std::collections::HashMap;

use crate::Operator;

/// Command to dispatch to eBPF tool
#[derive(Debug, Clone)]
pub struct ToolCommand {
    /// Tool name (e.g., "nmap", "masscan")
    pub tool: String,
    /// Tool-specific payload
    pub payload: Vec<u8>,
    /// Command ID for correlation
    pub cmd_id: u64,
}

/// Result from eBPF tool
#[derive(Debug, Clone)]
pub struct ToolResult {
    /// Tool name
    pub tool: String,
    /// Command ID for correlation
    pub cmd_id: u64,
    /// Result payload
    pub payload: Vec<u8>,
    /// Success flag
    pub success: bool,
}

/// eBPF tool manager
pub struct EbpfToolManager {
    /// Loaded tools
    tools: HashMap<String, LoadedTool>,
}

/// A loaded eBPF tool
struct LoadedTool {
    /// Tool name
    name: String,
    /// eBPF program file descriptor (would be aya::Bpf in production)
    #[allow(dead_code)]
    fd: i32,
    /// SDT payload type range
    type_range: (u8, u8),
}

impl EbpfToolManager {
    /// Create new tool manager and load tools for operator
    pub fn new(operator: &Operator) -> Result<Self> {
        let mut tools = HashMap::new();
        
        for tool_name in &operator.allowed_tools {
            let tool = load_tool(tool_name)
                .with_context(|| format!("Failed to load tool: {}", tool_name))?;
            
            tools.insert(tool_name.clone(), tool);
        }
        
        Ok(Self { tools })
    }
    
    /// Get number of loaded tools
    pub fn tool_count(&self) -> usize {
        self.tools.len()
    }
    
    /// Dispatch command to eBPF tool
    pub fn dispatch(&self, cmd: &ToolCommand) -> Result<()> {
        let tool = self.tools.get(&cmd.tool)
            .with_context(|| format!("Tool not loaded: {}", cmd.tool))?;
        
        // In production, this would write to a BPF map that the
        // eBPF program reads from
        
        tracing::debug!(
            "Dispatching to {}: cmd_id={}, payload_len={}",
            tool.name,
            cmd.cmd_id,
            cmd.payload.len()
        );
        
        // TODO: Write to BPF map
        // bpf_map_update_elem(tool.cmd_map_fd, &cmd.cmd_id, &cmd.payload)
        
        Ok(())
    }
    
    /// Read result from eBPF ring buffer
    pub async fn read_result(&self) -> Result<ToolResult> {
        // In production, this would read from the eBPF ring buffer
        // using aya's RingBuf interface

        // TODO: Read from ring buffer
        // let result = ring_buf.next().await?;

        // For now, return a stub
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        Ok(ToolResult {
            tool: "stub".to_string(),
            cmd_id: 0,
            payload: vec![],
            success: true,
        })
    }

    /// Dispatch L2 chain to BPF chain map
    ///
    /// Writes the full chain (header + steps) to the BPF_MAP_TYPE_ARRAY
    /// for L2 execution. The eBPF program reads each step sequentially.
    pub fn dispatch_chain(&self, chain_bytes: &[u8]) -> Result<u64> {
        // Validate minimum size (header = 32 bytes)
        if chain_bytes.len() < 32 {
            anyhow::bail!("Chain too small: {} bytes", chain_bytes.len());
        }

        // Validate magic bytes
        let magic = u16::from_le_bytes([chain_bytes[0], chain_bytes[1]]);
        if magic != 0xC4A1 {
            anyhow::bail!("Invalid chain magic: 0x{:04X}", magic);
        }

        // Extract chain ID for tracking
        let mut chain_id = [0u8; 6];
        chain_id.copy_from_slice(&chain_bytes[2..8]);
        let chain_id_u64 = u64::from_le_bytes([
            chain_id[0], chain_id[1], chain_id[2],
            chain_id[3], chain_id[4], chain_id[5],
            0, 0,
        ]);

        let step_count = chain_bytes[8];

        tracing::info!(
            "Dispatching L2 chain: id={:012X}, steps={}, size={}",
            chain_id_u64,
            step_count,
            chain_bytes.len()
        );

        // In production with aya:
        // let chain_map: Array<_, [u8; 32]> = Array::try_from(
        //     self.bpf.map_mut("chain_map").context("chain_map not found")?
        // )?;
        //
        // // Write header at index 0
        // chain_map.set(0, &chain_bytes[0..32].try_into().unwrap(), 0)?;
        //
        // // Write steps at indices 1..N
        // for (i, chunk) in chain_bytes[32..].chunks(32).enumerate() {
        //     let mut step = [0u8; 32];
        //     step[..chunk.len()].copy_from_slice(chunk);
        //     chain_map.set((i + 1) as u32, &step, 0)?;
        // }
        //
        // // Signal chain ready via perf event
        // self.perf_array.output(&chain_id, 0)?;

        Ok(chain_id_u64)
    }

    /// Read chain execution status from BPF map
    pub fn read_chain_status(&self, chain_id: u64) -> Result<ChainStatus> {
        // In production:
        // let chain_map: Array<_, [u8; 32]> = ...;
        // let header = chain_map.get(&0, 0)?;
        // return ChainStatus::from_byte(header[10]);

        // Stub: return completed
        let _ = chain_id;
        Ok(ChainStatus::Completed)
    }
}

/// Chain execution status from BPF map
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainStatus {
    Pending = 0,
    Running = 1,
    Completed = 2,
    Failed = 3,
    RolledBack = 4,
}

impl ChainStatus {
    pub fn from_byte(b: u8) -> Self {
        match b {
            0 => Self::Pending,
            1 => Self::Running,
            2 => Self::Completed,
            3 => Self::Failed,
            4 => Self::RolledBack,
            _ => Self::Failed,
        }
    }
}

/// Load an eBPF tool program
fn load_tool(name: &str) -> Result<LoadedTool> {
    // In production, this would:
    // 1. Load the eBPF program from /usr/lib/plasma/tools/{name}_ebpf.o
    // 2. Attach to XDP hook
    // 3. Create maps for command dispatch and result collection
    
    let type_range = match name {
        "nmap" => (0x10, 0x1F),
        "masscan" => (0x20, 0x2F),
        "nuclei" => (0x30, 0x3F),
        "sqlmap" => (0x40, 0x4F),
        "hydra" => (0x50, 0x5F),
        "metasploit" => (0x60, 0x6F),
        "responder" => (0x70, 0x7F),
        "impacket" => (0x80, 0x8F),
        "bloodhound" => (0x90, 0x9F),
        "crackmapexec" => (0xA0, 0xAF),
        _ => (0x00, 0x00),
    };
    
    tracing::info!("Loading eBPF tool: {} (types 0x{:02X}-0x{:02X})", name, type_range.0, type_range.1);
    
    Ok(LoadedTool {
        name: name.to_string(),
        fd: -1, // Stub
        type_range,
    })
}

/// Wipe all BPF maps (for self-destruct)
pub fn wipe_all_maps() {
    // In production, this would:
    // 1. Iterate all BPF maps
    // 2. Overwrite with zeros
    // 3. Delete maps
    
    tracing::warn!("Wiping all BPF maps...");
    
    // TODO: Implement with aya
}

/// Unload all eBPF programs (for graceful shutdown)
pub fn unload_all() {
    // In production, this would:
    // 1. Detach all XDP programs
    // 2. Close all program FDs
    // 3. Delete all maps
    
    tracing::info!("Unloading all eBPF programs...");
    
    // TODO: Implement with aya
}




