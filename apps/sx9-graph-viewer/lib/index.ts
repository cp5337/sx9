/**
 * SX9 Graph Viewer Library Exports
 *
 * This module re-exports all graph data types, Forge client,
 * and React hooks for external consumption.
 */

// Graph data types and utilities
export {
  type GraphNode,
  type GraphEdge,
  type EEIRequirement,
  type ForgeGraphData,
  mockNodes,
  mockEEIRequirements,
  getNodeColor,
  getPriorityColor,
  getRelationshipIcon,
  fetchGraphData,
  createToolFromMissionLoad,
  executeTool,
  getForgeHealth,
  forgeClient,
} from './graph-data'

// Forge API client
export {
  type ForgeNode,
  type ForgeEdge,
  type NonagonCell,
  type MissionLoad,
  type GeneratedTool,
  type ToolChain,
  type ForgeStatus,
  type ChainExecutionResult,
  forgeNodeToGraphNode,
  getHD4Color,
  getTethEntropyColor,
} from './forge-client'

// React hooks for Forge data
export {
  useForgeData,
  useForgeStatus,
  useMissionLoads,
  useToolExecution,
  getEntropyNodeStyle,
  type UseForgeDataResult,
  type UseForgeStatusResult,
} from './use-forge-data'
