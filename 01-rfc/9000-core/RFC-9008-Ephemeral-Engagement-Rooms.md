# RFC-9008 — Ephemeral Engagement Rooms & CDN Attack Surface Reduction

**Version:** 1.0  
**Status:** Implementation Specification  
**Date:** November 2025  
**Applies To:** Synaptix9, CTAS-7.3.1  
**Author:** CTAS Core Engineering Group  
**Dependencies:** RFC-9001, RFC-9004, RFC-9006, RFC-9007

---

## 1. Abstract

This RFC specifies **Ephemeral Engagement Rooms** - a native CTAS team collaboration system that uses hash+hydration patterns over existing infrastructure (NATS, CDN, Neural Mux). Additionally, this RFC defines **CDN attack surface reduction** measures to minimize exposure of the content delivery infrastructure.

**Core Principles:**
- Wire carries hashes only (96 bytes), never content
- Engagement key deletion = cryptographic death of all messages
- CDN sees only encrypted blobs with no correlation capability
- Zero new infrastructure - leverages existing CTAS stack

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    EPHEMERAL ENGAGEMENT ROOM ARCHITECTURE                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  OPERATOR A                              OPERATOR B                          │
│      │                                        ▲                              │
│      │ 1. Compose message                     │ 6. Read message              │
│      ▼                                        │                              │
│  ┌──────────────┐                        ┌──────────────┐                   │
│  │ Encrypt +    │                        │ Hydrate +    │                   │
│  │ Store to CDN │                        │ Decrypt      │                   │
│  └──────┬───────┘                        └──────▲───────┘                   │
│         │                                       │                            │
│         │ 2. Upload encrypted blob              │ 5. Fetch encrypted blob   │
│         ▼                                       │                            │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                         CLOUDFLARE R2 (CDN)                           │  │
│  │  Sees: Encrypted blobs keyed by UUID                                  │  │
│  │  Cannot: Decrypt, correlate, or identify content                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│         │                                       ▲                            │
│         │ 3. Return success                     │                            │
│         ▼                                       │                            │
│  ┌──────────────┐                        ┌──────────────┐                   │
│  │ Generate     │                        │ Receive      │                   │
│  │ Trivariate   │                        │ Trivariate   │                   │
│  └──────┬───────┘                        └──────▲───────┘                   │
│         │                                       │                            │
│         │ 4. Publish hash (96 bytes)            │                            │
│         ▼                                       │                            │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                         NATS (Internal Signaling)                     │  │
│  │  Subject: engagement.room.{room_id}                                   │  │
│  │  Payload: { sch_t, cuid_t, uuid, sender_id, timestamp }              │  │
│  │  Size: 96 bytes + metadata (~128 bytes total)                        │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  WIRE SEES: Fixed-size hash packets (no content, no patterns)               │
│  CDN SEES: Random UUIDs pointing to encrypted blobs                         │
│  ADVERSARY GETS: Nothing useful                                             │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Engagement Room Lifecycle

### 3.1 Room Creation

```rust
/// Create a new ephemeral engagement room
pub struct EngagementRoom {
    room_id: Uuid,
    engagement_secret: [u8; 32],
    created_at: DateTime<Utc>,
    created_by: OperatorId,
    transport_profile: TransportProfile,
    members: Vec<RoomMember>,
    ttl: Option<Duration>,  // Auto-destruct timer
}

impl EngagementRoom {
    /// Create new engagement room
    pub fn create(
        creator: &Operator,
        transport_profile: TransportProfile,
        ttl: Option<Duration>,
    ) -> Result<Self> {
        // Generate cryptographically secure engagement secret
        let mut engagement_secret = [0u8; 32];
        OsRng.fill_bytes(&mut engagement_secret);
        
        let room = Self {
            room_id: Uuid::now_v7(),
            engagement_secret,
            created_at: Utc::now(),
            created_by: creator.id,
            transport_profile,
            members: vec![RoomMember::new(creator)],
            ttl,
        };
        
        // Register NATS subject
        room.register_nats_subject()?;
        
        Ok(room)
    }
    
    /// Export engagement secret for out-of-band sharing
    /// WARNING: This is the only time the secret is exposed
    pub fn export_join_token(&self) -> JoinToken {
        JoinToken {
            room_id: self.room_id,
            engagement_secret: self.engagement_secret,
            nats_subject: self.nats_subject(),
            expires_at: Utc::now() + Duration::hours(24),
        }
    }
    
    fn nats_subject(&self) -> String {
        format!("engagement.room.{}", self.room_id)
    }
}
```

