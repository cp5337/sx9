# RFC-9006: CTAS GIS UI Specification

**Status:** Draft
**Created:** 2025-11-27
**Updated:** 2025-11-27
**Authors:** CTAS Engineering Team
**Domain:** sx9.io

## Implementation Status

| Implementation | Stack | Status | Port |
|----------------|-------|--------|------|
| `ctas7-ops-main-platform` | Vite, React, Mapbox GL JS | **WORKING** | 15174 |
| `sx9-v0` | Next.js 14, deck.gl, kepler.gl | BUILD ISSUES | - |
| `sx9-mobile` | React Native | CANONICAL (mobile) | - |
| `ctas7-foundation-core/src/gis.rs` | Rust | CANONICAL (backend) | - |

**References:**
- `ctas7-ops-main-platform` - **WORKING** Web GIS (Vite, React, Mapbox GL JS)
- `sx9-v0` - EXPERIMENTAL Web GIS (Next.js, deck.gl - has version conflicts)
- `sx9-mobile` - iPad Evidence Collection (React Native)
- `ctas7-foundation-core/src/gis.rs` - Rust GIS primitives
- `ctas7-dioxus-bootstrap` - Rust/WASM design system
- `ctas7-intelligent-transpiler` - React Native → iOS pipeline

> **Note:** sx9-v0 deck.gl/luma.gl "latest" versions cause ShaderAssembler import errors. Use ctas7-ops-main-platform for working GIS until version pinning is resolved.

---

## 1. Executive Summary

RFC-9006 defines a machine-readable UI specification for CTAS GIS visualization components. This specification enables automatic code generation across platforms (Web, iOS, Android) using the existing bolt.diy → Intelligent Transpiler pipeline.

### Pipeline Integration

```
┌─────────────┐     ┌───────────────┐     ┌──────────────────┐     ┌───────────┐
│  RFC-9006   │────▶│   bolt.diy    │────▶│   Intelligent    │────▶│  Native   │
│ GIS Spec    │     │  (Port 5173)  │     │   Transpiler     │     │   Apps    │
│ (JSON/YAML) │     │               │     │  (Port 19000)    │     │           │
└─────────────┘     └───────────────┘     └──────────────────┘     └───────────┘
       │                    │                      │                     │
       │                    ▼                      ▼                     ▼
       │             React/React Native      SwiftUI/UIKit           iOS/Android
       │                                                              macOS
       │
       └─────▶ Design Tokens + Component Specs + Form Factor Variants
```

---

## 2. Design Tokens

### 2.1 Base Token Schema

Location: `design-tokens/gis-tokens.json`

