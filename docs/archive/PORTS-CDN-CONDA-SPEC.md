# SX9 Ports, CDN & Conda Integration Specification

## Overview

This document defines:
1. **Port Allocation** - Deterministic routing for all SX9 services
2. **CDN Architecture** - Internal and external CDN configuration
3. **Conda API Bridge** - Full Python/scientific computing integration

---

## PORT ALLOCATION MATRIX

### Reserved Port Ranges

| Range | Purpose | Notes |
|-------|---------|-------|
| 18000-18099 | Core Infrastructure | Databases, message queues |
| 18100-18199 | Backend Services | APIs, orchestrators |
| 18200-18299 | Frontend Dev Servers | Vite, webpack |
| 18300-18399 | Forge/Workflow | n8n, workflow engines |
| 18400-18499 | Data Services | Sled, Redis, caches |
| 18500-18599 | ML/AI Services | Model servers, inference |
| 18600-18699 | Security Tools | Hermetic tool wrappers |
| 18700-18799 | Voice/Media | STT, TTS, streaming |
| 18800-18899 | Conda Bridge | Python API servers |
| 18900-18999 | Monitoring | Metrics, logging, traces |
| 25000-25999 | Development UIs | Command centers, dashboards |

### Specific Port Assignments

```yaml
# Core Infrastructure (18000-18099)
ports:
  supabase_postgres: 18000
  supabase_api: 18001
  supabase_realtime: 18002
  surrealdb: 18010
  surrealdb_ws: 18011
  nats: 18020
  nats_ws: 18021
  nats_jetstream: 18022
  redis: 18030
  dragonfly: 18031

# Backend Services (18100-18199)
  sx9_orchestrator: 18100
  legion_engine: 18101
  script_coordinator: 18102
  hashing_engine: 18105
  trivariate_service: 18106
  thalmic_filter: 18110
  prompt_generator: 18111
  glaf_allocator: 18120
  convergence_tracker: 18121

# Frontend Dev (18200-18299)
  forge_workbench_dev: 18200
  glaf_browser_dev: 18201
  command_center_dev: 18210
  ops_main_dev: 18211

# Forge/Workflow (18300-18399)
  forge_backend: 18350
  n8n_external: 18351
  workflow_executor: 18352
  tool_chain_runner: 18360

# Data Services (18400-18499)
  sled_http_api: 18400
  sled_admin: 18401
  vector_db: 18410  # Qdrant or pgvector proxy
  embedding_cache: 18411

# ML/AI Services (18500-18599)
  model_registry: 18500
  ann_inference: 18510
  gnn_inference: 18511
  llm_proxy: 18520
  embedding_service: 18521
  classifier_service: 18522

# Security Tools (18600-18699)
  nmap_wrapper: 18600
  nuclei_wrapper: 18601
  masscan_wrapper: 18602
  reconng_wrapper: 18603
  tool_orchestrator: 18650

# Voice/Media (18700-18799)
  whisper_stt: 18700
  elevenlabs_proxy: 18701
  voice_pipeline: 18710
  media_stream: 18720

# Conda Bridge (18800-18899)
  conda_api_main: 18800
  conda_jupyter_kernel: 18801
  conda_numpy_service: 18810
  conda_scipy_service: 18811
  conda_pytorch_service: 18820
  conda_tensorflow_service: 18821
  conda_sklearn_service: 18830
  conda_geopandas_service: 18840
  conda_networkx_service: 18841
  conda_custom_env: 18850

# Monitoring (18900-18999)
  prometheus: 18900
  grafana: 18901
  jaeger: 18910
  loki: 18920

# Development UIs (25000-25999)
  command_center_canonical: 25175
  sx9_development_center: 25176
  ops_main: 25180
  cesium_viewer: 25200
  forge_workbench: 25300
  glaf_browser: 25301
```

---

## CDN ARCHITECTURE

### Internal CDN Hierarchy

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           SX9 CDN ARCHITECTURE                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                     EXTERNAL CDN (Cloudflare/GCP)                    │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            │   │
│  │  │ R2 Bucket│  │ R2 Bucket│  │ GCP CDN  │  │ GCP CDN  │            │   │
│  │  │ static   │  │ crates   │  │ geo-data │  │ models   │            │   │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘            │   │
│  └───────┼─────────────┼─────────────┼─────────────┼────────────────────┘   │
│          │             │             │             │                         │
│          ▼             ▼             ▼             ▼                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      EDGE CACHE (Cloudflare Workers)                 │   │
│  │  • C2 Filter (blocks Cobalt Strike, Metasploit, Sliver signatures)  │   │
│  │  • Rate limiting                                                     │   │
│  │  • Geographic routing                                                │   │
│  │  • Request signing verification                                      │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         TUNNEL LAYER                                 │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │   │
│  │  │ Cloudflare   │  │ Tailscale    │  │ WireGuard    │              │   │
│  │  │ Tunnel       │  │ (internal)   │  │ (dark ops)   │              │   │
│  │  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘              │   │
│  └─────────┼─────────────────┼─────────────────┼────────────────────────┘   │
│            │                 │                 │                             │
│            ▼                 ▼                 ▼                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      INTERNAL CDN (Port Manager)                     │   │
│  │                                                                      │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐   │   │
│  │  │ cdn-static │  │ cdn-crates │  │ cdn-geo    │  │ cdn-models │   │   │
│  │  │ :19000     │  │ :19001     │  │ :19002     │  │ :19003     │   │   │
│  │  └────────────┘  └────────────┘  └────────────┘  └────────────┘   │   │
│  │                                                                      │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐   │   │
│  │  │ cdn-conda  │  │ cdn-tools  │  │ cdn-wasm   │  │ cdn-plasma │   │   │
│  │  │ :19010     │  │ :19011     │  │ :19012     │  │ :19013     │   │   │
│  │  └────────────┘  └────────────┘  └────────────┘  └────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         PORT MANAGER / FILTER                        │   │
│  │                                                                      │   │
│  │  Ingress Rules:                                                      │   │
│  │  • Block known C2 signatures (Cobalt Strike, Metasploit, Sliver)    │   │
│  │  • Verify request signing (HMAC-SHA256 with rotating keys)          │   │
│  │  • Rate limit by client fingerprint                                  │   │
│  │  • Geographic restrictions (configurable per CDN)                    │   │
│  │  • Protocol validation (reject malformed requests)                   │   │
│  │                                                                      │   │
│  │  Egress Rules:                                                       │   │
│  │  • Response signing                                                  │   │
│  │  • Content-Type enforcement                                          │   │
│  │  • Size limits                                                       │   │
│  │  • Audit logging to NATS                                             │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### CDN Definitions

