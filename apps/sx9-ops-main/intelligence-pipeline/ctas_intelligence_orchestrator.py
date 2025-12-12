#!/usr/bin/env python3
"""
CTAS Intelligence Orchestrator
Coordinates data collection from Kali tools, Caldera, and Atomic Red Team
"""

import json
import time
import asyncio
from pathlib import Path
from typing import Dict, List
import subprocess
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class CTASIntelligenceOrchestrator:
    """Orchestrates collection of tool and adversary emulation intelligence"""
    
    def __init__(self, output_dir: str = "./ctas_intelligence"):
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(exist_ok=True)
        
        # Data collection status
        self.collection_status = {
            "kali_tools": {"started": False, "completed": False, "count": 0},
            "kali_executions": {"started": False, "completed": False, "count": 0},
            "caldera_abilities": {"started": False, "completed": False, "count": 0},
            "atomic_tests": {"started": False, "completed": False, "count": 0},
            "sled_storage": {"started": False, "completed": False, "records": 0}
        }
    
    async def orchestrate_full_collection(self):
        """Run complete intelligence collection pipeline"""
        logger.info("🚀 Starting CTAS Intelligence Collection Pipeline")
        
        # Phase 1: Web scraping
        await self.phase1_web_scraping()
        
        # Phase 2: Adversary emulation extraction
        await self.phase2_adversary_emulation()
        
        # Phase 3: Docker tool execution
        await self.phase3_tool_execution()
        
        # Phase 4: Data aggregation and analysis
        await self.phase4_data_aggregation()
        
        # Phase 5: Generate final intelligence report
        await self.phase5_intelligence_report()
        
        logger.info("✅ CTAS Intelligence Collection Pipeline Complete!")
    
    async def phase1_web_scraping(self):
        """Phase 1: Scrape Kali tools metadata from web"""
        logger.info("📡 Phase 1: Web Scraping Kali Tools")
        
        self.collection_status["kali_tools"]["started"] = True
        
        try:
            # Run Kali web scraper
            result = subprocess.run([
                "python3", "kali_web_scraper.py",
                "--output-dir", str(self.output_dir / "kali_web_data")
            ], capture_output=True, text=True, timeout=300)
            
            if result.returncode == 0:
                # Count scraped tools
                tools_file = self.output_dir / "kali_web_data" / "kali_tools.json"
                if tools_file.exists():
                    with open(tools_file) as f:
                        tools_data = json.load(f)
                        self.collection_status["kali_tools"]["count"] = len(tools_data)
                
                self.collection_status["kali_tools"]["completed"] = True
                logger.info(f"✅ Scraped {self.collection_status['kali_tools']['count']} Kali tools")
            else:
                logger.error(f"❌ Kali scraping failed: {result.stderr}")
                
        except Exception as e:
            logger.error(f"❌ Phase 1 failed: {e}")
    
    async def phase2_adversary_emulation(self):
        """Phase 2: Extract Caldera and Atomic Red Team data"""
        logger.info("🎭 Phase 2: Adversary Emulation Extraction")
        
        self.collection_status["caldera_abilities"]["started"] = True
        self.collection_status["atomic_tests"]["started"] = True
        
        try:
            # Run adversary emulation extractor
            result = subprocess.run([
                "python3", "caldera_art_extractor.py",
                "--output-dir", str(self.output_dir / "adversary_data")
            ], capture_output=True, text=True, timeout=600)
            
            if result.returncode == 0:
                # Count extracted data
                adversary_dir = self.output_dir / "adversary_data"
                
                # Count Caldera abilities
                caldera_file = adversary_dir / "caldera_abilities.json"
                if caldera_file.exists():
                    with open(caldera_file) as f:
                        caldera_data = json.load(f)
                        self.collection_status["caldera_abilities"]["count"] = len(caldera_data)
                        self.collection_status["caldera_abilities"]["completed"] = True
                
                # Count Atomic tests
                atomic_file = adversary_dir / "atomic_red_team_tests.json"
                if atomic_file.exists():
                    with open(atomic_file) as f:
                        atomic_data = json.load(f)
                        self.collection_status["atomic_tests"]["count"] = len(atomic_data)
                        self.collection_status["atomic_tests"]["completed"] = True
                
                logger.info(f"✅ Extracted {self.collection_status['caldera_abilities']['count']} Caldera abilities")
                logger.info(f"✅ Extracted {self.collection_status['atomic_tests']['count']} Atomic tests")
            else:
                logger.error(f"❌ Adversary extraction failed: {result.stderr}")
                
        except Exception as e:
            logger.error(f"❌ Phase 2 failed: {e}")
    
    async def phase3_tool_execution(self):
        """Phase 3: Execute tools in Docker and capture behavior"""
        logger.info("🐳 Phase 3: Docker Tool Execution")
        
        self.collection_status["kali_executions"]["started"] = True
        
        try:
            # Check if Docker containers are running
            if not self.check_docker_containers():
                logger.warning("⚠️ Docker containers not ready, skipping execution phase")
                return
            
            # Run Docker tool executor
            result = subprocess.run([
                "python3", "docker_kali_executor.py",
                "--output-dir", str(self.output_dir / "tool_executions"),
                "--tools", "nmap", "gobuster", "nikto", "hydra"
            ], capture_output=True, text=True, timeout=900)
            
            if result.returncode == 0:
                # Count executions
                exec_dir = self.output_dir / "tool_executions" / "executions"
                if exec_dir.exists():
                    execution_count = len(list(exec_dir.iterdir()))
                    self.collection_status["kali_executions"]["count"] = execution_count
                    self.collection_status["kali_executions"]["completed"] = True
                    logger.info(f"✅ Completed {execution_count} tool executions")
            else:
                logger.error(f"❌ Tool execution failed: {result.stderr}")
                
        except Exception as e:
            logger.error(f"❌ Phase 3 failed: {e}")
    
    async def phase4_data_aggregation(self):
        """Phase 4: Aggregate all collected data"""
        logger.info("📊 Phase 4: Data Aggregation")
        
        try:
            # Create unified dataset
            unified_data = {
                "collection_timestamp": int(time.time()),
                "kali_tools": self.load_kali_tools_data(),
                "tool_executions": self.load_execution_data(),
                "caldera_abilities": self.load_caldera_data(),
                "atomic_tests": self.load_atomic_data(),
                "primitive_mapping": self.generate_primitive_mapping()
            }
            
            # Save unified dataset
            unified_file = self.output_dir / "ctas_unified_intelligence.json"
            with open(unified_file, 'w') as f:
                json.dump(unified_data, f, indent=2, default=str)
            
            logger.info(f"✅ Unified intelligence dataset saved to {unified_file}")
            
        except Exception as e:
            logger.error(f"❌ Phase 4 failed: {e}")
    
    async def phase5_intelligence_report(self):
        """Phase 5: Generate comprehensive intelligence report"""
        logger.info("📋 Phase 5: Intelligence Report Generation")
        
        try:
            report = self.generate_intelligence_report()
            
            # Save report
            report_file = self.output_dir / "ctas_intelligence_report.m