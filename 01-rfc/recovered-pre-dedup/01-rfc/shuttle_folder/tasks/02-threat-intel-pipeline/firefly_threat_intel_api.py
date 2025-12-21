#!/usr/bin/env python3
"""
Firefly IAC - Threat Intelligence Pipeline API
===============================================

REST API to drive the threat intel pipeline from UI → straight into Neon.

Pipeline Phases:
    1. Download from 27 sources (optional, skip with skip_download)
    2. Normalize with RFC-9001 hashes
    3. Convert to RFC-9005 unified entities
    4. Load directly into Neon PostgreSQL

Endpoints:
    POST /api/firefly/threat-intel/start     - Start pipeline
    POST /api/firefly/threat-intel/pause/:id - Pause pipeline  
    GET  /api/firefly/threat-intel/status/:id - Get status
    GET  /api/firefly/threat-intel/neon-stats - Get Neon DB stats

Environment:
    NEON_DATABASE_URL or DATABASE_URL - Neon connection string

Usage:
    python3 firefly_threat_intel_api.py
"""

import os
import sys
import json
import asyncio
import subprocess
import uuid
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Any
from dataclasses import dataclass, asdict, field
from enum import Enum

try:
    from fastapi import FastAPI, HTTPException, BackgroundTasks, Query
    from fastapi.middleware.cors import CORSMiddleware
    from pydantic import BaseModel
except ImportError:
    print("❌ FastAPI not installed. pip install fastapi uvicorn")
    sys.exit(1)

try:
    import psycopg2
    from psycopg2.extras import RealDictCursor
    HAS_PSYCOPG2 = True
except ImportError:
    HAS_PSYCOPG2 = False

# ═══════════════════════════════════════════════════════════════════════════
# Configuration
# ═══════════════════════════════════════════════════════════════════════════

SCRIPT_DIR = Path(__file__).parent
PIPELINE_SCRIPT = SCRIPT_DIR / "normalize_threat_intel.py"
CONVERTER_SCRIPT = SCRIPT_DIR / "rfc9005_converter.py"
SCHEMA_FILE = SCRIPT_DIR.parent.parent.parent / "neon" / "schema_rfc9005.sql"
OUTPUT_DIR = SCRIPT_DIR / "normalized"
NEON_DIR = SCRIPT_DIR / "neon_ready"
DOWNLOAD_DIR = SCRIPT_DIR / "node-interview-generator" / "output" / "threat_content"

DATABASE_URL = os.environ.get("NEON_DATABASE_URL") or os.environ.get("DATABASE_URL")

# ═══════════════════════════════════════════════════════════════════════════
# Models
# ═══════════════════════════════════════════════════════════════════════════

class PipelineStatus(str, Enum):
    IDLE = "idle"
    NORMALIZING = "normalizing"
    CONVERTING = "converting"
    LOADING = "loading"
    COMPLETE = "complete"
    ERROR = "error"

@dataclass
class ThreatSource:
    id: str
    name: str
    status: str = "pending"
    progress: int = 0
    itemCount: int = 0

@dataclass
class PipelineJob:
    id: str
    status: PipelineStatus = PipelineStatus.IDLE
    phase: str = "idle"
    currentSource: Optional[str] = None
    overallProgress: int = 0
    startTime: Optional[str] = None
    stats: Dict = field(default_factory=dict)
    neonStats: Dict = field(default_factory=dict)

DEFAULT_SOURCES = [
    ThreatSource("mitre-attack", "MITRE ATT&CK"),
    ThreatSource("atomic-red", "Atomic Red Team"),
    ThreatSource("nuclei", "Nuclei Templates"),
    ThreatSource("sigma", "Sigma Rules"),
    ThreatSource("kali", "Kali Tools"),
    ThreatSource("lolbas", "LOLBAS"),
]

jobs: Dict[str, PipelineJob] = {}

