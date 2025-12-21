import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

// ═══════════════════════════════════════════════════════════════════════════════
// HARNESS CONFIGURATIONS
// ═══════════════════════════════════════════════════════════════════════════════

const HARNESSES = {
  full_autonomous: {
    name: "Full Autonomous",
    icon: "🚀",
    description: "All tools, maximum flexibility",
    mode: "AUTONOMOUS",
    temp: "0.3",
    timeout: "120m",
    tools: {
      web_search: true,
      web_fetch: true,
      computer_use: true,
      memory: true,
      linear: true,
      canva: true,
      figma: true,
      google_drive: true,
      filesystem: true,
      vercel: false,
      huggingface: false,
    },
  },
  research: {
    name: "Research & Intel",
    icon: "🔬",
    description: "Information gathering focus",
    mode: "AUTONOMOUS",
    temp: "0.4",
    timeout: "60m",
    tools: {
      web_search: true,
      web_fetch: true,
      computer_use: true,
      memory: true,
      linear: false,
      canva: false,
      figma: false,
      google_drive: true,
      filesystem: true,
      vercel: false,
      huggingface: true,
    },
  },
  build: {
    name: "Build & Implement",
    icon: "🔧",
    description: "Code generation focus",
    mode: "SUPERVISED",
    temp: "0.2",
    timeout: "90m",
    tools: {
      web_search: true,
      web_fetch: true,
      computer_use: true,
      memory: true,
      linear: true,
      canva: false,
      figma: true,
      google_drive: true,
      filesystem: true,
      vercel: true,
      huggingface: false,
    },
  },
  security: {
    name: "Security Audit",
    icon: "🛡️",
    description: "Read-only analysis focus",
    mode: "STEP-CONFIRM",
    temp: "0.1",
    timeout: "120m",
    tools: {
      web_search: true,
      web_fetch: true,
      computer_use: true,
      memory: true,
      linear: true,
      canva: true,
      figma: false,
      google_drive: true,
      filesystem: true,
      vercel: false,
      huggingface: false,
    },
  },
  planning: {
    name: "Planning & Docs",
    icon: "📋",
    description: "Strategy and documentation",
    mode: "SUPERVISED",
    temp: "0.3",
    timeout: "60m",
    tools: {
      web_search: true,
      web_fetch: true,
      computer_use: true,
      memory: true,
      linear: true,
      canva: true,
      figma: false,
      google_drive: true,
      filesystem: true,
      vercel: false,
      huggingface: false,
    },
  },
};

const PERSONAS = {
  FORGE: {
    name: "FORGE - Build Engineer",
    harness: "build",
    role: "You are FORGE, a senior build/infrastructure engineer for SX9.",
  },
  AXIOM: {
    name: "AXIOM - Systems Architect",
    harness: "build",
    role: "You are AXIOM, a systems architect implementing clean, modular code.",
  },
  VECTOR: {
    name: "VECTOR - Security Engineer",
    harness: "security",
    role: "You are VECTOR, a security engineer conducting thorough audits.",
  },
  SENTINEL: {
    name: "SENTINEL - Threat Analyst",
    harness: "research",
    role: "You are SENTINEL, a threat analyst and red team operator.",
  },
  NEXUS: {
    name: "NEXUS - Integration Specialist",
    harness: "build",
    role: "You are NEXUS, an integration specialist handling data migrations.",
  },
  CIPHER: {
    name: "CIPHER - Crypto/Privacy",
    harness: "security",
    role: "You are CIPHER, a cryptography and privacy engineer.",
  },
  SCRIBE: {
    name: "SCRIBE - Documentation",
    harness: "planning",
    role: "You are SCRIBE, a technical writer creating clear documentation.",
  },
};

