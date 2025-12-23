import type React from "react"
import { ScrollView, Text, StyleSheet, View } from "react-native"
import { Container } from "../components/Container"
import { Stack } from "../components/Stack"
import { Card } from "../components/Card"
import { Button } from "../components/Button"
import { Grid } from "../components/Grid"
import { colors, spacing, typography } from "../tokens"
import { useResponsive } from "../hooks/useResponsive"

export const DashboardScreen: React.FC = () => {
  const { deviceType, width } = useResponsive()

  return (
    <ScrollView style={styles.scrollView}>
      <Container maxWidth="xl" padding="lg">
        <Stack spacing="xl">
          {/* Header */}
          <Stack spacing="sm">
            <Text style={styles.title}>Design System Dashboard</Text>
            <Text style={styles.subtitle}>
              Viewing on {deviceType} ({width}px wide)
            </Text>
          </Stack>

          {/* Stats Grid */}
          <Grid columns={{ mobile: 1, tablet: 2, desktop: 4 }} gap="md">
            <Card variant="elevated" padding="lg">
              <Stack spacing="sm">
                <Text style={styles.statLabel}>Components</Text>
                <Text style={styles.statValue}>24</Text>
                <Text style={styles.statChange}>+12% from last month</Text>
              </Stack>
            </Card>
            <Card variant="elevated" padding="lg">
              <Stack spacing="sm">
                <Text style={styles.statLabel}>Design Tokens</Text>
                <Text style={styles.statValue}>156</Text>
                <Text style={styles.statChange}>+8% from last month</Text>
              </Stack>
            </Card>
            <Card variant="elevated" padding="lg">
              <Stack spacing="sm">
                <Text style={styles.statLabel}>Active Projects</Text>
                <Text style={styles.statValue}>8</Text>
                <Text style={styles.statChange}>+2 new this week</Text>
              </Stack>
            </Card>
            <Card variant="elevated" padding="lg">
              <Stack spacing="sm">
                <Text style={styles.statLabel}>Team Members</Text>
                <Text style={styles.statValue}>12</Text>
                <Text style={styles.statChange}>All active</Text>
              </Stack>
            </Card>
          </Grid>

          {/* Recent Activity */}
          <Card variant="outlined" padding="lg">
            <Stack spacing="lg">
              <Text style={styles.cardTitle}>Recent Activity</Text>
              <Stack spacing="md">
                {[
                  "Button component updated with new variants",
                  "Design tokens synced across platforms",
                  "New color palette added to system",
                  "Typography scale refined for mobile",
                ].map((activity, index) => (
                  <View key={index} style={styles.activityItem}>
                    <View style={styles.activityDot} />
                    <Text style={styles.activityText}>{activity}</Text>
                  </View>
                ))}
              </Stack>
            </Stack>
          </Card>

          {/* Actions */}
          <Stack direction={deviceType === "mobile" ? "vertical" : "horizontal"} spacing="md">
            <Button
              title="Create Component"
              onPress={() => console.log("Create")}
              fullWidth={deviceType === "mobile"}
            />
            <Button
              title="View Documentation"
              onPress={() => console.log("Docs")}
              variant="outline"
              fullWidth={deviceType === "mobile"}
            />
            <Button
              title="Export Tokens"
              onPress={() => console.log("Export")}
              variant="ghost"
              fullWidth={deviceType === "mobile"}
            />
          </Stack>
        </Stack>
      </Container>
    </ScrollView>
  )
}

const styles = StyleSheet.create({
  scrollView: {
    flex: 1,
    backgroundColor: colors.background.primary,
  },
  title: {
    fontSize: typography.fontSize["4xl"],
    fontWeight: typography.fontWeight.bold,
    color: colors.text.primary,
  },
  subtitle: {
    fontSize: typography.fontSize.lg,
    color: colors.text.secondary,
  },
  statLabel: {
    fontSize: typography.fontSize.sm,
    color: colors.text.secondary,
    textTransform: "uppercase",
    letterSpacing: 0.5,
  },
  statValue: {
    fontSize: typography.fontSize["3xl"],
    fontWeight: typography.fontWeight.bold,
    color: colors.text.primary,
  },
  statChange: {
    fontSize: typography.fontSize.sm,
    color: colors.success,
  },
  cardTitle: {
    fontSize: typography.fontSize.xl,
    fontWeight: typography.fontWeight.semibold,
    color: colors.text.primary,
  },
  activityItem: {
    flexDirection: "row",
    alignItems: "center",
    gap: spacing.md,
  },
  activityDot: {
    width: 8,
    height: 8,
    borderRadius: 4,
    backgroundColor: colors.primary,
  },
  activityText: {
    flex: 1,
    fontSize: typography.fontSize.base,
    color: colors.text.secondary,
  },
})
