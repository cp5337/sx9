#!/bin/bash
# Cron job setup for Kali CLI incremental processing
# Processes 5 tools every 15 minutes, staying within free-tier limits

# Add to crontab with:
# crontab -e
# Then add this line:
# */15 * * * * /Users/cp5337/Developer/sx9/tools/abe/iac/kali_cron.sh >> /Users/cp5337/Developer/sx9/tools/abe/iac/kali_cron.log 2>&1

# Set environment
export GEMINI_API_KEY="AIzaSyA9wvdjofLJjzLpnEgfsoSyKgU0OhSnCeM"
export PATH="/opt/miniconda3/bin:$PATH"

# Navigate to directory
cd /Users/cp5337/Developer/sx9/tools/abe/iac

# Run incremental processor
/opt/miniconda3/bin/python3 kali_incremental_processor.py

# Exit
exit 0
