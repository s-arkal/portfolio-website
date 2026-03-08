/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  // dark mode
  darkMode: ['class', '[data-theme="dark"]'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['"Inter"', 'system-ui', '-apple-system', 'sans-serif'],
        mono: ['"JetBrains Mono"', 'monospace'],
      },
      colors: {
        // map tailwind classes
        bg: 'var(--bg)',
        panel: 'var(--panel)',
        txt: 'var(--text)',
        accent: 'var(--accent)',
        muted: 'var(--muted)',
        bdr: 'var(--border)',
        hover: 'var(--hover)',
      },
    },
  },
  plugins: [],
}
