# RFC-9117: Tool Response Block (U+E900-E9FF)

**Version:** 1.0  
**Status:** Draft  
**Date:** December 16, 2025  
**Author:** CTAS Core Engineering Group  
**Dependencies:** RFC-9002 (Unicode Routing), RFC-9001 (Trivariate Hashing)

---

## 1. Abstract

This RFC allocates **U+E900-E9FF** as **Class R (Response)** for encoding tool output data, completing the request-response loop for Kali tool execution.

---

## 2. Problem Statement

Current allocation has:
- **Triggers** (E000-E1FF for tools, E500-E51F for primitives)
- **Status codes** (E7E0-E7FF for success/failure/etc)

**Missing:** A block for encoding the actual tool **output payload** so it can be:
1. Routed back to the requester
2. Processed by downstream systems (ANN, GLAF, etc.)
3. Re-triggered for chained tool execution

---

## 3. Unicode Allocation

### 3.1 Class R: Tool Response Block (U+E900-E9FF)

```
U+E900-E90F: Output Format Markers (16 runes)
U+E910-E91F: Output Size Class (16 runes)
U+E920-E92F: Parser Hints (16 runes)
U+E930-E9BF: Tool-Specific Response Codes (144 runes)
U+E9C0-E9DF: Chaining Directives (32 runes)
U+E9E0-E9FF: Reserved (32 runes)
```

### 3.2 Output Format Markers (U+E900-E90F)

| Rune | Format | Description |
|------|--------|-------------|
| U+E900 | RAW | Raw bytes (no parsing) |
| U+E901 | JSON | JSON structured output |
| U+E902 | XML | XML structured output |
| U+E903 | TOML | TOML structured output |
| U+E904 | CSV | CSV tabular output |
| U+E905 | NDJSON | Newline-delimited JSON |
| U+E906 | GRPC | gRPC protobuf |
| U+E907 | MSGPACK | MessagePack binary |
| U+E908 | STDOUT | Plain text stdout |
| U+E909 | STDERR | Plain text stderr |
| U+E90A | MIXED | Combined stdout+stderr |
| U+E90B | BINARY | Binary blob (base64 in wire) |
| U+E90C | STREAM | Streaming output (chunked) |
| U+E90D | COMPRESSED | Compressed (zstd) |
| U+E90E | ENCRYPTED | Encrypted (AES-GCM) |
| U+E90F | RESERVED | Reserved |

### 3.3 Output Size Class (U+E910-E91F)

| Rune | Size Class | Range |
|------|------------|-------|
| U+E910 | TINY | < 256 bytes |
| U+E911 | SMALL | 256B - 4KB |
| U+E912 | MEDIUM | 4KB - 64KB |
| U+E913 | LARGE | 64KB - 1MB |
| U+E914 | XLARGE | 1MB - 16MB |
| U+E915 | HUGE | 16MB - 256MB |
| U+E916 | CHUNKED | Streaming (unknown size) |
| U+E917 | EMPTY | No output (0 bytes) |
| U+E918-E91F | RESERVED | Reserved |

### 3.4 Parser Hints (U+E920-E92F)

| Rune | Parser | Tool Family |
|------|--------|-------------|
| U+E920 | NMAP_XML | nmap -oX output |
| U+E921 | NMAP_GREP | nmap -oG output |
| U+E922 | MASSCAN_JSON | masscan -oJ output |
| U+E923 | NUCLEI_JSON | nuclei -json output |
| U+E924 | NIKTO_CSV | nikto -Format csv |
| U+E925 | SQLMAP_JSON | sqlmap --dump-format=JSON |
| U+E926 | HASHCAT_POT | hashcat potfile format |
| U+E927 | JOHN_POT | john potfile format |
| U+E928 | HARVESTER_JSON | theHarvester JSON |
| U+E929 | AMASS_JSON | amass JSON output |
| U+E92A | SUBFINDER_JSON | subfinder JSON |
| U+E92B | SIGMA_MATCH | Sigma rule match |
| U+E92C | YARA_MATCH | YARA rule match |
| U+E92D | CALDERA_OP | Caldera operation result |
| U+E92E | ATOMIC_TEST | Atomic Red Team result |
| U+E92F | GENERIC | Generic parser |

