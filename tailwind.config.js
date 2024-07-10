const plugin = require("tailwindcss/plugin");

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{svelte,html,js,ts}"],
  darkMode: "media", // or 'media' or 'class'
  daisyui: {
    themes: ["light", "dark"],
    darkTheme: "dark",
  },
  theme: {
    extend: {
      gridTemplateAreas: {
        // "assembly-part": ["select assy-no part-name"],
        // "assembly-no-detail": ["base dash1 model dash2 revision dash3 status"],
        // "assembly-parts": ["part-no latest-revision part-name bl sec item qty hns"],
        // "mto-app": ["model type options"],
        // "mto-app-detail": ["app0 app1 app2 app3 app4 app5 app6 app7 app8 app9"],
      },
      gridTemplateColumns: {
        // "assembly-part": "auto minmax(0, 1fr) minmax(0, 1fr)",
        // "assembly-no-detail": "minmax(0, 1fr) auto minmax(0, 1fr) auto minmax(0, 1fr) auto minmax(0, 1fr)",
        // "assembly-parts":
        //   "minmax(11rem,1fr) minmax(7rem,auto) minmax(14.25rem,1fr) minmax(1.5rem,auto) minmax(3.5rem,auto) minmax(4.5rem,auto) minmax(2.5rem,auto) minmax(2.25rem,auto)",
        // "mto-app": "minmax(3.5rem,1fr) minmax(3.5rem,1fr) minmax(20rem,1fr)",
        // "mto-app-detail":
        //   "minmax(2rem,auto) minmax(2rem,auto) minmax(2rem,auto) minmax(2rem,auto) minmax(2rem,auto) minmax(2rem,auto) minmax(2rem,auto) minmax(2rem,auto) minmax(2rem,auto) minmax(2rem,auto)",
      },
      fontFamily: {
        calculator: ["Calculator"],
        segoe: ["Segoe UI", "sans-serif"],
      },
    },
    fontFamily: {
      sans: ["Segoe UI Emoji", "Segoe UI Symbol", "Segoe UI", "ui-sans-serif", "system-ui", "sans-serif"],
    },
  },
  plugins: [
    require("@tailwindcss/typography"),
    require("@savvywombat/tailwindcss-grid-areas"),
    require("daisyui"),
    plugin(function ({ matchUtilities, theme }) {
      matchUtilities(
        {
          "animation-delay": (value) => ({
            "animation-delay": value,
          }),
        },
        {
          values: theme("animationDelay"),
        }
      );
    }),
  ],
};
