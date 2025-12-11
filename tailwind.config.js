/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
      "./src/**/*.rs",
      "./src/**/*.{rs,html}",   // includes components
      "./src/**/**/*.rs",       // ensures deeper dirs
      "./index.html"
  ],
  presets: [],
  darkMode: "media", // or 'class'
  theme: {
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ]
};
