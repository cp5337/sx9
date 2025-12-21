# RFC-9006 — Secure Transport Profiles

**Version:** 1.0  
**Status:** Implementation Specification  
**Date:** November 2025  
**Applies To:** Synaptix9, CTAS-7.3.1  
**Author:** CTAS Core Engineering Group  
**Dependencies:** RFC-9001, RFC-9004, RFC-9100

---

## 1. Abstract

This RFC extends the Neural Mux and Port Manager to support **transport-aware routing** via transport profiles encoded in the SCH-T hash. Transport profiles provide different security/performance tradeoffs ranging from direct local sockets to fully obfuscated tunneled connections.

**Core Principle:** The tunnel is just another ratchet size. Same deterministic routing, different transport guarantees.

---

## 2. Transport Profile Definitions

### 2.1 Profile Enumeration

| Code | Profile | Description | Latency | Security |
|------|---------|-------------|---------|----------|
| `0x0` | DIRECT | Unix socket, localhost only | <1ms | Physical isolation |
| `0x1` | INTERNAL | WireGuard mesh, internal network | ~1ms | Encrypted P2P |
| `0x2` | ENCRYPTED | TLS 1.3 + app-layer AES-GCM | ~5ms | Standard secure |
| `0x3` | TUNNELED | Nested WireGuard via ephemeral relay | ~20ms | No correlation |
| `0x4` | OBFUSCATED | Rotating ports + domain fronting | ~30ms | Traffic analysis resistant |
| `0x5` | AIRGAP | QR/sneakernet, hash-only transfer | Manual | Physical separation |
| `0x6-0xF` | Reserved | Future use | - | - |

### 2.2 Profile Selection Criteria

```
CONTENT SENSITIVITY → BASE TRANSPORT:
────────────────────────────────────
Public threat intel (MITRE, Sigma)     → 0x2 ENCRYPTED
Team analysis artifacts                 → 0x1 INTERNAL
Engagement comms                        → 0x3 TUNNELED
Operator-specific / eyes-only          → 0x4 OBFUSCATED
Exfil from air-gapped target           → 0x5 AIRGAP

HD4 PHASE → MINIMUM TRANSPORT (can only bump UP):
─────────────────────────────────────────────────
HUNT      → No minimum (use content default)
DETECT    → Minimum 0x1 INTERNAL
DISABLE   → Minimum 0x3 TUNNELED
DISRUPT   → Force 0x4 OBFUSCATED
DOMINATE  → Operator override allowed
```

---

## 3. SCH-T Extension

### 3.1 Bit Layout

```
SCH-T: 128 bits
┌──────────┬──────────┬──────────┬─────────────────────────────────┐
│ Zone     │ Transport│ Priority │ Content Hash                    │
│ (4 bit)  │ (4 bit)  │ (8 bit)  │ (112 bit)                       │
│ [127:124]│ [123:120]│ [119:112]│ [111:0]                         │
└──────────┴──────────┴──────────┴─────────────────────────────────┘

Zone:      Bernoulli zone (A=0, B=1, C=2, D=3)
Transport: Transport profile (0x0-0xF)
Priority:  Routing priority (0-255, higher = more urgent)
Content:   Murmur3-128 of content (truncated to 112 bits)
```

### 3.2 SCH-T Generation

```rust
pub fn generate_sch_t(
    content: &[u8],
    zone: BernoulliZone,
    transport: TransportProfile,
    priority: u8,
) -> u128 {
    // Compute content hash (Murmur3-128)
    let content_hash = murmur3_128(content, SEED);
    
    // Truncate to 112 bits
    let content_bits = content_hash & ((1u128 << 112) - 1);
    
    // Pack metadata into upper 16 bits
    let zone_bits = (zone as u128) << 124;
    let transport_bits = (transport as u128) << 120;
    let priority_bits = (priority as u128) << 112;
    
    zone_bits | transport_bits | priority_bits | content_bits
}
```

### 3.3 SCH-T Extraction

```rust
pub fn extract_sch_t_metadata(sch_t: u128) -> SchMetadata {
    SchMetadata {
        zone: BernoulliZone::from_bits(((sch_t >> 124) & 0x0F) as u8),
        transport: TransportProfile::from_bits(((sch_t >> 120) & 0x0F) as u8),
        priority: ((sch_t >> 112) & 0xFF) as u8,
        content_hash: sch_t & ((1u128 << 112) - 1),
    }
}
```