```json
{
  "$schema": "https://ctas.dev/schemas/design-tokens-v1.json",
  "version": "1.0.0",
  "metadata": {
    "name": "CTAS GIS Design System",
    "domain": "gis-visualization"
  },

  "color": {
    "gis": {
      "groundStation": {
        "online": { "value": "#10b981", "description": "Operational station" },
        "offline": { "value": "#ef4444", "description": "Offline station" },
        "degraded": { "value": "#f59e0b", "description": "Degraded performance" },
        "unknown": { "value": "#6b7280", "description": "Status unknown" }
      },
      "satellite": {
        "active": { "value": "#06b6d4", "description": "Active satellite" },
        "inactive": { "value": "#64748b", "description": "Inactive satellite" }
      },
      "networkLink": {
        "active": { "value": "#06b6d4", "alpha": 0.5 },
        "degraded": { "value": "#f59e0b", "alpha": 0.3 },
        "inactive": { "value": "#ef4444", "alpha": 0.2 }
      },
      "orbit": {
        "path": { "value": "#0ea5e9", "alpha": 0.5 }
      },
      "world": {
        "production": { "value": "#10b981" },
        "staging": { "value": "#f59e0b" },
        "sandbox": { "value": "#8b5cf6" },
        "fusion": { "value": "#ec4899" }
      }
    },
    "surface": {
      "panel": { "value": "#0f172a", "alpha": 0.95 },
      "panelHover": { "value": "#1e293b" },
      "overlay": { "value": "#000000", "alpha": 0.8 }
    },
    "text": {
      "primary": { "value": "#e2e8f0" },
      "secondary": { "value": "#94a3b8" },
      "accent": { "value": "#38bdf8" }
    }
  },

  "spacing": {
    "xs": { "value": 4, "unit": "px" },
    "sm": { "value": 8, "unit": "px" },
    "md": { "value": 16, "unit": "px" },
    "lg": { "value": 24, "unit": "px" },
    "xl": { "value": 32, "unit": "px" },
    "2xl": { "value": 48, "unit": "px" },
    "3xl": { "value": 64, "unit": "px" }
  },

  "touchTarget": {
    "minimum": { "value": 44, "unit": "px", "description": "iOS HIG minimum" },
    "vehicle": { "value": 56, "unit": "px", "description": "Vehicle-mounted displays" },
    "gloved": { "value": 64, "unit": "px", "description": "Gloved operation" }
  },

  "typography": {
    "fontFamily": {
      "primary": { "value": "Inter, system-ui, sans-serif" },
      "mono": { "value": "JetBrains Mono, monospace" }
    },
    "fontSize": {
      "xs": { "value": 12, "unit": "px" },
      "sm": { "value": 14, "unit": "px" },
      "base": { "value": 16, "unit": "px" },
      "lg": { "value": 18, "unit": "px" },
      "xl": { "value": 20, "unit": "px" },
      "2xl": { "value": 24, "unit": "px" }
    }
  },

  "borderRadius": {
    "sm": { "value": 4, "unit": "px" },
    "md": { "value": 8, "unit": "px" },
    "lg": { "value": 12, "unit": "px" },
    "full": { "value": 9999, "unit": "px" }
  },

  "shadow": {
    "panel": {
      "value": "0 4px 6px -1px rgba(0, 0, 0, 0.3), 0 2px 4px -1px rgba(0, 0, 0, 0.2)",
      "ios": { "shadowColor": "#000", "shadowOffset": { "width": 0, "height": 4 }, "shadowOpacity": 0.3, "shadowRadius": 6 }
    }
  },

  "animation": {
    "duration": {
      "fast": { "value": 150, "unit": "ms" },
      "normal": { "value": 250, "unit": "ms" },
      "slow": { "value": 400, "unit": "ms" }
    },
    "easing": {
      "default": { "value": "cubic-bezier(0.4, 0, 0.2, 1)" }
    }
  },

  "zIndex": {
    "globe": { "value": 0 },
    "panel": { "value": 100 },
    "overlay": { "value": 200 },
    "modal": { "value": 300 },
    "toast": { "value": 400 }
  }
}
```

---

## 3. Form Factor Variants

### 3.1 Form Factor Definitions

```json
{
  "formFactors": {
    "ipadOperator": {
      "id": "ipad-operator",
      "displayName": "iPad Operator",
      "description": "Tactical field operator with iPad",
      "constraints": {
        "minWidth": 768,
        "maxWidth": 1366,
        "touchTarget": 44,
        "orientation": ["landscape", "portrait"]
      },
      "features": {
        "voiceControl": true,
        "gestureNavigation": true,
        "offlineCapable": true
      }
    },
    "vehicleMounted": {
      "id": "vehicle-mounted",
      "displayName": "Vehicle Mounted Display",
      "description": "Ruggedized vehicle-mounted terminal",
      "constraints": {
        "minWidth": 1024,
        "maxWidth": 1920,
        "touchTarget": 56,
        "orientation": ["landscape"]
      },
      "features": {
        "voiceControl": true,
        "gestureNavigation": false,
        "highContrast": true,
        "glareResistant": true
      }
    },
    "analystWorkstation": {
      "id": "analyst",
      "displayName": "Analyst Workstation",
      "description": "Desktop mouse/keyboard interface",
      "constraints": {
        "minWidth": 1440,
        "maxWidth": null,
        "touchTarget": 32,
        "orientation": ["landscape"]
      },
      "features": {
        "keyboardShortcuts": true,
        "multiMonitor": true,
        "densePanels": true
      }
    },
    "opsCenter": {
      "id": "ops-center",
      "displayName": "Operations Center",
      "description": "Multi-display operations center wall",
      "constraints": {
        "minWidth": 3840,
        "maxWidth": null,
        "touchTarget": 44,
        "multiDisplay": true
      },
      "features": {
        "roleBasedViews": true,
        "realTimeSync": true,
        "alertPrioritization": true
      }
    }
  }
}
```

