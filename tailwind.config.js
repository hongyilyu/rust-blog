const colors = require('tailwindcss/colors')
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs", "./preline/*.js", "./posts/**/*.md"],
  darkMode: "class",
  theme: {
    extend: {
      opacity: ['hover'],
      colors: {
        gray: colors.slate,
        bgDark: '#0a101d',
        bgLight: 'white'
      }
    }
  },
  plugins: [require("@tailwindcss/typography")],
}
