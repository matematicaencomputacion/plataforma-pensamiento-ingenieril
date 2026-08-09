import { defineConfig, devices } from "@playwright/test";

/**
 * Leptos CSR shell (Trunk). Default: http://127.0.0.1:3001
 * Override with PPI_E2E_BASE_URL.
 *
 * Prerequisites:
 *   - Go API on :8080 (`make run`)
 *   - Trunk UI on :3001 (`cd web && env -u NO_COLOR trunk serve --port 3001`)
 */
const baseURL = process.env.PPI_E2E_BASE_URL ?? "http://127.0.0.1:3001";
const headed = process.env.PPI_E2E_HEADED === "1" || process.env.HEADED === "1";

export default defineConfig({
  testDir: "./tests",
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  workers: 1,
  // Cold Wasm hydrate + SQLite round-trips are slower on GHA runners.
  timeout: process.env.CI ? 90_000 : 60_000,
  expect: { timeout: process.env.CI ? 30_000 : 15_000 },
  reporter: process.env.CI ? [["github"], ["list"]] : "list",
  use: {
    baseURL,
    trace: "on-first-retry",
    screenshot: "only-on-failure",
    video: "retain-on-failure",
    headless: !headed,
    ...devices["Desktop Chrome"],
  },
  projects: [{ name: "chromium", use: { ...devices["Desktop Chrome"] } }],
});