### 3.2 Joining a Room

```rust
/// Join token shared out-of-band (Signal, in-person, etc.)
#[derive(Serialize, Deserialize)]
pub struct JoinToken {
    room_id: Uuid,
    engagement_secret: [u8; 32],
    nats_subject: String,
    expires_at: DateTime<Utc>,
}

impl JoinToken {
    /// Serialize for sharing (base64 encoded, fits in SMS/QR)
    pub fn to_shareable(&self) -> String {
        let bytes = bincode::serialize(self).unwrap();
        base64::encode_config(&bytes, base64::URL_SAFE_NO_PAD)
    }
    
    /// Parse from shared token
    pub fn from_shareable(token: &str) -> Result<Self> {
        let bytes = base64::decode_config(token, base64::URL_SAFE_NO_PAD)?;
        Ok(bincode::deserialize(&bytes)?)
    }
}

impl EngagementRoom {
    /// Join existing room with token
    pub async fn join(token: JoinToken, operator: &Operator) -> Result<Self> {
        if token.expires_at < Utc::now() {
            return Err(Error::TokenExpired);
        }
        
        // Reconstruct room from token
        let room = Self {
            room_id: token.room_id,
            engagement_secret: token.engagement_secret,
            transport_profile: TransportProfile::Tunneled,  // Safe default
            // ... other fields hydrated from NATS or CDN
        };
        
        // Subscribe to NATS subject
        room.subscribe_nats().await?;
        
        Ok(room)
    }
}
```

### 3.3 Room Destruction

```rust
impl EngagementRoom {
    /// Destroy room - all messages become unreadable
    pub async fn destroy(&mut self) -> Result<()> {
        // 1. Zero the engagement secret (messages now undecryptable)
        self.engagement_secret.zeroize();
        
        // 2. Unsubscribe from NATS
        self.nats_client.unsubscribe(&self.nats_subject()).await?;
        
        // 3. Optionally delete CDN blobs (not required - they're garbage now)
        if self.aggressive_cleanup {
            self.delete_all_blobs().await?;
        }
        
        // 4. Notify other members (last message)
        self.broadcast_room_closed().await?;
        
        Ok(())
    }
    
    /// Auto-destruct check (called periodically)
    pub fn check_ttl(&mut self) -> bool {
        if let Some(ttl) = self.ttl {
            if Utc::now() > self.created_at + ttl {
                self.destroy().await.ok();
                return true;
            }
        }
        false
    }
}
```

---

## 4. Message Format & Flow

### 4.1 Message Structure

```rust
/// Room message - content never travels on wire
#[derive(Clone)]
pub struct RoomMessage {
    // Metadata (travels on NATS)
    pub message_id: Uuid,
    pub sender_id: OperatorId,
    pub timestamp: DateTime<Utc>,
    pub message_type: MessageType,
    pub trivariate: Trivariate,
    
    // Content (stored encrypted on CDN, never on wire)
    // This field is only populated after hydration
    pub content: Option<MessageContent>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    File { filename: String, size: u64 },
    Image { dimensions: (u32, u32) },
    Voice { duration_secs: u32 },
    SystemEvent(SystemEvent),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    MemberJoined(OperatorId),
    MemberLeft(OperatorId),
    RoomClosing,
    KeyRotation,
}

/// What travels on NATS (fixed size, no content)
#[derive(Serialize, Deserialize)]
pub struct WireMessage {
    pub message_id: Uuid,           // 16 bytes
    pub sender_id: [u8; 16],        // 16 bytes
    pub timestamp: i64,             // 8 bytes
    pub message_type: u8,           // 1 byte
    pub trivariate: WireTrivariate, // 48 bytes
    pub signature: [u8; 64],        // 64 bytes (Ed25519)
    // Total: ~153 bytes (fixed)
}

#[derive(Serialize, Deserialize)]
pub struct WireTrivariate {
    pub sch_t: u128,    // 16 bytes
    pub cuid_t: u128,   // 16 bytes  
    pub uuid: [u8; 16], // 16 bytes
}
```

