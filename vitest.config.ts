import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    include: ['./test/tests/**/*.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    coverage: {
      statements: 93.02,
      branches: 79.24,
      functions: 91.66,
      lines: 93.02,
      exclude: ['test', 'src/index.ts'],
    },
  },
});
