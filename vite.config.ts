import { defineConfig } from "vitest/config";
import solidPlugin from "vite-plugin-solid";
// Import Tailwind CSS
import tailwindcss from 'tailwindcss';

export default defineConfig({
  esbuild: {
    target: 'esnext',
  },
  plugins: [
    solidPlugin(),
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: ["es2021", "chrome100", "safari13"],
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  define: {
    'import.meta.vitest': 'undefined',
  },
  test: {
    includeSource: ['src/**/*.{js,ts,tsx}']
  },
  css: {
    postcss: {
      plugins: [
        tailwindcss,
        // Add any other PostCSS plugins here if needed
      ],
    },
  },
});
