/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/*.{vue,js,ts,jsx,tsx}",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
    "./src/assets/css/*.{css}",
  ],
  theme: {
    extend: {
      colors: {
        // Primary colors
        primary: {
          DEFAULT: "#CB5520",
          light: "#E87A3A",
          lighter: "#FF985C",
        },
        secondary: {
          DEFAULT: "#F4C9A6", // Updated per CLAUDE.md
          light: "#FFF8F3",
        },
        // Background colors
        background: {
          DEFAULT: "#FFFFFF",
          secondary: "#FFF8F3",
        },
        // Text colors
        text: {
          DEFAULT: "#2E2E2E",
          primary: "#2E2E2E",
          secondary: "#7D7D7D",
          button: "#FFFFFF",
        },
        // Status colors
        error: "#E74C3C",
        success: "#27AE60",
        info: "#3498DB",
      },
      boxShadow: {
        // Subtle shadows for depth (as per CLAUDE.md guidelines)
        card: "0 2px 8px rgba(0, 0, 0, 0.08)",
        "card-hover": "0 4px 16px rgba(0, 0, 0, 0.12)",
        button: "0 2px 6px rgba(203, 85, 32, 0.2)",
        "button-hover": "0 4px 12px rgba(203, 85, 32, 0.3)",
        glow: "0 0 20px rgba(203, 85, 32, 0.3)",
        "inner-light": "inset 0 1px 2px rgba(255, 255, 255, 0.1)",
      },
      backgroundImage: {
        "gradient-primary": "linear-gradient(135deg, #FF985C 0%, #E87A3A 100%)",
        "gradient-primary-hover":
          "linear-gradient(135deg, #FF985C 0%, #CB5520 100%)",
      },
    },
  },
  plugins: [],
};