---

## 4. Neural Mux Extension

### 4.1 Transport-Aware Routing

```rust
impl NeuralMux {
    /// Route with transport-aware path selection
    pub async fn route(&self, sch_t: u128) -> Result<RouteDecision> {
        let meta = extract_sch_t_metadata(sch_t);
        
        // Get endpoint with appropriate transport
        let endpoint = self.port_manager
            .get_endpoint(meta.zone, meta.transport)?;
        
        // Build route based on transport profile
        let route = match meta.transport {
            TransportProfile::Direct => {
                self.build_direct_route(endpoint)
            }
            TransportProfile::Internal => {
                self.build_wireguard_route(endpoint).await?
            }
            TransportProfile::Encrypted => {
                self.build_tls_route(endpoint).await?
            }
            TransportProfile::Tunneled => {
                self.build_tunneled_route(endpoint).await?
            }
            TransportProfile::Obfuscated => {
                self.build_obfuscated_route(endpoint).await?
            }
            TransportProfile::Airgap => {
                self.build_airgap_route(endpoint)
            }
        };
        
        Ok(RouteDecision {
            route,
            sch_t,
            transport: meta.transport,
            priority: meta.priority,
        })
    }
    
    async fn build_tunneled_route(&self, endpoint: Endpoint) -> Result<Route> {
        // Acquire ephemeral relay from pool
        let relay = self.tunnel_pool.acquire().await?;
        
        // Establish nested WireGuard tunnel
        let tunnel = self.wireguard.connect_nested(&relay).await?;
        
        Ok(Route::Tunneled {
            relay: relay.endpoint,
            tunnel_id: tunnel.id,
            final_endpoint: endpoint,
        })
    }
    
    async fn build_obfuscated_route(&self, endpoint: Endpoint) -> Result<Route> {
        // Get current rotating port
        let current_port = self.obfuscation.current_port(endpoint.base_port);
        
        // Get fronting config if available
        let fronting = self.obfuscation.fronting_config();
        
        Ok(Route::Obfuscated {
            apparent_host: fronting.sni,
            actual_host: fronting.host,
            port: current_port,
            next_rotation: self.obfuscation.next_rotation_time(),
        })
    }
}
```

---

## 5. Port Manager Extension

### 5.1 Extended Allocation

```rust
impl PortManager {
    /// Allocate endpoint with transport requirements
    pub fn allocate_endpoint(
        &self,
        zone: BernoulliZone,
        transport: TransportProfile,
    ) -> Result<Endpoint> {
        let base_port = self.allocate_port(zone)?;
        
        let transport_config = match transport {
            TransportProfile::Direct => {
                TransportConfig::unix_socket(base_port)
            }
            TransportProfile::Internal => {
                TransportConfig::wireguard(
                    self.mesh_peer_for_zone(zone)?,
                    base_port,
                )
            }
            TransportProfile::Encrypted => {
                TransportConfig::tls(base_port, self.tls_config.clone())
            }
            TransportProfile::Tunneled => {
                TransportConfig::tunneled(
                    self.tunnel_pool.reserve()?,
                    base_port,
                )
            }
            TransportProfile::Obfuscated => {
                TransportConfig::obfuscated(
                    base_port,
                    self.obfuscation_secret.clone(),
                )
            }
            TransportProfile::Airgap => {
                TransportConfig::airgap(self.qr_endpoint.clone())
            }
        };
        
        Ok(Endpoint {
            port: base_port,
            transport,
            config: transport_config,
            allocated_at: Instant::now(),
        })
    }
}
```

### 5.2 Extended Port Allocation Table

| Zone | Base Port Range | Available Transports |
|------|-----------------|----------------------|
| A (Neural Mux) | 18100-18199 | Direct, Internal |
| B (ATLAS/Cognitive) | 18200-18299 | Direct, Internal, Encrypted |
| C (GLAF/Analysis) | 18300-18399 | All |
| D (Storage/IAC) | 18400-18499 | All |
| Relay Pool | 18500-18599 | Tunneled, Obfuscated only |
| Ephemeral/Rotating | 18600-18999 | TOTP-based rotation |