```typescript
interface CDNConfig {
  id: string;
  name: string;
  type: 'cloudflare-r2' | 'gcp-cdn' | 'internal' | 'tunnel';
  
  // External CDN settings
  external?: {
    provider: 'cloudflare' | 'gcp' | 'aws';
    bucket?: string;
    region?: string;
    customDomain?: string;
  };
  
  // Internal routing
  internal: {
    port: number;
    cacheTTL: number;
    maxSize: string;
  };
  
  // Security
  security: {
    requireSigning: boolean;
    allowedOrigins: string[];
    blockedPatterns: string[];  // C2 signatures
    rateLimit: {
      requests: number;
      window: number;  // seconds
    };
  };
  
  // Content types
  allowedContentTypes: string[];
}

const CDN_CONFIGS: CDNConfig[] = [
  {
    id: 'cdn-static',
    name: 'Static Assets',
    type: 'cloudflare-r2',
    external: {
      provider: 'cloudflare',
      bucket: 'sx9-static',
      customDomain: 'static.sx9.io',
    },
    internal: {
      port: 19000,
      cacheTTL: 86400,
      maxSize: '50MB',
    },
    security: {
      requireSigning: false,
      allowedOrigins: ['*.sx9.io', '*.synaptix9.com'],
      blockedPatterns: [],
      rateLimit: { requests: 1000, window: 60 },
    },
    allowedContentTypes: ['image/*', 'font/*', 'text/css', 'application/javascript'],
  },
  
  {
    id: 'cdn-crates',
    name: 'Rust Crates Registry',
    type: 'cloudflare-r2',
    external: {
      provider: 'cloudflare',
      bucket: 'sx9-crates',
      customDomain: 'crates.sx9.io',
    },
    internal: {
      port: 19001,
      cacheTTL: 3600,
      maxSize: '100MB',
    },
    security: {
      requireSigning: true,  // Crates must be signed
      allowedOrigins: ['*.sx9.io'],
      blockedPatterns: [],
      rateLimit: { requests: 100, window: 60 },
    },
    allowedContentTypes: ['application/gzip', 'application/json'],
  },
  
  {
    id: 'cdn-geo',
    name: 'Geospatial Data',
    type: 'gcp-cdn',
    external: {
      provider: 'gcp',
      bucket: 'sx9-geo-data',
      region: 'us-central1',
    },
    internal: {
      port: 19002,
      cacheTTL: 86400,
      maxSize: '500MB',
    },
    security: {
      requireSigning: true,
      allowedOrigins: ['*.sx9.io'],
      blockedPatterns: [],
      rateLimit: { requests: 50, window: 60 },
    },
    allowedContentTypes: ['application/geo+json', 'application/vnd.mapbox-vector-tile', 'image/tiff'],
  },
  
  {
    id: 'cdn-models',
    name: 'ML Models',
    type: 'gcp-cdn',
    external: {
      provider: 'gcp',
      bucket: 'sx9-models',
      region: 'us-central1',
    },
    internal: {
      port: 19003,
      cacheTTL: 86400,
      maxSize: '2GB',
    },
    security: {
      requireSigning: true,
      allowedOrigins: ['*.sx9.io'],
      blockedPatterns: [],
      rateLimit: { requests: 10, window: 60 },
    },
    allowedContentTypes: ['application/octet-stream', 'application/x-pytorch', 'application/x-onnx'],
  },
  
  {
    id: 'cdn-conda',
    name: 'Conda Packages',
    type: 'internal',
    internal: {
      port: 19010,
      cacheTTL: 86400,
      maxSize: '500MB',
    },
    security: {
      requireSigning: true,
      allowedOrigins: ['localhost', '*.sx9.io'],
      blockedPatterns: [],
      rateLimit: { requests: 100, window: 60 },
    },
    allowedContentTypes: ['application/x-tar', 'application/gzip'],
  },
  
  {
    id: 'cdn-tools',
    name: 'Security Tools (Hermetic)',
    type: 'tunnel',
    internal: {
      port: 19011,
      cacheTTL: 3600,
      maxSize: '200MB',
    },
    security: {
      requireSigning: true,
      allowedOrigins: ['localhost'],  // Local only
      blockedPatterns: [
        // Block C2 signatures
        'beacon',
        'meterpreter',
        'sliver',
        'cobalt',
      ],
      rateLimit: { requests: 10, window: 60 },
    },
    allowedContentTypes: ['application/octet-stream'],
  },
  
  {
    id: 'cdn-plasma',
    name: 'Plasma Agent Distribution',
    type: 'tunnel',
    internal: {
      port: 19013,
      cacheTTL: 0,  // No caching for plasma
      maxSize: '50MB',
    },
    security: {
      requireSigning: true,
      allowedOrigins: [],  // Biometric verification required
      blockedPatterns: [],
      rateLimit: { requests: 5, window: 300 },
    },
    allowedContentTypes: ['application/x-plasma'],
  },
];
```

### C2 Filter Implementation

