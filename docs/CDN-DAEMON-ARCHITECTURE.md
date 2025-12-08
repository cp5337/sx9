# CDN Daemon Architecture
## Should Each CDN Service Have a Daemon?

**Date:** December 2025  
**Status:** Architecture Decision  
**Question:** Should each CDN service have its own daemon component?

---

## Current Architecture

### Centralized Daemon (Current)
- **`ctas7-foundation-daemon`** (ATLAS) - Orchestrates all services
- **`SYNAPTIX9 Daemon Service`** - Manages CDN services via `CDNCoordinator`
- **CDN Services** - Axum servers (managed BY daemon, not daemons themselves)

### CDN Services (Managed by Daemon)
- `ctas7-cdn-data-fabric` (port 18100) - Axum server
- `ctas7-cdn-threat-intel` - Axum server
- `ctas7-cdn-geospatial` - Axum server
- `ctas7-cdn-monitoring` - Axum server
- `ctas7-cdn-threat-reaction` - Axum server
- `ctas7-cdn-statistical` - Axum server
- `ctas7-cdn-isolated-monitoring` - Axum server
- `ctas7-glaf-graph-server` (port 18050) - Axum server

---

## Two Architecture Options

### Option A: Centralized Daemon (Current)
**Pattern:** One daemon manages all CDN services

```
┌─────────────────────────────────────────┐
│  ctas7-foundation-daemon (ATLAS)       │
│  ┌───────────────────────────────────┐  │
│  │ CDNCoordinator                    │  │
│  │  - Manages all CDN services       │  │
│  │  - Health monitoring              │  │
│  │  - Lifecycle management          │  │
│  │  - OODA loop integration         │  │
│  └───────────────────────────────────┘  │
└───────────────┬─────────────────────────┘
                │
                ├─→ ctas7-cdn-data-fabric (Axum server)
                ├─→ ctas7-cdn-threat-intel (Axum server)
                ├─→ ctas7-cdn-geospatial (Axum server)
                └─→ ... (other CDN services)
```

**Pros:**
- ✅ Single point of control
- ✅ Unified health monitoring
- ✅ Centralized OODA loop
- ✅ Easier to manage

**Cons:**
- ❌ Single point of failure
- ❌ Less autonomy per service
- ❌ Harder to scale independently

---

### Option B: Embedded Daemon Per Service (Recommended)
**Pattern:** Each CDN service has its own daemon component

```
┌─────────────────────────────────────────┐
│  ctas7-cdn-data-fabric                  │
│  ┌───────────────────────────────────┐  │
│  │ Axum Server (HTTP/WebSocket)       │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │ Embedded Daemon Component         │  │
│  │  - Health monitoring              │  │
│  │  - Background tasks               │  │
│  │  - OODA loop (service-specific)   │  │
│  │  - Lifecycle management            │  │
│  └───────────────────────────────────┘  │
└───────────────┬─────────────────────────┘
                │
                └─→ Reports to ctas7-foundation-daemon
```

**Pros:**
- ✅ Service autonomy
- ✅ Independent scaling
- ✅ Service-specific OODA loops
- ✅ Better fault isolation
- ✅ Can run standalone

**Cons:**
- ❌ More complexity
- ❌ Need coordination layer

---

## Recommended Architecture: Hybrid

### Each CDN Service Has Embedded Daemon Component

**Structure:**
```
ctas7-cdn-data-fabric/
├── src/
│   ├── main.rs              # Axum server entry point
│   ├── server.rs            # Axum HTTP/WebSocket server
│   ├── daemon.rs            # Embedded daemon component
│   ├── health.rs            # Health monitoring
│   ├── ooda.rs              # Service-specific OODA loop
│   └── lifecycle.rs         # Lifecycle management
└── Cargo.toml
```

### Embedded Daemon Component

```rust
// src/daemon.rs

use sx9_atlas_bus::AtlasBus;
use tokio::time::{interval, Duration};

pub struct CDNDaemon {
    bus: AtlasBus,
    health_monitor: HealthMonitor,
    ooda: OodaLoop,
    lifecycle: LifecycleManager,
    tick_interval: Duration,
}

impl CDNDaemon {
    pub fn new(config: CDNConfig) -> Self {
        Self {
            bus: AtlasBus::new(),
            health_monitor: HealthMonitor::new(),
            ooda: OodaLoop::new(),
            lifecycle: LifecycleManager::new(),
            tick_interval: Duration::from_millis(100), // 100ms tick
        }
    }
    
    pub async fn run(&mut self) -> Result<()> {
        let mut ticker = interval(self.tick_interval);
        
        loop {
            ticker.tick().await;
            
            // OODA loop (service-specific)
            self.ooda.observe().await?;
            self.ooda.orient().await?;
            self.ooda.decide().await?;
            self.ooda.act().await?;
            
            // Health monitoring
            self.health_monitor.check().await?;
            
            // Lifecycle management
            self.lifecycle.tick().await?;
        }
    }
}
```

