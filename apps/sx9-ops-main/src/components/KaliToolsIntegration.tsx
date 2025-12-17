import React, { useState, useEffect } from "react";
import {
  Target,
  Shield,
  Zap,
  Globe,
  Activity,
  Database,
  AlertTriangle,
  CheckCircle,
  Clock,
  Play,
  Square,
  Settings,
  Download,
  Upload,
  RefreshCw,
  Package,
  Terminal,
  Code,
  Bug,
  Eye,
  Lock,
  Unlock,
} from "lucide-react";

interface KaliTool {
  id: string;
  name: string;
  category:
    | "reconnaissance"
    | "exploitation"
    | "post-exploitation"
    | "forensics"
    | "wireless"
    | "web-apps";
  description: string;
  version: string;
  status: "installed" | "available" | "running" | "error";
  icon: React.ReactNode;
  command: string;
  dependencies: string[];
  lastUsed?: Date;
  output?: string;
}

interface ToolCategory {
  name: string;
  icon: React.ReactNode;
  color: string;
  description: string;
}

interface KaliToolsIntegrationProps {
  activeTask?: any;
}

const KaliToolsIntegration: React.FC<KaliToolsIntegrationProps> = ({ activeTask }) => {
  const [tools, setTools] = useState<KaliTool[]>([]);
  const [selectedTool, setSelectedTool] = useState<KaliTool | null>(null);
  const [activeCategory, setActiveCategory] = useState<string>("all");
  const [isInstalling, setIsInstalling] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState("");

  const categories: Record<string, ToolCategory> = {
    reconnaissance: {
      name: "Reconnaissance",
      icon: <Eye size={16} />,
      color: "bg-blue-500",
      description: "Information gathering and enumeration tools",
    },
    exploitation: {
      name: "Exploitation",
      icon: <Bug size={16} />,
      color: "bg-red-500",
      description: "Vulnerability exploitation and attack tools",
    },
    "post-exploitation": {
      name: "Post-Exploitation",
      icon: <Lock size={16} />,
      color: "bg-purple-500",
      description: "Post-compromise and persistence tools",
    },
    forensics: {
      name: "Forensics",
      icon: <Database size={16} />,
      color: "bg-green-500",
      description: "Digital forensics and analysis tools",
    },
    wireless: {
      name: "Wireless",
      icon: <Globe size={16} />,
      color: "bg-yellow-500",
      description: "Wireless network testing tools",
    },
    "web-apps": {
      name: "Web Applications",
      icon: <Code size={16} />,
      color: "bg-orange-500",
      description: "Web application security testing tools",
    },
  };

  useEffect(() => {
    // Initialize with demo tools
    const demoTools: KaliTool[] = [
      {
        id: "1",
        name: "Nmap",
        category: "reconnaissance",
        description: "Network discovery and security auditing",
        version: "7.94",
        status: "installed",
        icon: <Target size={16} />,
        command: "nmap -sV -sC [target]",
        dependencies: [],
        lastUsed: new Date(),
      },
      {
        id: "2",
        name: "Metasploit",
        category: "exploitation",
        description: "Penetration testing framework",
        version: "6.3.4",
        status: "installed",
        icon: <Bug size={16} />,
        command: "msfconsole",
        dependencies: ["ruby", "postgresql"],
        lastUsed: new Date(Date.now() - 86400000),
      },
      {
        id: "3",
        name: "Wireshark",
        category: "forensics",
        description: "Network protocol analyzer",
        version: "4.0.3",
        status: "installed",
        icon: <Database size={16} />,
        command: "wireshark",
        dependencies: ["gtk3"],
        lastUsed: new Date(Date.now() - 172800000),
      },
      {
        id: "4",
        name: "Aircrack-ng",
        category: "wireless",
        description: "Wireless network security suite",
        version: "1.7",
        status: "available",
        icon: <Globe size={16} />,
        command: "aircrack-ng [capture-file]",
        dependencies: ["libpcap"],
        lastUsed: new Date(Date.now() - 259200000),
      },
      {
        id: "5",
        name: "Burp Suite",
        category: "web-apps",
        description: "Web application security testing platform",
        version: "2023.1",
        status: "installed",
        icon: <Code size={16} />,
        command: "burpsuite",
        dependencies: ["java"],
        lastUsed: new Date(Date.now() - 432000000),
      },
      {
        id: "6",
        name: "John the Ripper",
        category: "post-exploitation",
        description: "Password cracking tool",
        version: "1.9.0",
        status: "installed",
        icon: <Lock size={16} />,
        command: "john [hash-file]",
        dependencies: [],
        lastUsed: new Date(Date.now() - 604800000),
      },
    ];

    setTools(demoTools);
  }, []);

  // Filter tools based on active task's requirements if present
  const filteredTools = tools.filter(tool => {
    // If a task is selected, check if tool is in its required toolchain
    if (activeTask && activeTask.kali_tools && activeTask.kali_tools.length > 0) {
      // Simple name match for now
      const isRequired = activeTask.kali_tools.some(
        (t: string) =>
          t.toLowerCase().includes(tool.name.toLowerCase()) ||
          tool.name.toLowerCase().includes(t.toLowerCase())
      );
      if (isRequired) return true;
      // If filtering by task, maybe show only required tools?
      // Or promote them? Let's show only required if any match found, otherwise fallback to all

      const anyMatchStub = activeTask.kali_tools.some((t: string) =>
        tools.some(existing => existing.name.toLowerCase().includes(t.toLowerCase()))
      );

      if (anyMatchStub) {
        return isRequired;
      }
    }

    const matchesCategory = activeCategory === "all" || tool.category === activeCategory;
    const matchesSearch =
      tool.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      tool.description.toLowerCase().includes(searchTerm.toLowerCase());
    return matchesCategory && matchesSearch;
  });

  const installTool = async (toolId: string) => {
    setIsInstalling(toolId);

    // Simulate installation
    setTimeout(
      () => {
        setTools(prev =>
          prev.map(tool => (tool.id === toolId ? { ...tool, status: "installed" as const } : tool))
        );
        setIsInstalling(null);
      },
      2000 + Math.random() * 3000
    );
  };

  const runTool = async (tool: KaliTool) => {
    setSelectedTool(tool);

    // Simulate tool execution
    setTimeout(() => {
      const output = simulateToolOutput(tool);
      setTools(prev =>
        prev.map(t =>
          t.id === tool.id ? { ...t, status: "running" as const, output, lastUsed: new Date() } : t
        )
      );
    }, 1000);
  };

  const simulateToolOutput = (tool: KaliTool): string => {
    switch (tool.name.toLowerCase()) {
      case "nmap":
        return `Starting Nmap 7.94 ( https://nmap.org )
Nmap scan report for 192.168.1.1
Host is up (0.00047s latency).
Not shown: 998 closed ports
PORT    STATE SERVICE     VERSION
22/tcp  open  ssh         OpenSSH 8.2p1 Ubuntu 4ubuntu0.5
80/tcp  open  http        Apache httpd 2.4.41
443/tcp open  https       Apache httpd 2.4.41

Nmap done: 1 IP address (1 host up) scanned in 2.34 seconds`;

      case "metasploit":
        return `msf6 > search apache
[*] Searching for modules matching 'apache'...
[+] Found 45 modules matching 'apache'
msf6 > use exploit/multi/http/apache_mod_cgi_bash_env_exec
msf6 exploit(multi/http/apache_mod_cgi_bash_env_exec) > set RHOSTS 192.168.1.1
RHOSTS => 192.168.1.1
msf6 exploit(multi/http/apache_mod_cgi_bash_env_exec) > exploit`;

      default:
        return `Running ${tool.name}...\nTool execution completed successfully.`;
    }
  };

  const getStatusColor = (status: KaliTool["status"]) => {
    switch (status) {
      case "installed":
        return "text-green-500";
      case "available":
        return "text-yellow-500";
      case "running":
        return "text-blue-500";
      case "error":
        return "text-red-500";
      default:
        return "text-gray-500";
    }
  };

  const getStatusIcon = (status: KaliTool["status"]) => {
    switch (status) {
      case "installed":
        return <CheckCircle size={14} />;
      case "available":
        return <Package size={14} />;
      case "running":
        return <Activity size={14} className="animate-pulse" />;
      case "error":
        return <AlertTriangle size={14} />;
      default:
        return <Clock size={14} />;
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-semibold">Kali Tools Integration</h2>
          <p className="text-gray-600">Manage and execute Kali Linux security tools</p>
        </div>
        <div className="flex items-center space-x-2">
          <button className="px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600">
            <RefreshCw size={14} className="mr-1" />
            Refresh
          </button>
          <button className="px-3 py-1 bg-green-500 text-white rounded hover:bg-green-600">
            <Download size={14} className="mr-1" />
            Update Tools
          </button>
        </div>
      </div>

      {/* Search and Filters */}
      <div className="flex items-center space-x-4">
        <div className="flex-1">
          <input
            type="text"
            placeholder="Search tools..."
            value={searchTerm}
            onChange={e => setSearchTerm(e.target.value)}
            className="w-full px-3 py-2 border rounded-lg"
          />
        </div>
        <div className="flex space-x-2">
          <button
            onClick={() => setActiveCategory("all")}
            className={`px-3 py-2 rounded-lg ${
              activeCategory === "all" ? "bg-blue-500 text-white" : "bg-gray-200 text-gray-700"
            }`}
          >
            All
          </button>
          {Object.entries(categories).map(([key, category]) => (
            <button
              key={key}
              onClick={() => setActiveCategory(key)}
              className={`px-3 py-2 rounded-lg flex items-center ${
                activeCategory === key ? "bg-blue-500 text-white" : "bg-gray-200 text-gray-700"
              }`}
            >
              {category.icon}
              <span className="ml-1">{category.name}</span>
            </button>
          ))}
        </div>
      </div>

      {/* Tools Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {filteredTools.map(tool => (
          <div key={tool.id} className="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
            <div className="flex items-start justify-between mb-3">
              <div className="flex items-center">
                {tool.icon}
                <div className="ml-2">
                  <h3 className="font-semibold">{tool.name}</h3>
                  <p className="text-sm text-gray-600">{tool.version}</p>
                </div>
              </div>
              <div className={`flex items-center ${getStatusColor(tool.status)}`}>
                {getStatusIcon(tool.status)}
                <span className="ml-1 text-xs capitalize">{tool.status}</span>
              </div>
            </div>

            <p className="text-sm text-gray-600 mb-3">{tool.description}</p>

            <div className="text-xs text-gray-500 mb-3">
              <div>
                Command: <code className="bg-gray-100 px-1 rounded">{tool.command}</code>
              </div>
              {tool.dependencies.length > 0 && (
                <div>Dependencies: {tool.dependencies.join(", ")}</div>
              )}
            </div>

            <div className="flex items-center justify-between">
              <div className="flex space-x-2">
                {tool.status === "available" ? (
                  <button
                    onClick={() => installTool(tool.id)}
                    disabled={isInstalling === tool.id}
                    className="px-2 py-1 bg-green-500 text-white text-xs rounded hover:bg-green-600 disabled:opacity-50"
                  >
                    {isInstalling === tool.id ? "Installing..." : "Install"}
                  </button>
                ) : (
                  <button
                    onClick={() => runTool(tool)}
                    className="px-2 py-1 bg-blue-500 text-white text-xs rounded hover:bg-blue-600"
                  >
                    <Play size={12} className="mr-1" />
                    Run
                  </button>
                )}
                <button
                  onClick={() => setSelectedTool(tool)}
                  className="px-2 py-1 bg-gray-500 text-white text-xs rounded hover:bg-gray-600"
                >
                  <Settings size={12} className="mr-1" />
                  Config
                </button>
              </div>

              {tool.lastUsed && (
                <div className="text-xs text-gray-500">
                  Last used: {tool.lastUsed.toLocaleDateString()}
                </div>
              )}
            </div>
          </div>
        ))}
      </div>

      {/* Tool Details Modal */}
      {selectedTool && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-2xl w-full mx-4 max-h-[80vh] overflow-y-auto">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg font-semibold">{selectedTool.name}</h3>
              <button
                onClick={() => setSelectedTool(null)}
                className="text-gray-500 hover:text-gray-700"
              >
                ×
              </button>
            </div>

            <div className="space-y-4">
              <div>
                <h4 className="font-semibold mb-2">Description</h4>
                <p className="text-gray-600">{selectedTool.description}</p>
              </div>

              <div>
                <h4 className="font-semibold mb-2">Command</h4>
                <code className="bg-gray-100 px-2 py-1 rounded text-sm">
                  {selectedTool.command}
                </code>
              </div>

              {selectedTool.dependencies.length > 0 && (
                <div>
                  <h4 className="font-semibold mb-2">Dependencies</h4>
                  <div className="flex flex-wrap gap-2">
                    {selectedTool.dependencies.map(dep => (
                      <span
                        key={dep}
                        className="px-2 py-1 bg-blue-100 text-blue-800 rounded text-xs"
                      >
                        {dep}
                      </span>
                    ))}
                  </div>
                </div>
              )}

              {selectedTool.output && (
                <div>
                  <h4 className="font-semibold mb-2">Output</h4>
                  <pre className="bg-gray-100 p-3 rounded text-sm overflow-x-auto">
                    {selectedTool.output}
                  </pre>
                </div>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default KaliToolsIntegration;
