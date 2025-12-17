# 🚀 SX9 IaC - Quick Start Guide

**From Unicode → Running Infrastructure in Seconds**

---

## 🎯 THE COMPLETE FLOW

```
USER → Unicode (E800) → DATA PULL (3ms) → TERRAFORM → DOCKER → EXECUTE → CLEANUP
```

---

## 📋 PREREQUISITES

```bash
# Install dependencies
pip install aiohttp asyncpg

# Install Terraform
brew install terraform  # macOS
# OR
sudo apt install terraform  # Linux

# Verify Docker
docker --version

# Set environment
export CF_WORKER_URL="https://sx9-backend.usneodcp.workers.dev"
export SUPABASE_URL="https://your-project.supabase.co"
export SUPABASE_KEY="your-key"
```

---

## 🚀 USAGE EXAMPLES

### **Example 1: Single Tool (Nmap Scan)**

```bash
# Scan a network with Nmap
python3 iac_executor.py \
  --unicode E800 \
  --target 192.168.1.0/24

# What happens:
# 1. Pulls E800 (Nmap) manifest from CloudFlare KV (3ms)
# 2. Generates Terraform config
# 3. Spawns Docker container with Nmap
# 4. Runs: nmap -sV -O 192.168.1.0/24
# 5. Saves results to /var/sx9/runs/1234567890/
# 6. Auto-destroys container after 5 minutes

# Output:
📥 STEP 1: Pulling tool manifest...
✅ Pulled E800 from CloudFlare (X-Cache: HIT, ~3ms)
   Tool: nmap
   Category: NetworkRecon
   Docker: instrumentisto/nmap:latest

🏗️  STEP 2: Generating Terraform configuration...
   Written to: /var/sx9/runs/1234567890/terraform/main.tf

⚡ STEP 3: Executing Terraform apply...
   ✅ Container created: abc123def456
   📁 Output directory: /var/sx9/runs/1234567890

⏰ STEP 4: Scheduling cleanup in 300s...

📊 RESULT:
{
  "success": true,
  "container_id": "abc123def456",
  "output_dir": "/var/sx9/runs/1234567890",
  "terraform_dir": "/var/sx9/runs/1234567890/terraform"
}
```

---

### **Example 2: Tool Chain (Recon → Exploit)**

```bash
# Sequential attack chain
python3 iac_executor.py \
  --chain E800,E810,E820,E830 \
  --target example.com

# E800 = Nmap (port scan)
# E810 = Nikto (web scan)
# E820 = SQLMap (SQL injection)
# E830 = Metasploit (exploitation)

# What happens:
# 1. Pulls ALL 4 manifests (parallel, ~3-5ms each)
# 2. Generates chained Terraform with dependencies
# 3. Spawns containers sequentially:
#    a) Nmap runs → outputs to phase1/
#    b) Nikto waits for Nmap → uses ports from phase1/
#    c) SQLMap waits for Nikto → uses vulns from phase2/
#    d) Metasploit waits for SQLMap → auto-exploit
# 4. All results saved to /var/sx9/runs/1234567890/
# 5. Auto-cleanup after all phases complete

# Timeline:
[00:00] Nmap starts
[00:15] Nmap complete → Nikto starts
[01:30] Nikto complete → SQLMap starts
[03:45] SQLMap complete → Metasploit starts
[05:00] Metasploit complete
[10:00] Auto-cleanup
```

---

### **Example 3: Custom Tool**

```bash
# Use your custom scanner
python3 iac_executor.py \
  --unicode E900 \
  --target https://target.com

# E900 = Your custom tool (uploaded earlier)
```

---

## 📂 OUTPUT STRUCTURE

After execution, results are saved:

```
/var/sx9/runs/1234567890/
├── terraform/
│   ├── main.tf                  # Generated Terraform
│   ├── terraform.tfstate        # State file
│   └── terraform.tfstate.backup
│
├── nmap-results.xml            # Nmap output
├── nikto-results.json          # Nikto output
├── sqlmap-results.log          # SQLMap output
├── metasploit-session.txt      # Metasploit output
│
└── REPORT.md                   # Auto-generated summary
```

---

## 🔧 ADVANCED USAGE