---

## 4. Component Specifications

### 4.1 GIS Panel Component

```json
{
  "component": "GISPanel",
  "description": "Collapsible side panel for GIS controls",
  "category": "layout",

  "props": {
    "side": { "type": "enum", "values": ["left", "right"], "default": "left" },
    "collapsed": { "type": "boolean", "default": false },
    "width": { "type": "number", "default": 280 },
    "collapsedWidth": { "type": "number", "default": 64 }
  },

  "variants": {
    "ipadOperator": {
      "width": 320,
      "collapsedWidth": 72,
      "touchTarget": "{{ touchTarget.minimum }}"
    },
    "vehicleMounted": {
      "width": 360,
      "collapsedWidth": 80,
      "touchTarget": "{{ touchTarget.vehicle }}"
    },
    "analyst": {
      "width": 280,
      "collapsedWidth": 48,
      "resizable": true
    },
    "opsCenter": {
      "width": 400,
      "collapsedWidth": 64,
      "detachable": true
    }
  },

  "slots": {
    "header": { "required": false },
    "content": { "required": true },
    "footer": { "required": false }
  },

  "accessibility": {
    "role": "complementary",
    "ariaLabel": "GIS Control Panel",
    "keyboardNav": true
  },

  "transpilation": {
    "react": "src/components/GISPanel.tsx",
    "reactNative": "src/components/GISPanel.native.tsx",
    "swiftui": "Sources/Components/GISPanelView.swift",
    "dioxus": "src/components/gis_panel.rs"
  }
}
```

### 4.2 World Selector Component

```json
{
  "component": "WorldSelector",
  "description": "Multi-world environment selector",
  "category": "control",

  "props": {
    "currentWorld": {
      "type": "enum",
      "values": ["production", "staging", "sandbox", "fusion"],
      "required": true
    },
    "onWorldChange": { "type": "function", "required": true }
  },

  "dataContract": {
    "worlds": {
      "type": "array",
      "items": {
        "id": "WorldType",
        "label": "string",
        "color": "string",
        "icon": "string",
        "stats": {
          "groundStations": "number",
          "satellites": "number",
          "activeLinks": "number"
        }
      }
    }
  },

  "variants": {
    "ipadOperator": {
      "layout": "horizontal-pills",
      "showStats": true
    },
    "vehicleMounted": {
      "layout": "large-buttons",
      "showStats": false,
      "voiceCommands": ["switch to production", "switch to staging"]
    },
    "analyst": {
      "layout": "dropdown",
      "showStats": true,
      "keyboardShortcut": "Cmd+1-4"
    }
  },

  "transpilation": {
    "react": "src/components/WorldSelector.tsx",
    "swiftui": "Sources/Components/WorldSelectorView.swift"
  }
}
```

### 4.3 Layer Control Component

```json
{
  "component": "LayerControl",
  "description": "Hierarchical layer visibility and opacity control",
  "category": "control",

  "props": {
    "layers": { "type": "LayerConfig[]", "required": true },
    "onToggle": { "type": "function", "required": true },
    "onOpacityChange": { "type": "function", "required": true }
  },

  "dataContract": {
    "LayerConfig": {
      "id": "string",
      "label": "string",
      "visible": "boolean",
      "opacity": "number (0-1)",
      "color": "string",
      "children": "LayerConfig[]?"
    }
  },

  "variants": {
    "ipadOperator": {
      "expandable": true,
      "showOpacity": true,
      "gestureControl": true
    },
    "vehicleMounted": {
      "expandable": false,
      "showOpacity": false,
      "quickToggles": ["groundStations", "satellites"]
    },
    "analyst": {
      "expandable": true,
      "showOpacity": true,
      "batchOperations": true,
      "keyboardShortcuts": true
    }
  },

  "transpilation": {
    "react": "src/components/LayerControl.tsx",
    "swiftui": "Sources/Components/LayerControlView.swift"
  }
}
```

