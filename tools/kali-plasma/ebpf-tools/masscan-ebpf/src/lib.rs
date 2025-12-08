//! masscan eBPF implementation
//!
//! High-speed port scanning in eBPF.
//! Optimized for scanning large IP ranges at line rate.

#![no_std]

use plasma_ebpf_common::{ToolTrigger, murmur3_32, runes};

/// masscan batch scan configuration
#[repr(C)]
pub struct BatchConfig {
    /// Starting IP (network byte order)
    pub start_ip: u32,
    /// Ending IP (network byte order)
    pub end_ip: u32,
    /// Ports to scan (up to 16)
    pub ports: [u16; 16],
    /// Number of ports
    pub port_count: u8,
    /// Packets per second limit
    pub rate_pps: u32,
    /// Batch ID
    pub batch_id: u32,
}

/// masscan result (compact for high volume)
#[repr(C, packed)]
pub struct BatchResult {
    /// Target IP
    pub ip: u32,
    /// Open port (0 if none)
    pub port: u16,
    /// Batch ID
    pub batch_id: u32,
}

/// Generate pseudo-random source port based on target
pub fn generate_src_port(ip: u32, port: u16, batch_id: u32) -> u16 {
    let mut data = [0u8; 10];
    data[0..4].copy_from_slice(&ip.to_le_bytes());
    data[4..6].copy_from_slice(&port.to_le_bytes());
    data[6..10].copy_from_slice(&batch_id.to_le_bytes());
    
    let hash = murmur3_32(&data, 0xDEADBEEF);
    
    // Source port in ephemeral range (49152-65535)
    49152 + (hash as u16 % 16383)
}

/// Calculate next IP in scan sequence (randomized)
pub fn next_scan_ip(current: u32, start: u32, end: u32, batch_id: u32) -> u32 {
    // Use LCG for pseudo-random but deterministic sequence
    const A: u32 = 1103515245;
    const C: u32 = 12345;
    
    let range = end.saturating_sub(start);
    if range == 0 {
        return start;
    }
    
    let next = current.wrapping_mul(A).wrapping_add(C).wrapping_add(batch_id);
    start + (next % range)
}

/// Encode batch result as Unicode runes
pub fn encode_result(result: &BatchResult) -> [u32; 3] {
    [
        runes::TOOL_RESPONSE_BASE + 0x20 + (result.ip & 0xFFFF) as u32,
        runes::TOOL_RESPONSE_BASE + 0x21 + (result.port as u32),
        runes::COMPLETION,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_src_port_generation() {
        let port1 = generate_src_port(0x0A000001, 80, 1);
        let port2 = generate_src_port(0x0A000001, 80, 1);
        let port3 = generate_src_port(0x0A000002, 80, 1);
        
        // Same input = same output
        assert_eq!(port1, port2);
        
        // Different input = different output
        assert_ne!(port1, port3);
        
        // In ephemeral range
        assert!(port1 >= 49152);
    }
    
    #[test]
    fn test_next_scan_ip() {
        let start = 0x0A000000; // 10.0.0.0
        let end = 0x0A0000FF;   // 10.0.0.255
        
        let ip1 = next_scan_ip(0, start, end, 1);
        let ip2 = next_scan_ip(ip1, start, end, 1);
        
        // Should be in range
        assert!(ip1 >= start && ip1 <= end);
        assert!(ip2 >= start && ip2 <= end);
        
        // Should be different
        assert_ne!(ip1, ip2);
    }
}




