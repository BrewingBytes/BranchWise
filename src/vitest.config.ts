import { defineConfig } from "vite";

export default defineConfig({
    test: {
        reporters: ['json'],
        outputFile: './dist/test-output.json'
    }
});
