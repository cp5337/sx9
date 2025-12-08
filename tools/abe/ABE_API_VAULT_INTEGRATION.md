# ABE + sx9-api-vault Integration Architecture

**Version:** 1.0.0  
**Date:** 2025-12-01  
**Classification:** CTAS-7 Federation Pattern  
**Status:** Design & Implementation Guide

---

## Executive Summary

This document defines how **sx9-api-vault** (local/development API key management) federates with **ABE's GCP Secret Manager** (production credentials) to create a unified, multi-tier credential system supporting:

- **Tool Triage Workflow**: Integrate and test hundreds of Kali tools with credential management
- **Dual-Layer Access**: Python services via sx9-conda + Rust services via ctas7-foundation-python-client
- **Federation Strategy**: Local vault → GCP Secret Manager → Runtime services
- **Zero-Trust Model**: Per-service credential scoping with escalation-aware access

---

## Architecture Overview

### Federated Credential Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    ABE Credential Tiers                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Tier 1: Local Development                                      │
│  ┌──────────────────────────────┐                               │
│  │   sx9-api-vault              │  (~/.sx9-api-vault/)          │
│  │ - Tool registry              │  - keys.json (unencrypted)    │
│  │ - Discovery patterns         │  - tools.json                 │
│  │ - Test credentials           │  - registry.json              │
│  └──────────────────────────────┘                               │
│           │                                                      │
│           ├─→ Python Direct (sx9-conda)                         │
│           └─→ Rust Bridge (ctas7-foundation-python-client)     │
│                                                                  │
├─────────────────────────────────────────────────────────────────┤
│  Tier 2: GCP Secret Manager (Production)                        │
│  ┌──────────────────────────────┐                               │
│  │   Google Secret Manager      │  cognetix-abe-*-secrets       │
│  │ - External API keys          │  Encrypted @ rest             │
│  │ - OAuth credentials          │  Audit logging enabled        │
│  │ - Service account tokens     │  Per-service ACLs             │
│  └──────────────────────────────┘                               │
│           │                                                      │
│           ├─→ External API Gateway (Cloud Run)                  │
│           ├─→ Ingestion Service (Cloud Run)                     │
│           ├─→ Summarization Service (Cloud Run)                 │
│           └─→ CTAS Operations (if enabled)                      │
│                                                                  │
├─────────────────────────────────────────────────────────────────┤
│  Tier 3: Runtime Services                                       │
│  ┌──────────────────────────────┐                               │
│  │   ABE Services               │                               │
│  │ - Credential injection       │  Environment variables        │
│  │ - Cache layer                │  Per-request injection        │
│  │ - Escalation-aware scoping   │  Multi-tenant isolation       │
│  └──────────────────────────────┘                               │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Part 1: sx9-api-vault as Tool Triage System

### 1.1 Extended Registry for Hundreds of Tools

sx9-api-vault's `registry.json` should be expanded to categorize tools by:

```json
{
  "registry": {
    "categories": {
      "osint": {
        "domain_recon": [
          "theHarvester", "shodan", "censys", "fierce",
          "nslookup", "dig", "whois", "nmap"
        ],
        "social_media": [
          "twint", "instagrapi", "tiktok-scraper",
          "facebook-scraper", "youtube-dl"
        ],
        "email_recon": ["hunter", "rocketreach", "emailfinder"]
      },
      "exploitation": {
        "vulnerability_scanning": [
          "nessus", "qualys", "openvas", "nuclei"
        ],
        "exploit_frameworks": [
          "metasploit", "exploitdb", "nuclei-templates"
        ]
      },
      "password_testing": {
        "credential_testing": [
          "hashcat", "john", "hydra", "medusa"
        ],
        "breach_databases": [
          "haveibeenpwned", "breach-parse"
        ]
      },
      "intelligence": {
        "threat_intel": [
          "virustotal", "abuseipdb", "alienvault-otx"
        ],
        "geolocation": [
          "maxmind", "geoip2", "ip2location"
        ]
      },
      "data_processing": {
        "etl": ["airflow", "luigi", "dbt"],
        "data_validation": ["great-expectations"]
      }
    },
    "tool_profile_template": {
      "tool_name": "string",
      "category": "string",
      "capability": "string",
      "required_apis": ["array of api names"],
      "free_tier_available": "boolean",
      "rate_limits": {
        "api_name": "requests/period"
      },
      "authentication_type": "enum[api_key, oauth2, basic, token]",
      "data_classification": "enum[osint, sensitive, operational]",
      "kali_linux_package": "string or null",
      "docker_image": "string or null",
      "integration_status": "enum[untested, tested, production, deprecated]",
      "ctas_approved": "boolean"
    }
  }
}
```

