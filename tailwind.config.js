/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{rs}"],
  theme: {
    screens: {
      lg: "1080px",
      xl: "2560px",
      "2xl": "3440px",
    },
    extend: {},
  },
};
