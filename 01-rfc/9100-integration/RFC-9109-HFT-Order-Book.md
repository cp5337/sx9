# RFC-9109 Addendum: HFT Order Book

**Status:** DRAFT
**Date:** December 26, 2025
**Layer:** L2 Plasma Defender

---

## 1. Dual Book Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        DUAL ORDER BOOK                                  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│   BLUE BOOK (Defender)              RED BOOK (Attacker)                │
│   ════════════════════              ═══════════════════                │
│                                                                         │
│   OSSEC alerts                      TETH chains                        │
│        ↓                                 ↓                              │
│   Defense daemons                   Kali tools                         │
│                                                                         │
│              ┌──────────────────────────────────┐                      │
│              │     SINGLE MATCHING ENGINE        │                      │
│              │                                   │                      │
│              │   Price-Time Priority Matching    │                      │
│              │   TTL Enforcement                 │                      │
│              │   Partial Fill Support            │                      │
│              │                                   │                      │
│              └──────────────────────────────────┘                      │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Order Structure

```
order_id | side | price(priority) | time_ns | size | hd4_phase | tool_id | ttl_us
```

| Field | Type | Description |
|-------|------|-------------|
| `order_id` | UUID | Unique order identifier |
| `side` | BLUE/RED | Which book |
| `price` | u8 (1-4) | Priority level (lower = higher priority) |
| `time_ns` | u64 | Nanosecond timestamp for time priority |
| `size` | u32 | Order size (threat magnitude / tool capacity) |
| `hd4_phase` | enum | Current HD4 phase (market state) |
| `tool_id` | String | Response daemon (BLUE) or Kali tool (RED) |
| `ttl_us` | u64 | Time-to-live in microseconds |

---

## 3. Price = Priority

| Price | Level | Description |
|-------|-------|-------------|
| **1** | CRITICAL | Nation-state, APT |
| **2** | HIGH | Sophisticated attack |
| **3** | MEDIUM | Standard threat |
| **4** | LOW | Script kiddie |

---

## 4. Matching Rules

1. **Price-Time Priority**
   - CRITICAL (price=1) matched first
   - Within same price, earliest timestamp wins

2. **TTL Enforcement**
   - Stale orders expire automatically
   - No matching against expired orders

3. **Partial Fills**
   - Threat partially mitigated = partial fill
   - Remaining size stays in book

---

## 5. Spread Calculation

```
BLUE spread = response_time - threat_urgency
RED spread  = tool_exec_time - detection_window
```

| Spread | BLUE (Defender) | RED (Attacker) |
|--------|-----------------|----------------|
| **Negative** | Response faster than needed (WINNING) | Tool completed before detection (UNDETECTED) |
| **Zero** | Exactly meeting SLA | Racing the clock |
| **Positive** | Response lagging (SLIPPING) | Tool too slow (DETECTED) |

---

## 6. Cross-Feed Loop

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         FEED LOOP                                       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│   BLUE fills ──────────────────► RED intel                             │
│   (blocked attacks)               (inform red team what's blocked)     │
│                                                                         │
│   RED fills  ──────────────────► BLUE learning                         │
│   (attacks executed)              (update defense based on attacks)    │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 7. Latency Budgets

| Operation | Budget |
|-----------|--------|
| Order insert | <5μs |
| Match | <10μs |
| Fill dispatch | <20μs |
| **Total matching** | **<50μs** |
| **End-to-end** | **<100μs** |

---

## 8. Integration Points

| Component | Role |
|-----------|------|
| **TETH** | RED order emitter (attack chains) |
| **OSSEC** | BLUE order emitter (threat alerts) |
| **Legion ECS** | State updates on fill |
| **Sledis** | Order book persistence |

---

## 9. HD4 Phase = Market State

| HD4 Phase | Market Equivalent | Description |
|-----------|-------------------|-------------|
| **HUNT** | Pre-market | Scanning, surveillance |
| **DETECT** | Market open | Active matching begins |
| **DISRUPT** | Volatility | Rapid fills, tight spreads |
| **DISABLE** | Circuit breaker | Trading halt, crisis mode |
| **DOMINATE** | Market close | Cleanup, position reconciliation |

---

## 10. Rust Structures

```rust
/// Order entry
#[derive(Debug, Clone)]
pub struct Order {
    pub order_id: Uuid,
    pub side: BookSide,
    pub price: u8,           // 1-4, lower = higher priority
    pub time_ns: u64,
    pub size: u32,
    pub hd4_phase: HD4Phase,
    pub tool_id: String,
    pub ttl_us: u64,
}

/// Book side
#[derive(Debug, Clone, Copy)]
pub enum BookSide {
    Blue,  // Defender
    Red,   // Attacker
}

/// Fill result
#[derive(Debug, Clone)]
pub struct Fill {
    pub order_id: Uuid,
    pub side: BookSide,
    pub fill_size: u32,
    pub spread_us: i64,      // Negative = winning
    pub timestamp_ns: u64,
}

/// Dual order book
pub struct DualOrderBook {
    pub blue_orders: BTreeMap<(u8, u64), Order>,  // (price, time) → order
    pub red_orders: BTreeMap<(u8, u64), Order>,
    pub blue_responses: Vec<ResponseDaemon>,
    pub red_tools: Vec<KaliTool>,
}
```

---

## 11. Unicode Triggers (L2)

| Range | Purpose |
|-------|---------|
| U+E600-E6FF | Blue Book operations |
| U+E700-E7FF | Red Book operations |
| U+E800-E8FF | Cross-feed intelligence |
| U+E900-E9FF | Order types + HD4 phases |
