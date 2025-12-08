# SX9 CDN Terraform Specification

## Overview

Three-tier CDN architecture:
1. **Cloudflare R2** - Public/semi-public assets ($0 egress)
2. **GCP Cloud CDN** - IAM-gated assets ($0 idle LB)
3. **Internal** - Classified/air-gapped (existing infra)

## Tier 1: Cloudflare R2

### Buckets

```hcl
# terraform/cloudflare/r2_buckets.tf

variable "cf_account_id" {
  description = "Cloudflare account ID"
  type        = string
}

# Foundation crates (obfuscated, signed)
resource "cloudflare_r2_bucket" "sx9_crates" {
  account_id = var.cf_account_id
  name       = "sx9-foundation-crates"
  location   = "WNAM"  # West North America
}

# Geospatial assets (Cesium tiles, terrain)
resource "cloudflare_r2_bucket" "sx9_geo" {
  account_id = var.cf_account_id
  name       = "sx9-geospatial"
  location   = "WNAM"
}

# UI assets (JS, CSS, fonts, WASM)
resource "cloudflare_r2_bucket" "sx9_assets" {
  account_id = var.cf_account_id
  name       = "sx9-ui-assets"
  location   = "WNAM"
}

# Documentation and schemas
resource "cloudflare_r2_bucket" "sx9_docs" {
  account_id = var.cf_account_id
  name       = "sx9-documentation"
  location   = "WNAM"
}
```

### DNS

```hcl
# terraform/cloudflare/dns.tf

resource "cloudflare_zone" "sx9_io" {
  account_id = var.cf_account_id
  zone       = "sx9.io"
}

resource "cloudflare_record" "cdn" {
  zone_id = cloudflare_zone.sx9_io.id
  name    = "cdn"
  value   = cloudflare_r2_bucket.sx9_assets.endpoint
  type    = "CNAME"
  proxied = true
}

resource "cloudflare_record" "crates" {
  zone_id = cloudflare_zone.sx9_io.id
  name    = "crates"
  value   = cloudflare_r2_bucket.sx9_crates.endpoint
  type    = "CNAME"
  proxied = true
}
```

### Workers (Optional - for auth/routing)

```hcl
# terraform/cloudflare/workers.tf

resource "cloudflare_worker_script" "crate_auth" {
  account_id = var.cf_account_id
  name       = "sx9-crate-auth"
  content    = file("${path.module}/workers/crate-auth.js")
}

resource "cloudflare_worker_route" "crate_auth" {
  zone_id     = cloudflare_zone.sx9_io.id
  pattern     = "crates.sx9.io/*"
  script_name = cloudflare_worker_script.crate_auth.name
}
```

## Tier 2: GCP Cloud CDN

### Storage Bucket

```hcl
# terraform/gcp/storage.tf

resource "google_storage_bucket" "sx9_crates" {
  name          = "sx9-operator-crates"
  location      = "US"
  force_destroy = false
  
  uniform_bucket_level_access = true
  
  versioning {
    enabled = true
  }
  
  lifecycle_rule {
    condition {
      age = 90
    }
    action {
      type = "Delete"
    }
  }
}

resource "google_storage_bucket" "sx9_partner" {
  name          = "sx9-partner-distributions"
  location      = "US"
  force_destroy = false
  
  uniform_bucket_level_access = true
}
```

### Cloud CDN + Load Balancer

```hcl
# terraform/gcp/cdn.tf

# Backend bucket with CDN enabled
resource "google_compute_backend_bucket" "sx9_cdn" {
  name        = "sx9-cdn-backend"
  bucket_name = google_storage_bucket.sx9_crates.name
  enable_cdn  = true
  
  cdn_policy {
    cache_mode        = "CACHE_ALL_STATIC"
    default_ttl       = 3600
    max_ttl           = 86400
    client_ttl        = 3600
    negative_caching  = true
    
    cache_key_policy {
      include_host         = true
      include_protocol     = true
      include_query_string = false
    }
    
    signed_url_cache_max_age_sec = 7200
  }
}

# URL map
resource "google_compute_url_map" "sx9_cdn" {
  name            = "sx9-cdn-urlmap"
  default_service = google_compute_backend_bucket.sx9_cdn.id
  
  host_rule {
    hosts        = ["crates.synaptix9.com"]
    path_matcher = "crates"
  }
  
  path_matcher {
    name            = "crates"
    default_service = google_compute_backend_bucket.sx9_cdn.id
    
    path_rule {
      paths   = ["/v1/*"]
      service = google_compute_backend_bucket.sx9_cdn.id
    }
  }
}

# HTTPS proxy (LB is free when idle!)
resource "google_compute_target_https_proxy" "sx9_cdn" {
  name             = "sx9-cdn-https-proxy"
  url_map          = google_compute_url_map.sx9_cdn.id
  ssl_certificates = [google_compute_managed_ssl_certificate.sx9_cdn.id]
}

# Managed SSL certificate
resource "google_compute_managed_ssl_certificate" "sx9_cdn" {
  name = "sx9-cdn-cert"
  
  managed {
    domains = ["crates.synaptix9.com"]
  }
}

# Global forwarding rule
resource "google_compute_global_forwarding_rule" "sx9_cdn" {
  name       = "sx9-cdn-forwarding-rule"
  target     = google_compute_target_https_proxy.sx9_cdn.id
  port_range = "443"
  ip_address = google_compute_global_address.sx9_cdn.address
}

# Static IP
resource "google_compute_global_address" "sx9_cdn" {
  name = "sx9-cdn-ip"
}
```