### 3.5 Tool-Specific Response Codes (U+E930-E9BF)

Organized by tool category (matching Class A triggers):

```
U+E930-E94F: Reconnaissance responses (nmap, masscan, etc.)
U+E950-E96F: Exploitation responses (metasploit, etc.)
U+E970-E98F: Password responses (hashcat, john, etc.)
U+E990-E9AF: Web application responses (sqlmap, nikto, etc.)
U+E9B0-E9BF: OSINT responses (theHarvester, amass, etc.)
```

### 3.6 Chaining Directives (U+E9C0-E9DF)

| Rune | Directive | Description |
|------|-----------|-------------|
| U+E9C0 | CHAIN_NEXT | Trigger next tool in chain |
| U+E9C1 | CHAIN_PARALLEL | Trigger parallel tools |
| U+E9C2 | CHAIN_CONDITIONAL | Conditional next (if success) |
| U+E9C3 | CHAIN_BRANCH | Branch based on output |
| U+E9C4 | CHAIN_MERGE | Merge multiple outputs |
| U+E9C5 | CHAIN_FILTER | Filter output before next |
| U+E9C6 | CHAIN_TRANSFORM | Transform output format |
| U+E9C7 | CHAIN_STORE | Store to arena, continue |
| U+E9C8 | CHAIN_EMIT | Emit to NATS, continue |
| U+E9C9 | CHAIN_TERMINATE | End chain |
| U+E9CA | CHAIN_RETRY | Retry on failure |
| U+E9CB | CHAIN_TIMEOUT | Timeout handling |
| U+E9CC | CHAIN_ESCALATE | Escalate tier |
| U+E9CD | CHAIN_DEESCALATE | De-escalate tier |
| U+E9CE-E9DF | RESERVED | Reserved |

---

## 4. Response Message Format

### 4.1 Wire Format

```
┌────────────────────────────────────────────────────────────────┐
│  TOOL RESPONSE MESSAGE (Variable size)                         │
├────────────────────────────────────────────────────────────────┤
│  Bytes 0-1:   Response Trigger (U+E9xx)                        │
│  Bytes 2-3:   Format Marker (U+E90x)                           │
│  Bytes 4-5:   Size Class (U+E91x)                              │
│  Bytes 6-7:   Parser Hint (U+E92x)                             │
│  Bytes 8-23:  HashRef (16 bytes) - Links to request            │
│  Bytes 24-27: Payload Length (u32)                             │
│  Bytes 28-31: Checksum (CRC32)                                 │
│  Bytes 32+:   Payload (variable)                               │
└────────────────────────────────────────────────────────────────┘
```

### 4.2 Rust Structure

```rust
/// Tool response header (32 bytes fixed)
#[repr(C, packed)]
pub struct ToolResponseHeader {
    /// Response trigger rune (U+E9xx)
    pub response_trigger: u16,
    /// Output format (U+E90x)
    pub format: u16,
    /// Size class (U+E91x)
    pub size_class: u16,
    /// Parser hint (U+E92x)
    pub parser_hint: u16,
    /// HashRef linking to original request
    pub hash_ref: HashRef,  // 16 bytes
    /// Payload length
    pub payload_len: u32,
    /// CRC32 checksum
    pub checksum: u32,
}

/// Complete tool response
pub struct ToolResponse {
    pub header: ToolResponseHeader,
    pub payload: Vec<u8>,
}

impl ToolResponse {
    /// Get chaining directive (if present in response trigger)
    pub fn chaining_directive(&self) -> Option<ChainingDirective> {
        if self.header.response_trigger >= 0xE9C0 
           && self.header.response_trigger <= 0xE9DF {
            Some(ChainingDirective::from_rune(self.header.response_trigger))
        } else {
            None
        }
    }
    
    /// Check if response triggers next tool in chain
    pub fn should_chain(&self) -> bool {
        matches!(
            self.chaining_directive(),
            Some(ChainingDirective::ChainNext) |
            Some(ChainingDirective::ChainParallel) |
            Some(ChainingDirective::ChainConditional)
        )
    }
}
```

