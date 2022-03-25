// See the Tailwind configuration guide for advanced usage
// https://tailwindcss.com/docs/configuration
module.exports = {
  content: [
    './src/lib.rs'
  ],
  theme: {
    extend: {
      fontFamily: {
        'serif': ['Vollkorn', 'Georgia', 'Tahoma', 'serif'],
        'sans': ['Inter', 'Helvetica', 'Arial', 'sans-serif']
      },
      // Color scheme from: https://colorhunt.co/palette/eeebddce12128100001b1717
      colors: theme => ({
        "su-bg": "#EEEBDD",
        "su-bg-alt": "#DFDCCB",
        "su-fg": "#1B1717",
        "su-accent-1": "#CE1212",
        "su-accent-2": "#810000",
        "su-dark-bg": "#1B1717",
        "su-dark-bg-alt": "#2e2727",
        "su-dark-fg": "#EEEBDD",
        "su-dark-accent-1": "#CE1212",
        "su-dark-accent-2": "#810000"
      }),
    },
  },
  plugins: [
//    require('@tailwindcss/forms'),
//    require('@tailwindcss/aspect-ratio')
  ]
}