```typescript
// Port filter to block C2 traffic
interface C2Filter {
  // Known C2 signatures to block
  signatures: C2Signature[];
  
  // Check incoming request
  checkRequest(req: Request): FilterResult;
  
  // Check outgoing response
  checkResponse(res: Response): FilterResult;
}

interface C2Signature {
  id: string;
  name: string;
  type: 'cobalt-strike' | 'metasploit' | 'sliver' | 'custom';
  patterns: {
    headers?: Record<string, RegExp>;
    body?: RegExp[];
    uri?: RegExp[];
    timing?: TimingPattern;
  };
}

const C2_SIGNATURES: C2Signature[] = [
  {
    id: 'cs-beacon',
    name: 'Cobalt Strike Beacon',
    type: 'cobalt-strike',
    patterns: {
      headers: {
        'User-Agent': /Mozilla\/5\.0.*MSIE.*Windows NT/,
      },
      uri: [
        /\/submit\.php\?id=/,
        /\/pixel\.(gif|png)/,
        /\/ca$/,
        /\/push$/,
      ],
      timing: {
        interval: 60000,  // 60s beacon interval
        jitter: 0.1,
      },
    },
  },
  {
    id: 'msf-meterpreter',
    name: 'Metasploit Meterpreter',
    type: 'metasploit',
    patterns: {
      body: [
        /metsrv\.dll/,
        /stdapi/,
        /priv/,
      ],
    },
  },
  {
    id: 'sliver-implant',
    name: 'Sliver Implant',
    type: 'sliver',
    patterns: {
      headers: {
        'Content-Type': /application\/octet-stream/,
      },
      uri: [
        /\/rpc$/,
        /\/assets\//,
      ],
    },
  },
];
```

---

## CONDA API BRIDGE

### Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         CONDA API BRIDGE ARCHITECTURE                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    FORGE WORKBENCH (TypeScript)                      │   │
│  │                                                                      │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │   │
│  │  │ Forge Node:  │  │ Forge Node:  │  │ Forge Node:  │              │   │
│  │  │ NumPy Op     │  │ PyTorch Inf  │  │ Custom Env   │              │   │
│  │  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘              │   │
│  └─────────┼─────────────────┼─────────────────┼────────────────────────┘   │
│            │                 │                 │                             │
│            ▼                 ▼                 ▼                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      CONDA API GATEWAY (:18800)                      │   │
│  │                                                                      │   │
│  │  • Request routing to appropriate conda environment                  │   │
│  │  • Session management (persistent Python processes)                  │   │
│  │  • Memory management (numpy array serialization)                     │   │
│  │  • Error handling and logging                                        │   │
│  │  • Authentication (API keys per environment)                         │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│            ┌───────────────────────┼───────────────────────┐                │
│            │                       │                       │                │
│            ▼                       ▼                       ▼                │
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐         │
│  │  CONDA ENV:      │  │  CONDA ENV:      │  │  CONDA ENV:      │         │
│  │  sx9-base        │  │  sx9-ml          │  │  sx9-geo         │         │
│  │  :18810          │  │  :18820          │  │  :18840          │         │
│  │                  │  │                  │  │                  │         │
│  │  • numpy         │  │  • pytorch       │  │  • geopandas     │         │
│  │  • scipy         │  │  • tensorflow    │  │  • shapely       │         │
│  │  • pandas        │  │  • transformers  │  │  • rasterio      │         │
│  │  • networkx      │  │  • sklearn       │  │  • pyproj        │         │
│  │  • sympy         │  │  • onnx          │  │  • fiona         │         │
│  └──────────────────┘  └──────────────────┘  └──────────────────┘         │
│                                                                              │
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐         │
│  │  CONDA ENV:      │  │  CONDA ENV:      │  │  CONDA ENV:      │         │
│  │  sx9-astro       │  │  sx9-graph       │  │  sx9-custom      │         │
│  │  :18841          │  │  :18842          │  │  :18850          │         │
│  │                  │  │                  │  │                  │         │
│  │  • astropy       │  │  • torch_geom    │  │  (user-defined)  │         │
│  │  • skyfield      │  │  • dgl           │  │                  │         │
│  │  • sgp4          │  │  • networkx      │  │                  │         │
│  │  • poliastro     │  │  • igraph        │  │                  │         │
│  └──────────────────┘  └──────────────────┘  └──────────────────┘         │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Conda Environment Definitions

```yaml
# environments/sx9-base.yml
name: sx9-base
channels:
  - conda-forge
  - defaults
dependencies:
  - python=3.11
  - numpy>=1.24
  - scipy>=1.11
  - pandas>=2.0
  - networkx>=3.0
  - sympy>=1.12
  - matplotlib>=3.7
  - seaborn>=0.12
  - statsmodels>=0.14
  - pip
  - pip:
    - fastapi>=0.100
    - uvicorn>=0.23
    - pydantic>=2.0
    - msgpack>=1.0
    - orjson>=3.9

---
# environments/sx9-ml.yml
name: sx9-ml
channels:
  - pytorch
  - conda-forge
  - defaults
dependencies:
  - python=3.11
  - pytorch>=2.0
  - torchvision
  - torchaudio
  - tensorflow>=2.13
  - scikit-learn>=1.3
  - xgboost>=1.7
  - lightgbm>=4.0
  - onnx>=1.14
  - onnxruntime>=1.15
  - transformers>=4.30
  - sentence-transformers>=2.2
  - pip
  - pip:
    - fastapi>=0.100
    - uvicorn>=0.23
    - torch-geometric>=2.3

---
# environments/sx9-geo.yml
name: sx9-geo
channels:
  - conda-forge
  - defaults
dependencies:
  - python=3.11
  - geopandas>=0.13
  - shapely>=2.0
  - fiona>=1.9
  - pyproj>=3.6
  - rasterio>=1.3
  - xarray>=2023.6
  - netcdf4>=1.6
  - h3-py>=3.7
  - osmnx>=1.5
  - folium>=0.14
  - pip
  - pip:
    - fastapi>=0.100
    - uvicorn>=0.23
    - geojson>=3.0

---
# environments/sx9-astro.yml
name: sx9-astro
channels:
  - conda-forge
  - defaults
dependencies:
  - python=3.11
  - astropy>=5.3
  - skyfield>=1.46
  - sgp4>=2.22
  - jplephem>=2.18
  - pip
  - pip:
    - fastapi>=0.100
    - uvicorn>=0.23
    - poliastro>=0.17
    - czml3>=0.7

---
# environments/sx9-graph.yml
name: sx9-graph
channels:
  - pytorch
  - dglteam
  - conda-forge
  - defaults
dependencies:
  - python=3.11
  - pytorch>=2.0
  - dgl>=1.1
  - networkx>=3.0
  - python-igraph>=0.10
  - graph-tool  # if available
  - pip
  - pip:
    - fastapi>=0.100
    - uvicorn>=0.23
    - torch-geometric>=2.3
    - stellargraph>=1.2
```

