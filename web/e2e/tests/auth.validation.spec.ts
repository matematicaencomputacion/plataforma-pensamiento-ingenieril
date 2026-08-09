import { expect, test } from "@playwright/test";

/**
 * Scaffold: form validation UX for auth screens.
 * Enable when assertions stabilize against Leptos error rendering.
 */
test.describe.skip("auth validation (scaffold)", () => {
  test("login shows alert on invalid credentials", async ({ page }) => {
    await page.goto("/login");
    await page.locator("#login-email").fill("nobody@example.com");
    await page.locator("#login-password").fill("wrong-password-xx");
    await page.getByRole("button", { name: "Entrar" }).click();
    await expect(page.getByRole("alert")).toBeVisible();
    await expect(page).not.toHaveURL(/\/workspace/);
  });

  test("register rejects too-short password client-side or via API", async ({
    page,
  }) => {
    await page.goto("/register");
    await page.locator("#register-email").fill("short-pass@example.com");
    await page.locator("#register-password").fill("short");
    await page.getByRole("button", { name: "Crear cuenta" }).click();
    // Either HTML5 minlength blocks submit or API returns 400 alert.
    const stillOnRegister = page.url().includes("/register");
    const alertVisible = await page.getByRole("alert").isVisible().catch(() => false);
    expect(stillOnRegister || alertVisible).toBeTruthy();
  });
});
