import { defineConfig } from "vite"
import react from "@vitejs/plugin-react"
import wasm from "vite-plugin-wasm"
import topLevelAwait from "vite-plugin-top-level-await"
import path from "path"

export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
      "react-native": "react-native-web",
    },
    extensions: [".web.tsx", ".web.ts", ".web.jsx", ".web.js", ".tsx", ".ts", ".jsx", ".js"],
  },
  optimizeDeps: {
    exclude: ["cesium"], // Exclude WASM modules from optimization
    esbuildOptions: {
      target: "esnext",
    },
  },
  worker: {
    format: "es",
    plugins: [wasm(), topLevelAwait()],
  },
  build: {
    target: "esnext",
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ["react", "react-dom"],
          "react-native": ["react-native-web"],
        },
      },
    },
  },
  server: {
    port: 5173,        // Tauri standard port
    strictPort: true,  // Fail if port unavailable
  },
  envPrefix: ['VITE_', 'TAURI_'], // Tauri env vars
})