---

## 5. Request-Response Loop

### 5.1 Complete Flow

```
┌─────────────────────────────────────────────────────────────────┐
│  1. TRIGGER (E000-E1FF or E500-E51F)                           │
│     → Tool execution request                                    │
│     → HashRef generated (links request)                         │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. EXECUTION (Rust FFI, no shell)                             │
│     → Tool runs via libsx9_ffi.so                              │
│     → Output captured to ring buffer                           │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. RESPONSE (E900-E9FF) ← THIS RFC                            │
│     → Format marker (E90x)                                      │
│     → Size class (E91x)                                         │
│     → Parser hint (E92x)                                        │
│     → HashRef (links to request)                                │
│     → Payload (tool output)                                     │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  4. PROCESSING                                                  │
│     → Parser extracts structured data                          │
│     → ANN evaluates threat level                               │
│     → Crystal resonance check                                  │
│     → SDT gate decision                                        │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  5. CHAINING (E9C0-E9DF)                                       │
│     → If CHAIN_NEXT: Trigger next tool                         │
│     → If CHAIN_CONDITIONAL: Check output, maybe trigger        │
│     → If CHAIN_TERMINATE: End chain                            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  6. EMIT                                                        │
│     → NATS: sx9.tool.response.{tool_name}                      │
│     → JetStream: SX9_TOOL_RESPONSES stream                     │
│     → Status: E7E0 (success) or E7E1 (failure)                 │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 NATS Subjects

```
# Request (trigger)
sx9.tool.trigger.{tool_name}      → Tool execution request
sx9.tool.trigger.chain.{chain_id} → Chain execution request

# Response (this RFC)
sx9.tool.response.{tool_name}     → Tool output response
sx9.tool.response.parsed.{tool}   → Parsed/structured response
sx9.tool.response.chain.{chain_id} → Chain step response

# Processing
sx9.tool.ann.evaluate             → ANN threat evaluation
sx9.tool.crystal.resonance        → Crystal resonance check
sx9.tool.sdt.gate                 → SDT gate decision

# Chaining
sx9.tool.chain.next               → Trigger next in chain
sx9.tool.chain.complete           → Chain completed
sx9.tool.chain.failed             → Chain failed
```

---

## 6. Example: nmap Chain

```rust
// 1. Trigger nmap scan
let request = ToolRequest {
    trigger: 0xE000,  // nmap
    target: "192.168.1.0/24",
    args: vec!["-sV", "-p1-1000"],
    hash_ref: HashRef::new_request(),
};
nats.publish("sx9.tool.trigger.nmap", request).await?;

// 2. Receive response
let response: ToolResponse = subscriber.next().await?;
assert_eq!(response.header.format, 0xE901);      // JSON
assert_eq!(response.header.parser_hint, 0xE920); // NMAP_XML
assert_eq!(response.header.hash_ref, request.hash_ref); // Links back!

// 3. Parse output
let parsed = NmapParser::parse(&response.payload)?;
// → hosts: 15, ports: 47, services: 23

// 4. Check chaining directive
if response.should_chain() {
    // Response says to trigger next tool
    let next_trigger = response.chaining_directive();
    match next_trigger {
        Some(ChainingDirective::ChainNext) => {
            // Trigger nuclei on discovered hosts
            for host in parsed.hosts {
                let nuclei_request = ToolRequest {
                    trigger: 0xE923,  // nuclei
                    target: host.ip,
                    hash_ref: HashRef::derive(
                        response.header.hash_ref,
                        DeltaClass::Micro,
                    ),
                };
                nats.publish("sx9.tool.trigger.nuclei", nuclei_request).await?;
            }
        }
        _ => {}
    }
}
```

---

## 7. Integration with Plasma Defender

The `sx9-plasma-defender` crate already has `tool_handler.rs` which subscribes to tool results. This RFC adds:

```rust
// tool_handler.rs additions

