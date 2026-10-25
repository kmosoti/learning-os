import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir: '.', testMatch: '*.spec.mjs', timeout: 45_000, fullyParallel: false,
  outputDir: '../../artifacts/v2/browser', reporter: [['list'], ['json', { outputFile: '../../artifacts/v2/browser-results.json' }]],
  use: { baseURL: process.env.LEARNINGOS_TEST_URL || 'http://127.0.0.1:3000', viewport: { width: 1440, height: 900 }, trace: 'retain-on-failure' },
  projects: [
    { name: 'chromium', use: { browserName: 'chromium', launchOptions: { executablePath: process.env.PW_CHROMIUM_EXECUTABLE_PATH } } },
    { name: 'firefox', use: { browserName: 'firefox' } },
  ],
  webServer: process.env.LEARNINGOS_TEST_URL ? undefined : { command: 'LEARNINGOS_CATALOG=/tmp/learningos-ci-v2.sqlite3 LEARNINGOS_ASSETS=../../apps/web/public ../../target/release/learningos-server', url: 'http://127.0.0.1:3000', reuseExistingServer: !process.env.CI, timeout: 15_000 },
});
