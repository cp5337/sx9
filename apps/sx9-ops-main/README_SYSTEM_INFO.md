# CTAS Main Ops v7.3.1 - System Information

## ✅ THIS IS THE WORKING SYSTEM

**Directory:** `ctas6-reference/` (DO NOT RENAME - server depends on this path)

**Version:** 7.3.1 (originally v6.6, upgraded to v7.3.1)

**Status:** ✅ ACTIVE & WORKING

---

## 🚀 Running the System

```bash
cd /Users/cp5337/Developer/ctas-7-shipyard-staging/ctas6-reference
npm run dev
```

**Port:** 15174

**URLs:**
- Main: http://localhost:15174
- Hunt: http://localhost:15174/hunt
- Detect: http://localhost:15174/detect
- Disrupt: http://localhost:15174/disrupt
- Disable: http://localhost:15174/disable
- Dominate: http://localhost:15174/dominate

---

## 📦 What This System Has

- ✅ Complete HD4 phases (Hunt, Detect, Disrupt, Disable, Dominate)
- ✅ Mapbox GL integration with dark theme
- ✅ Neural Mux client (SynaptixCoreClient.ts → port 50051)
- ✅ Full database connectivity:
  - SurrealDB (port 8000)
  - Supabase
  - Sled KVS
  - SlotGraph
- ✅ GIS maps working on all pages
- ✅ Enhanced map with unique container IDs
- ✅ Demo data tracking system

---

## 🔧 Recent Fixes

1. **GIS Map Fix** (commit c63daf1)
   - Fixed container ID collision
   - Each map instance has unique ID
   - Maps load correctly on all HD4 pages

2. **MaxMind GeoIP Fix** (commit 60f0d07)
   - Removed Node.js-only @maxmind/geoip2-node package
   - Frontend uses mock data via geoipService.ts
   - Real GeoIP via backend API (port 18122)

---

## ⚠️ Important Notes

- **DO NOT RENAME THIS DIRECTORY** - The running Vite server depends on the path
- This is the canonical v7.3.1 system
- The old `ctas-7.0-main-ops-platform` is broken/offline
- All development should happen in this directory

---

## 🗂️ Other CTAS Systems

**Broken/Offline:**
- `ctas-7.0-main-ops-platform/` - Old v7.0 attempt (broken, do not use)

**Working:**
- `ctas6-reference/` - **THIS ONE** ✅

---

**Last Updated:** 2025-11-09

