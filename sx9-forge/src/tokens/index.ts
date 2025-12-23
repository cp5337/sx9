import designTokens from "../../design-tokens.json"

export const colors = {
  primary: designTokens.color.primary.value,
  secondary: designTokens.color.secondary.value,
  accent: designTokens.color.accent.value,
  success: designTokens.color.success.value,
  warning: designTokens.color.warning.value,
  error: designTokens.color.error.value,
  background: {
    primary: designTokens.color.background.primary.value,
    secondary: designTokens.color.background.secondary.value,
    tertiary: designTokens.color.background.tertiary.value,
  },
  surface: {
    primary: designTokens.color.surface.primary.value,
    secondary: designTokens.color.surface.secondary.value,
    elevated: designTokens.color.surface.elevated.value,
  },
  text: {
    primary: designTokens.color.text.primary.value,
    secondary: designTokens.color.text.secondary.value,
    tertiary: designTokens.color.text.tertiary.value,
    inverse: designTokens.color.text.inverse.value,
    onPrimary: designTokens.color.text.inverse.value, // Added onPrimary alias for text on primary color
  },
  border: {
    primary: designTokens.color.border.primary.value,
    secondary: designTokens.color.border.secondary.value,
    focus: designTokens.color.border.focus.value,
  },
}

export const spacing = {
  xs: designTokens.spacing.xs.value,
  sm: designTokens.spacing.sm.value,
  md: designTokens.spacing.md.value,
  lg: designTokens.spacing.lg.value,
  xl: designTokens.spacing.xl.value,
  "2xl": designTokens.spacing["2xl"].value,
  "3xl": designTokens.spacing["3xl"].value,
}

export const typography = {
  fontSize: {
    xs: designTokens.typography.fontSize.xs.value,
    sm: designTokens.typography.fontSize.sm.value,
    base: designTokens.typography.fontSize.base.value,
    lg: designTokens.typography.fontSize.lg.value,
    xl: designTokens.typography.fontSize.xl.value,
    "2xl": designTokens.typography.fontSize["2xl"].value,
    "3xl": designTokens.typography.fontSize["3xl"].value,
    "4xl": designTokens.typography.fontSize["4xl"].value,
    "5xl": designTokens.typography.fontSize["5xl"].value,
  },
  fontWeight: {
    regular: designTokens.typography.fontWeight.regular.value,
    medium: designTokens.typography.fontWeight.medium.value,
    semibold: designTokens.typography.fontWeight.semibold.value,
    bold: designTokens.typography.fontWeight.bold.value,
  },
  lineHeight: {
    tight: designTokens.typography.lineHeight.tight.value,
    normal: designTokens.typography.lineHeight.normal.value,
    relaxed: designTokens.typography.lineHeight.relaxed.value,
  },
}

export const borderRadius = {
  sm: designTokens.borderRadius.sm.value,
  md: designTokens.borderRadius.md.value,
  lg: designTokens.borderRadius.lg.value,
  xl: designTokens.borderRadius.xl.value,
  full: designTokens.borderRadius.full.value,
}

export const shadow = designTokens.shadow

export const breakpoints = {
  mobile: designTokens.breakpoints.mobile.value,
  tablet: designTokens.breakpoints.tablet.value,
  desktop: designTokens.breakpoints.desktop.value,
  wide: designTokens.breakpoints.wide.value,
}

export const tokens = {
  color: colors,
  spacing,
  typography,
  borderRadius,
  shadow,
  breakpoints,
}
