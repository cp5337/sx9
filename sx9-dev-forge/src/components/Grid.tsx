import React from "react"
import { View, StyleSheet, type ViewStyle } from "react-native"
import { spacing } from "../tokens"
import { useResponsive } from "../hooks/useResponsive"

export interface GridProps {
  children: React.ReactNode
  columns?: {
    mobile?: number
    tablet?: number
    desktop?: number
  }
  gap?: keyof typeof spacing
  style?: ViewStyle
}

export const Grid: React.FC<GridProps> = ({
  children,
  columns = { mobile: 1, tablet: 2, desktop: 3 },
  gap = "md",
  style,
}) => {
  const { isMobile, isTablet } = useResponsive()

  const columnCount = isMobile ? columns.mobile : isTablet ? columns.tablet : columns.desktop

  return (
    <View
      style={[
        styles.grid,
        {
          gap: spacing[gap],
        },
        style,
      ]}
    >
      {React.Children.map(children, (child) => (
        <View
          style={{
            flex: 1,
            minWidth: `${100 / (columnCount || 1)}%`,
            maxWidth: `${100 / (columnCount || 1)}%`,
          }}
        >
          {child}
        </View>
      ))}
    </View>
  )
}

const styles = StyleSheet.create({
  grid: {
    flexDirection: "row",
    flexWrap: "wrap",
  },
})