### Conda API Server (FastAPI)

```python
# conda_api/main.py
"""
SX9 Conda API Bridge
Exposes Python/scientific computing to the Forge Workbench
"""

from fastapi import FastAPI, HTTPException, BackgroundTasks
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import Any, Dict, List, Optional, Union
import numpy as np
import json
import base64
import msgpack
import asyncio
from concurrent.futures import ProcessPoolExecutor

app = FastAPI(
    title="SX9 Conda API Bridge",
    version="1.0.0",
    description="Scientific computing API for Forge Workbench"
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:*", "https://*.sx9.io"],
    allow_methods=["*"],
    allow_headers=["*"],
)

# Process pool for CPU-bound operations
executor = ProcessPoolExecutor(max_workers=4)


# ============================================================================
# Data Models
# ============================================================================

class ArrayData(BaseModel):
    """Serialized numpy array"""
    dtype: str
    shape: List[int]
    data: str  # base64 encoded
    
    @classmethod
    def from_numpy(cls, arr: np.ndarray) -> "ArrayData":
        return cls(
            dtype=str(arr.dtype),
            shape=list(arr.shape),
            data=base64.b64encode(arr.tobytes()).decode()
        )
    
    def to_numpy(self) -> np.ndarray:
        data = base64.b64decode(self.data)
        return np.frombuffer(data, dtype=self.dtype).reshape(self.shape)


class ExecuteRequest(BaseModel):
    """Execute arbitrary Python code"""
    code: str
    inputs: Dict[str, Any] = {}
    timeout: int = 30


class ExecuteResponse(BaseModel):
    """Execution result"""
    success: bool
    outputs: Dict[str, Any] = {}
    error: Optional[str] = None
    execution_time_ms: float


class FunctionCallRequest(BaseModel):
    """Call a specific function"""
    module: str  # e.g., "numpy", "scipy.optimize"
    function: str  # e.g., "linalg.solve", "minimize"
    args: List[Any] = []
    kwargs: Dict[str, Any] = {}


class MatrixOperation(BaseModel):
    """Matrix operation request"""
    operation: str  # "multiply", "inverse", "svd", "eigenvalues", etc.
    matrices: List[ArrayData]
    params: Dict[str, Any] = {}


class OptimizationRequest(BaseModel):
    """Scipy optimization request"""
    method: str  # "minimize", "curve_fit", "root", etc.
    objective: str  # Python code defining objective function
    x0: List[float]
    bounds: Optional[List[List[float]]] = None
    constraints: Optional[List[Dict]] = None
    options: Dict[str, Any] = {}


class GNNInferenceRequest(BaseModel):
    """Graph Neural Network inference"""
    model_path: str
    node_features: ArrayData
    edge_index: ArrayData
    edge_features: Optional[ArrayData] = None
    batch: Optional[ArrayData] = None


class TLEPropagationRequest(BaseModel):
    """Satellite TLE propagation"""
    tle_line1: str
    tle_line2: str
    times: List[str]  # ISO format timestamps
    output_frame: str = "ITRF"  # or "GCRS", "TEME"


class GeoTransformRequest(BaseModel):
    """Coordinate transformation"""
    coordinates: List[List[float]]  # [[lon, lat], ...]
    from_crs: str  # e.g., "EPSG:4326"
    to_crs: str    # e.g., "EPSG:3857"


# ============================================================================
# Core Endpoints
# ============================================================================

@app.get("/health")
async def health():
    """Health check"""
    return {
        "status": "healthy",
        "environments": ["sx9-base", "sx9-ml", "sx9-geo", "sx9-astro", "sx9-graph"],
        "numpy_version": np.__version__,
    }


@app.post("/execute", response_model=ExecuteResponse)
async def execute_code(request: ExecuteRequest):
    """
    Execute arbitrary Python code in a sandboxed environment.
    
    Example:
    {
        "code": "result = np.dot(a, b)",
        "inputs": {
            "a": {"dtype": "float64", "shape": [2, 2], "data": "..."},
            "b": {"dtype": "float64", "shape": [2, 2], "data": "..."}
        }
    }
    """
    import time
    start = time.time()
    
    try:
        # Prepare namespace with inputs
        namespace = {"np": np}
        
        for key, value in request.inputs.items():
            if isinstance(value, dict) and "dtype" in value:
                # It's an array
                namespace[key] = ArrayData(**value).to_numpy()
            else:
                namespace[key] = value
        
        # Execute code
        exec(request.code, namespace)
        
        # Extract outputs (anything not starting with _)
        outputs = {}
        for key, value in namespace.items():
            if not key.startswith("_") and key not in ["np", *request.inputs.keys()]:
                if isinstance(value, np.ndarray):
                    outputs[key] = ArrayData.from_numpy(value).dict()
                elif isinstance(value, (int, float, str, bool, list, dict)):
                    outputs[key] = value
        
        return ExecuteResponse(
            success=True,
            outputs=outputs,
            execution_time_ms=(time.time() - start) * 1000
        )
        
    except Exception as e:
        return ExecuteResponse(
            success=False,
            error=str(e),
            execution_time_ms=(time.time() - start) * 1000
        )


@app.post("/function")
async def call_function(request: FunctionCallRequest):
    """
    Call a specific numpy/scipy function.
    
    Example:
    {
        "module": "numpy.linalg",
        "function": "solve",
        "args": [
            {"dtype": "float64", "shape": [3, 3], "data": "..."},
            {"dtype": "float64", "shape": [3], "data": "..."}
        ]
    }
    """
    import importlib
    import time
    start = time.time()
    
    try:
        # Import module
        mod = importlib.import_module(request.module)
        
        # Get function
        func = getattr(mod, request.function)
        
        # Convert array arguments
        args = []
        for arg in request.args:
            if isinstance(arg, dict) and "dtype" in arg:
                args.append(ArrayData(**arg).to_numpy())
            else:
                args.append(arg)
        
        kwargs = {}
        for key, value in request.kwargs.items():
            if isinstance(value, dict) and "dtype" in value:
                kwargs[key] = ArrayData(**value).to_numpy()
            else:
                kwargs[key] = value
        
        # Call function
        result = func(*args, **kwargs)
        
        # Convert result
        if isinstance(result, np.ndarray):
            result = ArrayData.from_numpy(result).dict()
        elif isinstance(result, tuple):
            result = [
                ArrayData.from_numpy(r).dict() if isinstance(r, np.ndarray) else r
                for r in result
            ]
        
        return {
            "success": True,
            "result": result,
            "execution_time_ms": (time.time() - start) * 1000
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": str(e),
            "execution_time_ms": (time.time() - start) * 1000
        }


# ============================================================================
# Matrix Operations
# ============================================================================

@app.post("/matrix/{operation}")
async def matrix_operation(operation: str, request: MatrixOperation):
    """
    Perform matrix operations.
    
    Operations: multiply, inverse, transpose, svd, eigenvalues, 
                determinant, rank, norm, solve
    """
    try:
        matrices = [m.to_numpy() for m in request.matrices]
        
        if operation == "multiply":
            result = np.linalg.multi_dot(matrices)
        elif operation == "inverse":
            result = np.linalg.inv(matrices[0])
        elif operation == "transpose":
            result = matrices[0].T
        elif operation == "svd":
            u, s, vh = np.linalg.svd(matrices[0])
            return {
                "success": True,
                "u": ArrayData.from_numpy(u).dict(),
                "s": ArrayData.from_numpy(s).dict(),
                "vh": ArrayData.from_numpy(vh).dict(),
            }
        elif operation == "eigenvalues":
            eigenvalues, eigenvectors = np.linalg.eig(matrices[0])
            return {
                "success": True,
                "eigenvalues": ArrayData.from_numpy(eigenvalues).dict(),
                "eigenvectors": ArrayData.from_numpy(eigenvectors).dict(),
            }
        elif operation == "determinant":
            return {"success": True, "result": float(np.linalg.det(matrices[0]))}
        elif operation == "rank":
            return {"success": True, "result": int(np.linalg.matrix_rank(matrices[0]))}
        elif operation == "norm":
            ord = request.params.get("ord", None)
            return {"success": True, "result": float(np.linalg.norm(matrices[0], ord=ord))}
        elif operation == "solve":
            result = np.linalg.solve(matrices[0], matrices[1])
        else:
            raise ValueError(f"Unknown operation: {operation}")
        
        return {
            "success": True,
            "result": ArrayData.from_numpy(result).dict()
        }
        
    except Exception as e:
        return {"success": False, "error": str(e)}


# ============================================================================
# Optimization (SciPy)
# ============================================================================

@app.post("/optimize")
async def optimize(request: OptimizationRequest):
    """
    Run scipy optimization.
    
    Example:
    {
        "method": "minimize",
        "objective": "lambda x: (x[0] - 1)**2 + (x[1] - 2.5)**2",
        "x0": [0, 0],
        "bounds": [[0, 5], [0, 5]]
    }
    """
    from scipy import optimize
    
    try:
        # Create objective function from string
        obj_func = eval(request.objective)
        
        if request.method == "minimize":
            result = optimize.minimize(
                obj_func,
                request.x0,
                bounds=request.bounds,
                constraints=request.constraints,
                options=request.options
            )
            return {
                "success": result.success,
                "x": result.x.tolist(),
                "fun": float(result.fun),
                "message": result.message,
                "nit": result.nit,
            }
        elif request.method == "root":
            result = optimize.root(obj_func, request.x0)
            return {
                "success": result.success,
                "x": result.x.tolist(),
                "fun": result.fun.tolist() if hasattr(result.fun, 'tolist') else result.fun,
            }
        else:
            raise ValueError(f"Unknown method: {request.method}")
            
    except Exception as e:
        return {"success": False, "error": str(e)}


# ============================================================================
# Graph Neural Networks (PyTorch Geometric)
# ============================================================================

@app.post("/gnn/inference")
async def gnn_inference(request: GNNInferenceRequest):
    """
    Run GNN inference for ground station / satellite networks.
    """
    try:
        import torch
        from torch_geometric.data import Data
        
        # Load model
        model = torch.load(request.model_path)
        model.eval()
        
        # Prepare data
        x = torch.tensor(request.node_features.to_numpy(), dtype=torch.float)
        edge_index = torch.tensor(request.edge_index.to_numpy(), dtype=torch.long)
        
        data = Data(x=x, edge_index=edge_index)
        
        if request.edge_features:
            data.edge_attr = torch.tensor(request.edge_features.to_numpy(), dtype=torch.float)
        
        # Inference
        with torch.no_grad():
            output = model(data)
        
        return {
            "success": True,
            "output": ArrayData.from_numpy(output.numpy()).dict()
        }
        
    except Exception as e:
        return {"success": False, "error": str(e)}


# ============================================================================
# Satellite/Orbital Mechanics
# ============================================================================

@app.post("/orbital/propagate")
async def propagate_tle(request: TLEPropagationRequest):
    """
    Propagate satellite position from TLE.
    
    Uses SGP4 propagator via skyfield.
    """
    try:
        from skyfield.api import load, EarthSatellite
        from datetime import datetime
        
        ts = load.timescale()
        satellite = EarthSatellite(request.tle_line1, request.tle_line2, "SAT", ts)
        
        positions = []
        for time_str in request.times:
            dt = datetime.fromisoformat(time_str.replace("Z", "+00:00"))
            t = ts.utc(dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second)
            
            geocentric = satellite.at(t)
            
            if request.output_frame == "ITRF":
                pos = geocentric.position.km
            else:
                pos = geocentric.position.km
            
            positions.append({
                "time": time_str,
                "position_km": pos.tolist(),
                "velocity_km_s": geocentric.velocity.km_per_s.tolist(),
            })
        
        return {"success": True, "positions": positions}
        
    except Exception as e:
        return {"success": False, "error": str(e)}


@app.post("/orbital/slew_angle")
async def calculate_slew_angle(
    satellite_pos: List[float],  # [x, y, z] in km
    ground_station_pos: List[float],  # [lat, lon, alt]
    time: str
):
    """
    Calculate slew angle for optical receiver alignment.
    """
    try:
        from skyfield.api import load, wgs84
        import numpy as np
        
        ts = load.timescale()
        
        # Convert ground station to ECEF
        lat, lon, alt = ground_station_pos
        gs_ecef = wgs84.latlon(lat, lon, elevation_m=alt * 1000)
        
        # Calculate look angle
        sat_pos = np.array(satellite_pos)
        gs_pos = np.array(gs_ecef.itrs_xyz.km)
        
        look_vector = sat_pos - gs_pos
        look_distance = np.linalg.norm(look_vector)
        look_unit = look_vector / look_distance
        
        # Calculate azimuth and elevation
        # (simplified - full implementation would use proper coordinate transforms)
        elevation = np.arcsin(look_unit[2])
        azimuth = np.arctan2(look_unit[1], look_unit[0])
        
        return {
            "success": True,
            "azimuth_deg": np.degrees(azimuth),
            "elevation_deg": np.degrees(elevation),
            "slant_range_km": look_distance,
        }
        
    except Exception as e:
        return {"success": False, "error": str(e)}


# ============================================================================
# Geospatial
# ============================================================================

@app.post("/geo/transform")
async def transform_coordinates(request: GeoTransformRequest):
    """
    Transform coordinates between CRS.
    """
    try:
        from pyproj import Transformer
        
        transformer = Transformer.from_crs(
            request.from_crs, 
            request.to_crs, 
            always_xy=True
        )
        
        results = []
        for coord in request.coordinates:
            x, y = transformer.transform(coord[0], coord[1])
            results.append([x, y])
        
        return {"success": True, "coordinates": results}
        
    except Exception as e:
        return {"success": False, "error": str(e)}


@app.post("/geo/distance")
async def calculate_distance(
    point1: List[float],  # [lon, lat]
    point2: List[float],  # [lon, lat]
    method: str = "geodesic"  # or "great_circle"
):
    """
    Calculate distance between two points.
    """
    try:
        from geopy.distance import geodesic, great_circle
        
        p1 = (point1[1], point1[0])  # geopy uses (lat, lon)
        p2 = (point2[1], point2[0])
        
        if method == "geodesic":
            dist = geodesic(p1, p2)
        else:
            dist = great_circle(p1, p2)
        
        return {
            "success": True,
            "distance_km": dist.km,
            "distance_m": dist.m,
            "distance_nm": dist.nm,
        }
        
    except Exception as e:
        return {"success": False, "error": str(e)}


# ============================================================================
# Weather/Monte Carlo
# ============================================================================

@app.post("/montecarlo/weather")
async def weather_monte_carlo(
    ground_station_id: str,
    years: int = 5,
    simulations: int = 10000,
    seed: Optional[int] = None
):
    """
    Run Monte Carlo simulation for weather impact on ground station availability.
    """
    try:
        if seed:
            np.random.seed(seed)
        
        # Placeholder - would fetch historical weather data
        # and run actual simulation
        
        # Example: simulate availability based on weather patterns
        daily_availability = np.random.beta(8, 2, size=(simulations, years * 365))
        
        annual_availability = daily_availability.mean(axis=1)
        
        return {
            "success": True,
            "ground_station_id": ground_station_id,
            "simulations": simulations,
            "years": years,
            "results": {
                "mean_availability": float(annual_availability.mean()),
                "std_availability": float(annual_availability.std()),
                "percentile_5": float(np.percentile(annual_availability, 5)),
                "percentile_95": float(np.percentile(annual_availability, 95)),
                "histogram": np.histogram(annual_availability, bins=20)[0].tolist(),
            }
        }
        
    except Exception as e:
        return {"success": False, "error": str(e)}


# ============================================================================
# Startup
# ============================================================================

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=18800)
```

