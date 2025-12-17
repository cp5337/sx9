# Graph Engine Architectural Patterns
## Extracted from React Flow / xyflow Mature Implementation

### Reference for SYNAPTIX Graph Engine Development

---

## 1. CORE ARCHITECTURE: Layered System Design

```
┌─────────────────────────────────────────────────────────────┐
│                    Framework Layer                          │
│            (React Flow / Svelte Flow)                       │
│    ┌─────────────────────────────────────────────────┐     │
│    │              @xyflow/react                       │     │
│    │    - React-specific bindings                    │     │
│    │    - Hooks (useNodes, useEdges, useReactFlow)   │     │
│    │    - Component wrappers                         │     │
│    └─────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Core System Layer                        │
│                     @xyflow/system                          │
│    ┌─────────────────────────────────────────────────┐     │
│    │  Framework-Agnostic Utilities                   │     │
│    │  - XYDrag: Node/selection dragging              │     │
│    │  - XYPanZoom: Viewport control                  │     │
│    │  - XYHandle: Connection line management         │     │
│    │  - XYMiniMap: Overview navigation               │     │
│    │  - Edge path calculations                       │     │
│    │  - Store utilities                              │     │
│    │  - DOM measurement utilities                    │     │
│    └─────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Foundation Layer                          │
│    ┌─────────────────────────────────────────────────┐     │
│    │  d3-zoom      - Pan/zoom/drag interactions      │     │
│    │  d3-selection - DOM manipulation                │     │
│    │  zustand      - State management                │     │
│    └─────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

**SYNAPTIX Implication**: Separate framework-agnostic core from UI bindings. Your Rust core + WASM boundary follows this pattern naturally.

---

## 2. STATE ARCHITECTURE

### 2.1 Dual-Layer Node Model

```typescript
// USER-FACING NODE (what developers work with)
interface Node<T = any> {
    id: string;                           // Required, unique
    type?: string;                        // Node type key
    position: XYPosition;                 // {x, y} - user-controlled
    data: T;                              // Custom payload
    
    // Optional user-settable
    draggable?: boolean;
    selectable?: boolean;
    deletable?: boolean;
    connectable?: boolean;
    hidden?: boolean;
    
    // Read-only (set by system after render)
    measured?: { width: number; height: number };
}

// INTERNAL NODE (system-computed, extends Node)
interface InternalNode<T = any> extends Node<T> {
    internals: {
        positionAbsolute: XYPosition;     // Accounts for parent offsets
        handleBounds: {
            source: HandleBounds[];        // Computed handle positions
            target: HandleBounds[];
        };
        z: number;                         // Calculated z-index
    };
}
```

**Key Pattern**: User nodes are simple. System augments with `internals` containing computed properties. Never pollute user model with internal state.

### 2.2 Central Store Structure (Zustand-based)

```typescript
interface ReactFlowState {
    // Primary Data
    nodes: Node[];
    edges: Edge[];
    
    // Fast Lookups (Map for O(1) access)
    nodeLookup: Map<string, InternalNode>;
    edgeLookup: Map<string, Edge>;
    
    // Viewport State
    viewport: Viewport;                    // {x, y, zoom}
    
    // Interaction State
    connectionStartHandle: ConnectionHandle | null;
    selectedNodes: Node[];
    selectedEdges: Edge[];
    
    // Configuration
    nodeTypes: NodeTypes;
    edgeTypes: EdgeTypes;
    
    // Actions
    setNodes: (nodes: Node[]) => void;
    setEdges: (edges: Edge[]) => void;
    updateNode: (id: string, update: Partial<Node>) => void;
    updateNodeData: (id: string, data: any) => void;
    // ... more actions
}
```

**Critical Pattern**: `nodeLookup` is a Map for O(1) node access by ID. Arrays for ordered iteration, Maps for lookup.

---

## 3. CHANGE EVENT SYSTEM (Most Important Pattern)

### 3.1 Discriminated Union Changes

```typescript
// NODE CHANGES - Discriminated union by 'type' field
type NodeChange =
    | NodeDimensionChange
    | NodePositionChange
    | NodeSelectionChange
    | NodeRemoveChange
    | NodeAddChange
    | NodeReplaceChange;

