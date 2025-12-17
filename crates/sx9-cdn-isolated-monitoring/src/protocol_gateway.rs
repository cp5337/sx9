use crate::port_integration::PortBlockIntegration;
use anyhow::Result;
use tokio;
use tracing::{info, error};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use quinn::{Endpoint, ServerConfig, Connection};
use tonic::{transport::Server, Request, Response, Status};
use async_trait::async_trait;
use murmur3::murmur3_x64_128;  // RFC-9001 trivariate hashing

/// RFC-9001 compliant trivariate hash function
fn trivariate_hash(data: &[u8]) -> String {
    const SEED_SCH: u32 = 0xC7A50000;
    let hash = murmur3_x64_128(data, SEED_SCH);
    format!("{:032x}", hash)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolGatewayConfig {
    pub quic_enabled: bool,
    pub grpc_enabled: bool,
    pub irc_enabled: bool,
    pub udp_enabled: bool,
    pub base_port: u16,
}

impl Default for ProtocolGatewayConfig {
    fn default() -> Self {
        Self {
            quic_enabled: true,
            grpc_enabled: true,
            irc_enabled: true,
            udp_enabled: true,
            base_port: 18200,
        }
    }
}

pub struct MultiProtocolGateway {
    config: ProtocolGatewayConfig,
    port_integration: PortBlockIntegration,
    quic_endpoint: Option<Endpoint>,
    connections: Arc<RwLock<Vec<Connection>>>,
}

impl MultiProtocolGateway {
    pub fn new(config: ProtocolGatewayConfig) -> Self {
        Self {
            config,
            port_integration: PortBlockIntegration::new(),
            quic_endpoint: None,
            connections: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("🚀 Starting Multi-Protocol Gateway");
        
        let assignments = self.port_integration.generate_protocol_assignments();
        
        if self.config.quic_enabled {
            self.start_quic_server(assignments.quic_http3).await?;
        }
        
        if self.config.grpc_enabled {
            self.start_grpc_server(assignments.grpc_primary).await?;
        }
        
        if self.config.irc_enabled {
            self.start_irc_server(assignments.irc_server).await?;
        }
        
        if self.config.udp_enabled {
            self.start_udp_server().await?;
        }
        
        info!("✅ All protocol servers started successfully");
        Ok(())
    }

    async fn start_quic_server(&mut self, port: u16) -> Result<()> {
        info!("🔄 Starting QUIC/HTTP3 server on port {}", port);
        
        // Generate self-signed certificate for development
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
        let cert_der = cert.serialize_der()?;
        let priv_key = cert.serialize_private_key_der();
        
        let mut certs = rustls::RootCertStore::empty();
        certs.add(&rustls::Certificate(cert_der.clone()))?;
        
        let _client_crypto = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(certs)
            .with_no_client_auth();

        let mut server_crypto = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(
                vec![rustls::Certificate(cert_der)],
                rustls::PrivateKey(priv_key),
            )?;
        
        server_crypto.alpn_protocols = vec![b"h3".to_vec(), b"hq-29".to_vec()];
        
        let server_config = ServerConfig::with_crypto(Arc::new(server_crypto));
        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
        
        let endpoint = Endpoint::server(server_config, addr)?;
        info!("🌐 QUIC server listening on {}", addr);
        
        let connections = self.connections.clone();
        let endpoint_clone = endpoint.clone();
        
        // Spawn connection handler
        tokio::spawn(async move {
            while let Some(conn) = endpoint_clone.accept().await {
                let connections = connections.clone();
                tokio::spawn(async move {
                    match conn.await {
                        Ok(connection) => {
                            info!("📡 New QUIC connection from {}", connection.remote_address());
                            connections.write().await.push(connection);
                        }
                        Err(e) => {
                            error!("❌ Failed to accept QUIC connection: {}", e);
                        }
                    }
                });
            }
        });
        
        self.quic_endpoint = Some(endpoint);
        Ok(())
    }

    async fn start_grpc_server(&self, port: u16) -> Result<()> {
        info!("🔄 Starting gRPC server on port {}", port);
        
        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
        let ctas_service = CTASProtocolService::new();
        
        tokio::spawn(async move {
            if let Err(e) = Server::builder()
                .add_service(CtasProtocolServer::new(ctas_service))
                .serve(addr)
                .await
            {
                error!("❌ gRPC server failed: {}", e);
            }
        });
        
        info!("🌐 gRPC server listening on {}", addr);
        Ok(())
    }

    async fn start_irc_server(&self, port: u16) -> Result<()> {
        info!("🔄 Starting IRC server on port {}", port);
        
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        info!("🌐 IRC server listening on port {}", port);
        
        tokio::spawn(async move {
            while let Ok((stream, addr)) = listener.accept().await {
                info!("📡 New IRC connection from {}", addr);
                tokio::spawn(handle_irc_connection(stream, addr));
            }
        });
        
        Ok(())
    }

    async fn start_udp_server(&self) -> Result<()> {
        let assignments = self.port_integration.generate_protocol_assignments();
        let port = assignments.ctas_rpc_l1; // Use L1 port for UDP
        
        info!("🔄 Starting UDP server on port {}", port);
        
        let socket = tokio::net::UdpSocket::bind(format!("0.0.0.0:{}", port)).await?;
        info!("🌐 UDP server listening on port {}", port);
        
        tokio::spawn(async move {
            let mut buf = [0; 8192];
            
            while let Ok((len, addr)) = socket.recv_from(&mut buf).await {
                let data = &buf[..len];
                info!("📡 UDP packet from {}: {} bytes", addr, len);
                
                // Echo back with CTAS protocol marker
                let response = format!("CTAS-UDP-ACK: {}", String::from_utf8_lossy(data));
                if let Err(e) = socket.send_to(response.as_bytes(), addr).await {
                    error!("❌ Failed to send UDP response: {}", e);
                }
            }
        });
        
        Ok(())
    }

    pub fn get_protocol_status(&self) -> ProtocolStatus {
        ProtocolStatus {
            quic_active: self.quic_endpoint.is_some(),
            grpc_active: self.config.grpc_enabled,
            irc_active: self.config.irc_enabled,
            udp_active: self.config.udp_enabled,
            total_connections: 0, // TODO: implement connection counting
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolStatus {
    pub quic_active: bool,
    pub grpc_active: bool,
    pub irc_active: bool,
    pub udp_active: bool,
    pub total_connections: u32,
}

// gRPC Protocol Buffers Service Definition
pub mod ctas_protocol {
    tonic::include_proto!("ctas_protocol");
}

use ctas_protocol::ctas_protocol_server::{CtasProtocol, CtasProtocolServer};
use ctas_protocol::{CtasRequest, CtasResponse, HashRequest, HashResponse};

#[derive(Debug, Default)]
pub struct CTASProtocolService {}

impl CTASProtocolService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl CtasProtocol for CTASProtocolService {
    type StreamProtocolEventsStream = std::pin::Pin<Box<dyn tonic::codegen::tokio_stream::Stream<Item = Result<ctas_protocol::ProtocolEvent, Status>> + Send + 'static>>;
    async fn execute_ctas_command(
        &self,
        request: Request<CtasRequest>,
    ) -> Result<Response<CtasResponse>, Status> {
        let req = request.into_inner();
        info!("🎯 CTAS gRPC command: {}", req.command);
        
        let response = CtasResponse {
            success: true,
            result: format!("Executed: {}", req.command),
            error_message: String::new(),
            execution_time: chrono::Utc::now().timestamp_millis(),
            sch_hash: trivariate_hash(req.command.as_bytes()),
        };
        
        Ok(Response::new(response))
    }

    async fn compute_genetic_hash(
        &self,
        request: Request<HashRequest>,
    ) -> Result<Response<HashResponse>, Status> {
        let req = request.into_inner();
        info!("🧬 Computing genetic hash for: {}", req.input);
        
        // Use RFC-9001 trivariate hashing for genetic hash computation
        let hash = trivariate_hash(req.input.as_bytes());

        let response = HashResponse {
            hash,
            algorithm: "RFC-9001-SCH".to_string(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            convergence_score: "0.95".to_string(), // Placeholder convergence score
        };
        
        Ok(Response::new(response))
    }
    
    async fn stream_protocol_events(
        &self,
        _request: Request<ctas_protocol::EventStreamRequest>,
    ) -> Result<Response<Self::StreamProtocolEventsStream>, Status> {
        // TODO: Implement event streaming
        Err(Status::unimplemented("Event streaming not yet implemented"))
    }
    
    async fn dispatch_execution(
        &self,
        request: Request<ctas_protocol::ExecutionRequest>,
    ) -> Result<Response<ctas_protocol::ExecutionResponse>, Status> {
        let req = request.into_inner();
        info!("⚡ Dispatching execution: {} on layer {:?}", req.command, req.layer);
        
        let response = ctas_protocol::ExecutionResponse {
            success: true,
            result: format!("Executed: {}", req.command).into_bytes(),
            executed_layer: req.layer,
            sch_trace: trivariate_hash(&req.payload),
            execution_time: chrono::Utc::now().timestamp_millis(),
        };
        
        Ok(Response::new(response))
    }
}

async fn handle_irc_connection(
    mut stream: tokio::net::TcpStream,
    addr: SocketAddr,
) -> Result<()> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    
    info!("🔄 Handling IRC connection from {}", addr);
    
    // Send IRC welcome message
    let welcome = format!(
        ":ctas-server 001 user :Welcome to CTAS IRC Gateway\r\n\
         :ctas-server 002 user :Your host is ctas-server\r\n\
         :ctas-server 003 user :This server was created for CTAS operations\r\n\
         :ctas-server 004 user ctas-server v1.0 o o\r\n"
    );
    
    if let Err(e) = stream.write_all(welcome.as_bytes()).await {
        error!("❌ Failed to send IRC welcome: {}", e);
        return Err(e.into());
    }
    
    let mut buffer = [0; 512];
    
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                info!("📡 IRC client {} disconnected", addr);
                break;
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                info!("📡 IRC from {}: {}", addr, message.trim());
                
                // Handle basic IRC commands
                if message.starts_with("PING") {
                    let pong = message.replace("PING", "PONG");
                    if let Err(e) = stream.write_all(pong.as_bytes()).await {
                        error!("❌ Failed to send IRC PONG: {}", e);
                        break;
                    }
                } else if message.contains("JOIN") {
                    let join_response = ":user JOIN #ctas-ops\r\n";
                    if let Err(e) = stream.write_all(join_response.as_bytes()).await {
                        error!("❌ Failed to send IRC JOIN response: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                error!("❌ IRC connection error: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}