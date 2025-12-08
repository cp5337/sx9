//! nmap eBPF implementation
//!
//! Pure eBPF port scanning - no userspace execution.
//! Receives SDT frames with tool triggers, executes scans in kernel.

#![no_std]

use plasma_ebpf_common::{
    SdtHeader, ToolTrigger, SchHash, CuidHash,
    trivariate_to_ebpf_key, murmur3_32,
    runes,
};

/// nmap scan target
#[repr(C)]
pub struct ScanTarget {
    /// Target IP (network byte order)
    pub ip: u32,
    /// Target port
    pub port: u16,
    /// Scan type
    pub scan_type: ToolTrigger,
    /// Scan ID for correlation
    pub scan_id: u32,
}

/// nmap scan result
#[repr(C)]
pub struct ScanResult {
    /// Target IP
    pub ip: u32,
    /// Target port
    pub port: u16,
    /// Port state (open=1, closed=2, filtered=3)
    pub state: u8,
    /// Service detected (0=unknown)
    pub service: u8,
    /// Scan ID for correlation
    pub scan_id: u32,
    /// Response time in microseconds
    pub response_time_us: u32,
}

/// Port states
pub mod port_state {
    pub const OPEN: u8 = 1;
    pub const CLOSED: u8 = 2;
    pub const FILTERED: u8 = 3;
    pub const OPEN_FILTERED: u8 = 4;
    pub const UNFILTERED: u8 = 5;
}

/// Parse SDT frame and extract scan target
pub fn parse_scan_target(sdt: &SdtHeader, payload: &[u8]) -> Option<ScanTarget> {
    // Validate payload type is nmap range
    if sdt.payload_type < 0x10 || sdt.payload_type > 0x1F {
        return None;
    }
    
    // Parse target from payload
    // Format: [ip:4][port:2][scan_id:4]
    if payload.len() < 10 {
        return None;
    }
    
    let ip = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);
    let port = u16::from_be_bytes([payload[4], payload[5]]);
    let scan_id = u32::from_le_bytes([payload[6], payload[7], payload[8], payload[9]]);
    
    let scan_type = match sdt.payload_type {
        0x10 => ToolTrigger::NmapSynScan,
        0x11 => ToolTrigger::NmapUdpScan,
        0x12 => ToolTrigger::NmapVersionDetect,
        0x13 => ToolTrigger::NmapOsFingerprint,
        0x14 => ToolTrigger::NmapScriptScan,
        _ => return None,
    };
    
    Some(ScanTarget {
        ip,
        port,
        scan_type,
        scan_id,
    })
}

/// Generate eBPF map key for scan state
pub fn scan_state_key(target: &ScanTarget) -> [u8; 8] {
    [
        (target.ip & 0xFF) as u8,
        ((target.ip >> 8) & 0xFF) as u8,
        ((target.ip >> 16) & 0xFF) as u8,
        ((target.ip >> 24) & 0xFF) as u8,
        (target.port & 0xFF) as u8,
        ((target.port >> 8) & 0xFF) as u8,
        target.scan_type as u8,
        0, // reserved
    ]
}

/// Craft SYN packet for scan
///
/// Returns TCP SYN packet bytes (without IP header - XDP adds that)
pub fn craft_syn_packet(target: &ScanTarget, src_port: u16) -> [u8; 20] {
    let mut packet = [0u8; 20];
    
    // TCP header (20 bytes minimum)
    // Source port
    packet[0] = (src_port >> 8) as u8;
    packet[1] = (src_port & 0xFF) as u8;
    
    // Destination port
    packet[2] = (target.port >> 8) as u8;
    packet[3] = (target.port & 0xFF) as u8;
    
    // Sequence number (random-ish, based on scan_id)
    let seq = murmur3_32(&target.scan_id.to_le_bytes(), 0x12345678);
    packet[4] = (seq >> 24) as u8;
    packet[5] = (seq >> 16) as u8;
    packet[6] = (seq >> 8) as u8;
    packet[7] = (seq & 0xFF) as u8;
    
    // Acknowledgment number (0 for SYN)
    packet[8..12].copy_from_slice(&[0, 0, 0, 0]);
    
    // Data offset (5 = 20 bytes) + reserved
    packet[12] = 0x50;
    
    // Flags: SYN
    packet[13] = 0x02;
    
    // Window size
    packet[14] = 0xFF;
    packet[15] = 0xFF;
    
    // Checksum (placeholder - XDP calculates)
    packet[16] = 0;
    packet[17] = 0;
    
    // Urgent pointer
    packet[18] = 0;
    packet[19] = 0;
    
    packet
}

/// Analyze TCP response to determine port state
pub fn analyze_response(flags: u8) -> u8 {
    const SYN: u8 = 0x02;
    const ACK: u8 = 0x10;
    const RST: u8 = 0x04;
    
    match flags {
        f if f & (SYN | ACK) == (SYN | ACK) => port_state::OPEN,
        f if f & RST != 0 => port_state::CLOSED,
        _ => port_state::FILTERED,
    }
}

/// Generate result with Unicode encoding
pub fn encode_result(result: &ScanResult) -> [u32; 4] {
    [
        // Response rune with port state
        runes::TOOL_RESPONSE_BASE + (result.state as u32),
        // IP as rune
        runes::TOOL_RESPONSE_BASE + 0x10 + (result.ip & 0xFFFF) as u32,
        // Port as rune
        runes::TOOL_RESPONSE_BASE + 0x20 + (result.port as u32),
        // Completion
        runes::COMPLETION,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_syn_packet() {
        let target = ScanTarget {
            ip: 0x0A000001, // 10.0.0.1
            port: 80,
            scan_type: ToolTrigger::NmapSynScan,
            scan_id: 12345,
        };
        
        let packet = craft_syn_packet(&target, 54321);
        
        // Check source port
        assert_eq!(packet[0], (54321 >> 8) as u8);
        assert_eq!(packet[1], (54321 & 0xFF) as u8);
        
        // Check destination port
        assert_eq!(packet[2], 0);
        assert_eq!(packet[3], 80);
        
        // Check SYN flag
        assert_eq!(packet[13], 0x02);
    }
    
    #[test]
    fn test_response_analysis() {
        assert_eq!(analyze_response(0x12), port_state::OPEN);     // SYN+ACK
        assert_eq!(analyze_response(0x14), port_state::CLOSED);   // RST+ACK
        assert_eq!(analyze_response(0x04), port_state::CLOSED);   // RST
        assert_eq!(analyze_response(0x00), port_state::FILTERED); // No response
    }
    
    #[test]
    fn test_scan_state_key() {
        let target = ScanTarget {
            ip: 0x0A000001,
            port: 80,
            scan_type: ToolTrigger::NmapSynScan,
            scan_id: 1,
        };
        
        let key = scan_state_key(&target);
        
        assert_eq!(key.len(), 8);
        assert_eq!(key[0], 0x01); // IP low byte
        assert_eq!(key[4], 80);   // Port low byte
        assert_eq!(key[6], 0x10); // Scan type
    }
}