interface NodeDimensionChange {
    type: 'dimensions';
    id: string;
    dimensions: { width: number; height: number };
    resizing?: boolean;
    setAttributes?: boolean | 'width' | 'height';
}

interface NodePositionChange {
    type: 'position';
    id: string;
    position?: XYPosition;
    positionAbsolute?: XYPosition;
    dragging?: boolean;
}

interface NodeSelectionChange {
    type: 'select';
    id: string;
    selected: boolean;
}

interface NodeRemoveChange {
    type: 'remove';
    id: string;
}

interface NodeAddChange {
    type: 'add';
    item: Node;
    index?: number;
}

interface NodeReplaceChange {
    type: 'replace';
    id: string;
    item: Node;
}
```

### 3.2 Edge Changes (Same Pattern)

```typescript
type EdgeChange =
    | EdgeAddChange
    | EdgeRemoveChange
    | EdgeSelectionChange
    | EdgeReplaceChange;
```

### 3.3 Change Application Pattern

```typescript
// Pure function - takes changes, returns new state
function applyNodeChanges(changes: NodeChange[], nodes: Node[]): Node[] {
    const result = [...nodes];
    
    for (const change of changes) {
        switch (change.type) {
            case 'add':
                // Insert at index or push
                if (change.index !== undefined) {
                    result.splice(change.index, 0, change.item);
                } else {
                    result.push(change.item);
                }
                break;
                
            case 'remove':
                const removeIdx = result.findIndex(n => n.id === change.id);
                if (removeIdx !== -1) result.splice(removeIdx, 1);
                break;
                
            case 'position':
                const posNode = result.find(n => n.id === change.id);
                if (posNode && change.position) {
                    posNode.position = change.position;
                }
                break;
                
            case 'dimensions':
                const dimNode = result.find(n => n.id === change.id);
                if (dimNode) {
                    dimNode.measured = change.dimensions;
                }
                break;
                
            case 'select':
                const selNode = result.find(n => n.id === change.id);
                if (selNode) {
                    selNode.selected = change.selected;
                }
                break;
                
            case 'replace':
                const replaceIdx = result.findIndex(n => n.id === change.id);
                if (replaceIdx !== -1) {
                    result[replaceIdx] = change.item;
                }
                break;
        }
    }
    
    return result;
}
```

**SYNAPTIX Implication**: This pattern is perfect for:
- Undo/redo (changes are reversible)
- Collaborative editing (changes are sync-able)
- Event sourcing
- Batched updates

---

## 4. VIEWPORT MANAGEMENT

### 4.1 Viewport State

```typescript
interface Viewport {
    x: number;      // Pan offset X
    y: number;      // Pan offset Y
    zoom: number;   // Zoom level (1.0 = 100%)
}
```

### 4.2 Coordinate Transforms

```typescript
// Screen coords → Flow coords
function screenToFlowPosition(
    screenPos: XYPosition,
    viewport: Viewport,
    domRect: DOMRect
): XYPosition {
    return {
        x: (screenPos.x - domRect.left - viewport.x) / viewport.zoom,
        y: (screenPos.y - domRect.top - viewport.y) / viewport.zoom,
    };
}

// Flow coords → Screen coords
function flowToScreenPosition(
    flowPos: XYPosition,
    viewport: Viewport,
    domRect: DOMRect
): XYPosition {
    return {
        x: flowPos.x * viewport.zoom + viewport.x + domRect.left,
        y: flowPos.y * viewport.zoom + viewport.y + domRect.top,
    };
}
```

### 4.3 Fit View Pattern

```typescript
interface FitViewOptions {
    padding?: number;          // Padding around nodes (0.1 = 10%)
    includeHiddenNodes?: boolean;
    minZoom?: number;
    maxZoom?: number;
    duration?: number;         // Animation duration
    nodes?: Node[];            // Specific nodes to fit (default: all)
}

