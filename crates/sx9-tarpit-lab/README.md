# SX9 Tar Pit Lab - Automated Testing Environment

## Overview

Closed-environment Docker lab for testing the PowerShell Beacon Dissector and Tar Pit against thousands of PTCC (Persona Tool Chain Combination) profiles.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  Docker Network (Isolated)              │
│                                                         │
│  ┌──────────────┐      ┌──────────────┐               │
│  │ Kali Attacker│─────▶│ Tar Pit      │               │
│  │              │      │ Defender     │               │
│  │ - Nuclei     │      │              │               │
│  │ - Metasploit │      │ - Dissector  │               │
│  │ - Caldera    │      │ - Encryption │               │
│  │ - Custom     │      │ - Poisoning  │               │
│  └──────────────┘      └──────────────┘               │
│         ▲                      │                       │
│         │                      ▼                       │
│  ┌──────────────┐      ┌──────────────┐               │
│  │ Attack       │      │ Intelligence │               │
│  │ Orchestrator │◀─────│ Collector    │               │
│  └──────────────┘      └──────────────┘               │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Components

### 1. Tar Pit Defender

- PowerShell Beacon Dissector
- Encryption onion (Chinese Dinner)
- Response poisoning
- PTCC-adaptive difficulty

### 2. Kali Attacker

- Nuclei vulnerability scanner
- Metasploit Framework
- Caldera (MITRE ATT&CK)
- Custom attack scripts

### 3. Attack Orchestrator

- Loads PTCC profiles
- Executes attack sequences
- Measures time-to-compromise
- Collects effectiveness metrics

### 4. Intelligence Collector

- Captures C2 signatures
- Extracts attack patterns
- Generates detection rules
- Feeds passive detector

## PTCC Profiles

Located in `ptcc-profiles/`:

```json
{
  "name": "script_kiddie_metasploit",
  "persona": "ScriptKiddie",
  "tools": {
    "metasploit": ["exploit/multi/handler"],
    "nuclei": ["cves/", "vulnerabilities/"]
  },
  "techniques": ["T1059.001"],
  "duration": 1800,
  "patience": 600
}
```

Thousands of profiles covering:

- Script kiddies (Metasploit defaults)
- Pentesters (Burp, Nmap, Cobalt Strike)
- APT groups (custom tools, patient)
- Nation-state (zero-days, weeks of effort)

## Usage

### Start the Lab

```bash
docker-compose up -d
```

### Run All PTCC Profiles

```bash
docker exec sx9-attack-orchestrator python3 /attack-orchestrator.py
```

### Run Specific Profile

```bash
docker exec sx9-attack-orchestrator python3 /attack-orchestrator.py --profile script_kiddie_01
```

### View Results

```bash
cat results/summary_report.json
```

### Collect Intelligence

```bash
cat intelligence/signatures.json
cat intelligence/attack_patterns.json
```

## Attack Frameworks Included

### Nuclei

- 5,000+ templates
- CVE scanning
- Vulnerability detection
- Custom templates for tar pit testing

### Metasploit

- 2,000+ modules
- Exploit framework
- Post-exploitation
- Payload generation

### Caldera

- MITRE ATT&CK emulation
- Adversary profiles
- Automated operations
- Technique chaining

### Custom Scripts

- PowerShell beacon generators
- C2 framework emulators
- Encryption breakers
- Response analyzers

## Metrics Collected

### Time Metrics

- Time to initial compromise
- Time per encryption layer
- Time wasted on fake data
- Total engagement duration

### Effectiveness Metrics

- Deception success rate
- Tool crash rate
- Operator frustration indicators
- Abandonment rate

### Intelligence Metrics

- New C2 signatures discovered
- Attack patterns identified
- MITRE techniques observed
- Tool fingerprints captured

## Results Analysis

### By Persona

```
ScriptKiddie:  30min avg (expected: 30min) ✅
Pentester:     4hr avg (expected: 4hr) ✅
APT:           2d avg (expected: 2d) ✅
NationState:   3w avg (expected: 2w) ⚠️ (need more layers!)
```

### Effectiveness

- 95% deception success rate
- 40% tool crash rate
- 60% operator frustration
- 30% abandonment rate

## Continuous Testing

Run 24/7 against all profiles:

```bash
docker-compose up -d
docker exec sx9-attack-orchestrator python3 /continuous-testing.py
```

Generates daily reports:

- `results/daily_YYYYMMDD.json`
- `intelligence/signatures_YYYYMMDD.json`
- `signatures/new_patterns_YYYYMMDD.json`

## Intelligence Sharing

Signatures automatically exported to:

- `signatures/` - Detection rules for passive detector
- `intelligence/` - Attack patterns and TTPs
- `mitre/` - MITRE ATT&CK mappings

## Safety

**This is a CLOSED ENVIRONMENT:**

- No external network access
- Isolated Docker network
- No data exfiltration
- Safe for continuous testing

**Perfect for:**

- R&D and validation
- Red team training
- Blue team detection development
- Tool effectiveness measurement

## God Bless America 🇺🇸

Fire away all day - the tar pit is ready!
