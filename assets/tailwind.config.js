// See the Tailwind configuration guide for advanced usage
// https://tailwindcss.com/docs/configuration
module.exports = {
  content: [
    './**/*.rs',
    './public/**/*.html',
    './assets/**/*.ts'
  ],
  theme: {
    extend: {
      fontFamily: {
        'serif': ['Vollkorn', 'Georgia', 'Tahoma', 'serif'],
        'sans': ['Inter', 'Helvetica', 'Arial', 'sans-serif']
      },
      // Color scheme from: https://colorhunt.co/palette/eeebddce12128100001b1717
      colors: theme => ({
        "su-bg-1": "#EEEBDD",
        "su-bg-2": "#DFDCCB",
        "su-fg-1": "#1B1717",
        "su-accent-1": "#CE1212",
        "su-accent-2": "#810000",
        "su-dark-bg-1": "#1B1717",
        "su-dark-bg-2": "#2e2727",
        "su-dark-fg-1": "#EEEBDD",
        "su-dark-accent-1": "#CE1212",
        "su-dark-accent-2": "#810000"
      }),
    },
  },
  plugins: [
  ]
}
