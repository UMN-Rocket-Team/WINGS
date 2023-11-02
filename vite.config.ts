import { defineConfig } from "vitest/config";
import solidPlugin from "vite-plugin-solid";
import Unocss from 'unocss/vite';

export default defineConfig({
  plugins: [
    solidPlugin(),
    Unocss({ 
      rules: [
        [/^drop-shadow-(\w+)$/, match => ({ "box-shadow": `0px 0px 0.5rem 0.25rem ${match[1]}` })],
      ],
      shortcuts: {
        //defines style for different elements across the app
        externalButton: "bg-transparent dark:border-gray-4 dark:text-white border-1 border-rounded hover:bg-blue-600 hover:text-white p-1 hover:border-transparent",
        homePageButton: "flex items-center gap-2 bg-blue-600 hover:bg-blue-700 text-white py-2 px-8 border-transparent border-rounded",
        redButton: "bg-red border-rounded border-0 px-4 py-2",
        tab: "border-1 p-2 border-rounded gap-2 dark:text-gray-100 dark:border-gray-4",
        inputBox: "dark:bg-dark-700 dark:border-gray-4 dark:text-white border-1 border-rounded p-1",
        listButton: "border-transparent bg-transparent hover:bg-gray hover:bg-blue-600 hover:text-white",
        //widget shortcuts are for the boxes in the packets tab that are used to show packets and packets structures
        widgetSelected:"border-transparent bg-blue-600 text-white",
        widgetNotSelected:"bg-transparent border-black dark:border-gray-4",
        widgetGeneral:"border-rounded border-1 p-2 dark:text-white",
      }
    }),
  ],

  // Vite optiosns tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // Tauri supports es2021
    target: ["es2021", "chrome100", "safari13"],
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
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