/// Process tool response with Class R encoding
pub async fn process_tool_response(
    response: &ToolResponse,
    ann_daemon: &AnnDaemon,
    crystal: &CrystalIntegration,
    sdt: &SdtIntegration,
) -> Result<ProcessingResult> {
    // 1. Validate response header
    validate_response_header(&response.header)?;
    
    // 2. Parse based on parser hint
    let parsed = match response.header.parser_hint {
        0xE920 => NmapParser::parse(&response.payload)?,
        0xE921 => NmapGrepParser::parse(&response.payload)?,
        0xE922 => MasscanParser::parse(&response.payload)?,
        0xE923 => NucleiParser::parse(&response.payload)?,
        // ... etc
        _ => GenericParser::parse(&response.payload)?,
    };
    
    // 3. Evaluate through ANN
    let ann_result = ann_daemon.evaluate(&parsed).await?;
    
    // 4. Check crystal resonance
    let resonance = crystal.check_resonance(&parsed)?;
    
    // 5. SDT gate decision
    let allowed = sdt.should_proceed(resonance)?;
    
    // 6. Handle chaining
    if response.should_chain() && allowed {
        handle_chaining(response, &parsed).await?;
    }
    
    Ok(ProcessingResult {
        parsed,
        ann_result,
        resonance,
        allowed,
    })
}
```

---

## 8. Updated Master Unicode Allocation

```
U+E000-E1FF: Class A - Direct Execution (Kali tools) [TRIGGER]
U+E200-E2FF: Class B - CUID Slot Runes
U+E300-E3FF: Class C - Semantic Routing
U+E400-E4FF: Class D - Complex Routing
U+E500-E51F: Class T - Command Triggers (32 primitives) [TRIGGER]
U+E520-E5FF: Class T - Reserved triggers
U+E600-E60F: Class P - Priority encoding
U+E610-E6FF: Class P - Reserved status
U+E700-E70F: Class S - Domain encoding
U+E710-E71F: Class S - Execution environment
U+E720-E72F: Class S - Execution state
U+E730-E7DF: Class E - UI Elements
U+E7E0-E7FF: Class E - Response status runes
U+E800-E80C: Class H - Lisp Heredity Operators
U+E80D-E8FF: Class H - Reserved heredity
U+E900-E9FF: Class R - Tool Response Block [RESPONSE] ← THIS RFC
U+EA00-EA2F: Class I - IAC Triggers
```

---

## 9. Output Hash Compression Pipeline

Tool outputs MUST go through the same hash → compress → short code pipeline as all other SX9 data.

### 9.1 Hash Flow

```
┌─────────────────────────────────────────────────────────────────┐
│  RAW TOOL OUTPUT                                                │
│  (e.g., 500KB nmap XML)                                         │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  1. CONTENT HASH (SHA-256 → 32 bytes)                          │
│     → Hash of raw payload                                       │
│     → Stored in arena for dedup                                 │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. DUAL TRIVARIATE HASH                                        │
│     → Semantic hash (what the output means)                     │
│     → Operational hash (what triggered it)                      │
│     → Combined into HashRef (16 bytes)                          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. UNICODE BASE96 ENCODING                                     │
│     → HashRef → 48-char Base96 string                           │
│     → Or: HashRef → 22-char Base96 (compressed)                 │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  4. LISP HEREDITY OPERATOR                                      │
│     → (cons request_hash response_hash)                         │
│     → Links response to request in hash lineage                 │
│     → Encoded as U+E801 + operands                              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  5. SHORT CODE                                                  │
│     → 4-8 character human-readable code                         │
│     → Maps to full HashRef in registry                          │
│     → e.g., "NMP7X2" → full nmap scan result                    │
└─────────────────────────────────────────────────────────────────┘
```

### 9.2 Output HashRef Structure

```rust
/// HashRef for tool output (16 bytes)
#[repr(C, packed)]
pub struct OutputHashRef {
    /// Semantic hash component (8 bytes)
    /// - Tool category (2 bytes)
    /// - Output type (2 bytes)  
    /// - Content signature (4 bytes)
    pub semantic: u64,
    
