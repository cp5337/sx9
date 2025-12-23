
import type React from "react"
import { View, Text, StyleSheet, type ViewStyle } from "react-native"
import { Slider as RNSlider } from "@react-native-community/slider"
import { colors, spacing, typography } from "../tokens"

export interface SliderProps {
  label?: string
  value: number
  onValueChange: (value: number) => void
  minimumValue?: number
  maximumValue?: number
  step?: number
  showValue?: boolean
  style?: ViewStyle
}

export const Slider: React.FC<SliderProps> = ({
  label,
  value,
  onValueChange,
  minimumValue = 0,
  maximumValue = 1,
  step = 0.1,
  showValue = true,
  style,
}) => {
  return (
    <View style={[styles.container, style]}>
      <View style={styles.labelRow}>
        {label && <Text style={styles.label}>{label}</Text>}
        {showValue && <Text style={styles.value}>{value.toFixed(2)}</Text>}
      </View>
      <RNSlider
        value={value}
        onValueChange={onValueChange}
        minimumValue={minimumValue}
        maximumValue={maximumValue}
        step={step}
        minimumTrackTintColor={colors.primary}
        maximumTrackTintColor={colors.border.primary}
        thumbTintColor={colors.primary}
        style={styles.slider}
      />
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    gap: spacing.sm,
  },
  labelRow: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
  },
  label: {
    fontSize: typography.fontSize.sm,
    color: colors.text.secondary,
    fontWeight: typography.fontWeight.medium.toString(),
  },
  value: {
    fontSize: typography.fontSize.sm,
    color: colors.text.primary,
    fontWeight: typography.fontWeight.semibold.toString(),
  },
  slider: {
    width: "100%",
  },
})