### **Manual Terraform Inspection**

```bash
# Don't want auto-execute? Generate only:
cd /var/sx9/runs/1234567890/terraform

# Review Terraform config
cat main.tf

# Manually apply
terraform init
terraform plan
terraform apply

# Manual cleanup
terraform destroy
```

### **Custom Cleanup Time**

```bash
# Cleanup after 10 minutes instead of 5
export SX9_CLEANUP_SECONDS=600

python3 iac_executor.py --unicode E800 --target 192.168.1.0/24
```

### **Custom Output Directory**

```bash
# Save to custom location
export SX9_OUTPUT_DIR=/home/user/sx9-scans

python3 iac_executor.py --unicode E800 --target 192.168.1.0/24
```

---

## 🎯 REAL-WORLD SCENARIOS

### **Scenario 1: Authorized Penetration Test**

```bash
# Phase 1: Reconnaissance
python3 iac_executor.py \
  --chain E800,E810 \
  --target client-network.com

# Review results, get approval

# Phase 2: Vulnerability Testing
python3 iac_executor.py \
  --chain E820,E825 \
  --target client-network.com

# Review vulnerabilities, get approval

# Phase 3: Controlled Exploitation
python3 iac_executor.py \
  --unicode E830 \
  --target specific-vulnerable-host

# Document everything, deliver report
```

### **Scenario 2: Bug Bounty Hunting**

```bash
# Quick recon on target
python3 iac_executor.py \
  --chain E800,E810,E815 \
  --target bugbounty-target.com

# Results show potential SQLi

# Deep dive with SQLMap
python3 iac_executor.py \
  --unicode E820 \
  --target bugbounty-target.com/vuln-endpoint

# Submit findings with proof
```

### **Scenario 3: Security Research**

```bash
# Test new exploit against honeypot
python3 iac_executor.py \
  --unicode E900 \
  --target honeypot.research.local

# Ephemeral container = no contamination
# Results preserved = reproducible research
```

---

## ⚡ PERFORMANCE

```
OPERATION               | TIME
------------------------|----------
Data pull (KV cache)    | 3-5ms
Data pull (Supabase)    | 50ms
Terraform generation    | <100ms
Container spawn         | 2-5s
Total (single tool)     | ~8s
Total (tool chain)      | ~10s + execution time
```

---

## 💰 COST

```
COMPONENT               | COST
------------------------|----------
CloudFlare KV reads     | FREE (100k/day)
Supabase queries        | FREE (500MB)
Docker containers       | $0 (local)
Terraform               | $0 (OSS)
Cleanup                 | Automatic

TOTAL: $0/month
```

---

## 🐛 TROUBLESHOOTING

### **"Tool E800 not found"**

```bash
# Check CloudFlare Worker is accessible
curl https://sx9-backend.usneodcp.workers.dev/api/health

# Check tool exists
curl https://sx9-backend.usneodcp.workers.dev/api/tool/E800
```

### **"Terraform not found"**

```bash
# Install Terraform
brew install terraform  # macOS
sudo apt install terraform  # Linux

# Verify
terraform --version
```

### **"Docker daemon not running"**

```bash
# Start Docker
sudo systemctl start docker  # Linux
open -a Docker  # macOS
```

### **"Permission denied: /var/sx9"**

```bash
# Create output directory with permissions
sudo mkdir -p /var/sx9/runs
sudo chown $USER:$USER /var/sx9
```

---

## ✅ SUCCESS CRITERIA

After running:
- [ ] Tool manifest pulled (< 10ms)
- [ ] Terraform config generated
- [ ] Docker container spawned
- [ ] Tool executed successfully
- [ ] Results saved to output dir
- [ ] Cleanup scheduled

---

## 🎉 YOU'RE READY!

```bash
# Try it now!
python3 iac_executor.py --unicode E800 --target 8.8.8.8

# That's it! Infrastructure materialized from Unicode! 🚀
```

---

**Need Help?**
- Check logs: `/var/sx9/runs/*/terraform/*.log`
- Review Terraform: `/var/sx9/runs/*/terraform/main.tf`
- Inspect containers: `docker ps -a | grep sx9`