### 1.2 Tool Triage Workflow

```bash
# 1. DISCOVER: Scan Kali Linux tools and extract credential requirements
python3 sx9-api-vault.py crawl --kali-tools --output tool-requirements.json

# 2. CATALOG: Register each tool with credential mapping
python3 sx9-api-vault.py register SHODAN_KEY shodan <key> \
  --tool-category osint --tool-name theHarvester \
  --rate-limit "100/month" \
  --data-classification osint

# 3. VALIDATE: Test credentials against live APIs
python3 sx9-api-vault.py validate --all --fail-fast

# 4. REPORT: Generate coverage matrix
python3 sx9-api-vault.py report --format=matrix --output coverage.html

# 5. EXPORT: Prepare for ABE deployment
python3 sx9-api-vault.py export --target=gcp-secret-manager \
  --project=gen-lang-client-0290627006 \
  --prefix=cognetix-abe-tool-api-
```

---

## Part 2: Dual-Layer Access Patterns

### 2.1 Python Access Pattern (Direct via sx9-conda)

**Scenario**: OSINT service needs API key during execution

```python
# File: sx9-conda/services/osint_service.py
import sys
sys.path.insert(0, '/path/to/sx9-api-vault')
from sx9_api_vault import VaultClient, CredentialScope

class OSINTService:
    def __init__(self):
        # Access local vault first, fallback to GCP
        self.vault = VaultClient(
            tier1_path="~/.sx9-api-vault",
            tier2_gcp_project="gen-lang-client-0290627006",
            tier2_prefix="cognetix-abe-tool-api-"
        )
    
    async def enumerate_domain(self, domain: str, escalation_level: str):
        """Example: Domain enumeration with theHarvester"""
        
        # Request credential with escalation context
        credential = await self.vault.get_credential(
            service="shodan",
            scope=CredentialScope(
                operation="domain_recon",
                escalation_level=escalation_level,  # Tactical/Operational/Strategic
                requesting_service="osint",
                audit_context={
                    "trivariate_hash": "SCH|CUID|UUID",
                    "ctas_operation_id": "op-123"
                }
            )
        )
        
        # Use credential in tool execution
        from theHarvester.lib import stash
        results = stash.theHarvester(
            domain=domain,
            shodan_api_key=credential.key,
            rate_limit=credential.rate_limit
        )
        return results
```

**Advantages**:
- Direct access to local vault during development
- No network roundtrip for local keys
- Immediate fallback to GCP for production
- Full audit trail through credential scope

### 2.2 Rust Access Pattern (via ctas7-foundation-python-client)

**Scenario**: CTAS Rust operation needs API key from ABE

```rust
// File: ctas7-foundation-daemon/src/services/abe_api_bridge.rs
use ctas7_foundation_python_client::{TcpPythonClient, PythonRequest, EscalationLevel};
use serde_json::json;

pub struct ABEAPIBridge {
    vault_client: TcpPythonClient,
}

impl ABEAPIBridge {
    pub async fn get_api_credential(
        &self,
        service_name: &str,
        escalation_level: EscalationLevel,
    ) -> Result<APICredential, Box<dyn std::error::Error>> {
        // Create federated credential request
        let request = PythonRequest::new(
            "get_credential",
            json!({
                "service": service_name,
                "scope": {
                    "operation": "ctas_operation",
                    "escalation_level": escalation_level.to_string(),
                    "requesting_service": "ctas-foundation-daemon"
                }
            })
        )
        .with_escalation(escalation_level);
        
        // Call vault service (running in sx9-conda)
        let response = self.vault_client.call(request).await?;
        
        // Parse credential response
        let credential: APICredential = serde_json::from_value(response.result)?;
        Ok(credential)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APICredential {
    pub service: String,
    pub key: String,
    pub rate_limit: RateLimit,
    pub expires_at: DateTime<Utc>,
    pub tier: CredentialTier,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CredentialTier {
    Local,       // From sx9-api-vault local storage
    GCPSecret,   // From Google Secret Manager
    RuntimeInjected, // From Cloud Run environment
}
```

**Advantages**:
- Rust operations get credentials through RFC-9001/9003/9004 compliant bridge
- Trivariate hash audit trail maintained
- Escalation-aware scoping applied
- Clean service boundary between CTAS and Python layer

---

