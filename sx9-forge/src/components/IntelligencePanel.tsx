
import { useEffect, useState } from "react"
import { View, Text, Pressable, ScrollView, ActivityIndicator, StyleSheet } from "react-native"
import { useIntelligenceStore, useIntelligenceActions } from "../store/intelligenceStore"
import {
  connectLeptose,
  connectChromaDB,
  queryPatterns,
  queryTools,
  queryThreats,
  askEEI,
} from "../store/intelligence/actions"
import {
  selectLeptoseStatus,
  selectChromaStatus,
  selectPatternResults,
  selectToolResults,
  selectThreatResults,
  selectEEIAnswer,
  selectIsQuerying,
} from "../store/intelligence/selectors"
import { tokens } from "../tokens"

interface IntelligencePanelProps {
  query: string
  onApplyPattern?: (pattern: any) => void
  onSelectTool?: (tool: any) => void
}

export function IntelligencePanel({ query, onApplyPattern, onSelectTool }: IntelligencePanelProps) {
  const { dispatch } = useIntelligenceActions()
  const [activeTab, setActiveTab] = useState<"patterns" | "tools" | "threats" | "eei">("patterns")

  const intelligence = useIntelligenceStore()
  const leptoseStatus = selectLeptoseStatus({ intelligence })
  const chromaStatus = selectChromaStatus({ intelligence })
  const patterns = selectPatternResults({ intelligence })
  const tools = selectToolResults({ intelligence })
  const threats = selectThreatResults({ intelligence })
  const eeiAnswer = selectEEIAnswer({ intelligence })
  const isQuerying = selectIsQuerying({ intelligence })

  // Connect on mount
  useEffect(() => {
    dispatch(connectLeptose())
    dispatch(connectChromaDB())
  }, [dispatch])

  // Query when text changes
  useEffect(() => {
    if (!query || query.length < 3) return

    switch (activeTab) {
      case "patterns":
        dispatch(queryPatterns(query, 5))
        break
      case "tools":
        dispatch(queryTools(query, 10))
        break
      case "threats":
        dispatch(queryThreats(query, 5))
        break
      case "eei":
        dispatch(askEEI(query))
        break
    }
  }, [query, activeTab, dispatch])

  const getStatusColor = (status: string) => {
    switch (status) {
      case "ready":
        return tokens.color.success
      case "querying":
        return tokens.color.warning
      case "error":
        return tokens.color.error
      default:
        return tokens.color.textSecondary
    }
  }

  return (
    <View style={styles.container}>
      {/* Status Bar */}
      <View style={styles.statusBar}>
        <View style={styles.statusItem}>
          <View style={[styles.statusDot, { backgroundColor: getStatusColor(leptoseStatus.status) }]} />
          <Text style={styles.statusText}>Leptose: {leptoseStatus.status}</Text>
        </View>
        <View style={styles.statusItem}>
          <View style={[styles.statusDot, { backgroundColor: getStatusColor(chromaStatus.status) }]} />
          <Text style={styles.statusText}>ChromaDB: {chromaStatus.status}</Text>
        </View>
        {isQuerying && <ActivityIndicator size="small" color={tokens.color.primary} />}
      </View>

      {/* Tabs */}
      <View style={styles.tabs}>
        {(["patterns", "tools", "threats", "eei"] as const).map((tab) => (
          <Pressable
            key={tab}
            style={[styles.tab, activeTab === tab && styles.tabActive]}
            onPress={() => setActiveTab(tab)}
          >
            <Text style={[styles.tabText, activeTab === tab && styles.tabTextActive]}>{tab.toUpperCase()}</Text>
          </Pressable>
        ))}
      </View>

      {/* Results */}
      <ScrollView style={styles.results}>
        {activeTab === "patterns" &&
          patterns.map((pattern, i) => (
            <Pressable key={i} style={styles.resultItem} onPress={() => onApplyPattern?.(pattern)}>
              <Text style={styles.resultTitle}>{pattern.pattern}</Text>
              <Text style={styles.resultMeta}>Similarity: {(pattern.similarity * 100).toFixed(0)}%</Text>
              <Text style={styles.resultText}>{pattern.voice_narrative}</Text>
            </Pressable>
          ))}

        {activeTab === "tools" &&
          tools.map((tool, i) => (
            <Pressable key={i} style={styles.resultItem} onPress={() => onSelectTool?.(tool)}>
              <Text style={styles.resultTitle}>{tool.tool_name}</Text>
              <Text style={styles.resultMeta}>
                {tool.category} • Entropy: {(tool.entropy * 100).toFixed(0)}%
              </Text>
              <Text style={styles.resultText}>{tool.why_relevant}</Text>
            </Pressable>
          ))}

        {activeTab === "threats" &&
          threats.map((threat, i) => (
            <View key={i} style={styles.resultItem}>
              <Text style={styles.resultTitle}>{threat.apt_group}</Text>
              <Text style={styles.resultMeta}>Techniques: {threat.techniques.join(", ")}</Text>
              <Text style={styles.resultText}>Tools: {threat.tools_used.join(", ")}</Text>
            </View>
          ))}

        {activeTab === "eei" && eeiAnswer && (
          <View style={styles.resultItem}>
            <Text style={styles.resultTitle}>Answer</Text>
            <Text style={styles.resultMeta}>Confidence: {(eeiAnswer.confidence * 100).toFixed(0)}%</Text>
            <Text style={styles.resultText}>{eeiAnswer.answer}</Text>
            {eeiAnswer.sources.length > 0 && (
              <Text style={styles.resultMeta}>Sources: {eeiAnswer.sources.join(", ")}</Text>
            )}
          </View>
        )}

        {!isQuerying &&
          patterns.length === 0 &&
          tools.length === 0 &&
          threats.length === 0 &&
          !eeiAnswer &&
          query.length >= 3 && <Text style={styles.emptyText}>No results found</Text>}

        {query.length < 3 && <Text style={styles.emptyText}>Enter at least 3 characters to search</Text>}
      </ScrollView>
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: tokens.color.background,
  },
  statusBar: {
    flexDirection: "row",
    alignItems: "center",
    gap: tokens.spacing.md,
    padding: tokens.spacing.sm,
    backgroundColor: tokens.color.surface,
    borderBottomWidth: 1,
    borderBottomColor: tokens.color.border,
  },
  statusItem: {
    flexDirection: "row",
    alignItems: "center",
    gap: tokens.spacing.xs,
  },
  statusDot: {
    width: 8,
    height: 8,
    borderRadius: 4,
  },
  statusText: {
    fontSize: tokens.typography.fontSize.sm,
    color: tokens.color.textSecondary,
  },
  tabs: {
    flexDirection: "row",
    borderBottomWidth: 1,
    borderBottomColor: tokens.color.border,
  },
  tab: {
    flex: 1,
    paddingVertical: tokens.spacing.sm,
    alignItems: "center",
  },
  tabActive: {
    borderBottomWidth: 2,
    borderBottomColor: tokens.color.primary,
  },
  tabText: {
    fontSize: tokens.typography.fontSize.sm,
    fontWeight: tokens.typography.fontWeight.medium,
    color: tokens.color.textSecondary,
  },
  tabTextActive: {
    color: tokens.color.primary,
  },
  results: {
    flex: 1,
    padding: tokens.spacing.md,
  },
  resultItem: {
    backgroundColor: tokens.color.surface,
    padding: tokens.spacing.md,
    borderRadius: tokens.borderRadius.md,
    marginBottom: tokens.spacing.sm,
    borderWidth: 1,
    borderColor: tokens.color.border,
  },
  resultTitle: {
    fontSize: tokens.typography.fontSize.base,
    fontWeight: tokens.typography.fontWeight.semibold,
    color: tokens.color.textPrimary,
    marginBottom: tokens.spacing.xs,
  },
  resultMeta: {
    fontSize: tokens.typography.fontSize.sm,
    color: tokens.color.textSecondary,
    marginBottom: tokens.spacing.xs,
  },
  resultText: {
    fontSize: tokens.typography.fontSize.sm,
    color: tokens.color.textPrimary,
    lineHeight: 20,
  },
  emptyText: {
    fontSize: tokens.typography.fontSize.base,
    color: tokens.color.textSecondary,
    textAlign: "center",
    marginTop: tokens.spacing.xl,
  },
})
