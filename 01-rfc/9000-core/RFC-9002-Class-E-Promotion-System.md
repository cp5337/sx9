# RFC-9002 Addendum: Class E Promotion System

**Version:** 2.1  
**Date:** December 14, 2025  
**Purpose:** Define promotion paths for Class E UI runes through execution contexts

---

## 1. Promotion Architecture

### 1.1 Concept

**Class E runes can be "promoted" to other classes to track UI interactions through the full execution stack.**

```
User Action (Class E) → Promotion → Execution (Class A/C/D)
                                  → Response tracking
                                  → Lineage preservation
```

### 1.2 Promotion Path

```
┌─────────────────────────────────────────────────────────────┐
│ 1. UI Event (Class E)                                       │
│    User clicks button U+E760 → "Execute nmap scan"          │
│    H1: UI operation hash                                    │
│    H2: Semantic context hash                                │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Promotion to Class A (Direct Execution)                  │
│    U+E760 promotes to U+E000 (nmap)                         │
│    Lineage: E760 → E000                                     │
│    Correlation ID: Links UI event to execution              │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. Execution (Class A)                                      │
│    nmap executes directly                                   │
│    Emits telemetry with correlation ID                      │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. Response Tracking                                        │
│    Execution result → Class E response rune                 │
│    U+E7F0 (success) or U+E7F1 (failure)                     │
│    Correlation ID preserved                                 │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. Promotion Encoding

### 2.1 Promotion Metadata

Each promotion carries metadata:

```rust
struct PromotionContext {
    source_rune: char,        // Original Class E rune
    target_rune: char,        // Promoted rune (Class A/C/D)
    correlation_id: Uuid,     // Links UI → execution → response
    h1_operational: String,   // H1 hash from UI element
    h2_semantic: String,      // H2 hash from UI element
    timestamp: DateTime,      // Promotion timestamp
    user_context: UserContext, // User who triggered
    lineage: Vec<char>,       // Full promotion chain
}
```

### 2.2 Correlation ID Format

```
corr:{UI_RUNE}_{EXEC_RUNE}_{TIMESTAMP}_{NONCE}

Example:
corr:E760_E000_20251214T120000_a1b2c3
```

---

## 3. Promotion Rules

### 3.1 Class E → Class A (Direct Execution)

**UI button triggers Kali tool:**

```
U+E760 (Button: "Run nmap scan")
    ↓ promotes to
U+E000 (nmap direct execution)

Promotion metadata:
- source: U+E760
- target: U+E000
- correlation_id: corr:E760_E000_...
- h1: {UI operation hash}
- h2: {semantic context}
```

### 3.2 Class E → Class C (Neural Mux Routing)

**UI action requires intelligent routing:**

```
U+E708 (Page: "Info Streams")
    ↓ promotes to
U+E34A (Neural Mux: cyber, tier-3, soft-delta)

Promotion metadata:
- source: U+E708
- target: U+E34A
- routing_context: {domain, tier, delta}
```

### 3.3 Class E → Class D (Complex Routing)

**UI action triggers multi-stage workflow:**

```
U+E706 (Page: "Tasks")
    ↓ promotes to
U+E400 (Complex routing: task orchestration)

Promotion metadata:
- source: U+E706
- target: U+E400
- workflow_stages: [fetch, analyze, route, execute]
```

---

## 4. Response Tracking

### 4.1 Response Runes (U+E7F0-E7FF)

Reserved Class E range for execution responses:

| Rune        | Status    | Description                      |
| :---------- | :-------- | :------------------------------- |
| U+E7F0      | Success   | Execution completed successfully |
| U+E7F1      | Failure   | Execution failed                 |
| U+E7F2      | Timeout   | Execution timed out              |
| U+E7F3      | Partial   | Partial success                  |
| U+E7F4      | Queued    | Execution queued                 |
| U+E7F5      | Running   | Execution in progress            |
| U+E7F6      | Cancelled | User cancelled                   |
| U+E7F7-E7FF | Reserved  | Future statuses                  |

### 4.2 Response Payload

```rust
struct ExecutionResponse {
    correlation_id: Uuid,      // Links to original UI event
    status_rune: char,         // U+E7F0-E7FF
    result: ExecutionResult,   // Execution output
    duration_ms: u64,          // Execution time
    error: Option<String>,     // Error message if failed
    telemetry: Telemetry,      // Full telemetry data
}
```

---

## 5. Lineage Tracking

### 5.1 Promotion Chain

Track full promotion chain from UI to execution:

```
Lineage: [U+E760, U+E000, U+E7F0]
         │       │       │
         │       │       └─ Response (success)
         │       └─ Execution (nmap)
         └─ UI Event (button click)
