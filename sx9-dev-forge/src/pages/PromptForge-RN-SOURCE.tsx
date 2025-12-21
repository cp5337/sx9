"use client";

import React, { useState, useCallback, useEffect } from "react";
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  TouchableOpacity,
  LayoutAnimation,
  Platform,
  UIManager,
  SafeAreaView,
} from "react-native";
import {
  ChevronLeft,
  ChevronRight,
  Activity,
  CheckCircle,
  Wifi,
  Flag,
  Zap,
  Target,
  Lock,
  Brain,
  Wrench,
  Clock,
  BookOpen,
  List,
} from "lucide-react";
import { Button } from "../components/Button";
import { Input } from "../components/Input";
import { Stack } from "../components/Stack";
import { Grid } from "../components/Grid";
import { Checkbox } from "../components/Checkbox";
import { Accordion } from "../components/Accordion";
import { colors, spacing, borderRadius } from "../tokens";
import { useResponsive } from "../hooks/useResponsive";
import {
  PROMPT_TYPES,
  HARNESSES,
  PERSONAS,
  TOOLS_LIST,
  CONTEXT_SOURCES,
  RECENT_MISSIONS,
  DEFAULT_FORM_STATE,
} from "../config/prompt-forge.config";

if (
  Platform.OS === "android" &&
  UIManager.setLayoutAnimationEnabledExperimental
) {
  UIManager.setLayoutAnimationEnabledExperimental(true);
}

