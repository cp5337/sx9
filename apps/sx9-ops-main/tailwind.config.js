/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      fontSize: {
        xxs: "0.625rem",
        "2xs": "11px",
      },
      colors: {
        dark: {
          bg: "#0a0e1a",
          surface: "#121826",
          elevated: "#1a2234",
          border: "#2a3447",
          text: {
            primary: "#e5e7eb",
            secondary: "#9ca3af",
            muted: "#6b7280",
          },
        },
        glow: {
          blue: "#60a5fa",
          DEFAULT: "#60a5fa",
        },
        status: {
          connected: "#10b981",
          running: "#10b981",
          disconnected: "#6b7280",
          stopped: "#6b7280",
          error: "#ef4444",
          warning: "#f59e0b",
        },
      },
      boxShadow: {
        "glow-sm": "0 0 0 2px rgba(96, 165, 250, 0.4)",
        "glow-md": "0 0 0 3px rgba(96, 165, 250, 0.6)",
        "glow-inset": "inset 2px 0 0 0 rgba(96, 165, 250, 0.6)",
        "glow-inset-active": "inset 3px 0 0 0 rgba(96, 165, 250, 0.8)",
      },
      animation: {
        "pulse-slow": "pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite",
      },
    },
  },
  plugins: [],
};
