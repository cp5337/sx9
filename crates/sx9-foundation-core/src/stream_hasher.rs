//! High-Speed Stream Hashing Engine
//! Optimized for real-time incoming stream processing with Murmur3 and Blake3

use crate::murmur3_trivariate::{murmur3_hash, murmur3_to_base96};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, instrument};

/// High-speed stream hasher configuration
#[derive(Debug, Clone)]
pub struct StreamHasherConfig {
    /// Buffer size for batch processing
    pub buffer_size: usize,
    /// Hash algorithm selection
    pub algorithm: HashAlgorithm,
    /// Stream processing mode
    pub mode: StreamMode,
    /// Enable parallel processing
    pub parallel: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashAlgorithm {
    Murmur3,
    Blake3,
    Hybrid,  // Use both for verification
}

#[derive(Debug, Clone)]
pub enum StreamMode {
    RealTime,     // Process immediately
    Batched,      // Buffer and process in batches
    Streaming,    // Continuous processing
}

/// Incoming stream data packet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamPacket {
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub sequence: u64,
    pub source: String,
    pub packet_id: String,
}

/// Hash result for stream data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamHashResult {
    pub packet_id: String,
    pub hash: String,
    pub algorithm: HashAlgorithm,
    pub processing_time_ns: u64,
    pub throughput_mbps: f64,
    pub sequence: u64,
}

/// High-performance stream hasher
pub struct StreamHasher {
    config: StreamHasherConfig,
    buffer: Arc<RwLock<VecDeque<StreamPacket>>>,
    stats: Arc<RwLock<StreamStats>>,
}

#[derive(Debug, Default)]
struct StreamStats {
    packets_processed: u64,
    total_bytes: u64,
    total_time_ns: u64,
    errors: u64,
}

impl StreamHasher {
    /// Create new high-speed stream hasher
    pub fn new(config: StreamHasherConfig) -> Self {
        Self {
            config,
            buffer: Arc::new(RwLock::new(VecDeque::with_capacity(config.buffer_size))),
            stats: Arc::new(RwLock::new(StreamStats::default())),
        }
    }

    /// Process incoming stream packet at high speed
    #[instrument(skip(self, packet))]
    pub async fn process_stream_packet(&self, packet: StreamPacket) -> Result<StreamHashResult> {
        let start_time = std::time::Instant::now();

        let hash_result = match self.config.algorithm {
            HashAlgorithm::Murmur3 => self.hash_murmur3(&packet).await?,
            HashAlgorithm::Blake3 => self.hash_blake3(&packet).await?,
            HashAlgorithm::Hybrid => self.hash_hybrid(&packet).await?,
        };

        let processing_time = start_time.elapsed();
        let processing_time_ns = processing_time.as_nanos() as u64;

        // Calculate throughput (MB/s)
        let data_size_mb = packet.data.len() as f64 / (1024.0 * 1024.0);
        let time_seconds = processing_time.as_secs_f64();
        let throughput_mbps = if time_seconds > 0.0 { data_size_mb / time_seconds } else { 0.0 };

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.packets_processed += 1;
            stats.total_bytes += packet.data.len() as u64;
            stats.total_time_ns += processing_time_ns;
        }

