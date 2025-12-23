/**
 * Database types for graph nodes and relationships
 */

export interface GraphNode {
  id: string
  label: string
  properties: Record<string, any>
}

export interface GraphRelationship {
  id: string
  source: string
  target: string
  type: string
  properties: Record<string, any>
}