---

## 5. CesiumWorldManager Integration

### 5.1 Manager Interface (Canonical)

```typescript
interface CesiumWorldManager {
  // World management
  switchWorld(world: WorldType): void;
  getCurrentWorld(): WorldType;
  getWorldConfig(world: WorldType): WorldConfig;

  // Entity management
  addGroundStation(world: WorldType, station: GroundStationData): void;
  addSatellite(world: WorldType, satellite: SatelliteData): void;
  addNetworkLink(world: WorldType, link: NetworkLinkData): void;

  // Layer control
  setLayerVisibility(layerId: string, visible: boolean): void;
  setLayerOpacity(layerId: string, opacity: number): void;

  // Events
  getEventBus(): EventTarget;

  // Lifecycle
  destroy(): void;
}

type WorldType = 'production' | 'staging' | 'sandbox' | 'fusion';

interface WorldConfig {
  layers: Record<string, boolean>;
  camera: { lon: number; lat: number; height: number };
  timeScale: number;
}

interface GroundStationData {
  id: string;
  name: string;
  latitude: number;
  longitude: number;
  altitude?: number;
  status?: 'online' | 'offline' | 'degraded' | 'operational';
}

interface SatelliteData {
  id: string;
  name: string;
  tle?: [string, string];
  norad_id?: number;
}

interface NetworkLinkData {
  id: string;
  source_id: string;
  target_id: string;
  status: 'active' | 'degraded' | 'inactive';
}
```

### 5.2 Event Bus Contract

```typescript
// Events emitted by CesiumWorldManager
interface GISEventMap {
  'entity-selected': {
    id: string;
    name: string;
    type: 'ground_station' | 'satellite' | 'network_link';
    world: WorldType;
  };
  'world-changed': {
    world: WorldType;
  };
  'time-update': {
    currentTime: string;
    multiplier: number;
  };
  'layer-changed': {
    layerId: string;
    visible: boolean;
    opacity: number;
  };
}
```

---

## 6. bolt.diy Code Generation

### 6.1 Generation Request

When RFC-9006 spec files change, trigger bolt.diy generation:

```json
{
  "prompt": "Generate GIS components from RFC-9006 specification",
  "framework": "react-native",
  "style_system": "tailwind",
  "typescript": true,
  "design_tokens": "file://design-tokens/gis-tokens.json",
  "component_specs": "file://03-docs/rfcs/RFC-9006-GIS-UI-SPECIFICATION.md",
  "target_platform": ["web", "mobile", "ios"],
  "form_factor": "ipad-operator"
}
```

### 6.2 Generation Pipeline

```yaml
# .github/workflows/rfc-9006-codegen.yml
name: RFC-9006 Code Generation

on:
  push:
    paths:
      - '03-docs/rfcs/RFC-9006-*.md'
      - 'design-tokens/gis-tokens.json'

jobs:
  generate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Generate React Components
        run: |
          curl -X POST http://bolt-diy:5173/api/generate \
            -H "Content-Type: application/json" \
            -d @.github/rfc-9006-generation-request.json

      - name: Transpile to iOS
        run: |
          curl -X POST http://transpiler:19000/api/transpile \
            -H "Content-Type: application/json" \
            -d '{"project_id": "${{ steps.generate.outputs.project_id }}", "target": "ios"}'

      - name: Create PR
        uses: peter-evans/create-pull-request@v5
        with:
          title: "chore: Update GIS components from RFC-9006"
          branch: auto/rfc-9006-codegen
```

---

## 7. Transpilation Mappings

### 7.1 React Native → SwiftUI

