module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs",
    // optionally your tests and other crates
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Open Sans", "sans-serif", "Ubuntu"],
      },
    },
  },
  plugins: [],
};
