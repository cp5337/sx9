# Threat Content Fetcher Refactoring Plan

**Current State:**
- `threat_content_fetcher.py`: 1,901 lines
- Single class `ThreatContentFetcher` with ~25 methods
- All functionality in one file

**Proposed Structure:**

```
node-interview-generator/
├── threat_content_fetcher.py          # Main orchestrator (thin wrapper)
├── fetchers/
│   ├── __init__.py
│   ├── base_fetcher.py                # Base class with common utilities
│   ├── mitre_fetcher.py               # MITRE ATT&CK, D3FEND, CAR, ATLAS
│   ├── detection_fetcher.py           # Sigma, YARA, Wazuh, Nuclei
│   ├── emulation_fetcher.py           # Atomic Red Team, Caldera
│   ├── osint_fetcher.py               # OSINT tools (Sherlock, Maigret, etc.)
│   ├── kali_fetcher.py                # Kali tools integration
│   └── lolbas_fetcher.py              # LOLBAS, GTFOBins, LOLDrivers, etc.
├── processors/
│   ├── __init__.py
│   ├── spires_processor.py            # SPIRES ontology generation
│   ├── dsl_processor.py               # YAML to DSL conversion
│   └── ml_training_processor.py        # ML model training
└── utils/
    ├── __init__.py
    ├── git_utils.py                   # Git clone/update utilities
    ├── http_utils.py                 # HTTP download utilities
    └── index_utils.py                 # Index generation utilities
```

**Benefits:**
1. **Modularity**: Each fetcher handles one domain (MITRE, detection, etc.)
2. **Testability**: Easier to unit test individual fetchers
3. **Maintainability**: Changes to one source don't affect others
4. **Extensibility**: Easy to add new fetchers without touching existing code
5. **Readability**: Smaller files are easier to understand

**Migration Strategy:**
1. Create base `BaseFetcher` class with common methods
2. Extract each fetcher group into separate module
3. Update main `ThreatContentFetcher` to orchestrate fetchers
4. Maintain backward compatibility (same API)

**Estimated Effort:** 2-3 hours

**Should I proceed with the refactoring?**



