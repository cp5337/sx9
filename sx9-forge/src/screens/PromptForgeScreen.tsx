/**
 * SX9 Prompt Forge - Tauri Version
 *
 * Converted from Next.js to Tauri:
 * - Removed "use client" directive
 * - Replaced fetch() API calls with Tauri invoke()
 * - Kept Redux for UI state (intelligence status)
 * - Backend operations now go through Rust commands
 */

import React, { useState, useCallback, useEffect, useMemo } from "react";
import { useSelector, useDispatch } from "react-redux";
import { invoke } from "@tauri-apps/api/core";
import {
  ChevronLeft,
  ChevronRight,
  FilePlus,
  FileEdit,
  Cpu,
  Zap,
  Target,
  Send,
  Settings,
  Database,
  Brain,
  Wrench,
  Shield,
  Copy,
  Play,
  RefreshCw,
  CheckCircle,
} from "lucide-react";
import {
  selectLeptoseStatus,
  selectChromaStatus,
} from "../store/intelligence/selectors";
import {
  connectLeptose,
  connectChromaDB,
  refreshStatus,
} from "../store/intelligence/actions";

// ============================================================================
// CONSTANTS
// ============================================================================

const GLYPH = 48;
const RAIL_CLOSED = 56;
const RAIL_OPEN = 300;
const HEADER_H = 48;
const STATUS_H = 32;

const C = {
  bg: "#0F172A",
  bg2: "#1E293B",
  bg3: "#334155",
  editor: "#0B1120",
  text: "#F8FAFC",
  text2: "#94A3B8",
  text3: "#64748B",
  cyan: "#22D3EE",
  green: "#22C55E",
  red: "#EF4444",
  border: "#334155",
};

// ============================================================================
// TAURI COMMAND INTERFACES
// ============================================================================

interface SavePromptResult {
  success: boolean;
  path: string;
  error?: string;
}

interface LinearIssueResult {
  success: boolean;
  issue_id: string;
  url: string;
  error?: string;
}

interface SlackNotifyResult {
  success: boolean;
  error?: string;
}

// ============================================================================
// MAIN COMPONENT
// ============================================================================

