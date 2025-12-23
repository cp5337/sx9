/**
 * Validation Utilities
 * Functions for validating graph nodes and edges
 * @module validators
 */

import type { GraphNode } from "../GraphNode"
import type { GraphEdge } from "../GraphEdge"
import { type GraphHandle, validateHandle as validateHandleConfig } from "../GraphHandle"

/**
 * Validation result
 */
export interface ValidationResult {
  valid: boolean
  errors: string[]
  warnings: string[]
}

/**
 * Validate a graph node
 */
export function validateNode(node: GraphNode): ValidationResult {
  const errors: string[] = []
  const warnings: string[] = []

  // Required: id
  if (!node.id || node.id.trim() === "") {
    errors.push("Node must have a non-empty id")
  }

  // Size validation
  if (node.size !== undefined && node.size <= 0) {
    errors.push("Node size must be greater than 0")
  }

  // Position validation
  if (node.x !== undefined && !isFinite(node.x)) {
    errors.push("Node x coordinate must be a finite number")
  }
  if (node.y !== undefined && !isFinite(node.y)) {
    errors.push("Node y coordinate must be a finite number")
  }

  // Style validation
  if (node.style) {
    if (node.style.borderWidth !== undefined && node.style.borderWidth < 0) {
      errors.push("Border width cannot be negative")
    }
    if (node.style.opacity !== undefined && (node.style.opacity < 0 || node.style.opacity > 1)) {
      errors.push("Opacity must be between 0 and 1")
    }
    if (
      node.style.backgroundOpacity !== undefined &&
      (node.style.backgroundOpacity < 0 || node.style.backgroundOpacity > 1)
    ) {
      errors.push("Background opacity must be between 0 and 1")
    }
  }

  // ACOG sight validation
  if (node.acogSight) {
    if (node.acogSight.size !== undefined && (node.acogSight.size <= 0 || node.acogSight.size > 1)) {
      warnings.push("ACOG sight size should be between 0 and 1 (relative to node size)")
    }
    if (
      node.acogSight.glowOpacity !== undefined &&
      (node.acogSight.glowOpacity < 0 || node.acogSight.glowOpacity > 1)
    ) {
      errors.push("ACOG glow opacity must be between 0 and 1")
    }
  }

  // Icon validation
  if (node.icon && typeof node.icon === "object") {
    if (!node.icon.url || node.icon.url.trim() === "") {
      errors.push("Icon must have a valid URL")
    }
    if (node.icon.size !== undefined && node.icon.size <= 0) {
      warnings.push("Icon size should be greater than 0")
    }
  }

  // Overlay icon validation
  if (node.overlayIcon) {
    if (!node.overlayIcon.url || node.overlayIcon.url.trim() === "") {
      errors.push("Overlay icon must have a valid URL")
    }
    if (node.overlayIcon.position) {
      const [x, y] = node.overlayIcon.position
      if (x < -1 || x > 1 || y < -1 || y > 1) {
        warnings.push("Overlay icon position should be between -1 and 1")
      }
    }
  }

  // Animation validation
  if (node.animation) {
    if (node.animation.duration !== undefined && node.animation.duration <= 0) {
      errors.push("Animation duration must be greater than 0")
    }
    if (node.animation.delay !== undefined && node.animation.delay < 0) {
      errors.push("Animation delay cannot be negative")
    }
  }

  return {
    valid: errors.length === 0,
    errors,
    warnings,
  }
}

/**
 * Validate a graph edge
 */
export function validateEdge(edge: GraphEdge): ValidationResult {
  const errors: string[] = []
  const warnings: string[] = []

  // Required: id, source, target
  if (!edge.id || edge.id.trim() === "") {
    errors.push("Edge must have a non-empty id")
  }
  if (!edge.source || edge.source.trim() === "") {
    errors.push("Edge must have a non-empty source node id")
  }
  if (!edge.target || edge.target.trim() === "") {
    errors.push("Edge must have a non-empty target node id")
  }

  // Self-loop warning
  if (edge.source === edge.target) {
    warnings.push("Edge creates a self-loop (source and target are the same)")
  }

  // Style validation
  if (edge.style) {
    if (edge.style.width !== undefined && edge.style.width <= 0) {
      errors.push("Edge width must be greater than 0")
    }
    if (edge.style.opacity !== undefined && (edge.style.opacity < 0 || edge.style.opacity > 1)) {
      errors.push("Edge opacity must be between 0 and 1")
    }
    if (edge.style.curvature !== undefined && (edge.style.curvature < 0 || edge.style.curvature > 1)) {
      warnings.push("Edge curvature should typically be between 0 and 1")
    }
  }

  // Marker validation
  const validateMarker = (marker: any, position: "start" | "end") => {
    if (marker && typeof marker === "object") {
      if (marker.scale !== undefined && marker.scale <= 0) {
        errors.push(`Edge marker ${position} scale must be greater than 0`)
      }
      if (marker.strokeWidth !== undefined && marker.strokeWidth < 0) {
        errors.push(`Edge marker ${position} stroke width cannot be negative`)
      }
    }
  }

  validateMarker(edge.markerStart, "start")
  validateMarker(edge.markerEnd, "end")

  // Label validation
  if (edge.labelConfig) {
    if (edge.labelConfig.fontSize !== undefined && edge.labelConfig.fontSize <= 0) {
      errors.push("Label font size must be greater than 0")
    }
    if (
      edge.labelConfig.backgroundOpacity !== undefined &&
      (edge.labelConfig.backgroundOpacity < 0 || edge.labelConfig.backgroundOpacity > 1)
    ) {
      errors.push("Label background opacity must be between 0 and 1")
    }
  }

  // Behavior validation
  if (edge.behavior) {
    if (edge.behavior.interactionWidth !== undefined && edge.behavior.interactionWidth <= 0) {
      warnings.push("Interaction width should be greater than 0")
    }
  }

  // Animation validation
  if (edge.animation) {
    if (edge.animation.duration !== undefined && edge.animation.duration <= 0) {
      errors.push("Animation duration must be greater than 0")
    }
    if (edge.animation.delay !== undefined && edge.animation.delay < 0) {
      errors.push("Animation delay cannot be negative")
    }
  }

  return {
    valid: errors.length === 0,
    errors,
    warnings,
  }
}

