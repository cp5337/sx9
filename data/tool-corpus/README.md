# SX9 Tool Corpus

Canonical storage for tool output harvesting and the hash pipeline.

## Directory Structure

```
tool-corpus/
├── raw/                    # Raw tool outputs (compressed)
│   ├── nmap/
│   │   ├── NMP7X2A.zst     # Compressed raw output
│   │   └── ...
│   ├── nuclei/
│   └── ...
│
├── parsed/                 # Parsed/structured outputs
│   ├── nmap/
│   │   ├── NMP7X2A.json    # Parsed hosts, ports, services
│   │   └── ...
│   └── ...
│
├── hashed/                 # Hash metadata (short code → full data)
│   ├── NMP7X2A.json        # Hash metadata
│   ├── NUC3K9B.json
│   └── ...
│
├── index/                  # Indexes for fast lookup
│   ├── by_tool.json        # tool → [short_codes]
│   ├── by_hash.json        # content_hash → short_code
│   ├── by_date.json        # date → [short_codes]
│   └── lineage.json        # heredity tree
│
├── corpus.db               # Sled database (optional)
└── manifest.json           # Corpus manifest
```

## Short Code Format

```
{TOOL_PREFIX}{HASH_SUFFIX}

Examples:
  NMP7X2A  → nmap output, hash prefix 7X2A
  NUC3K9B  → nuclei output, hash prefix 3K9B
  MSC4P1C  → masscan output, hash prefix 4P1C
```

## Tool Prefixes

| Prefix | Tool | Category |
|--------|------|----------|
| NMP | nmap | Network Recon |
| MSC | masscan | Network Recon |
| NKT | nikto | Web Testing |
| NUC | nuclei | Vuln Scanning |
| SQL | sqlmap | Web Testing |
| JHN | john | Password |
| HSH | hashcat | Password |
| GOB | gobuster | Web Testing |
| AMS | amass | OSINT |
| SBF | subfinder | OSINT |
| HRV | theHarvester | OSINT |
| WWB | whatweb | Web Testing |

## Hash Metadata Schema

```json
{
  "short_code": "NMP7X2A",
  "tool": "nmap",
  "content_hash": "7f3a9b2c4d5e6f...",
  "output_hash_ref": {
    "semantic": "0x...",
    "operational": "0x..."
  },
  "format_rune": "0xE901",
  "size_rune": "0xE912",
  "parser_hint": "0xE920",
  "size_bytes": 45230,
  "duration_ms": 1234,
  "timestamp": "2025-12-16T...",
  "heredity": {
    "operator": "0xE801",
    "parent": "REQ7K3",
    "expr": "(cons REQ7K3 NMP7X2A)"
  },
  "paths": {
    "raw": "raw/nmap/NMP7X2A.zst",
    "parsed": "parsed/nmap/NMP7X2A.json"
  }
}
```

## Integration Points

1. **Exerciser** → Writes to tool-corpus/
2. **NATS** → Streams sx9.tool.response.{short_code}
3. **JetStream** → SX9_TOOL_CORPUS stream for persistence
4. **Plasma Defender** → Reads from corpus for threat analysis
5. **ANN** → Trains on corpus data

## Lifecycle

```
Tool Execution
     │
     ▼
[Exerciser] ─────────────────────────────────────────┐
     │                                               │
     ▼                                               ▼
[raw/] ──────► [parsed/] ──────► [hashed/] ──────► [index/]
     │              │                │                │
     │              │                │                │
     └──────────────┴────────────────┴────────────────┘
                         │
                         ▼
                    [NATS Stream]
                         │
                         ▼
                 [Plasma Defender]
                         │
                         ▼
                    [ANN/Crystal]
```

## Usage

```bash
# From exerciser
./run.sh 0  # Outputs to tool-exerciser/output/

# Sync to corpus
rsync -av tool-exerciser/output/ /path/to/data/tool-corpus/

# Or configure exerciser to write directly
export CORPUS_DIR=/path/to/data/tool-corpus
```