### 4.2 Send Message Flow

```rust
impl EngagementRoom {
    /// Send message to room
    pub async fn send(&self, content: MessageContent) -> Result<RoomMessage> {
        // 1. Serialize content
        let plaintext = content.serialize()?;
        
        // 2. Derive message-specific key from engagement secret + message ID
        let message_id = Uuid::now_v7();
        let message_key = self.derive_message_key(&message_id);
        
        // 3. Encrypt content
        let nonce = generate_nonce();
        let ciphertext = aes_gcm_encrypt(&plaintext, &message_key, &nonce)?;
        
        // 4. Upload to CDN (returns UUID for retrieval)
        let blob_id = self.cdn.put(&ciphertext, &nonce).await?;
        
        // 5. Generate trivariate for routing
        let trivariate = Trivariate {
            sch_t: generate_sch_t(&ciphertext, self.zone(), self.transport_profile),
            cuid_t: generate_cuid_t(&ciphertext, &self.context()),
            uuid: blob_id,
        };
        
        // 6. Create wire message (hash only, no content)
        let wire_msg = WireMessage {
            message_id,
            sender_id: self.operator.id.to_bytes(),
            timestamp: Utc::now().timestamp(),
            message_type: content.type_byte(),
            trivariate: trivariate.to_wire(),
            signature: self.operator.sign(&trivariate)?,
        };
        
        // 7. Publish to NATS (other operators receive hash)
        self.nats.publish(&self.nats_subject(), &wire_msg.serialize()?).await?;
        
        Ok(RoomMessage {
            message_id,
            sender_id: self.operator.id,
            timestamp: Utc::now(),
            message_type: content.message_type(),
            trivariate,
            content: Some(content),
        })
    }
    
    /// Derive per-message encryption key
    fn derive_message_key(&self, message_id: &Uuid) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new_keyed(&self.engagement_secret);
        hasher.update(message_id.as_bytes());
        hasher.update(b"ctas-room-message-v1");
        *hasher.finalize().as_bytes()
    }
}
```

### 4.3 Receive & Hydrate Flow

```rust
impl EngagementRoom {
    /// Handle incoming wire message
    pub async fn on_message(&self, wire_msg: WireMessage) -> Result<RoomMessage> {
        // 1. Verify signature
        let sender = self.get_member(&wire_msg.sender_id)?;
        sender.verify_signature(&wire_msg.trivariate, &wire_msg.signature)?;
        
        // 2. Hydrate content from CDN
        let content = self.hydrate(&wire_msg).await?;
        
        Ok(RoomMessage {
            message_id: wire_msg.message_id,
            sender_id: OperatorId::from_bytes(wire_msg.sender_id),
            timestamp: DateTime::from_timestamp(wire_msg.timestamp, 0).unwrap(),
            message_type: MessageType::from_byte(wire_msg.message_type),
            trivariate: wire_msg.trivariate.to_full(),
            content: Some(content),
        })
    }
    
    /// Hydrate content from CDN
    async fn hydrate(&self, wire_msg: &WireMessage) -> Result<MessageContent> {
        // 1. Fetch encrypted blob from CDN
        let (ciphertext, nonce) = self.cdn.get(&wire_msg.trivariate.uuid).await?;
        
        // 2. Derive message key
        let message_key = self.derive_message_key(&wire_msg.message_id);
        
        // 3. Decrypt
        let plaintext = aes_gcm_decrypt(&ciphertext, &message_key, &nonce)?;
        
        // 4. Deserialize content
        MessageContent::deserialize(&plaintext)
    }
}
```