    /// Operational hash component (8 bytes)
    /// - Request HashRef (4 bytes, truncated)
    /// - Timestamp bucket (2 bytes)
    /// - Sequence number (2 bytes)
    pub operational: u64,
}

impl OutputHashRef {
    /// Create from tool response
    pub fn from_response(
        response: &ToolResponse,
        request_hash: &HashRef,
    ) -> Self {
        let content_hash = sha256(&response.payload);
        
        Self {
            semantic: Self::compute_semantic(
                response.header.parser_hint,  // Tool type
                response.header.format,       // Output format
                &content_hash[..4],           // Content signature
            ),
            operational: Self::compute_operational(
                request_hash,                 // Links to request
                Utc::now(),                   // Timestamp
            ),
        }
    }
    
    /// Encode to Base96 (22 chars compressed)
    pub fn to_base96(&self) -> String {
        base96::encode(&self.to_bytes())
    }
    
    /// Generate short code (4-8 chars)
    pub fn to_short_code(&self) -> String {
        // First 3 chars: tool prefix (NMP, NUC, MSC, etc.)
        // Next 3-5 chars: Base36 of hash prefix
        let tool_prefix = tool_prefix_from_parser(self.semantic);
        let hash_suffix = base36::encode(&self.to_bytes()[..4]);
        format!("{}{}", tool_prefix, hash_suffix)
    }
}
```

### 9.3 Heredity Linking

Every response is linked to its request via Lisp heredity:

```rust
/// Link response hash to request hash
pub fn link_response_to_request(
    request_hash: &HashRef,
    response_hash: &OutputHashRef,
) -> HeredityExpr {
    // (cons request response) - pairs the hashes
    HeredityExpr::Cons {
        car: Box::new(HeredityExpr::Quote(request_hash.clone())),
        cdr: Box::new(HeredityExpr::Quote(response_hash.to_hash_ref())),
    }
}

/// For chained tools, derive child hash from parent
pub fn derive_chain_hash(
    parent_response: &OutputHashRef,
    delta_class: DeltaClass,
) -> HashRef {
    // (derive parent delta) - creates child with delta angle
    HashRef::derive(parent_response.to_hash_ref(), delta_class)
}
```

### 9.4 Short Code Registry

Short codes are registered for quick lookup:

```rust
/// Short code registry entry
pub struct ShortCodeEntry {
    /// The short code (e.g., "NMP7X2")
    pub code: String,
    /// Full HashRef
    pub hash_ref: OutputHashRef,
    /// Tool that produced this
    pub tool: ToolType,
    /// Timestamp
    pub created: DateTime<Utc>,
    /// TTL (short codes expire)
    pub ttl: Duration,
    /// Arena slot where payload is stored
    pub arena_slot: u32,
}

impl ShortCodeRegistry {
    /// Register new output, get short code
    pub fn register(&mut self, response: &ToolResponse) -> String {
        let hash = OutputHashRef::from_response(response, &response.header.hash_ref);
        let code = hash.to_short_code();
        
        // Store in registry
        self.entries.insert(code.clone(), ShortCodeEntry {
            code: code.clone(),
            hash_ref: hash,
            tool: response.tool_type(),
            created: Utc::now(),
            ttl: Duration::hours(24),  // Configurable
            arena_slot: self.arena.store(&response.payload),
        });
        
        code
    }
    
