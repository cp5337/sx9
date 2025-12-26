#!/usr/bin/env python3
"""
Claude Code ↔ Antigravity Bridge
Enables multi-agent long-running harness communication

This script:
1. Reads Antigravity boot configs (TOML)
2. Exposes agent capabilities to Claude Code via stdin/stdout MCP protocol
3. Manages agent handoffs and state persistence
4. Routes through sx9-harness governance gates

Usage:
  claude mcp add antigravity -- python3 /path/to/claude_code_bridge.py
"""

import json
import sys
import os
from pathlib import Path
from typing import Any
import tomllib  # Python 3.11+

# Antigravity root
AG_ROOT = Path(__file__).parent
SX9_ROOT = AG_ROOT.parent

# Load configurations
def load_toml(path: Path) -> dict:
    """Load TOML configuration file."""
    if not path.exists():
        return {}
    with open(path, "rb") as f:
        return tomllib.load(f)

def load_boot_manifest() -> dict:
    """Load the master boot manifest."""
    return load_toml(AG_ROOT / "boot_manifest.toml")

def load_clsgs_agents() -> dict:
    """Load CLSGS agent definitions."""
    return load_toml(AG_ROOT / "clsgs_agents.toml")

def load_harness_bridge() -> dict:
    """Load harness bridge configuration."""
    return load_toml(AG_ROOT / "harness_bridge.toml")

def load_brain(brain_file: str) -> dict:
    """Load a brain configuration."""
    return load_toml(AG_ROOT / brain_file)

