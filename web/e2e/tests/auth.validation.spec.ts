import { expect, test } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  gotoApp,
  waitForAuthFormReady,
} from "./helpers";

/**
 * Auth form validation UX (ADR 003) + Rebanada 2 network/a11y.
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

  test("login keyboard focus reaches submit; offline API unlocks busy", async ({
    page,
  }) => {
    await gotoApp(page, "/login");
    await waitForAuthFormReady(page, {
      emailSelector: "#login-email",
      passwordSelector: "#login-password",
      submitName: "Entrar",
    });

    await page.locator("#login-email").focus();
    await page.keyboard.press("Tab");
    await expect(page.locator("#login-password")).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(page.getByRole("button", { name: "Entrar" })).toBeFocused();

    await page.route("**/api/auth/login", async (route) => {
      await route.abort("failed");
    });

    await fillLeptosInput(page, "#login-email", "offline@example.com");
    await fillLeptosInput(page, "#login-password", "secreto12xx");
    await page.getByRole("button", { name: "Entrar" }).click();

    const alert = page.getByRole("alert");
    await expect(alert).toBeVisible({ timeout: e2eTimeout });
    await expect(alert).toContainText(/conectar con el servidor/i);
    await expect(page.getByRole("button", { name: "Entrar" })).toBeEnabled({
      timeout: e2eTimeout,
    });
    await expect(page).not.toHaveURL(/\/workspace/);
  });
});