        Ok(StreamHashResult {
            packet_id: packet.packet_id,
            hash: hash_result,
            algorithm: self.config.algorithm.clone(),
            processing_time_ns,
            throughput_mbps,
            sequence: packet.sequence,
        })
    }

    /// High-speed Murmur3 hashing for streams
    async fn hash_murmur3(&self, packet: &StreamPacket) -> Result<String> {
        // Use different seeds for different stream sources for collision resistance
        let source_seed = self.calculate_source_seed(&packet.source);
        let sequence_seed = (packet.sequence & 0xFFFFFFFF) as u32;
        let combined_seed = source_seed.wrapping_add(sequence_seed);

        let hash = murmur3_hash(&packet.data, combined_seed);
        Ok(format!("{:08x}", hash))
    }

    /// High-speed Blake3 hashing for streams
    async fn hash_blake3(&self, packet: &StreamPacket) -> Result<String> {
        let hash = HashEngine::new().generate_trivariate_hash(&packet.data);
        Ok(hex::encode(hash.as_bytes()))
    }

    /// Hybrid hashing for verification
    async fn hash_hybrid(&self, packet: &StreamPacket) -> Result<String> {
        let murmur3_hash = self.hash_murmur3(packet).await?;
        let blake3_hash = self.hash_blake3(packet).await?;

        // Combine both hashes for verification
        Ok(format!("m3:{},b3:{}", murmur3_hash, blake3_hash))
    }

    /// Calculate source-specific seed for collision resistance
    fn calculate_source_seed(&self, source: &str) -> u32 {
        murmur3_hash(source.as_bytes(), 0xDEADBEEF)
    }

    /// Process buffered stream in batches for higher throughput
    pub async fn process_batch(&self) -> Result<Vec<StreamHashResult>> {
        let mut buffer = self.buffer.write().await;
        let batch_size = std::cmp::min(buffer.len(), self.config.buffer_size);

        if batch_size == 0 {
            return Ok(Vec::new());
        }

        let mut results = Vec::with_capacity(batch_size);
        let start_time = std::time::Instant::now();

        // Process batch in parallel if enabled
        if self.config.parallel && batch_size > 1 {
            let packets: Vec<StreamPacket> = buffer.drain(..batch_size).collect();

            let futures: Vec<_> = packets.into_iter().map(|packet| {
                self.process_stream_packet(packet)
            }).collect();

            let batch_results = futures::future::join_all(futures).await;

            for result in batch_results {
                match result {
                    Ok(hash_result) => results.push(hash_result),
                    Err(e) => {
                        let mut stats = self.stats.write().await;
                        stats.errors += 1;
                        debug!("Batch processing error: {}", e);
                    }
                }
            }
        } else {
            // Sequential processing for small batches
            for _ in 0..batch_size {
                if let Some(packet) = buffer.pop_front() {
                    match self.process_stream_packet(packet).await {
                        Ok(result) => results.push(result),
                        Err(e) => {
                            let mut stats = self.stats.write().await;
                            stats.errors += 1;
                            debug!("Sequential processing error: {}", e);
                        }
                    }
                }
            }
        }

        let batch_time = start_time.elapsed();
        info!("Processed batch of {} packets in {:?}", results.len(), batch_time);

        Ok(results)
    }

    /// Add packet to buffer for batch processing
    pub async fn buffer_packet(&self, packet: StreamPacket) -> Result<()> {
        let mut buffer = self.buffer.write().await;

        if buffer.len() >= self.config.buffer_size {
            // Buffer full, drop oldest packet (FIFO)
            buffer.pop_front();
            let mut stats = self.stats.write().await;
            stats.errors += 1;
        }

        buffer.push_back(packet);
        Ok(())
    }

    /// Get processing statistics
    pub async fn get_stats(&self) -> StreamStats {
        self.stats.read().await.clone()
    }

    /// Get current throughput (packets/second)
    pub async fn get_throughput(&self) -> f64 {
        let stats = self.stats.read().await;
        if stats.total_time_ns > 0 {
            let time_seconds = stats.total_time_ns as f64 / 1_000_000_000.0;
            stats.packets_processed as f64 / time_seconds
        } else {
            0.0
        }
    }

    /// Get current data rate (MB/s)
    pub async fn get_data_rate(&self) -> f64 {
        let stats = self.stats.read().await;
        if stats.total_time_ns > 0 {
            let time_seconds = stats.total_time_ns as f64 / 1_000_000_000.0;
            let mb_processed = stats.total_bytes as f64 / (1024.0 * 1024.0);
            mb_processed / time_seconds
        } else {
            0.0
        }
    }
}

impl Default for StreamHasherConfig {
    fn default() -> Self {
        Self {
            buffer_size: 1000,
            algorithm: HashAlgorithm::Murmur3,
            mode: StreamMode::RealTime,
            parallel: true,
        }
    }
}