export const PromptForgeScreen = () => {
  const { isDesktop } = useResponsive();

  // Drawer State
  const [leftOpen, setLeftOpen] = useState(true);
  const [rightOpen, setRightOpen] = useState(true);

  // Configuration State
  const [promptType, setPromptType] =
    useState<keyof typeof PROMPT_TYPES>("CUSTOM");
  const [harness, setHarness] = useState<keyof typeof HARNESSES>("build");
  const [persona, setPersona] = useState<keyof typeof PERSONAS>("FORGE");

  // Form State
  const [form, setForm] = useState<any>(DEFAULT_FORM_STATE);

  // @ts-ignore
  const set = (k: string, v: any) => setForm((f: any) => ({ ...f, [k]: v }));

  // Render Output
  const [output, setOutput] = useState("");
  const [copyFeedback, setCopyFeedback] = useState("");
  const [isSaving, setIsSaving] = useState(false);

  // PHASE 2: BACKEND ACTIONS
  const saveToDisk = async (content: string) => {
    try {
      // FIXED PATH: /api/fs
      const res = await fetch("/api/fs", {
        method: "POST",
        body: JSON.stringify({
          content,
          workdir: form.workdir,
          filename: `${form.title.replace(/\s+/g, "-").toLowerCase()}-${new Date().getTime()}.yaml`,
        }),
        headers: { "Content-Type": "application/json" },
      });
      const data = await res.json();
      if (data.success) {
        console.log("Saved to disk:", data.path);
        return true;
      }
    } catch (e) {
      console.error("Save failed", e);
    }
    return false;
  };

  const notifySlack = async (msg: string) => {
    if (!form.notifySlack) return;
    try {
      // FIXED PATH: /api/slack/notify
      await fetch("/api/slack/notify", {
        method: "POST",
        body: JSON.stringify({ message: msg, channel: form.slackChannel }),
        headers: { "Content-Type": "application/json" },
      });
    } catch (e) {
      console.error("Slack failed", e);
    }
  };

  const deployToLinear = async (content: string) => {
    if (!form.createLinearIssue) return;
    try {
      // FIXED PATH: /api/linear
      const res = await fetch("/api/linear", {
        method: "POST",
        body: JSON.stringify({
          title: form.title,
          description: "```yaml\n" + content + "\n```",
          teamId: form.linearTeam,
          projectId: form.linearProject,
          labels: form.linearLabels,
        }),
        headers: { "Content-Type": "application/json" },
      });
      const data = await res.json();
      if (data.success) {
        console.log("Linear Issue Created:", data.identifier);
        // NOTIFY SLACK
        if (form.notifySlack) {
          await notifySlack(
            `🚀 *Prompt Deployed*: ${form.title}\n🎫 Ticket: ${data.identifier}\n📂 Saved: Yes`
          );
        }
        return data.identifier;
      }
    } catch (e) {
      console.error("Linear failed", e);
    }
    return null;
  };

  const copyToClipboard = async () => {
    setIsSaving(true);
    if (Platform.OS === "web") {
      try {
        await navigator.clipboard.writeText(output);

        // Auto-Save to Disk on Copy (User Request: "Send to IDE")
        const saved = await saveToDisk(output);

        setCopyFeedback(saved ? "COPIED & SAVED!" : "COPIED (LOCAL ONLY)");
        setTimeout(() => setCopyFeedback(""), 2000);
      } catch (e) {
        console.error("Failed to copy", e);
        setCopyFeedback("ERROR");
      }
    }
    setIsSaving(false);
  };

  const generate = useCallback(() => {
    // @ts-ignore
    const enabledTools = TOOLS_LIST.filter((t) => form[t.id])
      .map((t) => t.id)
      .join(", ");
    // @ts-ignore
    const contextSrcs = CONTEXT_SOURCES.filter((t) => form[t.id])
      .map((t) => t.id)
      .join(", ");

    // Config Lookup
    const pType = PROMPT_TYPES[promptType];
    const hType = HARNESSES[harness];
    const pPersona = PERSONAS[persona];

    const yaml = `# SX9-PROMPT v3.0
# Generated: ${new Date().toISOString()}
# Type: ${pType.name}

header:
  title: "${form.title}"
  rfc: ${form.rfc}
  phase: ${form.phase}
  priority: ${form.priority}
  
utilization:
  harness: ${hType.name}
  persona: ${pPersona.role}
  mode: ${form.mode}
  temp: ${hType.temp}
  timeout: ${form.timeout}

tools: [${enabledTools}]
context_sources: [${contextSrcs}]

mission:
  objective: "${form.objective}"
  context: "${form.context}"

constraints:
  hard:
${form.hardConstraints
  .split("\n")
  .map((l: string) => `    - "${l}"`)
  .join("\n")}
  soft:
${form.softConstraints
  .split("\n")
  .map((l: string) => `    - "${l}"`)
  .join("\n")}

deliverables:
${form.deliverables
  .split("\n")
  .map((l: string) => `  - "${l}"`)
  .join("\n")}

acceptance:
${form.acceptance
  .split("\n")
  .map((l: string) => `  - "${l}"`)
  .join("\n")}
  
integration:
  linear:
    team: "${form.linearTeam}"
    project: "${form.linearProject}"
    labels: [${form.linearLabels}]
    create_issue: ${form.createLinearIssue}
  filesystem:
    workdir: "${form.workdir}"
    forbid: [${form.forbidPaths}]
`;
    setOutput(yaml);
  }, [form, harness, persona, promptType]);

  useEffect(() => {
    generate();
  }, [form, harness, persona, promptType, generate]);

  return (
    <SafeAreaView style={styles.safeArea}>
      <View style={styles.screen}>
        {/* LEFT DRAWER (DO-ERS) */}
        <View style={[styles.drawer, { width: leftOpen ? 320 : 50 }]}>
          <View style={styles.drawerHeader}>
            <TouchableOpacity
              onPress={() => setLeftOpen(!leftOpen)}
              style={styles.iconButton}
            >
              {leftOpen ? (
                <ChevronLeft size={16} color={colors.text.secondary} />
              ) : (
                <ChevronRight size={16} color={colors.text.secondary} />
              )}
            </TouchableOpacity>
            {leftOpen && (
              <Text style={styles.drawerTitle}>DO-ERS (ACTION)</Text>
            )}
          </View>

          <ScrollView style={styles.drawerScroll}>
            {leftOpen ? (
              <Stack spacing="md">
                {/* GROUP 1: IDENTITY */}
                <Accordion title="1. IDENTITY" expanded={true}>
                  <Grid columns={4} gap={4}>
                    {Object.entries(PROMPT_TYPES).map(([k, t]) => (
                      <TouchableOpacity
                        key={k}
                        onPress={() => setPromptType(k as any)}
                        style={[
                          styles.typeBtn,
                          promptType === k && styles.typeBtnActive,
                        ]}
                      >
                        <t.icon
                          size={14}
                          color={
                            promptType === k
                              ? colors.primary
                              : colors.text.tertiary
                          }
                        />
                      </TouchableOpacity>
                    ))}
                  </Grid>
                  <Stack spacing="xs" style={{ marginTop: 8 }}>
                    <Input
                      label="Title"
                      value={form.title}
                      onChangeText={(t) => set("title", t)}
                    />
                    <Grid columns={2} gap={spacing.xs}>
                      <Input
                        label="RFC"
                        value={form.rfc}
                        onChangeText={(t) => set("rfc", t)}
                      />
                      <Input
                        label="Phase"
                        value={form.phase}
                        onChangeText={(t) => set("phase", t)}
                      />
                    </Grid>
                  </Stack>
                </Accordion>

                {/* GROUP 2: STRATEGY */}
                <Accordion title="2. STRATEGY" expanded={true}>
                  <Stack spacing="xs">
                    {Object.entries(HARNESSES)
                      .slice(0, 3)
                      .map(([k, h]) => (
                        <TouchableOpacity
                          key={k}
                          onPress={() => setHarness(k as any)}
                          style={[
                            styles.rowItem,
                            harness === k && styles.rowItemActive,
                          ]}
                        >
                          <h.Icon
                            size={14}
                            color={
                              harness === k
                                ? colors.primary
                                : colors.text.secondary
                            }
                          />
                          <Text
                            style={[
                              styles.labelSmall,
                              harness === k && { color: colors.primary },
                            ]}
                          >
                            {h.name}
                          </Text>
                        </TouchableOpacity>
                      ))}
                  </Stack>
                  <Grid columns={3} gap={4} style={{ marginTop: 8 }}>
                    {Object.entries(PERSONAS).map(([k, p]) => (
                      <TouchableOpacity
                        key={k}
                        onPress={() => setPersona(k as any)}
                        style={[
                          styles.miniButton,
                          persona === k && {
                            borderColor: p.color,
                            backgroundColor: p.color + "10",
                          },
                        ]}
                      >
                        <Text
                          style={{
                            fontSize: 9,
                            fontWeight: "bold",
                            color:
                              persona === k ? p.color : colors.text.secondary,
                          }}
                        >
                          {p.name}
                        </Text>
                      </TouchableOpacity>
                    ))}
                  </Grid>
                </Accordion>

                {/* GROUP 3: OUTPUT */}
                <Accordion title="3. OUTPUT SPECS">
                  <Input
                    label="Deliverables (Line separated)"
                    multiline
                    numberOfLines={3}
                    value={form.deliverables}
                    onChangeText={(t) => set("deliverables", t)}
                    style={{ height: 60 }}
                  />
                  <Input
                    label="Acceptance Criteria"
                    multiline
                    numberOfLines={3}
                    value={form.acceptance}
                    onChangeText={(t) => set("acceptance", t)}
                    style={{ height: 60 }}
                  />
                  <Checkbox
                    label="Create Linear Issue"
                    checked={form.createLinearIssue}
                    onChange={(c) => set("createLinearIssue", c)}
                  />
                  {form.createLinearIssue && (
                    <View
                      style={{
                        marginLeft: 24,
                        marginTop: 4,
                        borderLeftWidth: 1,
                        borderLeftColor: colors.border.secondary,
                        paddingLeft: 8,
                      }}
                    >
                      <Checkbox
                        label="Notify Slack"
                        checked={form.notifySlack}
                        onChange={(c) => set("notifySlack", c)}
                      />
                      {form.notifySlack && (
                        <Input
                          label="Channel"
                          value={form.slackChannel}
                          onChangeText={(t) => set("slackChannel", t)}
                          style={{ marginTop: 4, fontSize: 10 }}
                          placeholder="#builds"
                        />
                      )}
                    </View>
                  )}
                </Accordion>

                {/* GROUP 4: RULES */}
                <Accordion title="4. RULES & CONSTRAINTS">
                  <Input
                    label="Hard Constraints"
                    multiline
                    numberOfLines={2}
                    value={form.hardConstraints}
                    onChangeText={(t) => set("hardConstraints", t)}
                    style={{ height: 50 }}
                  />
                  <Input
                    label="Soft Constraints"
                    multiline
                    numberOfLines={2}
                    value={form.softConstraints}
                    onChangeText={(t) => set("softConstraints", t)}
                    style={{ height: 50 }}
                  />
                  <Input
                    label="Workdir"
                    value={form.workdir}
                    onChangeText={(t) => set("workdir", t)}
                  />
                </Accordion>
              </Stack>
            ) : (
              <Stack
                spacing="xl"
                style={{ alignItems: "center", paddingTop: spacing.md }}
              >
                <TouchableOpacity>
                  <Flag size={20} color={colors.text.secondary} />
                </TouchableOpacity>
                <TouchableOpacity>
                  <Zap size={20} color={colors.text.secondary} />
                </TouchableOpacity>
                <TouchableOpacity>
                  <Target size={20} color={colors.text.secondary} />
                </TouchableOpacity>
                <TouchableOpacity>
                  <Lock size={20} color={colors.text.secondary} />
                </TouchableOpacity>
              </Stack>
            )}
          </ScrollView>
        </View>

        {/* CENTER WORKSPACE (Hero) */}
        <View style={styles.main}>
          {/* Status Bar */}
          <View style={styles.statusBar}>
            <Stack
              direction="row"
              spacing="md"
              style={{ alignItems: "center" }}
            >
              <Wifi size={14} color={colors.success} />
              <Text style={styles.statusText}>{copyFeedback || "ONLINE"}</Text>
              <View style={styles.divider} />
              <Text style={styles.statusText}>
                {PROMPT_TYPES[promptType].name.toUpperCase()}
              </Text>
            </Stack>
            <View style={{ flex: 1 }} />
            <Button
              title={isSaving ? "SAVING..." : "COPY"}
              onPress={copyToClipboard}
              size="sm"
              style={{ marginRight: 8, backgroundColor: "#334155" }}
            />
            <Button title="GENERATE" onPress={generate} size="sm" />
          </View>

          {/* Editor Area */}
          <View style={styles.editorContainer}>
            <View style={styles.lineNumbers}>
              {Array.from({ length: 30 }).map((_, i) => (
                <Text key={i} style={styles.lineNumber}>
                  {i + 1}
                </Text>
              ))}
            </View>
            <ScrollView style={styles.editorScroll}>
              <Text style={styles.code}>{output}</Text>
            </ScrollView>
          </View>

          {/* Bottom Prompter */}
          <View style={styles.prompterArea}>
            <Text style={styles.sectionLabel}>MISSION OBJECTIVE</Text>
            <Input
              multiline
              numberOfLines={3}
              value={form.objective}
              onChangeText={(t) => set("objective", t)}
              style={{
                backgroundColor: colors.background.primary,
                borderWidth: 0,
                fontFamily: "monospace",
              }}
            />
          </View>
        </View>

        {/* RIGHT DRAWER (FINDERS/READERS) */}
        <View
          style={[
            styles.drawer,
            {
              width: rightOpen ? 260 : 50,
              borderRightWidth: 0,
              borderLeftWidth: 1,
            },
          ]}
        >
          <View style={styles.drawerHeader}>
            {rightOpen && (
              <Text style={styles.drawerTitle}>FINDERS (CONTEXT)</Text>
            )}
            <TouchableOpacity
              onPress={() => setRightOpen(!rightOpen)}
              style={styles.iconButton}
            >
              {rightOpen ? (
                <ChevronRight size={16} color={colors.text.secondary} />
              ) : (
                <ChevronLeft size={16} color={colors.text.secondary} />
              )}
            </TouchableOpacity>
          </View>

          <ScrollView style={styles.drawerScroll}>
            {rightOpen ? (
              <Stack spacing="md">
                {/* GROUP 1: CONTEXT */}
                <Accordion title="1. CONTEXT SOURCES" expanded={true}>
                  <Stack spacing="xs">
                    {CONTEXT_SOURCES.map((tool) => (
                      <Checkbox
                        key={tool.id}
                        label={tool.label}
                        checked={form[tool.id]}
                        onChange={(c) => set(tool.id, c)}
                      />
                    ))}
                  </Stack>
                </Accordion>

                {/* GROUP 2: TOOLS (CAPABILITIES) */}
                <Accordion title="2. ACTIVE TOOLS" expanded={true}>
                  <Grid columns={2} gap={4}>
                    {TOOLS_LIST.map((tool) => (
                      <TouchableOpacity
                        key={tool.id}
                        onPress={() => set(tool.id, !form[tool.id])}
                        style={[
                          styles.miniTool,
                          form[tool.id] && styles.miniToolActive,
                        ]}
                      >
                        <tool.Icon
                          size={12}
                          color={
                            form[tool.id]
                              ? colors.primary
                              : colors.text.secondary
                          }
                        />
                        <Text
                          style={[
                            styles.labelTiny,
                            form[tool.id] && { color: colors.primary },
                          ]}
                        >
                          {tool.label}
                        </Text>
                      </TouchableOpacity>
                    ))}
                  </Grid>
                </Accordion>

                {/* GROUP 3: HISTORY */}
                <Accordion title="3. MISSION HISTORY">
                  <Stack spacing="xs">
                    {RECENT_MISSIONS.map((m) => (
                      <TouchableOpacity key={m.id} style={styles.historyItem}>
                        <Stack
                          direction="row"
                          spacing="sm"
                          style={{ alignItems: "center" }}
                        >
                          {m.status === "COMPLETE" ? (
                            <CheckCircle size={10} color={colors.success} />
                          ) : (
                            <Activity size={10} color={colors.primary} />
                          )}
                          <Text style={styles.labelBold}>{m.title}</Text>
                        </Stack>
                        <Text style={styles.labelTiny}>{m.time}</Text>
                      </TouchableOpacity>
                    ))}
                  </Stack>
                </Accordion>

                {/* GROUP 4: REFERENCE */}
                <Accordion title="4. REFERENCE">
                  <Stack spacing="xs">
                    <TouchableOpacity style={styles.rowItem}>
                      <BookOpen size={12} color={colors.text.secondary} />
                      <Text style={styles.labelSmall}>RFC Index</Text>
                    </TouchableOpacity>
                    <TouchableOpacity style={styles.rowItem}>
                      <List size={12} color={colors.text.secondary} />
                      <Text style={styles.labelSmall}>Prompt Templates</Text>
                    </TouchableOpacity>
                  </Stack>
                </Accordion>
              </Stack>
            ) : (
              <Stack
                spacing="xl"
                style={{ alignItems: "center", paddingTop: spacing.md }}
              >
                <TouchableOpacity>
                  <Brain size={20} color={colors.text.secondary} />
                </TouchableOpacity>
                <TouchableOpacity>
                  <Wrench size={20} color={colors.text.secondary} />
                </TouchableOpacity>
                <TouchableOpacity>
                  <Clock size={20} color={colors.text.secondary} />
                </TouchableOpacity>
                <TouchableOpacity>
                  <BookOpen size={20} color={colors.text.secondary} />
                </TouchableOpacity>
              </Stack>
            )}
          </ScrollView>
        </View>
      </View>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  safeArea: { flex: 1, backgroundColor: "#0F172A" },
  screen: {
    flex: 1,
    flexDirection: "row",
    height: Platform.OS === "web" ? "100vh" : "100%",
    backgroundColor: "#0F172A",
    color: colors.text.primary,
  },
  drawer: {
    backgroundColor: colors.surface.primary,
    borderRightWidth: 1,
    borderLeftColor: colors.border.secondary,
    borderRightColor: colors.border.secondary,
    display: "flex",
    flexDirection: "column",
  },
  drawerHeader: {
    height: 36,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    paddingHorizontal: spacing.sm,
    borderBottomWidth: 1,
    borderBottomColor: colors.border.secondary,
    backgroundColor: "#0F172A",
  },
  iconButton: {
    width: 24,
    height: 24,
    borderRadius: borderRadius.sm,
    alignItems: "center",
    justifyContent: "center",
    backgroundColor: "transparent",
  },
  drawerTitle: {
    fontSize: 10,
    fontWeight: "700",
    color: colors.text.secondary,
    letterSpacing: 1.5,
    textTransform: "uppercase",
  },
  drawerScroll: { flex: 1, padding: spacing.sm, backgroundColor: "#020617" },
  main: {
    flex: 1,
    display: "flex",
    flexDirection: "column",
    backgroundColor: "#0B1120",
  },
  statusBar: {
    height: 36,
    backgroundColor: "#0F172A",
    borderBottomWidth: 1,
    borderBottomColor: colors.border.secondary,
    flexDirection: "row",
    alignItems: "center",
    paddingHorizontal: spacing.md,
  },
  statusText: {
    fontSize: 11,
    fontFamily: "monospace",
    color: colors.text.secondary,
  },
  divider: {
    width: 1,
    height: 16,
    backgroundColor: colors.border.secondary,
    marginHorizontal: spacing.sm,
  },
  editorContainer: { flex: 1, flexDirection: "row" },
  lineNumbers: {
    width: 40,
    paddingVertical: spacing.md,
    backgroundColor: colors.surface.primary,
    borderRightWidth: 1,
    borderRightColor: colors.border.secondary,
    alignItems: "center",
  },
  lineNumber: {
    fontSize: 12,
    fontFamily: "monospace",
    color: colors.text.tertiary,
    lineHeight: 20,
  },
  editorScroll: { flex: 1, padding: spacing.md },
  code: {
    fontFamily: "monospace",
    fontSize: 13,
    color: "#E2E8F0",
    lineHeight: 20,
  },
  prompterArea: {
    height: 150,
    borderTopWidth: 1,
    borderTopColor: colors.border.secondary,
    backgroundColor: colors.surface.primary,
    padding: spacing.md,
  },
  sectionLabel: {
    fontSize: 10,
    fontWeight: "bold",
    color: colors.text.tertiary,
    marginBottom: spacing.sm,
    letterSpacing: 1,
  },
  typeBtn: {
    width: "100%",
    height: 32,
    alignItems: "center",
    justifyContent: "center",
    borderWidth: 1,
    borderColor: colors.border.secondary,
    borderRadius: borderRadius.sm,
  },
  typeBtnActive: {
    borderColor: colors.primary,
    backgroundColor: colors.primary + "20",
  },
  rowItem: {
    flexDirection: "row",
    alignItems: "center",
    padding: 6,
    borderRadius: borderRadius.sm,
    gap: 8,
  },
  rowItemActive: { backgroundColor: colors.surface.secondary },
  labelSmall: { fontSize: 11, color: colors.text.secondary },
  labelTiny: { fontSize: 9, color: colors.text.tertiary },
  labelBold: { fontSize: 11, fontWeight: "bold", color: colors.text.secondary },
  miniButton: {
    height: 28,
    alignItems: "center",
    justifyContent: "center",
    borderWidth: 1,
    borderColor: colors.border.secondary,
    borderRadius: borderRadius.sm,
  },
  miniTool: {
    flexDirection: "row",
    alignItems: "center",
    gap: 4,
    padding: 4,
    borderRadius: borderRadius.sm,
    borderWidth: 1,
    borderColor: "transparent",
  },
  miniToolActive: {
    borderColor: colors.primary,
    backgroundColor: colors.primary + "10",
  },
  historyItem: { padding: 4, marginBottom: 2 },
});
