//! Entropy harvesting module
//!
//! Collects entropy from various sources:
//! - NIC timing jitter
//! - CPU cycle variations
//! - Interrupt timing
//! - ENTROPY_DRIP from Van Allen birds (when available)

use sx9_atlas_bus::PlasmaState;

/// Entropy harvester
pub struct EntropyHarvester {
    /// Accumulated entropy
    accumulated: u64,
    /// Sample count
    samples: u64,
}

impl EntropyHarvester {
    /// Create new entropy harvester
    pub fn new() -> Self {
        Self {
            accumulated: 0,
            samples: 0,
        }
    }
    
    /// Run entropy harvesting loop
    pub async fn run(&self, plasma: &PlasmaState) {
        loop {
            // Harvest from multiple sources
            let nic_entropy = harvest_nic_timing();
            let cpu_entropy = harvest_cpu_cycles();
            let interrupt_entropy = harvest_interrupt_timing();
            
            // Combine using XOR (simple but effective)
            let combined = nic_entropy ^ cpu_entropy ^ interrupt_entropy;
            
            // Add to plasma state
            plasma.add_entropy(combined);
            
            // Sleep briefly
            tokio::time::sleep(tokio::time::Duration::from_micros(100)).await;
        }
    }
}

/// Get initial entropy at startup
pub fn initial_entropy() -> u32 {
    let mut entropy = 0u32;
    
    // Use CPU timestamp counter
    #[cfg(target_arch = "x86_64")]
    {
        entropy ^= unsafe { core::arch::x86_64::_rdtsc() as u32 };
    }
    
    // Mix with process ID and time
    entropy ^= std::process::id();
    entropy ^= std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u32;
    
    entropy
}

/// Harvest entropy from NIC timing
fn harvest_nic_timing() -> u32 {
    // In production, this would:
    // 1. Read NIC interrupt timestamps
    // 2. Calculate inter-arrival time jitter
    // 3. Extract entropy from jitter
    
    // For now, use system time nanoseconds
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos()
}

/// Harvest entropy from CPU cycle variations
fn harvest_cpu_cycles() -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        // Read TSC twice and use the difference
        let t1 = unsafe { core::arch::x86_64::_rdtsc() };
        
        // Small computation to introduce variation
        let mut x = 0u64;
        for i in 0..100 {
            x = x.wrapping_add(i);
        }
        
        let t2 = unsafe { core::arch::x86_64::_rdtsc() };
        
        // The low bits of the difference have the most entropy
        ((t2 - t1) ^ x) as u32
    }
    
    #[cfg(not(target_arch = "x86_64"))]
    {
        // Fallback for non-x86
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos()
    }
}

/// Harvest entropy from interrupt timing
fn harvest_interrupt_timing() -> u32 {
    // In production, this would:
    // 1. Read /proc/interrupts or BPF map
    // 2. Calculate timing jitter between interrupts
    // 3. Extract entropy from jitter
    
    // For now, use thread ID mixed with time
    let tid = std::thread::current().id();
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    
    // Mix using rotation
    let tid_bits = format!("{:?}", tid).len() as u32;
    time.rotate_left(tid_bits)
}

/// Process ENTROPY_DRIP from Van Allen bird
///
/// This is called when we receive an SDT frame with type 0x09 (KEY/ENTROPY_DRIP)
#[allow(dead_code)]
pub fn process_entropy_drip(payload: &[u8]) -> Option<[u8; 128]> {
    if payload.len() < 138 {
        return None;
    }
    
    // Parse ENTROPY_DRIP payload
    // [bird_id:2][bits:128][harvest_ts:8]
    
    let _bird_id = u16::from_le_bytes([payload[0], payload[1]]);
    
    let mut bits = [0u8; 128];
    bits.copy_from_slice(&payload[2..130]);
    
    let _harvest_ts = u64::from_le_bytes([
        payload[130], payload[131], payload[132], payload[133],
        payload[134], payload[135], payload[136], payload[137],
    ]);
    
    Some(bits)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_initial_entropy() {
        let e1 = initial_entropy();
        let e2 = initial_entropy();
        
        // Should be different (high probability)
        assert_ne!(e1, e2);
    }
    
    #[test]
    fn test_entropy_drip_parse() {
        let mut payload = vec![0u8; 138];
        
        // bird_id = 1
        payload[0] = 1;
        payload[1] = 0;
        
        // 128 bytes of entropy
        for i in 2..130 {
            payload[i] = i as u8;
        }
        
        // timestamp
        payload[130..138].copy_from_slice(&12345678u64.to_le_bytes());
        
        let bits = process_entropy_drip(&payload).unwrap();
        
        assert_eq!(bits[0], 2);
        assert_eq!(bits[127], 129);
    }
}




