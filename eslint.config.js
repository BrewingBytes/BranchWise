import js from '@eslint/js'
import eslintPluginVue from 'eslint-plugin-vue'
import vueTsEslintConfig from "@vue/eslint-config-typescript";
import ts from 'typescript-eslint'

export default ts.config(
  js.configs.recommended,
  ...ts.configs.recommended,
  ...eslintPluginVue.configs['flat/recommended'],
  ...vueTsEslintConfig({
    extends: [
      "recommended",
      "stylistic",
    ]
  },
  ),
  {
    files: ['*.vue', '**/*.vue', '*.ts', '**/*.ts'],
    languageOptions: {
      parserOptions: {
        parser: '@typescript-eslint/parser'
      }
    },
    rules: {
      "comma-dangle": [
        "error",
        "only-multiline"
      ],
      "quotes": [
        "error",
        "double"
      ],
      "semi": [
        "error",
        "always"
      ],
      "no-useless-constructor": "off",
      "@typescript-eslint/no-useless-constructor": [
        "error"
      ],
      "no-tabs": "off",
      "indent": [
        2,
        "tab",
        {
          "SwitchCase": 1
        }
      ],
      "vue/html-indent": "off",
      "vue/multi-word-component-names": "off"
    }
  }
)
