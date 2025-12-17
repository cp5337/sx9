# Dual Trivariate Hash Example

**Short Code:** `NMP7263D`  
**Tool:** nmap  
**Location:** `data/tool-corpus/hashed/`

---

## h1 - Semantic Hash (TOML) - Design Time

**File:** `NMP7263D.h1.toml`

```toml
# h1 - Semantic Hash (SCH) - TOML
# What the output MEANS - design time / configuration
# Seed: 0xC7A50000

[semantic]
short_code = "NMP7263D"
hash = "7263d2a1d2c29b5b"
seed = "0xC7A50000"

[tool]
name = "nmap"
trigger_rune = "0xE000"
parser = "nmap_xml"
parser_rune = "0xE920"

[content]
format_rune = "0xE901"
size_rune = "0xE912"
size_bytes = 45230
```

---

## h2 - Operational Hash (JSON) - Runtime

**File:** `NMP7263D.h2.json`

```json
{
  "operational": {
    "short_code": "NMP7263D",
    "hash": "972311fa391c5cc2",
    "seed": "0xC7A50001"
  },
  "request": {
    "id": "REQ1734372000",
    "timestamp": "2025-12-16T12:00:00-05:00",
    "sequence": 1
  },
  "execution": {
    "duration_ms": 1234,
    "exit_code": 0,
    "env": "kali-plasma"
  }
}
```

---

## Summary

| Hash | Format | Purpose | Seed | Content |
|------|--------|---------|------|---------|
| **h1** (SCH) | TOML | Semantic / design-time | `0xC7A50000` (even) | What it *means* |
| **h2** (Heredity) | JSON | Operational / runtime | `0xC7A50001` (odd) | What *happened* |

### Key Principles

1. **Same short code** (`NMP7263D`) links both files
2. **Different hashes** derived from different seeds
3. **Even seed** = semantic (h1), **Odd seed** = operational (h2)
4. **TOML** for human-readable configuration/meaning
5. **JSON** for machine-processable runtime data

### Unicode Runes Used

| Rune | Meaning |
|------|---------|
| `0xE000` | Tool trigger (nmap) |
| `0xE901` | Format indicator |
| `0xE912` | Size indicator |
| `0xE920` | Parser (nmap_xml) |

---

## Usage

```bash
# Find both files for a short code
ls data/tool-corpus/hashed/NMP7263D.*

# Output:
# NMP7263D.h1.toml  (semantic)
# NMP7263D.h2.json  (operational)
```

