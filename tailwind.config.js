const {fontFamily} = require('tailwindcss/defaultTheme');
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./templates/*.html'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['MonaspaceKrypton', ...fontFamily.sans]
      }
    },
  },
  plugins: [],
}

