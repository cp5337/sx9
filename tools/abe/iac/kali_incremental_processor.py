#!/usr/bin/env python3
"""
Kali CLI Incremental Processor
Runs as a cron job to process tools that were missed due to rate limits
Stays within free-tier limits: 10 requests/min for Gemini API
"""

import json
import os
import time
from pathlib import Path
from datetime import datetime

# Import the extractor
import sys
sys.path.insert(0, str(Path(__file__).parent))
from kali_cli_dual_llm_extractor import KaliCLIExtractor

# Configuration
TOOLS_FILE = Path("/Users/cp5337/Developer/sx9/.qodo/04-abe-iac/node-interview-generator/output/kali_tools/kali_tools_with_commands.json")
BATCH_SIZE = 5  # Process 5 tools per run (stays under 10/min limit)
RATE_LIMIT_DELAY = 7  # 7 seconds between requests (8.5 requests/min)

def load_data():
    """Load existing extraction data"""
    if not TOOLS_FILE.exists():
        print("❌ Output file not found. Run full extraction first.")
        return None
    
    with open(TOOLS_FILE, 'r') as f:
        return json.load(f)

def find_missed_tools(data):
    """Find tools that used fallback (confidence < 0.5)"""
    missed = []
    
    for slug, tool in data['tools'].items():
        confidence = tool.get('confidence', 1.0)
        source = tool.get('source', '')
        
        # Find tools that need reprocessing
        if confidence < 0.5 or source == 'fallback':
            missed.append((slug, tool))
    
    return missed

def process_batch(extractor, missed_tools, batch_size):
    """Process a batch of missed tools"""
    processed = []
    
    for i, (slug, tool_data) in enumerate(missed_tools[:batch_size]):
        print(f"[{i+1}/{batch_size}] Reprocessing: {slug}")
        
        # Extract with rate limiting
        cli_data = extractor.extract_tool(slug, tool_data)
        
        # Merge with existing data
        updated_tool = {
            **tool_data,
            **cli_data,
            'reprocessed_at': datetime.utcnow().isoformat()
        }
        
        processed.append((slug, updated_tool))
        
        # Rate limiting
        if i < batch_size - 1:
            time.sleep(RATE_LIMIT_DELAY)
    
    return processed

def update_data(data, processed_tools):
    """Update data with reprocessed tools"""
    for slug, tool_data in processed_tools:
        data['tools'][slug] = tool_data
    
    # Update stats
    data['last_incremental_run'] = datetime.utcnow().isoformat()
    
    return data

def main():
    print("=" * 60)
    print("Kali CLI Incremental Processor")
    print(f"Run at: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print("=" * 60)
    
    # Load data
    data = load_data()
    if not data:
        return
    
    # Find missed tools
    missed = find_missed_tools(data)
    print(f"\n📊 Found {len(missed)} tools needing reprocessing")
    
    if not missed:
        print("✅ All tools processed successfully!")
        return
    
    # Process batch
    print(f"\n🔄 Processing batch of {min(BATCH_SIZE, len(missed))} tools...")
    
    extractor = KaliCLIExtractor()
    processed = process_batch(extractor, missed, BATCH_SIZE)
    
    # Update data
    data = update_data(data, processed)
    
    # Save
    with open(TOOLS_FILE, 'w') as f:
        json.dump(data, f, indent=2)
    
    # Report
    remaining = len(missed) - len(processed)
    print("\n" + "=" * 60)
    print("✅ Batch Complete!")
    print("=" * 60)
    print(f"Processed: {len(processed)} tools")
    print(f"Remaining: {remaining} tools")
    print(f"Next run will process: {min(BATCH_SIZE, remaining)} tools")
    print("=" * 60)
    
    # Estimate completion
    if remaining > 0:
        runs_needed = (remaining + BATCH_SIZE - 1) // BATCH_SIZE
        hours_needed = runs_needed * 0.25  # Assuming 15-min cron
        print(f"\n📅 Estimated completion: {runs_needed} runs (~{hours_needed:.1f} hours)")

if __name__ == '__main__':
    # Set environment
    os.environ['GEMINI_API_KEY'] = os.environ.get('GEMINI_API_KEY', 'AIzaSyA9wvdjofLJjzLpnEgfsoSyKgU0OhSnCeM')
    
    main()
