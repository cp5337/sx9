"use client"

import { Badge } from "../ui/badge"
import { Button } from "../ui/button"
import { ScrollArea } from "../ui/scroll-area"
import type { WazuhAgent } from "../../types/plasma"
import { MoreHorizontal, Activity, AlertCircle, Clock } from "lucide-react"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "../ui/dropdown-menu"

interface AgentPanelProps {
  agents: WazuhAgent[]
  onAgentClick: (agent: WazuhAgent) => void
  onRestartAgent: (id: string) => void
  onDeleteAgent: (id: string) => void
}

export function AgentPanel({ agents, onAgentClick, onRestartAgent, onDeleteAgent }: AgentPanelProps) {
  const getStatusColor = (status: WazuhAgent["status"]) => {
    switch (status) {
      case "active":
        return "text-green-500"
      case "disconnected":
        return "text-amber-500"
      case "never_connected":
        return "text-slate-500"
      case "pending":
        return "text-blue-500"
      default:
        return "text-slate-500"
    }
  }

  const getStatusBadge = (status: WazuhAgent["status"]) => {
    switch (status) {
      case "active":
        return <Badge className="bg-green-950/30 text-green-400 border-green-900/50">ACTIVE</Badge>
      case "disconnected":
        return <Badge className="bg-amber-950/30 text-amber-400 border-amber-900/50">DISCONNECTED</Badge>
      case "never_connected":
        return (
          <Badge variant="outline" className="text-slate-500">
            NEVER CONNECTED
          </Badge>
        )
      case "pending":
        return <Badge className="bg-blue-950/30 text-blue-400 border-blue-900/50">PENDING</Badge>
    }
  }

  const activeCount = agents.filter((a) => a.status === "active").length
  const disconnectedCount = agents.filter((a) => a.status === "disconnected").length

  return (
    <div className="flex flex-col h-full border-r border-gray-800 bg-gray-900/30">
      <div className="p-4 border-b border-gray-800">
        <div className="flex items-center justify-between mb-3">
          <h2 className="text-sm font-mono font-semibold tracking-wider">AGENTS</h2>
          <div className="flex gap-2 text-xs font-mono">
            <span className="text-green-500">{activeCount}</span>
            <span className="text-slate-400">/</span>
            <span className="text-amber-500">{disconnectedCount}</span>
          </div>
        </div>
        <div className="grid grid-cols-2 gap-2 text-xs font-mono">
          <div className="flex items-center gap-2">
            <Activity className="w-3 h-3 text-green-500" />
            <span className="text-slate-400">Active: {activeCount}</span>
          </div>
          <div className="flex items-center gap-2">
            <AlertCircle className="w-3 h-3 text-amber-500" />
            <span className="text-slate-400">Alerts: {agents.reduce((sum, a) => sum + a.alertCount, 0)}</span>
          </div>
        </div>
      </div>

      <ScrollArea className="flex-1">
        <div className="p-2 space-y-1">
          {agents.map((agent) => (
            <div
              key={agent.id}
              className="p-3 rounded-md border border-gray-800 bg-gray-900/50 hover:bg-gray-800/50 transition-colors cursor-pointer group"
              onClick={() => onAgentClick(agent)}
            >
              <div className="flex items-start justify-between mb-2">
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 mb-1">
                    <div className={`w-2 h-2 rounded-full ${getStatusColor(agent.status)} animate-pulse`} />
                    <span className="font-mono text-xs font-mono font-semibold truncate">{agent.name}</span>
                  </div>
                  <div className="text-xs font-mono text-slate-400 font-mono">{agent.ip}</div>
                </div>
                <DropdownMenu>
                  <DropdownMenuTrigger asChild onClick={(e) => e.stopPropagation()}>
                    <Button variant="ghost" size="sm" className="h-6 w-6 p-0 opacity-0 group-hover:opacity-100">
                      <MoreHorizontal className="w-3 h-3" />
                    </Button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="end" className="w-40">
                    <DropdownMenuItem
                      onClick={(e) => {
                        e.stopPropagation()
                        onRestartAgent(agent.id)
                      }}
                    >
                      Restart Agent
                    </DropdownMenuItem>
                    <DropdownMenuItem
                      onClick={(e) => {
                        e.stopPropagation()
                        onDeleteAgent(agent.id)
                      }}
                    >
                      Delete Agent
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </div>

              <div className="flex items-center justify-between">
                {getStatusBadge(agent.status)}
                <div className="flex items-center gap-1 text-xs font-mono text-slate-400">
                  {agent.alertCount > 0 && (
                    <Badge variant="outline" className="text-xs font-mono px-1.5 py-0">
                      {agent.alertCount}
                    </Badge>
                  )}
                </div>
              </div>

              <div className="mt-2 flex items-center gap-1 text-[10px] text-slate-400 font-mono">
                <Clock className="w-3 h-3" />
                <span suppressHydrationWarning>
                  {agent.lastKeepAlive ? new Date(agent.lastKeepAlive).toLocaleTimeString() : "Never"}
                </span>
              </div>
            </div>
          ))}
        </div>
      </ScrollArea>
    </div>
  )
}
