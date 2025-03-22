import eslint from '@eslint/js'
import tseslint from 'typescript-eslint'
import reactLint from 'eslint-plugin-react'
import eslintConfigPrettier from 'eslint-config-prettier'
import pluginQuery from '@tanstack/eslint-plugin-query'
import { fileURLToPath } from 'url'
import { dirname } from 'path'

const ignores = ['**/*.js', 'prettier.config.mjs', 'vite.config.ts']
const reactConfig = reactLint.configs.flat.recommended

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

reactConfig.ignores = ignores
reactConfig.settings = {
    react: {
        version: 'detect',
    },
}

export default tseslint.config(
    {
        ...eslint.configs.recommended,
        ignores,
    },
    reactConfig,
    reactLint.configs.flat['jsx-runtime'],

    ...tseslint.configs.recommendedTypeChecked.map((config) => ({
        ...config,
        ignores,
    })),
    {
        languageOptions: {
            parserOptions: {
                projectService: true,
                tsconfigRootDir: __dirname,
            },
        },
    },
    {
        rules: {
            'react/jsx-filename-extension': [
                1,
                {
                    extensions: ['.tsx'],
                },
            ],
        },
    },
    eslintConfigPrettier,
    ...pluginQuery.configs['flat/recommended']
)
