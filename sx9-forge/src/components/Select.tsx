
import type React from "react"
import { View, Text, StyleSheet, type ViewStyle } from "react-native"
import { Picker } from "@react-native-picker/picker"
import { colors, spacing, typography, borderRadius } from "../tokens"

export interface SelectOption {
  label: string
  value: string
}

export interface SelectProps {
  label?: string
  value: string
  options: SelectOption[]
  onChange: (value: string) => void
  placeholder?: string
  style?: ViewStyle
}

export const Select: React.FC<SelectProps> = ({ label, value, options, onChange, placeholder, style }) => {
  return (
    <View style={[styles.container, style]}>
      {label && <Text style={styles.label}>{label}</Text>}
      <View style={styles.pickerContainer}>
        <Picker
          selectedValue={value}
          onValueChange={onChange}
          style={styles.picker}
          dropdownIconColor={colors.text.secondary}
        >
          {placeholder && <Picker.Item label={placeholder} value="" />}
          {options.map((option) => (
            <Picker.Item key={option.value} label={option.label} value={option.value} />
          ))}
        </Picker>
      </View>
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    gap: spacing.sm,
  },
  label: {
    fontSize: typography.fontSize.sm,
    color: colors.text.secondary,
    fontWeight: typography.fontWeight.medium.toString(),
  },
  pickerContainer: {
    backgroundColor: colors.surface.secondary,
    borderRadius: borderRadius.md,
    borderWidth: 1,
    borderColor: colors.border.primary,
    overflow: "hidden",
  },
  picker: {
    color: colors.text.primary,
    backgroundColor: "transparent",
  },
})
