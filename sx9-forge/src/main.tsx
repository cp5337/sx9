import { AppRegistry } from "react-native-web"
import App from "./App"
import "./index.css"

// Register the app with React Native Web
AppRegistry.registerComponent("CX9TemplateUI", () => App)

// Get root element
const rootElement = document.getElementById("root")

if (rootElement) {
  // Render using React Native Web
  AppRegistry.runApplication("CX9TemplateUI", {
    rootTag: rootElement,
  })
}
