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
      },
      colors: {
        "atom": {
          "bg-dark": "#21252b",
          "bg": "#282c34",
          "text": "#abb2bf",
          "text-dark": "#58727d",
          "primary": "#61afef",
        }
      }
    },
  },
  plugins: [],
}

