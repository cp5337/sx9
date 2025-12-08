/**
 * React Hook for SX9 Forge Data
 * Provides real-time graph data from sx9-forge backend with mock fallback
 */

'use client'

import { useState, useEffect, useCallback } from 'react'
import { fetchGraphData, getForgeHealth, type ForgeGraphData, type GraphNode } from './graph-data'
import { forgeClient, getTethEntropyColor, getHD4Color } from './forge-client'

export interface UseForgeDataResult {
  data: ForgeGraphData | null
  loading: boolean
  error: Error | null
  connected: boolean
  refresh: () => Promise<void>
}

export interface UseForgeStatusResult {
  status: {
    online: boolean
    version: string
    nodeCount: number
    edgeCount: number
    missionLoads: number
    ringBusNode: number
    l2Execution: boolean
  } | null
  loading: boolean
  error: Error | null
}

// Hook to fetch and manage graph data from Forge
export function useForgeData(autoRefresh: boolean = false, intervalMs: number = 30000): UseForgeDataResult {
  const [data, setData] = useState<ForgeGraphData | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)
  const [connected, setConnected] = useState(false)

  const refresh = useCallback(async () => {
    try {
      setLoading(true)
      const graphData = await fetchGraphData()
      setData(graphData)
      setConnected(graphData.status.connected)
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Failed to fetch graph data'))
      setConnected(false)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    refresh()

    if (autoRefresh) {
      const interval = setInterval(refresh, intervalMs)
      return () => clearInterval(interval)
    }
  }, [refresh, autoRefresh, intervalMs])

  return { data, loading, error, connected, refresh }
}

// Hook to get Forge service status
export function useForgeStatus(): UseForgeStatusResult {
  const [status, setStatus] = useState<UseForgeStatusResult['status']>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  useEffect(() => {
    async function fetchStatus() {
      try {
        const forgeStatus = await forgeClient.getStatus()
        setStatus({
          online: true,
          version: forgeStatus.version,
          nodeCount: forgeStatus.graph_nodes,
          edgeCount: forgeStatus.graph_edges,
          missionLoads: forgeStatus.mission_loads,
          ringBusNode: forgeStatus.ring_bus_node,
          l2Execution: forgeStatus.l2_execution,
        })
      } catch (err) {
        setStatus({
          online: false,
          version: 'offline',
          nodeCount: 0,
          edgeCount: 0,
          missionLoads: 0,
          ringBusNode: 0,
          l2Execution: false,
        })
        setError(err instanceof Error ? err : new Error('Forge offline'))
      } finally {
        setLoading(false)
      }
    }

    fetchStatus()
  }, [])

  return { status, loading, error }
}

// Hook to get Mission Loads from Forge
export function useMissionLoads() {
  const [missionLoads, setMissionLoads] = useState<Array<{
    id: string
    name: string
    hd4_phase: string
    clearance: string
    price_credits: number
    primitive_count: number
    color: string
  }>>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    async function fetchMissionLoads() {
      try {
        const loads = await forgeClient.getMissionLoads()
        setMissionLoads(loads.map(ml => ({
          ...ml,
          color: getHD4Color(ml.hd4_phase),
        })))
      } catch {
        setMissionLoads([])
      } finally {
        setLoading(false)
      }
    }

    fetchMissionLoads()
  }, [])

  return { missionLoads, loading }
}

// Hook to create and execute tools
export function useToolExecution() {
  const [executing, setExecuting] = useState(false)
  const [result, setResult] = useState<{ success: boolean; message: string } | null>(null)

  const createAndExecuteTool = useCallback(async (missionLoadId: string) => {
    setExecuting(true)
    setResult(null)

    try {
      // Create tool from mission load
      const tool = await forgeClient.createToolFromLoad(missionLoadId)

      // Execute the tool
      const execResult = await forgeClient.executeTool(tool.id)
      setResult(execResult)

      return { tool, execResult }
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Execution failed'
      setResult({ success: false, message })
      throw err
    } finally {
      setExecuting(false)
    }
  }, [])

  const executeTool = useCallback(async (toolId: string) => {
    setExecuting(true)
    setResult(null)

    try {
      const execResult = await forgeClient.executeTool(toolId)
      setResult(execResult)
      return execResult
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Execution failed'
      setResult({ success: false, message })
      throw err
    } finally {
      setExecuting(false)
    }
  }, [])

  return { createAndExecuteTool, executeTool, executing, result }
}

// Utility: Get entropy-based node styling
export function getEntropyNodeStyle(node: GraphNode): {
  borderColor: string
  borderWidth: number
  glowIntensity: number
} {
  const entropy = node.metadata.tethEntropy ?? 2.5
  const color = getTethEntropyColor(entropy)

  return {
    borderColor: color,
    borderWidth: entropy >= 3.0 ? 3 : 2,
    glowIntensity: Math.min(1, entropy / 4),
  }
}
