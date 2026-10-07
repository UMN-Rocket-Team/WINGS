import { defineConfig } from "eslint/config";
import globals from "globals";
import js from "@eslint/js";
import tseslint from "typescript-eslint";
import plugin from 'eslint-plugin-solid';
import jsxA11y from 'eslint-plugin-jsx-a11y';
import eslintConfigPrettier from "eslint-config-prettier/flat";


export default defineConfig([
  { files: ["**/*.{js,mjs,cjs,ts}"] },
  { files: ["**/*.{js,mjs,cjs,ts}"], languageOptions: { globals: globals.browser } },
  { files: ["**/*.{js,mjs,cjs,ts}"], plugins: { js }, extends: ["js/recommended"] },
  tseslint.configs.recommended,
  plugin.configs['flat/typescript'],
  {
    files: ['**/*.{js,mjs,cjs,jsx,mjsx,ts,tsx,mtsx}'],
    ...jsxA11y.flatConfigs.recommended
  },
  eslintConfigPrettier,
  
]);