---

## 6. Tunnel Pool

### 6.1 Ephemeral Relay Management

```rust
pub struct TunnelPool {
    relays: Vec<RelayNode>,
    wireguard: WireGuardManager,
    vultr_client: VultrClient,
}

impl TunnelPool {
    /// Acquire relay for tunneled transport
    pub async fn acquire(&self) -> Result<RelayLease> {
        // Find healthy relay with capacity
        let relay = self.select_healthy_relay()?;
        
        // Establish nested WireGuard tunnel
        let tunnel = self.wireguard.connect_nested(relay).await?;
        
        Ok(RelayLease {
            relay: relay.clone(),
            tunnel,
            acquired_at: Instant::now(),
        })
    }
    
    /// Spawn ephemeral relay (hourly VPS)
    pub async fn spawn_ephemeral(&mut self) -> Result<RelayNode> {
        let vps = self.vultr_client.create_instance(VultrConfig {
            region: "ewr",
            plan: "vc2-1c-1gb",
            os: "ubuntu-minimal",
            user_data: RELAY_INIT_SCRIPT,  // RAM-only WireGuard
        }).await?;
        
        // Exchange WireGuard keys
        let wg_pubkey = self.exchange_keys(&vps).await?;
        
        let relay = RelayNode {
            id: vps.id,
            endpoint: vps.main_ip,
            wg_pubkey,
            created_at: Utc::now(),
            ephemeral: true,
        };
        
        self.relays.push(relay.clone());
        Ok(relay)
    }
    
    /// Destroy relay (end of engagement)
    pub async fn destroy(&mut self, relay_id: &str) -> Result<()> {
        self.vultr_client.delete_instance(relay_id).await?;
        self.relays.retain(|r| r.id != relay_id);
        Ok(())
    }
}
```

### 6.2 Relay Init Script (RAM-Only)

```bash
#!/bin/bash
# relay-init.sh - Runs in RAM only, no disk persistence

# Mount tmpfs for all sensitive data
mount -t tmpfs -o size=100M tmpfs /dev/shm/wg

# Generate ephemeral keys in RAM
wg genkey | tee /dev/shm/wg/privatekey | wg pubkey > /dev/shm/wg/publickey

# Configure WireGuard from RAM
cat > /dev/shm/wg/wg0.conf << EOF
[Interface]
PrivateKey = $(cat /dev/shm/wg/privatekey)
Address = 10.200.200.1/24
ListenPort = 51820

# Peers added dynamically via API
EOF

wg-quick up /dev/shm/wg/wg0.conf

# Disable all logging
systemctl stop rsyslog
rm -rf /var/log/*
```

---

## 7. Obfuscation Layer

### 7.1 Port Rotation (TOTP-Based)

```rust
pub struct ObfuscationLayer {
    base_port: u16,
    secret: [u8; 32],
    rotation_interval: u64,  // seconds
    fronting_domains: Vec<String>,
}

impl ObfuscationLayer {
    /// Get current port based on TOTP
    pub fn current_port(&self, base: u16) -> u16 {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let interval = epoch / self.rotation_interval;
        let hmac = hmac_sha256(&self.secret, &interval.to_le_bytes());
        
        base + (u16::from_le_bytes([hmac[0], hmac[1]]) % 400)
    }
    
    /// Time until next port rotation
    pub fn next_rotation_time(&self) -> Duration {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let current_interval = epoch / self.rotation_interval;
        let next_interval_start = (current_interval + 1) * self.rotation_interval;
        
        Duration::from_secs(next_interval_start - epoch)
    }
    
    /// Get domain fronting configuration
    pub fn fronting_config(&self) -> FrontingConfig {
        FrontingConfig {
            sni: "cdn.cloudflare.com",  // TLS SNI shows Cloudflare
            host: self.fronting_domains
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap_or_else(|| "cdn.ctas.cloud".to_string()),
        }
    }
}
```

---

## 8. Airgap Transport

### 8.1 Hash-Only Transfer

