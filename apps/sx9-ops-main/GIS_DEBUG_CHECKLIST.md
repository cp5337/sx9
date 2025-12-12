# GIS Not Showing - Debug Checklist

## 🔍 User Report
**Issue:** No GIS showing on http://localhost:15174/hunt

---

## ✅ What We've Verified

1. **Server Status**
   - ✅ Server running on port 15174 (PID 84758)
   - ✅ Page loads successfully
   - ✅ Mapbox token is set in .env

2. **Code Structure**
   - ✅ Hunt.tsx → HD4PhaseContent → HD4Map → EnhancedMap
   - ✅ View prop defaults to 'map'
   - ✅ Navigation has map/grid/graph/cognigraph toggles
   - ✅ EnhancedMap has unique container IDs (fixed)

3. **Recent Fixes**
   - ✅ Container ID collision fixed (commit c63daf1)
   - ✅ MaxMind package removed (commit 60f0d07)

---

## 🔍 Things to Check in Browser

### 1. **Is the Map Icon Selected?**
Look at the top navigation bar - there should be 4 icons:
- 🗺️ Map (should be blue/highlighted)
- 📊 Grid
- 🔗 Graph
- 🧠 Cognigraph

**Action:** Click the Map icon if it's not selected

### 2. **Is the Overview Tab Selected?**
Look at the tabs below the navigation:
- Overview (should be blue/highlighted)
- Kali Tools
- Playbooks
- Red Team
- Phase Mapping
- Tasks

**Action:** Click "Overview" tab if not selected

### 3. **Browser Console Errors**
Open browser console (F12) and look for:
- `EnhancedMap: Starting map initialization...`
- `EnhancedMap: Map initialized successfully`
- Any red errors?

### 4. **What Do You See Instead?**
- Blank space where map should be?
- Loading spinner stuck?
- Error message?
- Graph animation instead of map?

---

## 🛠️ Quick Fixes to Try

### Fix 1: Force Map View
In browser console, run:
```javascript
localStorage.setItem('ctas-view', 'map');
location.reload();
```

### Fix 2: Clear Mapbox Cache
In browser console, run:
```javascript
localStorage.clear();
location.reload();
```

### Fix 3: Check Mapbox Token
In browser console, run:
```javascript
console.log('Mapbox Token:', import.meta.env.VITE_MAPBOX_ACCESS_TOKEN);
```

---

## 📊 Expected Behavior

When working correctly, you should see:
1. **Top Navigation:** Map icon highlighted (blue)
2. **Tabs:** "Overview" tab selected
3. **Main Area:** 
   - Top 2/3: Interactive Mapbox map with dark theme
   - Bottom 1/3: Task list for Hunt phase
4. **Right Side:** Layer controls panel (if enabled)

---

## 🚨 Common Issues

### Issue: Graph Shows Instead of Map
**Cause:** View is set to 'graph' instead of 'map'
**Fix:** Click the Map icon in top navigation

### Issue: Loading Spinner Forever
**Cause:** Mapbox token invalid or network issue
**Fix:** Check console for specific error

### Issue: "Map Error" Message
**Cause:** Mapbox initialization failed
**Fix:** Check console for specific error, verify token

### Issue: Blank Space
**Cause:** Container height/width issue or CSS problem
**Fix:** Check browser inspector for element dimensions

---

## 📝 Next Steps

**User:** Please check the browser and tell me:
1. Which icon is highlighted in top navigation? (Map/Grid/Graph/Cognigraph)
2. Which tab is selected? (Overview/Kali/Playbooks/etc)
3. What do you see in the main area?
4. Any errors in browser console (F12)?

This will help me pinpoint the exact issue!