## Part 3: Federation Strategy

### 3.1 Tier 1 → Tier 2 Synchronization

ABE External API Gateway monitors Tier 1 (sx9-api-vault) and syncs new/updated credentials to Tier 2 (GCP Secret Manager):

```python
# File: sx9-conda/services/vault_sync_service.py
import asyncio
import json
from pathlib import Path
from google.cloud import secretmanager
import hashlib

class VaultSyncService:
    """Federate sx9-api-vault changes to GCP Secret Manager"""
    
    def __init__(self):
        self.vault_path = Path.home() / ".sx9-api-vault"
        self.gcp_project = "gen-lang-client-0290627006"
        self.client = secretmanager.SecretManagerServiceClient()
        self.sync_state_file = self.vault_path / "sync_state.json"
        self.last_synced_hashes = self._load_sync_state()
    
    async def continuous_sync(self):
        """Watch vault changes and sync to GCP"""
        while True:
            try:
                current_state = self._compute_vault_state()
                
                # Only sync changed credentials
                for service_name, hash_value in current_state.items():
                    if self._has_changed(service_name, hash_value):
                        await self._sync_credential_to_gcp(service_name)
                        self.last_synced_hashes[service_name] = hash_value
                
                self._save_sync_state()
                await asyncio.sleep(300)  # Check every 5 minutes
                
            except Exception as e:
                print(f"Sync error: {e}")
                await asyncio.sleep(60)
    
    async def _sync_credential_to_gcp(self, service_name: str):
        """Upload credential to GCP Secret Manager"""
        secret_id = f"cognetix-abe-tool-api-{service_name}"
        credential_data = self._read_vault_credential(service_name)
        
        # Create or update secret
        parent = self.client.project_path(self.gcp_project)
        
        try:
            secret = self.client.create_secret(
                parent,
                secret_id,
                {
                    "replication": {
                        "automatic": {}
                    },
                    "labels": {
                        "managed-by": "sx9-api-vault",
                        "sync-tier": "automated",
                        "service": service_name
                    }
                }
            )
        except Exception:
            # Secret exists, add new version
            secret_name = f"{parent}/secrets/{secret_id}"
        
        # Add secret version
        self.client.add_secret_version(
            secret_name,
            {"data": json.dumps(credential_data).encode()}
        )
        
        print(f"✓ Synced {service_name} to GCP Secret Manager")
    
    def _compute_vault_state(self) -> dict:
        """Hash current vault state"""
        keys_file = self.vault_path / "keys.json"
        with open(keys_file) as f:
            data = json.load(f)
        
        return {
            service: hashlib.sha256(
                json.dumps(creds).encode()
            ).hexdigest()
            for service, creds in data.get("keys", {}).items()
        }
    
    def _has_changed(self, service: str, new_hash: str) -> bool:
        return self.last_synced_hashes.get(service) != new_hash
```

### 3.2 GCP Secret Manager Integration in ABE Services

```hcl
# File: terraform/abe-external-api-gateway.tf

module "cognetix-abe-external-api-gateway" {
  source = "github.com/GoogleCloudPlatform/terraform-google-cloud-run"
  
  service_name = "cognetix-abe-external-api-gateway"
  location     = "us-central1"
  
  service_account_roles = [
    "roles/secretmanager.secretAccessor",  # Read Tier 2 secrets
    "roles/storage.objectAdmin",           # Access tool outputs
    "roles/pubsub.publisher",
  ]
  
  env_vars = {
    # Tier 2 credential sources
    CREDENTIAL_TIER_1_PATH    = "/local/vault"      # Mounted from CI/CD
    CREDENTIAL_TIER_2_PROJECT = "gen-lang-client-0290627006"
    CREDENTIAL_TIER_2_PREFIX  = "cognetix-abe-tool-api-"
    
    # Service configuration
    VAULT_FALLBACK_ORDER  = "gcp,local,env"
    CREDENTIAL_CACHE_TTL  = "3600"
    ESCALATION_ENFORCEMENT = "true"
  }
  
  # Mount Tier 1 vault (for dev/testing)
  volumes = [
    {
      name = "vault-mount"
      secret {
        secret_id = "sx9-api-vault-keys"
        items = [{
          key  = "keys.json"
          path = "keys.json"
        }]
      }
    }
  ]
  
  volume_mounts = [
    {
      name       = "vault-mount"
      mount_path = "/local/vault"
    }
  ]
}
```

---

## Part 4: Unified Service Interface

### 4.1 Python Service Endpoint (Port 18119 - Unified Gateway)