---

## 5. CDN Attack Surface Reduction

### 5.1 Threat Model for CDN

| Threat | Attack Vector | Mitigation |
|--------|---------------|------------|
| Cloudflare compromise | Access to all blobs | Client-side encryption |
| Traffic analysis | Timing/size correlation | Fixed-size blobs, batched uploads |
| Access pattern analysis | Which blobs accessed together | Decoy fetches, random delays |
| Subpoena | Legal request to Cloudflare | Nothing to hand over (encrypted) |
| CDN credentials theft | Attacker uploads/deletes | Scoped tokens, no delete permission |
| Origin exposure | Direct attacks on origin | No origin - R2 only |

### 5.2 CDN Hardening Measures

```rust
/// Hardened CDN client with attack surface reduction
pub struct HardenedCdnClient {
    r2_client: CloudflareR2Client,
    config: CdnHardeningConfig,
}

#[derive(Clone)]
pub struct CdnHardeningConfig {
    // Blob padding to fixed sizes
    pub blob_size_buckets: Vec<usize>,  // e.g., [1KB, 4KB, 16KB, 64KB, 256KB]
    
    // Decoy traffic generation
    pub decoy_fetch_ratio: f32,  // Fetch X decoys per real fetch
    pub decoy_upload_interval: Duration,
    
    // Timing obfuscation
    pub random_delay_range: Range<Duration>,
    
    // Access pattern obfuscation
    pub batch_requests: bool,
    pub batch_size: usize,
    pub batch_interval: Duration,
    
    // Credential scoping
    pub read_only_token: bool,
    pub no_list_permission: bool,
    pub no_delete_permission: bool,
}

impl Default for CdnHardeningConfig {
    fn default() -> Self {
        Self {
            blob_size_buckets: vec![1024, 4096, 16384, 65536, 262144],
            decoy_fetch_ratio: 0.3,
            decoy_upload_interval: Duration::from_secs(60),
            random_delay_range: Duration::from_millis(50)..Duration::from_millis(500),
            batch_requests: true,
            batch_size: 5,
            batch_interval: Duration::from_secs(1),
            read_only_token: false,
            no_list_permission: true,
            no_delete_permission: true,
        }
    }
}
```

### 5.3 Blob Padding (Fixed-Size Buckets)

```rust
impl HardenedCdnClient {
    /// Pad blob to next bucket size (prevents size-based correlation)
    fn pad_to_bucket(&self, data: &[u8]) -> Vec<u8> {
        let target_size = self.config.blob_size_buckets
            .iter()
            .find(|&&size| size >= data.len())
            .copied()
            .unwrap_or(*self.config.blob_size_buckets.last().unwrap());
        
        let mut padded = Vec::with_capacity(target_size);
        
        // Format: [4 bytes length][data][random padding]
        padded.extend_from_slice(&(data.len() as u32).to_le_bytes());
        padded.extend_from_slice(data);
        
        // Fill remainder with random bytes
        let padding_len = target_size - padded.len();
        let mut padding = vec![0u8; padding_len];
        OsRng.fill_bytes(&mut padding);
        padded.extend_from_slice(&padding);
        
        padded
    }
    
    /// Remove padding after fetch
    fn unpad(&self, padded: &[u8]) -> Result<Vec<u8>> {
        if padded.len() < 4 {
            return Err(Error::InvalidBlob);
        }
        
        let len = u32::from_le_bytes(padded[..4].try_into()?) as usize;
        if len + 4 > padded.len() {
            return Err(Error::InvalidBlob);
        }
        
        Ok(padded[4..4 + len].to_vec())
    }
}
```

### 5.4 Decoy Traffic Generation