/**
 * Validate a connection between two nodes
 */
export function validateConnection(
  sourceNodeId: string,
  targetNodeId: string,
  nodes: GraphNode[],
  edges: GraphEdge[],
  options?: {
    allowSelfLoops?: boolean
    allowDuplicates?: boolean
  },
): ValidationResult {
  const errors: string[] = []
  const warnings: string[] = []

  // Check if source node exists
  const sourceNode = nodes.find((n) => n.id === sourceNodeId)
  if (!sourceNode) {
    errors.push(`Source node '${sourceNodeId}' not found`)
  }

  // Check if target node exists
  const targetNode = nodes.find((n) => n.id === targetNodeId)
  if (!targetNode) {
    errors.push(`Target node '${targetNodeId}' not found`)
  }

  // Check for self-loops
  if (sourceNodeId === targetNodeId && !options?.allowSelfLoops) {
    errors.push("Self-loops are not allowed")
  }

  // Check for duplicate edges
  if (!options?.allowDuplicates) {
    const duplicate = edges.find((e) => e.source === sourceNodeId && e.target === targetNodeId)
    if (duplicate) {
      warnings.push(`Duplicate edge already exists between '${sourceNodeId}' and '${targetNodeId}'`)
    }
  }

  // Check if nodes are disabled
  if (sourceNode?.disabled) {
    warnings.push(`Source node '${sourceNodeId}' is disabled`)
  }
  if (targetNode?.disabled) {
    warnings.push(`Target node '${targetNodeId}' is disabled`)
  }

  return {
    valid: errors.length === 0,
    errors,
    warnings,
  }
}

/**
 * Validate a handle
 */
export function validateHandle(handle: GraphHandle): ValidationResult {
  const errors: string[] = []
  const warnings: string[] = []

  const isValid = validateHandleConfig(handle)
  if (!isValid) {
    errors.push("Handle configuration is invalid")
  }

  // Additional validation
  if (handle.style) {
    if (handle.style.size !== undefined && handle.style.size <= 0) {
      errors.push("Handle size must be greater than 0")
    }
    if (handle.style.opacity !== undefined && (handle.style.opacity < 0 || handle.style.opacity > 1)) {
      errors.push("Handle opacity must be between 0 and 1")
    }
  }

  if (handle.maxConnections !== undefined && handle.maxConnections < 0) {
    errors.push("Max connections cannot be negative")
  }

  if (handle.connectionCount !== undefined && handle.connectionCount < 0) {
    errors.push("Connection count cannot be negative")
  }

  return {
    valid: errors.length === 0,
    errors,
    warnings,
  }
}

/**
 * Validate an entire graph
 */
export function validateGraph(nodes: GraphNode[], edges: GraphEdge[]): ValidationResult {
  const errors: string[] = []
  const warnings: string[] = []

  // Validate all nodes
  const nodeIds = new Set<string>()
  nodes.forEach((node, index) => {
    const result = validateNode(node)
    if (!result.valid) {
      errors.push(`Node ${index} (${node.id}): ${result.errors.join(", ")}`)
    }
    warnings.push(...result.warnings.map((w) => `Node ${index} (${node.id}): ${w}`))

    // Check for duplicate IDs
    if (nodeIds.has(node.id)) {
      errors.push(`Duplicate node ID: ${node.id}`)
    }
    nodeIds.add(node.id)
  })

  // Validate all edges
  const edgeIds = new Set<string>()
  edges.forEach((edge, index) => {
    const result = validateEdge(edge)
    if (!result.valid) {
      errors.push(`Edge ${index} (${edge.id}): ${result.errors.join(", ")}`)
    }
    warnings.push(...result.warnings.map((w) => `Edge ${index} (${edge.id}): ${w}`))

    // Check for duplicate IDs
    if (edgeIds.has(edge.id)) {
      errors.push(`Duplicate edge ID: ${edge.id}`)
    }
    edgeIds.add(edge.id)

    // Check if referenced nodes exist
    if (!nodeIds.has(edge.source)) {
      errors.push(`Edge ${edge.id}: source node '${edge.source}' does not exist`)
    }
    if (!nodeIds.has(edge.target)) {
      errors.push(`Edge ${edge.id}: target node '${edge.target}' does not exist`)
    }
  })

  return {
    valid: errors.length === 0,
    errors,
    warnings,
  }
}