export const PromptForgeScreen: React.FC = () => {
  const dispatch = useDispatch();
  const leptose = useSelector(selectLeptoseStatus);
  const chroma = useSelector(selectChromaStatus);

  // Rails
  const [leftOpen, setLeftOpen] = useState(false);
  const [rightOpen, setRightOpen] = useState(false);
  const [leftTab, setLeftTab] = useState("harness");
  const [rightTab, setRightTab] = useState("intel");

  // Form
  const [title, setTitle] = useState("");
  const [rfc, setRfc] = useState("RFC-");
  const [phase, setPhase] = useState("IMPLEMENT");
  const [objective, setObjective] = useState("");

  // Phase 1: Critical Controls
  const [harnessMode, setHarnessMode] = useState("Build");
  const [persona, setPersona] = useState("FORGE");
  const [model, setModel] = useState("Claude Sonnet 4.5");
  const [temperature, setTemperature] = useState(0.2);
  const [useHarness, setUseHarness] = useState(false);

  // Phase 2: Integration Controls
  const [linearTeam, setLinearTeam] = useState("");
  const [createLinearIssue, setCreateLinearIssue] = useState(true);
  const [slackChannel, setSlackChannel] = useState("#all-synaptixops");
  const [enableSlack, setEnableSlack] = useState(true);
  const [contextSources, setContextSources] = useState<{
    memory: boolean;
    linear: boolean;
    drive: boolean;
    filesystem: boolean;
    web: boolean;
  }>({
    memory: true,
    linear: true,
    drive: false,
    filesystem: true,
    web: false,
  });

  // Output
  const [feedback, setFeedback] = useState("");
  const [timestamp, setTimestamp] = useState("");

  // Action handlers for buttons
  const showFeedback = useCallback((msg: string) => {
    setFeedback(msg);
    setTimeout(() => setFeedback(""), 2000);
  }, []);

  const handleNewPrompt = useCallback((action: string) => {
    if (action === "blank") {
      setTitle("");
      setRfc("RFC-");
      setPhase("IMPLEMENT");
      setObjective("");
      showFeedback("New blank prompt");
    } else if (action === "template") {
      showFeedback("Loading templates...");
      // TODO: invoke("list_templates")
    } else if (action === "clone") {
      showFeedback("Select prompt to clone...");
    }
  }, [showFeedback]);

  const handleEditPrompt = useCallback((action: string) => {
    if (action === "load") {
      invoke("open_file_dialog", { filters: ["yaml", "yml"] })
        .then(() => showFeedback("Loading..."))
        .catch(() => showFeedback("Load cancelled"));
    } else if (action === "recent") {
      showFeedback("Loading recent prompts...");
    } else if (action === "history") {
      showFeedback("Loading version history...");
    }
  }, [showFeedback]);

  const handleMission = useCallback((action: string) => {
    showFeedback(`Mission: ${action}`);
  }, [showFeedback]);

  const handleDeploy = useCallback((action: string) => {
    if (action === "execute") {
      generate();
    } else if (action === "schedule") {
      showFeedback("Scheduling not yet implemented");
    } else if (action === "draft") {
      invoke("save_prompt", {
        filename: `draft-${rfc || "prompt"}-${Date.now()}.yaml`,
        content: output,
        workdir: "drafts",
      })
        .then(() => showFeedback("Draft saved"))
        .catch(() => showFeedback("Save failed"));
    }
  }, [generate, rfc, output, showFeedback]);

  const handleSettings = useCallback((action: string) => {
    showFeedback(`Settings: ${action}`);
  }, [showFeedback]);

  const handleData = useCallback((action: string) => {
    if (action === "vector") {
      dispatch(refreshStatus());
      showFeedback("Refreshing vector status...");
    } else if (action === "cache") {
      showFeedback("Cache cleared");
    } else if (action === "export") {
      showFeedback("Exporting prompts...");
    } else if (action === "import") {
      showFeedback("Import not yet implemented");
    }
  }, [dispatch, showFeedback]);

  const handleToolToggle = useCallback((tool: string) => {
    showFeedback(`${tool} toggled`);
  }, [showFeedback]);

  // Legacy harness state for backward compatibility
  const harness = `${harnessMode} & Implement`;

  useEffect(() => {
    setTimestamp(new Date().toISOString());
    dispatch(connectLeptose());
    dispatch(connectChromaDB());
  }, [dispatch]);

  // Generate YAML
  const output = useMemo(
    () => `# SX9-PROMPT v4.0
# Generated: ${timestamp || "Loading..."}

header:
  title: "${title}"
  rfc: ${rfc}
  phase: ${phase}

utilization:
  harness: ${harness}
  persona: ${persona}
  model: ${model}
  temperature: ${temperature}
  mode: ${useHarness ? "HARNESS" : "SUPERVISED"}

mission:
  objective: "${objective}"

integration:
  linear:
    team: "${linearTeam}"
    create_issue: ${createLinearIssue}
  slack:
    channel: "${slackChannel}"
    enabled: ${enableSlack}

context:
  sources:
    memory: ${contextSources.memory}
    linear: ${contextSources.linear}
    drive: ${contextSources.drive}
    filesystem: ${contextSources.filesystem}
    web: ${contextSources.web}
`,
    [
      title,
      rfc,
      phase,
      harness,
      persona,
      model,
      temperature,
      useHarness,
      objective,
      linearTeam,
      createLinearIssue,
      slackChannel,
      enableSlack,
      contextSources,
      timestamp,
    ]
  );

  // Copy to clipboard via Tauri
  const copy = useCallback(async () => {
    try {
      await invoke("copy_to_clipboard", { content: output });
      setFeedback("COPIED");
      setTimeout(() => setFeedback(""), 2000);
    } catch (e) {
      // Fallback to browser API
      await navigator.clipboard.writeText(output);
      setFeedback("COPIED");
      setTimeout(() => setFeedback(""), 2000);
    }
  }, [output]);

  // Generate - Save, Create Linear Issue, Notify Slack via Tauri commands
  const generate = useCallback(async () => {
    try {
      setFeedback("SAVING...");

      // 1. Save to disk via Tauri command
      const saveResult = await invoke<SavePromptResult>("save_prompt", {
        filename: `${rfc || "prompt"}-${Date.now()}.yaml`,
        content: output,
        workdir: "prompts",
      });

      if (!saveResult.success) {
        throw new Error(saveResult.error || "Failed to save file");
      }

      // 2. Create Linear issue via Tauri command (if enabled)
      let linearUrl = "";
      if (createLinearIssue) {
        setFeedback("CREATING ISSUE...");
        const linearResult = await invoke<LinearIssueResult>(
          "create_linear_issue_forge",
          {
            title: title || "New Prompt",
            description: `${objective}\n\n---\nRFC: ${rfc}\nPhase: ${phase}\nHarness: ${harness}\nPersona: ${persona}`,
            team_id: linearTeam || "SX9",
          }
        );

        if (linearResult.success) {
          linearUrl = linearResult.url;
        }
      }

      // 3. Notify Slack via Tauri command (if enabled)
      if (enableSlack) {
        setFeedback("NOTIFYING SLACK...");
        await invoke<SlackNotifyResult>("notify_slack", {
          channel: slackChannel,
          message: `🚀 New Prompt Generated\n*${title || "Untitled"}* (${rfc})\n${linearUrl || "No Linear issue created"}`,
        });
      }

      setFeedback("✓ DEPLOYED");
      setTimeout(() => setFeedback(""), 3000);
    } catch (error: any) {
      setFeedback(`ERROR: ${error.message || error}`);
      setTimeout(() => setFeedback(""), 5000);
    }
  }, [
    output,
    title,
    rfc,
    phase,
    objective,
    harness,
    persona,
    linearTeam,
    createLinearIssue,
    slackChannel,
    enableSlack,
  ]);

  return (
    <div style={S.container}>
      {/* HEADER */}
      <header style={S.header}>
        <span style={S.logo}>SX9 Prompt Forge</span>
        <div style={S.headerCenter}>
          <input
            style={S.headerInput}
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder="Title"
          />
          <input
            style={{ ...S.headerInput, width: 100 }}
            value={rfc}
            onChange={(e) => setRfc(e.target.value)}
          />
          <select
            style={S.headerSelect}
            value={phase}
            onChange={(e) => setPhase(e.target.value)}
          >
            <option>PLAN</option>
            <option>IMPLEMENT</option>
            <option>TEST</option>
            <option>DEPLOY</option>
          </select>
        </div>
        <div style={S.headerRight}>
          <Btn icon={Copy} onClick={copy} />
          <Btn icon={Play} onClick={generate} accent />
        </div>
      </header>

      {/* MAIN */}
      <div style={S.main}>
        {/* LEFT RAIL */}
        <div style={{ ...S.rail, width: leftOpen ? RAIL_OPEN : RAIL_CLOSED }}>
          <div style={S.railHeader}>
            <IconBtn
              icon={leftOpen ? ChevronLeft : ChevronRight}
              onClick={() => setLeftOpen(!leftOpen)}
            />
            {leftOpen && <span style={S.railTitle}>ACTIONS</span>}
          </div>
          <div style={S.railBody}>
            {leftOpen ? (
              <LeftContent
                tab={leftTab}
                setTab={setLeftTab}
                harnessMode={harnessMode}
                setHarnessMode={setHarnessMode}
                persona={persona}
                setPersona={setPersona}
                model={model}
                setModel={setModel}
                temperature={temperature}
                setTemperature={setTemperature}
                useHarness={useHarness}
                setUseHarness={setUseHarness}
                linearTeam={linearTeam}
                setLinearTeam={setLinearTeam}
                createLinearIssue={createLinearIssue}
                setCreateLinearIssue={setCreateLinearIssue}
                slackChannel={slackChannel}
                setSlackChannel={setSlackChannel}
                enableSlack={enableSlack}
                setEnableSlack={setEnableSlack}
                contextSources={contextSources}
                setContextSources={setContextSources}
                onNewPrompt={handleNewPrompt}
                onEditPrompt={handleEditPrompt}
                onMission={handleMission}
                onDeploy={handleDeploy}
                onSettings={handleSettings}
                onData={handleData}
              />
            ) : (
              <LeftIcons
                active={leftTab}
                onClick={(id) => {
                  setLeftTab(id);
                  setLeftOpen(true);
                }}
              />
            )}
          </div>
        </div>

        {/* CENTER */}
        <div style={S.center}>
          <div style={S.editor}>
            <div style={S.lineNums}>
              {output.split("\n").map((_, i) => (
                <div key={i} style={S.lineNum}>
                  {i + 1}
                </div>
              ))}
            </div>
            <pre style={S.code}>{output}</pre>
          </div>
          <div style={S.mission}>
            <label style={S.label}>MISSION OBJECTIVE</label>
            <textarea
              style={S.textarea}
              value={objective}
              onChange={(e) => setObjective(e.target.value)}
              placeholder="Describe the mission..."
              rows={3}
            />
          </div>
        </div>

        {/* RIGHT RAIL */}
        <div
          style={{
            ...S.rail,
            width: rightOpen ? RAIL_OPEN : RAIL_CLOSED,
            borderLeft: `1px solid ${C.border}`,
            borderRight: "none",
          }}
        >
          <div style={S.railHeader}>
            {rightOpen && <span style={S.railTitle}>CONTEXT</span>}
            <IconBtn
              icon={rightOpen ? ChevronRight : ChevronLeft}
              onClick={() => setRightOpen(!rightOpen)}
            />
          </div>
          <div style={S.railBody}>
            {rightOpen ? (
              <RightContent tab={rightTab} setTab={setRightTab} onToolToggle={handleToolToggle} />
            ) : (
              <RightIcons
                active={rightTab}
                onClick={(id) => {
                  setRightTab(id);
                  setRightOpen(true);
                }}
              />
            )}
          </div>
        </div>
      </div>

      {/* STATUS BAR */}
      <footer style={S.status}>
        <Dot color={leptose.status === "ready" ? C.green : C.text3} />
        <span style={S.statusText}>Inference</span>
        <div style={S.divider} />
        <Dot color={chroma.status === "ready" ? C.green : C.text3} />
        <span style={S.statusText}>Vector</span>
        <div style={S.divider} />
        <span style={S.shortCode}>{rfc.slice(-4) || "----"}</span>
        <div style={{ flex: 1 }} />
        <span style={S.statusText}>{feedback || "READY"}</span>
        <IconBtn
          icon={RefreshCw}
          onClick={() => dispatch(refreshStatus())}
          small
        />
      </footer>
    </div>
  );
};

