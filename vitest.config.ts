import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		include: ["./test/tests/**/*.{js,mjs,cjs,ts,mts,cts,jsx,tsx}"],
		coverage: {
			statements: 89,
			branches: 77,
			functions: 87,
			lines: 89,
			exclude: ["test", "src/index.ts"],
		},
	},
});
