import type React from "react"
import { View, Text, TouchableOpacity, StyleSheet, ScrollView } from "react-native"
import { colors, spacing, typography } from "../tokens"

export interface GlyphRailTab {
  id: string
  label: string
  icon?: string
}

export interface GlyphRailProps {
  title: string
  tabs: GlyphRailTab[]
  activeTab: string
  onTabChange: (tabId: string) => void
  collapsed: boolean
  onToggleCollapse: () => void
  children: React.ReactNode
  side: "left" | "right"
}

export const GlyphRail: React.FC<GlyphRailProps> = ({
  title,
  tabs,
  activeTab,
  onTabChange,
  collapsed,
  onToggleCollapse,
  children,
  side,
}) => {
  return (
    <View style={[styles.container, side === "right" && styles.containerRight]}>
      {/* Header with title and collapse button */}
      <View style={styles.header}>
        <Text style={styles.title}>{title}</Text>
        <TouchableOpacity onPress={onToggleCollapse} style={styles.collapseButton}>
          <Text style={styles.collapseIcon}>
            {collapsed ? (side === "left" ? "→" : "←") : side === "left" ? "←" : "→"}
          </Text>
        </TouchableOpacity>
      </View>

      {!collapsed && (
        <>
          {/* Tab navigation */}
          <View style={styles.tabContainer}>
            <ScrollView horizontal showsHorizontalScrollIndicator={false} style={styles.tabScroll}>
              {tabs.map((tab) => (
                <TouchableOpacity
                  key={tab.id}
                  style={[styles.tab, activeTab === tab.id && styles.tabActive]}
                  onPress={() => onTabChange(tab.id)}
                >
                  <Text style={[styles.tabText, activeTab === tab.id && styles.tabTextActive]}>{tab.label}</Text>
                </TouchableOpacity>
              ))}
            </ScrollView>
          </View>

          {/* Tab content */}
          <ScrollView style={styles.content}>{children}</ScrollView>
        </>
      )}
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    backgroundColor: colors.surface.primary,
    borderRightWidth: 1,
    borderRightColor: colors.border.primary,
    minWidth: 280,
    maxWidth: 400,
    flex: 1,
  },
  containerRight: {
    borderRightWidth: 0,
    borderLeftWidth: 1,
    borderLeftColor: colors.border.primary,
  },
  header: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    padding: spacing.lg,
    borderBottomWidth: 1,
    borderBottomColor: colors.border.primary,
  },
  title: {
    fontSize: typography.fontSize.lg,
    fontWeight: typography.fontWeight.semibold.toString(),
    color: colors.text.primary,
  },
  collapseButton: {
    padding: spacing.sm,
  },
  collapseIcon: {
    fontSize: typography.fontSize.lg,
    color: colors.text.secondary,
  },
  tabContainer: {
    borderBottomWidth: 1,
    borderBottomColor: colors.border.primary,
  },
  tabScroll: {
    flexDirection: "row",
  },
  tab: {
    paddingVertical: spacing.md,
    paddingHorizontal: spacing.lg,
    borderBottomWidth: 2,
    borderBottomColor: "transparent",
  },
  tabActive: {
    borderBottomColor: colors.primary,
  },
  tabText: {
    fontSize: typography.fontSize.sm,
    color: colors.text.secondary,
    fontWeight: typography.fontWeight.medium.toString(),
  },
  tabTextActive: {
    color: colors.primary,
    fontWeight: typography.fontWeight.semibold.toString(),
  },
  content: {
    flex: 1,
    padding: spacing.lg,
  },
})