// ============================================================================
// SUB COMPONENTS
// ============================================================================

const Btn: React.FC<{ icon: any; onClick: () => void; accent?: boolean }> = ({
  icon: Icon,
  onClick,
  accent,
}) => (
  <button
    style={{
      ...S.btn,
      backgroundColor: accent ? C.cyan + "20" : "transparent",
    }}
    onClick={onClick}
  >
    <Icon size={20} color={accent ? C.cyan : C.text2} />
  </button>
);

const IconBtn: React.FC<{
  icon: any;
  onClick: () => void;
  small?: boolean;
}> = ({ icon: Icon, onClick, small }) => (
  <button
    style={{ ...S.iconBtn, width: small ? 24 : 32, height: small ? 24 : 32 }}
    onClick={onClick}
  >
    <Icon size={small ? 14 : 18} color={C.text2} />
  </button>
);

const Dot: React.FC<{ color: string }> = ({ color }) => (
  <div
    style={{ width: 8, height: 8, borderRadius: "50%", backgroundColor: color }}
  />
);

const GlyphBtn: React.FC<{
  icon: any;
  active: boolean;
  onClick: () => void;
}> = ({ icon: Icon, active, onClick }) => (
  <button
    style={{ ...S.glyphBtn, backgroundColor: active ? C.bg3 : "transparent" }}
    onClick={onClick}
  >
    <Icon size={24} color={active ? C.cyan : C.text2} />
  </button>
);

