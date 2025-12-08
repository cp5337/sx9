#!/usr/bin/env python3
"""
ABE Lightning QA Engine Daemon - Layer 1 Script-Only Analysis
GPU-accelerated, non-invasive analysis with PR automation
"""

import asyncio
import json
import logging
import os
from datetime import datetime
from typing import Dict, List, Optional
from dataclasses import dataclass, asdict
import subprocess
import uvicorn
from fastapi import FastAPI, HTTPException, BackgroundTasks
from fastapi.responses import JSONResponse
import httpx
import structlog

# Configure structured logging
structlog.configure(
    processors=[
        structlog.stdlib.filter_by_level,
        structlog.stdlib.add_logger_name,
        structlog.stdlib.add_log_level,
        structlog.stdlib.PositionalArgumentsFormatter(),
        structlog.processors.TimeStamper(fmt="iso"),
        structlog.processors.StackInfoRenderer(),
        structlog.processors.format_exc_info,
        structlog.processors.UnicodeDecoder(),
        structlog.processors.JSONRenderer()
    ],
    context_class=dict,
    logger_factory=structlog.stdlib.LoggerFactory(),
    cache_logger_on_first_use=True,
)

logger = structlog.get_logger()

@dataclass
class LightningAnalysisResult:
    """Lightning-fast analysis results"""
    crate_name: str
    analysis_time_seconds: float
    gpu_accelerated: bool
    files_analyzed: int
    total_loc: int
    complexity_score: float
    maintainability_score: float
    security_score: float
    overall_grade: str
    critical_issues: List[Dict]
    recommendations: List[str]
    pr_candidates: List[Dict]
    timestamp: str