```

### 5.2 Database Schema

```sql
CREATE TABLE promotion_lineage (
    id UUID PRIMARY KEY,
    correlation_id TEXT NOT NULL UNIQUE,
    source_rune CHAR(1) NOT NULL,      -- Class E UI rune
    target_rune CHAR(1),                -- Promoted rune
    response_rune CHAR(1),              -- Response status
    h1_operational TEXT,
    h2_semantic TEXT,
    user_id UUID,
    timestamp TIMESTAMPTZ DEFAULT NOW(),
    duration_ms INTEGER,
    status TEXT,
    error TEXT,
    lineage JSONB,                      -- Full promotion chain
    telemetry JSONB                     -- Execution telemetry
);

-- Index for fast correlation lookup
CREATE INDEX idx_promotion_correlation ON promotion_lineage(correlation_id);
CREATE INDEX idx_promotion_source ON promotion_lineage(source_rune);
CREATE INDEX idx_promotion_user ON promotion_lineage(user_id);
```

---

## 6. Implementation Example

### 6.1 Rust Promotion Handler

```rust
use uuid::Uuid;

pub struct PromotionHandler {
    lineage_db: Arc<Database>,
    neural_mux: Arc<NeuralMux>,
}

impl PromotionHandler {
    /// Promote Class E UI rune to execution rune
    pub async fn promote(
        &self,
        ui_rune: char,
        user_context: &UserContext,
    ) -> Result<PromotionContext> {
        // Validate source is Class E
        if !Self::is_class_e(ui_rune) {
            return Err(Error::InvalidSourceRune);
        }

        // Lookup UI element in modal inventory
        let ui_element = self.lookup_ui_element(ui_rune).await?;

        // Determine promotion target based on UI element action
        let target_rune = self.determine_promotion_target(&ui_element)?;

        // Generate correlation ID
        let correlation_id = Uuid::new_v4();

        // Create promotion context
        let promotion = PromotionContext {
            source_rune: ui_rune,
            target_rune,
            correlation_id,
            h1_operational: ui_element.h1_operational.clone(),
            h2_semantic: ui_element.h2_semantic.clone(),
            timestamp: Utc::now(),
            user_context: user_context.clone(),
            lineage: vec![ui_rune],
        };

        // Store in lineage database
        self.record_promotion(&promotion).await?;

        Ok(promotion)
    }

    /// Execute promoted rune and track response
    pub async fn execute_promoted(
        &self,
        promotion: &PromotionContext,
    ) -> Result<ExecutionResponse> {
        let start = Instant::now();

        // Execute based on target class
        let result = match Self::get_rune_class(promotion.target_rune) {
            RuneClass::A => self.execute_direct(promotion).await?,
            RuneClass::C => self.execute_via_neural_mux(promotion).await?,
            RuneClass::D => self.execute_complex(promotion).await?,
            _ => return Err(Error::InvalidPromotionTarget),
        };

        let duration_ms = start.elapsed().as_millis() as u64;

        // Determine response rune
        let status_rune = match &result {
            Ok(_) => '\u{E7F0}',      // Success
            Err(_) => '\u{E7F1}',     // Failure
        };

        // Create response
        let response = ExecutionResponse {
            correlation_id: promotion.correlation_id,
            status_rune,
            result,
            duration_ms,
            error: None,
            telemetry: self.collect_telemetry(promotion).await?,
        };

        // Update lineage with response
        self.record_response(&response).await?;

        Ok(response)
    }

    fn is_class_e(rune: char) -> bool {
        let codepoint = rune as u32;
        codepoint >= 0xE700 && codepoint < 0xE800
    }