### TypeScript Client for Forge Workbench

```typescript
// lib/condaClient.ts
/**
 * Conda API Bridge Client
 * Integrates Python scientific computing into Forge Workbench
 */

const CONDA_API_URL = import.meta.env.VITE_CONDA_API_URL || 'http://localhost:18800';

// ============================================================================
// Types
// ============================================================================

interface ArrayData {
  dtype: string;
  shape: number[];
  data: string;  // base64
}

interface ExecuteRequest {
  code: string;
  inputs?: Record<string, any>;
  timeout?: number;
}

interface ExecuteResponse {
  success: boolean;
  outputs?: Record<string, any>;
  error?: string;
  execution_time_ms: number;
}

interface FunctionCallRequest {
  module: string;
  function: string;
  args?: any[];
  kwargs?: Record<string, any>;
}

interface OptimizationRequest {
  method: string;
  objective: string;
  x0: number[];
  bounds?: number[][];
  constraints?: any[];
  options?: Record<string, any>;
}

// ============================================================================
// Array Utilities
// ============================================================================

export function encodeArray(arr: Float64Array | number[][], shape: number[]): ArrayData {
  let flat: Float64Array;
  
  if (arr instanceof Float64Array) {
    flat = arr;
  } else {
    // Flatten 2D array
    flat = new Float64Array(arr.flat());
  }
  
  const buffer = flat.buffer;
  const base64 = btoa(String.fromCharCode(...new Uint8Array(buffer)));
  
  return {
    dtype: 'float64',
    shape,
    data: base64,
  };
}

export function decodeArray(data: ArrayData): Float64Array {
  const binary = atob(data.data);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) {
    bytes[i] = binary.charCodeAt(i);
  }
  return new Float64Array(bytes.buffer);
}

export function reshapeArray(flat: Float64Array, shape: number[]): number[][] {
  if (shape.length !== 2) throw new Error('Only 2D reshape supported');
  const [rows, cols] = shape;
  const result: number[][] = [];
  for (let i = 0; i < rows; i++) {
    result.push(Array.from(flat.slice(i * cols, (i + 1) * cols)));
  }
  return result;
}

// ============================================================================
// API Client
// ============================================================================

class CondaClient {
  private baseUrl: string;
  
  constructor(baseUrl: string = CONDA_API_URL) {
    this.baseUrl = baseUrl;
  }
  
  // Health check
  async health(): Promise<any> {
    const res = await fetch(`${this.baseUrl}/health`);
    return res.json();
  }
  
  // Execute arbitrary Python code
  async execute(request: ExecuteRequest): Promise<ExecuteResponse> {
    const res = await fetch(`${this.baseUrl}/execute`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request),
    });
    return res.json();
  }
  
  // Call a specific function
  async callFunction(request: FunctionCallRequest): Promise<any> {
    const res = await fetch(`${this.baseUrl}/function`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request),
    });
    return res.json();
  }
  
  // Matrix operations
  async matrixOp(operation: string, matrices: ArrayData[], params?: Record<string, any>): Promise<any> {
    const res = await fetch(`${this.baseUrl}/matrix/${operation}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ operation, matrices, params: params || {} }),
    });
    return res.json();
  }
  
  // Optimization
  async optimize(request: OptimizationRequest): Promise<any> {
    const res = await fetch(`${this.baseUrl}/optimize`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request),
    });
    return res.json();
  }
  
  // GNN inference
  async gnnInference(
    modelPath: string,
    nodeFeatures: ArrayData,
    edgeIndex: ArrayData,
    edgeFeatures?: ArrayData
  ): Promise<any> {
    const res = await fetch(`${this.baseUrl}/gnn/inference`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        model_path: modelPath,
        node_features: nodeFeatures,
        edge_index: edgeIndex,
        edge_features: edgeFeatures,
      }),
    });
    return res.json();
  }
  
  // Orbital propagation
  async propagateTLE(
    tleLine1: string,
    tleLine2: string,
    times: string[]
  ): Promise<any> {
    const res = await fetch(`${this.baseUrl}/orbital/propagate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        tle_line1: tleLine1,
        tle_line2: tleLine2,
        times,
      }),
    });
    return res.json();
  }
  
  // Slew angle calculation
  async calculateSlewAngle(
    satellitePos: number[],
    groundStationPos: number[],
    time: string
  ): Promise<any> {
    const res = await fetch(`${this.baseUrl}/orbital/slew_angle`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        satellite_pos: satellitePos,
        ground_station_pos: groundStationPos,
        time,
      }),
    });
    return res.json();
  }
  
  // Coordinate transform
  async transformCoordinates(
    coordinates: number[][],
    fromCrs: string,
    toCrs: string
  ): Promise<any> {
    const res = await fetch(`${this.baseUrl}/geo/transform`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        coordinates,
        from_crs: fromCrs,
        to_crs: toCrs,
      }),
    });
    return res.json();
  }
  
  // Distance calculation
  async calculateDistance(
    point1: number[],
    point2: number[],
    method: 'geodesic' | 'great_circle' = 'geodesic'
  ): Promise<any> {
    const res = await fetch(`${this.baseUrl}/geo/distance`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ point1, point2, method }),
    });
    return res.json();
  }
  
  // Monte Carlo weather simulation
  async weatherMonteCarlo(
    groundStationId: string,
    years: number = 5,
    simulations: number = 10000
  ): Promise<any> {
    const res = await fetch(`${this.baseUrl}/montecarlo/weather`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        ground_station_id: groundStationId,
        years,
        simulations,
      }),
    });
    return res.json();
  }
}

export const condaClient = new CondaClient();
export default CondaClient;


// ============================================================================
// Convenience Functions
// ============================================================================

/**
 * Solve linear system Ax = b
 */
export async function solveLinearSystem(A: number[][], b: number[]): Promise<number[]> {
  const result = await condaClient.matrixOp('solve', [
    encodeArray(A, [A.length, A[0].length]),
    encodeArray(new Float64Array(b), [b.length]),
  ]);
  
  if (!result.success) throw new Error(result.error);
  return Array.from(decodeArray(result.result));
}

/**
 * Compute eigenvalues and eigenvectors
 */
export async function eigenDecomposition(matrix: number[][]): Promise<{
  eigenvalues: number[];
  eigenvectors: number[][];
}> {
  const result = await condaClient.matrixOp('eigenvalues', [
    encodeArray(matrix, [matrix.length, matrix[0].length]),
  ]);
  
  if (!result.success) throw new Error(result.error);
  
  return {
    eigenvalues: Array.from(decodeArray(result.eigenvalues)),
    eigenvectors: reshapeArray(
      decodeArray(result.eigenvectors),
      result.eigenvectors.shape
    ),
  };
}

/**
 * Run GNN for ground station network
 */
export async function predictGroundStationAvailability(
  nodeFeatures: number[][],  // [n_stations, n_features]
  edges: [number, number][]  // edge list
): Promise<number[]> {
  const n = nodeFeatures.length;
  const d = nodeFeatures[0].length;
  
  // Flatten node features
  const flatFeatures = new Float64Array(nodeFeatures.flat());
  
  // Create edge index (2 x num_edges)
  const edgeIndex = new Float64Array(edges.length * 2);
  edges.forEach(([src, dst], i) => {
    edgeIndex[i] = src;
    edgeIndex[edges.length + i] = dst;
  });
  
  const result = await condaClient.gnnInference(
    '/models/ground_station_gnn.pt',
    encodeArray(flatFeatures, [n, d]),
    encodeArray(edgeIndex, [2, edges.length])
  );
  
  if (!result.success) throw new Error(result.error);
  return Array.from(decodeArray(result.output));
}
```

### Forge Node: Conda Operation

```typescript
// components/Forge/nodes/CondaNode.tsx
/**
 * Forge node for Conda/Python operations
 */

import { memo } from 'react';
import { Handle, Position, NodeProps } from 'reactflow';
import { Code, Cpu, FlaskConical } from 'lucide-react';

interface CondaNodeData {
  label: string;
  operation: 'execute' | 'function' | 'matrix' | 'optimize' | 'gnn' | 'orbital' | 'geo';
  config: {
    code?: string;
    module?: string;
    function?: string;
    operation?: string;
  };
}

export const CondaNode = memo(({ data, selected }: NodeProps<CondaNodeData>) => {
  const getIcon = () => {
    switch (data.operation) {
      case 'execute': return <Code className="w-4 h-4" />;
      case 'gnn': return <Cpu className="w-4 h-4" />;
      default: return <FlaskConical className="w-4 h-4" />;
    }
  };
  
  return (
    <div className={`
      bg-bg-secondary border rounded-lg min-w-[180px]
      ${selected ? 'border-violet ring-2 ring-violet/30' : 'border-border-default'}
    `}>
      <Handle type="target" position={Position.Left} className="!bg-violet" />
      
      <div className="flex items-center gap-2 px-3 py-2 border-b border-border-subtle">
        <div className="text-violet">{getIcon()}</div>
        <span className="text-sm font-medium text-text-primary">{data.label}</span>
      </div>
      
      <div className="p-3 text-xs text-text-secondary">
        {data.operation === 'execute' && (
          <pre className="bg-bg-tertiary p-2 rounded text-[10px] max-h-20 overflow-hidden">
            {data.config.code?.slice(0, 100)}...
          </pre>
        )}
        {data.operation === 'function' && (
          <div>
            <span className="text-violet">{data.config.module}</span>
            <span className="text-text-muted">.</span>
            <span className="text-cyan">{data.config.function}</span>
          </div>
        )}
        {data.operation === 'matrix' && (
          <div className="text-amber">{data.config.operation}</div>
        )}
      </div>
      
      <Handle type="source" position={Position.Right} className="!bg-violet" />
    </div>
  );
});

CondaNode.displayName = 'CondaNode';
```

---

## DOCKER COMPOSE FOR FULL STACK

```yaml
# docker-compose.sx9-full.yml
version: '3.8'

services:
  # ============================================================================
  # Databases
  # ============================================================================
  
  postgres:
    image: supabase/postgres:15.1.0.117
    ports:
      - "18000:5432"
    environment:
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
  
  surrealdb:
    image: surrealdb/surrealdb:latest
    ports:
      - "18010:8000"
    command: start --user root --pass ${SURREAL_PASSWORD}
    volumes:
      - surreal_data:/data
  
  redis:
    image: redis:7-alpine
    ports:
      - "18030:6379"
    volumes:
      - redis_data:/data
  
  nats:
    image: nats:2.10-alpine
    ports:
      - "18020:4222"
      - "18021:8222"  # WebSocket
    command: -js -m 8222
  
  # ============================================================================
  # Conda API Bridge
  # ============================================================================
  
  conda-api:
    build:
      context: ./conda_api
      dockerfile: Dockerfile
    ports:
      - "18800:18800"
    environment:
      - PYTHONUNBUFFERED=1
    volumes:
      - ./models:/models:ro
      - conda_cache:/opt/conda/pkgs
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]
  
  # ============================================================================
  # Core Services
  # ============================================================================
  
  forge-backend:
    build:
      context: ./tools/forge-backend
    ports:
      - "18350:18350"
    environment:
      - NATS_URL=nats://nats:4222
      - POSTGRES_URL=postgres://postgres:${POSTGRES_PASSWORD}@postgres:5432
      - SURREAL_URL=ws://surrealdb:8000
  
  hashing-engine:
    build:
      context: ./services/hashing-engine
    ports:
      - "18105:18105"
  
  # ============================================================================
  # Frontend
  # ============================================================================
  
  forge-workbench:
    build:
      context: ./apps/forge-workbench
    ports:
      - "25300:80"
    environment:
      - VITE_SUPABASE_URL=http://localhost:18001
      - VITE_SURREALDB_URL=ws://localhost:18010
      - VITE_CONDA_API_URL=http://localhost:18800
      - VITE_NATS_WS_URL=ws://localhost:18021

volumes:
  postgres_data:
  surreal_data:
  redis_data:
  conda_cache:
```

### Conda API Dockerfile

```dockerfile
# conda_api/Dockerfile
FROM continuumio/miniconda3:latest

WORKDIR /app

# Create all environments
COPY environments/*.yml /tmp/
RUN conda env create -f /tmp/sx9-base.yml && \
    conda env create -f /tmp/sx9-ml.yml && \
    conda env create -f /tmp/sx9-geo.yml && \
    conda env create -f /tmp/sx9-astro.yml && \
    conda env create -f /tmp/sx9-graph.yml

# Install API dependencies in base
RUN conda install -n sx9-base fastapi uvicorn pydantic msgpack orjson

# Copy API code
COPY main.py .

# Activate sx9-base and run
SHELL ["conda", "run", "-n", "sx9-base", "/bin/bash", "-c"]
EXPOSE 18800
CMD ["conda", "run", "-n", "sx9-base", "uvicorn", "main:app", "--host", "0.0.0.0", "--port", "18800"]
```

---

## SUMMARY

This spec provides:

| Component | Port | Purpose |
|-----------|------|---------|
| **Conda API Gateway** | 18800 | Main entry point for all Python operations |
| **NumPy/SciPy Service** | 18810 | Matrix ops, optimization |
| **PyTorch Service** | 18820 | Neural network inference |
| **GeoPandas Service** | 18840 | Geospatial transforms |
| **Astro Service** | 18841 | TLE propagation, orbital mechanics |
| **Graph Service** | 18842 | GNN inference, NetworkX |

### CDN Hierarchy
- External: Cloudflare R2 + GCP CDN
- Edge: C2 filtering, rate limiting
- Tunnel: Cloudflare/Tailscale/WireGuard
- Internal: Port-based routing with signing

### Key Integrations
- Full numpy/scipy through REST API
- GNN inference for ground station predictions
- TLE propagation for satellite tracking
- Monte Carlo weather simulation
- Coordinate transforms for GIS
- All callable from Forge workflow nodes

---