# MCP Protocol Implementation
class AntigravityMCP:
    """MCP server for Antigravity multi-agent harness."""

    def __init__(self):
        self.manifest = load_boot_manifest()
        self.agents = load_clsgs_agents()
        self.harness = load_harness_bridge()
        self.current_agent = "FORGE"  # Default
        self.session_state = {}

    def get_capabilities(self) -> dict:
        """Return MCP capabilities."""
        return {
            "name": "antigravity",
            "version": "1.0.0",
            "description": "Multi-agent harness for SX9 cognitive factory",
            "tools": self._get_tools(),
            "resources": self._get_resources(),
        }

    def _get_tools(self) -> list:
        """Define available MCP tools."""
        return [
            {
                "name": "switch_agent",
                "description": "Switch to a different CLSGS agent persona",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "agent": {
                            "type": "string",
                            "enum": list(self.agents.get("agents", {}).keys()),
                            "description": "Agent to switch to (FORGE, AXIOM, VECTOR, etc.)"
                        },
                        "context": {
                            "type": "string",
                            "description": "Context to pass to the new agent"
                        }
                    },
                    "required": ["agent"]
                }
            },
            {
                "name": "get_agent_scope",
                "description": "Get the N-V-N-N behavioral scope for an agent",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "agent": {
                            "type": "string",
                            "description": "Agent name"
                        }
                    },
                    "required": ["agent"]
                }
            },
            {
                "name": "check_governance_gate",
                "description": "Check if an action passes governance gates",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "description": "Action to check"
                        },
                        "drift_score": {
                            "type": "number",
                            "description": "Current drift score (0.0-1.0)"
                        }
                    },
                    "required": ["action"]
                }
            },
            {
                "name": "get_v0_preamble",
                "description": "Get the V0 framework preamble (Vite/React/Tauri, NOT Next.js)",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "log_lineage_event",
                "description": "Log a lineage tracking event for git history",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "event_type": {
                            "type": "string",
                            "enum": ["annotation_added", "annotation_removed", "scope_change", "file_modified"]
                        },
                        "file": {"type": "string"},
                        "details": {"type": "string"}
                    },
                    "required": ["event_type", "file"]
                }
            },
            {
                "name": "get_current_mode",
                "description": "Get current Antigravity boot mode and configuration",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            },
            {
                "name": "list_agents",
                "description": "List all available CLSGS agents with their roles",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            }
        ]

    def _get_resources(self) -> list:
        """Define available MCP resources."""
        return [
            {
                "uri": "antigravity://config/manifest",
                "name": "Boot Manifest",
                "description": "Current Antigravity boot configuration"
            },
            {
                "uri": "antigravity://agents/current",
                "name": "Current Agent",
                "description": "Currently active agent and its scope"
            },
            {
                "uri": "antigravity://qa/status",
                "name": "QA Status",
                "description": "Current dual-heartbeat QA status"
            }
        ]

    def handle_tool_call(self, tool_name: str, arguments: dict) -> dict:
        """Handle an MCP tool call."""
        handlers = {
            "switch_agent": self._switch_agent,
            "get_agent_scope": self._get_agent_scope,
            "check_governance_gate": self._check_governance_gate,
            "get_v0_preamble": self._get_v0_preamble,
            "log_lineage_event": self._log_lineage_event,
            "get_current_mode": self._get_current_mode,
            "list_agents": self._list_agents,
        }

        handler = handlers.get(tool_name)
        if handler:
            return handler(arguments)
        return {"error": f"Unknown tool: {tool_name}"}

    def _switch_agent(self, args: dict) -> dict:
        """Switch to a different agent."""
        agent = args.get("agent", "FORGE")
        context = args.get("context", "")

        if agent not in self.agents.get("agents", {}):
            return {"error": f"Unknown agent: {agent}"}

        self.current_agent = agent
        agent_config = self.agents["agents"][agent]

        return {
            "success": True,
            "agent": agent,
            "role": agent_config.get("role"),
            "description": agent_config.get("description"),
            "behavioral_scope": agent_config.get("behavioral_scope"),
            "tools": agent_config.get("tools", []),
            "context_passed": context,
            "message": f"Switched to {agent} agent. Role: {agent_config.get('role')}"
        }

    def _get_agent_scope(self, args: dict) -> dict:
        """Get N-V-N-N behavioral scope for an agent."""
        agent = args.get("agent", self.current_agent)

        if agent not in self.agents.get("agents", {}):
            return {"error": f"Unknown agent: {agent}"}

        scope = self.agents["agents"][agent].get("behavioral_scope", {})
        nvnn = f"{scope.get('role', '?')} {scope.get('action', '?')} {scope.get('constraint', '?')} {scope.get('object', '?')}"

        return {
            "agent": agent,
            "nvnn_pattern": nvnn,
            "scope": scope
        }

    def _check_governance_gate(self, args: dict) -> dict:
        """Check governance gate for an action."""
        action = args.get("action", "unknown")
        drift_score = args.get("drift_score", 0.0)

        brain = load_brain("soft_dev_brain.toml")
        gates = brain.get("qa_doctrine", {}).get("governance_gates", {})

        if drift_score >= gates.get("halt", {}).get("threshold", 0.7):
            return {"gate": "HALT", "action": "block", "drift_score": drift_score}
        elif drift_score >= gates.get("warn", {}).get("threshold", 0.3):
            return {"gate": "WARN", "action": "advisory", "drift_score": drift_score}
        else:
            return {"gate": "OBSERVE", "action": "log", "drift_score": drift_score}

    def _get_v0_preamble(self, args: dict) -> dict:
        """Get V0 framework preamble."""
        brain = load_brain("soft_dev_brain.toml")
        v0_config = brain.get("v0_config", {})
        banned = v0_config.get("banned_patterns", {})

        preamble = f"""## V0 FRAMEWORK REQUIREMENTS (CRITICAL)

Generate code for: {v0_config.get('framework', 'Vite')} + {v0_config.get('ui_library', 'React 18')} + {v0_config.get('desktop', 'Tauri 2')}

### DO NOT USE (these are Next.js patterns):
- {', '.join(banned.get('next_js', []))}
- Server Components: {banned.get('server_components', False)}
- App Router: {banned.get('app_router', False)}

### USE INSTEAD:
- Standard React functional components
- {v0_config.get('styling', 'Tailwind CSS')} for styling
- {v0_config.get('icons', 'lucide-react')} for icons
- @tauri-apps/api for native features
"""
        return {"preamble": preamble, "config": v0_config}

    def _log_lineage_event(self, args: dict) -> dict:
        """Log a lineage tracking event."""
        event_type = args.get("event_type")
        file = args.get("file")
        details = args.get("details", "")

        # In production, this would write to NATS or a log file
        event = {
            "type": event_type,
            "file": file,
            "details": details,
            "agent": self.current_agent,
            "timestamp": "2025-12-26T00:00:00Z"  # Would use actual timestamp
        }

        return {"logged": True, "event": event}

    def _get_current_mode(self, args: dict) -> dict:
        """Get current boot mode."""
        default_mode = self.manifest.get("default_mode", "DEVELOPMENT")
        mode_config = self.manifest.get("modes", {}).get(default_mode, {})

        return {
            "mode": default_mode,
            "description": mode_config.get("description"),
            "brain": mode_config.get("brain"),
            "agents_file": mode_config.get("agents"),
            "tools": mode_config.get("tools", []),
            "environment": mode_config.get("environment"),
            "current_agent": self.current_agent
        }

    def _list_agents(self, args: dict) -> dict:
        """List all available agents."""
        agents_list = []
        for name, config in self.agents.get("agents", {}).items():
            agents_list.append({
                "name": name,
                "role": config.get("role"),
                "description": config.get("description"),
                "is_current": name == self.current_agent
            })
        return {"agents": agents_list, "current": self.current_agent}


def main():
    """Main MCP server loop."""
    mcp = AntigravityMCP()

    # MCP uses JSON-RPC over stdin/stdout
    for line in sys.stdin:
        try:
            request = json.loads(line.strip())
            method = request.get("method", "")
            params = request.get("params", {})
            req_id = request.get("id")

            if method == "initialize":
                response = {
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": mcp.get_capabilities()
                }
            elif method == "tools/list":
                response = {
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": {"tools": mcp._get_tools()}
                }
            elif method == "tools/call":
                tool_name = params.get("name", "")
                arguments = params.get("arguments", {})
                result = mcp.handle_tool_call(tool_name, arguments)
                response = {
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": {"content": [{"type": "text", "text": json.dumps(result, indent=2)}]}
                }
            elif method == "resources/list":
                response = {
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": {"resources": mcp._get_resources()}
                }
            else:
                response = {
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "error": {"code": -32601, "message": f"Method not found: {method}"}
                }

            print(json.dumps(response), flush=True)

        except json.JSONDecodeError:
            pass
        except Exception as e:
            error_response = {
                "jsonrpc": "2.0",
                "id": None,
                "error": {"code": -32603, "message": str(e)}
            }
            print(json.dumps(error_response), flush=True)


if __name__ == "__main__":
    main()
