import { defineConfig } from "vite";

export default defineConfig({
    test: {
        reporters: ['json', 'json-summary', 'text'],
        outputFile: './dist/test-output.json'
    }
});