function getViewportForBounds(
    bounds: Rect,                    // {x, y, width, height}
    viewportWidth: number,
    viewportHeight: number,
    minZoom: number,
    maxZoom: number,
    padding: number
): Viewport {
    const xZoom = viewportWidth / (bounds.width * (1 + padding));
    const yZoom = viewportHeight / (bounds.height * (1 + padding));
    const zoom = Math.min(xZoom, yZoom, maxZoom);
    const clampedZoom = Math.max(zoom, minZoom);
    
    const x = viewportWidth / 2 - (bounds.x + bounds.width / 2) * clampedZoom;
    const y = viewportHeight / 2 - (bounds.y + bounds.height / 2) * clampedZoom;
    
    return { x, y, zoom: clampedZoom };
}
```

---

## 5. EDGE PATH CALCULATIONS

### 5.1 Edge Path Functions

```typescript
// All return: [path: string, labelX: number, labelY: number, offsetX: number, offsetY: number]

interface EdgePathParams {
    sourceX: number;
    sourceY: number;
    sourcePosition: Position;      // 'top' | 'right' | 'bottom' | 'left'
    targetX: number;
    targetY: number;
    targetPosition: Position;
    curvature?: number;            // For bezier curves
}

// Bezier (default) - smooth curve
function getBezierPath(params: EdgePathParams): [string, number, number, number, number];

// Straight - direct line
function getStraightPath(params: EdgePathParams): [string, number, number, number, number];

// Step - right angles only
function getSmoothStepPath(params: EdgePathParams & {
    borderRadius?: number;
    offset?: number;
}): [string, number, number, number, number];

// Simple bezier - less curvature
function getSimpleBezierPath(params: EdgePathParams): [string, number, number, number, number];
```

### 5.2 Example Implementation (Straight Path)

```typescript
function getStraightPath({
    sourceX, sourceY,
    targetX, targetY
}: EdgePathParams): [string, number, number, number, number] {
    const path = `M ${sourceX},${sourceY} L ${targetX},${targetY}`;
    const labelX = (sourceX + targetX) / 2;
    const labelY = (sourceY + targetY) / 2;
    
    return [path, labelX, labelY, 0, 0];
}
```

### 5.3 Handle Position Calculation

```typescript
function getHandlePosition(
    node: InternalNode,
    handle: HandleBounds,
    position: Position
): XYPosition {
    const nodeX = node.internals.positionAbsolute.x;
    const nodeY = node.internals.positionAbsolute.y;
    const nodeWidth = node.measured?.width || 0;
    const nodeHeight = node.measured?.height || 0;
    
    // Handle offset within node
    const handleX = handle.x + handle.width / 2;
    const handleY = handle.y + handle.height / 2;
    
    return {
        x: nodeX + handleX,
        y: nodeY + handleY,
    };
}
```

---

## 6. LAYOUT INTEGRATION PATTERN

### 6.1 Abstract Layout Interface

```typescript
interface LayoutAlgorithm {
    // Takes nodes/edges, returns positioned nodes
    layout(
        nodes: Node[],
        edges: Edge[],
        options: LayoutOptions
    ): Promise<{ nodes: Node[]; edges: Edge[] }> | { nodes: Node[]; edges: Edge[] };
}

interface LayoutOptions {
    direction?: 'TB' | 'BT' | 'LR' | 'RL';
    nodeSpacing?: number;
    rankSpacing?: number;
}
```

### 6.2 Dagre Integration Example

```typescript
import dagre from 'dagre';

