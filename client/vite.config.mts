import { defineConfig } from "vitest/config";
import devtools from 'solid-devtools/vite';
import solidPlugin from "vite-plugin-solid";
// Import Tailwind CSS
import tailwindcss from '@tailwindcss/vite';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [
    devtools({
      /* features options - all disabled by default */
      autoname: true, // e.g. enable autoname
    }),
    solidPlugin(),
    tailwindcss()
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
        protocol: 'ws',
        host,
        port: 1421
      }
      : undefined,
      watch: {
        ignored: ['**/src-tauri/**']
      }
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_ENV_PLATFORM == 'windows'
      ? 'chrome105'
      : 'safari13',
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "oxc" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  define: {
    'import.meta.vitest': 'undefined',
  },
  test: {
    includeSource: ['src/**/*.{js,ts,tsx}']
  },
});
