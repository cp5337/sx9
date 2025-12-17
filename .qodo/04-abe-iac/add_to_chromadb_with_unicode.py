#!/usr/bin/env python3
"""
Add all threat intelligence data to ChromaDB with Unicode operations (RFC-9002, RFC-9012)

WORKFLOW:
1. Load tools, CTAS tasks, PTCCs, tool chains from match_tools_to_ctas_tasks.py output
2. Extract dual-trivariate hashes (RFC-9001)
3. Map hashes to Unicode operations (RFC-9002: U+E000-E9FF)
4. Generate embeddings with Unicode in metadata
5. Add to ChromaDB collections (RFC-9021)

Collections:
- tools: All threat intelligence tools (Kali, ATT&CK, Atomic, Nuclei, etc.)
- ctas_tasks: CTAS tasks (uuid- format)
- ptcc_configs: PTCC configurations
- tool_chains: Derived tool chains
"""

import os
import sys
import json
import hashlib
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Any
from datetime import datetime

# ChromaDB
try:
    import chromadb
    from chromadb.config import Settings
    HAS_CHROMADB = True
except ImportError:
    HAS_CHROMADB = False
    print("⚠️  chromadb not installed. Run: pip install chromadb")

# Sentence transformers for embeddings
try:
    from sentence_transformers import SentenceTransformer
    HAS_SENTENCE_TRANSFORMERS = True
except ImportError:
    HAS_SENTENCE_TRANSFORMERS = False
    print("⚠️  sentence-transformers not installed. Run: pip install sentence-transformers")

# ============================================================================
# CONFIGURATION
# ============================================================================

# Paths
_script_dir = Path(__file__).parent.resolve()
BASE_DIR = _script_dir.parent.resolve()

# Input files (from match_tools_to_ctas_tasks.py)
MATCHING_OUTPUT_DIR = BASE_DIR / "ctas-glaf" / "import"
TOOLS_TASKS_JSON = MATCHING_OUTPUT_DIR / "tools_tasks_matching.json"
TOOLS_TASKS_TOML = MATCHING_OUTPUT_DIR / "tools_tasks_matching.toml"

# ChromaDB path
CHROMADB_DIR = BASE_DIR / "04-abe-iac" / "node-interview-generator" / "output" / "vectors" / "chromadb"
CHROMADB_DIR.mkdir(parents=True, exist_ok=True)

# Embedding model (RFC-9021: all-MiniLM-L6-v2, 384-dim)
EMBEDDING_MODEL = "all-MiniLM-L6-v2"
EMBEDDING_DIM = 384

# Unicode ranges (RFC-9002)
UNICODE_SYSTEM_CONTROLLER_START = 0xE000  # U+E000-E0FF: System Controller
UNICODE_TRIVARIATE_PROCESSOR_START = 0xE100  # U+E100-E1FF: Trivariate Processor (SCH)
UNICODE_CONTEXT_PROCESSOR_START = 0xE200  # U+E200-E2FF: Context Processor (CUID)
UNICODE_INTELLIGENCE_PROCESSOR_START = 0xE300  # U+E300-E3FF: Intelligence Processor
UNICODE_NEURAL_MUX_START = 0xE400  # U+E400-E6FF: Neural Mux ops
UNICODE_KALI_TOOLS_START = 0xE800  # U+E800-E8FF: Kali Tools

