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
          "bg-light": "#2c313c",
          "highlight": "#3e4451",
          "text": "#abb2bf",
          "text-dark": "#58727d",
          "text-light": "#4b5364",
          "primary": "#61afef",
          "highlight-None": "#abb2bf",
          "highlight-White": "#abb2bf",
          "highlight-Red": "#e06c75",
          "highlight-Orange": "#d19a66",
          "highlight-Blue": "#5faae8",
          "highlight-Green": "#98c379",
          "highlight-Purple": "#c678dd",
          "highlight-Yellow": "#e5c07b",
          "highlight-Gray": "#5c6370",
          "highlight-Turquoise": "#56b6c2",
        }
      },
      keyframes: {
        blink: {
          '0%, 100%': { opacity: 1 },
          '50%': { opacity: 0 },
        }
      },
      animation: {
        blink: "blink 1s infinite"
      }
    },
  },
  plugins: [],
}

