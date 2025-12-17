1. Container Overview

Image name (suggested): glaf-intel-engine

Primary binary: glafd (Rust)

Responsibilities:

Hold the authoritative in-memory graph (petgraph).

Persist nodes/edges in sled/SledIS on disk.

Run Nonagon analytics (α semantic, β ops, γ temporal).

Accept USIM/TDO inputs from:

Graph UI (queue)

n8n-clone + Temporal

Agents (via shuttle / ACB)

Expose REST + WebSocket APIs for:

Graph reads/writes

Job submission & results

Streaming updates to the viewer.

Bridge to tool runners (APECS/Legion, Plasma, Kali) via NATS or similar bus.

One container, one process: glafd with pluggable modules.

2. Internal Modules (Rust Crate Layout)
crates/
  glaf-engine/         # core graph + store + analytics
  glaf-api/            # HTTP + WS interfaces
  glaf-tasks/          # job queue + workers
  glaf-nonagon/        # α/β/γ analytics
  glaf-usim/           # USIM/TDO parsing + validation
  glaf-connectors/     # Surreal, Supabase, Wazuh, etc.

2.1 glaf-engine

Uses petgraph for in-memory adjacency.

Uses sled (or SledIS) for on-disk KVS.

Keeps a NodeIndex and EdgeIndex for fast lookups.

Core structs (simplified):

pub struct GlafEngine {
    graph: petgraph::Graph<Node, Edge>,
    store: sled::Db,
    index: Indexes,
}

pub struct Node {
    pub id: NodeId,
    pub label: String,
    pub node_type: String,     // "Satellite", "Database", etc.
    pub realm: Option<String>, // domain tag if needed
    pub properties: HashMap<String, serde_json::Value>,
}

pub struct Edge {
    pub id: EdgeId,
    pub rel_type: String,      // "CONTROLS", "DOWNLINKS", ...
    pub properties: HashMap<String, serde_json::Value>,
}


H1/H2/H3, USIM, etc. are layered via properties for now; we don’t hard-bake them until you’re ready.

2.2 glaf-nonagon

Implements Nonagon analytic primitives (from RFC-9302):

α – semantic features (embeddings / tags)

β – operational features (HD4 / PTCC)

γ – temporal features (time bounds, event density)

Exposed as:

pub fn run_nonagon(
    engine: &GlafEngine,
    node_ids: &[NodeId],
    params: NonagonParams
) -> NonagonResult;


This is what the backend calls when a “temporal / forecast / simulate” job comes in.

2.3 glaf-tasks

Job queue + worker execution:

pub enum JobKind {
    GraphAnalyze,
    OpsEnrich,
    ForecastTemporal,
    Custom(String),
}

pub struct Job {
    pub id: JobId,
    pub kind: JobKind,
    pub node_ids: Vec<NodeId>,
    pub payload: serde_json::Value,  // parameters, USIM, etc.
    pub status: JobStatus,
}


Backed by:

sled or SledIS keyspace: jobs:{id} → job record

Optional bus (e.g., NATS) for multi-process later.

2.4 glaf-usim

Responsible for:

Parsing USIM/TDO JSON from:

agents

n8n-clone

UI

Validating minimal fields (no fancy TOV math yet unless you want it).

Mapping USIM packets to graph mutations or jobs.

3. Data Shapes GLAF Accepts
3.1 Graph Queue (from UI)
POST /api/glaf/queue

{
  "node_ids": ["<uuid>", "<uuid>"],
  "mode": "analyze | task | forecast | simulate",
  "parameters": {}
}


Backend creates a Job, stores it, and returns:

{
  "job_id": "<uuid>",
  "status": "queued"
}

3.2 USIM / TDO (from agents / workflows)
POST /api/glaf/ingest/usim