    fn get_rune_class(rune: char) -> RuneClass {
        let codepoint = rune as u32;
        match codepoint {
            c if c >= 0xE000 && c < 0xE200 => RuneClass::A,
            c if c >= 0xE200 && c < 0xE300 => RuneClass::B,
            c if c >= 0xE300 && c < 0xE400 => RuneClass::C,
            c if c >= 0xE400 && c < 0xE700 => RuneClass::D,
            c if c >= 0xE700 && c < 0xE800 => RuneClass::E,
            _ => RuneClass::Unknown,
        }
    }
}
```

### 6.2 TypeScript Frontend Integration

```typescript
// Frontend promotion client
class PromotionClient {
  async promoteUIAction(
    uiRune: string,
    action: UIAction
  ): Promise<PromotionContext> {
    // Send promotion request to backend
    const response = await fetch("/api/promote", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        ui_rune: uiRune,
        action: action,
        h1: action.h1_operational,
        h2: action.h2_semantic,
      }),
    });

    return await response.json();
  }

  async trackExecution(correlationId: string): Promise<ExecutionResponse> {
    // Poll for execution status
    const response = await fetch(`/api/promotion/${correlationId}/status`);

    return await response.json();
  }

  // WebSocket for real-time updates
  subscribeToExecution(
    correlationId: string,
    callback: (response: ExecutionResponse) => void
  ): void {
    const ws = new WebSocket(`ws://localhost:18120/promotion/${correlationId}`);

    ws.onmessage = (event) => {
      const response = JSON.parse(event.data);
      callback(response);
    };
  }
}

// Example usage
const client = new PromotionClient();

// User clicks "Run nmap" button (U+E760)
const promotion = await client.promoteUIAction("\uE760", {
  type: "execute_tool",
  tool: "nmap",
  h1_operational: "abc123...",
  h2_semantic: "mno345...",
});

// Subscribe to execution updates
client.subscribeToExecution(promotion.correlation_id, (response) => {
  console.log(`Status: ${response.status_rune}`);
  console.log(`Duration: ${response.duration_ms}ms`);

  if (response.status_rune === "\uE7F0") {
    console.log("✅ Execution successful!");
  }
});
```

---

## 7. Telemetry Integration

### 7.1 NATS Subject Topology

```
sx9.promotion.{correlation_id}.initiated
sx9.promotion.{correlation_id}.executing
sx9.promotion.{correlation_id}.completed
sx9.promotion.{correlation_id}.failed
```

### 7.2 Telemetry Payload

```json
{
  "correlation_id": "corr:E760_E000_20251214T120000_a1b2c3",
  "source_rune": "\uE760",
  "target_rune": "\uE000",
  "response_rune": "\uE7F0",
  "h1_operational": "abc123def456ghi7",
  "h2_semantic": "mno345pqr678stu9",
  "user_id": "user-uuid",
  "timestamp": "2025-12-14T12:00:00Z",
  "duration_ms": 1234,
  "status": "success",
  "lineage": ["\uE760", "\uE000", "\uE7F0"],
  "telemetry": {
    "cpu_usage": 45.2,
    "memory_mb": 128,
    "network_bytes": 4096
  }
}
```

---

## 8. Use Cases

### 8.1 UI Button → Kali Tool Execution

```
1. User clicks "Run nmap" button
   → U+E760 (Class E: button)

2. Frontend promotes to execution
   → U+E000 (Class A: nmap)
   → correlation_id: corr:E760_E000_...

3. Backend executes nmap directly
   → Emits telemetry with correlation_id

4. Execution completes
   → U+E7F0 (Class E: success response)

5. Frontend receives WebSocket update
   → Display results in UI
   → Update button state
```

### 8.2 UI Page → Complex Workflow

```
1. User navigates to "Tasks" page
   → U+E706 (Class E: page)

2. Frontend promotes to complex routing
   → U+E400 (Class D: task orchestration)
   → correlation_id: corr:E706_E400_...

3. Backend executes multi-stage workflow
   → Fetch tasks from database
   → Analyze task dependencies
   → Route to appropriate services
   → Aggregate results

4. Workflow completes
   → U+E7F0 (Class E: success response)

5. Frontend receives aggregated data
   → Render task list
   → Update page state
```

---

## 9. Benefits

1. **Full Traceability:** Track UI action → execution → response
2. **Debugging:** Correlation IDs link UI events to backend telemetry
3. **Performance Monitoring:** Measure end-to-end latency
4. **User Analytics:** Understand UI usage patterns
5. **Error Tracking:** Link UI errors to execution failures
6. **Audit Trail:** Complete lineage for compliance

---

## 10. References

- RFC-9001: Trivariate Hashing Standard
- RFC-9002: Unicode Operational Routing System
- RFC-9112: Deterministic Prompt Engineering
- RFC-9114: SX9 Gateway Neural Retrofit
