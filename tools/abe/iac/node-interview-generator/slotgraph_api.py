#!/usr/bin/env python3
"""
SlotGraph API Service - CTAS v7.3.1 Node Interview Graph Queries
=================================================================

Provides REST API for querying node interviews in Neo4j, supporting:
- HD4 phase-based queries
- TTL task label filtering
- GLAF algorithm inputs (convergence, clustering, paths)
- MITRE ATT&CK technique mapping
- Key indicator and interdiction point analysis

Port: 18140 (CTAS slot-graph range)
"""

from fastapi import FastAPI, HTTPException, Query
from fastapi.middleware.cors import CORSMiddleware
from neo4j import GraphDatabase
from typing import List, Optional, Dict, Any
from pydantic import BaseModel
from datetime import datetime
import json

# Neo4j connection
NEO4J_URI = "bolt://localhost:7687"
NEO4J_USER = "neo4j"
NEO4J_PASS = "testpassword123"

app = FastAPI(
    title="SlotGraph API",
    description="CTAS v7.3.1 Node Interview Graph Service",
    version="7.3.1"
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Response models
class NodeInterview(BaseModel):
    task_id: str
    task_seq: int
    task_name: str
    hd4_phase: str
    task_label: str
    is_key_indicator: bool
    is_interdiction_point: bool
    voice: Optional[str] = None
    purpose: Optional[str] = None
    perspective_1n: Optional[str] = None
    perspective_2n: Optional[str] = None
    mitre_tactics: Optional[List[str]] = None
    mitre_techniques: Optional[List[str]] = None
    likelihood: Optional[float] = None
    impact: Optional[float] = None
    detectability: Optional[float] = None

class AttackPath(BaseModel):
    path: List[str]
    phases: List[str]
    total_weight: float
    key_indicators_count: int
    interdiction_points: List[str]

class PhaseStats(BaseModel):
    phase: str
    count: int
    mandatory_count: int
    key_indicators: int
    interdiction_points: int

class ConvergenceResult(BaseModel):
    task_id: str
    task_name: str
    convergence_score: float
    converging_nodes: List[str]
    phase: str

# Database connection
driver = None

def get_driver():
    global driver
    if driver is None:
        driver = GraphDatabase.driver(NEO4J_URI, auth=(NEO4J_USER, NEO4J_PASS))
    return driver

def close_driver():
    global driver
    if driver:
        driver.close()
        driver = None

@app.on_event("shutdown")
async def shutdown():
    close_driver()

# === CORE ENDPOINTS ===

@app.get("/health")
async def health_check():
    """Health check with Neo4j connection status"""
    try:
        with get_driver().session() as session:
            result = session.run("RETURN 1 as status")
            result.single()
        return {"status": "healthy", "neo4j": "connected", "timestamp": datetime.utcnow().isoformat()}
    except Exception as e:
        return {"status": "unhealthy", "neo4j": str(e)}

@app.get("/nodes", response_model=List[NodeInterview])
async def get_all_nodes(
    phase: Optional[str] = Query(None, description="Filter by HD4 phase"),
    label: Optional[str] = Query(None, description="Filter by task label"),
    key_indicators_only: bool = Query(False, description="Only return key indicators"),
    interdiction_only: bool = Query(False, description="Only return interdiction points")
):
    """Get all node interviews with optional filtering"""
    query = "MATCH (n:NodeInterview) WHERE 1=1"
    params = {}

    if phase:
        query += " AND n.hd4_phase = $phase"
        params["phase"] = phase
    if label:
        query += " AND n.task_label = $label"
        params["label"] = label
    if key_indicators_only:
        query += " AND n.is_key_indicator = true"
    if interdiction_only:
        query += " AND n.is_interdiction_point = true"

    query += " RETURN n ORDER BY n.task_seq"

    with get_driver().session() as session:
        result = session.run(query, params)
        nodes = []
        for record in result:
            node = dict(record["n"])
            nodes.append(NodeInterview(**node))
        return nodes

@app.get("/nodes/{task_id}", response_model=NodeInterview)
async def get_node(task_id: str):
    """Get a specific node interview by task_id"""
    query = "MATCH (n:NodeInterview {task_id: $task_id}) RETURN n"

    with get_driver().session() as session:
        result = session.run(query, task_id=task_id)
        record = result.single()
        if not record:
            raise HTTPException(status_code=404, detail=f"Node {task_id} not found")
        return NodeInterview(**dict(record["n"]))

# === HD4 PHASE ENDPOINTS ===

@app.get("/phases/stats", response_model=List[PhaseStats])
async def get_phase_stats():
    """Get statistics for each HD4 phase"""
    query = """
    MATCH (n:NodeInterview)
    RETURN n.hd4_phase as phase,
           count(n) as count,
           sum(CASE WHEN n.task_label = 'mandatory' THEN 1 ELSE 0 END) as mandatory_count,
           sum(CASE WHEN n.is_key_indicator THEN 1 ELSE 0 END) as key_indicators,
           sum(CASE WHEN n.is_interdiction_point THEN 1 ELSE 0 END) as interdiction_points
    ORDER BY CASE n.hd4_phase
        WHEN 'Hunt' THEN 1
        WHEN 'Detect' THEN 2
        WHEN 'Disrupt' THEN 3
        WHEN 'Disable' THEN 4
        WHEN 'Dominate' THEN 5
    END
    """

    with get_driver().session() as session:
        result = session.run(query)
        return [PhaseStats(**dict(record)) for record in result]

@app.get("/phases/{phase}/nodes", response_model=List[NodeInterview])
async def get_phase_nodes(phase: str):
    """Get all nodes in a specific HD4 phase"""
    valid_phases = ["Hunt", "Detect", "Disrupt", "Disable", "Dominate"]
    if phase not in valid_phases:
        raise HTTPException(status_code=400, detail=f"Invalid phase. Must be one of: {valid_phases}")

    query = """
    MATCH (n:NodeInterview {hd4_phase: $phase})
    RETURN n ORDER BY n.task_seq
    """

    with get_driver().session() as session:
        result = session.run(query, phase=phase)
        return [NodeInterview(**dict(record["n"])) for record in result]

# === ATTACK PATH ENDPOINTS (GLAF lstar.learn input) ===

@app.get("/paths/hunt-to-dominate", response_model=List[AttackPath])
async def get_attack_paths(max_hops: int = Query(10, le=15)):
    """Find all attack paths from Hunt phase to Dominate phase"""
    # Neo4j doesn't support parameters in variable-length paths, use f-string
    query = f"""
    MATCH path = (hunt:NodeInterview {{hd4_phase: 'Hunt'}})
                 -[:ENABLES*1..{max_hops}]->
                 (dominate:NodeInterview {{hd4_phase: 'Dominate'}})
    WITH path,
         [n IN nodes(path) | n.task_id] as task_ids,
         [n IN nodes(path) | n.hd4_phase] as phases,
         reduce(w = 0.0, r IN relationships(path) | w + coalesce(r.weight, 1.0)) as total_weight,
         [n IN nodes(path) WHERE n.is_key_indicator | n.task_id] as key_ind,
         [n IN nodes(path) WHERE n.is_interdiction_point | n.task_id] as interdiction
    RETURN task_ids as path, phases, total_weight,
           size(key_ind) as key_indicators_count,
           interdiction as interdiction_points
    ORDER BY length(path), total_weight DESC
    LIMIT 10
    """

    with get_driver().session() as session:
        result = session.run(query)
        return [AttackPath(**dict(record)) for record in result]

@app.get("/paths/from/{task_id}")
async def get_downstream_paths(task_id: str, depth: int = Query(5, le=10)):
    """Get all paths downstream from a specific task"""
    # Neo4j doesn't support parameters in variable-length paths
    query = f"""
    MATCH path = (start:NodeInterview {{task_id: $task_id}})-[:ENABLES*1..{depth}]->(end:NodeInterview)
    RETURN [n IN nodes(path) | {{task_id: n.task_id, task_name: n.task_name, phase: n.hd4_phase}}] as path,
           length(path) as hops
    ORDER BY hops
    """

    with get_driver().session() as session:
        result = session.run(query, task_id=task_id)
        return [dict(record) for record in result]

# === CONVERGENCE ENDPOINTS (GLAF teth.entropy input) ===

@app.get("/convergence/high-risk", response_model=List[ConvergenceResult])
async def get_high_risk_convergence():
    """
    Find nodes where multiple key indicators converge.
    This feeds GLAF teth.entropy() for risk calculation.
    """
    query = """
    MATCH (n:NodeInterview {is_key_indicator: true})
    OPTIONAL MATCH (related:NodeInterview)-[:RELATED_TO|ENABLES*1..2]-(n)
    WHERE related.is_key_indicator = true AND related <> n
    WITH n, collect(DISTINCT related.task_id) as converging
    WHERE size(converging) > 0
    RETURN n.task_id as task_id,
           n.task_name as task_name,
           n.hd4_phase as phase,
           toFloat(size(converging)) / 5.0 as convergence_score,
           converging as converging_nodes
    ORDER BY convergence_score DESC
    """

    with get_driver().session() as session:
        result = session.run(query)
        return [ConvergenceResult(**dict(record)) for record in result]

@app.get("/convergence/calculate")
async def calculate_convergence(active_tasks: str = Query(..., description="Comma-separated task IDs")):
    """
    Calculate convergence score given a set of active indicators.
    Input: comma-separated task_ids that are currently "firing"
    Output: convergence score and predicted next tasks
    """
    task_list = [t.strip() for t in active_tasks.split(",")]

    query = """
    MATCH (active:NodeInterview)
    WHERE active.task_id IN $task_ids
    WITH collect(active) as active_nodes, count(active) as active_count

    // Find enabled tasks not yet active
    UNWIND active_nodes as a
    MATCH (a)-[:ENABLES]->(next:NodeInterview)
    WHERE NOT next.task_id IN $task_ids
    WITH next, active_count, count(DISTINCT a) as enabling_count

    RETURN next.task_id as predicted_task,
           next.task_name as task_name,
           next.hd4_phase as phase,
           next.task_label as priority,
           toFloat(enabling_count) / toFloat(active_count) as probability,
           next.is_key_indicator as is_key_indicator
    ORDER BY probability DESC,
             CASE next.task_label WHEN 'mandatory' THEN 1 WHEN 'desirable' THEN 2 ELSE 3 END
    LIMIT 5
    """

    with get_driver().session() as session:
        result = session.run(query, task_ids=task_list)
        predictions = [dict(record) for record in result]

        # Calculate overall convergence
        convergence_score = len(task_list) / 12.0  # Normalized by total nodes

        return {
            "active_tasks": task_list,
            "active_count": len(task_list),
            "convergence_score": min(convergence_score, 1.0),
            "predictions": predictions,
            "threat_level": "HIGH" if convergence_score > 0.5 else "MEDIUM" if convergence_score > 0.25 else "LOW"
        }

# === MITRE ATT&CK ENDPOINTS ===

@app.get("/mitre/techniques")
async def get_technique_coverage():
    """Get MITRE ATT&CK technique coverage across all nodes"""
    query = """
    MATCH (n:NodeInterview)
    WHERE n.mitre_techniques IS NOT NULL
    UNWIND n.mitre_techniques AS technique
    RETURN technique,
           count(*) as usage_count,
           collect(DISTINCT n.hd4_phase) as phases,
           collect(DISTINCT n.task_id) as tasks
    ORDER BY usage_count DESC
    """

    with get_driver().session() as session:
        result = session.run(query)
        return [dict(record) for record in result]

@app.get("/mitre/by-technique/{technique_id}")
async def get_nodes_by_technique(technique_id: str):
    """Find all nodes that map to a specific MITRE technique"""
    query = """
    MATCH (n:NodeInterview)
    WHERE $technique IN n.mitre_techniques
    RETURN n.task_id, n.task_name, n.hd4_phase, n.voice, n.perspective_1n
    ORDER BY n.task_seq
    """

    with get_driver().session() as session:
        result = session.run(query, technique=technique_id)
        return [dict(record) for record in result]

# === INTERDICTION ANALYSIS ===

@app.get("/interdiction/optimal")
async def get_optimal_interdiction():
    """
    Find optimal interdiction points that block the most attack paths.
    This is critical for defense prioritization.
    """
    query = """
    MATCH (interdict:NodeInterview {is_interdiction_point: true})
    OPTIONAL MATCH path = (upstream:NodeInterview)-[:ENABLES*1..5]->(interdict)-[:ENABLES*1..5]->(downstream:NodeInterview)
    WITH interdict,
         count(DISTINCT upstream) as blocked_upstream,
         count(DISTINCT downstream) as blocked_downstream
    RETURN interdict.task_id as task_id,
           interdict.task_name as task_name,
           interdict.hd4_phase as phase,
           interdict.perspective_1n as defender_action,
           blocked_upstream + blocked_downstream as path_impact,
           blocked_upstream,
           blocked_downstream
    ORDER BY path_impact DESC
    """

    with get_driver().session() as session:
        result = session.run(query)
        return [dict(record) for record in result]

# === GRAPH STATISTICS ===

@app.get("/stats")
async def get_graph_stats():
    """Get overall graph statistics"""
    query = """
    MATCH (n:NodeInterview)
    OPTIONAL MATCH (n)-[r]->()
    WITH count(DISTINCT n) as total_nodes,
         count(r) as total_edges,
         sum(CASE WHEN n.is_key_indicator THEN 1 ELSE 0 END) as key_indicators,
         sum(CASE WHEN n.is_interdiction_point THEN 1 ELSE 0 END) as interdiction_points,
         sum(CASE WHEN n.task_label = 'mandatory' THEN 1 ELSE 0 END) as mandatory_tasks,
         sum(CASE WHEN n.task_label = 'desirable' THEN 1 ELSE 0 END) as desirable_tasks,
         sum(CASE WHEN n.task_label = 'optional' THEN 1 ELSE 0 END) as optional_tasks
    RETURN total_nodes, total_edges, key_indicators, interdiction_points,
           mandatory_tasks, desirable_tasks, optional_tasks
    """

    with get_driver().session() as session:
        result = session.run(query)
        record = result.single()

        # Get phase breakdown
        phase_query = """
        MATCH (n:NodeInterview)
        RETURN n.hd4_phase as phase, count(n) as count
        ORDER BY CASE n.hd4_phase
            WHEN 'Hunt' THEN 1 WHEN 'Detect' THEN 2
            WHEN 'Disrupt' THEN 3 WHEN 'Disable' THEN 4
            WHEN 'Dominate' THEN 5
        END
        """
        phase_result = session.run(phase_query)
        phases = {r["phase"]: r["count"] for r in phase_result}

        return {
            **dict(record),
            "phases": phases,
            "graph_density": dict(record)["total_edges"] / (dict(record)["total_nodes"] ** 2) if dict(record)["total_nodes"] > 0 else 0
        }

# === GLAF EXPORT FORMATS ===

@app.get("/export/glaf-nodes")
async def export_for_glaf():
    """
    Export nodes in format suitable for GLAF algorithm processing.
    Includes risk dimensions for teth.entropy() calculation.
    """
    query = """
    MATCH (n:NodeInterview)
    OPTIONAL MATCH (n)-[:ENABLES]->(downstream)
    OPTIONAL MATCH (upstream)-[:ENABLES]->(n)
    WITH n,
         collect(DISTINCT downstream.task_id) as enables,
         collect(DISTINCT upstream.task_id) as enabled_by
    RETURN n.task_id as task_id,
           n.task_seq as task_seq,
           n.hd4_phase as hd4_phase,
           n.task_label as task_label,
           n.is_key_indicator as is_key_indicator,
           n.is_interdiction_point as is_interdiction_point,
           n.likelihood as likelihood,
           n.impact as impact,
           n.detectability as detectability,
           enables,
           enabled_by,
           n.mitre_techniques as mitre_techniques
    ORDER BY n.task_seq
    """

    with get_driver().session() as session:
        result = session.run(query)
        nodes = []
        for record in result:
            node = dict(record)
            # Calculate composite risk score for GLAF
            likelihood = node.get("likelihood") or 0.5
            impact = node.get("impact") or 0.5
            detectability = node.get("detectability") or 0.5

            # teth.entropy formula approximation
            node["risk_score"] = likelihood * impact * (1 - detectability)
            nodes.append(node)

        return {
            "version": "7.3.1",
            "export_type": "glaf_nodes",
            "timestamp": datetime.utcnow().isoformat(),
            "node_count": len(nodes),
            "nodes": nodes
        }

if __name__ == "__main__":
    import uvicorn
    print("Starting SlotGraph API on port 18140...")
    print("Neo4j Browser: http://localhost:7474")
    print("API Docs: http://localhost:18140/docs")
    uvicorn.run(app, host="0.0.0.0", port=18140)
