#!/usr/bin/env python3
"""
SX9 (Synaptix9) System Status Board
Formalized ASCII dashboard for system status monitoring
"""

import os
import sys
import json
import subprocess
import socket
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Tuple

# ANSI Colors
class Colors:
    RED = '\033[0;31m'
    GREEN = '\033[0;32m'
    YELLOW = '\033[1;33m'
    BLUE = '\033[0;34m'
    CYAN = '\033[0;36m'
    MAGENTA = '\033[0;35m'
    BOLD = '\033[1m'
    NC = '\033[0m'  # No Color

# Status Symbols
class Symbols:
    CHECK = f"{Colors.GREEN}✓{Colors.NC}"
    CROSS = f"{Colors.RED}✗{Colors.NC}"
    WARN = f"{Colors.YELLOW}⚠{Colors.NC}"
    INFO = f"{Colors.BLUE}ℹ{Colors.NC}"


class StatusBoard:
    """SX9 (Synaptix9) System Status Board"""
    
    def __init__(self, base_dir: Optional[Path] = None):
        if base_dir is None:
            base_dir = Path(__file__).parent
        self.base_dir = Path(base_dir)
        self.output_dir = self.base_dir / "output"
        self.logs_dir = self.base_dir / "logs"
        self.node_interview_dir = self.base_dir / "node-interview-generator"
    
    def check_port(self, host: str, port: int) -> bool:
        """Check if a port is open"""
        try:
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(1)
            result = sock.connect_ex((host, port))
            sock.close()
            return result == 0
        except Exception:
            return False
    
    def get_directory_size(self, path: Path) -> Tuple[str, int]:
        """Get directory size and file count"""
        if not path.exists():
            return ("0B", 0)
        
        try:
            # Get size
            result = subprocess.run(
                ["du", "-sh", str(path)],
                capture_output=True,
                text=True,
                timeout=5
            )
            size = result.stdout.split()[0] if result.returncode == 0 else "0B"
            
            # Get file count
            if path.is_dir():
                count = len(list(path.glob("*.json")))
            else:
                count = 0
            
            return (size, count)
        except Exception:
            return ("0B", 0)
    
    def check_pipeline_status(self) -> Dict:
        """Check pipeline execution status"""
        status = {
            "running": False,
            "pid": None,
            "phases": {}
        }
        
        pid_file = self.logs_dir / "pipeline.pid"
        if pid_file.exists():
            try:
                pid = int(pid_file.read_text().strip())
                # Check if process is running
                try:
                    os.kill(pid, 0)
                    status["running"] = True
                    status["pid"] = pid
                except OSError:
                    status["running"] = False
            except (ValueError, FileNotFoundError):
                pass
        
        # Check phase outputs
        threat_content_dir = self.node_interview_dir / "output" / "threat_content"
        if threat_content_dir.exists():
            size, count = self.get_directory_size(threat_content_dir)
            status["phases"]["download"] = {"status": "complete", "size": size, "count": count}
        else:
            status["phases"]["download"] = {"status": "not_found"}
        
        ontology_dir = self.output_dir / "ontology"
        if ontology_dir.exists() and (ontology_dir / "ontology_raw.json").exists():
            size, count = self.get_directory_size(ontology_dir)
            status["phases"]["spires"] = {"status": "complete", "size": size, "count": count}
        else:
            status["phases"]["spires"] = {"status": "not_found"}
        
        dsl_dir = self.output_dir / "sx9_dsl"
        if dsl_dir.exists() and list(dsl_dir.glob("*.json")):
            size, count = self.get_directory_size(dsl_dir)
            status["phases"]["dsl"] = {"status": "complete", "size": size, "count": count}
        else:
            status["phases"]["dsl"] = {"status": "not_found"}
        
        # Check storage execution
        storage_logs = list(self.logs_dir.glob("storage_*.log"))
        if storage_logs:
            latest = max(storage_logs, key=lambda p: p.stat().st_mtime)
            if "✅" in latest.read_text() or "complete" in latest.read_text().lower():
                status["phases"]["storage"] = {"status": "complete"}
            else:
                status["phases"]["storage"] = {"status": "failed"}
        else:
            status["phases"]["storage"] = {"status": "not_executed"}
        
        return status
    
    def check_services(self) -> Dict:
        """Check service status"""
        services = {}
        
        # Neo4j
        services["neo4j_bolt"] = {
            "name": "Neo4j (Bolt)",
            "port": 7687,
            "running": self.check_port("localhost", 7687)
        }
        services["neo4j_http"] = {
            "name": "Neo4j (HTTP)",
            "port": 7474,
            "running": self.check_port("localhost", 7474)
        }
        
        # GLAF
        services["glaf"] = {
            "name": "GLAF",
            "port": 18050,
            "running": self.check_port("localhost", 18050)
        }
        
        # Supabase (check config)
        env_file = self.base_dir.parent / "sx9-ops-main-platform" / ".env.local"
        supabase_configured = (
            os.environ.get("SUPABASE_URL") is not None or
            env_file.exists()
        )
        services["supabase"] = {
            "name": "Supabase",
            "port": None,
            "running": supabase_configured
        }
        
        # Docker
        docker_running = False
        container_count = 0
        try:
            result = subprocess.run(
                ["docker", "ps", "-q"],
                capture_output=True,
                text=True,
                timeout=2
            )
            if result.returncode == 0:
                docker_running = True
                container_count = len([l for l in result.stdout.strip().split('\n') if l])
        except Exception:
            pass
        
        services["docker"] = {
            "name": "Docker",
            "port": None,
            "running": docker_running,
            "containers": container_count
        }
        
        return services
    
    def check_integration_plans(self) -> Dict:
        """Check if integration plans exist"""
        plans = {
            "unified": self.base_dir / "CTAS_INTEGRATION_PLANS.md",
            "osint": self.base_dir / "OSINT_CTAS_INTEGRATION.md",
            "kali": self.base_dir / "KALI_CTAS_INTEGRATION.md",
            "threat_intel": self.base_dir / "THREAT_INTEL_CTAS_INTEGRATION.md"
        }
        
        return {
            name: {"exists": path.exists(), "path": str(path)}
            for name, path in plans.items()
        }
    
    def get_recent_activity(self) -> Optional[str]:
        """Get recent log activity"""
        log_files = list(self.logs_dir.glob("*.log"))
        if not log_files:
            return None
        
        latest = max(log_files, key=lambda p: p.stat().st_mtime)
        try:
            lines = latest.read_text().splitlines()
            return "\n".join(lines[-3:]) if len(lines) > 3 else "\n".join(lines)
        except Exception:
            return None
    
    def render(self):
        """Render the status board"""
        # Header with ASCII Art - SX9
        print("")
        print("╔══════════════════════════════════════════════════════════════════════════════╗")
        print("║                                                                              ║")
        print("║" + Colors.CYAN + "     ███████╗██╗  ██╗" + Colors.MAGENTA + "█████╗ " + Colors.NC + "                                                      ║")
        print("║" + Colors.CYAN + "     ██╔════╝╚██╗██╔╝" + Colors.MAGENTA + "██╔══██╗" + Colors.NC + "                                                     ║")
        print("║" + Colors.CYAN + "     ███████╗ ╚███╔╝ " + Colors.MAGENTA + "██║  ██║" + Colors.NC + "                                                     ║")
        print("║" + Colors.CYAN + "     ╚════██║ ██╔██╗ " + Colors.MAGENTA + "██║  ██║" + Colors.NC + "                                                     ║")
        print("║" + Colors.CYAN + "     ███████║██╔╝ ██╗" + Colors.MAGENTA + "╚█████╔╝" + Colors.NC + "                                                     ║")
        print("║" + Colors.CYAN + "     ╚══════╝╚═╝  ╚═╝" + Colors.MAGENTA + " ╚════╝ " + Colors.NC + "                                                      ║")
        print("║                                                                              ║")
        print("║" + Colors.MAGENTA + "                    SYSTEM STATUS BOARD" + Colors.NC + "                                  ║")
        print("╚══════════════════════════════════════════════════════════════════════════════╝")
        print("")
        
        # Pipeline Status
        print("┌─ PIPELINE STATUS ───────────────────────────────────────────────────────────┐")
        pipeline = self.check_pipeline_status()
        
        if pipeline["running"]:
            print(f"│ Pipeline:        {Colors.GREEN}RUNNING{Colors.NC} (PID: {pipeline['pid']})")
        elif pipeline["pid"]:
            print(f"│ Pipeline:        {Colors.YELLOW}COMPLETED{Colors.NC} (process finished)")
        else:
            print(f"│ Pipeline:        {Colors.YELLOW}NOT RUNNING{Colors.NC}")
        
        print("│")
        print("│ Phases:")
        
        for phase_name, phase_info in pipeline["phases"].items():
            phase_display = phase_name.replace("_", " ").title()
            if phase_info["status"] == "complete":
                size = phase_info.get("size", "?")
                count = phase_info.get("count", 0)
                print(f"│   {Symbols.CHECK} {phase_display:15} {count} files ({size})")
            else:
                status_display = phase_info["status"].replace("_", " ").title()
                print(f"│   {Symbols.CROSS} {phase_display:15} {status_display}")
        
        print("└──────────────────────────────────────────────────────────────────────────────┘")
        print("")
        
        # Data Status
        print("┌─ DATA STATUS ────────────────────────────────────────────────────────────────┐")
        
        threat_dir = self.node_interview_dir / "output" / "threat_content"
        if threat_dir.exists():
            size, count = self.get_directory_size(threat_dir)
            print(f"│ Threat Content:  {Colors.GREEN}{size:>8}{Colors.NC} ({count} files)")
        else:
            print(f"│ Threat Content:  {Colors.RED}Not found{Colors.NC}")
        
        ontology_dir = self.output_dir / "ontology"
        if ontology_dir.exists():
            size, count = self.get_directory_size(ontology_dir)
            print(f"│ Ontology:        {Colors.GREEN}{size:>8}{Colors.NC} ({count} files)")
        else:
            print(f"│ Ontology:        {Colors.RED}Not found{Colors.NC}")
        
        dsl_dir = self.output_dir / "sx9_dsl"
        if dsl_dir.exists():
            size, count = self.get_directory_size(dsl_dir)
            print(f"│ DSL Conversion:  {Colors.GREEN}{size:>8}{Colors.NC} ({count} files)")
        else:
            print(f"│ DSL Conversion:  {Colors.YELLOW}Not generated{Colors.NC}")
        
        task_graph_dir = self.output_dir / "task_graph"
        if task_graph_dir.exists():
            size, count = self.get_directory_size(task_graph_dir)
            print(f"│ Task Graph:       {Colors.GREEN}{size:>8}{Colors.NC} ({count} files)")
        else:
            print(f"│ Task Graph:       {Colors.YELLOW}Not generated{Colors.NC}")
        
        print("└──────────────────────────────────────────────────────────────────────────────┘")
        print("")
        
        # Integration Plans
        print("┌─ INTEGRATION PLANS ──────────────────────────────────────────────────────────┐")
        plans = self.check_integration_plans()
        plan_names = {
            "unified": "Unified Plan",
            "osint": "OSINT Plan",
            "kali": "Kali Plan",
            "threat_intel": "Threat Intel"
        }
        
        for key, name in plan_names.items():
            if plans[key]["exists"]:
                print(f"│ {Symbols.CHECK} {name:18} {Path(plans[key]['path']).name}")
            else:
                print(f"│ {Symbols.CROSS} {name:18} Not found")
        
        print("└──────────────────────────────────────────────────────────────────────────────┘")
        print("")
        
        # Services
        print("┌─ SERVICES STATUS ────────────────────────────────────────────────────────────┐")
        services = self.check_services()
        
        for service_key, service_info in services.items():
            name = service_info["name"]
            if service_info["running"]:
                port_info = f" (port {service_info['port']})" if service_info["port"] else ""
                container_info = f" ({service_info['containers']} containers)" if "containers" in service_info else ""
                print(f"│ {name:18} {Colors.GREEN}RUNNING{Colors.NC}{port_info}{container_info}")
            else:
                print(f"│ {name:18} {Colors.YELLOW}NOT RUNNING{Colors.NC}")
        
        print("└──────────────────────────────────────────────────────────────────────────────┘")
        print("")
        
        # Recent Activity
        print("┌─ RECENT ACTIVITY ─────────────────────────────────────────────────────────────┐")
        activity = self.get_recent_activity()
        if activity:
            log_files = list(self.logs_dir.glob("*.log"))
            if log_files:
                latest = max(log_files, key=lambda p: p.stat().st_mtime)
                print(f"│ Latest log:      {latest.name}")
                print("│")
                for line in activity.splitlines()[:3]:
                    if line.strip():
                        print(f"│   {line[:70]}")
        else:
            print("│ No recent activity")
        
        print("└──────────────────────────────────────────────────────────────────────────────┘")
        print("")
        
        # Quick Actions
        print("┌─ QUICK ACTIONS ───────────────────────────────────────────────────────────────┐")
        print("│")
        print(f"│   {Colors.CYAN}./execute_full_pipeline.sh{Colors.NC}     Run full pipeline")
        print(f"│   {Colors.CYAN}./execute_storage_plan.sh{Colors.NC}      Run storage plan only")
        print(f"│   {Colors.CYAN}tail -f logs/pipeline_*.log{Colors.NC}    Watch pipeline logs")
        print(f"│   {Colors.CYAN}./status_board.py{Colors.NC}              Refresh this status board")
        print("│")
        print("└──────────────────────────────────────────────────────────────────────────────┘")
        print("")
        
        # Footer
        timestamp = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
        print(f"Last updated: {timestamp}")
        print("")


def main():
    """Main entry point"""
    board = StatusBoard()
    board.render()


if __name__ == "__main__":
    main()

