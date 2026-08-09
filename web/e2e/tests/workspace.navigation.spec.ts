import { expect, test } from "@playwright/test";

/**
 * Scaffold: post-auth workspace chrome (session bar / logout).
 * Requires PPI_E2E_EMAIL + PPI_E2E_PASSWORD (or register mode).
 */
test.describe.skip("workspace navigation (scaffold)", () => {
  test.beforeEach(async ({ page }) => {
    const email = process.env.PPI_E2E_EMAIL;
    const password = process.env.PPI_E2E_PASSWORD;
    test.skip(!email || !password, "PPI_E2E_EMAIL/PASSWORD required");

    await page.goto("/login");
    await page.locator("#login-email").fill(email!);
    await page.locator("#login-password").fill(password!);
    await page.getByRole("button", { name: "Entrar" }).click();
    await page.waitForURL(/\/workspace/);
  });

  test("shows session email and can logout to landing", async ({ page }) => {
    const email = process.env.PPI_E2E_EMAIL!;
    await expect(page.locator(".session-bar__email")).toContainText(email);
    await page.getByRole("button", { name: "Salir" }).click();
    await page.waitForURL(/\/$/);
    await expect(page.getByRole("heading", { name: "IngenierIA" })).toBeVisible();
  });

  test("unauthenticated /workspace redirects to login", async ({ page }) => {
    await page.evaluate(() => localStorage.clear());
    await page.goto("/workspace");
    await page.waitForURL(/\/login/);
  });
});
