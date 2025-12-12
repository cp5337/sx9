"use client"

import { useState, useEffect, useCallback } from "react"
import type { WazuhAgent, WazuhManager, CognitiveStatus } from "../types/plasma"
import { plasmaCognitiveClient } from "../lib/api/plasma-cognitive-client"

// NVNN: Mock agents with RFC-9021 cognitive status
const MOCK_AGENTS: WazuhAgent[] = [
  {
    id: "001",
    name: "web-server-01",
    ip: "10.0.1.15",
    status: "active",
    os: "linux",
    osVersion: "Ubuntu 22.04",
    version: "4.7.0",
    lastKeepAlive: new Date(Date.now() - 30000).toISOString(),
    group: ["webservers", "production"],
    manager: "wazuh-manager-01",
    registerDate: "2024-01-15T08:00:00Z",
    alertCount: 47,
    // RFC-9021 cognitive status
    cognitive_status: {
      thalamic: {
        gate_decision: "full_processing",
        pathway: "threat_analysis",
        priority: "high",
        activated_domains: ["detection", "technique_mapping"],
      },
      glaf: { h1_operational: 0.85, h2_semantic: 0.78, combined: 0.80 },
      last_inference: new Date().toISOString(),
    },
  },
  {
    id: "002",
    name: "db-primary",
    ip: "10.0.2.20",
    status: "active",
    os: "linux",
    osVersion: "RHEL 8.6",
    version: "4.7.0",
    lastKeepAlive: new Date(Date.now() - 15000).toISOString(),
    group: ["databases", "production"],
    manager: "wazuh-manager-01",
    registerDate: "2024-01-10T10:30:00Z",
    alertCount: 23,
    cognitive_status: {
      thalamic: {
        gate_decision: "reflexive",
        pathway: "operational",
        priority: "medium",
        activated_domains: ["detection"],
      },
      glaf: { h1_operational: 0.65, h2_semantic: 0.70, combined: 0.68 },
    },
  },
  {
    id: "003",
    name: "win-endpoint-12",
    ip: "10.0.3.45",
    status: "disconnected",
    os: "windows",
    osVersion: "Windows 10 Pro",
    version: "4.6.0",
    lastKeepAlive: new Date(Date.now() - 300000).toISOString(),
    group: ["endpoints", "workstations"],
    manager: "wazuh-manager-01",
    registerDate: "2024-02-01T14:20:00Z",
    alertCount: 156,
    cognitive_status: {
      thalamic: {
        gate_decision: "full_processing",
        pathway: "threat_analysis",
        priority: "critical",
        activated_domains: ["incident_response", "technique_mapping", "detection"],
      },
      glaf: { h1_operational: 0.92, h2_semantic: 0.88, combined: 0.89 },
    },
  },
  {
    id: "004",
    name: "cloud-api-gateway",
    ip: "172.16.0.10",
    status: "active",
    os: "linux",
    osVersion: "Amazon Linux 2",
    version: "4.7.0",
    lastKeepAlive: new Date(Date.now() - 45000).toISOString(),
    group: ["cloud", "api"],
    manager: "wazuh-manager-02",
    registerDate: "2024-01-20T09:15:00Z",
    alertCount: 89,
    cognitive_status: {
      thalamic: {
        gate_decision: "reflexive",
        pathway: "informational",
        priority: "low",
        activated_domains: ["detection"],
      },
      glaf: { h1_operational: 0.45, h2_semantic: 0.52, combined: 0.50 },
    },
  },
  {
    id: "005",
    name: "stage-agent-05",
    ip: "192.168.1.100",
    status: "pending",
    os: "linux",
    osVersion: "Debian 11",
    version: "4.7.0",
    group: ["staging"],
    manager: "wazuh-manager-01",
    registerDate: new Date().toISOString(),
    alertCount: 0,
    // No cognitive status yet for pending agents
  },
]

