"use client"

import { useState, useEffect } from "react"
import { Dimensions, type ScaledSize } from "react-native"
import { breakpoints } from "../tokens"

export type DeviceType = "mobile" | "tablet" | "desktop" | "wide"

export interface ResponsiveInfo {
  width: number
  height: number
  deviceType: DeviceType
  isMobile: boolean
  isTablet: boolean
  isDesktop: boolean
  isWide: boolean
}

export const getDeviceType = (width: number): DeviceType => {
  if (width >= breakpoints.wide) return "wide"
  if (width >= breakpoints.desktop) return "desktop"
  if (width >= breakpoints.tablet) return "tablet"
  return "mobile"
}

export const useResponsive = (): ResponsiveInfo => {
  const [dimensions, setDimensions] = useState<ScaledSize>(Dimensions.get("window"))

  useEffect(() => {
    const subscription = Dimensions.addEventListener("change", ({ window }) => {
      setDimensions(window)
    })

    return () => subscription?.remove()
  }, [])

  const deviceType = getDeviceType(dimensions.width)

  return {
    width: dimensions.width,
    height: dimensions.height,
    deviceType,
    isMobile: deviceType === "mobile",
    isTablet: deviceType === "tablet",
    isDesktop: deviceType === "desktop",
    isWide: deviceType === "wide",
  }
}
