/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './src/*.{vue,js,ts,jsx,tsx}',
    './src/*/**.{vue,js,ts,jsx,tsx}',
    './src/assets/css/*.{css}'
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}

