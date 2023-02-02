import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    include: ['./test/tests/**/*.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    coverage: {
      statements: 91,
      branches: 78,
      functions: 87,
      lines: 91,
      exclude: ['test', 'src/index.ts'],
    },
  },
});
