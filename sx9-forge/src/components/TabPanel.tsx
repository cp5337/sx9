import type React from "react"
import { View, Text, StyleSheet } from "react-native"
import { spacing, typography, colors } from "../tokens"

export interface TabPanelProps {
  title?: string
  children: React.ReactNode
}

export const TabPanel: React.FC<TabPanelProps> = ({ title, children }) => {
  return (
    <View style={styles.container}>
      {title && <Text style={styles.title}>{title}</Text>}
      {children}
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    gap: spacing.lg,
  },
  title: {
    fontSize: typography.fontSize.base,
    fontWeight: typography.fontWeight.semibold.toString(),
    color: colors.text.primary,
    marginBottom: spacing.sm,
  },
})
