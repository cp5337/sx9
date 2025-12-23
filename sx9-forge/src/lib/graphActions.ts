// handlers/graphActions.ts
import type { GraphNode, GraphRelationship } from "../lib/database.types"

/**
 * Generate a 9-sector Nonagon around a central node
 */
export function generateNonagon(
  centerNodeId: string,
  nodes: GraphNode[],
): { nodes: GraphNode[]; relationships: GraphRelationship[] } | null {
  const centerNode = nodes.find((n) => n.id === centerNodeId)
  if (!centerNode) return null

  const newNodes: GraphNode[] = []
  const newRels: GraphRelationship[] = []
  const sectors = 9
  const baseId = `nonagon-${Date.now()}`

  // Create 9 Sector Nodes
  for (let i = 1; i <= sectors; i++) {
    const sectorId = `${baseId}-sector-${i}`
    newNodes.push({
      id: sectorId,
      label: "sector",
      properties: { name: `Sector ${i}`, type: "Intelligence" },
    })

    // Link to Core
    newRels.push({
      id: `${baseId}-rel-core-${i}`,
      source: centerNodeId,
      target: sectorId,
      type: "CORRELATED_WITH",
      properties: {},
    })
  }

  // Create Ring Connections (1-2, 2-3, ... 9-1)
  for (let i = 0; i < sectors; i++) {
    const current = newNodes[i]
    const next = newNodes[(i + 1) % sectors]
    newRels.push({
      id: `${baseId}-rel-ring-${i}`,
      source: current.id,
      target: next.id,
      type: "LINKED",
      properties: {},
    })
  }

  return { nodes: newNodes, relationships: newRels }
}

/**
 * Cross-reference probe - surgical insertion of external context
 * Limits fetch to direct 1st-degree correlations to prevent "hairballs"
 */
export function generateCrossReference(
  centerNodeId: string,
  database: string,
  nodes: GraphNode[],
): { nodes: GraphNode[]; relationships: GraphRelationship[] } | null {
  const centerNode = nodes.find((n) => n.id === centerNodeId)
  if (!centerNode) return null

  const newNodes: GraphNode[] = []
  const newRels: GraphRelationship[] = []
  const count = 5 // Limit to 5 items to avoid clutter
  const baseId = `xref-${database}-${Date.now()}`

  for (let i = 1; i <= count; i++) {
    const targetId = `${baseId}-${i}`
    newNodes.push({
      id: targetId,
      label: "Database",
      properties: {
        name: `${database.toUpperCase()} Record #${i * 142}`,
        type: "External Record",
        source: database,
      },
    })

    newRels.push({
      id: `${baseId}-rel-${i}`,
      source: centerNodeId,
      target: targetId,
      type: "XREF_MATCH",
      properties: { score: (0.9 - i * 0.05).toFixed(2) },
    })
  }

  return { nodes: newNodes, relationships: newRels }
}
