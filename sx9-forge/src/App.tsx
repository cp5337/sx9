import { StyleSheet } from "react-native"
import { PromptForgeScreen } from "./screens/PromptForgeScreen"
import { colors } from "./tokens"

export default function App() {
  return <PromptForgeScreen />
}

const styles = StyleSheet.create({
  scrollView: {
    flex: 1,
    backgroundColor: colors.background.primary,
  },
  header: {
    paddingVertical: 24,
    alignItems: "center",
  },
  title: {
    fontSize: 24,
    fontWeight: "bold",
    color: colors.text.primary,
    marginBottom: 8,
  },
  subtitle: {
    fontSize: 16,
    color: colors.text.secondary,
  },
  sectionTitle: {
    fontSize: 20,
    fontWeight: "semibold",
    color: colors.text.primary,
  },
  cardContent: {
    fontSize: 16,
    color: colors.text.secondary,
  },
})
