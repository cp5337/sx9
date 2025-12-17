import React, { useState } from "react";
import { useNavigate, useLocation } from "react-router-dom";
import GlyphRail, { GlyphRailItem, SidePanel } from "./GlyphRail";
import {
  Command,
  Hexagon,
  Zap,
  Brain,
  Satellite,
  Anchor,
  Activity,
  Radio,
  Cloud,
  Settings,
  Link as LinkIcon,
} from "lucide-react";

/* 
  RFC-9304: 9 Workspaces mapped to glyphs
*/
const WORKSPACE_ITEMS: GlyphRailItem[] = [
  { id: "command", label: "Command", icon: Command, position: "left" },
  { id: "graph", label: "Graph", icon: Hexagon, position: "left" },
  { id: "forge", label: "Forge", icon: Zap, position: "left" },
  { id: "intel", label: "Intel", icon: Brain, position: "left" },
  { id: "orbital", label: "Orbital", icon: Satellite, position: "left" },
  { id: "maritime", label: "Maritime", icon: Anchor, position: "left" },
  { id: "tunnel", label: "Tunnel", icon: Cloud, position: "left" }, // Using Cloud as placeholder
  { id: "spectrum", label: "Spectrum", icon: Radio, position: "left" },
  { id: "timeline", label: "Timeline", icon: Activity, position: "left" },
];

const RIGHT_ITEMS: GlyphRailItem[] = [
  {
    id: "connections",
    label: "Connections",
    icon: LinkIcon,
    position: "right",
    statusIndicator: "connected",
  },
  { id: "settings", label: "Settings", icon: Settings, position: "right" },
];

interface WorkbenchLayoutProps {
  children?: React.ReactNode;
}

export const WorkbenchLayout: React.FC<WorkbenchLayoutProps> = ({ children }) => {
  const navigate = useNavigate();
  const location = useLocation();
  const [rightPanel, setRightPanel] = useState<string | null>(null);

  // Derive active workspace from path
  const currentPath = location.pathname.split("/")[2] || "dashboard";
  const activeLeft = WORKSPACE_ITEMS.find(i => i.id === currentPath)?.id || "command";

  const handleLeftClick = (id: string) => {
    navigate(`/workspace/${id}`);
  };

  const handleRightClick = (id: string) => {
    if (rightPanel === id) {
      setRightPanel(null); // Toggle off
    } else {
      setRightPanel(id);
    }
  };

  return (
    <div className="flex h-screen bg-dark-bg text-dark-text-primary overflow-hidden">
      {/* Left Rail */}
      <GlyphRail
        items={WORKSPACE_ITEMS}
        activeId={activeLeft}
        onItemClick={handleLeftClick}
        position="left"
      />

      {/* Main Content */}
      <div className="flex-1 flex flex-col min-w-0 bg-gray-100 dark:bg-gray-900 relative">
        <main className="flex-1 overflow-auto p-4">{children}</main>
      </div>

      {/* Right Panel (Collapsible) */}
      <SidePanel
        isOpen={!!rightPanel}
        title={rightPanel ? rightPanel.charAt(0).toUpperCase() + rightPanel.slice(1) : ""}
        position="right"
      >
        <div className="p-4">
          {rightPanel === "settings" && <p>Settings Panel Content</p>}
          {rightPanel === "connections" && <p>Connection Status: Stable</p>}
        </div>
      </SidePanel>

      {/* Right Rail */}
      <GlyphRail
        items={RIGHT_ITEMS}
        activeId={rightPanel}
        onItemClick={handleRightClick}
        position="right"
      />
    </div>
  );
};
