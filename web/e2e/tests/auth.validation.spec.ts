import { expect, test } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  gotoApp,
  waitForAuthFormReady,
} from "./helpers";

/**
 * Auth form validation UX (ADR 003).
 */
test.describe("auth validation", () => {
  test("login shows alert on invalid credentials", async ({ page }) => {
    await gotoApp(page, "/login");
    await waitForAuthFormReady(page, {
      emailSelector: "#login-email",
      passwordSelector: "#login-password",
      submitName: "Entrar",
    });
    await fillLeptosInput(page, "#login-email", "nobody@example.com");
    await fillLeptosInput(page, "#login-password", "wrong-password-xx");
    await page.getByRole("button", { name: "Entrar" }).click();
    await expect(page.getByRole("alert")).toBeVisible({ timeout: e2eTimeout });
    await expect(page).not.toHaveURL(/\/workspace/);
  });

  test("register rejects too-short password client-side or via API", async ({
    page,
  }) => {
    await gotoApp(page, "/register");
    await waitForAuthFormReady(page, {
      emailSelector: "#register-email",
      passwordSelector: "#register-password",
      submitName: "Crear cuenta",
    });
    await fillLeptosInput(page, "#register-email", `short-${Date.now()}@example.com`);
    await fillLeptosInput(page, "#register-password", "short");
    await page.getByRole("button", { name: "Crear cuenta" }).click();
    // HTML5 minlength may block submit, or API/Leptos shows an alert.
    await page.waitForTimeout(500);
    const stillOnRegister = page.url().includes("/register");
    const alertVisible = await page.getByRole("alert").isVisible().catch(() => false);
    expect(stillOnRegister || alertVisible).toBeTruthy();
    await expect(page).not.toHaveURL(/\/workspace/);
  });
});