### Integration with Central Daemon

```rust
// Reports to central daemon via sx9-atlas-bus
impl CDNDaemon {
    pub async fn report_to_atlas(&self) -> Result<()> {
        let status = DaemonStatus {
            service_id: "cdn-data-fabric",
            health: self.health_monitor.get_status(),
            metrics: self.health_monitor.get_metrics(),
            ooda_phase: self.ooda.current_phase(),
        };
        
        self.bus.send_command(DaemonCommand::ReportStatus(status)).await?;
        Ok(())
    }
}
```

---

## Benefits of Embedded Daemon

### 1. Service Autonomy
- Each CDN service can run standalone
- Independent health monitoring
- Service-specific OODA loops

### 2. Better Fault Isolation
- If one CDN service fails, others continue
- No single point of failure
- Independent scaling

### 3. Service-Specific OODA Loops
- Data Fabric: Optimize for database queries
- Threat Intel: Optimize for threat correlation
- Geospatial: Optimize for GIS operations

### 4. Lifecycle Management
- Graceful shutdown
- Health checks
- Auto-restart on failure
- Resource limits

---

## Implementation Plan

### Phase 1: Add Daemon Component to Each CDN

**For each CDN service:**

1. **Create `daemon.rs` module**
   ```rust
   pub struct CDNDaemon {
       // Service-specific daemon
   }
   ```

2. **Add to `main.rs`**
   ```rust
   #[tokio::main]
   async fn main() -> Result<()> {
       // Start Axum server
       let server = tokio::spawn(start_axum_server());
       
       // Start embedded daemon
       let daemon = tokio::spawn(start_cdn_daemon());
       
       // Wait for both
       tokio::select! {
           _ = server => {},
           _ = daemon => {},
       }
   }
   ```

3. **Integrate with sx9-atlas-bus**
   - Report status to central daemon
   - Receive commands from central daemon
   - Participate in OODA loops

### Phase 2: Gateway Daemon

**`sx9-gateway-primary` should also have embedded daemon:**

```rust
// sx9-gateway-primary/src/daemon.rs

pub struct GatewayDaemon {
    bus: AtlasBus,
    health: HealthMonitor,
    ooda: OodaLoop,
    routing: RoutingOptimizer,
}
```

---

## Unified Daemon Interface

### Trait for All Service Daemons

```rust
pub trait ServiceDaemon {
    /// Service identifier
    fn service_id(&self) -> &str;
    
    /// Health status
    async fn health(&self) -> HealthStatus;
    
    /// Current OODA phase
    fn ooda_phase(&self) -> OodaPhase;
    
    /// Metrics
    async fn metrics(&self) -> ServiceMetrics;
    
    /// Report to central daemon
    async fn report(&self) -> Result<()>;
    
    /// Receive command from central daemon
    async fn handle_command(&mut self, cmd: DaemonCommand) -> Result<()>;
}
```

### Implementation for CDN Services

```rust
impl ServiceDaemon for CDNDaemon {
    fn service_id(&self) -> &str {
        "cdn-data-fabric"
    }
    
    async fn health(&self) -> HealthStatus {
        self.health_monitor.get_status().await
    }
    
    fn ooda_phase(&self) -> OodaPhase {
        self.ooda.current_phase()
    }
    
    async fn metrics(&self) -> ServiceMetrics {
        self.health_monitor.get_metrics().await
    }
    
    async fn report(&self) -> Result<()> {
        self.report_to_atlas().await
    }
    
    async fn handle_command(&mut self, cmd: DaemonCommand) -> Result<()> {
        match cmd {
            DaemonCommand::Restart => self.lifecycle.restart().await?,
            DaemonCommand::Shutdown => self.lifecycle.shutdown().await?,
            DaemonCommand::HealthCheck => self.health_monitor.check().await?,
            _ => {}
        }
        Ok(())
    }
}
```

---

## Summary

**Answer: Yes, each CDN service should have an embedded daemon component**

**Architecture:**
- **Embedded Daemon** - Each CDN service has its own daemon component
- **Central Daemon** - `ctas7-foundation-daemon` coordinates all services
- **Communication** - Via `sx9-atlas-bus` for status reporting and commands

**Benefits:**
- ✅ Service autonomy
- ✅ Better fault isolation
- ✅ Service-specific OODA loops
- ✅ Independent scaling
- ✅ Can run standalone

**Implementation:**
1. Add `daemon.rs` module to each CDN service
2. Implement `ServiceDaemon` trait
3. Integrate with `sx9-atlas-bus`
4. Report to central daemon

---

**Status:** Recommended architecture defined. Ready for implementation.



