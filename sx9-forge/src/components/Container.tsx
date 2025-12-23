import type React from "react"
import { View, StyleSheet, type ViewStyle } from "react-native"
import { spacing } from "../tokens"
import { useResponsive } from "../hooks/useResponsive"

export interface ContainerProps {
  children: React.ReactNode
  maxWidth?: "sm" | "md" | "lg" | "xl" | "full"
  padding?: keyof typeof spacing
  style?: ViewStyle
}

const maxWidths = {
  sm: 640,
  md: 768,
  lg: 1024,
  xl: 1280,
  full: "100%",
}

export const Container: React.FC<ContainerProps> = ({ children, maxWidth = "xl", padding = "md", style }) => {
  const { width } = useResponsive()

  return (
    <View
      style={[
        styles.container,
        {
          maxWidth: maxWidths[maxWidth],
          paddingHorizontal: spacing[padding],
          width: width < (maxWidths[maxWidth] as number) ? "100%" : maxWidths[maxWidth],
        },
        style,
      ]}
    >
      {children}
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    width: "100%",
    marginHorizontal: "auto",
  },
})
