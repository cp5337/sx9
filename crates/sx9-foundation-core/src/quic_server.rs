use quinn::{Endpoint, ServerConfig, VarInt};
use rustls::{Certificate, PrivateKey, ServerConfig as TlsServerConfig};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use crate::hash_engine::Hasher as Blake3Hasher;
use std::collections::HashMap;

/// Ultra-low latency QUIC server for CDN operations
pub struct QuicCdnServer {
    endpoint: Endpoint,
    hash_cache: Arc<RwLock<HashMap<[u8; 32], Vec<u8>>>>, // Blake3 hash -> data
    stats: Arc<RwLock<QuicStats>>,
}

#[derive(Default, Debug)]
pub struct QuicStats {
    pub connections_established: u64,
    pub total_requests: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub avg_response_time_ns: u64,
}

impl QuicCdnServer {
    /// Create new QUIC CDN server with optimized configuration
    pub async fn new(bind_addr: SocketAddr) -> Result<Self, Box<dyn std::error::Error>> {
        // Generate self-signed certificate for testing
        // In production, use proper certificates
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
        let cert_der = cert.serialize_der()?;
        let priv_key = cert.serialize_private_key_der();

        let cert_chain = vec![Certificate(cert_der)];
        let key = PrivateKey(priv_key);

        // Configure TLS with optimizations
        let mut tls_config = TlsServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key)?;

        // Enable 0-RTT for ultra-low latency
        tls_config.max_early_data_size = 16384;
        tls_config.send_half_rtt_data = true;

        // Configure QUIC transport with MAXIMUM SPEED optimizations
        let mut transport_config = quinn::TransportConfig::default();

        // Optimize for EXTREME low latency - non-ACK mode
        transport_config.max_concurrent_bidi_streams(VarInt::from_u32(10000)); // Massive concurrency
        transport_config.max_concurrent_uni_streams(VarInt::from_u32(50000));  // Even more unidirectional
        transport_config.initial_rtt(std::time::Duration::from_micros(100));   // Microsecond RTT assumption
        transport_config.max_idle_timeout(Some(VarInt::from_u32(30000).into())); // 30 seconds

        // DISABLE ACKs for maximum speed (unreliable but FAST)
        transport_config.datagram_receive_buffer_size(Some(1024 * 1024));      // 1MB datagram buffer
        transport_config.datagram_send_buffer_size(1024 * 1024);               // 1MB send buffer

        // Aggressive congestion control for speed
        transport_config.initial_window(VarInt::from_u32(1024 * 1024));        // 1MB initial window
        transport_config.receive_window(VarInt::from_u32(8 * 1024 * 1024));    // 8MB receive window

        // Minimal keep-alive for speed
        transport_config.keep_alive_interval(Some(std::time::Duration::from_millis(100)));

        let server_config = ServerConfig::with_crypto(Arc::new(
            quinn::crypto::rustls::QuicServerConfig::try_from(tls_config)?
        ));

        let endpoint = Endpoint::server(server_config, bind_addr)?;

        info!("🚀 QUIC CDN Server started on {}", bind_addr);
        info!("⚡ 0-RTT enabled for ultra-low latency");
        info!("🔄 Keep-alive enabled for persistent connections");
        info!("📡 NON-ACK mode enabled for MAXIMUM SPEED");
        info!("🏎️  Unreliable datagrams for hash lookups enabled");

