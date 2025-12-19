# ═══════════════════════════════════════════════════════════════════════════════
# SX9 Infrastructure v2.0 - Three Vertical Architecture
# ═══════════════════════════════════════════════════════════════════════════════
#
# VERTICALS:
#   1. sx9-ops-main    - Main operations platform, forward provisioning
#   2. sx9-orbital     - Laser Light free space optical sat constellation
#   3. sx9-dev-center  - DevSecOps/EA solutions center (DoD anchor)
#
# COMPONENTS:
#   - GCP Global Load Balancer + CDN
#   - Cloudflare WAF/DNS (origin shield)
#   - NATS JetStream (forward provisioning)
#   - Nonagon GPU/HPC burst (scale-to-zero)
#   - Gallery (iTunes-style toolchains)
#   - AlloyDB + SurrealDB hybrid storage
#
# RFC: 9130, 9200, 9876, 9302
# ═══════════════════════════════════════════════════════════════════════════════

terraform {
  required_version = ">= 1.5"
  
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
    google-beta = {
      source  = "hashicorp/google-beta"
      version = "~> 5.0"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.0"
    }
  }
  
  backend "gcs" {
    bucket = "sx9-terraform-state"
    prefix = "production/v2"
  }
}

# ─────────────────────────────────────────────────────────────────────────────────
# PROVIDERS
# ─────────────────────────────────────────────────────────────────────────────────

provider "google" {
  project = var.project_id
  region  = var.region
  
  default_labels = {
    managed-by  = "terraform"
    platform    = "sx9"
    environment = var.environment
  }
}