// Left rail icons (collapsed)
const LeftIcons: React.FC<{
  active: string;
  onClick: (id: string) => void;
}> = ({ active, onClick }) => (
  <div style={S.iconCol}>
    <GlyphBtn
      icon={FilePlus}
      active={active === "new"}
      onClick={() => onClick("new")}
    />
    <GlyphBtn
      icon={FileEdit}
      active={active === "edit"}
      onClick={() => onClick("edit")}
    />
    <div style={S.sep} />
    <GlyphBtn
      icon={Cpu}
      active={active === "harness"}
      onClick={() => onClick("harness")}
    />
    <GlyphBtn
      icon={Zap}
      active={active === "persona"}
      onClick={() => onClick("persona")}
    />
    <GlyphBtn
      icon={Target}
      active={active === "mission"}
      onClick={() => onClick("mission")}
    />
    <GlyphBtn
      icon={Send}
      active={active === "deploy"}
      onClick={() => onClick("deploy")}
    />
    <div style={{ flex: 1 }} />
    <div style={S.sep} />
    <GlyphBtn
      icon={Settings}
      active={active === "settings"}
      onClick={() => onClick("settings")}
    />
    <GlyphBtn
      icon={Database}
      active={active === "data"}
      onClick={() => onClick("data")}
    />
  </div>
);

// Right rail icons (collapsed)
const RightIcons: React.FC<{
  active: string;
  onClick: (id: string) => void;
}> = ({ active, onClick }) => (
  <div style={S.iconCol}>
    <GlyphBtn
      icon={Brain}
      active={active === "intel"}
      onClick={() => onClick("intel")}
    />
    <GlyphBtn
      icon={Wrench}
      active={active === "tools"}
      onClick={() => onClick("tools")}
    />
    <GlyphBtn
      icon={Shield}
      active={active === "threats"}
      onClick={() => onClick("threats")}
    />
    <div style={S.sep} />
    <GlyphBtn
      icon={CheckCircle}
      active={active === "qa"}
      onClick={() => onClick("qa")}
    />
  </div>
);

