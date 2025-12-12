/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_MAPBOX_TOKEN: string
  readonly VITE_BOLT_NEW_URL: string
  readonly VITE_WOLFRAM_APP_ID: string
  readonly VITE_OPENAI_API_KEY: string
  readonly VITE_SHODAN_API_KEY: string
  readonly VITE_NEO4J_URI: string
  readonly VITE_NEO4J_USER: string
  readonly VITE_NEO4J_PASSWORD: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}