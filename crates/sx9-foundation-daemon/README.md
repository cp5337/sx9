# CTAS-7 ATLAS Daemon

> **ATLAS**: Autonomous Task & Lifecycle Automation System  
> **Version**: 7.3.1  
> **Status**: Production Ready  
> **Tagline**: "The Foundation That Holds Everything Up"

---

## 🏛️ **What is ATLAS?**

**ATLAS** is the CTAS-7 enterprise-grade process orchestration and service management daemon that replaces PM2 with:

- ✅ **HFT-Optimized Performance** (<250ns routing latency)
- ✅ **OrbStack Native Integration** (2-second container startup)
- ✅ **Service Discovery** (mDNS + custom registry)
- ✅ **Container Orchestration** (Docker + Kubernetes ready)
- ✅ **Real-time Monitoring** (health checks, metrics, logs)
- ✅ **Multi-Modal Support** (Rust binaries, Node.js, Python, containers)
- ✅ **Foundation Integration** (Trivariate hash, Neural Mux, Port Manager)

---

## 🚀 **Quick Start**

### **Installation**

```bash
# Build ATLAS
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-foundation-daemon
cargo build --release

# Binary location
target/release/atlas

# Add to PATH (optional)
sudo ln -s $(pwd)/target/release/atlas /usr/local/bin/atlas
```

### **Basic Commands**

```bash
# Start ATLAS daemon
atlas start

# Check status
atlas status

# List services
atlas ps

# View logs
atlas logs <service-name>

# Restart service
atlas restart <service-name>

# Stop ATLAS
atlas stop
```

### **Deploy with OrbStack**

```bash
# Start complete ATLAS stack
docker-compose -f docker-compose.orbstack-complete.yml up -d

# Check ATLAS container
docker ps --filter "name=ctas7-atlas"

# View ATLAS logs
docker logs -f ctas7-atlas

# Access ATLAS dashboard
open http://localhost:18500
```

---

## 📊 **ATLAS Architecture**

### **Core Components**

```
ATLAS Daemon (Port 18500)
├── Service Registry (18650)      # Central service discovery
├── Port Manager (18103)          # Dynamic port allocation
├── Hash Engine (18105)           # MurmurHash3 validation
├── Container Orchestrator        # Docker/OrbStack integration
├── Process Manager               # PM2-compatible process control
├── Health Monitor                # Real-time health checks
└── Metrics Collector             # Performance monitoring
```

### **Service Tiers**

ATLAS manages services across 6 tiers (PM2 ecosystem compatibility):

```
Tier 0: Document Intelligence (ABE)
├── abe-firefly (18191)
└── abe-drive-sync (18192)

Tier 1: Core Agent Infrastructure
├── repoagent-gateway (15180)
└── agent-mesh (50055)

Tier 2: Linear Integration
├── linear-integration (15182)
└── linear-agent (18180)

Tier 3: Intelligence Services
├── intelligence-coordination (18200-18207)
└── glaf-intelligence (8090-8092)

Tier 4: Memory Mesh v2.0
└── memory-mesh (19014)

Tier 5: Custom GPT Endpoints
└── gpt-endpoint (TBD)

Tier 6: Tool Orchestration
└── tool-orchestrator (TBD)
```

---

## 🐳 **OrbStack Integration**

ATLAS is optimized for OrbStack with:

- **Native Performance**: Near-native macOS performance
- **Fast Startup**: 2-second container initialization
- **Low Memory**: Efficient resource usage
- **Auto-Discovery**: Automatic service registration
- **Health Checks**: Built-in container health monitoring

### **OrbStack Services Managed by ATLAS**

```yaml
Foundation Services (3):
✅ service-discovery (18650)
✅ atlas-daemon (18500)
✅ backend-mcp (18600)

Intelligence Services (3):
✅ glaf-intelligence (8090-8092)
✅ abe-controlled-access (18630)
✅ phi3-guardian (11434)

Validation Services (3):
✅ database-validator (18605)
✅ performance-tester (18620)
✅ emergency-recovery (18615)

Monitoring (2):
✅ watchdog-dashboard (18610)
✅ health-dashboard (18888)
```

---

## 🎯 **Key Features**

### **1. PM2 Compatibility**

ATLAS can read and execute PM2 ecosystem files:

```bash
# Run PM2 ecosystem config
atlas start ecosystem.config.cjs

# PM2-compatible commands
atlas list
atlas restart all
atlas logs
atlas monit
```

### **2. Service Discovery**

Automatic service registration and discovery:

```bash
# Discover all services
curl http://localhost:18650/discover

# Register new service
curl -X POST http://localhost:18650/register \
  -H "Content-Type: application/json" \
  -d '{"name":"my-service","port":8080,"health":"/health"}'
```

### **3. Container Orchestration**

Native Docker/OrbStack integration:

```bash
# List containers
atlas containers

# Start container
atlas container start <name>

# View container logs
atlas container logs <name>
```

### **4. Health Monitoring**