```python
# File: sx9-conda/services/vault_gateway_service.py
import json
from fastapi import FastAPI, HTTPException, Header
from typing import Optional

app = FastAPI(title="ABE Vault Gateway", version="1.0.0")

class VaultGatewayService:
    """Unified API for accessing Tier 1 + Tier 2 credentials"""
    
    def __init__(self):
        self.local_vault = LocalVaultClient("~/.sx9-api-vault")
        self.gcp_vault = GCPSecretManagerClient("gen-lang-client-0290627006")
        self.cache = CredentialCache(ttl=3600)
    
    @app.post("/credential")
    async def get_credential(
        self,
        service: str,
        escalation_level: str = "Tactical",
        requesting_service: str = Header(...),
        trivariate_hash: str = Header(...),
    ):
        """Federated credential retrieval with audit trail"""
        
        # Check cache first
        cache_key = f"{service}:{escalation_level}:{requesting_service}"
        if cached := self.cache.get(cache_key):
            return cached
        
        # Tier 1: Try local vault
        try:
            credential = self.local_vault.get(service)
            credential["tier"] = "local"
        except Exception:
            # Tier 2: Fallback to GCP
            try:
                credential = await self.gcp_vault.get(
                    f"cognetix-abe-tool-api-{service}"
                )
                credential["tier"] = "gcp"
            except Exception as e:
                raise HTTPException(status_code=404, detail=f"Credential not found: {service}")
        
        # Apply escalation scoping
        credential = self._apply_escalation_scope(
            credential,
            escalation_level,
            requesting_service
        )
        
        # Audit log with trivariate hash
        self._audit_log(
            service=service,
            requesting_service=requesting_service,
            trivariate_hash=trivariate_hash,
            escalation_level=escalation_level,
            tier=credential["tier"]
        )
        
        # Cache and return
        self.cache.set(cache_key, credential)
        return credential
    
    def _apply_escalation_scope(self, cred, level, service):
        """Limit credential scope based on escalation level"""
        escalation_limits = {
            "Tactical": {"rate_limit": 100, "max_calls": 1000},
            "Operational": {"rate_limit": 500, "max_calls": 5000},
            "Strategic": {"rate_limit": None, "max_calls": None}
        }
        
        limits = escalation_limits.get(level, escalation_limits["Tactical"])
        cred["effective_rate_limit"] = limits["rate_limit"]
        return cred
    
    def _audit_log(self, **kwargs):
        """Log all credential access for compliance"""
        audit_entry = {
            "timestamp": datetime.utcnow().isoformat(),
            **kwargs
        }
        # Write to audit trail (GCP Cloud Logging, DataStax, etc.)
        print(f"[AUDIT] {json.dumps(audit_entry)}")
```

---

## Part 5: Implementation Roadmap

### Phase 1: Extend sx9-api-vault (Week 1)

- [ ] Add tool registry with 100+ Kali tools
- [ ] Implement `crawl` command for automated discovery
- [ ] Create tool triage workflow (test/validate/report)
- [ ] Add federation state tracking

### Phase 2: Create Vault Gateway Service (Week 2)

- [ ] Build unified FastAPI endpoint
- [ ] Implement GCP Secret Manager client
- [ ] Add credential caching layer
- [ ] Create audit logging system

### Phase 3: Integrate with ctas7-foundation-python-client (Week 3)

- [ ] Add vault credential factory
- [ ] Support Rust-to-Python bridge for credentials
- [ ] Implement escalation scoping
- [ ] Add comprehensive error handling

### Phase 4: ABE Service Integration (Week 4)

- [ ] Update External API Gateway to use vault
- [ ] Integrate with Ingestion Service
- [ ] Integrate with Summarization Service
- [ ] Add production safety checks

---

## Part 6: Security & Compliance

### 6.1 Multi-Tier Security Model

| Tier | Encryption | Access Control | Audit | Use Case |
|------|-----------|-----------------|-------|----------|
| **Tier 1 (Local)** | Optional (plaintext/encrypted file) | File permissions | .log file | Development, testing |
| **Tier 2 (GCP)** | AES-256 at rest | IAM + Secret Manager ACLs | Cloud Logging | Production, staging |
| **Tier 3 (Runtime)** | In-memory (non-persistent) | Per-service injection | Application logs | Service execution |

### 6.2 Credential Lifecycle

