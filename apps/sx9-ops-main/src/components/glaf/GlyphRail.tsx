import { ReactNode } from "react";
import { LucideIcon } from "lucide-react";
import { ServiceStatusIndicator } from "./ServiceStatusIndicator";

interface GlyphRailItem {
  id: string;
  icon: LucideIcon;
  label: string;
  position: "left" | "right";
  statusIndicator?:
    | "connected"
    | "running"
    | "disconnected"
    | "stopped"
    | "error"
    | "warning";
  pulse?: boolean;
}

export interface GlyphRailGroup {
  items: GlyphRailItem[];
  divider?: boolean;
}

interface GlyphRailProps {
  items: GlyphRailItem[];
  activeId: string | null;
  onItemClick: (id: string) => void;
  position: "left" | "right";
}

export default function GlyphRail({
  items,
  activeId,
  onItemClick,
  position,
}: GlyphRailProps) {
  const railItems = items.filter((item) => item.position === position);

  const renderGlyph = (item: GlyphRailItem) => {
    const Icon = item.icon;
    const isActive = activeId === item.id;

    return (
      <button
        key={item.id}
        onClick={() => onItemClick(item.id)}
        className={`group relative flex items-center justify-center h-10 w-12 transition-all duration-150 glyph-icon ${
          isActive
            ? "bg-blue-900/10 dark:bg-blue-900/20 text-glow-blue shadow-glow-inset-active"
            : "text-dark-text-secondary opacity-60 hover:opacity-100 hover:shadow-glow-inset hover:bg-dark-elevated/50"
        }`}
        title={item.label}
      >
        <Icon size={20} className="flex-shrink-0" />

        {item.statusIndicator && (
          <div className="absolute top-1 right-1">
            <ServiceStatusIndicator
              status={item.statusIndicator}
              size="sm"
              pulse={item.pulse}
            />
          </div>
        )}

        <div
          className={`absolute ${
            position === "left" ? "left-full ml-2" : "right-full mr-2"
          } px-2 py-1 bg-dark-elevated text-dark-text-primary text-2xs rounded whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50`}
        >
          {item.label}
        </div>
      </button>
    );
  };

  const renderDivider = (key: string) => (
    <div key={key} className="h-px bg-dark-border mx-3 my-4" />
  );

  return (
    <div
      className={`flex flex-col bg-dark-surface w-12 ${
        position === "left" ? "border-r" : "border-l"
      } border-dark-border`}
    >
      <div className="flex-1 flex flex-col py-2">
        {railItems.map((item, index) => (
          <div key={item.id}>
            {renderGlyph(item)}
            {index < railItems.length - 1 &&
              item.id.endsWith("-divider") &&
              renderDivider(`divider-${index}`)}
          </div>
        ))}
      </div>
    </div>
  );
}

interface SidePanelProps {
  isOpen: boolean;
  onClose?: () => void;
  position: "left" | "right";
  title: string;
  children: ReactNode;
}

export function SidePanel({
  isOpen,
  position,
  title,
  children,
}: SidePanelProps) {
  if (!isOpen) return null;

  return (
    <div
      className={`flex flex-col bg-white dark:bg-dark-surface w-80 h-full ${
        position === "left" ? "border-r" : "border-l"
      } border-gray-200 dark:border-dark-border flex-shrink-0`}
    >
      <div className="flex items-center justify-between px-4 py-3 border-b border-gray-200 dark:border-dark-border">
        <h3 className="text-sm font-semibold text-gray-900 dark:text-dark-text-primary">
          {title}
        </h3>
      </div>
      <div className="flex-1 overflow-auto">{children}</div>
    </div>
  );
}