Real-time health checks for all services:

```bash
# Check all service health
atlas health

# Check specific service
atlas health <service-name>

# Health dashboard
open http://localhost:18500/health
```

### **5. Performance Metrics**

HFT-optimized performance tracking:

```bash
# View metrics
atlas metrics

# Performance dashboard
open http://localhost:18500/metrics
```

---

## 🔧 **Configuration**

### **ATLAS Config File** (`atlas.toml`)

```toml
[atlas]
version = "7.3.1"
mode = "multi_modal"
port = 18500

[service_discovery]
enabled = true
port = 18650
mdns = true

[port_manager]
enabled = true
port = 18103
range = [15000, 19999]

[hash_engine]
enabled = true
port = 18105
algorithm = "murmurhash3"
seed = 1337

[hft]
enabled = true
routing_latency_target = 250  # nanoseconds
hash_performance_target = 15240  # MB/sec

[orbstack]
enabled = true
native = true
auto_discovery = true

[monitoring]
health_checks = true
metrics = true
dashboard_port = 18500
```

---

## 📡 **API Endpoints**

### **ATLAS Daemon API** (Port 18500)

```bash
# Health check
GET http://localhost:18500/health

# Service status
GET http://localhost:18500/api/services

# Start service
POST http://localhost:18500/api/services/:name/start

# Stop service
POST http://localhost:18500/api/services/:name/stop

# Restart service
POST http://localhost:18500/api/services/:name/restart

# View logs
GET http://localhost:18500/api/services/:name/logs

# Metrics
GET http://localhost:18500/api/metrics
```

---

## 🏗️ **Development**

### **Build from Source**

```bash
# Clone repository
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-foundation-daemon

# Build ATLAS
cargo build --release

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug ./target/release/atlas start
```

### **Features**

```bash
# Build with HFT optimization (default)
cargo build --release --features hft

# Build with PM2 compatibility
cargo build --release --features pm2-compatibility

# Build all features
cargo build --release --all-features
```

---

## 📊 **Monitoring & Dashboards**

### **ATLAS Dashboard** (Port 18500)

```
http://localhost:18500/

Features:
- Service status overview
- Real-time metrics
- Container management
- Log viewer
- Health checks
```

### **Watchdog Dashboard** (Port 18610)

```
http://localhost:18610/

Grafana-based monitoring:
- Performance graphs
- Resource usage
- Alert management
```

### **Health Dashboard** (Port 18888)

```
http://localhost:18888/

CTAS 7.3.1 system health:
- Backend services
- Container infrastructure
- Node interviews
- ABE QA system
```

---

## 🚨 **Troubleshooting**

### **ATLAS won't start**

```bash
# Check if port 18500 is in use
lsof -i :18500

# Check ATLAS logs
tail -f logs/atlas.log

# Restart ATLAS
atlas stop && atlas start
```

### **Service not discovered**

```bash
# Check service discovery
curl http://localhost:18650/discover

# Manually register service
atlas register <service-name> --port <port>
```

### **Container issues**

```bash
# Check OrbStack status
docker context use orbstack
docker ps

# Restart container
atlas container restart <name>
```

---

## 📁 **File Locations**

```
ctas7-foundation-daemon/
├── src/
│   ├── main.rs                    # ATLAS entry point
│   ├── service_manager.rs         # Service orchestration
│   ├── container_orchestrator.rs  # Docker/OrbStack integration
│   ├── service_discovery.rs       # mDNS discovery
│   ├── port_manager.rs            # Port allocation
│   └── health_monitor.rs          # Health checks
├── Cargo.toml                     # Package config (renamed to ctas7-atlas)
├── atlas.toml                     # ATLAS configuration
├── docker-compose.orbstack-complete.yml  # OrbStack deployment
└── README.md                      # This file
```

---

## 🎯 **Roadmap**

- [x] PM2 replacement functionality
- [x] OrbStack integration
- [x] Service discovery
- [x] Container orchestration
- [x] Health monitoring
- [ ] Kubernetes support
- [ ] Auto-scaling
- [ ] Load balancing
- [ ] Multi-node clustering
- [ ] Web UI dashboard

---

## 📚 **Resources**

- **CTAS 7.3.1 Specification**: `CTAS_7.3.1_CONSOLIDATED_ARCHITECTURE.md`
- **Dashboard Integration**: `DASHBOARD_FULL_BACKEND_VISIBILITY.md`
- **OrbStack Config**: `docker-compose.orbstack-complete.yml`
- **PM2 Ecosystem**: `ecosystem.config.cjs`

---

## 🏛️ **Why ATLAS?**

> In Greek mythology, Atlas holds up the celestial heavens. In CTAS-7, **ATLAS holds up the entire infrastructure** - managing services, containers, and orchestration with unwavering reliability.

**ATLAS**: The foundation that holds everything up. ⚡

---

**Version**: 7.3.1  
**Status**: Production Ready  
**License**: MIT  
**Maintained by**: CTAS-7 Engineering Team