# ═══════════════════════════════════════════════════════════════════════════
# Database Functions
# ═══════════════════════════════════════════════════════════════════════════

def get_db_connection():
    if not DATABASE_URL:
        raise Exception("DATABASE_URL not set")
    if not HAS_PSYCOPG2:
        raise Exception("psycopg2 not installed")
    return psycopg2.connect(DATABASE_URL)

def ensure_schema():
    """Ensure RFC-9005 schema exists."""
    if not SCHEMA_FILE.exists():
        print(f"⚠️ Schema not found: {SCHEMA_FILE}")
        return False
    
    conn = get_db_connection()
    cur = conn.cursor()
    
    cur.execute("""
        SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_name = 'entities'
        )
    """)
    exists = cur.fetchone()[0]
    
    if not exists:
        print("📦 Creating schema...")
        with open(SCHEMA_FILE) as f:
            cur.execute(f.read())
        conn.commit()
        print("✅ Schema created")
    
    cur.close()
    conn.close()
    return True

def load_to_neon(seed_file: Path) -> Dict[str, int]:
    """Load seed SQL into Neon."""
    conn = get_db_connection()
    cur = conn.cursor()
    
    with open(seed_file) as f:
        cur.execute(f.read())
    conn.commit()
    
    cur.execute("SELECT COUNT(*) FROM entities WHERE entity_type = 'tool'")
    tools = cur.fetchone()[0]
    
    cur.execute("SELECT COUNT(*) FROM entities WHERE entity_type = 'technique'")
    techniques = cur.fetchone()[0]
    
    cur.execute("SELECT COUNT(*) FROM relationships")
    relationships = cur.fetchone()[0]
    
    cur.close()
    conn.close()
    
    return {"tools": tools, "techniques": techniques, "relationships": relationships}

def get_neon_stats() -> Dict:
    """Get stats from Neon."""
    try:
        conn = get_db_connection()
        cur = conn.cursor()
        
        stats = {}
        cur.execute("SELECT COUNT(*) FROM entities WHERE entity_type = 'tool'")
        stats["tools"] = cur.fetchone()[0]
        
        cur.execute("SELECT COUNT(*) FROM entities WHERE entity_type = 'technique'")
        stats["techniques"] = cur.fetchone()[0]
        
        cur.execute("SELECT COUNT(*) FROM entities")
        stats["entities"] = cur.fetchone()[0]
        
        cur.execute("SELECT COUNT(*) FROM relationships")
        stats["relationships"] = cur.fetchone()[0]
        
        cur.close()
        conn.close()
        return stats
    except Exception as e:
        return {"error": str(e)}

# ═══════════════════════════════════════════════════════════════════════════
# Pipeline Execution
# ═══════════════════════════════════════════════════════════════════════════

