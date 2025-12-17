# Kali CLI Incremental Processing - Cron Setup

## 🔄 Smart Incremental Processing

**Problem:** API rate limits cause some tools to use fallback  
**Solution:** Cron job processes 5 tools every 15 minutes  
**Result:** All 335 tools extracted within free-tier limits  

---

## 📋 Setup Instructions

### 1. Install Cron Job

```bash
# Edit crontab
crontab -e

# Add this line (processes 5 tools every 15 minutes):
*/15 * * * * /Users/cp5337/Developer/sx9/tools/abe/iac/kali_cron.sh >> /Users/cp5337/Developer/sx9/tools/abe/iac/kali_cron.log 2>&1
```

### 2. Verify Cron Job

```bash
# List cron jobs
crontab -l

# Check log
tail -f /Users/cp5337/Developer/sx9/tools/abe/iac/kali_cron.log
```

### 3. Monitor Progress

```bash
# Check how many tools still need processing
python3 kali_incremental_processor.py
```

---

## ⚙️ Configuration

**Batch Size:** 5 tools per run  
**Frequency:** Every 15 minutes  
**Rate Limit:** 7 seconds between requests (8.5 req/min)  
**Free Tier Limit:** 10 requests/min (Gemini API)  

**Safety Margin:** 15% below rate limit to avoid quota errors

---

## 📊 Processing Schedule

| Time | Tools Processed | Remaining | Status |
|------|----------------|-----------|--------|
| 0:00 | 5 | 30 | Running |
| 0:15 | 10 | 25 | Running |
| 0:30 | 15 | 20 | Running |
| 0:45 | 20 | 15 | Running |
| 1:00 | 25 | 10 | Running |
| 1:15 | 30 | 5 | Running |
| 1:30 | 35 | 0 | ✅ Complete |

**Estimated completion:** ~2 hours for 35 missed tools

---

## 🎯 How It Works

### 1. Find Missed Tools
```python
# Tools with confidence < 0.5 or source == 'fallback'
missed = find_missed_tools(data)
```

### 2. Process Batch
```python
# Process 5 tools with rate limiting
for tool in missed[:5]:
    extract_cli(tool)
    sleep(7)  # Stay under 10 req/min
```

### 3. Update Data
```python
# Merge new data with existing
data['tools'][slug] = updated_tool
save(data)
```

### 4. Repeat
```
Cron runs every 15 minutes until all tools processed
```

---

## 📈 Rate Limit Strategy

### Free Tier Limits
- **Gemini API:** 10 requests/min
- **Vertex AI:** 10 requests/min (experimental model)

### Our Strategy
- **Batch size:** 5 tools
- **Delay:** 7 seconds between requests
- **Actual rate:** 8.5 requests/min
- **Safety margin:** 15%

### Why This Works
```
5 tools × 7 seconds = 35 seconds total
35 seconds / 5 tools = 7 seconds/tool
60 seconds / 7 seconds = 8.57 requests/min
8.57 < 10 ✅ (under limit)
```

---

## 🔍 Monitoring Commands

### Check Progress
```bash
# How many tools left?
python3 -c "
import json
data = json.load(open('kali_tools_with_commands.json'))
missed = [t for t in data['tools'].values() if t.get('confidence', 1) < 0.5]
print(f'{len(missed)} tools remaining')
"
```

### View Cron Log
```bash
tail -f kali_cron.log
```

### Manual Run (for testing)
```bash
./kali_cron.sh
```

---

## ✅ Benefits

1. **Zero Cost:** Stays within free tier
2. **Automatic:** Runs in background
3. **Resilient:** Handles rate limits gracefully
4. **Incremental:** Processes missed tools only
5. **Logged:** Full audit trail in cron.log

---

## 🚀 Quick Start

```bash
# 1. Make executable
chmod +x kali_cron.sh kali_incremental_processor.py

# 2. Test run
./kali_cron.sh

# 3. Install cron job
crontab -e
# Add: */15 * * * * /path/to/kali_cron.sh >> /path/to/kali_cron.log 2>&1

# 4. Verify
crontab -l

# 5. Monitor
tail -f kali_cron.log
```

---

## 📝 Example Output

```
============================================================
Kali CLI Incremental Processor
Run at: 2025-12-15 11:15:00
============================================================

📊 Found 35 tools needing reprocessing

🔄 Processing batch of 5 tools...
[1/5] Reprocessing: pupy
    ✅ Tier 3a (gemini): pupy
    ✓ Verified by Vertex AI (match!)
[2/5] Reprocessing: cupp
    ✅ Tier 3a (gemini): cupp
[3/5] Reprocessing: mentalist
    ✅ Tier 3b (vertex): mentalist
[4/5] Reprocessing: haiti
    ✅ Tier 3a (gemini): haiti
[5/5] Reprocessing: kerbrute
    ✅ Tier 3a (gemini): kerbrute

============================================================
✅ Batch Complete!
============================================================
Processed: 5 tools
Remaining: 30 tools
Next run will process: 5 tools
============================================================

📅 Estimated completion: 6 runs (~1.5 hours)
```

---

## 🎯 Result

**All 335 Kali tools extracted with CLI commands, staying 100% within free-tier limits!** 🚀
