import { expect, test } from "@playwright/test";
import { fillLeptosInput, gotoApp } from "./helpers";

/**
 * Smoke: email/password auth against the Leptos shell → Go API.
 *
 * Not Google OAuth — PPI auth is local (POST /api/auth/login|register).
 *
 * Required env (never commit secrets):
 *   PPI_E2E_EMAIL
 *   PPI_E2E_PASSWORD
 *
 * Optional:
 *   PPI_E2E_MODE=login|register   (default: login)
 *   PPI_E2E_BASE_URL=http://127.0.0.1:3001
 *   PPI_E2E_HEADED=1
 */

function requireCreds(): { email: string; password: string } {
  const email = process.env.PPI_E2E_EMAIL?.trim() ?? "";
  const password = process.env.PPI_E2E_PASSWORD ?? "";
  if (!email || !password) {
    test.skip(
      true,
      "Set PPI_E2E_EMAIL and PPI_E2E_PASSWORD (e.g. in .env.local — gitignored)",
    );
    throw new Error("unreachable: missing PPI_E2E credentials");
  }
  if (password.length < 8) {
    test.skip(true, "PPI_E2E_PASSWORD must be at least 8 characters (API rule)");
    throw new Error("unreachable: short PPI_E2E_PASSWORD");
  }
  return { email, password };
}

test.describe("auth smoke (email/password)", () => {
  test("reaches workspace after login or register", async ({ page }) => {
    const { email, password } = requireCreds();
    const mode = (process.env.PPI_E2E_MODE ?? "login").toLowerCase();

    if (mode === "register") {
      await gotoApp(page, "/register");
      await expect(page.getByRole("heading", { name: "Crear cuenta" })).toBeVisible();
      await fillLeptosInput(page, "#register-email", email);
      await fillLeptosInput(page, "#register-password", password);
      await page.getByRole("button", { name: "Crear cuenta" }).click();
    } else {
      await gotoApp(page, "/login");
      await expect(page.getByRole("heading", { name: "Iniciar sesión" })).toBeVisible();
      await fillLeptosInput(page, "#login-email", email);
      await fillLeptosInput(page, "#login-password", password);
      await page.getByRole("button", { name: "Entrar" }).click();
    }

    // Success → workspace; failure stays on auth with alert.
    await Promise.race([
      page.waitForURL(/\/workspace/, { timeout: 20_000 }),
      page
        .getByRole("alert")
        .waitFor({ state: "visible", timeout: 20_000 })
        .then(async () => {
          const msg = (await page.getByRole("alert").textContent())?.trim() ?? "auth failed";
          throw new Error(`Auth did not reach /workspace: ${msg}`);
        }),
    ]);

    await expect(page).toHaveURL(/\/workspace/);
    await expect(page.getByRole("heading", { name: "Workspace" })).toBeVisible();
    await expect(page.locator(".session-bar__email")).toContainText(email);
  });

  test("landing exposes login and register CTAs", async ({ page }) => {
    await gotoApp(page, "/");
    await expect(page.getByRole("heading", { name: "IngenierIA" })).toBeVisible();
    await expect(page.getByRole("link", { name: "Iniciar sesión" }).first()).toBeVisible();
    await expect(page.getByRole("link", { name: "Crear cuenta" }).first()).toBeVisible();
  });
});
