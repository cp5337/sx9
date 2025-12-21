import type React from "react"
import { View, StyleSheet, type ViewStyle } from "react-native"
import { spacing } from "../tokens"

export interface StackProps {
  children: React.ReactNode
  direction?: "vertical" | "horizontal"
  spacing?: keyof typeof spacing
  align?: "start" | "center" | "end" | "stretch"
  justify?: "start" | "center" | "end" | "between" | "around"
  style?: ViewStyle
}

export const Stack: React.FC<StackProps> = ({
  children,
  direction = "vertical",
  spacing: spacingKey = "md",
  align = "stretch",
  justify = "start",
  style,
}) => {
  const alignItems = {
    start: "flex-start",
    center: "center",
    end: "flex-end",
    stretch: "stretch",
  }[align]

  const justifyContent = {
    start: "flex-start",
    center: "center",
    end: "flex-end",
    between: "space-between",
    around: "space-around",
  }[justify]

  return (
    <View
      style={[
        styles.stack,
        {
          flexDirection: direction === "horizontal" ? "row" : "column",
          gap: spacing[spacingKey],
          alignItems,
          justifyContent,
        },
        style,
      ]}
    >
      {children}
    </View>
  )
}

const styles = StyleSheet.create({
  stack: {
    display: "flex",
  },
})
