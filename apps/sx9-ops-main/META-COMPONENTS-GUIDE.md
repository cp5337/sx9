# Meta Components Guide

**Created:** 2025-01-27  
**Purpose:** Reusable components to eliminate button-by-button programming

---

## 🎯 Overview

Instead of programming 13+ buttons individually on each page, use these meta components that provide consistent functionality across all pages.

---

## 📦 Available Meta Components

### 1. **DataVisualizationToolbar**

**Location:** `src/components/shared/toolbars/DataVisualizationToolbar.tsx`

**Purpose:** Provides all 13 common data visualization actions that appear on every page.

**Actions Included:**
- Network View
- Task Graph
- Sectors
- Filters
- Data Sources
- GIS Layers
- OSINT Nodes
- Threat Intel
- Infrastructure
- GeoIP
- Supabase
- SurrealDB
- GEE (KMZ)

**Usage:**
```tsx
import { DataVisualizationToolbar } from '@/components/shared';

<DataVisualizationToolbar
  onNetworkView={() => handleNetworkView()}
  onTaskGraph={() => handleTaskGraph()}
  onFilters={() => handleFilters()}
  variant="compact" // "default" | "compact" | "minimal"
  enabledActions={['networkView', 'taskGraph', 'filters']} // Optional whitelist
/>
```

**Variants:**
- `default` - Full toolbar with icons and labels
- `compact` - Icon-only buttons
- `minimal` - Dropdown menu

---

### 2. **DatabaseConnectionPanel**

**Location:** `src/components/shared/toolbars/DatabaseConnectionPanel.tsx`

**Purpose:** Unified database connection management with status monitoring.

**Databases Supported:**
- Supabase (port 3000)
- SurrealDB (port 8000)
- Sled KVR
- SlotGraph

**Usage:**
```tsx
import { DatabaseConnectionPanel } from '@/components/shared';

<DatabaseConnectionPanel
  onConnect={(dbId) => handleConnect(dbId)}
  onDisconnect={(dbId) => handleDisconnect(dbId)}
  onRefresh={(dbId) => handleRefresh(dbId)}
  compact={false} // true for icon-only buttons
  showStatus={true}
/>
```

**Features:**
- Connection status monitoring
- Connect/disconnect/refresh actions
- Health check integration
- Visual status indicators

---

### 3. **FilterPanel**

**Location:** `src/components/shared/toolbars/FilterPanel.tsx`

**Purpose:** Advanced filtering with sectors, HD4 phases, priorities, and search.

**Usage:**
```tsx
import { FilterPanel } from '@/components/shared';

<FilterPanel
  onFilterChange={(filters) => handleFilterChange(filters)}
  onSectorChange={(sectors) => handleSectorChange(sectors)}
  onSearchChange={(query) => handleSearch(query)}
  compact={false} // true for popover mode
  filters={customFilters} // Optional custom filter groups
/>
```

**Default Filters:**
- Sectors (Energy, Finance, Healthcare, etc.)
- HD4 Phase (Hunt, Detect, Disrupt, Disable, Dominate)
- Priority (Critical, High, Medium, Low)

**Features:**
- Multi-select support
- Search integration
- Active filter count
- Clear all functionality

---

## 🚀 Quick Start

### Step 1: Import Components

```tsx
import {
  DataVisualizationToolbar,
  DatabaseConnectionPanel,
  FilterPanel
} from '@/components/shared';
```

### Step 2: Add to Your Page

```tsx
export const MyPage: React.FC = () => {
  return (
    <div>
      {/* Toolbar at top */}
      <DataVisualizationToolbar
        variant="compact"
        onNetworkView={() => showNetworkView()}
        onTaskGraph={() => showTaskGraph()}
      />

      {/* Filters on the side */}
      <FilterPanel
        compact
        onFilterChange={(filters) => applyFilters(filters)}
      />

      {/* Database connections */}
      <DatabaseConnectionPanel
        compact
        onConnect={(dbId) => connectDatabase(dbId)}
      />

      {/* Your page content */}
    </div>
  );
};
```

---

## 📊 Impact

### Before (Button-by-Button):
- 13 buttons × 10 pages = 130 individual button implementations
- Inconsistent UI across pages
- Duplicate code
- Hard to maintain

### After (Meta Components):
- 3 components used across all pages
- Consistent UI everywhere
- Single source of truth
- Easy to update globally

---

## 🎨 Customization

### Enable/Disable Actions

```tsx
<DataVisualizationToolbar
  enabledActions={['networkView', 'taskGraph', 'filters']}
  // Only these 3 actions will be shown
/>
```

### Custom Filters

```tsx
<FilterPanel
  filters={[
    {
      id: 'custom',
      label: 'Custom Filter',
      options: [
        { id: 'opt1', label: 'Option 1', value: 'opt1' },
        { id: 'opt2', label: 'Option 2', value: 'opt2' }
      ],
      multiSelect: true
    }
  ]}
/>
```

### Custom Database List

```tsx
<DatabaseConnectionPanel
  databases={[
    {
      id: 'custom-db',
      name: 'Custom Database',
      type: 'supabase',
      port: 5432,
      status: 'disconnected'
    }
  ]}
/>
```

---

## 📝 Example Page

See `src/components/shared/toolbars/MetaComponentsExample.tsx` for a complete example showing all variants and usage patterns.

---

## ✅ Benefits

1. **Consistency** - Same UI across all pages
2. **Efficiency** - No need to program buttons individually
3. **Maintainability** - Update once, applies everywhere
4. **Flexibility** - Multiple variants and customization options
5. **Type Safety** - Full TypeScript support

---

## 🔗 Related Files

- `BUTTONS-FUNCTIONS-WORK-REQUIRED.md` - Original gap analysis
- `IMPLEMENTATION-PLAN.md` - Implementation strategy
- `src/components/shared/index.ts` - Export file
- `src/components/shared/toolbars/` - Component directory

---

**Next Steps:** Start replacing individual buttons with these meta components across all pages!