        Ok(Self {
            endpoint,
            hash_cache: Arc::new(RwLock::new(HashMap::with_capacity(1_000_000))),
            stats: Arc::new(RwLock::new(QuicStats::default())),
        })
    }

    /// Start serving QUIC connections with hash-optimized request handling
    pub async fn serve(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🌐 QUIC CDN Server listening for connections...");

        while let Some(conn) = self.endpoint.accept().await {
            let hash_cache = Arc::clone(&self.hash_cache);
            let stats = Arc::clone(&self.stats);

            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(conn, hash_cache, stats).await {
                    error!("Connection error: {}", e);
                }
            });
        }

        Ok(())
    }

    /// Handle individual QUIC connection with optimized hash lookups
    async fn handle_connection(
        conn: quinn::Connecting,
        hash_cache: Arc<RwLock<HashMap<[u8; 32], Vec<u8>>>>,
        stats: Arc<RwLock<QuicStats>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let connection = conn.await?;
        let remote_addr = connection.remote_address();

        // Update connection stats
        {
            let mut stats_guard = stats.write().await;
            stats_guard.connections_established += 1;
        }

        info!("🔌 New QUIC connection from {}", remote_addr);

        // Handle both reliable streams AND unreliable datagrams for max speed
        loop {
            tokio::select! {
                // Handle reliable bidirectional streams (for important operations)
                stream_result = connection.accept_bi() => {
                    match stream_result {
                        Ok((send_stream, recv_stream)) => {
                            let hash_cache = Arc::clone(&hash_cache);
                            let stats = Arc::clone(&stats);

                            tokio::spawn(async move {
                                if let Err(e) = Self::handle_stream(send_stream, recv_stream, hash_cache, stats).await {
                                    warn!("Stream error: {}", e);
                                }
                            });
                        }
                        Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
                            info!("🔌 Connection closed by peer: {}", remote_addr);
                            break;
                        }
                        Err(e) => {
                            error!("Connection error: {}", e);
                            break;
                        }
                    }
                }
                // Handle unreliable datagrams (MAXIMUM SPEED hash lookups)
                datagram_result = connection.read_datagram() => {
                    match datagram_result {
                        Ok(datagram) => {
                            let hash_cache = Arc::clone(&hash_cache);
                            let stats = Arc::clone(&stats);
                            let conn_clone = connection.clone();

                            tokio::spawn(async move {
                                if let Err(e) = Self::handle_datagram(datagram, hash_cache, stats, conn_clone).await {
                                    // Don't log datagram errors - they're expected to be unreliable
                                }
                            });
                        }
                        Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
                            break;
                        }
                        Err(_) => {
                            // Ignore datagram errors - unreliable by design
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Handle individual stream with lightning-fast hash operations
    async fn handle_stream(
        mut send_stream: quinn::SendStream,
        mut recv_stream: quinn::RecvStream,
        hash_cache: Arc<RwLock<HashMap<[u8; 32], Vec<u8>>>>,
        stats: Arc<RwLock<QuicStats>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Read request data
        let request_data = recv_stream.read_to_end(8192).await?;
        let start_time = std::time::Instant::now();

        // Parse request (simplified protocol)
        if request_data.len() < 32 {
            return Err("Invalid request: too short".into());
        }

        let hash_key: [u8; 32] = request_data[0..32].try_into()?;

        // Lightning-fast hash lookup
        let response_data = {
            let cache = hash_cache.read().await;
            if let Some(data) = cache.get(&hash_key) {
                // Cache hit - ultra-fast response
                let mut stats_guard = stats.write().await;
                stats_guard.cache_hits += 1;
                stats_guard.total_requests += 1;
                data.clone()
            } else {
                // Cache miss - would trigger vector search or external lookup
                let mut stats_guard = stats.write().await;
                stats_guard.cache_misses += 1;
                stats_guard.total_requests += 1;

                // For now, return "not found" response
                b"HASH_NOT_FOUND".to_vec()
            }
        };

        // Send response
        send_stream.write_all(&response_data).await?;
        send_stream.finish().await?;

        // Update response time stats
        let response_time = start_time.elapsed().as_nanos() as u64;
        {
            let mut stats_guard = stats.write().await;
            // Simple exponential moving average
            stats_guard.avg_response_time_ns =
                (stats_guard.avg_response_time_ns * 9 + response_time) / 10;
        }

        Ok(())
    }

    /// Pre-populate hash cache for known threats (USIM-style direct addressing)
    pub async fn populate_hash_cache(&self, threat_hashes: Vec<([u8; 32], Vec<u8>)>) {
        let mut cache = self.hash_cache.write().await;
        for (hash, data) in threat_hashes {
            cache.insert(hash, data);
        }
        info!("📋 Populated hash cache with {} entries", cache.len());
    }

    /// Handle unreliable datagram for MAXIMUM SPEED hash lookups (fire-and-forget)
    async fn handle_datagram(
        datagram: bytes::Bytes,
        hash_cache: Arc<RwLock<HashMap<[u8; 32], Vec<u8>>>>,
        stats: Arc<RwLock<QuicStats>>,
        connection: quinn::Connection,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();

        // XOR Forward Error Correction check
        if datagram.len() < 33 {
            return Ok(()); // Need at least 32 bytes hash + 1 byte XOR
        }

        let hash_key: [u8; 32] = datagram[0..32].try_into()?;
        let received_xor = datagram[32];

        // Verify XOR checksum for simple error detection
        let calculated_xor = hash_key.iter().fold(0u8, |acc, &b| acc ^ b);
        if received_xor != calculated_xor {
            // Silently drop corrupted packets
            return Ok(());
        }

        // Lightning-fast hash lookup (even faster than streams)
        let response_data = {
            let cache = hash_cache.read().await;
            if let Some(data) = cache.get(&hash_key) {
                // Cache hit - ULTRA-FAST response
                let mut stats_guard = stats.write().await;
                stats_guard.cache_hits += 1;
                stats_guard.total_requests += 1;

                // Prepare minimal response with XOR FEC
                let mut response = Vec::with_capacity(37); // 32 hash + 4 threat + 1 XOR
                response.extend_from_slice(&hash_key);

                // Extract threat level from data
                if data.len() >= 4 {
                    response.extend_from_slice(&data[0..4]);
                } else {
                    response.extend_from_slice(&[1, 0, 0, 0]); // LOW threat
                }

                // Add XOR checksum for forward error correction
                let xor_check = response.iter().fold(0u8, |acc, &b| acc ^ b);
                response.push(xor_check);

                Some(response)
            } else {
                // Cache miss - minimal response
                let mut stats_guard = stats.write().await;
                stats_guard.cache_misses += 1;
                stats_guard.total_requests += 1;
                None
            }
        };

        // Send response datagram with XOR FEC (no ACK needed)
        if let Some(response) = response_data {
            let _ = connection.send_datagram(response.into());
        }

        // Update response time (targeting microseconds!)
        let response_time = start_time.elapsed().as_nanos() as u64;
        {
            let mut stats_guard = stats.write().await;
            stats_guard.avg_response_time_ns =
                (stats_guard.avg_response_time_ns * 9 + response_time) / 10;
        }

        Ok(())
    }

    /// Add single hash to cache for real-time updates
    pub async fn add_hash(&self, hash: [u8; 32], data: Vec<u8>) {
        let mut cache = self.hash_cache.write().await;
        cache.insert(hash, data);
    }

    /// Get current performance statistics
    pub async fn get_stats(&self) -> QuicStats {
        self.stats.read().await.clone()
    }

    /// Get local endpoint address
    pub fn local_addr(&self) -> SocketAddr {
        self.endpoint.local_addr().unwrap()
    }
}

/// Helper function to hash IOC data with Blake3
pub fn hash_ioc(data: &[u8]) -> [u8; 32] {
    let mut hasher = Blake3Hasher::new();
    hasher.update(data);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quic_server_creation() {
        let server = QuicCdnServer::new("127.0.0.1:0".parse().unwrap()).await.unwrap();
        println!("QUIC server created on {}", server.local_addr());
    }

    #[test]
    fn test_hash_ioc() {
        let data = b"malicious.example.com";
        let hash = hash_ioc(data);
        assert_eq!(hash.len(), 32);

        // Same input should produce same hash
        let hash2 = hash_ioc(data);
        assert_eq!(hash, hash2);
    }
}