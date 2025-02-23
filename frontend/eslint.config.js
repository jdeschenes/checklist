const eslint = require('@eslint/js')
const tseslint = require('typescript-eslint')
const reactLint = require('eslint-plugin-react')
const eslintConfigPrettier = require('eslint-config-prettier')

const ignores = ['**/*.js', 'prettier.config.mjs']
const reactConfig = reactLint.configs.flat.all

reactConfig.ignores = ignores
reactConfig.settings = {
    react: {
        version: 'detect',
    },
}

module.exports = tseslint.config(
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
    eslintConfigPrettier
)
