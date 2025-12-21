# SX9 Threat Intelligence Suite

## Components

1. **TETH** - Tool Entropy Testing Harness (`tools/teth/`)
2. **PowerShell Beacon Dissector** - C2 beacon analysis (`crates/sx9-plasma-defender/`)
3. **Kali Plasma** - Tool execution framework (`tools/kali-plasma/`)

## Data Flow

```
Tool Execution (Kali Plasma)
  ↓
Output Capture & Hashing (RFC-9001)
  ↓
Entropy Analysis (TETH)
  ↓
Beacon Detection (Plasma Defender)
  ↓
APT Attribution (TETH + Beacon Dissector)
  ↓
Campaign Tracking (OODA/HD4)
```

## Integration Points

- **NATS Subjects:**
  - `sx9.tool.result.ann` - Tool results to Plasma Defender
  - `sx9.plasma.ann.advisory` - ANN recommendations
  - `sx9.tool.response.{CODE}` - Hashed outputs

- **Atlas Bus:** Plasma state management, polycrystal resonance

- **Dependencies:**
  - RFC-9001: Trivariate Hashing
  - RFC-9121: Lightning QA
  - RFC-9125: TETH Specification

## Quick Start

### TETH
```bash
cd tools/teth
python fire_chain.py --help
```

### Kali Plasma
```bash
cd tools/kali-plasma/tool-exerciser
./exerciser.sh -t 0 run  # Tier 0: help only
```

### Plasma Defender
```bash
cargo build -p sx9-plasma-defender
```