// Left rail content (expanded)
const LeftContent: React.FC<{
  tab: string;
  setTab: (t: string) => void;
  harnessMode: string;
  setHarnessMode: (h: string) => void;
  persona: string;
  setPersona: (p: string) => void;
  model: string;
  setModel: (m: string) => void;
  temperature: number;
  setTemperature: (t: number) => void;
  useHarness: boolean;
  setUseHarness: (u: boolean) => void;
  linearTeam: string;
  setLinearTeam: (t: string) => void;
  createLinearIssue: boolean;
  setCreateLinearIssue: (c: boolean) => void;
  slackChannel: string;
  setSlackChannel: (c: string) => void;
  enableSlack: boolean;
  setEnableSlack: (e: boolean) => void;
  contextSources: {
    memory: boolean;
    linear: boolean;
    drive: boolean;
    filesystem: boolean;
    web: boolean;
  };
  setContextSources: (s: {
    memory: boolean;
    linear: boolean;
    drive: boolean;
    filesystem: boolean;
    web: boolean;
  }) => void;
  onNewPrompt: (action: string) => void;
  onEditPrompt: (action: string) => void;
  onMission: (action: string) => void;
  onDeploy: (action: string) => void;
  onSettings: (action: string) => void;
  onData: (action: string) => void;
}> = ({
  tab,
  setTab,
  harnessMode,
  setHarnessMode,
  persona,
  setPersona,
  model,
  setModel,
  temperature,
  setTemperature,
  useHarness,
  setUseHarness,
  linearTeam,
  setLinearTeam,
  createLinearIssue,
  setCreateLinearIssue,
  slackChannel,
  setSlackChannel,
  enableSlack,
  setEnableSlack,
  contextSources,
  setContextSources,
  onNewPrompt,
  onEditPrompt,
  onMission,
  onDeploy,
  onSettings,
  onData,
}) => (
  <div style={S.content}>
    <div style={S.tabs}>
      {["harness", "persona", "agents", "linear", "slack", "context"].map(
        (t) => (
          <button
            key={t}
            style={{
              ...S.tab,
              borderBottom: tab === t ? `2px solid ${C.cyan}` : "none",
              fontSize: 11,
              padding: "8px 6px",
            }}
            onClick={() => setTab(t)}
          >
            {t.toUpperCase()}
          </button>
        )
      )}
    </div>
    <div style={S.tabContent}>
      {/* NEW PROMPT */}
      {tab === "new" && (
        <div style={{ padding: 12 }}>
          <label style={S.label}>New Prompt</label>
          <p style={{ fontSize: 12, color: C.text3, marginBottom: 12 }}>
            Create a new prompt from template or scratch.
          </p>
          <div style={S.options}>
            <button style={S.optBtn} onClick={() => onNewPrompt("blank")}>Blank Prompt</button>
            <button style={S.optBtn} onClick={() => onNewPrompt("template")}>From Template</button>
            <button style={S.optBtn} onClick={() => onNewPrompt("clone")}>Clone Existing</button>
          </div>
        </div>
      )}

      {/* EDIT PROMPT */}
      {tab === "edit" && (
        <div style={{ padding: 12 }}>
          <label style={S.label}>Edit Prompt</label>
          <p style={{ fontSize: 12, color: C.text3, marginBottom: 12 }}>
            Load and modify existing prompts.
          </p>
          <div style={S.options}>
            <button style={S.optBtn} onClick={() => onEditPrompt("load")}>Load from File</button>
            <button style={S.optBtn} onClick={() => onEditPrompt("recent")}>Recent Prompts</button>
            <button style={S.optBtn} onClick={() => onEditPrompt("history")}>Version History</button>
          </div>
        </div>
      )}

      {/* PHASE 1: HARNESS MODE */}
      {tab === "harness" && (
        <div style={{ padding: 12 }}>
          <label style={S.label}>Harness Mode</label>
          <div style={S.options}>
            {["Build", "Research", "Security", "Planning"].map((h) => (
              <button
                key={h}
                style={{
                  ...S.optBtn,
                  borderColor: harnessMode === h ? C.cyan : C.border,
                  backgroundColor:
                    harnessMode === h ? C.cyan + "10" : "transparent",
                }}
                onClick={() => setHarnessMode(h)}
              >
                {h}
              </button>
            ))}
          </div>
          <div style={{ marginTop: 16 }}>
            <label
              style={{
                ...S.label,
                display: "flex",
                alignItems: "center",
                gap: 8,
              }}
            >
              <input
                type="checkbox"
                checked={useHarness}
                onChange={(e) => setUseHarness(e.target.checked)}
                style={{ width: 16, height: 16 }}
              />
              Use Agent Harness
            </label>
          </div>
        </div>
      )}

      {/* PHASE 1: PERSONA - SX9 SPECIALIZED AGENTS (CLSGS Annex A.2) */}
      {tab === "persona" && (
        <div style={{ padding: 12 }}>
          <label style={S.label}>SX9 Agents (CLSGS Annex A.2)</label>
          <div style={{ ...S.grid, gridTemplateColumns: "repeat(2, 1fr)" }}>
            {[
              "FORGE", "AXIOM", "VECTOR", "SENTINEL", "GUARDIAN",
              "ORACLE", "SCRIBE", "RELAY", "ARBITER", "WEAVER"
            ].map((p) => (
              <button
                key={p}
                style={{
                  ...S.gridBtn,
                  borderColor: persona === p ? C.cyan : C.border,
                  backgroundColor:
                    persona === p ? C.cyan + "10" : "transparent",
                }}
                onClick={() => setPersona(p)}
              >
                {p}
              </button>
            ))}
          </div>
          <div style={{ marginTop: 12, fontSize: 11, color: C.text3, lineHeight: 1.4 }}>
            {persona === "FORGE" && "Code Generation • Filesystem, CI/CD, MCP tools"}
            {persona === "AXIOM" && "Analysis • Mathematical reasoning, Figma design"}
            {persona === "VECTOR" && "Architecture • Read-only audits, dependency graphs"}
            {persona === "SENTINEL" && "Security • MITRE ATT&CK, vulnerability scanning"}
            {persona === "GUARDIAN" && "QA • Test coverage, verification gates"}
            {persona === "ORACLE" && "Research • Web search, knowledge synthesis"}
            {persona === "SCRIBE" && "Documentation • RFC drafting, changelog"}
            {persona === "RELAY" && "Integration • Slack, Linear, external APIs"}
            {persona === "ARBITER" && "Governance • Drift detection, gate enforcement"}
            {persona === "WEAVER" && "Orchestration • Multi-agent coordination"}
          </div>
        </div>
      )}

      {/* AI PROVIDER SELECTION */}
      {tab === "agents" && (
        <div style={{ padding: 12 }}>
          <label style={S.label}>AI Provider</label>
          <select
            value={model}
            onChange={(e) => setModel(e.target.value)}
            style={{ ...S.headerSelect, width: "100%", marginBottom: 12 }}
          >
            <option>Claude Opus 4.5</option>
            <option>Claude Sonnet 4</option>
            <option>GPT-4o</option>
            <option>Gemini 2.5 Pro</option>
            <option>Grok-3</option>
            <option>O3-mini</option>
          </select>

          <label style={S.label}>Temperature: {temperature.toFixed(1)}</label>
          <input
            type="range"
            min="0"
            max="1"
            step="0.1"
            value={temperature}
            onChange={(e) => setTemperature(parseFloat(e.target.value))}
            style={{ width: "100%", marginTop: 8 }}
          />

          <div style={{ marginTop: 16, fontSize: 11, color: C.text3 }}>
            Provider routes through SX9 harness for governance tracking.
          </div>
        </div>
      )}

      {/* PHASE 2: LINEAR INTEGRATION */}
      {tab === "linear" && (
        <div style={{ padding: 12 }}>
          <label style={S.label}>Linear Team</label>
          <input
            type="text"
            value={linearTeam}
            onChange={(e) => setLinearTeam(e.target.value)}
            placeholder="SX9"
            style={{ ...S.headerInput, width: "100%", marginBottom: 12 }}
          />

          <label
            style={{
              ...S.label,
              display: "flex",
              alignItems: "center",
              gap: 8,
            }}
          >
            <input
              type="checkbox"
              checked={createLinearIssue}
              onChange={(e) => setCreateLinearIssue(e.target.checked)}
              style={{ width: 16, height: 16 }}
            />
            Auto-create Linear Issue
          </label>
        </div>
      )}

      {/* PHASE 2: SLACK INTEGRATION */}
      {tab === "slack" && (
        <div style={{ padding: 12 }}>
          <label style={S.label}>Slack Channel</label>
          <select
            value={slackChannel}
            onChange={(e) => setSlackChannel(e.target.value)}
            style={{ ...S.headerSelect, width: "100%", marginBottom: 12 }}
          >
            <option>#all-synaptixops</option>
            <option>#build-notifications</option>
            <option>#dev-alerts</option>
          </select>

          <label
            style={{
              ...S.label,
              display: "flex",
              alignItems: "center",
              gap: 8,
            }}
          >
            <input
              type="checkbox"
              checked={enableSlack}
              onChange={(e) => setEnableSlack(e.target.checked)}
              style={{ width: 16, height: 16 }}
            />
            Enable Slack Notifications
          </label>
        </div>
      )}

      {/* PHASE 2: CONTEXT SOURCES */}
      {tab === "context" && (
        <div style={{ padding: 12 }}>
          <label style={S.label}>Context Sources</label>
          {Object.entries(contextSources).map(([source, enabled]) => (
            <label
              key={source}
              style={{
                ...S.label,
                display: "flex",
                alignItems: "center",
                gap: 8,
                marginTop: 8,
                textTransform: "capitalize",
              }}
            >
              <input
                type="checkbox"
                checked={enabled}
                onChange={(e) =>
                  setContextSources({
                    ...contextSources,
                    [source]: e.target.checked,
                  })
                }
                style={{ width: 16, height: 16 }}
              />
              {source}
            </label>
          ))}
        </div>
      )}

      {/* MISSION TAB */}
      {tab === "mission" && (
        <div style={{ padding: 12 }}>
          <label style={S.label}>Mission Parameters</label>
          <p style={{ fontSize: 12, color: C.text3, marginBottom: 12 }}>
            Define constraints and success criteria.
          </p>
          <div style={S.options}>
            <button style={S.optBtn} onClick={() => onMission("scope")}>Set Scope Boundaries</button>
            <button style={S.optBtn} onClick={() => onMission("exit")}>Define Exit Criteria</button>
            <button style={S.optBtn} onClick={() => onMission("risk")}>Risk Thresholds</button>
          </div>
        </div>
      )}

      {/* DEPLOY TAB */}
      {tab === "deploy" && (
        <div style={{ padding: 12 }}>
          <label style={S.label}>Deployment Options</label>
          <p style={{ fontSize: 12, color: C.text3, marginBottom: 12 }}>
            Configure output and execution.
          </p>
          <div style={S.options}>
            <button style={{...S.optBtn, borderColor: C.cyan, backgroundColor: C.cyan + "10"}} onClick={() => onDeploy("execute")}>Execute Immediately</button>
            <button style={S.optBtn} onClick={() => onDeploy("schedule")}>Schedule for Later</button>
            <button style={S.optBtn} onClick={() => onDeploy("draft")}>Save as Draft</button>
          </div>
        </div>
      )}

      {/* SETTINGS TAB */}
      {tab === "settings" && (
        <div style={{ padding: 12 }}>
          <label style={S.label}>Forge Settings</label>
          <div style={S.options}>
            <button style={S.optBtn} onClick={() => onSettings("api")}>API Configuration</button>
            <button style={S.optBtn} onClick={() => onSettings("templates")}>Default Templates</button>
            <button style={S.optBtn} onClick={() => onSettings("shortcuts")}>Keyboard Shortcuts</button>
            <button style={S.optBtn} onClick={() => onSettings("theme")}>Theme & Display</button>
          </div>
        </div>
      )}

      {/* DATA TAB */}
      {tab === "data" && (
        <div style={{ padding: 12 }}>
          <label style={S.label}>Data Management</label>
          <div style={S.options}>
            <button style={S.optBtn} onClick={() => onData("vector")}>Vector Store Status</button>
            <button style={S.optBtn} onClick={() => onData("cache")}>Clear Cache</button>
            <button style={S.optBtn} onClick={() => onData("export")}>Export Prompts</button>
            <button style={S.optBtn} onClick={() => onData("import")}>Import History</button>
          </div>
        </div>
      )}
    </div>
  </div>
);