provider "google-beta" {
  project = var.project_id
  region  = var.region
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

# ─────────────────────────────────────────────────────────────────────────────────
# NETWORKING MODULE
# ─────────────────────────────────────────────────────────────────────────────────

module "networking" {
  source     = "./modules/networking"
  project_id = var.project_id
  region     = var.region
}

# ─────────────────────────────────────────────────────────────────────────────────
# NATS MODULE - Message Fabric
# ─────────────────────────────────────────────────────────────────────────────────

module "nats" {
  source       = "./modules/nats"
  project_id   = var.project_id
  region       = var.region
  network_id   = module.networking.network_id
  subnet_id    = module.networking.shared_subnet_id
  cluster_size = var.nats_cluster_size
}

# ─────────────────────────────────────────────────────────────────────────────────
# NONAGON MODULE - GPU Burst Compute
# ─────────────────────────────────────────────────────────────────────────────────

module "nonagon" {
  source     = "./modules/nonagon"
  project_id = var.project_id
  region     = var.region
  network_id = module.networking.network_id
  subnet_id  = module.networking.shared_subnet_id
  gpu_type   = var.gpu_type
  max_nodes  = var.nonagon_max_nodes
  nats_ip    = module.nats.nats_internal_ip
}

# ─────────────────────────────────────────────────────────────────────────────────
# SERVICE ACCOUNTS
# ─────────────────────────────────────────────────────────────────────────────────

resource "google_service_account" "ops_main" {
  account_id   = "sx9-ops-main-sa"
  display_name = "SX9 Ops Main"
}

resource "google_service_account" "orbital" {
  account_id   = "sx9-orbital-sa"
  display_name = "SX9 Orbital"
}

resource "google_service_account" "dev_center" {
  account_id   = "sx9-dev-center-sa"
  display_name = "SX9 Dev Center"
}

resource "google_service_account" "gallery" {
  account_id   = "sx9-gallery-sa"
  display_name = "SX9 Gallery"
}

# IAM Bindings
locals {
  service_accounts = {
    ops_main   = google_service_account.ops_main.email
    orbital    = google_service_account.orbital.email
    dev_center = google_service_account.dev_center.email
    gallery    = google_service_account.gallery.email
  }
}

resource "google_project_iam_member" "sa_pubsub" {
  for_each = local.service_accounts
  project  = var.project_id
  role     = "roles/pubsub.publisher"
  member   = "serviceAccount:${each.value}"
}

resource "google_project_iam_member" "sa_secrets" {
  for_each = local.service_accounts
  project  = var.project_id
  role     = "roles/secretmanager.secretAccessor"
  member   = "serviceAccount:${each.value}"
}

resource "google_project_iam_member" "dev_center_vertex" {
  project = var.project_id
  role    = "roles/aiplatform.user"
  member  = "serviceAccount:${google_service_account.dev_center.email}"
}

# ─────────────────────────────────────────────────────────────────────────────────
# SECRET MANAGER
# ─────────────────────────────────────────────────────────────────────────────────

resource "google_secret_manager_secret" "supabase" {
  secret_id = "supabase-credentials"
  replication { auto {} }
}

resource "google_secret_manager_secret" "surrealdb" {
  secret_id = "surrealdb-credentials"
  replication { auto {} }
}

resource "google_secret_manager_secret" "api_vault" {
  secret_id = "sx9-api-vault"
  replication { auto {} }
}

# ─────────────────────────────────────────────────────────────────────────────────
# PUBSUB TOPICS
# ─────────────────────────────────────────────────────────────────────────────────

resource "google_pubsub_topic" "hash_events" {
  name                       = "hash-event-topic"
  message_retention_duration = "86400s"
  labels = { vertical = "ops-main", rfc = "9001" }
}

resource "google_pubsub_topic" "state_updates" {
  name                       = "state-update-topic"
  message_retention_duration = "3600s"
  labels = { vertical = "orbital", rfc = "9502a" }
}

resource "google_pubsub_topic" "abe_ingestion" {
  name                       = "abe-ingestion-topic"
  message_retention_duration = "604800s"
  labels = { vertical = "dev-center", service = "abe" }
}

resource "google_pubsub_topic" "gallery_events" {
  name                       = "gallery-events-topic"
  message_retention_duration = "86400s"
  labels = { vertical = "gallery" }
}

# ─────────────────────────────────────────────────────────────────────────────────
# CLOUD RUN SERVICES - ops-main
# ─────────────────────────────────────────────────────────────────────────────────

resource "google_cloud_run_v2_service" "slot_graph" {
  name     = "rust-slot-graph-service"
  location = var.region

  template {
    service_account = google_service_account.ops_main.email
    
    scaling {
      min_instance_count = 0
      max_instance_count = 10
    }
    
    vpc_access {
      connector = module.networking.vpc_connector_id
      egress    = "ALL_TRAFFIC"
    }

    containers {
      name  = "slot-graph"
      image = "${var.container_registry}/slot-graph-service:latest"
      
      ports { container_port = 8080 }
      
      env { name = "HASH_EVENT_TOPIC"; value = google_pubsub_topic.hash_events.id }
      env { name = "NATS_URL"; value = "nats://${module.nats.nats_internal_ip}:4222" }
      
      resources {
        limits = { cpu = "2"; memory = "4Gi" }
      }
    }
  }
  
  labels = { vertical = "ops-main" }
}

resource "google_cloud_run_v2_service" "fusion_layer" {
  name     = "fusion-layer-service"
  location = var.region

  template {
    service_account = google_service_account.ops_main.email
    
    scaling {
      min_instance_count = 0
      max_instance_count = 10
    }
    
    vpc_access {
      connector = module.networking.vpc_connector_id
      egress    = "ALL_TRAFFIC"
    }

    containers {
      name  = "fusion-layer"
      image = "${var.container_registry}/fusion-layer-service:latest"
      
      ports { container_port = 8080 }
      
      env { name = "HASH_EVENT_TOPIC"; value = google_pubsub_topic.hash_events.id }
      env { name = "STATE_UPDATE_TOPIC"; value = google_pubsub_topic.state_updates.id }
      env { name = "NATS_URL"; value = "nats://${module.nats.nats_internal_ip}:4222" }
      
      resources {
        limits = { cpu = "2"; memory = "4Gi" }
      }
    }
  }
  
  labels = { vertical = "ops-main" }
}

# ─────────────────────────────────────────────────────────────────────────────────
# CLOUD RUN SERVICES - orbital
# ─────────────────────────────────────────────────────────────────────────────────

resource "google_cloud_run_v2_service" "simulation_orchestrator" {
  name     = "simulation-orchestrator-service"
  location = var.region

  template {
    service_account = google_service_account.orbital.email
    timeout         = "60s"
    
    scaling {
      min_instance_count = 1  # Always-on for satellite ops
      max_instance_count = 5
    }
    
    vpc_access {
      connector = module.networking.vpc_connector_id
      egress    = "ALL_TRAFFIC"
    }

    containers {
      name  = "sim-orchestrator"
      image = "${var.container_registry}/simulation-orchestrator:latest"
      
      ports { container_port = 8080 }
      
      env { name = "STATE_UPDATE_TOPIC"; value = google_pubsub_topic.state_updates.id }
      env { name = "ABE_INGESTION_TOPIC"; value = google_pubsub_topic.abe_ingestion.id }
      env { name = "NATS_URL"; value = "nats://${module.nats.nats_internal_ip}:4222" }
      
      resources {
        limits = { cpu = "4"; memory = "8Gi" }
      }
    }
  }
  
  labels = { vertical = "orbital", crystal_type = "orbital-sim" }
}

resource "google_cloud_run_v2_service" "ground_station" {
  name     = "simulated-ground-station-crate"
  location = var.region

  template {
    service_account = google_service_account.orbital.email
    timeout         = "300s"  # Long timeout for satellite passes
    
    scaling {
      min_instance_count = 1
      max_instance_count = 3
    }
    
    vpc_access {
      connector = module.networking.vpc_connector_id
      egress    = "ALL_TRAFFIC"
    }

    containers {
      name  = "ground-station"
      image = "${var.container_registry}/ground-station-crate:latest"
      
      ports { container_port = 8080 }
      
      env { name = "STATE_UPDATE_TOPIC"; value = google_pubsub_topic.state_updates.id }
      env { name = "NATS_URL"; value = "nats://${module.nats.nats_internal_ip}:4222" }
      
      resources {
        limits = { cpu = "2"; memory = "4Gi" }
      }
    }
  }
  
  labels = { vertical = "orbital", crystal_type = "ground-station" }
}

# ─────────────────────────────────────────────────────────────────────────────────
# CLOUD RUN SERVICES - dev-center
# ─────────────────────────────────────────────────────────────────────────────────

resource "google_cloud_run_v2_service" "crate_processor" {
  name     = "crate-processor-service"
  location = var.region

  template {
    service_account = google_service_account.dev_center.email
    
    scaling {
      min_instance_count = 0
      max_instance_count = 20  # High scale for batch processing
    }
    
    vpc_access {
      connector = module.networking.vpc_connector_id
      egress    = "ALL_TRAFFIC"
    }

    containers {
      name  = "crate-processor"
      image = "${var.container_registry}/crate-processor:latest"
      
      ports { container_port = 8080 }
      
      env { name = "HASH_EVENT_TOPIC"; value = google_pubsub_topic.hash_events.id }
      env { name = "ABE_INGESTION_TOPIC"; value = google_pubsub_topic.abe_ingestion.id }
      env { name = "NATS_URL"; value = "nats://${module.nats.nats_internal_ip}:4222" }
      
      resources {
        limits = { cpu = "4"; memory = "8Gi" }
      }
    }
  }
  
  labels = { vertical = "dev-center" }
}

resource "google_cloud_run_v2_service" "ea_automation" {
  name     = "ea-automation-service"
  location = var.region

  template {
    service_account = google_service_account.dev_center.email
    
    scaling {
      min_instance_count = 1  # Always-on for EA clients
      max_instance_count = 5
    }
    
    vpc_access {
      connector = module.networking.vpc_connector_id
      egress    = "ALL_TRAFFIC"
    }

    containers {
      name  = "ea-automation"
      image = "${var.container_registry}/ea-automation:latest"
      
      ports { container_port = 8080 }
      
      env { name = "ABE_INGESTION_TOPIC"; value = google_pubsub_topic.abe_ingestion.id }
      env { name = "NATS_URL"; value = "nats://${module.nats.nats_internal_ip}:4222" }
      
      resources {
        limits = { cpu = "2"; memory = "4Gi" }
      }
    }
  }
  
  labels = { vertical = "dev-center", client = "dod-anchor" }
}

# ─────────────────────────────────────────────────────────────────────────────────
# CLOUD RUN SERVICES - gallery
# ─────────────────────────────────────────────────────────────────────────────────

resource "google_cloud_run_v2_service" "gallery_api" {
  name     = "gallery-api-service"
  location = var.region

  template {
    service_account = google_service_account.gallery.email
    
    scaling {
      min_instance_count = 1
      max_instance_count = 10
    }
    
    vpc_access {
      connector = module.networking.vpc_connector_id
      egress    = "ALL_TRAFFIC"
    }

    containers {
      name  = "gallery-api"
      image = "${var.container_registry}/gallery-api:latest"
      
      ports { container_port = 8080 }
      
      env { name = "GALLERY_EVENTS_TOPIC"; value = google_pubsub_topic.gallery_events.id }
      env { name = "GALLERY_BUCKET"; value = google_storage_bucket.gallery_assets.name }
      
      resources {
        limits = { cpu = "2"; memory = "2Gi" }
      }
    }
  }
  
  labels = { vertical = "gallery" }
}

# ─────────────────────────────────────────────────────────────────────────────────
# CLOUD STORAGE
# ─────────────────────────────────────────────────────────────────────────────────

resource "google_storage_bucket" "cesium_frontend" {
  name          = "${var.project_id}-cesium-frontend"
  location      = "US"
  storage_class = "STANDARD"
  
  uniform_bucket_level_access = true
  
  website {
    main_page_suffix = "index.html"
    not_found_page   = "404.html"
  }
  
  labels = { vertical = "ops-main" }
}

resource "google_storage_bucket" "gallery_assets" {
  name          = "${var.project_id}-gallery-assets"
  location      = "US"
  storage_class = "STANDARD"
  
  uniform_bucket_level_access = true
  versioning { enabled = true }
  
  lifecycle_rule {
    condition { age = 365 }
    action { type = "SetStorageClass"; storage_class = "NEARLINE" }
  }
  
  labels = { vertical = "gallery" }
}

resource "google_storage_bucket" "simulation_data" {
  name          = "${var.project_id}-simulation-data"
  location      = var.region
  storage_class = "STANDARD"
  
  uniform_bucket_level_access = true
  versioning { enabled = true }
  
  labels = { vertical = "orbital" }
}

resource "google_storage_bucket" "threat_intel" {
  name          = "${var.project_id}-threat-intel"
  location      = var.region
  storage_class = "STANDARD"
  
  uniform_bucket_level_access = true
  versioning { enabled = true }
  
  labels = { vertical = "dev-center", service = "abe" }
}

# ─────────────────────────────────────────────────────────────────────────────────
# ALLOYDB
# ─────────────────────────────────────────────────────────────────────────────────

resource "random_password" "alloydb" {
  length  = 32
  special = true
}

resource "google_alloydb_cluster" "sx9" {
  cluster_id = "sx9-primary"
  location   = var.region
  
  network_config {
    network = module.networking.network_id
  }
  
  initial_user {
    user     = "sx9admin"
    password = random_password.alloydb.result
  }
  
  automated_backup_policy {
    enabled = true
    weekly_schedule {
      days_of_week = ["SUNDAY"]
      start_times { hours = 3; minutes = 0 }
    }
    backup_window = "3600s"
    quantity_based_retention { count = 7 }
  }
}

resource "google_alloydb_instance" "sx9" {
  cluster       = google_alloydb_cluster.sx9.name
  instance_id   = "sx9-primary-instance"
  instance_type = "PRIMARY"
  
  machine_config {
    cpu_count = var.alloydb_cpu_count
  }
  
  database_flags = {
    "max_connections" = "500"
  }
}

# ─────────────────────────────────────────────────────────────────────────────────
# GLOBAL LOAD BALANCER
# ─────────────────────────────────────────────────────────────────────────────────

resource "google_compute_global_address" "sx9" {
  name = "sx9-global-ip"
}

# Serverless NEGs
resource "google_compute_region_network_endpoint_group" "ops_main" {
  name                  = "ops-main-neg"
  network_endpoint_type = "SERVERLESS"
  region                = var.region
  cloud_run { service = google_cloud_run_v2_service.slot_graph.name }
}

resource "google_compute_region_network_endpoint_group" "orbital" {
  name                  = "orbital-neg"
  network_endpoint_type = "SERVERLESS"
  region                = var.region
  cloud_run { service = google_cloud_run_v2_service.simulation_orchestrator.name }
}

resource "google_compute_region_network_endpoint_group" "dev_center" {
  name                  = "dev-center-neg"
  network_endpoint_type = "SERVERLESS"
  region                = var.region
  cloud_run { service = google_cloud_run_v2_service.crate_processor.name }
}

resource "google_compute_region_network_endpoint_group" "gallery" {
  name                  = "gallery-neg"
  network_endpoint_type = "SERVERLESS"
  region                = var.region
  cloud_run { service = google_cloud_run_v2_service.gallery_api.name }
}

# Backend Services
resource "google_compute_backend_service" "ops_main" {
  name                  = "ops-main-backend"
  protocol              = "HTTPS"
  load_balancing_scheme = "EXTERNAL_MANAGED"
  enable_cdn            = true
  cdn_policy { cache_mode = "CACHE_ALL_STATIC"; default_ttl = 3600 }
  backend { group = google_compute_region_network_endpoint_group.ops_main.id }
}

resource "google_compute_backend_service" "orbital" {
  name                  = "orbital-backend"
  protocol              = "HTTPS"
  load_balancing_scheme = "EXTERNAL_MANAGED"
  timeout_sec           = 10
  backend { group = google_compute_region_network_endpoint_group.orbital.id }
}

resource "google_compute_backend_service" "dev_center" {
  name                  = "dev-center-backend"
  protocol              = "HTTPS"
  load_balancing_scheme = "EXTERNAL_MANAGED"
  timeout_sec           = 60
  backend { group = google_compute_region_network_endpoint_group.dev_center.id }
}

resource "google_compute_backend_service" "gallery" {
  name                  = "gallery-backend"
  protocol              = "HTTPS"
  load_balancing_scheme = "EXTERNAL_MANAGED"
  enable_cdn            = true
  cdn_policy { cache_mode = "CACHE_ALL_STATIC"; default_ttl = 86400 }
  backend { group = google_compute_region_network_endpoint_group.gallery.id }
}

# Backend Bucket for Cesium
resource "google_compute_backend_bucket" "cesium" {
  name        = "cesium-frontend-backend"
  bucket_name = google_storage_bucket.cesium_frontend.name
  enable_cdn  = true
}

# URL Map
resource "google_compute_url_map" "sx9" {
  name            = "sx9-url-map"
  default_service = google_compute_backend_service.ops_main.id

  host_rule {
    hosts        = ["ops.${var.domain}"]
    path_matcher = "ops"
  }
  host_rule {
    hosts        = ["orbital.${var.domain}"]
    path_matcher = "orbital"
  }
  host_rule {
    hosts        = ["dev.${var.domain}"]
    path_matcher = "dev"
  }
  host_rule {
    hosts        = ["gallery.${var.domain}"]
    path_matcher = "gallery"
  }

  path_matcher {
    name            = "ops"
    default_service = google_compute_backend_service.ops_main.id
    path_rule {
      paths   = ["/cesium/*", "/static/*"]
      service = google_compute_backend_bucket.cesium.id
    }
  }
  path_matcher {
    name            = "orbital"
    default_service = google_compute_backend_service.orbital.id
  }
  path_matcher {
    name            = "dev"
    default_service = google_compute_backend_service.dev_center.id
  }
  path_matcher {
    name            = "gallery"
    default_service = google_compute_backend_service.gallery.id
  }
}

# SSL Certificate
resource "google_compute_managed_ssl_certificate" "sx9" {
  name = "sx9-ssl-cert"
  managed {
    domains = [
      "ops.${var.domain}",
      "orbital.${var.domain}",
      "dev.${var.domain}",
      "gallery.${var.domain}",
      "api.${var.domain}"
    ]
  }
}

# HTTPS Proxy
resource "google_compute_target_https_proxy" "sx9" {
  name             = "sx9-https-proxy"
  url_map          = google_compute_url_map.sx9.id
  ssl_certificates = [google_compute_managed_ssl_certificate.sx9.id]
}

# Forwarding Rule
resource "google_compute_global_forwarding_rule" "sx9" {
  name                  = "sx9-https-forwarding"
  target                = google_compute_target_https_proxy.sx9.id
  port_range            = "443"
  ip_address            = google_compute_global_address.sx9.address
  load_balancing_scheme = "EXTERNAL_MANAGED"
}

# HTTP Redirect
resource "google_compute_url_map" "redirect" {
  name = "sx9-http-redirect"
  default_url_redirect {
    https_redirect         = true
    redirect_response_code = "MOVED_PERMANENTLY_DEFAULT"
    strip_query            = false
  }
}

resource "google_compute_target_http_proxy" "redirect" {
  name    = "sx9-http-redirect-proxy"
  url_map = google_compute_url_map.redirect.id
}

resource "google_compute_global_forwarding_rule" "redirect" {
  name                  = "sx9-http-redirect"
  target                = google_compute_target_http_proxy.redirect.id
  port_range            = "80"
  ip_address            = google_compute_global_address.sx9.address
  load_balancing_scheme = "EXTERNAL_MANAGED"
}

# ─────────────────────────────────────────────────────────────────────────────────
# CLOUDFLARE MODULE
# ─────────────────────────────────────────────────────────────────────────────────

module "cloudflare" {
  source    = "./modules/cloudflare"
  zone_id   = var.cloudflare_zone_id
  domain    = var.domain
  global_ip = google_compute_global_address.sx9.address
}

# ─────────────────────────────────────────────────────────────────────────────────
# VERTEX AI
# ─────────────────────────────────────────────────────────────────────────────────

resource "google_project_service" "vertex_ai" {
  service            = "aiplatform.googleapis.com"
  disable_on_destroy = false
}