# Base96 charset (RFC-9001)
BASE96_CHARSET = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[]^_{|}~`\"'\\"

# ============================================================================
# UNICODE OPERATION MAPPING (RFC-9002)
# ============================================================================

def hash_to_unicode(hash_component: str, component_type: str) -> int:
    """
    Map hash component to Unicode operation (RFC-9002).
    
    Args:
        hash_component: Base96 hash string (16 chars for SCH/CUID/UUID)
        component_type: "SCH", "CUID", "UUID", "TOOL", etc.
    
    Returns:
        Unicode code point (U+E000-E9FF)
    """
    if not hash_component:
        return UNICODE_SYSTEM_CONTROLLER_START
    
    # Convert Base96 hash to integer
    hash_int = 0
    for i, c in enumerate(hash_component[:8]):  # Use first 8 chars
        if c in BASE96_CHARSET:
            hash_int += BASE96_CHARSET.index(c) * (96 ** i)
    
    # Map to Unicode range based on component type
    if component_type == "SCH":
        # U+E100-E1FF: Trivariate Processor (SCH positions 1-16)
        return UNICODE_TRIVARIATE_PROCESSOR_START + (hash_int % 256)
    elif component_type == "CUID":
        # U+E200-E2FF: Context Processor (CUID positions 17-32)
        return UNICODE_CONTEXT_PROCESSOR_START + (hash_int % 256)
    elif component_type == "UUID":
        # U+E000-E0FF: System Controller (UUID positions 33-48)
        return UNICODE_SYSTEM_CONTROLLER_START + (hash_int % 256)
    elif component_type == "TOOL":
        # U+E800-E8FF: Kali Tools
        return UNICODE_KALI_TOOLS_START + (hash_int % 256)
    else:
        # Default: Intelligence Processor (U+E300-E3FF)
        return UNICODE_INTELLIGENCE_PROCESSOR_START + (hash_int % 256)


def extract_trivariate_components(trivariate_hash: str) -> Dict[str, str]:
    """
    Extract SCH, CUID, UUID from dual-trivariate hash (RFC-9001).
    
    Format: [SCH|CUID|UUID] (48 chars total, 16 each)
    Or: [SCH]_[CUID]_[UUID] (with separators)
    """
    if not trivariate_hash:
        return {"SCH": "", "CUID": "", "UUID": ""}
    
    # Handle separator format
    if "_" in trivariate_hash:
        parts = trivariate_hash.split("_")
        return {
            "SCH": parts[0] if len(parts) > 0 else "",
            "CUID": parts[1] if len(parts) > 1 else "",
            "UUID": parts[2] if len(parts) > 2 else "",
        }
    
    # Handle concatenated format (48 chars)
    if len(trivariate_hash) >= 48:
        return {
            "SCH": trivariate_hash[0:16],
            "CUID": trivariate_hash[16:32],
            "UUID": trivariate_hash[32:48],
        }
    
    # Fallback: try to split evenly
    chunk_size = len(trivariate_hash) // 3
    return {
        "SCH": trivariate_hash[0:chunk_size],
        "CUID": trivariate_hash[chunk_size:chunk_size*2],
        "UUID": trivariate_hash[chunk_size*2:],
    }


def generate_unicode_ops_from_hash(trivariate_hash: str, entity_type: str = "TOOL") -> List[int]:
    """
    Generate Unicode operations from trivariate hash (RFC-9002).
    
    Returns list of Unicode code points for embedding metadata.
    """
    components = extract_trivariate_components(trivariate_hash)
    
    unicode_ops = []
    
    # Map each component to Unicode
    if components["SCH"]:
        unicode_ops.append(hash_to_unicode(components["SCH"], "SCH"))
    if components["CUID"]:
        unicode_ops.append(hash_to_unicode(components["CUID"], "CUID"))
    if components["UUID"]:
        unicode_ops.append(hash_to_unicode(components["UUID"], "UUID"))
    
    # Add entity-specific Unicode
    if entity_type == "TOOL":
        unicode_ops.append(hash_to_unicode(trivariate_hash[:8], "TOOL"))
    
    return unicode_ops if unicode_ops else [UNICODE_SYSTEM_CONTROLLER_START]


def unicode_ops_to_string(unicode_ops: List[int]) -> str:
    """Convert Unicode code points to string for metadata."""
    return "".join(chr(op) for op in unicode_ops if 0xE000 <= op <= 0xE9FF)


# ============================================================================
# CHROMADB INTEGRATION
# ============================================================================

class ChromaDBUnicodeLoader:
    """Load threat intelligence data into ChromaDB with Unicode operations."""
    
    def __init__(self):
        self.chroma_client = None
        self.collections = {}
        self.embedding_model = None
        
        if not HAS_CHROMADB:
            raise RuntimeError("ChromaDB not available. Install with: pip install chromadb")
        
        # Initialize ChromaDB client
        self.chroma_client = chromadb.PersistentClient(
            path=str(CHROMADB_DIR),
            settings=Settings(anonymized_telemetry=False)
        )
        print(f"✅ ChromaDB client initialized at: {CHROMADB_DIR}")
        
        # Initialize embedding model
        if HAS_SENTENCE_TRANSFORMERS:
            try:
                self.embedding_model = SentenceTransformer(EMBEDDING_MODEL)
                print(f"✅ Embedding model loaded: {EMBEDDING_MODEL} ({EMBEDDING_DIM} dims)")
            except Exception as e:
                print(f"⚠️  Could not load embedding model: {e}")
                self.embedding_model = None
        else:
            print("⚠️  sentence-transformers not available. Will use ChromaDB's default embeddings.")
        
        # Create/get collections (RFC-9021)
        self.collections = {
            "tools": self.chroma_client.get_or_create_collection(
                name="tools",
                metadata={"hnsw:space": "cosine", "description": "Threat intelligence tools with Unicode ops"}
            ),
            "ctas_tasks": self.chroma_client.get_or_create_collection(
                name="ctas_tasks",
                metadata={"hnsw:space": "cosine", "description": "CTAS tasks (uuid- format) with Unicode ops"}
            ),
            "ptcc_configs": self.chroma_client.get_or_create_collection(
                name="ptcc_configs",
                metadata={"hnsw:space": "cosine", "description": "PTCC configurations with Unicode ops"}
            ),
            "tool_chains": self.chroma_client.get_or_create_collection(
                name="tool_chains",
                metadata={"hnsw:space": "cosine", "description": "Tool chains with Unicode ops"}
            ),
        }
        print(f"✅ Created/accessed {len(self.collections)} ChromaDB collections")
    
    def load_data_from_matching_output(self) -> Dict[str, Any]:
        """Load data from match_tools_to_ctas_tasks.py output."""
        print("\n" + "=" * 70)
        print("Loading Data from Matching Output")
        print("=" * 70)
        
        data = {
            "tools": {},
            "ctas_tasks": [],
            "ptcc_configs": {},
            "tool_chains": {},
        }
        
        # Load from JSON
        if TOOLS_TASKS_JSON.exists():
            print(f"📂 Loading from: {TOOLS_TASKS_JSON}")
            with open(TOOLS_TASKS_JSON, 'r') as f:
                json_data = json.load(f)
                
                # Load tools
                tools_data = json_data.get("tools", {})
                for tool_category, tools in tools_data.items():
                    if isinstance(tools, dict):
                        for tool_id, tool_info in tools.items():
                            data["tools"][tool_id] = tool_info
                print(f"  ✅ Loaded {len(data['tools'])} tools")
                
                # Load CTAS tasks
                data["ctas_tasks"] = json_data.get("ctas_tasks", [])
                print(f"  ✅ Loaded {len(data['ctas_tasks'])} CTAS tasks")
                
                # Load PTCC configs
                data["ptcc_configs"] = json_data.get("ptcc_configurations", {})
                print(f"  ✅ Loaded {len(data['ptcc_configs'])} PTCC configs")
                
                # Load tool chains
                data["tool_chains"] = json_data.get("ptcc_tool_chains", {})
                print(f"  ✅ Loaded {len(data['tool_chains'])} tool chains")
        else:
            print(f"⚠️  Matching output not found: {TOOLS_TASKS_JSON}")
            print("   Run match_tools_to_ctas_tasks.py first")
        
        return data
    
    def add_tools_to_chromadb(self, tools: Dict[str, Any]) -> int:
        """Add tools to ChromaDB with Unicode operations."""
        print("\n" + "=" * 70)
        print("Adding Tools to ChromaDB")
        print("=" * 70)
        
        collection = self.collections["tools"]
        ids = []
        documents = []
        metadatas = []
        embeddings = []
        
        count = 0
        for tool_id, tool_info in tools.items():
            # Extract trivariate hash
            trivariate_primary = tool_info.get("trivariate_primary") or tool_info.get("trivariate", "")
            trivariate_secondary = tool_info.get("trivariate_secondary", "")
            
            # Generate Unicode operations
            unicode_ops_primary = generate_unicode_ops_from_hash(trivariate_primary, "TOOL")
            unicode_ops_secondary = generate_unicode_ops_from_hash(trivariate_secondary, "TOOL") if trivariate_secondary else []
            
            # Create embedding text (include Unicode ops in text for semantic search)
            tool_name = tool_info.get("name", tool_id)
            tool_desc = tool_info.get("description", "")
            tool_type = tool_info.get("type", tool_info.get("source", "unknown"))
            
            unicode_str_primary = unicode_ops_to_string(unicode_ops_primary)
            unicode_str_secondary = unicode_ops_to_string(unicode_ops_secondary) if unicode_ops_secondary else ""
            
            embedding_text = f"Tool: {tool_name}\nType: {tool_type}\nDescription: {tool_desc}\nUnicode: {unicode_str_primary}"
            if unicode_str_secondary:
                embedding_text += f" {unicode_str_secondary}"
            
            # Generate embedding
            if self.embedding_model:
                embedding = self.embedding_model.encode(embedding_text, convert_to_numpy=True).tolist()
            else:
                embedding = None  # ChromaDB will generate
            
            # Create metadata
            metadata = {
                "tool_id": tool_id,
                "name": tool_name[:200],
                "type": tool_type,
                "source": tool_info.get("source", "unknown"),
                "trivariate_primary": trivariate_primary,
                "trivariate_secondary": trivariate_secondary,
                "unicode_ops_primary": ",".join(str(op) for op in unicode_ops_primary),
                "unicode_ops_secondary": ",".join(str(op) for op in unicode_ops_secondary) if unicode_ops_secondary else "",
                "unicode_string_primary": unicode_str_primary,
                "unicode_string_secondary": unicode_str_secondary,
            }
            
            # Add category if available
            if "category" in tool_info:
                metadata["category"] = tool_info["category"]
            
            ids.append(tool_id)
            documents.append(embedding_text)
            metadatas.append(metadata)
            if embedding:
                embeddings.append(embedding)
            
            count += 1
            
            # Batch insert every 1000 items
            if count % 1000 == 0:
                if embeddings:
                    collection.upsert(ids=ids, documents=documents, metadatas=metadatas, embeddings=embeddings)
                else:
                    collection.upsert(ids=ids, documents=documents, metadatas=metadatas)
                print(f"  ✅ Added {count} tools...")
                ids, documents, metadatas, embeddings = [], [], [], []
        
        # Insert remaining
        if ids:
            if embeddings:
                collection.upsert(ids=ids, documents=documents, metadatas=metadatas, embeddings=embeddings)
            else:
                collection.upsert(ids=ids, documents=documents, metadatas=metadatas)
        
        print(f"✅ Added {count} tools to ChromaDB collection 'tools'")
        return count
    
    def add_ctas_tasks_to_chromadb(self, tasks: List[Dict[str, Any]]) -> int:
        """Add CTAS tasks to ChromaDB with Unicode operations."""
        print("\n" + "=" * 70)
        print("Adding CTAS Tasks to ChromaDB")
        print("=" * 70)
        
        collection = self.collections["ctas_tasks"]
        ids = []
        documents = []
        metadatas = []
        embeddings = []
        
        count = 0
        for task in tasks:
            task_id = task.get("hash_id", "")
            if not task_id.startswith("uuid-"):
                continue  # Skip non-CTAS tasks
            
            # Extract trivariate hash (if available)
            trivariate = task.get("trivariate", "")
            
            # Generate Unicode operations
            unicode_ops = generate_unicode_ops_from_hash(trivariate, "TASK")
            unicode_str = unicode_ops_to_string(unicode_ops)
            
            # Create embedding text
            task_name = task.get("task_name", "")
            task_desc = task.get("description", "")
            task_category = task.get("category", "")
            hd4_phase = task.get("hd4_phase", "")
            
            embedding_text = f"CTAS Task: {task_name}\nCategory: {task_category}\nHD4 Phase: {hd4_phase}\nDescription: {task_desc}\nUnicode: {unicode_str}"
            
            # Generate embedding
            if self.embedding_model:
                embedding = self.embedding_model.encode(embedding_text, convert_to_numpy=True).tolist()
            else:
                embedding = None
            
            # Create metadata
            metadata = {
                "task_id": task_id,
                "task_name": task_name[:200],
                "category": task_category,
                "hd4_phase": hd4_phase,
                "primitive_type": task.get("primitive_type", ""),
                "trivariate": trivariate,
                "unicode_ops": ",".join(str(op) for op in unicode_ops),
                "unicode_string": unicode_str,
            }
            
            ids.append(task_id)
            documents.append(embedding_text)
            metadatas.append(metadata)
            if embedding:
                embeddings.append(embedding)
            
            count += 1
            
            # Batch insert every 500 items
            if count % 500 == 0:
                if embeddings:
                    collection.upsert(ids=ids, documents=documents, metadatas=metadatas, embeddings=embeddings)
                else:
                    collection.upsert(ids=ids, documents=documents, metadatas=metadatas)
                print(f"  ✅ Added {count} tasks...")
                ids, documents, metadatas, embeddings = [], [], [], []
        
        # Insert remaining
        if ids:
            if embeddings:
                collection.upsert(ids=ids, documents=documents, metadatas=metadatas, embeddings=embeddings)
            else:
                collection.upsert(ids=ids, documents=documents, metadatas=metadatas)
        
        print(f"✅ Added {count} CTAS tasks to ChromaDB collection 'ctas_tasks'")
        return count
    
    def add_ptcc_configs_to_chromadb(self, ptcc_configs: Dict[str, Any]) -> int:
        """Add PTCC configurations to ChromaDB with Unicode operations."""
        print("\n" + "=" * 70)
        print("Adding PTCC Configurations to ChromaDB")
        print("=" * 70)
        
        collection = self.collections["ptcc_configs"]
        ids = []
        documents = []
        metadatas = []
        embeddings = []
        
        count = 0
        for ptcc_id, config in ptcc_configs.items():
            # Extract trivariate hash (if available)
            trivariate = config.get("trivariate", "")
            
            # Generate Unicode operations
            unicode_ops = generate_unicode_ops_from_hash(trivariate, "PTCC")
            unicode_str = unicode_ops_to_string(unicode_ops)
            
            # Create embedding text
            operator = config.get("operator", "")
            tool = config.get("tool", "")
            skill_level = config.get("skill_level", 0.0)
            region = config.get("region", "")
            hd4_phase = config.get("recommended_hd4_phase", "")
            
            embedding_text = f"PTCC: {ptcc_id}\nOperator: {operator}\nTool: {tool}\nSkill Level: {skill_level}\nRegion: {region}\nHD4 Phase: {hd4_phase}\nUnicode: {unicode_str}"
            
            # Generate embedding
            if self.embedding_model:
                embedding = self.embedding_model.encode(embedding_text, convert_to_numpy=True).tolist()
            else:
                embedding = None
            
            # Create metadata
            metadata = {
                "ptcc_id": ptcc_id,
                "operator": operator,
                "tool": tool,
                "skill_level": skill_level,
                "region": region,
                "hd4_phase": hd4_phase,
                "trivariate": trivariate,
                "unicode_ops": ",".join(str(op) for op in unicode_ops),
                "unicode_string": unicode_str,
            }
            
            ids.append(ptcc_id)
            documents.append(embedding_text)
            metadatas.append(metadata)
            if embedding:
                embeddings.append(embedding)
            
            count += 1
            
            # Batch insert every 1000 items
            if count % 1000 == 0:
                if embeddings:
                    collection.upsert(ids=ids, documents=documents, metadatas=metadatas, embeddings=embeddings)
                else:
                    collection.upsert(ids=ids, documents=documents, metadatas=metadatas)
                print(f"  ✅ Added {count} PTCC configs...")
                ids, documents, metadatas, embeddings = [], [], [], []
        
        # Insert remaining
        if ids:
            if embeddings:
                collection.upsert(ids=ids, documents=documents, metadatas=metadatas, embeddings=embeddings)
            else:
                collection.upsert(ids=ids, documents=documents, metadatas=metadatas)
        
        print(f"✅ Added {count} PTCC configs to ChromaDB collection 'ptcc_configs'")
        return count
    
    def add_tool_chains_to_chromadb(self, tool_chains: Dict[str, Any]) -> int:
        """Add tool chains to ChromaDB with Unicode operations."""
        print("\n" + "=" * 70)
        print("Adding Tool Chains to ChromaDB")
        print("=" * 70)
        
        collection = self.collections["tool_chains"]
        ids = []
        documents = []
        metadatas = []
        embeddings = []
        
        count = 0
        for chain_id, chain_info in tool_chains.items():
            # Extract trivariate hash (if available)
            trivariate = chain_info.get("trivariate", "")
            
            # Generate Unicode operations
            unicode_ops = generate_unicode_ops_from_hash(trivariate, "CHAIN")
            unicode_str = unicode_ops_to_string(unicode_ops)
            
            # Create embedding text
            chain_name = chain_info.get("name", chain_id)
            chain_desc = chain_info.get("description", "")
            chain_type = chain_info.get("type", "")
            tools = chain_info.get("tools", [])
            hd4_phases = chain_info.get("hd4_phases", [])
            
            embedding_text = f"Tool Chain: {chain_name}\nType: {chain_type}\nDescription: {chain_desc}\nTools: {', '.join(tools[:10])}\nHD4 Phases: {', '.join(hd4_phases)}\nUnicode: {unicode_str}"
            
            # Generate embedding
            if self.embedding_model:
                embedding = self.embedding_model.encode(embedding_text, convert_to_numpy=True).tolist()
            else:
                embedding = None
            
            # Create metadata
            metadata = {
                "chain_id": chain_id,
                "name": chain_name[:200],
                "type": chain_type,
                "tools": "|".join(tools[:20]),  # Limit for metadata
                "hd4_phases": "|".join(hd4_phases),
                "trivariate": trivariate,
                "unicode_ops": ",".join(str(op) for op in unicode_ops),
                "unicode_string": unicode_str,
            }
            
            ids.append(chain_id)
            documents.append(embedding_text)
            metadatas.append(metadata)
            if embedding:
                embeddings.append(embedding)
            
            count += 1
        
        # Insert all
        if ids:
            if embeddings:
                collection.upsert(ids=ids, documents=documents, metadatas=metadatas, embeddings=embeddings)
            else:
                collection.upsert(ids=ids, documents=documents, metadatas=metadatas)
        
        print(f"✅ Added {count} tool chains to ChromaDB collection 'tool_chains'")
        return count
    
    def run_full_pipeline(self) -> Dict[str, int]:
        """Run the complete pipeline: load data and add to ChromaDB."""
        results = {}
        
        # Load data
        data = self.load_data_from_matching_output()
        
        # Add to ChromaDB
        if data["tools"]:
            results["tools"] = self.add_tools_to_chromadb(data["tools"])
        
        if data["ctas_tasks"]:
            results["ctas_tasks"] = self.add_ctas_tasks_to_chromadb(data["ctas_tasks"])
        
        if data["ptcc_configs"]:
            results["ptcc_configs"] = self.add_ptcc_configs_to_chromadb(data["ptcc_configs"])
        
        if data["tool_chains"]:
            results["tool_chains"] = self.add_tool_chains_to_chromadb(data["tool_chains"])
        
        # Print summary
        print("\n" + "=" * 70)
        print("PIPELINE COMPLETE")
        print("=" * 70)
        for key, value in results.items():
            print(f"  {key}: {value} vectors added")
        
        # Print collection stats
        print("\n📊 ChromaDB Collection Stats:")
        for name, collection in self.collections.items():
            count = collection.count()
            print(f"  {name}: {count} vectors")
        
        return results


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Add threat intelligence to ChromaDB with Unicode operations")
    parser.add_argument("--tools-only", action="store_true", help="Only add tools")
    parser.add_argument("--tasks-only", action="store_true", help="Only add CTAS tasks")
    parser.add_argument("--ptcc-only", action="store_true", help="Only add PTCC configs")
    parser.add_argument("--chains-only", action="store_true", help="Only add tool chains")
    args = parser.parse_args()
    
    if not HAS_CHROMADB:
        print("❌ ChromaDB not available. Install with: pip install chromadb")
        return
    
    loader = ChromaDBUnicodeLoader()
    
    if args.tools_only or args.tasks_only or args.ptcc_only or args.chains_only:
        data = loader.load_data_from_matching_output()
        
        if args.tools_only:
            loader.add_tools_to_chromadb(data["tools"])
        if args.tasks_only:
            loader.add_ctas_tasks_to_chromadb(data["ctas_tasks"])
        if args.ptcc_only:
            loader.add_ptcc_configs_to_chromadb(data["ptcc_configs"])
        if args.chains_only:
            loader.add_tool_chains_to_chromadb(data["tool_chains"])
    else:
        loader.run_full_pipeline()


if __name__ == "__main__":
    main()