```json
{
  "mappings": {
    "View": "VStack/HStack/ZStack",
    "TouchableOpacity": "Button",
    "Text": "Text",
    "ScrollView": "ScrollView",
    "FlatList": "List/ForEach",
    "Image": "Image",
    "Switch": "Toggle",
    "Slider": "Slider",
    "TextInput": "TextField"
  },
  "styleMapping": {
    "flexDirection: row": "HStack",
    "flexDirection: column": "VStack",
    "position: absolute": ".overlay()",
    "backgroundColor": ".background()",
    "borderRadius": ".cornerRadius()",
    "padding": ".padding()",
    "margin": "Spacer() / .padding()"
  }
}
```

### 7.2 React Native → Dioxus (Rust)

```json
{
  "mappings": {
    "View": "div",
    "TouchableOpacity": "button",
    "Text": "span/p",
    "ScrollView": "div[overflow-y: auto]",
    "Image": "img",
    "Switch": "input[type=checkbox]",
    "Slider": "input[type=range]"
  },
  "hooks": {
    "useState": "use_signal",
    "useEffect": "use_effect",
    "useRef": "use_signal"
  }
}
```

---

## 8. Implementation Status

### Working Implementation (ctas7-ops-main-platform)

| Component | Status | File |
|-----------|--------|------|
| WorkingMap | ✅ WORKING | `src/components/WorkingMap.tsx` |
| EnhancedMap | ✅ WORKING | `src/components/EnhancedMap.tsx` |
| NetworkMap | ✅ WORKING | `src/components/NetworkMap.tsx` |
| GeospatialDataManager | ✅ WORKING | `src/components/GeospatialDataManager.tsx` |
| TopAPTs | ✅ WORKING | `src/components/TopAPTs.tsx` |
| Shodan | ✅ WORKING | `src/components/Shodan.tsx` |

**Stack:** Vite + React + Mapbox GL JS (dark-v11 style)
**Port:** 15174 (dev server)

### Experimental (sx9-v0 - BUILD ISSUES)

| Component | React (sx9-v0) | React Native (sx9-mobile) | SwiftUI | Dioxus |
|-----------|----------------|---------------------------|---------|--------|
| GISPanel | ⚠️ `gis-control-panel.tsx` | Pending | Pending | Pending |
| WorldSelector | ⚠️ `mode-switcher.tsx` | Pending | Pending | Pending |
| LayerControl | ⚠️ `layer-panel.tsx` | Pending | Pending | Pending |
| CesiumGlobe | ⚠️ `cesium-globe.tsx` | N/A (native maps) | MapKit | N/A |
| SatelliteTracker | ⚠️ `satellite-tracker-panel.tsx` | Pending | Pending | Pending |
| FlightTracker | ⚠️ `flight-tracker-panel.tsx` | Pending | Pending | Pending |
| GraphAnalytics | ⚠️ `graph-analytics-panel.tsx` | Pending | Pending | Pending |
| LocationHashDisplay | ⚠️ `location-hash-display.tsx` | ✅ `useLocation.js` | Pending | Pending |

> ⚠️ sx9-v0 components exist but build fails due to deck.gl/luma.gl version conflicts

---

## 9. Related Files

### Working GIS (USE THIS)
- `ctas7-ops-main-platform/` - Mapbox GL JS (Vite, React, port 15174)
  - `src/components/WorkingMap.tsx` - Basic Mapbox map
  - `src/components/EnhancedMap.tsx` - Enhanced features
  - `src/components/NetworkMap.tsx` - Network visualization

### Rust GIS Foundation (CANONICAL)
- `ctas7-foundation-core/src/gis.rs` - point_m, haversine_m, inside
- `ctas7-slotgraph-engine/src/gis.rs` - ECS GIS integration

### Do Not Touch (Orbital/Command Center)
- `sx9-v0/` - deck.gl/kepler.gl/Cesium (experimental, build issues)
- `sx9-mobile/` - iPad Evidence Collection (React Native)

### Archived
- `google-drive-repository/archived-md-files/ctas7-gis-cesium*`
- `ctas-gis-nextjs/` - Archived (Turbopack conflicts)

---

## 10. Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-11-27 | Initial RFC-9006 specification |
| 1.1.0 | 2025-11-27 | Aligned with working Mapbox implementation (ops-main-platform) |

---

**RFC-9006 Status:** Draft - Awaiting Review