// Right rail content (expanded)
const RightContent: React.FC<{
  tab: string;
  setTab: (t: string) => void;
  onToolToggle: (tool: string) => void;
}> = ({
  tab,
  setTab,
  onToolToggle,
}) => (
  <div style={S.content}>
    <div style={S.tabs}>
      {["intel", "tools", "threats", "qa"].map((t) => (
        <button
          key={t}
          style={{
            ...S.tab,
            borderBottom: tab === t ? `2px solid ${C.cyan}` : "none",
          }}
          onClick={() => setTab(t)}
        >
          {t.toUpperCase()}
        </button>
      ))}
    </div>
    <div style={S.tabContent}>
      {/* INTEL TAB */}
      {tab === "intel" && (
        <div style={{ padding: 4 }}>
          <label style={S.label}>Pattern Intelligence</label>
          <div style={S.intelList}>
            <div style={S.intelItem}>
              <span style={S.intelDot}>●</span>
              <span>Similar prompts in memory (3)</span>
            </div>
            <div style={S.intelItem}>
              <span style={S.intelDot}>●</span>
              <span>Related RFC patterns (RFC-9141)</span>
            </div>
            <div style={S.intelItem}>
              <span style={S.intelDot}>●</span>
              <span>Behavioral scope matches (5)</span>
            </div>
          </div>
          <div style={{ marginTop: 16 }}>
            <label style={S.label}>Leptose Status</label>
            <div style={{ fontSize: 11, color: C.text3 }}>
              Vector similarity: Ready
            </div>
          </div>
        </div>
      )}

      {/* TOOLS TAB */}
      {tab === "tools" && (
        <div style={{ padding: 4 }}>
          <label style={S.label}>Recommended Tools</label>
          <div style={S.options}>
            <button style={S.toolBtn} onClick={() => onToolToggle("filesystem")}>
              <span style={{ fontWeight: 600 }}>MCP:Filesystem</span>
              <span style={{ fontSize: 10, color: C.text3 }}>Read/Write access</span>
            </button>
            <button style={S.toolBtn} onClick={() => onToolToggle("linear")}>
              <span style={{ fontWeight: 600 }}>MCP:Linear</span>
              <span style={{ fontSize: 10, color: C.text3 }}>Issue tracking</span>
            </button>
            <button style={S.toolBtn} onClick={() => onToolToggle("slack")}>
              <span style={{ fontWeight: 600 }}>MCP:Slack</span>
              <span style={{ fontSize: 10, color: C.text3 }}>Notifications</span>
            </button>
            <button style={S.toolBtn} onClick={() => onToolToggle("git")}>
              <span style={{ fontWeight: 600 }}>Git</span>
              <span style={{ fontSize: 10, color: C.text3 }}>Version control</span>
            </button>
          </div>
        </div>
      )}

      {/* THREATS TAB */}
      {tab === "threats" && (
        <div style={{ padding: 4 }}>
          <label style={S.label}>MITRE ATT&CK Analysis</label>
          <div style={S.threatList}>
            <div style={{ ...S.threatItem, borderColor: C.green }}>
              <span style={{ color: C.green }}>LOW</span>
              <span>Prompt injection risk</span>
            </div>
            <div style={{ ...S.threatItem, borderColor: C.text3 }}>
              <span style={{ color: C.text3 }}>N/A</span>
              <span>Data exfiltration</span>
            </div>
            <div style={{ ...S.threatItem, borderColor: C.text3 }}>
              <span style={{ color: C.text3 }}>N/A</span>
              <span>Privilege escalation</span>
            </div>
          </div>
          <div style={{ marginTop: 12, fontSize: 10, color: C.text3 }}>
            Last scan: {new Date().toLocaleTimeString()}
          </div>
        </div>
      )}

      {/* QA TAB - Governance Status (CLSGS) */}
      {tab === "qa" && (
        <div style={{ padding: 4 }}>
          <label style={S.label}>Governance Status (RFC-9141)</label>
          <div style={S.qaGrid}>
            <div style={S.qaItem}>
              <span style={{ ...S.qaDot, backgroundColor: C.green }}>✓</span>
              <span>Static QA</span>
            </div>
            <div style={S.qaItem}>
              <span style={{ ...S.qaDot, backgroundColor: C.cyan }}>○</span>
              <span>Semantic QA</span>
            </div>
            <div style={S.qaItem}>
              <span style={{ ...S.qaDot, backgroundColor: C.text3 }}>—</span>
              <span>Lineage Check</span>
            </div>
            <div style={S.qaItem}>
              <span style={{ ...S.qaDot, backgroundColor: C.text3 }}>—</span>
              <span>Drift Score</span>
            </div>
          </div>
          <div style={{ marginTop: 16 }}>
            <label style={S.label}>Gate Status</label>
            <div style={{ fontSize: 12, color: C.green }}>
              OBSERVE • No blocking gates
            </div>
          </div>
        </div>
      )}
    </div>
  </div>
);

