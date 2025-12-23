
import type React from "react"
import { useState } from "react"
import { View, StyleSheet, Text, Pressable } from "react-native"
import { GlyphRail, type GlyphRailTab } from "../components/GlyphRail"
import { TabPanel } from "../components/TabPanel"
import { Select, type SelectOption } from "../components/Select"
import { Input } from "../components/Input"
import { Button } from "../components/Button"
import { Slider } from "../components/Slider"
import { CodeEditor } from "../components/CodeEditor"
import { IntelligencePanel } from "../components/IntelligencePanel"
import { useForgeStore } from "../store/forgeStore"
import { useFileSystem, useLinear, useSlack } from "../hooks/useAPI"
import { cx9API } from "../lib/tauri"
import { colors, spacing, typography } from "../tokens"

export const PromptForgeScreen: React.FC = () => {
  const [showIntelligence, setShowIntelligence] = useState(false)
  const [intelligenceHeight, setIntelligenceHeight] = useState(300)

  const {
    harness,
    persona,
    inference,
    linear,
    slack,
    context,
    ui,
    editor,
    setHarnessField,
    setPersonaField,
    setInferenceField,
    setLinearField,
    setSlackField,
    setContextField,
    setUIField,
    setEditorContent,
    setEditorLanguage,
    markEditorClean,
  } = useForgeStore()

  const { savePrompt, loadPrompt, loading: fsLoading } = useFileSystem()
  const { fetchIssue, createIssue, loading: linearLoading } = useLinear()
  const { sendNotification, loading: slackLoading } = useSlack()

  // Tab definitions
  const leftTabs: GlyphRailTab[] = [
    { id: "harness", label: "Harness" },
    { id: "persona", label: "Persona" },
    { id: "inference", label: "Inference" },
  ]

  const rightTabs: GlyphRailTab[] = [
    { id: "linear", label: "Linear" },
    { id: "slack", label: "Slack" },
    { id: "context", label: "Context" },
  ]

  const statusOptions: SelectOption[] = [
    { label: "Active", value: "active" },
    { label: "Idle", value: "idle" },
    { label: "Error", value: "error" },
  ]

  const complexityOptions: SelectOption[] = [
    { label: "Low", value: "low" },
    { label: "Medium", value: "medium" },
    { label: "High", value: "high" },
  ]

  const modelOptions: SelectOption[] = [
    { label: "GPT-4", value: "gpt-4" },
    { label: "GPT-3.5 Turbo", value: "gpt-3.5-turbo" },
    { label: "Claude 3", value: "claude-3" },
  ]

  const toneOptions: SelectOption[] = [
    { label: "Professional", value: "professional" },
    { label: "Casual", value: "casual" },
    { label: "Technical", value: "technical" },
    { label: "Creative", value: "creative" },
  ]

  const languageOptions: SelectOption[] = [
    { label: "Markdown", value: "markdown" },
    { label: "Plain Text", value: "plaintext" },
    { label: "YAML", value: "yaml" },
    { label: "JSON", value: "json" },
  ]

  const handleSavePrompt = async () => {
    const filename = `prompt-${Date.now()}.md`
    const result = await savePrompt(editor.content, filename)
    if (result.success) {
      markEditorClean()
      console.log("[v0] Saved prompt to:", result.data)
    } else {
      console.error("[v0] Failed to save:", result.error)
    }
  }

  const handleExecuteHarness = async () => {
    console.log("[v0] Executing harness with config:", harness)
    if (cx9API.isTauri()) {
      try {
        const result = await cx9API.executeRustPattern("validate", harness.promptText)
        console.log("[v0] Harness result:", result)
      } catch (error) {
        console.error("[v0] Harness execution failed:", error)
      }
    }
  }

  const handleFetchLinearIssue = async () => {
    if (!linear.issueId) {
      console.error("[v0] No issue ID provided")
      return
    }
    const result = await fetchIssue(linear.issueId)
    if (result.success && result.data) {
      setLinearField("issueTitle", result.data.title)
      setLinearField("issueDescription", result.data.description)
      console.log("[v0] Fetched Linear issue:", result.data)
    } else {
      console.error("[v0] Failed to fetch issue:", result.error)
    }
  }

  const handleCreateLinearIssue = async () => {
    if (!linear.issueTitle) {
      console.error("[v0] No issue title provided")
      return
    }
    const result = await createIssue(linear.issueTitle, linear.issueDescription)
    if (result.success && result.data) {
      setLinearField("issueId", result.data.id)
      console.log("[v0] Created Linear issue:", result.data)
    } else {
      console.error("[v0] Failed to create issue:", result.error)
    }
  }

  const handleSendSlackNotification = async () => {
    if (!slack.channel || !slack.message) {
      console.error("[v0] Channel and message required")
      return
    }
    const result = await sendNotification(slack.channel, slack.message)
    if (result.success) {
      console.log("[v0] Sent Slack notification")
      setSlackField("message", "")
    } else {
      console.error("[v0] Failed to send notification:", result.error)
    }
  }

  const handleLoadContext = async () => {
    console.log("[v0] Loading context from:", context.contextPath)
    if (cx9API.isTauri()) {
      try {
        const files = await cx9API.listFiles(context.contextPath)
        console.log("[v0] Found context files:", files)
      } catch (error) {
        console.error("[v0] Failed to load context:", error)
      }
    }
  }

  const handleApplyPattern = (pattern: any) => {
    console.log("[v0] Applying pattern:", pattern)
    setEditorContent(pattern.voice_narrative)
  }

  const handleSelectTool = (tool: any) => {
    console.log("[v0] Selected tool:", tool)
    // Add tool to harness or context
    setHarnessField("promptText", `${harness.promptText}\n\nTool: ${tool.tool_name}`)
  }

  return (
    <View style={styles.container}>
      {/* Left Glyph Rail */}
      <GlyphRail
        title="Configuration"
        tabs={leftTabs}
        activeTab={ui.leftActiveTab}
        onTabChange={(tab) => setUIField("leftActiveTab", tab)}
        collapsed={ui.leftRailCollapsed}
        onToggleCollapse={() => setUIField("leftRailCollapsed", !ui.leftRailCollapsed)}
        side="left"
      >
        {ui.leftActiveTab === "harness" && (
          <TabPanel title="Harness Configuration">
            <Select
              label="Leptose Status"
              value={harness.leptoseStatus}
              options={statusOptions}
              onChange={(value) => setHarnessField("leptoseStatus", value)}
              placeholder="Select status"
            />
            <Select
              label="Chroma Status"
              value={harness.chromaStatus}
              options={statusOptions}
              onChange={(value) => setHarnessField("chromaStatus", value)}
              placeholder="Select status"
            />
            <Select
              label="Complexity"
              value={harness.complexity}
              options={complexityOptions}
              onChange={(value) => setHarnessField("complexity", value)}
              placeholder="Select complexity"
            />
            <Input
              label="Prompt Text"
              value={harness.promptText}
              onChangeText={(value) => setHarnessField("promptText", value)}
              placeholder="Enter prompt text"
              multiline
              numberOfLines={4}
            />
            <Button title="Execute Harness" onPress={handleExecuteHarness} variant="primary" disabled={fsLoading} />
          </TabPanel>
        )}
        {ui.leftActiveTab === "persona" && (
          <TabPanel title="Persona Settings">
            <Input
              label="Persona Name"
              value={persona.personaName}
              onChangeText={(value) => setPersonaField("personaName", value)}
              placeholder="Enter persona name"
            />
            <Input
              label="Persona Role"
              value={persona.personaRole}
              onChangeText={(value) => setPersonaField("personaRole", value)}
              placeholder="e.g., Senior Developer, Product Manager"
            />
            <Select
              label="Persona Tone"
              value={persona.personaTone}
              options={toneOptions}
              onChange={(value) => setPersonaField("personaTone", value)}
              placeholder="Select tone"
            />
          </TabPanel>
        )}
        {ui.leftActiveTab === "inference" && (
          <TabPanel title="Inference Settings">
            <Select
              label="Model"
              value={inference.model}
              options={modelOptions}
              onChange={(value) => setInferenceField("model", value)}
              placeholder="Select model"
            />
            <Slider
              label="Temperature"
              value={inference.temperature}
              onValueChange={(value) => setInferenceField("temperature", value)}
              minimumValue={0}
              maximumValue={2}
              step={0.1}
            />
            <Slider
              label="Top P"
              value={inference.topP}
              onValueChange={(value) => setInferenceField("topP", value)}
              minimumValue={0}
              maximumValue={1}
              step={0.05}
            />
            <Input
              label="Max Tokens"
              value={inference.maxTokens.toString()}
              onChangeText={(value) => setInferenceField("maxTokens", Number.parseInt(value) || 0)}
              placeholder="2048"
              keyboardType="numeric"
            />
          </TabPanel>
        )}
      </GlyphRail>

      {/* Center Content Area with Intelligence Panel */}
      <View style={styles.centerContent}>
        <View style={styles.editorHeader}>
          <View style={styles.editorHeaderLeft}>
            <Text style={styles.editorTitle}>
              {editor.currentFile || "Untitled"}
              {editor.isDirty && <Text style={styles.unsavedIndicator}> •</Text>}
            </Text>
          </View>
          <View style={styles.editorHeaderRight}>
            <Pressable
              style={[styles.intelligenceToggle, showIntelligence && styles.intelligenceToggleActive]}
              onPress={() => setShowIntelligence(!showIntelligence)}
            >
              <Text style={[styles.intelligenceToggleText, showIntelligence && styles.intelligenceToggleTextActive]}>
                Intelligence
              </Text>
            </Pressable>
            <Select
              value={editor.language}
              options={languageOptions}
              onChange={setEditorLanguage}
              style={styles.languageSelect}
            />
          </View>
        </View>

        <View style={{ flex: 1 }}>
          <View style={[styles.editorWrapper, showIntelligence && { height: `calc(100% - ${intelligenceHeight}px)` }]}>
            <CodeEditor
              value={editor.content}
              onChange={setEditorContent}
              language={editor.language}
              showToolbar={true}
              onSave={handleSavePrompt}
              onFormat={() => console.log("[v0] Format document")}
            />
          </View>

          {showIntelligence && (
            <View style={[styles.intelligencePanel, { height: intelligenceHeight }]}>
              <IntelligencePanel
                query={harness.promptText || editor.content.slice(0, 200)}
                onApplyPattern={handleApplyPattern}
                onSelectTool={handleSelectTool}
              />
            </View>
          )}
        </View>
      </View>

      {/* Right Glyph Rail */}
      <GlyphRail
        title="Integrations"
        tabs={rightTabs}
        activeTab={ui.rightActiveTab}
        onTabChange={(tab) => setUIField("rightActiveTab", tab)}
        collapsed={ui.rightRailCollapsed}
        onToggleCollapse={() => setUIField("rightRailCollapsed", !ui.rightRailCollapsed)}
        side="right"
      >
        {ui.rightActiveTab === "linear" && (
          <TabPanel title="Linear Integration">
            <Input
              label="Issue ID"
              value={linear.issueId}
              onChangeText={(value) => setLinearField("issueId", value)}
              placeholder="Enter Linear issue ID"
            />
            <Input
              label="Issue Title"
              value={linear.issueTitle}
              onChangeText={(value) => setLinearField("issueTitle", value)}
              placeholder="Issue title"
            />
            <Input
              label="Description"
              value={linear.issueDescription}
              onChangeText={(value) => setLinearField("issueDescription", value)}
              placeholder="Issue description"
              multiline
              numberOfLines={3}
            />
            <View style={{ flexDirection: "row", gap: spacing.sm }}>
              <Button
                title="Fetch Issue"
                onPress={handleFetchLinearIssue}
                variant="secondary"
                disabled={linearLoading}
                style={{ flex: 1 }}
              />
              <Button
                title="Create Issue"
                onPress={handleCreateLinearIssue}
                variant="primary"
                disabled={linearLoading}
                style={{ flex: 1 }}
              />
            </View>
          </TabPanel>
        )}
        {ui.rightActiveTab === "slack" && (
          <TabPanel title="Slack Notifications">
            <Input
              label="Channel"
              value={slack.channel}
              onChangeText={(value) => setSlackField("channel", value)}
              placeholder="#general"
            />
            <Input
              label="Message"
              value={slack.message}
              onChangeText={(value) => setSlackField("message", value)}
              placeholder="Enter message"
              multiline
              numberOfLines={3}
            />
            <Button
              title="Send Notification"
              onPress={handleSendSlackNotification}
              variant="secondary"
              disabled={slackLoading || !slack.channel || !slack.message}
            />
          </TabPanel>
        )}
        {ui.rightActiveTab === "context" && (
          <TabPanel title="Context Management">
            <Input
              label="Context Path"
              value={context.contextPath}
              onChangeText={(value) => setContextField("contextPath", value)}
              placeholder="/path/to/context"
            />
            <Input
              label="Active Context"
              value={context.activeContext}
              onChangeText={(value) => setContextField("activeContext", value)}
              placeholder="Current context"
            />
            <Button title="Load Context" onPress={handleLoadContext} variant="secondary" disabled={fsLoading} />
          </TabPanel>
        )}
      </GlyphRail>
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    flexDirection: "row",
    backgroundColor: colors.background.primary,
  },
  centerContent: {
    flex: 1,
    backgroundColor: colors.background.secondary,
  },
  editorHeader: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
    paddingHorizontal: spacing.lg,
    paddingVertical: spacing.md,
    backgroundColor: colors.surface.primary,
    borderBottomWidth: 1,
    borderBottomColor: colors.border.primary,
  },
  editorHeaderLeft: {
    flexDirection: "row",
    alignItems: "center",
  },
  editorTitle: {
    fontSize: typography.fontSize.base,
    fontWeight: typography.fontWeight.semibold.toString(),
    color: colors.text.primary,
  },
  unsavedIndicator: {
    color: colors.warning,
  },
  editorHeaderRight: {
    flexDirection: "row",
    gap: spacing.sm,
    alignItems: "center",
  },
  languageSelect: {
    minWidth: 150,
  },
  intelligenceToggle: {
    paddingHorizontal: spacing.md,
    paddingVertical: spacing.sm,
    borderRadius: spacing.sm,
    borderWidth: 1,
    borderColor: colors.border.primary,
    backgroundColor: colors.surface.secondary,
  },
  intelligenceToggleActive: {
    backgroundColor: colors.primary,
    borderColor: colors.primary,
  },
  intelligenceToggleText: {
    fontSize: typography.fontSize.sm,
    fontWeight: typography.fontWeight.medium.toString(),
    color: colors.text.secondary,
  },
  intelligenceToggleTextActive: {
    color: colors.text.onPrimary,
  },
  editorWrapper: {
    flex: 1,
  },
  intelligencePanel: {
    borderTopWidth: 1,
    borderTopColor: colors.border.primary,
  },
})
