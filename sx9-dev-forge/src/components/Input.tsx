"use client"

import type React from "react"
import { useState } from "react"
import { TextInput, View, Text, StyleSheet, type TextInputProps, type ViewStyle } from "react-native"
import { colors, spacing, typography, borderRadius } from "../tokens"

export interface InputProps extends TextInputProps {
  label?: string
  error?: string
  helperText?: string
  containerStyle?: ViewStyle
}

export const Input: React.FC<InputProps> = ({ label, error, helperText, containerStyle, style, ...props }) => {
  const [isFocused, setIsFocused] = useState(false)

  return (
    <View style={[styles.container, containerStyle]}>
      {label && <Text style={styles.label}>{label}</Text>}
      <TextInput
        style={[styles.input, isFocused && styles.inputFocused, error && styles.inputError, style]}
        placeholderTextColor={colors.text.tertiary}
        onFocus={() => setIsFocused(true)}
        onBlur={() => setIsFocused(false)}
        {...props}
      />
      {error && <Text style={styles.errorText}>{error}</Text>}
      {helperText && !error && <Text style={styles.helperText}>{helperText}</Text>}
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    width: "100%",
  },
  label: {
    fontSize: typography.fontSize.sm,
    fontWeight: typography.fontWeight.medium,
    color: colors.text.primary,
    marginBottom: spacing.sm,
  },
  input: {
    backgroundColor: colors.surface.secondary,
    borderWidth: 1,
    borderColor: colors.border.secondary,
    borderRadius: borderRadius.md,
    paddingHorizontal: spacing.md,
    paddingVertical: spacing.md,
    fontSize: typography.fontSize.base,
    color: colors.text.primary,
    minHeight: 44, // Touch target minimum
  },
  inputFocused: {
    borderColor: colors.border.focus,
  },
  inputError: {
    borderColor: colors.error,
  },
  errorText: {
    fontSize: typography.fontSize.sm,
    color: colors.error,
    marginTop: spacing.xs,
  },
  helperText: {
    fontSize: typography.fontSize.sm,
    color: colors.text.secondary,
    marginTop: spacing.xs,
  },
})