async def run_pipeline(job_id: str):
    """Run full pipeline into Neon."""
    job = jobs[job_id]
    job.startTime = datetime.utcnow().isoformat()
    
    try:
        # Phase 1: Normalize
        job.status = PipelineStatus.NORMALIZING
        job.phase = "Phase 1/3: Normalizing (RFC-9001)"
        job.currentSource = "normalize_threat_intel.py"
        job.overallProgress = 10
        
        OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
        
        proc = subprocess.run(
            [sys.executable, str(PIPELINE_SCRIPT), 
             "--input", str(DOWNLOAD_DIR),
             "--output", str(OUTPUT_DIR)],
            capture_output=True, text=True
        )
        
        if proc.returncode != 0:
            raise Exception(f"Normalizer failed: {proc.stderr[:300]}")
        
        job.overallProgress = 40
        
        # Phase 2: Convert to RFC-9005
        job.status = PipelineStatus.CONVERTING
        job.phase = "Phase 2/3: Converting to RFC-9005"
        job.currentSource = "rfc9005_converter.py"
        job.overallProgress = 50
        
        NEON_DIR.mkdir(parents=True, exist_ok=True)
        
        proc = subprocess.run(
            [sys.executable, str(CONVERTER_SCRIPT),
             "--input", str(OUTPUT_DIR),
             "--output", str(NEON_DIR)],
            capture_output=True, text=True
        )
        
        if proc.returncode != 0:
            raise Exception(f"Converter failed: {proc.stderr[:300]}")
        
        job.overallProgress = 70
        
        # Phase 3: Load to Neon
        job.status = PipelineStatus.LOADING
        job.phase = "Phase 3/3: Loading to Neon"
        job.currentSource = "Connecting..."
        job.overallProgress = 75
        
        if DATABASE_URL and HAS_PSYCOPG2:
            ensure_schema()
            
            job.currentSource = "Loading data..."
            seed_file = NEON_DIR / "neon_seed.sql"
            
            if seed_file.exists():
                neon_counts = load_to_neon(seed_file)
                job.neonStats = neon_counts
                job.currentSource = f"✅ Loaded: {neon_counts['tools']} tools, {neon_counts['techniques']} techniques"
            
            job.overallProgress = 95
            job.neonStats = get_neon_stats()
        else:
            job.currentSource = "⚠️ DATABASE_URL not set"
        
        # Complete
        job.status = PipelineStatus.COMPLETE
        job.phase = "Complete"
        job.overallProgress = 100
        job.currentSource = None
        
        print(f"✅ Pipeline complete! Neon: {job.neonStats}")
        
    except Exception as e:
        job.status = PipelineStatus.ERROR
        job.phase = "Error"
        job.currentSource = str(e)
        print(f"❌ Error: {e}")

# ═══════════════════════════════════════════════════════════════════════════
# FastAPI
# ═══════════════════════════════════════════════════════════════════════════

app = FastAPI(title="Firefly IAC - Threat Intel → Neon", version="2.0.0")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

@app.get("/")
async def root():
    return {"service": "Firefly IAC", "database": "configured" if DATABASE_URL else "not set"}

@app.get("/health")
async def health():
    return {"status": "healthy", "database": bool(DATABASE_URL)}

@app.post("/api/firefly/threat-intel/start")
async def start_pipeline(background_tasks: BackgroundTasks):
    """Start pipeline → Neon."""
    running = [j for j in jobs.values() if j.status in [
        PipelineStatus.NORMALIZING, PipelineStatus.CONVERTING, PipelineStatus.LOADING
    ]]
    if running:
        raise HTTPException(409, "Already running")
    
    job_id = str(uuid.uuid4())[:8]
    jobs[job_id] = PipelineJob(id=job_id)
    
    background_tasks.add_task(run_pipeline, job_id)
    
    return {"jobId": job_id, "message": "Pipeline started → Neon"}

@app.get("/api/firefly/threat-intel/status/{job_id}")
async def get_status(job_id: str):
    if job_id not in jobs:
        raise HTTPException(404, "Not found")
    
    job = jobs[job_id]
    return {
        "state": {
            "status": job.status.value,
            "phase": job.phase,
            "currentSource": job.currentSource,
            "overallProgress": job.overallProgress,
        },
        "stats": job.stats,
        "neonStats": job.neonStats
    }

@app.get("/api/firefly/threat-intel/neon-stats")
async def neon_stats():
    if not DATABASE_URL:
        raise HTTPException(503, "DATABASE_URL not configured")
    return get_neon_stats()

@app.get("/api/firefly/threat-intel/jobs")
async def list_jobs():
    return {"jobs": [{"id": j.id, "status": j.status.value, "progress": j.overallProgress} for j in jobs.values()]}

if __name__ == "__main__":
    import uvicorn
    print("🔥 Firefly IAC - Threat Intel → Neon (port 18300)")
    print(f"   DB: {'✅' if DATABASE_URL else '❌ Set DATABASE_URL'}")
    uvicorn.run(app, host="0.0.0.0", port=18300)
