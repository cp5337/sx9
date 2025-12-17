import { Search, Shield, Activity, Eye, Database } from "lucide-react";

export interface RadialMenuProps {
  x: number;
  y: number;
  onClose: () => void;
  onAction: (action: string) => void;
  isOpen: boolean;
}

export default function RadialMenu({
  x,
  y,
  onClose,
  onAction,
  isOpen,
}: RadialMenuProps) {
  if (!isOpen) return null;

  const actions = [
    {
      id: "inspect",
      icon: Eye,
      label: "Inspect",
      color: "text-blue-400",
      bg: "hover:bg-blue-900/50",
    },
    {
      id: "correlate",
      icon: Activity,
      label: "Correlate (Nonagon)",
      color: "text-purple-400",
      bg: "hover:bg-purple-900/50",
    },
    {
      id: "xref_sledis",
      icon: Database,
      label: "Sledis X-Ref",
      color: "text-orange-400",
      bg: "hover:bg-orange-900/50",
    },
    {
      id: "trace",
      icon: Search,
      label: "Trace Path",
      color: "text-green-400",
      bg: "hover:bg-green-900/50",
    },
    {
      id: "secure",
      icon: Shield,
      label: "Secure",
      color: "text-red-400",
      bg: "hover:bg-red-900/50",
    },
  ];

  // Position items in a semi-circle or full circle
  const radius = 55; // Reduced from 80
  const startAngle = -90; // Top
  const stepAngle = 360 / actions.length;

  return (
    <div
      className="absolute z-50 pointer-events-none"
      style={{ left: x, top: y }}
    >
      {/* Backdrop to catch clicks outside - pointer-events-auto re-enabled here */}
      <div className="fixed inset-0 pointer-events-auto" onClick={onClose} />

      <div className="relative pointer-events-auto">
        {/* Tactical Knurled Ring - Rotating Background */}
        <div className="absolute -translate-x-1/2 -translate-y-1/2 w-32 h-32 pointer-events-none opacity-30">
          <svg
            viewBox="0 0 100 100"
            className="w-full h-full animate-[spin_20s_linear_infinite]"
          >
            {/* Knurled Outer Edge */}
            <circle
              cx="50"
              cy="50"
              r="48"
              fill="none"
              stroke="#60a5fa"
              strokeWidth="2"
              strokeDasharray="3 3"
            />
            {/* Inner Guide Ring */}
            <circle
              cx="50"
              cy="50"
              r="42"
              fill="none"
              stroke="#60a5fa"
              strokeWidth="0.5"
              strokeOpacity="0.5"
            />
          </svg>
        </div>

        {/* Center Hub - ACOG Sight Picture */}
        <div className="absolute -translate-x-1/2 -translate-y-1/2 w-8 h-8 flex items-center justify-center pointer-events-none z-10">
          {/* Red Chevron Reticle */}
          <svg
            viewBox="0 0 24 24"
            className="w-full h-full text-red-500 drop-shadow-[0_0_5px_rgba(239,68,68,0.8)]"
          >
            <path d="M12 6 L17 16 L12 13 L7 16 Z" fill="currentColor" />
            {/* Range hashes */}
            <path
              d="M12 16 L12 22 M8 18 L16 18 M10 20 L14 20"
              stroke="currentColor"
              strokeWidth="1"
              strokeOpacity="0.6"
            />
          </svg>
        </div>

        {/* Menu Items */}
        {actions.map((action, index) => {
          const angle = (startAngle + index * stepAngle) * (Math.PI / 180);
          const itemX = Math.cos(angle) * radius;
          const itemY = Math.sin(angle) * radius;

          return (
            <button
              key={action.id}
              onClick={(e) => {
                e.stopPropagation();
                onAction(action.id);
              }}
              className={`absolute -translate-x-1/2 -translate-y-1/2 w-8 h-8 rounded-full bg-dark-surface border border-dark-border flex items-center justify-center transition-all duration-150 hover:scale-110 shadow-md ${action.color} ${action.bg}`}
              style={{
                left: itemX,
                top: itemY,
                animation: `slideOut 0.15s ease-out ${index * 0.03}s backwards`,
              }}
              title={action.label}
            >
              <action.icon size={15} />
            </button>
          );
        })}
      </div>

      <style>{`
            @keyframes slideOut {
                from { opacity: 0; transform: translate(-50%, -50%) scale(0.5); left: 0; top: 0; }
            }
            @keyframes spin {
                from { transform: rotate(0deg); }
                to { transform: rotate(360deg); }
            }
        `}</style>
    </div>
  );
}