function getLayoutedElements(
    nodes: Node[],
    edges: Edge[],
    direction: 'TB' | 'LR' = 'TB'
): { nodes: Node[]; edges: Edge[] } {
    const dagreGraph = new dagre.graphlib.Graph();
    dagreGraph.setDefaultEdgeLabel(() => ({}));
    dagreGraph.setGraph({ rankdir: direction });
    
    const isHorizontal = direction === 'LR';
    
    // Add nodes with dimensions
    nodes.forEach(node => {
        dagreGraph.setNode(node.id, {
            width: node.measured?.width || 172,
            height: node.measured?.height || 36,
        });
    });
    
    // Add edges
    edges.forEach(edge => {
        dagreGraph.setEdge(edge.source, edge.target);
    });
    
    // Run layout
    dagre.layout(dagreGraph);
    
    // Map positions back
    const layoutedNodes = nodes.map(node => {
        const nodeWithPosition = dagreGraph.node(node.id);
        
        return {
            ...node,
            targetPosition: isHorizontal ? 'left' : 'top',
            sourcePosition: isHorizontal ? 'right' : 'bottom',
            // Dagre uses center, React Flow uses top-left
            position: {
                x: nodeWithPosition.x - (node.measured?.width || 172) / 2,
                y: nodeWithPosition.y - (node.measured?.height || 36) / 2,
            },
        };
    });
    
    return { nodes: layoutedNodes, edges };
}
```

### 6.3 ELK Integration (More Complex)

```typescript
import ELK from 'elkjs/lib/elk.bundled.js';

const elk = new ELK();

async function getElkLayout(
    nodes: Node[],
    edges: Edge[],
    options: Record<string, string> = {}
): Promise<{ nodes: Node[]; edges: Edge[] }> {
    const graph = {
        id: 'root',
        layoutOptions: {
            'elk.algorithm': 'layered',
            'elk.direction': 'DOWN',
            'elk.spacing.nodeNode': '50',
            'elk.layered.spacing.nodeNodeBetweenLayers': '100',
            ...options,
        },
        children: nodes.map(node => ({
            id: node.id,
            width: node.measured?.width || 150,
            height: node.measured?.height || 50,
            // For multiple handles, define ports
            ports: node.data?.ports?.map((port, i) => ({
                id: `${node.id}_port_${i}`,
                properties: {
                    side: port.position,
                    index: i,
                },
            })),
        })),
        edges: edges.map(edge => ({
            id: edge.id,
            sources: [edge.source],
            targets: [edge.target],
        })),
    };
    
    const layoutedGraph = await elk.layout(graph);
    
    const layoutedNodes = layoutedGraph.children!.map(elkNode => {
        const originalNode = nodes.find(n => n.id === elkNode.id)!;
        return {
            ...originalNode,
            position: { x: elkNode.x!, y: elkNode.y! },
        };
    });
    
    return { nodes: layoutedNodes, edges };
}
```

---

## 7. PERFORMANCE PATTERNS

### 7.1 Selector Pattern (Zustand)

```typescript
// BAD: Re-renders on ANY state change
const nodes = useStore(state => state.nodes);

// GOOD: Only re-renders when nodes.length changes
const nodesLength = useStore(state => state.nodes.length);

// GOOD: Memoized selector with shallow equality
const selectedNodes = useStore(
    useCallback(state => state.nodes.filter(n => n.selected), []),
    shallow
);
```

### 7.2 Node Lookup Optimization

```typescript
// O(1) node access via Map
const nodeLookup = new Map<string, InternalNode>();

// Update lookup when nodes change
function updateNodeLookup(nodes: Node[]): void {
    nodeLookup.clear();
    for (const node of nodes) {
        nodeLookup.set(node.id, computeInternalNode(node));
    }
}

// Fast access
function getNode(id: string): InternalNode | undefined {
    return nodeLookup.get(id);
}
```

### 7.3 Batched Updates

```typescript
// Multiple changes in single update
function batchNodeUpdates(updates: Array<{id: string; changes: Partial<Node>}>) {
    setNodes(nodes => {
        const result = [...nodes];
        for (const {id, changes} of updates) {
            const idx = result.findIndex(n => n.id === id);
            if (idx !== -1) {
                result[idx] = { ...result[idx], ...changes };
            }
        }
        return result;
    });
}
```

---

## 8. CONNECTION VALIDATION PATTERN

```typescript
interface IsValidConnection {
    (connection: Connection): boolean;
}

interface Connection {
    source: string;
    target: string;
    sourceHandle: string | null;
    targetHandle: string | null;
}

