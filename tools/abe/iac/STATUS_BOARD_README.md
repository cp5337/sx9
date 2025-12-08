# SX9 (Synaptix9) System Status Board

Quick ASCII dashboard for monitoring SX9 system status, pipeline execution, and service health.

## 📋 **Usage**

### **Quick Status Check**
```bash
cd 04-abe-iac
./status_board.sh    # Bash version
# OR
./status_board.py    # Python version (recommended)
```

### **Auto-refresh (watch mode)**
```bash
watch -n 5 ./status_board.py
```

## 🎯 **What It Shows**

### **1. Pipeline Status**
- Current pipeline execution state (running/completed/not running)
- Phase completion status:
  - Download (threat content)
  - SPIRES (ontology generation)
  - DSL Conversion
  - Storage (upload to Supabase/CDN)

### **2. Data Status**
- Threat Content size and file count
- Ontology size and file count
- DSL Conversion status
- Task Graph status

### **3. Integration Plans**
- Checks for existence of integration plan documents:
  - Unified Plan
  - OSINT Plan
  - Kali Plan
  - Threat Intel Plan

### **4. Services Status**
- Neo4j (Bolt port 7687, HTTP port 7474)
- GLAF (port 18050)
- Supabase (configuration check)
- Docker (container count)

### **5. Recent Activity**
- Latest log file
- Last 3 lines of recent activity

### **6. Quick Actions**
- Commands to run pipeline, storage, or watch logs

## 🔧 **Extending the Status Board**

### **Adding New Checks (Python Version)**

Edit `status_board.py`:

```python
def check_custom_service(self) -> Dict:
    """Add your custom service check"""
    return {
        "name": "Custom Service",
        "port": 8080,
        "running": self.check_port("localhost", 8080)
    }

# Then add to render() method:
services = self.check_services()
services["custom"] = self.check_custom_service()
```

### **Adding New Data Sources**

```python
def check_custom_data(self) -> Dict:
    """Check custom data directory"""
    custom_dir = self.output_dir / "custom_data"
    if custom_dir.exists():
        size, count = self.get_directory_size(custom_dir)
        return {"status": "complete", "size": size, "count": count}
    return {"status": "not_found"}
```

## 📊 **Output Format**

The status board uses:
- ✅ Green checkmarks for success
- ❌ Red X for failures
- ⚠️ Yellow warnings for partial states
- Color-coded status indicators

## 🔄 **Integration with CI/CD**

You can integrate the status board into automation:

```bash
# Check if pipeline is running
if ./status_board.py | grep -q "RUNNING"; then
    echo "Pipeline is active"
fi

# Check if all services are up
if ./status_board.py | grep -q "NOT RUNNING"; then
    echo "Some services are down"
fi
```

## 📝 **Notes**

- **Bash version** (`status_board.sh`): Faster, no dependencies
- **Python version** (`status_board.py`): More maintainable, easier to extend
- Both versions provide the same output format
- Python version is recommended for future enhancements

## 🚀 **Future Enhancements**

Potential additions:
- [ ] Real-time WebSocket updates
- [ ] JSON output mode (`--json`)
- [ ] Historical trend tracking
- [ ] Alert thresholds
- [ ] Integration with monitoring systems (Prometheus, Grafana)
- [ ] Email/Slack notifications