```rust
impl HardenedCdnClient {
    /// Generate decoy fetches to obscure real access patterns
    async fn fetch_with_decoys(&self, real_uuid: &Uuid) -> Result<Vec<u8>> {
        let mut uuids_to_fetch = vec![*real_uuid];
        
        // Add decoy UUIDs
        let decoy_count = (self.config.decoy_fetch_ratio * 1.0) as usize;
        for _ in 0..decoy_count {
            uuids_to_fetch.push(self.generate_decoy_uuid());
        }
        
        // Shuffle so real UUID isn't always first
        uuids_to_fetch.shuffle(&mut OsRng);
        
        // Fetch all (including decoys)
        let mut results = Vec::new();
        for uuid in &uuids_to_fetch {
            // Random delay between fetches
            let delay = self.random_delay();
            tokio::time::sleep(delay).await;
            
            let result = self.r2_client.get(uuid).await;
            results.push((*uuid, result));
        }
        
        // Return only the real one
        results
            .into_iter()
            .find(|(uuid, _)| uuid == real_uuid)
            .map(|(_, result)| result)
            .unwrap()
    }
    
    /// Upload decoy blobs periodically
    pub async fn run_decoy_uploader(&self) {
        loop {
            tokio::time::sleep(self.config.decoy_upload_interval).await;
            
            // Upload random decoy blob
            let decoy_data = self.generate_decoy_content();
            let uuid = Uuid::now_v7();
            self.put_internal(&uuid, &decoy_data).await.ok();
        }
    }
    
    fn generate_decoy_uuid(&self) -> Uuid {
        // Generate UUID that looks valid but points to decoy or nothing
        Uuid::now_v7()
    }
    
    fn generate_decoy_content(&self) -> Vec<u8> {
        let size = *self.config.blob_size_buckets.choose(&mut OsRng).unwrap();
        let mut data = vec![0u8; size];
        OsRng.fill_bytes(&mut data);
        data
    }
}
```

### 5.5 Request Batching

```rust
impl HardenedCdnClient {
    /// Batch multiple fetches into single timing window
    pub async fn batch_fetch(&self, uuids: &[Uuid]) -> Result<HashMap<Uuid, Vec<u8>>> {
        let mut results = HashMap::new();
        
        // Group into batches
        for chunk in uuids.chunks(self.config.batch_size) {
            let mut futures = Vec::new();
            
            for uuid in chunk {
                futures.push(self.fetch_single(uuid));
            }
            
            // Execute batch concurrently
            let batch_results = futures::future::join_all(futures).await;
            
            for (uuid, result) in chunk.iter().zip(batch_results) {
                if let Ok(data) = result {
                    results.insert(*uuid, data);
                }
            }
            
            // Delay between batches
            tokio::time::sleep(self.config.batch_interval).await;
        }
        
        Ok(results)
    }
}
```

### 5.6 Credential Scoping

```rust
/// Scoped CDN credentials with minimal permissions
pub struct ScopedCdnCredentials {
    // Read-only token for message fetching
    read_token: String,
    
    // Write-only token for message uploading  
    write_token: String,
    
    // No list permission - can't enumerate blobs
    // No delete permission - can't remove evidence
}

impl ScopedCdnCredentials {
    /// Generate from Cloudflare API
    pub async fn generate(cf_api_key: &str, bucket: &str) -> Result<Self> {
        let client = CloudflareApiClient::new(cf_api_key);
        
        // Create read-only token
        let read_token = client.create_api_token(TokenConfig {
            name: "ctas-read-only",
            permissions: vec![
                Permission::R2ObjectRead(bucket.to_string()),
            ],
            // No list, no delete
        }).await?;
        
        // Create write-only token
        let write_token = client.create_api_token(TokenConfig {
            name: "ctas-write-only",
            permissions: vec![
                Permission::R2ObjectWrite(bucket.to_string()),
            ],
            // No read, no list, no delete
        }).await?;
        
        Ok(Self { read_token, write_token })
    }
}
```

### 5.7 Obfuscated Naming

```rust
impl HardenedCdnClient {
    /// Generate obfuscated blob path (no correlation to content)
    fn obfuscated_path(&self, uuid: &Uuid) -> String {
        // Hash the UUID with a secret to prevent enumeration
        let mut hasher = blake3::Hasher::new_keyed(&self.path_secret);
        hasher.update(uuid.as_bytes());
        let hash = hasher.finalize();
        
        // Use first 32 chars of hash as path
        let hash_hex = hex::encode(&hash.as_bytes()[..16]);
        
        // Distribute across "directories" to prevent hot spots
        format!(
            "{}/{}/{}.enc",
            &hash_hex[0..2],
            &hash_hex[2..4],
            &hash_hex[4..]
        )
    }
}
```