/**
 * Wazuh Agents Hook with RFC-9021 Cognitive Status
 * NVNN: useWazuhAgents manages agents with cognitive enrichment
 */
export function useWazuhAgents() {
  const [agents, setAgents] = useState<WazuhAgent[]>(MOCK_AGENTS)
  const [managers, setManagers] = useState<WazuhManager[]>([
    {
      id: "mgr-01",
      name: "wazuh-manager-01",
      url: "https://wazuh-01.internal:55000",
      apiKey: "",
      enabled: true,
      agentCount: 3,
      lastSync: new Date().toISOString(),
    },
    {
      id: "mgr-02",
      name: "wazuh-manager-02",
      url: "https://wazuh-02.internal:55000",
      apiKey: "",
      enabled: true,
      agentCount: 1,
      lastSync: new Date().toISOString(),
    },
  ])
  const [loading, setLoading] = useState(false)
  const [useMockData, setUseMockData] = useState(true)

  const refreshAgents = useCallback(async () => {
    setLoading(true)

    try {
      // Try to fetch from real API
      const realAgents = await plasmaCognitiveClient.getAgents()
      setAgents(realAgents)
      setUseMockData(false)
    } catch (err) {
      console.log("[Wazuh] API unavailable, using mock data")
      setUseMockData(true)

      // Update mock agent statuses with simulated cognitive updates
      setAgents((prev) =>
        prev.map((agent) => ({
          ...agent,
          lastKeepAlive: agent.status === "active" ? new Date().toISOString() : agent.lastKeepAlive,
          alertCount: agent.status === "active" ? agent.alertCount + Math.floor(Math.random() * 3) : agent.alertCount,
          // Simulate cognitive status updates
          cognitive_status: agent.cognitive_status
            ? {
                ...agent.cognitive_status,
                glaf: {
                  h1_operational: Math.min(1, (agent.cognitive_status.glaf?.h1_operational || 0.5) + (Math.random() - 0.5) * 0.1),
                  h2_semantic: Math.min(1, (agent.cognitive_status.glaf?.h2_semantic || 0.5) + (Math.random() - 0.5) * 0.1),
                  combined: Math.min(1, (agent.cognitive_status.glaf?.combined || 0.5) + (Math.random() - 0.5) * 0.1),
                },
                last_inference: new Date().toISOString(),
              }
            : undefined,
        })),
      )
    }

    setLoading(false)
  }, [])

  const addManager = useCallback((manager: Omit<WazuhManager, "id" | "agentCount" | "lastSync">) => {
    const newManager: WazuhManager = {
      ...manager,
      id: `mgr-${Date.now()}`,
      agentCount: 0,
      lastSync: new Date().toISOString(),
    }
    setManagers((prev) => [...prev, newManager])
  }, [])

  const removeManager = useCallback((id: string) => {
    setManagers((prev) => prev.filter((m) => m.id !== id))
  }, [])

  const updateAgent = useCallback((id: string, updates: Partial<WazuhAgent>) => {
    setAgents((prev) => prev.map((agent) => (agent.id === id ? { ...agent, ...updates } : agent)))
  }, [])

  const restartAgent = useCallback(
    async (id: string) => {
      console.log("[v0] Restarting agent:", id)
      await new Promise((resolve) => setTimeout(resolve, 500))
      updateAgent(id, { status: "active", lastKeepAlive: new Date().toISOString() })
    },
    [updateAgent],
  )

  const deleteAgent = useCallback((id: string) => {
    setAgents((prev) => prev.filter((agent) => agent.id !== id))
  }, [])

  // Auto-refresh agents every 30 seconds
  useEffect(() => {
    const interval = setInterval(refreshAgents, 30000)
    return () => clearInterval(interval)
  }, [refreshAgents])

  return {
    agents,
    managers,
    loading,
    useMockData,
    refreshAgents,
    addManager,
    removeManager,
    updateAgent,
    restartAgent,
    deleteAgent,
  }
}
