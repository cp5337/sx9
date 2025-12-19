# SX9 Infrastructure v2.0 - Three Vertical Architecture

## Overview

Production-ready Terraform for the SX9 platform with three verticals:

| Vertical | Domain | Description |
|----------|--------|-------------|
| **ops-main** | ops.sx9.io | Main operations platform, forward provisioning |
| **orbital** | orbital.sx9.io | Laser Light free space optical satellite constellation |
| **dev-center** | dev.sx9.io | DevSecOps/EA solutions center (DoD anchor) |
| **gallery** | gallery.sx9.io | iTunes-style toolchain distribution |

## Architecture

```
                    ┌─────────────────┐
                    │   Cloudflare    │
                    │   WAF + CDN     │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │  GCP Global LB  │
                    │   + SSL/TLS     │
                    └────────┬────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
   ┌────▼────┐         ┌─────▼────┐         ┌────▼────┐
   │ops-main │         │ orbital  │         │dev-ctr  │
   │ Cloud   │         │ Cloud    │         │ Cloud   │
   │  Run    │         │  Run     │         │  Run    │
   └────┬────┘         └────┬─────┘         └────┬────┘
        │                   │                    │
        └───────────────────┼────────────────────┘
                            │
              ┌─────────────┴─────────────┐
              │                           │
        ┌─────▼─────┐              ┌──────▼──────┐
        │   NATS    │              │  Nonagon    │
        │ JetStream │              │  GPU Pool   │
        │ (3 nodes) │              │ (0-4 nodes) │
        └─────┬─────┘              └─────────────┘
              │                    (scale to zero)
        ┌─────▼─────┐
        │  AlloyDB  │
        │   + GCS   │
        └───────────┘
```

## Components

### Message Fabric (Forward Provisioning)
- **NATS JetStream** cluster (3 nodes)
- Internal TCP load balancer
- Subjects: `sx9.*`, `forge.*`, `orbital.*`

### GPU Burst Compute (Nonagon)
- Scale-to-zero GPU pool (T4)
- NATS-coordinated job queue
- Preemptible VMs for cost optimization

### Storage
| Bucket | Purpose |
|--------|---------|
| cesium-frontend | Cesium.js assets for GIS |
| gallery-assets | Toolchain packages (iTunes model) |
| simulation-data | Orbital simulation data |
| threat-intel | ABE threat intelligence |

### PubSub Topics
| Topic | Vertical | RFC |
|-------|----------|-----|
| hash-event-topic | ops-main | RFC-9001 |
| state-update-topic | orbital | RFC-9502A |
| abe-ingestion-topic | dev-center | RFC-9105 |
| gallery-events-topic | gallery | - |

## Deployment

```bash
# 1. Copy example tfvars
cp terraform.tfvars.example terraform.tfvars

# 2. Fill in sensitive values
# - cloudflare_api_token
# - cloudflare_account_id
# - cloudflare_zone_id

# 3. Initialize
terraform init

# 4. Plan
terraform plan

# 5. Apply
terraform apply
```

## RFC Compliance

- **RFC-9001**: Trivariate hashing (hash-event-topic)
- **RFC-9105**: SPIRES extraction (abe-ingestion)
- **RFC-9130**: L2 NATS execution
- **RFC-9200**: Development center
- **RFC-9302**: Nonagon burst compute
- **RFC-9502A**: Orbital state management
- **RFC-9876**: Layer-Two orchestration

## Cost Optimization

- Cloud Run: Scale to zero (ops-main, dev-center)
- Cloud Run: Always-on minimum (orbital - 24/7 ops)
- Nonagon: Preemptible GPUs, scale to zero
- Gallery: Heavy CDN caching (7-day edge TTL)

## Migration from ADC Template

This replaces the auto-generated ADC terraform with:
1. Consolidated variables (1 project_id vs 20+)
2. Proper vertical separation
3. NATS message fabric (vs PubSub only)
4. Nonagon GPU burst compute
5. Cloudflare integration
6. Gallery infrastructure
7. Proper container images (not hello placeholder)