---

## 6. Transport Profile Integration (RFC-9006)

### 6.1 Room Transport Escalation

```rust
impl EngagementRoom {
    /// Get transport profile based on HD4 phase
    pub fn effective_transport(&self) -> TransportProfile {
        let base = self.transport_profile;
        
        // HD4 phase can bump up transport security
        match self.current_hd4_phase {
            Hd4Phase::Hunt => base,
            Hd4Phase::Detect => base.max(TransportProfile::Internal),
            Hd4Phase::Disable => base.max(TransportProfile::Tunneled),
            Hd4Phase::Disrupt => TransportProfile::Obfuscated,
            Hd4Phase::Dominate => TransportProfile::Obfuscated,
        }
    }
    
    /// Escalate room transport (cannot de-escalate)
    pub fn escalate_transport(&mut self, new_profile: TransportProfile) -> Result<()> {
        if new_profile < self.transport_profile {
            return Err(Error::CannotDeescalate);
        }
        
        self.transport_profile = new_profile;
        self.broadcast_system_event(SystemEvent::TransportEscalated(new_profile)).await?;
        
        Ok(())
    }
}
```

### 6.2 Per-Message Transport Override

```rust
impl EngagementRoom {
    /// Send with specific transport (must be >= room minimum)
    pub async fn send_with_transport(
        &self,
        content: MessageContent,
        transport: TransportProfile,
    ) -> Result<RoomMessage> {
        if transport < self.transport_profile {
            return Err(Error::TransportBelowMinimum);
        }
        
        // Generate trivariate with specified transport
        let trivariate = Trivariate {
            sch_t: generate_sch_t(
                &content.serialize()?,
                self.zone(),
                transport,  // Use specified transport
            ),
            // ...
        };
        
        // Route through Neural Mux with transport
        self.neural_mux.route_with_transport(&trivariate, transport).await?;
        
        // ... rest of send flow
    }
}
```

---

## 7. Biometric Integration (RFC-9007)

### 7.1 Room Access Control

```rust
impl EngagementRoom {
    /// Verify operator biometric before room access
    pub async fn authenticate_access(&self, operator: &Operator) -> Result<AccessLevel> {
        let biometric_result = operator.biometric_gate.authenticate().await;
        
        match biometric_result {
            ExecutionMode::Operational => {
                // Full access
                Ok(AccessLevel::Full)
            }
            ExecutionMode::Honeypot => {
                // Duress detected - show fake room with fake messages
                self.trigger_duress_protocol(operator).await?;
                Ok(AccessLevel::Honeypot)
            }
            ExecutionMode::Decoy => {
                // Wrong biometric - no room access
                Err(Error::AuthenticationFailed)
            }
            ExecutionMode::Lockout => {
                // Too many failures
                Err(Error::DeviceLocked)
            }
        }
    }
    
    async fn trigger_duress_protocol(&self, operator: &Operator) -> Result<()> {
        // 1. Silent alert to other operators
        self.broadcast_covert_alert(CovertAlert::DuressDetected {
            operator_id: operator.id,
            timestamp: Utc::now(),
        }).await?;
        
        // 2. Switch to honeypot room state
        self.switch_to_honeypot_mode().await?;
        
        Ok(())
    }
}
```

### 7.2 Honeypot Room

```rust
/// Fake room shown during duress
pub struct HoneypotRoom {
    real_room_id: Uuid,
    fake_messages: Vec<RoomMessage>,
}

impl HoneypotRoom {
    /// Generate convincing fake room
    pub fn generate(real_room: &EngagementRoom) -> Self {
        let fake_messages = FakeMessageGenerator::generate(
            real_room.member_count(),
            real_room.created_at,
            100,  // Generate 100 fake messages
        );
        
        Self {
            real_room_id: real_room.room_id,
            fake_messages,
        }
    }
}
```

