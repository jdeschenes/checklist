import eslint from "@eslint/js";
import tseslint from "typescript-eslint";
import reactLint from "eslint-plugin-react";
import eslintConfigPrettier from "eslint-config-prettier";
import pluginQuery from "@tanstack/eslint-plugin-query";
import { fileURLToPath } from "url";
import { dirname } from "path";

const ignores = ["dist", "prettier.config.mjs", "vite.config.ts"];

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export default tseslint.config(
  { ignores },
  eslint.configs.recommended,
  reactLint.configs.flat.recommended,
  reactLint.configs.flat["jsx-runtime"],
  ...tseslint.configs.recommendedTypeChecked,
  {
    languageOptions: {
      parserOptions: {
        projectService: true,
        tsconfigRootDir: __dirname,
      },
    },
    settings: {
      react: {
        version: "detect",
      },
    },
  },
  eslintConfigPrettier,
  ...pluginQuery.configs["flat/recommended"]
);
