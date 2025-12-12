"use client"

import { Dialog, DialogContent, DialogHeader, DialogTitle } from "../ui/dialog"
import { Badge } from "../ui/badge"
import { Button } from "../ui/button"
import { ScrollArea } from "../ui/scroll-area"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "../ui/tabs"
import type { WazuhAgent } from "../../types/plasma"
import { Activity, Clock, FileCode, RefreshCw, Terminal } from "lucide-react"

interface AgentDetailModalProps {
  agent: WazuhAgent | null
  open: boolean
  onClose: () => void
  onRestart: (id: string) => void
}

export function AgentDetailModal({ agent, open, onClose, onRestart }: AgentDetailModalProps) {
  if (!agent) return null

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="max-w-4xl h-[80vh] bg-gray-900/95 backdrop-blur">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-3 font-mono">
            <Activity className="w-5 h-5 text-green-500" />
            <span>{agent.name}</span>
            <Badge variant="outline" className="font-normal">
              {agent.id}
            </Badge>
          </DialogTitle>
        </DialogHeader>

        <Tabs defaultValue="overview" className="flex-1">
          <TabsList className="grid w-full grid-cols-4">
            <TabsTrigger value="overview">Overview</TabsTrigger>
            <TabsTrigger value="config">Configuration</TabsTrigger>
            <TabsTrigger value="alerts">Alerts</TabsTrigger>
            <TabsTrigger value="logs">Logs</TabsTrigger>
          </TabsList>

          <ScrollArea className="h-[calc(80vh-180px)] mt-4">
            <TabsContent value="overview" className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div className="p-4 rounded-md border border-gray-800 bg-gray-900/50">
                  <div className="text-xs font-mono text-slate-400 mb-1 font-mono">STATUS</div>
                  <div className="flex items-center gap-2">
                    <div
                      className={`w-2 h-2 rounded-full ${agent.status === "active" ? "bg-green-500" : "bg-amber-500"} animate-pulse`}
                    />
                    <span className="font-mono text-sm font-mono uppercase">{agent.status}</span>
                  </div>
                </div>

                <div className="p-4 rounded-md border border-gray-800 bg-gray-900/50">
                  <div className="text-xs font-mono text-slate-400 mb-1 font-mono">IP ADDRESS</div>
                  <div className="font-mono text-sm font-mono">{agent.ip}</div>
                </div>

                <div className="p-4 rounded-md border border-gray-800 bg-gray-900/50">
                  <div className="text-xs font-mono text-slate-400 mb-1 font-mono">OPERATING SYSTEM</div>
                  <div className="font-mono text-sm font-mono">
                    {agent.os.toUpperCase()} / {agent.osVersion}
                  </div>
                </div>

                <div className="p-4 rounded-md border border-gray-800 bg-gray-900/50">
                  <div className="text-xs font-mono text-slate-400 mb-1 font-mono">AGENT VERSION</div>
                  <div className="font-mono text-sm font-mono">{agent.version}</div>
                </div>

                <div className="p-4 rounded-md border border-gray-800 bg-gray-900/50">
                  <div className="text-xs font-mono text-slate-400 mb-1 font-mono flex items-center gap-1">
                    <Clock className="w-3 h-3" />
                    LAST KEEP ALIVE
                  </div>
                  <div className="font-mono text-sm font-mono">
                    {agent.lastKeepAlive ? new Date(agent.lastKeepAlive).toLocaleString() : "Never"}
                  </div>
                </div>

                <div className="p-4 rounded-md border border-gray-800 bg-gray-900/50">
                  <div className="text-xs font-mono text-slate-400 mb-1 font-mono">REGISTERED</div>
                  <div className="font-mono text-sm font-mono">{new Date(agent.registerDate).toLocaleDateString()}</div>
                </div>

                <div className="p-4 rounded-md border border-gray-800 bg-gray-900/50">
                  <div className="text-xs font-mono text-slate-400 mb-1 font-mono">MANAGER</div>
                  <div className="font-mono text-sm font-mono">{agent.manager}</div>
                </div>

                <div className="p-4 rounded-md border border-gray-800 bg-gray-900/50">
                  <div className="text-xs font-mono text-slate-400 mb-1 font-mono">ALERT COUNT</div>
                  <div className="font-mono text-sm font-mono text-amber-500">{agent.alertCount}</div>
                </div>
              </div>

              <div className="p-4 rounded-md border border-gray-800 bg-gray-900/50">
                <div className="text-xs font-mono text-slate-400 mb-2 font-mono">GROUPS</div>
                <div className="flex flex-wrap gap-2">
                  {agent.group.map((g) => (
                    <Badge key={g} variant="outline" className="font-mono">
                      {g}
                    </Badge>
                  ))}
                </div>
              </div>

              <div className="flex gap-2">
                <Button onClick={() => onRestart(agent.id)} variant="outline" className="gap-2">
                  <RefreshCw className="w-4 h-4" />
                  Restart Agent
                </Button>
                <Button variant="outline" className="gap-2 bg-transparent">
                  <Terminal className="w-4 h-4" />
                  Execute Command
                </Button>
              </div>
            </TabsContent>

            <TabsContent value="config" className="space-y-4">
              <div className="p-4 rounded-md border border-gray-800 bg-gray-900/50">
                <div className="flex items-center gap-2 mb-3">
                  <FileCode className="w-4 h-4 text-slate-400" />
                  <span className="text-sm font-mono font-semibold">ossec.conf</span>
                </div>
                <pre className="text-xs font-mono text-slate-400 overflow-x-auto">
                  {`<ossec_config>
  <client>
    <server>
      <address>${agent.manager}</address>
      <port>1514</port>
      <protocol>tcp</protocol>
    </server>
  </client>
  
  <syscheck>
    <frequency>43200</frequency>
    <scan_on_start>yes</scan_on_start>
  </syscheck>
  
  <rootcheck>
    <frequency>43200</frequency>
  </rootcheck>
</ossec_config>`}
                </pre>
              </div>

              <div className="p-4 rounded-md border border-gray-800 bg-gray-900/50">
                <div className="text-xs font-mono text-slate-400 mb-2 font-mono">CONFIG CHECKSUM</div>
                <div className="font-mono text-xs font-mono break-all">
                  {agent.configSum || "ab4cf2d3e1f5a9b7c8d2e4f6a1b3c5d7e9f1a2b4"}
                </div>
              </div>
            </TabsContent>

            <TabsContent value="alerts" className="space-y-2">
              {[...Array(5)].map((_, i) => (
                <div key={i} className="p-3 rounded-md border border-gray-800 bg-gray-900/50">
                  <div className="flex items-start justify-between mb-2">
                    <Badge className="bg-amber-950/30 text-amber-400 border-amber-900/50">HIGH</Badge>
                    <span className="text-xs font-mono text-slate-400 font-mono">
                      {new Date(Date.now() - i * 300000).toLocaleTimeString()}
                    </span>
                  </div>
                  <div className="text-sm font-mono mb-1">Authentication failure detected</div>
                  <div className="text-xs font-mono text-slate-400 font-mono">Rule: 5710 | Level: 8</div>
                </div>
              ))}
            </TabsContent>

            <TabsContent value="logs" className="space-y-2">
              <div className="p-4 rounded-md border border-gray-800 bg-gray-900/50">
                <pre className="text-xs font-mono text-slate-400">
                  {`2024/11/09 14:23:45 INFO: Agent started successfully
2024/11/09 14:23:47 INFO: Connected to manager ${agent.manager}
2024/11/09 14:23:48 INFO: Syscheck scan initiated
2024/11/09 14:28:12 INFO: Keep alive sent
2024/11/09 14:33:12 INFO: Keep alive sent
2024/11/09 14:38:12 INFO: Keep alive sent`}
                </pre>
              </div>
            </TabsContent>
          </ScrollArea>
        </Tabs>
      </DialogContent>
    </Dialog>
  )
}