---

## 8. NATS Subject Hierarchy

```
engagement.
├── room.
│   ├── {room_id}           # Main room messages
│   │   ├── messages        # WireMessage payloads
│   │   ├── system          # System events (join/leave/close)
│   │   └── presence        # Online status
│   └── discovery           # Room announcements (encrypted)
│
├── operator.
│   └── {operator_id}
│       ├── direct          # Direct messages (not room)
│       └── alerts          # System alerts
│
└── system.
    ├── key_rotation        # Engagement key rotation notices
    └── emergency           # Emergency broadcasts
```

---

## 9. UI Integration

### 9.1 Plasma Component

```rust
/// Plasma UI component for engagement room
pub struct EngagementRoomPanel {
    room: EngagementRoom,
    message_list: MessageListComponent,
    input_field: MessageInputComponent,
    member_list: MemberListComponent,
}

impl PlasmaComponent for EngagementRoomPanel {
    fn render(&self, ctx: &RenderContext) -> Element {
        Column::new()
            .child(self.render_header())
            .child(self.message_list.render(ctx))
            .child(self.input_field.render(ctx))
            .child(self.member_list.render(ctx))
            .into()
    }
}
```

### 9.2 Mobile (sx9-mobile)

```swift
// Swift/SwiftUI for iPad
struct EngagementRoomView: View {
    @StateObject var room: EngagementRoomViewModel
    
    var body: some View {
        VStack {
            // Messages
            ScrollView {
                LazyVStack {
                    ForEach(room.messages) { message in
                        MessageBubble(message: message)
                    }
                }
            }
            
            // Input
            HStack {
                TextField("Message", text: $room.inputText)
                Button("Send") {
                    room.sendMessage()
                }
            }
        }
        .onAppear {
            room.authenticate()  // Biometric check
        }
    }
}
```

---

## 10. Implementation Checklist

| Component | Crate/File | Status |
|-----------|------------|--------|
| EngagementRoom | `ctas7-engagement/src/room.rs` | 🔴 Need |
| Message Flow | `ctas7-engagement/src/message.rs` | 🔴 Need |
| CDN Hardening | `ctas7-cdn/src/hardened.rs` | 🔴 Need |
| Blob Padding | `ctas7-cdn/src/padding.rs` | 🔴 Need |
| Decoy Traffic | `ctas7-cdn/src/decoy.rs` | 🔴 Need |
| NATS Integration | `ctas7-engagement/src/nats.rs` | 🔴 Need |
| Transport Integration | `ctas7-engagement/src/transport.rs` | 🔴 Need |
| Biometric Integration | `ctas7-engagement/src/auth.rs` | 🔴 Need |
| Honeypot Room | `ctas7-engagement/src/honeypot.rs` | 🔴 Need |
| Plasma Component | `ctas7-plasma/src/engagement.rs` | 🔴 Need |
| Mobile View | `sx9-mobile/Sources/Engagement/` | 🔴 Need |

---

## 11. Security Summary

### What Travels Where

| Channel | What Travels | Size |
|---------|--------------|------|
| NATS | Trivariate hash + metadata | ~150 bytes fixed |
| CDN | Encrypted blob | Bucket sizes (1K/4K/16K/64K/256K) |
| Wire (external) | TLS-wrapped NATS or CDN traffic | Padded, timed |

### What Each Party Sees

| Party | Sees | Cannot See |
|-------|------|------------|
| NATS | Hash references | Content |
| Cloudflare | Encrypted blobs | Content, correlation |
| Network observer | TLS traffic | Anything useful |
| Compromised operator | Their messages | Messages after key rotation |
| Adversary with device | Honeypot room | Real room |

### Key Destruction = Total Death

```
engagement_secret.zeroize()
    │
    ├── All message keys become underivable
    ├── All CDN blobs become random garbage
    ├── No recovery possible (no key escrow)
    └── Nothing to hand over under subpoena
```

---

**End of RFC-9008**