const PROMPT_TYPES = {
  BUILD_PIPELINE: {
    name: "Build Pipeline",
    icon: "🔧",
    persona: "FORGE",
    phase: "IMPLEMENT",
    priority: "P1",
    hardConstraints: [
      "DO NOT modify source files in src/",
      "DO NOT delete existing files",
    ],
    softConstraints: ["Prefer Makefile/justfile over custom scripts"],
    linearLabels: ["type:BUILD"],
    contextSources: { memory: true, linear: true, drive: true, web: true },
  },
  SECURITY_AUDIT: {
    name: "Security Audit",
    icon: "🛡️",
    persona: "VECTOR",
    phase: "ANALYZE",
    priority: "P0",
    hardConstraints: [
      "READ-ONLY - DO NOT modify any files",
      "HALT on credential exposure",
    ],
    softConstraints: ["Flag findings with severity"],
    linearLabels: ["type:SECURITY"],
    contextSources: { memory: true, linear: true, drive: true, web: true },
  },
  THREAT_EMULATION: {
    name: "Threat Emulation",
    icon: "🎯",
    persona: "SENTINEL",
    phase: "IMPLEMENT",
    priority: "P1",
    hardConstraints: ["DO NOT execute against production", "Log ALL actions"],
    softConstraints: ["Map to MITRE ATT&CK"],
    linearLabels: ["type:THREAT-EMULATION"],
    contextSources: { memory: true, linear: true, drive: false, web: true },
  },
  CODE_GENERATION: {
    name: "Code Generation",
    icon: "💻",
    persona: "AXIOM",
    phase: "IMPLEMENT",
    priority: "P2",
    hardConstraints: [
      "DO NOT create files > 300 lines",
      "All new code must include tests",
    ],
    softConstraints: ["Follow existing conventions"],
    linearLabels: ["type:CODE"],
    contextSources: { memory: true, linear: true, drive: true, web: true },
  },
  RFC_ALIGNMENT: {
    name: "RFC Alignment",
    icon: "📋",
    persona: "FORGE",
    phase: "WALK",
    priority: "P1",
    hardConstraints: ["DO NOT renumber existing RFCs"],
    softConstraints: ["Use shared LaTeX system"],
    linearLabels: ["type:RFC"],
    contextSources: { memory: true, linear: true, drive: true, web: false },
  },
  RESEARCH: {
    name: "Research & Analysis",
    icon: "🔬",
    persona: "SENTINEL",
    phase: "RESEARCH",
    priority: "P2",
    hardConstraints: ["Cite ALL sources"],
    softConstraints: ["Prefer primary sources"],
    linearLabels: ["type:RESEARCH"],
    contextSources: { memory: true, linear: false, drive: true, web: true },
  },
  DOCUMENTATION: {
    name: "Documentation",
    icon: "📝",
    persona: "SCRIBE",
    phase: "WALK",
    priority: "P3",
    hardConstraints: ["DO NOT modify source code"],
    softConstraints: ["Include examples for all APIs"],
    linearLabels: ["type:DOCS"],
    contextSources: { memory: true, linear: true, drive: true, web: false },
  },
  CUSTOM: {
    name: "Custom",
    icon: "⚙️",
    persona: "",
    phase: "PLAN",
    priority: "P2",
    hardConstraints: [],
    softConstraints: [],
    linearLabels: [],
    contextSources: { memory: true, linear: false, drive: false, web: false },
  },
};

type HarnessKey = keyof typeof HARNESSES;
type PersonaKey = keyof typeof PERSONAS | "";
type PromptTypeKey = keyof typeof PROMPT_TYPES;

interface FormState {
  [key: string]: string | boolean;
}