### IAM

```hcl
# terraform/gcp/iam.tf

# Service account for CDN access
resource "google_service_account" "sx9_cdn_reader" {
  account_id   = "sx9-cdn-reader"
  display_name = "SX9 CDN Reader"
}

# Bucket IAM binding
resource "google_storage_bucket_iam_binding" "sx9_crates_reader" {
  bucket = google_storage_bucket.sx9_crates.name
  role   = "roles/storage.objectViewer"
  
  members = [
    "serviceAccount:${google_service_account.sx9_cdn_reader.email}",
  ]
}

# Signed URL key for time-limited access
resource "google_compute_backend_bucket_signed_url_key" "sx9_cdn" {
  name           = "sx9-cdn-signed-key"
  key_value      = base64encode(random_password.signed_url_key.result)
  backend_bucket = google_compute_backend_bucket.sx9_cdn.name
}

resource "random_password" "signed_url_key" {
  length  = 128
  special = false
}
```

## Port Filter Spec

```hcl
# terraform/gcp/armor.tf

# Cloud Armor policy for C2 blocking
resource "google_compute_security_policy" "sx9_c2_filter" {
  name = "sx9-c2-filter"
  
  # Block Cobalt Strike beacon patterns
  rule {
    action   = "deny(403)"
    priority = 1000
    match {
      expr {
        expression = "request.headers['user-agent'].matches('Mozilla/5.0.*MSIE.*')"
      }
    }
    description = "Block Cobalt Strike default UA"
  }
  
  # Block suspicious beacon intervals (rate limiting)
  rule {
    action   = "rate_based_ban"
    priority = 2000
    match {
      versioned_expr = "SRC_IPS_V1"
      config {
        src_ip_ranges = ["*"]
      }
    }
    rate_limit_options {
      conform_action = "allow"
      exceed_action  = "deny(429)"
      rate_limit_threshold {
        count        = 100
        interval_sec = 60
      }
      ban_duration_sec = 600
    }
    description = "Rate limit to detect beacon patterns"
  }
  
  # Block high-entropy payloads in headers
  rule {
    action   = "deny(403)"
    priority = 3000
    match {
      expr {
        expression = "request.headers['cookie'].size() > 4096"
      }
    }
    description = "Block oversized cookies (potential C2 channel)"
  }
  
  # Default allow
  rule {
    action   = "allow"
    priority = 2147483647
    match {
      versioned_expr = "SRC_IPS_V1"
      config {
        src_ip_ranges = ["*"]
      }
    }
    description = "Default allow"
  }
}
```

## Directory Structure

```
terraform/
├── cloudflare/
│   ├── main.tf
│   ├── r2_buckets.tf
│   ├── dns.tf
│   ├── workers.tf
│   ├── variables.tf
│   ├── outputs.tf
│   └── workers/
│       └── crate-auth.js
├── gcp/
│   ├── main.tf
│   ├── storage.tf
│   ├── cdn.tf
│   ├── iam.tf
│   ├── armor.tf
│   ├── variables.tf
│   └── outputs.tf
├── variables.tf
├── outputs.tf
├── backend.tf
└── versions.tf
```

## Estimated Costs

| Service | Monthly Cost |
|---------|--------------|
| Cloudflare R2 Storage (10GB) | $0.15 |
| Cloudflare R2 Operations | ~$1-5 |
| Cloudflare R2 Egress | $0 |
| GCP GCS Storage (10GB) | $0.20 |
| GCP Cloud CDN Cache | ~$5-20 |
| GCP LB (idle) | $0 |
| GCP Cloud Armor | ~$5/policy |
| **Total** | **~$15-50/month** |

## Deployment

```bash
# Initialize
cd terraform
terraform init

# Plan
terraform plan -var-file="production.tfvars"

# Apply
terraform apply -var-file="production.tfvars"
```

## production.tfvars

```hcl
cf_account_id = "your-cloudflare-account-id"
gcp_project   = "your-gcp-project"
gcp_region    = "us-central1"
```




