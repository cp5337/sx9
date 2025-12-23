import type React from "react"
import { TouchableOpacity, Text, StyleSheet, ActivityIndicator, type ViewStyle, type TextStyle } from "react-native"
import { colors, spacing, typography, borderRadius } from "../tokens"

export interface ButtonProps {
  title: string
  onPress: () => void
  variant?: "primary" | "secondary" | "outline" | "ghost"
  size?: "sm" | "md" | "lg"
  disabled?: boolean
  loading?: boolean
  fullWidth?: boolean
  style?: ViewStyle
}

export const Button: React.FC<ButtonProps> = ({
  title,
  onPress,
  variant = "primary",
  size = "md",
  disabled = false,
  loading = false,
  fullWidth = false,
  style,
}) => {
  const buttonStyle: ViewStyle[] = [
    styles.base,
    styles[variant],
    styles[`size_${size}`],
    fullWidth && styles.fullWidth,
    (disabled || loading) && styles.disabled,
    style,
  ]

  const textStyle: TextStyle[] = [
    styles.text,
    styles[`text_${variant}`],
    styles[`text_${size}`],
    (disabled || loading) && styles.textDisabled,
  ]

  return (
    <TouchableOpacity style={buttonStyle} onPress={onPress} disabled={disabled || loading} activeOpacity={0.7}>
      {loading ? (
        <ActivityIndicator color={variant === "primary" ? colors.text.inverse : colors.primary} />
      ) : (
        <Text style={textStyle}>{title}</Text>
      )}
    </TouchableOpacity>
  )
}

const styles = StyleSheet.create({
  base: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    borderRadius: borderRadius.md,
    minHeight: 44, // Touch target minimum
  },
  primary: {
    backgroundColor: colors.primary,
  },
  secondary: {
    backgroundColor: colors.surface.secondary,
  },
  outline: {
    backgroundColor: "transparent",
    borderWidth: 1,
    borderColor: colors.border.primary,
  },
  ghost: {
    backgroundColor: "transparent",
  },
  size_sm: {
    paddingHorizontal: spacing.md,
    paddingVertical: spacing.sm,
    minHeight: 36,
  },
  size_md: {
    paddingHorizontal: spacing.lg,
    paddingVertical: spacing.md,
    minHeight: 44,
  },
  size_lg: {
    paddingHorizontal: spacing.xl,
    paddingVertical: spacing.lg,
    minHeight: 52,
  },
  fullWidth: {
    width: "100%",
  },
  disabled: {
    opacity: 0.5,
  },
  text: {
    fontWeight: typography.fontWeight.semibold,
  },
  text_primary: {
    color: colors.text.inverse,
  },
  text_secondary: {
    color: colors.text.primary,
  },
  text_outline: {
    color: colors.text.primary,
  },
  text_ghost: {
    color: colors.primary,
  },
  text_sm: {
    fontSize: typography.fontSize.sm,
  },
  text_md: {
    fontSize: typography.fontSize.base,
  },
  text_lg: {
    fontSize: typography.fontSize.lg,
  },
  textDisabled: {
    opacity: 0.7,
  },
})
