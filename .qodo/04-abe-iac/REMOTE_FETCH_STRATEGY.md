# Remote Fetch Strategy - Minimal Local Data

## Problem
- External repos (CAR, ATLAS, ENGAGE, etc.) are large (100s of MB each)
- Cloning locally creates ~1-2 GB of data
- Repos change frequently, requiring updates
- Local storage fills up quickly

## Best Practice Solution: Remote Processing + Cloud Storage

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    LOCAL (Minimal)                          │
│  • Scripts (threat_content_fetcher.py)                      │
│  • Config files                                             │
│  • Processed JSON outputs (small, ~200 MB)                  │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ API Calls
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              GCP Cloud Function / Cloud Run                  │
│  • Clones repos to /tmp (ephemeral)                         │
│  • Processes data                                            │
│  • Uploads to GCS / Supabase                                 │
│  • Deletes local clones after processing                     │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ Store Results
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    CLOUD STORAGE                              │
│  • Supabase: Processed JSON (primary)                        │
│  • GCS: Raw repos archive (backup)                           │
│  • CDN: Public access (Cloudflare R2 / GCP CDN)             │
│  • Neo4j: Graph relationships                                │
└─────────────────────────────────────────────────────────────┘
```

## Implementation Options

### Option 1: GCP Cloud Function (Recommended)
**Pros:**
- No local data storage
- Automatic scaling
- Pay-per-use
- Ephemeral storage (/tmp is cleaned after execution)

**Cons:**
- Requires GCP setup
- Cold start latency (~1-2s)

**Cost:** ~$0.01-0.05 per run

### Option 2: GCP Cloud Run
**Pros:**
- Container-based (can use existing Docker setup)
- Better for long-running processes
- Can cache repos in memory

**Cons:**
- Slightly more expensive
- Requires containerization

**Cost:** ~$0.10-0.50 per run

### Option 3: Remote Cache in GCS
**Pros:**
- Keep current local script
- Cache repos in GCS bucket
- Download only when needed
- Clean up local after processing

**Cons:**
- Still requires some local storage temporarily
- More complex sync logic

**Cost:** ~$0.01/month storage + egress

### Option 4: Hybrid (Recommended for Now)
**Current approach + improvements:**
1. **Local:** Keep scripts, configs, processed JSON only
2. **GCS:** Archive raw repos (compressed, ~400 MB)
3. **Supabase:** Processed data (primary source)
4. **CDN:** Public access layer
5. **Cleanup:** Auto-delete local repos after processing

## Recommended Implementation: Hybrid Approach

### Step 1: Update `threat_content_fetcher.py`

```python
# Add GCS upload after processing
def upload_to_gcs(self, bucket_name: str = "sx9-threat-intel"):
    """Upload processed data to GCS, then delete local repos."""
    from google.cloud import storage
    
    client = storage.Client()
    bucket = client.bucket(bucket_name)
    
    # Upload processed JSON
    for json_file in self.cache_dir.glob("*.json"):
        blob = bucket.blob(f"processed/{json_file.name}")
        blob.upload_from_filename(str(json_file))
    
    # Archive and upload raw repos (compressed)
    import tarfile
    archive = self.cache_dir / "repos_archive.tar.gz"
    with tarfile.open(archive, "w:gz") as tar:
        for repo_dir in ["car", "atlas", "engage", ...]:
            if (self.cache_dir / repo_dir).exists():
                tar.add(self.cache_dir / repo_dir, arcname=repo_dir)
    
    blob = bucket.blob("archives/repos_archive.tar.gz")
    blob.upload_from_filename(str(archive))
    
    # Delete local repos (keep processed JSON)
    for repo_dir in ["car", "atlas", "engage", ...]:
        shutil.rmtree(self.cache_dir / repo_dir, ignore_errors=True)
    
    archive.unlink()
```

### Step 2: Add Remote Fetch Mode

```python
def fetch_from_gcs(self, bucket_name: str = "sx9-threat-intel"):
    """Fetch processed data from GCS instead of cloning repos."""
    from google.cloud import storage
    
    client = storage.Client()
    bucket = client.bucket(bucket_name)
    
    # Download processed JSON only
    for blob in bucket.list_blobs(prefix="processed/"):
        local_path = self.cache_dir / blob.name.split("/")[-1]
        blob.download_to_filename(str(local_path))
    
    # Optionally: Download and extract archive if needed
    # (for re-processing or updates)
```

### Step 3: Update Pipeline Script

```bash
# execute_full_pipeline.sh
# After processing:
1. Upload to GCS
2. Upload to Supabase
3. Delete local repos (keep JSON)
4. Report: "Local: 200 MB, Remote: 1.3 GB"
```

## Local Data Footprint

### Before (Current)
```
threat_content/
├── car/              ~50 MB
├── atlas/            ~20 MB
├── engage/           ~15 MB
├── atomic-red-team/  ~200 MB
├── nuclei-templates/ ~100 MB
├── sigma/            ~50 MB
├── yara-rules/       ~30 MB
└── ...               ~500 MB
Total: ~1.0-1.5 GB
```

### After (Optimized)
```
threat_content/
├── mitre_attack.json      ~30 MB
├── mitre_ics.json         ~2 MB
├── mitre_mobile.json      ~3 MB
├── d3fend.json           ~3 MB
├── car_analytics.json    ~5 MB
├── atlas_techniques.json ~2 MB
├── engage_activities.json ~1 MB
├── atomic_tests.json     ~50 MB
├── nuclei_templates.json  ~30 MB
├── sigma_rules.json      ~20 MB
└── crosswalk_index.json  ~5 MB
Total: ~150-200 MB (processed JSON only)
```

**Savings: ~1.0-1.3 GB (85-90% reduction)**

## Cloud Storage Costs

### Supabase (Primary)
- **Storage:** Free tier (500 MB) or $0.021/GB/month
- **Bandwidth:** Free (within limits)
- **Total:** ~$0.01/month for processed data

### GCS (Archive)
- **Storage:** $0.020/GB/month
- **Egress:** $0.12/GB (first 10 GB free)
- **Total:** ~$0.01/month for 500 MB archive

### CDN (Public Access)
- **Cloudflare R2:** $0.015/GB storage, $0 egress
- **GCP CDN:** $0.08/GB egress
- **Total:** ~$0.01/month

**Grand Total: ~$0.03/month for all storage**

## Migration Plan

### Phase 1: Immediate (No Code Changes)
1. ✅ Add `.gitignore` (done)
2. ✅ Remove repos from git tracking (script created)
3. Add cleanup step: Delete repos after processing
4. Keep only processed JSON locally

### Phase 2: GCS Integration (1-2 hours)
1. Add GCS upload to `threat_content_fetcher.py`
2. Archive repos to GCS after processing
3. Delete local repos after upload
4. Update pipeline to use GCS archive for re-processing

### Phase 3: Remote Processing (Optional, Future)
1. Move to Cloud Function / Cloud Run
2. Zero local data (except configs)
3. All processing in cloud

## Recommended: Start with Phase 1 + Phase 2

**Benefits:**
- Minimal code changes
- Immediate 85% storage reduction
- Cloud backup for safety
- Can still process locally if needed
- Easy migration path to Phase 3

**Implementation Time:** ~2 hours