    /// Lookup by short code
    pub fn lookup(&self, code: &str) -> Option<&ShortCodeEntry> {
        self.entries.get(code)
    }
}
```

### 9.5 Wire Format with Hash

Updated response header includes output hash:

```
┌────────────────────────────────────────────────────────────────┐
│  TOOL RESPONSE MESSAGE (with hash compression)                 │
├────────────────────────────────────────────────────────────────┤
│  Bytes 0-1:   Response Trigger (U+E9xx)                        │
│  Bytes 2-3:   Format Marker (U+E90x)                           │
│  Bytes 4-5:   Size Class (U+E91x)                              │
│  Bytes 6-7:   Parser Hint (U+E92x)                             │
│  Bytes 8-23:  Request HashRef (16 bytes) - Links to request    │
│  Bytes 24-39: Output HashRef (16 bytes) - Hash of this output  │ ← NEW
│  Bytes 40-47: Short Code (8 bytes, null-padded)                │ ← NEW
│  Bytes 48-51: Payload Length (u32)                             │
│  Bytes 52-55: Checksum (CRC32)                                 │
│  Bytes 56+:   Payload (variable, or arena reference)           │
└────────────────────────────────────────────────────────────────┘
```

### 9.6 Compression Options

For large outputs, compress before hashing:

```rust
pub enum OutputStorage {
    /// Inline (small outputs < 4KB)
    Inline(Vec<u8>),
    
    /// Compressed inline (4KB - 64KB)
    Compressed {
        algorithm: CompressionAlgorithm,
        data: Vec<u8>,
    },
    
    /// Arena reference (large outputs > 64KB)
    ArenaRef {
        slot: u32,
        offset: u32,
        length: u32,
        compressed: bool,
    },
    
    /// Content-addressed (deduped)
    ContentAddressed {
        content_hash: [u8; 32],
        arena_slot: u32,
    },
}

#[derive(Clone, Copy)]
pub enum CompressionAlgorithm {
    None = 0,
    Zstd = 1,
    Lz4 = 2,
    Snappy = 3,
}
```

---

## 10. Example: Full Pipeline

```rust
// 1. Tool executes, produces output
let raw_output: Vec<u8> = nmap_execute(target).await?;
// → 500KB XML

// 2. Create response with hashes
let request_hash = original_request.hash_ref;
let response = ToolResponse::new(raw_output, request_hash);
// → Automatically computes OutputHashRef

// 3. Get short code
let short_code = registry.register(&response);
// → "NMP7X2"

// 4. Link via heredity
let lineage = link_response_to_request(&request_hash, &response.output_hash);
// → (cons "REQ123" "NMP7X2")

// 5. Store compressed in arena
let storage = if response.payload.len() > 64_000 {
    OutputStorage::ArenaRef {
        slot: arena.store_compressed(&response.payload, Zstd),
        offset: 0,
        length: response.payload.len() as u32,
        compressed: true,
    }
} else {
    OutputStorage::Inline(response.payload)
};

// 6. Emit to NATS with short code
nats.publish(
    &format!("sx9.tool.response.{}", short_code),
    response.to_wire_format(),
).await?;

// 7. Later: retrieve by short code
let entry = registry.lookup("NMP7X2")?;
let payload = arena.retrieve(entry.arena_slot)?;
```

---

## 11. Implementation Checklist

- [ ] Add `[unicode.response_block]` to smart-crate.toml
- [ ] Create `ToolResponseHeader` struct in foundation-core
- [ ] Create `OutputHashRef` with dual trivariate hashing
- [ ] Implement `to_base96()` and `to_short_code()` methods
- [ ] Create `ShortCodeRegistry` for lookup
- [ ] Implement heredity linking (`cons`, `derive`)
- [ ] Add compression support (zstd, arena refs)
- [ ] Create parsers for 10 initial tools
- [ ] Update plasma-defender tool_handler.rs
- [ ] Add NATS subjects for responses
- [ ] Implement chaining directives
- [ ] Test full request-response-hash loop

---

**This RFC completes the trigger → execute → respond → hash → compress → short code → chain loop!**

