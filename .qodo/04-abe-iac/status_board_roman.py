#!/usr/bin/env python3
"""
SX9 (Synaptix9) System Status Board - Roman Numeral Style
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

# Import from main status board
sys.path.insert(0, str(Path(__file__).parent))
from status_board import StatusBoard, Colors, Symbols


class StatusBoardRoman(StatusBoard):
    """SX9 System Status Board with Roman Numeral Styling"""
    
    def render(self):
        """Render the status board with Roman numeral style"""
        # Header with ASCII Art - SX9 in Roman Numeral Style
        print("")
        print("╔══════════════════════════════════════════════════════════════════════════════╗")
        print("║                                                                              ║")
        print("║" + Colors.CYAN + "     ╔═══╗╔╗   ╔╗" + Colors.MAGENTA + " ╔╗ ╔╗" + Colors.NC + "                                                          ║")
        print("║" + Colors.CYAN + "     ║╔═╗║║║   ║║" + Colors.MAGENTA + " ║║ ║║" + Colors.NC + "                                                          ║")
        print("║" + Colors.CYAN + "     ║╚══╗║║   ║║" + Colors.MAGENTA + " ║║ ║║" + Colors.NC + "                                                          ║")
        print("║" + Colors.CYAN + "     ╚══╗║║║   ║║" + Colors.MAGENTA + " ║║ ║║" + Colors.NC + "                                                          ║")
        print("║" + Colors.CYAN + "     ║╚═╝║║╚═╝║║" + Colors.MAGENTA + " ║╚═╝║" + Colors.NC + "                                                          ║")
        print("║" + Colors.CYAN + "     ╚═══╝╚═══╝╚" + Colors.MAGENTA + " ╚═══╝" + Colors.NC + "                                                          ║")
        print("║                                                                              ║")
        print("║" + Colors.MAGENTA + "                    SYSTEM STATUS BOARD" + Colors.NC + "                                  ║")
        print("╚══════════════════════════════════════════════════════════════════════════════╝")
        print("")
        
        # Call parent render for the rest
        # We'll override just the header, then call parent methods for content
        self._render_content()
    
    def _render_content(self):
        """Render the content sections (reusing parent logic)"""
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
        print(f"│   {Colors.CYAN}./status_board_roman.py{Colors.NC}        Refresh this status board")
        print("│")
        print("└──────────────────────────────────────────────────────────────────────────────┘")
        print("")
        
        # Footer
        timestamp = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
        print(f"Last updated: {timestamp}")
        print("")


def main():
    """Main entry point"""
    board = StatusBoardRoman()
    board.render()


if __name__ == "__main__":
    main()