// ============================================================================
// STYLES
// ============================================================================

const S: Record<string, React.CSSProperties> = {
  container: {
    display: "flex",
    flexDirection: "column",
    height: "100vh",
    background: C.bg,
    color: C.text,
    fontFamily: "Inter, sans-serif",
  },

  // Header
  header: {
    height: HEADER_H,
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    padding: "0 16px",
    background: C.bg2,
    borderBottom: `1px solid ${C.border}`,
  },
  logo: { fontSize: 15, fontWeight: 600 },
  headerCenter: { display: "flex", gap: 12 },
  headerInput: {
    width: 180,
    padding: "6px 10px",
    background: C.bg,
    border: `1px solid ${C.border}`,
    borderRadius: 4,
    color: C.text,
    fontSize: 13,
  },
  headerSelect: {
    padding: "6px 10px",
    background: C.bg,
    border: `1px solid ${C.border}`,
    borderRadius: 4,
    color: C.text,
    fontSize: 13,
  },
  headerRight: { display: "flex", gap: 8 },
  btn: {
    width: 36,
    height: 36,
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    border: `1px solid ${C.border}`,
    borderRadius: 6,
    cursor: "pointer",
  },

  // Main
  main: { flex: 1, display: "flex", overflow: "hidden" },

  // Rails
  rail: {
    display: "flex",
    flexDirection: "column",
    background: C.bg2,
    borderRight: `1px solid ${C.border}`,
    transition: "width 0.15s",
  },
  railHeader: {
    height: 40,
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    padding: "0 8px",
    borderBottom: `1px solid ${C.border}`,
  },
  railTitle: {
    fontSize: 10,
    fontWeight: 600,
    letterSpacing: 1,
    color: C.text3,
  },
  railBody: { flex: 1, overflow: "auto" },
  iconBtn: {
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    background: "transparent",
    border: "none",
    cursor: "pointer",
    borderRadius: 4,
  },
  iconCol: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    padding: "8px 0",
    gap: 4,
    height: "100%",
  },
  glyphBtn: {
    width: GLYPH,
    height: GLYPH,
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    border: "none",
    borderRadius: 6,
    cursor: "pointer",
  },
  sep: { height: 1, width: 32, background: C.border, margin: "4px 0" },

  // Center
  center: {
    flex: 1,
    display: "flex",
    flexDirection: "column",
    background: C.editor,
  },
  editor: { flex: 1, display: "flex", overflow: "auto" },
  lineNums: {
    width: 40,
    padding: "12px 0",
    background: C.bg2,
    borderRight: `1px solid ${C.border}`,
    textAlign: "right",
  },
  lineNum: {
    height: 20,
    paddingRight: 8,
    fontSize: 12,
    fontFamily: "monospace",
    color: C.text3,
  },
  code: {
    flex: 1,
    margin: 0,
    padding: 12,
    fontSize: 13,
    fontFamily: "monospace",
    color: C.text,
    lineHeight: "20px",
    whiteSpace: "pre",
  },
  mission: {
    padding: 12,
    borderTop: `1px solid ${C.border}`,
    background: C.bg2,
  },
  label: {
    display: "block",
    marginBottom: 8,
    fontSize: 10,
    fontWeight: 600,
    letterSpacing: 1,
    color: C.text3,
  },
  textarea: {
    width: "100%",
    padding: 10,
    background: C.bg,
    border: `1px solid ${C.border}`,
    borderRadius: 4,
    color: C.text,
    fontSize: 13,
    fontFamily: "monospace",
    resize: "none",
  },

  // Status
  status: {
    height: STATUS_H,
    display: "flex",
    alignItems: "center",
    padding: "0 16px",
    background: C.bg2,
    borderTop: `1px solid ${C.border}`,
    gap: 8,
  },
  statusText: { fontSize: 11, color: C.text2 },
  divider: { width: 1, height: 16, background: C.border },
  shortCode: { fontSize: 12, fontFamily: "monospace", color: C.cyan },

  // Content panels
  content: { display: "flex", flexDirection: "column", height: "100%" },
  tabs: { display: "flex", borderBottom: `1px solid ${C.border}` },
  tab: {
    flex: 1,
    height: 36,
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    background: "transparent",
    border: "none",
    color: C.text2,
    fontSize: 10,
    fontWeight: 600,
    cursor: "pointer",
  },
  tabContent: { flex: 1, padding: 12, overflow: "auto" },
  hint: { fontSize: 12, color: C.text3, fontStyle: "italic" },
  options: { display: "flex", flexDirection: "column", gap: 6 },
  optBtn: {
    padding: "10px 12px",
    background: "transparent",
    border: `1px solid ${C.border}`,
    borderRadius: 4,
    color: C.text,
    fontSize: 12,
    textAlign: "left",
    cursor: "pointer",
  },
  grid: { display: "grid", gridTemplateColumns: "repeat(3, 1fr)", gap: 4 },
  gridBtn: {
    padding: 8,
    background: "transparent",
    border: `1px solid ${C.border}`,
    borderRadius: 4,
    color: C.text,
    fontSize: 10,
    fontWeight: 600,
    cursor: "pointer",
  },

  // Right rail styles
  intelList: {
    display: "flex",
    flexDirection: "column",
    gap: 8,
    marginTop: 8,
  },
  intelItem: {
    display: "flex",
    alignItems: "center",
    gap: 8,
    fontSize: 11,
    color: C.text2,
  },
  intelDot: {
    color: C.cyan,
    fontSize: 8,
  },
  toolBtn: {
    display: "flex",
    flexDirection: "column",
    alignItems: "flex-start",
    gap: 2,
    padding: "10px 12px",
    background: "transparent",
    border: `1px solid ${C.border}`,
    borderRadius: 4,
    color: C.text,
    fontSize: 12,
    textAlign: "left",
    cursor: "pointer",
  },
  threatList: {
    display: "flex",
    flexDirection: "column",
    gap: 6,
    marginTop: 8,
  },
  threatItem: {
    display: "flex",
    alignItems: "center",
    gap: 12,
    padding: "8px 10px",
    border: "1px solid",
    borderRadius: 4,
    fontSize: 11,
    color: C.text2,
  },
  qaGrid: {
    display: "grid",
    gridTemplateColumns: "repeat(2, 1fr)",
    gap: 8,
    marginTop: 8,
  },
  qaItem: {
    display: "flex",
    alignItems: "center",
    gap: 8,
    padding: "8px 10px",
    background: C.bg,
    borderRadius: 4,
    fontSize: 11,
    color: C.text2,
  },
  qaDot: {
    width: 18,
    height: 18,
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    borderRadius: "50%",
    fontSize: 10,
    color: C.bg,
    fontWeight: 600,
  },
};

export default PromptForgeScreen;
