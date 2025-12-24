import { AppRegistry } from "react-native-web"
import { Provider } from "react-redux"
import { store } from "./store/store"
import App from "./App"
import "./index.css"

// Wrap App with Redux Provider
const AppWithProviders = () => (
  <Provider store={store}>
    <App />
  </Provider>
)

// Register the app with React Native Web
AppRegistry.registerComponent("CX9TemplateUI", () => AppWithProviders)

// Get root element
const rootElement = document.getElementById("root")

if (rootElement) {
  // Render using React Native Web
  AppRegistry.runApplication("CX9TemplateUI", {
    rootTag: rootElement,
  })
}