```
New Tool Registration
  ↓
[sx9-api-vault] - Local testing with test credentials
  ↓
Tool Validation (live API test)
  ↓
Manual Approval (security review)
  ↓
[Tier 1 → Tier 2 Sync] - Automatic push to GCP
  ↓
[ABE Services] - Automatic credential injection
  ↓
Monitoring & Alerts (rate limit, errors)
  ↓
Rotation/Revocation
```

### 6.3 Escalation Scoping

```rust
// Tactical: Limited access for initial reconnaissance
Tactical {
    rate_limit: 100 requests/month,
    tools_allowed: ["basic_osint"],
    api_endpoints: ["read_only"],
    data_retention: 7 days
}

// Operational: Full access for investigation
Operational {
    rate_limit: 500 requests/month,
    tools_allowed: ["all_approved"],
    api_endpoints: ["read_write"],
    data_retention: 30 days
}

// Strategic: Unlimited access for enterprise operations
Strategic {
    rate_limit: unlimited,
    tools_allowed: ["all"],
    api_endpoints: ["full_access"],
    data_retention: 365 days
}
```

---

## Part 7: Advantages of Dual-Access

### Python Direct (sx9-conda)
- ✅ Instant access during development
- ✅ No network latency for local keys
- ✅ Easy credential testing and validation
- ✅ Lightweight for microservices

### Rust Bridge (ctas7-foundation-python-client)
- ✅ Full RFC-9001/9003/9004 compliance
- ✅ Unified audit trail with trivariate hashing
- ✅ Escalation-aware access control
- ✅ Clean service boundary (Python isolated)
- ✅ Type-safe credential handling in Rust

### Federation (Both Tiers)
- ✅ Seamless dev-to-prod transition
- ✅ No code changes for environment switching
- ✅ Comprehensive audit trail across tiers
- ✅ Supports hundreds of tools at scale
- ✅ Flexible credential rotation

---

## Usage Examples

### Example 1: Deploy Shodan + Censys to ABE

```bash
# Register credentials
python3 sx9-api-vault.py register SHODAN_KEY shodan <key> \
  --tool-category osint --tool-name theHarvester

python3 sx9-api-vault.py register CENSYS_KEY censys <key> \
  --tool-category osint --tool-name censys-python

# Validate against live APIs
python3 sx9-api-vault.py validate --services shodan,censys

# Export to GCP for ABE
python3 sx9-api-vault.py export --target=gcp-secret-manager \
  --project=gen-lang-client-0290627006 \
  --prefix=cognetix-abe-tool-api-

# Trigger sync
curl -X POST http://localhost:18119/sync
```

### Example 2: CTAS Operation Needs API Key

```rust
let bridge = ABEAPIBridge::new();

// Request credential with operational context
let credential = bridge.get_api_credential(
    "virustotal",
    EscalationLevel::Operational
).await?;

// VT API call with scoped limit (500/month)
let hash = "d41d8cd98f00b204e9800998ecf8427e";
let response = virustotal_api::lookup_hash(
    &hash,
    &credential.key,
    credential.rate_limit
)?;
```

### Example 3: Service Triage - Testing 50 OSINT Tools

```bash
# Crawl Kali for OSINT tools requiring APIs
python3 sx9-api-vault.py crawl --category osint \
  --output osint-requirements.json

# Bulk register test credentials (from free tiers)
python3 sx9-api-vault.py bulk-import osint-requirements.json

# Validate all at once
python3 sx9-api-vault.py validate --all --parallel 10

# Generate coverage report
python3 sx9-api-vault.py report --format=matrix \
  --output osint-coverage.html

# Output: Shows which tools have keys, which are free, which need registration
```

---

## Next Steps

1. **Extend sx9-api-vault** with tool registry and triage workflow
2. **Create vault-gateway-service** in sx9-conda (port 18119)
3. **Update ctas7-foundation-python-client** with credential factory
4. **Integrate ABE services** with vault gateway
5. **Deploy to staging** for integrated testing
6. **Production rollout** with GCP Secret Manager

---

## References

- **sx9-api-vault**: `/Users/cp5337/Developer/sx9-conda/scripts/api-management/sx9-api-vault.py`
- **ctas7-foundation-python-client**: `/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-foundation-python-client/`
- **ABE Architecture**: `/Users/cp5337/Developer/ctas-7-shipyard-staging/03-docs/architecture/ABE_INTEGRATED_DEPLOYMENT_ARCHITECTURE.md`
- **ctas7-foundation-daemon**: `/Users/cp5337/Developer/ctas-7-shipyard-staging/ctas7-foundation-daemon/src/services/abe_controlled_access.rs`