[
  {
    "usim_id": "UUID",
    "origin_agent": "Natasha",
    "target_node_id": "UUID",
    "operation": "analyze | enrich | task | predict",
    "hash_h1": "string",
    "hash_h2": "string",
    "hash_h3": null,
    "summary": "human-readable line",
    "directive": "update node props / enqueue job / add edge",
    "payload": { "any_tool_result": "..." },
    "created_at": "ISO8601"
  }
]


Backend:

Logs packet

Optionally updates node properties

May spawn a Job depending on operation / directive.

4. External Interfaces (Backend APIs)

All served by glaf-api over HTTP + WebSocket.

4.1 Health
GET /healthz  → 200 OK { "status": "ok" }

4.2 Graph Read
GET /api/v1/graph/nodes
GET /api/v1/graph/nodes/{id}
GET /api/v1/graph/edges
GET /api/v1/graph/edges/{id}


Return minimal viewer-ready JSON:

{
  "id": "UUID",
  "label": "Satellite",
  "node_type": "Satellite",
  "properties": { "realm": "SPACE" }
}

4.3 Graph Mutations

Note: you can limit these to internal use; UI doesn’t have to call them directly.

POST   /api/v1/graph/nodes        # create
PATCH  /api/v1/graph/nodes/{id}   # update props
DELETE /api/v1/graph/nodes/{id}
POST   /api/v1/graph/edges
PATCH  /api/v1/graph/edges/{id}
DELETE /api/v1/graph/edges/{id}

4.4 Job / Processing
POST /api/glaf/queue
GET  /api/glaf/jobs/{job_id}
GET  /api/glaf/jobs/{job_id}/result


Result shape (simplified):

{
  "job_id": "UUID",
  "status": "completed",
  "result": {
    "network": { "nodes": [...], "edges": [...] },
    "metrics": { "score": 0.93 },
    "notes": "short explanation"
  }
}

4.5 WebSocket
WS /ws
  → sends:
     NODE_UPDATED
     EDGE_UPDATED
     JOB_STATUS
     NETWORK_RESULT


This is how the viewer gets live updates without polling.

5. Storage Layout

Using sled/SledIS inside the container:

trees:
  graph:nodes        // id -> Node (bincode/json)
  graph:edges        // id -> Edge
  graph:index:label  // "Satellite" -> [ids]
  jobs               // job_id -> Job
  logs:usim          // usim_id -> raw packet


You can add a snapshot routine:

Periodically serialize petgraph to sled.

On start, hydrate in-memory graph from sled.

6. Integration Points

This backend expects:

Graph UI

Hits /api/v1/graph/* for initial data

Submits queue requests to /api/glaf/queue

Listens on /ws for updates

Agent Layer (Natasha, Kali, etc.)

Writes TDO/USIM to /api/glaf/ingest/usim

n8n-Clone + Temporal

Calls /api/glaf/ingest/usim with workflow outputs

Or /api/glaf/queue to run analytic jobs

Tool World (Kali / Plasma / APECS/Legion)

Optionally integrated via a glaf-tasks worker that:

reads Jobs from sled

calls external tools

writes results back to GLAF

7. “Spin It” Checklist

For the Docker back end of GLAF to be considered “latest and sane”:

 glafd binary with modules: engine + api + tasks + nonagon + usim

 sled DB path configured via GLAF_DB_PATH

 HTTP port exported (GLAF_PORT, e.g., 18019)

 /healthz working

 /api/v1/graph/nodes returns at least a dummy node

 /api/glaf/queue accepts a test job and stores it

 /ws accepts a connection and can send a heartbeat

 Optionally: glaf-tasks worker loop enabled for jobs

You can drop this whole thing into a file like:

docs/GLAF-BACKEND-SPEC-vCurrent.md

and that’s your canonical “latest version” of the GLAF backend until we start going deeper with H1/H2/H3 and full USIM/TOV semantics.

If you want, next step I can sketch:

A Dockerfile for glaf-intel-engine, or

The Rust main.rs skeleton that wires glaf-engine + glaf-api together.

ChatGPT can make mistakes. Check important info.