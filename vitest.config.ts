import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    include: ['./test/tests/**/*.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    coverage: {
      statements: 90,
      branches: 79.24,
      functions: 87,
      lines: 90,
      exclude: ['test', 'src/index.ts'],
    },
  },
});
