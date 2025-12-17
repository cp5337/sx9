import React, { useState, useEffect, useRef } from "react";
import {
  Map,
  Network,
  Target,
  Server,
  Terminal,
  Database,
  Activity,
  Globe,
  Brain,
  Send,
  X,
  Settings,
  History,
  Download,
  RotateCcw,
  Grid,
  Wifi,
  WifiOff,
} from "lucide-react";
import CTASGraphAnimation from "@/components/CTASGraphAnimation";
import SimpleMapbox from "@/components/SimpleMapbox";
import MultiCLI from "@/components/MultiCLI";
import { synaptixCore } from "@/services/SynaptixCoreClient";
import { CTAS7_API_ENDPOINTS } from "@/services/ctas7-api-integration";

interface DashboardProps {
  view: "map" | "grid" | "graph" | "cognigraph";
}

interface DeployedAsset {
  id: string;
  name: string;
  type: "raptor" | "vkali" | "database" | "service";
  status: "active" | "inactive" | "deploying" | "error";
  location: {
    lat: number;
    lng: number;
  };
  metrics: {
    cpu: number;
    memory: number;
    network: number;
  };
  lastSeen: string;
}

const Dashboard: React.FC<DashboardProps> = ({ view }) => {
  const [error, setError] = React.useState<Error | null>(null);
  const [deployedAssets, setDeployedAssets] = useState<DeployedAsset[]>([]);
  const [selectedAsset, setSelectedAsset] = useState<DeployedAsset | null>(null);
  const [systemMetrics, setSystemMetrics] = useState({
    totalAssets: 0,
    activeAssets: 0,
    alerts: 0,
    threats: 0,
  });

  // AI CLI Chat State
  const [chatMessages, setChatMessages] = useState<
    Array<{ id: string; type: "user" | "ai"; content: string; timestamp: Date }>
  >([]);
  const [chatInput, setChatInput] = useState("");
  const [isChatOpen, setIsChatOpen] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const [neuralMuxStatus, setNeuralMuxStatus] = useState<"connected" | "disconnected">(
    "disconnected"
  );
  const chatRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Fetch real deployed assets from Docker/API
    const fetchAssets = async () => {
      try {
        // Fetch from API Gateway (using standardized endpoint)
        const response = await fetch(`${CTAS7_API_ENDPOINTS.mcpBackend}/api/assets`);
        if (response.ok) {
          const data = await response.json();
          const assets = data.assets || [];
          console.log(`✅ Dashboard: Loaded ${assets.length} real assets`);
          setDeployedAssets(assets);
          setSystemMetrics({
            totalAssets: assets.length,
            activeAssets: assets.filter((a: DeployedAsset) => a.status === "active").length,
            alerts: data.alerts || 0,
            threats: data.threats || 0,
          });
        } else {
          console.warn("⚠️  Dashboard: API not available, showing empty state");
          setDeployedAssets([]);
          setSystemMetrics({ totalAssets: 0, activeAssets: 0, alerts: 0, threats: 0 });
        }
      } catch (err) {
        console.error("❌ Dashboard: Failed to fetch assets:", err);
        setDeployedAssets([]);
        setSystemMetrics({ totalAssets: 0, activeAssets: 0, alerts: 0, threats: 0 });
      }
    };

    fetchAssets();

    // Refresh every 10 seconds
    const interval = setInterval(fetchAssets, 10000);
    return () => clearInterval(interval);
  }, []);

  // Initialize SynaptixCore Neural Mux
  useEffect(() => {
    const initializeSynaptixCore = async () => {
      setNeuralMuxStatus(
        synaptixCore.getConnectionStatus() === "connected" ? "connected" : "disconnected"
      );

      // Subscribe to neural mux alerts for dashboard
      synaptixCore.subscribeToAlerts(alertData => {
        setChatMessages(prev => [
          ...prev,
          {
            id: Date.now().toString(),
            type: "ai",
            content: `🚨 Neural Mux Alert: ${alertData.title || "Security event detected"}`,
            timestamp: new Date(),
          },
        ]);
      });

      // Get initial system status from neural mux
      try {
        const toolMetrics = await synaptixCore.getToolPerformanceMetrics();
        if (toolMetrics.success) {
          setChatMessages(prev => [
            ...prev,
            {
              id: Date.now().toString(),
              type: "ai",
              content: "🤖 Neural Mux connected. System metrics loaded successfully.",
              timestamp: new Date(),
            },
          ]);
        }
      } catch (error) {
        console.log("Neural Mux not available, using local data");
      }
    };

    initializeSynaptixCore();

    return () => {
      synaptixCore.unsubscribeFromAlerts();
    };
  }, []);

  if (error) {
    return (
      <div className="p-4 bg-red-100 border border-red-400 text-red-700 rounded">
        <h2 className="font-bold mb-2">Error Loading Dashboard</h2>
        <p>{error.message}</p>
      </div>
    );
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case "active":
        return "text-green-400";
      case "inactive":
        return "text-gray-400";
      case "deploying":
        return "text-yellow-400";
      case "error":
        return "text-red-400";
      default:
        return "text-gray-400";
    }
  };

  const getTypeIcon = (type: string) => {
    switch (type) {
      case "raptor":
        return <Target size={16} />;
      case "vkali":
        return <Terminal size={16} />;
      case "database":
        return <Database size={16} />;
      case "service":
        return <Server size={16} />;
      default:
        return <Activity size={16} />;
    }
  };

  const openTerminal = (asset: DeployedAsset) => {
    // Open terminal for the selected asset
    console.log(`Opening terminal for ${asset.name}`);
    // This would integrate with the AI CLI or terminal system
  };

  const sendChatMessage = async (message: string) => {
    if (!message.trim()) return;

    const userMessage = {
      id: Date.now().toString(),
      type: "user" as const,
      content: message,
      timestamp: new Date(),
    };

    setChatMessages(prev => [...prev, userMessage]);
    setChatInput("");
    setIsProcessing(true);

    // Send to real AI CLI via Neural Mux Bridge
    try {
      // Use standardized bridge port (15001) or Neural Mux if direct
      const response = await fetch("http://localhost:15001/ai-cli/chat", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ message }),
      });

      if (response.ok) {
        const data = await response.json();
        const aiResponse = {
          id: (Date.now() + 1).toString(),
          type: "ai" as const,
          content: data.response || "No response from AI",
          timestamp: new Date(),
        };
        setChatMessages(prev => [...prev, aiResponse]);
      } else {
        const errorResponse = {
          id: (Date.now() + 1).toString(),
          type: "ai" as const,
          content: "❌ AI CLI not available. Start Neural Mux service.",
          timestamp: new Date(),
        };
        setChatMessages(prev => [...prev, errorResponse]);
      }
    } catch (err) {
      const errorResponse = {
        id: (Date.now() + 1).toString(),
        type: "ai" as const,
        content: `❌ Connection failed: ${err instanceof Error ? err.message : "Unknown error"}`,
        timestamp: new Date(),
      };
      setChatMessages(prev => [...prev, errorResponse]);
    } finally {
      setIsProcessing(false);
    }
  };

  const clearChat = () => {
    setChatMessages([]);
  };

  const downloadChat = () => {
    const chatLog = chatMessages
      .map(msg => `[${msg.timestamp.toLocaleString()}] ${msg.type.toUpperCase()}: ${msg.content}`)
      .join("\n\n");

    const blob = new Blob([chatLog], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `ctas-chat-${new Date().toISOString().split("T")[0]}.txt`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const [mapHeight, setMapHeight] = useState(72); // percentage
  const [isResizing, setIsResizing] = useState(false);

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);
  };

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isResizing) return;

      const container = document.getElementById("dashboard-main-container");
      if (!container) return;

      const containerRect = container.getBoundingClientRect();
      const relativeY = e.clientY - containerRect.top - 48; // Subtract stats bar height
      const newHeightPercent = (relativeY / (containerRect.height - 48)) * 100;

      if (newHeightPercent >= 30 && newHeightPercent <= 80) {
        setMapHeight(newHeightPercent);
      }
    };

    const handleMouseUp = () => {
      setIsResizing(false);
    };

    if (isResizing) {
      document.addEventListener("mousemove", handleMouseMove);
      document.addEventListener("mouseup", handleMouseUp);
    }

    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };
  }, [isResizing]);

  return (
    <div id="dashboard-main-container" className="h-full flex flex-col bg-gray-900">
      {/* Compact Stats Bar */}
      <div className="flex-none flex bg-gray-800 border-b border-gray-700 h-10 pt-2">
        <div className="flex-1 flex items-center justify-between px-2 border-r border-gray-700">
          <span className="text-xs text-gray-400">Total</span>
          <span className="text-sm font-bold">{systemMetrics.totalAssets}</span>
        </div>
        <div className="flex-1 flex items-center justify-between px-2 border-r border-gray-700">
          <span className="text-xs text-gray-400">Active</span>
          <span className="text-sm font-bold text-green-500">{systemMetrics.activeAssets}</span>
        </div>
        <div className="flex-1 flex items-center justify-between px-2 border-r border-gray-700">
          <span className="text-xs text-gray-400">Alerts</span>
          <span className="text-sm font-bold text-yellow-500">{systemMetrics.alerts}</span>
        </div>
        <div className="flex-1 flex items-center justify-between px-2">
          <span className="text-xs text-gray-400">Threats</span>
          <span className="text-sm font-bold text-red-500">{systemMetrics.threats}</span>
        </div>
      </div>

      {/* Main Content - FULL BLEED, NO PADDING */}
      <div className="flex-1 flex flex-col">
        {/* Map - Full width, resizable */}
        <div className="bg-gray-900" style={{ height: `${mapHeight}%` }}>
          {view === "map" && <SimpleMapbox />}
          {view === "grid" && <CTASGraphAnimation />}
          {view === "graph" && <CTASGraphAnimation />}
          {view === "cognigraph" && <CTASGraphAnimation />}
        </div>

        {/* Resize Handle - 1px line */}
        <div
          onMouseDown={handleMouseDown}
          className={`cursor-ns-resize ${isResizing ? "bg-blue-500/30 h-1" : "bg-gray-700/50 h-px hover:bg-blue-500/20"}`}
        />

        {/* Bottom Panel - Multi-CLI */}
        <MultiCLI className="flex-1" />
      </div>

      {/* Asset Detail Modal */}
      {selectedAsset && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full max-h-[80vh] overflow-y-auto">
            <div className="p-6">
              <div className="flex items-center justify-between mb-4">
                <div className="flex items-center gap-3">
                  {getTypeIcon(selectedAsset.type)}
                  <h2 className="text-xl font-bold text-gray-900 dark:text-white">
                    {selectedAsset.name}
                  </h2>
                </div>
                <button
                  onClick={() => setSelectedAsset(null)}
                  className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                >
                  <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M6 18L18 6M6 6l12 12"
                    />
                  </svg>
                </button>
              </div>

              <div className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400">Type</h3>
                    <p className="text-gray-900 dark:text-white capitalize">{selectedAsset.type}</p>
                  </div>
                  <div>
                    <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400">Status</h3>
                    <p className={`font-medium ${getStatusColor(selectedAsset.status)}`}>
                      {selectedAsset.status}
                    </p>
                  </div>
                  <div>
                    <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400">
                      Location
                    </h3>
                    <p className="text-gray-900 dark:text-white">
                      {selectedAsset.location.lat.toFixed(4)},{" "}
                      {selectedAsset.location.lng.toFixed(4)}
                    </p>
                  </div>
                  <div>
                    <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400">
                      Last Seen
                    </h3>
                    <p className="text-gray-900 dark:text-white">
                      {new Date(selectedAsset.lastSeen).toLocaleString()}
                    </p>
                  </div>
                </div>

                <div>
                  <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-2">
                    Performance Metrics
                  </h3>
                  <div className="grid grid-cols-3 gap-4">
                    <div className="bg-gray-50 dark:bg-gray-700 p-3 rounded">
                      <div className="text-2xl font-bold text-blue-600">
                        {selectedAsset.metrics.cpu}%
                      </div>
                      <div className="text-xs text-gray-500">CPU Usage</div>
                    </div>
                    <div className="bg-gray-50 dark:bg-gray-700 p-3 rounded">
                      <div className="text-2xl font-bold text-green-600">
                        {selectedAsset.metrics.memory}%
                      </div>
                      <div className="text-xs text-gray-500">Memory Usage</div>
                    </div>
                    <div className="bg-gray-50 dark:bg-gray-700 p-3 rounded">
                      <div className="text-2xl font-bold text-purple-600">
                        {selectedAsset.metrics.network}%
                      </div>
                      <div className="text-xs text-gray-500">Network Usage</div>
                    </div>
                  </div>
                </div>

                <div className="flex gap-2 pt-4 border-t border-gray-200 dark:border-gray-700">
                  <button
                    onClick={() => openTerminal(selectedAsset)}
                    className="flex-1 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
                  >
                    Open Terminal
                  </button>
                  <button
                    onClick={() => setSelectedAsset(null)}
                    className="px-4 py-2 bg-gray-300 dark:bg-gray-600 text-gray-700 dark:text-gray-300 rounded-md hover:bg-gray-400 dark:hover:bg-gray-500 transition-colors"
                  >
                    Close
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default Dashboard;
