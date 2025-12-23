
import type React from "react"
import { useState } from "react"
import { ScrollView, Text, StyleSheet } from "react-native"
import { Container } from "../components/Container"
import { Stack } from "../components/Stack"
import { Card } from "../components/Card"
import { Button } from "../components/Button"
import { Input } from "../components/Input"
import { Grid } from "../components/Grid"
import { colors, typography } from "../tokens"
import { useResponsive } from "../hooks/useResponsive"

export const ComponentShowcaseScreen: React.FC = () => {
  const { deviceType } = useResponsive()
  const [email, setEmail] = useState("")
  const [loading, setLoading] = useState(false)

  const handleSubmit = () => {
    setLoading(true)
    setTimeout(() => setLoading(false), 2000)
  }

  return (
    <ScrollView style={styles.scrollView}>
      <Container maxWidth="lg" padding="lg">
        <Stack spacing="2xl">
          {/* Header */}
          <Stack spacing="md">
            <Text style={styles.title}>Component Showcase</Text>
            <Text style={styles.description}>
              A comprehensive library of adaptive components for {deviceType} devices
            </Text>
          </Stack>

          {/* Buttons Section */}
          <Card variant="outlined" padding="lg">
            <Stack spacing="lg">
              <Text style={styles.sectionTitle}>Buttons</Text>
              <Stack spacing="md">
                <Button title="Primary Button" onPress={() => {}} />
                <Button title="Secondary Button" onPress={() => {}} variant="secondary" />
                <Button title="Outline Button" onPress={() => {}} variant="outline" />
                <Button title="Ghost Button" onPress={() => {}} variant="ghost" />
                <Button title="Loading Button" onPress={() => {}} loading={loading} />
                <Button title="Disabled Button" onPress={() => {}} disabled />
              </Stack>
            </Stack>
          </Card>

          {/* Inputs Section */}
          <Card variant="outlined" padding="lg">
            <Stack spacing="lg">
              <Text style={styles.sectionTitle}>Inputs</Text>
              <Input
                label="Email Address"
                placeholder="Enter your email"
                value={email}
                onChangeText={setEmail}
                keyboardType="email-address"
                autoCapitalize="none"
              />
              <Input
                label="Password"
                placeholder="Enter your password"
                secureTextEntry
                helperText="Must be at least 8 characters"
              />
              <Input label="Error State" placeholder="This field has an error" error="This field is required" />
            </Stack>
          </Card>

          {/* Cards Section */}
          <Stack spacing="lg">
            <Text style={styles.sectionTitle}>Cards</Text>
            <Grid columns={{ mobile: 1, tablet: 2, desktop: 3 }} gap="md">
              <Card variant="default" padding="lg">
                <Stack spacing="sm">
                  <Text style={styles.cardTitle}>Default Card</Text>
                  <Text style={styles.cardText}>Standard card with border</Text>
                </Stack>
              </Card>
              <Card variant="elevated" padding="lg">
                <Stack spacing="sm">
                  <Text style={styles.cardTitle}>Elevated Card</Text>
                  <Text style={styles.cardText}>Card with shadow elevation</Text>
                </Stack>
              </Card>
              <Card variant="outlined" padding="lg">
                <Stack spacing="sm">
                  <Text style={styles.cardTitle}>Outlined Card</Text>
                  <Text style={styles.cardText}>Card with prominent border</Text>
                </Stack>
              </Card>
            </Grid>
          </Stack>

          {/* Form Example */}
          <Card variant="elevated" padding="lg">
            <Stack spacing="lg">
              <Text style={styles.sectionTitle}>Sign Up Form</Text>
              <Input label="Full Name" placeholder="John Doe" />
              <Input label="Email" placeholder="john@example.com" keyboardType="email-address" autoCapitalize="none" />
              <Input label="Password" placeholder="••••••••" secureTextEntry />
              <Button title="Create Account" onPress={handleSubmit} loading={loading} fullWidth />
            </Stack>
          </Card>
        </Stack>
      </Container>
    </ScrollView>
  )
}

export default ComponentShowcaseScreen

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
  description: {
    fontSize: typography.fontSize.lg,
    color: colors.text.secondary,
    lineHeight: typography.fontSize.lg * typography.lineHeight.relaxed,
  },
  sectionTitle: {
    fontSize: typography.fontSize["2xl"],
    fontWeight: typography.fontWeight.semibold,
    color: colors.text.primary,
  },
  cardTitle: {
    fontSize: typography.fontSize.lg,
    fontWeight: typography.fontWeight.semibold,
    color: colors.text.primary,
  },
  cardText: {
    fontSize: typography.fontSize.base,
    color: colors.text.secondary,
  },
})
