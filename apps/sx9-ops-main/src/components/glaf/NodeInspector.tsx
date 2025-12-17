import { useState, useEffect, useRef, CSSProperties } from "react";
import type { GraphNode } from "../lib/database.types";

// ═══════════════════════════════════════════════════════════
// NODE INSPECTOR - SlotGraph Right-Click Card
// ═══════════════════════════════════════════════════════════

interface Props {
  node: GraphNode;
  position?: { x: number; y: number }; // Optional, defaults to center or calculated
  onUpdate?: (key: string, value: string) => void;
  onClose: () => void;
}

// Smart dropdown options by field name
const OPTIONS: Record<string, string[]> = {
  plane: ["1", "2", "3"],
  slot: ["1", "2", "3", "4"],
  status: ["nominal", "degraded", "offline", "maintenance"],
  domain: ["kinetic", "cyber", "cognitive", "geospatial"],
  link: ["locked", "acquiring", "degraded", "rf_backup", "lost"],
};

// Read-only fields (no input/select)
const READONLY = ["alt", "margin", "rate", "elev", "range", "id"];

// Field groupings
const SECTIONS: Record<string, string[]> = {
  identity: ["id", "plane", "slot"],
  orbital: ["raan", "anomaly", "alt"],
  status: ["status", "domain", "link"],
  metrics: ["margin", "rate", "elev"],
  // Catch-all for others not listed?
};

// ───────────────────────────────────────────────────────────
// Styles (Slate palette)
// ───────────────────────────────────────────────────────────

const S: Record<string, CSSProperties> = {
  card: {
    position: "fixed", // Changed from absolute to fixed for viewport positioning
    width: 240,
    minHeight: 200,
    background: "#1a1a24",
    border: "1px solid #3d3d4d",
    borderRadius: 2,
    fontFamily: '"JetBrains Mono", Consolas, monospace',
    fontSize: 11,
    lineHeight: 1.4,
    color: "#b0b0b0",
    resize: "both",
    // overflow: "auto", // Handle in body
    zIndex: 1000,
    boxShadow: "0 0 20px rgba(0,0,0,0.5)",
  },
  header: {
    padding: "6px 8px",
    borderBottom: "1px solid #3d3d4d",
    background: "#22222e",
    display: "flex",
    justifyContent: "space-between",
    alignItems: "center",
    cursor: "move",
    userSelect: "none",
  },
  unicode: { color: "#569cd6", letterSpacing: 0.5 },
  nodeType: { color: "#4ec9b0", textTransform: "uppercase", fontSize: 10 },
  close: { color: "#666", cursor: "pointer", fontSize: 14 },
  body: { padding: "6px 8px", overflow: "auto", maxHeight: "60vh" },
  section: {
    color: "#666",
    fontSize: 9,
    textTransform: "uppercase",
    letterSpacing: 1,
    marginBottom: 4,
    marginTop: 4,
  },
  divider: { borderTop: "1px solid #33333f", margin: "6px 0" },
  row: { display: "flex", marginBottom: 4, alignItems: "center" },
  key: { color: "#9cdcfe", minWidth: 70 },
  colon: { color: "#666", marginRight: 4 },
  value: { color: "#ce9178", flex: 1, wordBreak: "break-all" },
  input: {
    flex: 1,
    height: 18,
    padding: "2px 4px",
    background: "#282836",
    border: "1px solid #3d3d4d",
    borderRadius: 1,
    color: "#ce9178",
    fontFamily: "inherit",
    fontSize: 11,
    outline: "none",
  },
};

// ───────────────────────────────────────────────────────────
// Component
// ───────────────────────────────────────────────────────────

export function NodeInspector({
  node,
  position: initialPos,
  onUpdate,
  onClose,
}: Props) {
  const [props, setProps] = useState(node.properties);
  const [isExpanded, setIsExpanded] = useState(true);

  // Draggable Logic
  const [position, setPosition] = useState(
    initialPos || { x: window.innerWidth / 2 - 120, y: 100 }
  );
  const [isDragging, setIsDragging] = useState(false);
  const dragStart = useRef({ x: 0, y: 0 });

  useEffect(() => {
    if (initialPos) setPosition(initialPos);
  }, [initialPos]);

  const handleMouseDown = (e: React.MouseEvent) => {
    // Only drag from header
    setIsDragging(true);
    dragStart.current = {
      x: e.clientX - position.x,
      y: e.clientY - position.y,
    };
  };

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isDragging) return;
      setPosition({
        x: e.clientX - dragStart.current.x,
        y: e.clientY - dragStart.current.y,
      });
    };
    const handleMouseUp = () => {
      setIsDragging(false);
    };

    if (isDragging) {
      window.addEventListener("mousemove", handleMouseMove);
      window.addEventListener("mouseup", handleMouseUp);
    }
    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("mouseup", handleMouseUp);
    };
  }, [isDragging]);

  const handleChange = (key: string, val: string) => {
    setProps((p) => ({ ...p, [key]: val }));
    if (onUpdate) onUpdate(key, val);
  };

  const Field = ({ k }: { k: string }) => {
    const v = String(props[k] ?? "");
    const opts = OPTIONS[k];
    const readonly = READONLY.includes(k);

    return (
      <div style={S.row}>
        <span style={S.key}>{k}</span>
        <span style={S.colon}>:</span>
        {readonly ? (
          <span style={S.value}>{v}</span>
        ) : opts ? (
          <select
            style={S.input}
            value={v}
            onChange={(e) => handleChange(k, e.target.value)}
          >
            {opts.map((o) => (
              <option key={o} value={o}>
                {o}
              </option>
            ))}
          </select>
        ) : (
          <input
            style={S.input}
            value={v}
            onChange={(e) => handleChange(k, e.target.value)}
            spellCheck={false}
          />
        )}
      </div>
    );
  };

  const Section = ({ name, fields }: { name: string; fields: string[] }) => {
    const visible = fields.filter((f) => f in props);
    if (!visible.length) return null;
    return (
      <>
        <div style={S.section}>{name}</div>
        {visible.map((k) => (
          <Field key={k} k={k} />
        ))}
        <div style={S.divider} />
      </>
    );
  };

  // Safe unicode derivation
  const unicode = "0xE305"; // Placeholder or derive from node.id hash?

  return (
    <div
      style={{
        ...S.card,
        left: position.x,
        top: position.y,
        height: isExpanded ? "auto" : "auto",
      }}
    >
      <div style={S.header} onMouseDown={handleMouseDown}>
        <div style={{ display: "flex", gap: "8px", alignItems: "center" }}>
          <span style={S.unicode}>{unicode}</span>
          <span style={S.nodeType}>{node.label}</span>
        </div>
        <div style={{ display: "flex", gap: "8px", alignItems: "center" }}>
          <span
            style={{ ...S.close, fontSize: 10 }}
            onClick={() => setIsExpanded(!isExpanded)}
          >
            {isExpanded ? "[-]" : "[+]"}
          </span>
          <span style={S.close} onClick={onClose}>
            ×
          </span>
        </div>
      </div>

      {isExpanded && (
        <div style={S.body}>
          {Object.entries(SECTIONS).map(([name, fields]) => (
            <Section key={name} name={name} fields={fields} />
          ))}

          {/* Catch-all for properties not in sections */}
          <div style={S.section}>OTHER</div>
          {Object.keys(props)
            .filter((k) => !Object.values(SECTIONS).flat().includes(k))
            .map((k) => (
              <Field key={k} k={k} />
            ))}
        </div>
      )}
    </div>
  );
}
