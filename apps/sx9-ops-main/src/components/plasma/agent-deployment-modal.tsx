"use client"

import { Dialog, DialogContent, DialogHeader, DialogTitle } from "../ui/dialog"
import { Button } from "../ui/button"
import { Input } from "../ui/input"
import { Label } from "../ui/label"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "../ui/tabs"
import { Copy, Terminal } from "lucide-react"
import { useState } from "react"
import { useToast } from "../../hooks/use-toast"

interface AgentDeploymentModalProps {
  open: boolean
  onClose: () => void
  managerUrl: string
}

export function AgentDeploymentModal({ open, onClose, managerUrl }: AgentDeploymentModalProps) {
  const [agentName, setAgentName] = useState("")
  const [agentIP, setAgentIP] = useState("")
  const { toast } = useToast()

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text)
    toast({ description: "Copied to clipboard" })
  }

  const linuxInstall = `# Download and install Wazuh agent
curl -s https://packages.wazuh.com/4.x/apt/pool/main/w/wazuh-agent/wazuh-agent_4.7.0-1_amd64.deb -o wazuh-agent.deb
sudo dpkg -i wazuh-agent.deb

# Configure manager address
sudo sed -i "s/MANAGER_IP/${managerUrl}/g" /var/ossec/etc/ossec.conf

# Register agent
sudo /var/ossec/bin/agent-auth -m ${managerUrl} ${agentName ? `-A ${agentName}` : ""}

# Start agent
sudo systemctl daemon-reload
sudo systemctl enable wazuh-agent
sudo systemctl start wazuh-agent`

  const windowsInstall = `# Download Wazuh agent installer
Invoke-WebRequest -Uri https://packages.wazuh.com/4.x/windows/wazuh-agent-4.7.0-1.msi -OutFile wazuh-agent.msi

# Install agent
msiexec.exe /i wazuh-agent.msi /q WAZUH_MANAGER="${managerUrl}"${agentName ? ` WAZUH_AGENT_NAME="${agentName}"` : ""}

# Start service
NET START WazuhSvc`

  const macosInstall = `# Download and install Wazuh agent
curl -so wazuh-agent.pkg https://packages.wazuh.com/4.x/macos/wazuh-agent-4.7.0-1.pkg
sudo installer -pkg wazuh-agent.pkg -target /

# Configure manager
sudo /Library/Ossec/bin/agent-auth -m ${managerUrl}${agentName ? ` -A ${agentName}` : ""}

# Start agent
sudo /Library/Ossec/bin/wazuh-control start`

  const dockerInstall = `docker run -d \\
  --name wazuh-agent \\
  --restart=always \\
  --hostname $(hostname) \\
  -e WAZUH_MANAGER="${managerUrl}" \\
  ${agentName ? `-e WAZUH_AGENT_NAME="${agentName}" \\\n  ` : ""}wazuh/wazuh-agent:4.7.0`

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="max-w-3xl h-[80vh] bg-gray-900/95 backdrop-blur">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-3 font-mono">
            <Terminal className="w-5 h-5" />
            Deploy Wazuh Agent
          </DialogTitle>
        </DialogHeader>

        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="agent-name" className="text-xs font-mono">
                AGENT NAME (Optional)
              </Label>
              <Input
                id="agent-name"
                value={agentName}
                onChange={(e) => setAgentName(e.target.value)}
                placeholder="web-server-01"
                className="font-mono text-sm font-mono"
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="agent-ip" className="text-xs font-mono">
                AGENT IP (Optional)
              </Label>
              <Input
                id="agent-ip"
                value={agentIP}
                onChange={(e) => setAgentIP(e.target.value)}
                placeholder="10.0.1.15"
                className="font-mono text-sm font-mono"
              />
            </div>
          </div>

          <Tabs defaultValue="linux" className="flex-1">
            <TabsList className="grid w-full grid-cols-4">
              <TabsTrigger value="linux">Linux</TabsTrigger>
              <TabsTrigger value="windows">Windows</TabsTrigger>
              <TabsTrigger value="macos">macOS</TabsTrigger>
              <TabsTrigger value="docker">Docker</TabsTrigger>
            </TabsList>

            <div className="mt-4">
              <TabsContent value="linux" className="space-y-2">
                <div className="relative p-4 rounded-md border border-gray-800 bg-black/40">
                  <Button
                    size="sm"
                    variant="ghost"
                    className="absolute top-2 right-2"
                    onClick={() => copyToClipboard(linuxInstall)}
                  >
                    <Copy className="w-3 h-3" />
                  </Button>
                  <pre className="text-xs font-mono text-green-400 overflow-x-auto pr-12">{linuxInstall}</pre>
                </div>
                <div className="text-xs font-mono text-slate-400">
                  Supports: Ubuntu, Debian, RHEL, CentOS, Amazon Linux
                </div>
              </TabsContent>

              <TabsContent value="windows" className="space-y-2">
                <div className="relative p-4 rounded-md border border-gray-800 bg-black/40">
                  <Button
                    size="sm"
                    variant="ghost"
                    className="absolute top-2 right-2"
                    onClick={() => copyToClipboard(windowsInstall)}
                  >
                    <Copy className="w-3 h-3" />
                  </Button>
                  <pre className="text-xs font-mono text-blue-400 overflow-x-auto pr-12">{windowsInstall}</pre>
                </div>
                <div className="text-xs font-mono text-slate-400">Run PowerShell as Administrator</div>
              </TabsContent>

              <TabsContent value="macos" className="space-y-2">
                <div className="relative p-4 rounded-md border border-gray-800 bg-black/40">
                  <Button
                    size="sm"
                    variant="ghost"
                    className="absolute top-2 right-2"
                    onClick={() => copyToClipboard(macosInstall)}
                  >
                    <Copy className="w-3 h-3" />
                  </Button>
                  <pre className="text-xs font-mono text-slate-400 overflow-x-auto pr-12">{macosInstall}</pre>
                </div>
                <div className="text-xs font-mono text-slate-400">Supports: macOS 10.15 (Catalina) and later</div>
              </TabsContent>

              <TabsContent value="docker" className="space-y-2">
                <div className="relative p-4 rounded-md border border-gray-800 bg-black/40">
                  <Button
                    size="sm"
                    variant="ghost"
                    className="absolute top-2 right-2"
                    onClick={() => copyToClipboard(dockerInstall)}
                  >
                    <Copy className="w-3 h-3" />
                  </Button>
                  <pre className="text-xs font-mono text-cyan-400 overflow-x-auto pr-12">{dockerInstall}</pre>
                </div>
                <div className="text-xs font-mono text-slate-400">Containerized agent deployment</div>
              </TabsContent>
            </div>
          </Tabs>
        </div>
      </DialogContent>
    </Dialog>
  )
}
