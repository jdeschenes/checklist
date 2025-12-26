import eslint from "@eslint/js";
import tseslint from "typescript-eslint";
import reactLint from "eslint-plugin-react";
import eslintConfigPrettier from "eslint-config-prettier";
import pluginQuery from "@tanstack/eslint-plugin-query";
import { fileURLToPath } from "url";
import { dirname } from "path";

const ignores = [
  "dist",
  "eslint.config.mjs",
  "prettier.config.mjs",
  "vite.config.ts",
];
const tsFiles = ["**/*.{ts,tsx}"];

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export default tseslint.config(
  { ignores },
  eslint.configs.recommended,
  reactLint.configs.flat.recommended,
  reactLint.configs.flat["jsx-runtime"],
  {
    settings: {
      react: {
        version: "detect",
      },
    },
  },
  ...tseslint.configs.recommendedTypeChecked.map((config) => ({
    ...config,
    files: tsFiles,
  })),
  {
    files: tsFiles,
    languageOptions: {
      parserOptions: {
        projectService: true,
        tsconfigRootDir: __dirname,
      },
    },
    rules: {
      "react/prop-types": "off",
    },
  },
  ...pluginQuery.configs["flat/recommended"],
  eslintConfigPrettier
);