```rust
pub struct AirgapTransport {
    qr_generator: QrGenerator,
    hash_verifier: HashVerifier,
}

impl AirgapTransport {
    /// Generate QR code for hash transfer
    pub fn generate_transfer_qr(&self, trivariate: &Trivariate) -> Result<Vec<u8>> {
        let payload = AirgapPayload {
            sch_t: trivariate.sch_t,
            cuid_t: trivariate.cuid_t,
            uuid: trivariate.uuid,
            signature: self.sign_payload(trivariate)?,
        };
        
        self.qr_generator.generate(&payload.to_bytes())
    }
    
    /// Verify received airgap payload
    pub fn verify_transfer(&self, qr_data: &[u8]) -> Result<Trivariate> {
        let payload = AirgapPayload::from_bytes(qr_data)?;
        
        // Verify signature
        self.verify_signature(&payload)?;
        
        Ok(Trivariate {
            sch_t: payload.sch_t,
            cuid_t: payload.cuid_t,
            uuid: payload.uuid,
        })
    }
}
```

---

## 9. Foundation Core Integration

### 9.1 Transport-Aware Trivariate Generation

```rust
impl FoundationCore {
    /// Generate trivariate with transport profile
    pub fn generate_trivariate(
        &self,
        content: &[u8],
        context: &OperationalContext,
    ) -> Trivariate {
        // Base transport from content classification
        let base_transport = self.classify_sensitivity(content);
        
        // HD4 phase can bump up (never down)
        let transport = self.apply_hd4_minimum(base_transport, context.hd4_phase);
        
        // Apply operator override if DOMINATE phase
        let final_transport = if context.hd4_phase == Hd4Phase::Dominate {
            context.operator_transport.unwrap_or(transport)
        } else {
            transport
        };
        
        // Generate SCH-T with transport bits
        let sch_t = generate_sch_t(
            content,
            context.zone,
            final_transport,
            context.priority,
        );
        
        Trivariate {
            sch_t,
            cuid_t: self.compute_cuid_t(content, context),
            uuid: Uuid::now_v7(),
        }
    }
    
    fn apply_hd4_minimum(
        &self,
        base: TransportProfile,
        phase: Hd4Phase,
    ) -> TransportProfile {
        let minimum = match phase {
            Hd4Phase::Hunt => TransportProfile::Direct,
            Hd4Phase::Detect => TransportProfile::Internal,
            Hd4Phase::Disable => TransportProfile::Tunneled,
            Hd4Phase::Disrupt => TransportProfile::Obfuscated,
            Hd4Phase::Dominate => TransportProfile::Direct,  // Operator chooses
        };
        
        base.max(minimum)
    }
}
```

---

## 10. Security Considerations

### 10.1 Transport Profile Security Properties

| Profile | Confidentiality | Integrity | Anonymity | Availability |
|---------|-----------------|-----------|-----------|--------------|
| DIRECT | Physical | Physical | N/A | High |
| INTERNAL | ChaCha20 | Poly1305 | Network-level | High |
| ENCRYPTED | AES-256-GCM | GCM MAC | None | High |
| TUNNELED | Nested ChaCha20 | Nested Poly1305 | Relay hop | Medium |
| OBFUSCATED | AES-256-GCM | GCM MAC | Traffic analysis resistant | Medium |
| AIRGAP | N/A (hash only) | Signature | Physical | Low |

### 10.2 Threat Model

| Threat | Mitigation |
|--------|------------|
| Network observer | ENCRYPTED minimum for external traffic |
| Compromised relay | Nested encryption - relay sees ciphertext only |
| Traffic analysis | OBFUSCATED profile with port rotation |
| Endpoint seizure | AIRGAP for sensitive content |
| Key compromise | Per-engagement keys, automatic rotation |

---

## 11. Implementation Status

| Component | File | Status |
|-----------|------|--------|
| Transport enum | `foundation_types.rs` | 🔴 Need |
| SCH-T extension | `foundation_core.rs` | 🔴 Need |
| Neural Mux routing | `neural_mux.rs` | 🔴 Need |
| Port Manager | `port_manager.rs` | 🔴 Need |
| Tunnel Pool | `tunnel_pool.rs` | 🔴 Need |
| Obfuscation Layer | `obfuscation.rs` | 🔴 Need |
| Airgap Transport | `airgap.rs` | 🔴 Need |

---

**End of RFC-9006**
