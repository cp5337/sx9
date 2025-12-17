#!/usr/bin/env python3
"""
Quick Integration Test for ABE QA System
TODO: Move to conda environment after initial testing

Run this to verify the ZenCoder + Playwright + Crawl4AI integration works
"""

import asyncio
import sys
import os

# Add src to path for imports
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'src'))

from zencoder_integration import test_zencoder_integration
import structlog

# Configure logging
structlog.configure(
    processors=[
        structlog.stdlib.add_log_level,
        structlog.processors.TimeStamper(fmt="iso"),
        structlog.processors.JSONRenderer()
    ],
    logger_factory=structlog.stdlib.LoggerFactory(),
)

logger = structlog.get_logger()

async def main():
    """Run all integration tests"""
    logger.info("=== ABE QA System Integration Test ===")
    logger.info("NOTE: Using Python prototype - TODO migrate to conda")

    try:
        # Test ZenCoder integration
        await test_zencoder_integration()

        logger.info("✅ All integration tests passed!")
        logger.info("Ready to move to conda environment")
        return 0

    except Exception as e:
        logger.error("❌ Integration tests failed", error=str(e))
        return 1

if __name__ == "__main__":
    exit_code = asyncio.run(main())