// Example: Prevent self-connections and type mismatches
const isValidConnection: IsValidConnection = (connection) => {
    // No self-connections
    if (connection.source === connection.target) return false;
    
    // Get node types
    const sourceNode = getNode(connection.source);
    const targetNode = getNode(connection.target);
    
    // Type-based validation
    if (sourceNode?.data.outputType !== targetNode?.data.inputType) {
        return false;
    }
    
    // Prevent duplicate connections
    const existingEdge = edges.find(
        e => e.source === connection.source && 
             e.target === connection.target &&
             e.sourceHandle === connection.sourceHandle &&
             e.targetHandle === connection.targetHandle
    );
    
    return !existingEdge;
};
```

---

## 9. SYNAPTIX TRANSLATION GUIDE

| React Flow Concept | SYNAPTIX Equivalent |
|---|---|
| Zustand Store | Rust State + WASM bindings |
| NodeChange discriminated union | Rust enum with variants |
| applyNodeChanges | Rust impl with pattern matching |
| nodeLookup Map | HashMap<NodeId, InternalNode> |
| d3-zoom | Custom WebGL/Canvas viewport |
| @xyflow/system | `synaptix-core` crate |
| React hooks | Dioxus hooks or Leptos signals |

### Rust Change Enum Example

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum NodeChange {
    #[serde(rename = "position")]
    Position {
        id: NodeId,
        position: Option<XYPosition>,
        dragging: Option<bool>,
    },
    #[serde(rename = "dimensions")]
    Dimensions {
        id: NodeId,
        dimensions: Dimensions,
        resizing: Option<bool>,
    },
    #[serde(rename = "select")]
    Selection {
        id: NodeId,
        selected: bool,
    },
    #[serde(rename = "remove")]
    Remove {
        id: NodeId,
    },
    #[serde(rename = "add")]
    Add {
        item: Node,
        index: Option<usize>,
    },
    #[serde(rename = "replace")]
    Replace {
        id: NodeId,
        item: Node,
    },
}

impl NodeChange {
    pub fn apply(changes: Vec<NodeChange>, nodes: &mut Vec<Node>) {
        for change in changes {
            match change {
                NodeChange::Position { id, position, .. } => {
                    if let Some(pos) = position {
                        if let Some(node) = nodes.iter_mut().find(|n| n.id == id) {
                            node.position = pos;
                        }
                    }
                }
                NodeChange::Remove { id } => {
                    nodes.retain(|n| n.id != id);
                }
                NodeChange::Add { item, index } => {
                    match index {
                        Some(i) => nodes.insert(i, item),
                        None => nodes.push(item),
                    }
                }
                // ... other variants
            }
        }
    }
}
```

---

## 10. KEY TAKEAWAYS

1. **Separate User Model from Internal Model**: Users work with simple `Node`, system computes `InternalNode` with derived properties.

2. **Discriminated Unions for Changes**: Type-safe, serializable, reversible changes enable undo/redo, collaboration, and event sourcing.

3. **Pure Change Application**: `applyNodeChanges(changes, nodes)` is a pure function - easy to test, reason about, and replay.

4. **Lookup Maps for Performance**: O(1) node/edge access via `Map<id, item>` alongside array for iteration order.

5. **Framework-Agnostic Core**: `@xyflow/system` pattern - put all graph logic in framework-agnostic layer, thin bindings on top.

6. **Coordinate Transform Clarity**: Clear functions for screen↔flow coordinate conversion, accounting for viewport pan/zoom.

7. **Layout is External**: No built-in layout - provide clean integration pattern for dagre, ELK, d3-force, custom algorithms.

8. **Connection Validation Hook**: Single point for validating all new connections - prevent cycles, type mismatches, duplicates.

9. **Selector-Based Subscriptions**: Only re-render components when their specific slice of state changes.

10. **Edge Path as Pure Functions**: Path calculation is stateless - `(sourceX, sourceY, targetX, targetY, positions) → SVG path string`.

---

*Document generated from React Flow / xyflow analysis for SYNAPTIX development reference*
