/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        "code": "Consolas, ui-monospace, SFMono-Regular, Menlo, Monaco, monospace",
      }
    },
  },
  plugins: [],
}

