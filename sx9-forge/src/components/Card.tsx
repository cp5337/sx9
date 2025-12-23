import type React from "react"
import { View, StyleSheet, type ViewStyle } from "react-native"
import { colors, spacing, borderRadius, shadow } from "../tokens"

export interface CardProps {
  children: React.ReactNode
  variant?: "default" | "elevated" | "outlined"
  padding?: keyof typeof spacing
  style?: ViewStyle
}

export const Card: React.FC<CardProps> = ({ children, variant = "default", padding = "md", style }) => {
  return <View style={[styles.base, styles[variant], { padding: spacing[padding] }, style]}>{children}</View>
}

const styles = StyleSheet.create({
  base: {
    borderRadius: borderRadius.lg,
    backgroundColor: colors.surface.primary,
  },
  default: {
    borderWidth: 1,
    borderColor: colors.border.secondary,
  },
  elevated: {
    ...shadow.lg,
    borderWidth: 0,
  },
  outlined: {
    borderWidth: 1,
    borderColor: colors.border.primary,
  },
})
