"use client"

import { Dialog, DialogContent, DialogHeader, DialogTitle } from "../ui/dialog"
import { Button } from "../ui/button"
import { Input } from "../ui/input"
import { Label } from "../ui/label"
import { ScrollArea } from "../ui/scroll-area"
import { Badge } from "../ui/badge"
import type { WazuhManager } from "../../types/plasma"
import { Plus, Trash2, Activity } from "lucide-react"
import { useState } from "react"

interface WazuhConfigModalProps {
  open: boolean
  onClose: () => void
  managers: WazuhManager[]
  onAddManager: (manager: Omit<WazuhManager, "id" | "agentCount" | "lastSync">) => void
  onRemoveManager: (id: string) => void
}

export function WazuhConfigModal({ open, onClose, managers, onAddManager, onRemoveManager }: WazuhConfigModalProps) {
  const [name, setName] = useState("")
  const [url, setUrl] = useState("")
  const [apiKey, setApiKey] = useState("")

  const handleAdd = () => {
    if (name && url) {
      onAddManager({ name, url, apiKey, enabled: true })
      setName("")
      setUrl("")
      setApiKey("")
    }
  }

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="max-w-2xl h-[70vh] bg-gray-900/95 backdrop-blur">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-3 font-mono">
            <Activity className="w-5 h-5" />
            Wazuh Manager Configuration
          </DialogTitle>
        </DialogHeader>

        <div className="space-y-4 flex-1">
          <div className="p-4 rounded-md border border-gray-800 bg-gray-900/50 space-y-3">
            <h3 className="text-sm font-mono font-semibold">Add Manager</h3>
            <div className="grid gap-3">
              <div className="space-y-2">
                <Label htmlFor="manager-name" className="text-xs font-mono">
                  NAME
                </Label>
                <Input
                  id="manager-name"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="wazuh-manager-01"
                  className="font-mono text-sm font-mono"
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="manager-url" className="text-xs font-mono">
                  URL
                </Label>
                <Input
                  id="manager-url"
                  value={url}
                  onChange={(e) => setUrl(e.target.value)}
                  placeholder="https://wazuh.internal:55000"
                  className="font-mono text-sm font-mono"
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="manager-apikey" className="text-xs font-mono">
                  API KEY (Optional)
                </Label>
                <Input
                  id="manager-apikey"
                  type="password"
                  value={apiKey}
                  onChange={(e) => setApiKey(e.target.value)}
                  placeholder="Enter API key"
                  className="font-mono text-sm font-mono"
                />
              </div>
              <Button onClick={handleAdd} className="gap-2">
                <Plus className="w-4 h-4" />
                Add Manager
              </Button>
            </div>
          </div>

          <div className="space-y-2">
            <h3 className="text-sm font-mono font-semibold">Configured Managers</h3>
            <ScrollArea className="h-64">
              <div className="space-y-2">
                {managers.map((manager) => (
                  <div key={manager.id} className="p-3 rounded-md border border-gray-800 bg-gray-900/50">
                    <div className="flex items-start justify-between">
                      <div className="flex-1">
                        <div className="flex items-center gap-2 mb-1">
                          <span className="font-mono text-sm font-mono font-semibold">{manager.name}</span>
                          {manager.enabled && (
                            <Badge className="bg-green-950/30 text-green-400 border-green-900/50 text-xs font-mono">ACTIVE</Badge>
                          )}
                        </div>
                        <div className="text-xs font-mono text-slate-400 font-mono mb-1">{manager.url}</div>
                        <div className="text-xs font-mono text-slate-400">
                          Agents: {manager.agentCount} | Last sync:{" "}
                          {manager.lastSync ? new Date(manager.lastSync).toLocaleTimeString() : "Never"}
                        </div>
                      </div>
                      <Button
                        size="sm"
                        variant="ghost"
                        onClick={() => onRemoveManager(manager.id)}
                        className="text-destructive"
                      >
                        <Trash2 className="w-4 h-4" />
                      </Button>
                    </div>
                  </div>
                ))}
              </div>
            </ScrollArea>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
}