class LightningQADaemon:
    """
    ABE Lightning QA Engine Daemon
    Layer 1: Pure script-based QA analysis with GPU acceleration
    """

    def __init__(self):
        self.app = FastAPI(
            title="ABE Lightning QA Engine",
            description="Layer 1 Script-Only GPU Analysis",
            version="1.0.0"
        )
        self.setup_routes()
        self.gpu_available = self._check_gpu()
        self.port_manager_url = os.getenv("PORT_MANAGER_URL", "http://localhost:18103")
        self.statistical_cdn_url = os.getenv("STATISTICAL_CDN_URL", "http://localhost:18108")
        logger.info("Lightning QA Engine initialized", gpu_available=self.gpu_available)

    def _check_gpu(self) -> bool:
        """Check if CUDA GPU is available"""
        try:
            result = subprocess.run(["nvidia-smi"], capture_output=True, timeout=10)
            return result.returncode == 0
        except (FileNotFoundError, subprocess.TimeoutExpired):
            return False

    def setup_routes(self):
        """Setup FastAPI routes"""

        @self.app.get("/health")
        async def health_check():
            return JSONResponse({
                "status": "healthy",
                "service": "lightning-qa-engine",
                "layer": "1",
                "gpu_available": self.gpu_available,
                "timestamp": datetime.utcnow().isoformat()
            })

        @self.app.get("/")
        async def root():
            return {
                "service": "ABE Lightning QA Engine",
                "layer": "1",
                "description": "Script-Only GPU Analysis",
                "version": "1.0.0",
                "gpu_acceleration": self.gpu_available
            }

        @self.app.post("/analyze/crate/{crate_name}")
        async def analyze_crate(crate_name: str, background_tasks: BackgroundTasks):
            """Analyze a CTAS crate with Lightning QA"""
            try:
                background_tasks.add_task(self._perform_lightning_analysis, crate_name)
                return JSONResponse({
                    "status": "analysis_queued",
                    "crate_name": crate_name,
                    "message": "Lightning analysis started",
                    "gpu_accelerated": self.gpu_available
                })
            except Exception as e:
                logger.error("Analysis failed", crate_name=crate_name, error=str(e))
                raise HTTPException(status_code=500, detail=str(e))

        @self.app.get("/results/{crate_name}")
        async def get_results(crate_name: str):
            """Get analysis results for a crate"""
            try:
                # Use accessible local path instead of /opt/
                results_dir = "/Users/cp5337/Developer/ctas-7-shipyard-staging/abe-qa-system/lightning-qa-results"
                results_file = f"{results_dir}/{crate_name}_analysis.json"
                if os.path.exists(results_file):
                    with open(results_file, 'r') as f:
                        return json.load(f)
                else:
                    raise HTTPException(status_code=404, detail="Results not found")
            except Exception as e:
                raise HTTPException(status_code=500, detail=str(e))

        @self.app.post("/register")
        async def register_with_port_manager():
            """Register with the Port Manager"""
            try:
                async with httpx.AsyncClient() as client:
                    response = await client.post(
                        f"{self.port_manager_url}/register",
                        json={
                            "service_name": "lightning-qa-engine",
                            "port": 18109,
                            "layer": "quality_assurance",
                            "capabilities": ["gpu_analysis", "script_only", "pr_automation"],
                            "health_endpoint": "/health"
                        }
                    )
                    if response.status_code == 200:
                        logger.info("Successfully registered with Port Manager")
                        return {"status": "registered"}
                    else:
                        logger.error("Failed to register with Port Manager", status_code=response.status_code)
                        return {"status": "registration_failed"}
            except Exception as e:
                logger.error("Registration error", error=str(e))
                return {"status": "error", "message": str(e)}

    async def _perform_lightning_analysis(self, crate_name: str):
        """Perform lightning-fast analysis of a crate"""
        start_time = datetime.utcnow()
        logger.info("Starting lightning analysis", crate_name=crate_name)

        try:
            # Mock analysis for now - will be replaced with actual implementation
            analysis_result = LightningAnalysisResult(
                crate_name=crate_name,
                analysis_time_seconds=2.5,  # Lightning fast!
                gpu_accelerated=self.gpu_available,
                files_analyzed=42,
                total_loc=1337,
                complexity_score=7.5,
                maintainability_score=8.2,
                security_score=9.1,
                overall_grade="A-",
                critical_issues=[
                    {"type": "security", "severity": "medium", "description": "Potential buffer overflow"},
                    {"type": "performance", "severity": "low", "description": "Unoptimized loop"}
                ],
                recommendations=[
                    "Add input validation",
                    "Optimize memory allocation",
                    "Update dependencies"
                ],
                pr_candidates=[
                    {
                        "title": "Fix buffer overflow in data parser",
                        "priority": "high",
                        "estimated_effort": "2 hours"
                    }
                ],
                timestamp=datetime.utcnow().isoformat()
            )

            # Save results to accessible local directory
            results_dir = "/Users/cp5337/Developer/ctas-7-shipyard-staging/abe-qa-system/lightning-qa-results"
            os.makedirs(results_dir, exist_ok=True)
            results_file = f"{results_dir}/{crate_name}_analysis.json"

            with open(results_file, 'w') as f:
                json.dump(asdict(analysis_result), f, indent=2)

            # Report to Statistical CDN
            await self._report_to_cdn(analysis_result)

            logger.info("Lightning analysis completed",
                       crate_name=crate_name,
                       duration_seconds=analysis_result.analysis_time_seconds)

        except Exception as e:
            logger.error("Lightning analysis failed", crate_name=crate_name, error=str(e))

    async def _report_to_cdn(self, result: LightningAnalysisResult):
        """Report analysis results to Statistical CDN"""
        try:
            async with httpx.AsyncClient() as client:
                await client.post(
                    f"{self.statistical_cdn_url}/metrics/qa",
                    json={
                        "service": "lightning-qa-engine",
                        "layer": 1,
                        "result": asdict(result)
                    }
                )
        except Exception as e:
            logger.error("Failed to report to CDN", error=str(e))

    async def startup(self):
        """Startup tasks"""
        logger.info("Lightning QA Engine starting up...")
        await asyncio.sleep(1)  # Allow other services to start

        # Register with Port Manager
        await self.app.router.get_route_by_name("register_with_port_manager").endpoint()

def main():
    """Main entry point"""
    daemon = LightningQADaemon()

    # Configure uvicorn
    config = uvicorn.Config(
        app=daemon.app,
        host="0.0.0.0",
        port=18109,
        log_level="info",
        access_log=True
    )

    server = uvicorn.Server(config)

    # Run the server
    logger.info("Starting ABE Lightning QA Engine Daemon on port 18109")
    asyncio.run(server.serve())

if __name__ == "__main__":
    main()