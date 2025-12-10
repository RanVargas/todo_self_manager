/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  presets: [],
  darkMode: "media", // or 'class'
  theme: {
    accentColor: ({ theme }) => ({
      ...theme("colors"),
      auto: "auto",
    }),
  },
};
