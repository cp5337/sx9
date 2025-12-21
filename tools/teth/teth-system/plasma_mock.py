#!/usr/bin/env python3
"""
Mock PLASMA Defender

A simple HTTP server that receives TETH tool events
and simulates threat detection. For testing TETH integration.
"""

import json
import random
import time
import uuid
from datetime import datetime
from http.server import HTTPServer, BaseHTTPRequestHandler
from typing import Dict, List, Any

# Detection rules based on tool characteristics
DETECTION_RULES = {
    # High-risk tools - always detected
    "high_risk": ["wannacry", "destover", "rootkit", "bootkit"],
    
    # APT-exclusive tools - high detection rate
    "apt_tools": ["sunburst", "teardrop", "xagent", "sofacy", "carbanak"],
    
    # Common tools - moderate detection
    "common": ["mimikatz", "cobalt_strike", "metasploit", "powershell_empire"],
    
    # Stealthy tools - low detection
    "stealthy": ["dns_tunnel", "steganography", "timestomp"],
    
    # Recon tools - usually undetected
    "recon": ["nmap", "shodan", "masscan", "theharvester"],
}


class PLASMAHandler(BaseHTTPRequestHandler):
    """HTTP handler for PLASMA mock."""
    
    # Track chains for correlation
    chains: Dict[str, List[Dict]] = {}
    
    def log_message(self, format, *args):
        """Custom logging."""
        timestamp = datetime.now().strftime("%H:%M:%S")
        print(f"[{timestamp}] {args[0]}")
    
    def send_json(self, status: int, data: Dict):
        """Send JSON response."""
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(json.dumps(data).encode())
    
    def do_GET(self):
        """Handle GET requests."""
        if self.path == "/health":
            self.send_json(200, {
                "status": "healthy",
                "service": "plasma-defender-mock",
                "version": "1.0.0"
            })
        elif self.path == "/stats":
            self.send_json(200, {
                "chains_tracked": len(self.chains),
                "total_events": sum(len(c) for c in self.chains.values())
            })
        else:
            self.send_json(404, {"error": "not found"})
    
    def do_POST(self):
        """Handle POST requests."""
        content_length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(content_length)
        
        try:
            data = json.loads(body) if body else {}
        except json.JSONDecodeError:
            self.send_json(400, {"error": "invalid JSON"})
            return
        
        if self.path in ["/api/v1/ingest/tool", "/api/v1/ingest/chain"]:
            response = self.handle_tool_event(data)
            self.send_json(200, response)
        else:
            self.send_json(404, {"error": "not found"})
    
    def handle_tool_event(self, data: Dict) -> Dict:
        """Process a tool event and return detection result."""
        start_time = time.time()
        
        tool = data.get("tool", {})
        tool_id = tool.get("id", "unknown")
        tool_name = tool.get("name", "Unknown")
        entropy = tool.get("entropy", 0)
        risk = tool.get("operational_risk", 0.5)
        
        chain_context = data.get("chain_context")
        attribution = data.get("attribution")
        
        # Track chain
        if chain_context:
            chain_id = chain_context.get("chain_id")
            if chain_id not in PLASMAHandler.chains:
                PLASMAHandler.chains[chain_id] = []
            PLASMAHandler.chains[chain_id].append(data)
        
        # Determine detection
        detected, confidence, alerts = self.evaluate_detection(
            tool_id, entropy, chain_context, attribution
        )
        
        detection_time = (time.time() - start_time) * 1000
        
        # Log
        status = "🚨 DETECTED" if detected else "✅ Passed"
        print(f"  {status}: {tool_name} (entropy={entropy:.1f})")
        
        if chain_context:
            pos = chain_context.get("position", 0) + 1
            total = chain_context.get("total_tools", 1)
            print(f"    Chain position: {pos}/{total}")
        
        if attribution:
            apt = attribution.get("apt_group", "unknown")
            conf = attribution.get("confidence", 0)
            print(f"    Attribution: {apt} ({conf:.0%})")
        
        return {
            "event_id": data.get("event_id", str(uuid.uuid4())),
            "detected": detected,
            "detection_time_ms": detection_time,
            "threat_score": confidence,
            "alerts": alerts,
            "recommended_action": self.get_recommendation(detected, confidence)
        }
    
    def evaluate_detection(
        self,
        tool_id: str,
        entropy: float,
        chain_context: Dict = None,
        attribution: Dict = None
    ) -> tuple[bool, float, List[str]]:
        """Evaluate if a tool should be detected."""
        alerts = []
        confidence = 0.0
        
        # Rule-based detection
        if tool_id in DETECTION_RULES["high_risk"]:
            alerts.append(f"HIGH_RISK_TOOL: {tool_id}")
            confidence = 0.95
        elif tool_id in DETECTION_RULES["apt_tools"]:
            alerts.append(f"APT_TOOL_DETECTED: {tool_id}")
            confidence = 0.85
        elif tool_id in DETECTION_RULES["common"]:
            confidence = 0.6 + random.uniform(0, 0.2)
            if random.random() < 0.7:
                alerts.append(f"KNOWN_ATTACK_TOOL: {tool_id}")
        elif tool_id in DETECTION_RULES["stealthy"]:
            confidence = 0.3 + random.uniform(0, 0.2)
            if random.random() < 0.3:
                alerts.append(f"SUSPICIOUS_ACTIVITY: {tool_id}")
        elif tool_id in DETECTION_RULES["recon"]:
            confidence = 0.1 + random.uniform(0, 0.1)
            if random.random() < 0.1:
                alerts.append(f"RECON_DETECTED: {tool_id}")
        else:
            # Unknown tool - entropy-based detection
            if entropy > 30:
                confidence = 0.7
                alerts.append(f"HIGH_ENTROPY_TOOL: entropy={entropy:.1f}")
            elif entropy > 20:
                confidence = 0.4
            else:
                confidence = 0.2
        
        # Chain correlation bonus
        if chain_context:
            chain_id = chain_context.get("chain_id")
            position = chain_context.get("position", 0)
            
            if position > 2:
                # Sustained activity increases detection
                confidence += 0.1
                if position > 4:
                    alerts.append(f"CHAIN_ACTIVITY: {position+1} tools in sequence")
                    confidence += 0.1
        
        # Attribution bonus
        if attribution:
            apt = attribution.get("apt_group")
            attr_conf = attribution.get("confidence", 0)
            
            if attr_conf > 0.7:
                alerts.append(f"APT_ATTRIBUTION: {apt} ({attr_conf:.0%})")
                confidence += 0.15
        
        # Cap confidence
        confidence = min(1.0, confidence)
        
        # Detection threshold
        detected = confidence > 0.5 or len(alerts) > 0
        
        return detected, confidence, alerts
    
    def get_recommendation(self, detected: bool, confidence: float) -> str:
        """Get recommended action."""
        if not detected:
            return "MONITOR"
        elif confidence > 0.8:
            return "ISOLATE_AND_INVESTIGATE"
        elif confidence > 0.6:
            return "ALERT_SOC"
        else:
            return "LOG_AND_MONITOR"


def run_server(port: int = 8080):
    """Run the mock PLASMA server."""
    server = HTTPServer(("0.0.0.0", port), PLASMAHandler)
    
    print("=" * 60)
    print("  PLASMA Defender (Mock)")
    print("=" * 60)
    print(f"  Listening on port {port}")
    print()
    print("  Endpoints:")
    print("    GET  /health              - Health check")
    print("    GET  /stats               - Statistics")
    print("    POST /api/v1/ingest/tool  - Single tool event")
    print("    POST /api/v1/ingest/chain - Chain event")
    print()
    print("  Waiting for TETH events...")
    print("-" * 60)
    
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nShutting down...")
        server.shutdown()


if __name__ == "__main__":
    import sys
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8080
    run_server(port)