export default function PromptForge() {
  const [promptType, setPromptType] = useState<PromptTypeKey>("CUSTOM");
  const [harness, setHarness] = useState<HarnessKey>("build");
  const [persona, setPersona] = useState<PersonaKey>("");

  const [form, setForm] = useState<FormState>({
    title: "",
    rfc: "",
    phase: "PLAN",
    classification: "INTERNAL",
    priority: "P2",
    mode: "SUPERVISED",
    temp: "0.2",
    timeout: "30m",
    onFail: "HALT",
    web_search: true,
    web_fetch: true,
    computer_use: true,
    memory: true,
    linear: true,
    canva: false,
    figma: false,
    google_drive: true,
    filesystem: true,
    vercel: false,
    huggingface: false,
    ctx_memory: true,
    ctx_linear: true,
    ctx_drive: true,
    ctx_web: true,
    linearTeam: "SX9",
    linearProject: "",
    createLinearIssue: true,
    linearLabels: "",
    workdir: "",
    forbidPaths: ".env, secrets/, .git/config",
    objective: "",
    context: "",
    hardConstraints: "",
    softConstraints: "",
    deliverables: "",
    acceptance: "",
    role: "",
    task: "",
  });

  const [output, setOutput] = useState("");
  const [copied, setCopied] = useState(false);
  const [activeTab, setActiveTab] = useState<
    "config" | "tools" | "linear" | "mission"
  >("config");
  const [creating, setCreating] = useState(false);
  const [notification, setNotification] = useState<string | null>(null);

  useEffect(() => {
    const type = PROMPT_TYPES[promptType];
    if (type) {
      const pKey = type.persona as PersonaKey;
      const p = pKey ? PERSONAS[pKey as keyof typeof PERSONAS] : null;
      const h = p ? HARNESSES[p.harness as HarnessKey] : HARNESSES.build;
      setPersona(pKey || "");
      if (p) setHarness(p.harness as HarnessKey);
      setForm((f) => ({
        ...f,
        phase: type.phase,
        priority: type.priority,
        mode: h?.mode || f.mode,
        temp: h?.temp || f.temp,
        timeout: h?.timeout || f.timeout,
        ...(h?.tools || {}),
        ctx_memory: type.contextSources?.memory ?? true,
        ctx_linear: type.contextSources?.linear ?? false,
        ctx_drive: type.contextSources?.drive ?? false,
        ctx_web: type.contextSources?.web ?? false,
        hardConstraints: (type.hardConstraints || []).join("\n"),
        softConstraints: (type.softConstraints || []).join("\n"),
        linearLabels: (type.linearLabels || []).join(", "),
        role: p?.role || "",
      }));
    }
  }, [promptType]);

  useEffect(() => {
    const h = HARNESSES[harness];
    if (h)
      setForm((f) => ({
        ...f,
        mode: h.mode,
        temp: h.temp,
        timeout: h.timeout,
        ...h.tools,
      }));
  }, [harness]);

  useEffect(() => {
    if (persona && persona in PERSONAS) {
      const p = PERSONAS[persona as keyof typeof PERSONAS];
      setHarness(p.harness as HarnessKey);
      setForm((f) => ({ ...f, role: p.role }));
    }
  }, [persona]);

  const set = (k: string, v: string | boolean) =>
    setForm((f) => ({ ...f, [k]: v }));
  const genId = () => {
    const d = new Date();
    return `PRM-${d.toISOString().slice(0, 10).replace(/-/g, "")}-${d.toTimeString().slice(0, 5).replace(":", "")}`;
  };

  const generate = useCallback(() => {
    const id = genId();
    const date = new Date().toISOString().split("T")[0];
    const enabledTools = [
      "web_search",
      "web_fetch",
      "computer_use",
      "memory",
      "linear",
      "canva",
      "figma",
      "google_drive",
      "filesystem",
      "vercel",
      "huggingface",
    ].filter((t) => form[t]);
    const contextSources = [
      form.ctx_memory && "memory",
      form.ctx_linear && "linear",
      form.ctx_drive && "google_drive",
      form.ctx_web && "web_search",
    ].filter(Boolean);

    const yaml = `# SX9-PROMPT v2.0
# Type: ${PROMPT_TYPES[promptType]?.name || "Custom"}
# Generated: ${date}

header:
  id: ${id}
  rfc: ${form.rfc || "N/A"}
  title: "${form.title}"
  date: ${date}
  phase: ${form.phase}
  priority: ${form.priority}

harness:
  base: ${harness}
  persona: ${persona || "DEFAULT"}
  mode: ${form.mode}
  temperature: ${form.temp}
  timeout: ${form.timeout}
  on_fail: ${form.onFail}
  tools: [${enabledTools.join(", ")}]
  context_loading: [${contextSources.join(", ")}]

linear:
  enabled: ${form.createLinearIssue}
  team: "${form.linearTeam}"
  labels: [${form.linearLabels}]

objective: |
  ${form.objective || "[DEFINE OBJECTIVE]"}

constraints:
  hard:
${
  (form.hardConstraints as string)
    .split("\n")
    .filter(Boolean)
    .map((c) => `    - "${c.trim()}"`)
    .join("\n") || '    - "DO NOT modify files outside working directory"'
}
  soft:
${
  (form.softConstraints as string)
    .split("\n")
    .filter(Boolean)
    .map((c) => `    - "${c.trim()}"`)
    .join("\n") || '    - "Document all decisions"'
}

deliverables:
${
  (form.deliverables as string)
    .split("\n")
    .filter(Boolean)
    .map((d, i) => `  - D${i + 1}: "${d.trim()}"`)
    .join("\n") || '  - D1: "[DEFINE DELIVERABLE]"'
}

role: |
  ${form.role || "You are a senior engineer for SX9."}

task: |
  ${form.task || "[DEFINE TASK STEPS]"}
`;
    setOutput(yaml);
    setCopied(false);
  }, [form, promptType, harness, persona]);

  const copy = async () => {
    await navigator.clipboard.writeText(output);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const createMission = useCallback(async () => {
    if (!form.title || !persona) {
      setNotification("⚠️ Title and Persona required");
      setTimeout(() => setNotification(null), 3000);
      return;
    }

    setCreating(true);
    try {
      await invoke("create_mission", {
        input: {
          title: form.title as string,
          prompt_type: promptType,
          persona,
          harness,
          objective: (form.objective as string) || "",
          hard_constraints: ((form.hardConstraints as string) || "")
            .split("\n")
            .filter((s) => s.trim()),
          soft_constraints: ((form.softConstraints as string) || "")
            .split("\n")
            .filter((s) => s.trim()),
          deliverables: ((form.deliverables as string) || "")
            .split("\n")
            .filter((s) => s.trim()),
          rfcs: ((form.rfc as string) || "").split(",").filter((s) => s.trim()),
          priority: form.priority as string,
          phase: form.phase as string,
        },
      });

      setNotification("✅ Mission created & saved to clipboard!");
      setTimeout(() => setNotification(null), 3000);

      // Generate YAML output
      generate();
    } catch (error) {
      setNotification(`❌ Error: ${error}`);
      setTimeout(() => setNotification(null), 5000);
    } finally {
      setCreating(false);
    }
  }, [form, promptType, persona, harness, generate]);

  const inputClass =
    "w-full bg-zinc-800 border border-zinc-700 rounded px-2 py-1.5 text-sm text-zinc-100 focus:border-emerald-500 focus:outline-none";
  const labelClass = "block text-xs text-zinc-500 mb-1";
  const checkClass =
    "flex items-center gap-2 text-xs text-zinc-300 cursor-pointer hover:text-zinc-100";

  return (
    <div className="p-4">
      <div className="max-w-7xl mx-auto">
        <div className="flex items-center justify-between mb-4 pb-3 border-b border-zinc-700">
          <div>
            <h1 className="text-xl font-bold text-emerald-400">PROMPT FORGE</h1>
            <p className="text-xs text-zinc-500">
              Generate mission YAML for IDE bootstrap
            </p>
          </div>
          <div className="flex gap-2">
            <button
              onClick={createMission}
              disabled={creating || !form.title || !persona}
              className="px-4 py-2 bg-blue-600 hover:bg-blue-500 disabled:bg-zinc-700 disabled:cursor-not-allowed rounded font-medium text-sm transition-colors"
            >
              {creating ? "CREATING..." : "CREATE MISSION"}
            </button>
            <button
              onClick={generate}
              className="px-4 py-2 bg-emerald-600 hover:bg-emerald-500 rounded font-medium text-sm"
            >
              GENERATE
            </button>
          </div>
          {notification && (
            <div className="absolute top-2 right-2 px-4 py-2 bg-zinc-800 border border-zinc-600 rounded-lg text-sm animate-fade-in">
              {notification}
            </div>
          )}
        </div>

        <div className="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-8 gap-2 mb-4">
          {Object.entries(PROMPT_TYPES).map(([k, t]) => (
            <button
              key={k}
              onClick={() => setPromptType(k as PromptTypeKey)}
              className={`p-2 rounded border text-left ${promptType === k ? "border-emerald-500 bg-emerald-900/20" : "border-zinc-700 bg-zinc-800 hover:border-zinc-600"}`}
            >
              <div className="flex items-center gap-1">
                <span>{t.icon}</span>
                <span className="font-medium text-xs truncate">{t.name}</span>
              </div>
            </button>
          ))}
        </div>

        <div className="grid grid-cols-2 gap-4 mb-4">
          <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
            <label className={labelClass}>AGENT HARNESS</label>
            <div className="grid grid-cols-5 gap-1">
              {Object.entries(HARNESSES).map(([k, h]) => (
                <button
                  key={k}
                  onClick={() => setHarness(k as HarnessKey)}
                  className={`p-2 rounded border text-center ${harness === k ? "border-cyan-500 bg-cyan-900/20" : "border-zinc-700 bg-zinc-900"}`}
                >
                  <div className="text-lg">{h.icon}</div>
                  <div className="text-xs truncate">{h.name}</div>
                </button>
              ))}
            </div>
          </div>
          <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
            <label className={labelClass}>PERSONA</label>
            <div className="grid grid-cols-4 gap-1">
              {Object.entries(PERSONAS).map(([k]) => (
                <button
                  key={k}
                  onClick={() => setPersona(k as PersonaKey)}
                  className={`p-2 rounded border text-center ${persona === k ? "border-amber-500 bg-amber-900/20" : "border-zinc-700 bg-zinc-900"}`}
                >
                  <div className="text-xs font-medium truncate">{k}</div>
                </button>
              ))}
              <button
                onClick={() => setPersona("")}
                className={`p-2 rounded border ${!persona ? "border-amber-500 bg-amber-900/20" : "border-zinc-700 bg-zinc-900"}`}
              >
                <div className="text-xs">DEFAULT</div>
              </button>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-5 gap-4">
          <div className="lg:col-span-3 space-y-3">
            <div className="flex gap-1 border-b border-zinc-700 pb-2">
              {["config", "tools", "linear", "mission"].map((tab) => (
                <button
                  key={tab}
                  onClick={() => setActiveTab(tab)}
                  className={`px-3 py-1 rounded-t text-sm ${activeTab === tab ? "bg-zinc-700 text-white" : "text-zinc-500"}`}
                >
                  {tab.toUpperCase()}
                </button>
              ))}
            </div>

            {activeTab === "config" && (
              <div className="space-y-3">
                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700 grid grid-cols-2 gap-3">
                  <div className="col-span-2">
                    <label className={labelClass}>Title</label>
                    <input
                      className={inputClass}
                      value={form.title as string}
                      onChange={(e) => set("title", e.target.value)}
                      placeholder="Descriptive title"
                    />
                  </div>
                  <div>
                    <label className={labelClass}>RFC Reference</label>
                    <input
                      className={inputClass}
                      value={form.rfc as string}
                      onChange={(e) => set("rfc", e.target.value)}
                      placeholder="RFC-XXXX"
                    />
                  </div>
                  <div>
                    <label className={labelClass}>Phase</label>
                    <select
                      className={inputClass}
                      value={form.phase as string}
                      onChange={(e) => set("phase", e.target.value)}
                    >
                      {[
                        "PULL",
                        "ANALYZE",
                        "RESEARCH",
                        "PLAN",
                        "IMPLEMENT",
                        "WALK",
                        "COMMIT",
                      ].map((p) => (
                        <option key={p}>{p}</option>
                      ))}
                    </select>
                  </div>
                  <div>
                    <label className={labelClass}>Priority</label>
                    <select
                      className={inputClass}
                      value={form.priority as string}
                      onChange={(e) => set("priority", e.target.value)}
                    >
                      {["P0", "P1", "P2", "P3"].map((p) => (
                        <option key={p}>{p}</option>
                      ))}
                    </select>
                  </div>
                  <div>
                    <label className={labelClass}>Mode</label>
                    <select
                      className={inputClass}
                      value={form.mode as string}
                      onChange={(e) => set("mode", e.target.value)}
                    >
                      {["AUTONOMOUS", "SUPERVISED", "STEP-CONFIRM"].map((m) => (
                        <option key={m}>{m}</option>
                      ))}
                    </select>
                  </div>
                </div>
              </div>
            )}

            {activeTab === "tools" && (
              <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                <label className={labelClass}>ENABLED TOOLS</label>
                <div className="grid grid-cols-4 gap-2 mt-2">
                  {[
                    ["web_search", "Web Search", "🔍"],
                    ["web_fetch", "Web Fetch", "📥"],
                    ["computer_use", "Computer", "💻"],
                    ["memory", "Memory", "🧠"],
                    ["linear", "Linear", "📋"],
                    ["canva", "Canva", "🎨"],
                    ["figma", "Figma", "🖼️"],
                    ["google_drive", "Drive", "📁"],
                    ["filesystem", "Filesystem", "📂"],
                    ["vercel", "Vercel", "▲"],
                    ["huggingface", "HuggingFace", "🤗"],
                  ].map(([k, l, i]) => (
                    <label
                      key={k}
                      className={`${checkClass} p-2 rounded border ${form[k] ? "border-emerald-700 bg-emerald-900/20" : "border-zinc-700"}`}
                    >
                      <input
                        type="checkbox"
                        checked={form[k] as boolean}
                        onChange={(e) => set(k, e.target.checked)}
                        className="w-3 h-3"
                      />
                      <span>{i}</span>
                      <span className="truncate">{l}</span>
                    </label>
                  ))}
                </div>
              </div>
            )}

            {activeTab === "linear" && (
              <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                <label className={`${checkClass} mb-3`}>
                  <input
                    type="checkbox"
                    checked={form.createLinearIssue as boolean}
                    onChange={(e) => set("createLinearIssue", e.target.checked)}
                    className="w-4 h-4"
                  />
                  <span className="text-sm font-medium">
                    Create Linear Issue
                  </span>
                </label>
                <div className="grid grid-cols-2 gap-3">
                  <div>
                    <label className={labelClass}>Team</label>
                    <input
                      className={inputClass}
                      value={form.linearTeam as string}
                      onChange={(e) => set("linearTeam", e.target.value)}
                    />
                  </div>
                  <div>
                    <label className={labelClass}>Labels</label>
                    <input
                      className={inputClass}
                      value={form.linearLabels as string}
                      onChange={(e) => set("linearLabels", e.target.value)}
                    />
                  </div>
                </div>
              </div>
            )}

            {activeTab === "mission" && (
              <div className="space-y-3">
                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>Objective</label>
                  <textarea
                    className={inputClass}
                    rows={2}
                    value={form.objective as string}
                    onChange={(e) => set("objective", e.target.value)}
                  />
                </div>
                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>Hard Constraints</label>
                  <textarea
                    className={`${inputClass} font-mono`}
                    rows={3}
                    value={form.hardConstraints as string}
                    onChange={(e) => set("hardConstraints", e.target.value)}
                  />
                </div>
                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>Soft Constraints</label>
                  <textarea
                    className={`${inputClass} font-mono`}
                    rows={2}
                    value={form.softConstraints as string}
                    onChange={(e) => set("softConstraints", e.target.value)}
                  />
                </div>
                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>Deliverables</label>
                  <textarea
                    className={`${inputClass} font-mono`}
                    rows={2}
                    value={form.deliverables as string}
                    onChange={(e) => set("deliverables", e.target.value)}
                  />
                </div>
                <div className="bg-zinc-800/50 rounded p-3 border border-zinc-700">
                  <label className={labelClass}>Task Steps</label>
                  <textarea
                    className={`${inputClass} font-mono`}
                    rows={4}
                    value={form.task as string}
                    onChange={(e) => set("task", e.target.value)}
                  />
                </div>
              </div>
            )}
          </div>

          <div className="lg:col-span-2 lg:sticky lg:top-4 lg:self-start">
            <div className="bg-zinc-800 rounded border border-zinc-700 overflow-hidden">
              <div className="flex items-center justify-between px-3 py-2 bg-zinc-900 border-b border-zinc-700">
                <div className="flex items-center gap-2">
                  <span className="text-xs text-zinc-400">OUTPUT</span>
                  {persona && (
                    <span className="text-xs px-2 py-0.5 bg-amber-900/30 text-amber-400 rounded">
                      {persona}
                    </span>
                  )}
                </div>
                <button
                  onClick={copy}
                  disabled={!output}
                  className="px-2 py-1 text-xs bg-zinc-700 hover:bg-zinc-600 disabled:opacity-50 rounded"
                >
                  {copied ? "✓ COPIED" : "COPY"}
                </button>
              </div>
              <pre className="p-3 text-xs text-zinc-300 overflow-auto max-h-[70vh] font-mono">
                {output || "Select type → configure → GENERATE"}
              </pre>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
