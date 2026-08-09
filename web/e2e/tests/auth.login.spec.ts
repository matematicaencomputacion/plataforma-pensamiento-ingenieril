import { expect, test } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  gotoApp,
  waitForAuthFormReady,
} from "./helpers";

/**
 * Smoke: email/password auth against the Leptos shell → Go API.
 *
 * Seed the account via HTTP (avoids CSR submit races on cold Wasm in CI),
 * then exercise the real login form in the browser with explicit hydrate waits.
 *
 * Optional overrides:
 *   PPI_E2E_EMAIL / PPI_E2E_PASSWORD — fixed credentials (must not already exist
 *   as a bad password; unique-by-default is safer in CI)
 *   PPI_E2E_BASE_URL=http://127.0.0.1:3001
 */

function uniqueCreds(): { email: string; password: string } {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  const email =
    process.env.PPI_E2E_EMAIL?.trim() ||
    `e2e-${Date.now()}-${Math.floor(Math.random() * 1e6)}@example.com`;
  if (password.length < 8) {
    test.skip(true, "PPI_E2E_PASSWORD must be at least 8 characters (API rule)");
    throw new Error("unreachable: short PPI_E2E_PASSWORD");
  }
  return { email, password };
}

test.describe("auth smoke (email/password)", () => {
  test("reaches workspace after login", async ({ page, request }) => {
    const { email, password } = uniqueCreds();

    // Unique email every attempt (including CI retries) so 409 cannot flake.
    const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
    const accountEmail = email.includes("@")
      ? email.replace(/@/, `+${stamp}@`)
      : `e2e-${stamp}@example.com`;

    const reg = await request.post("/api/auth/register", {
      data: { email: accountEmail, password },
      timeout: e2eTimeout,
    });
    expect(
      reg.ok(),
      `register seed failed: ${reg.status()} ${await reg.text()}`,
    ).toBeTruthy();

    await gotoApp(page, "/login");
    await expect(page.getByRole("heading", { name: "Iniciar sesión" })).toBeVisible({
      timeout: e2eTimeout,
    });
    await waitForAuthFormReady(page, {
      emailSelector: "#login-email",
      passwordSelector: "#login-password",
      submitName: "Entrar",
    });

    await fillLeptosInput(page, "#login-email", accountEmail);
    await fillLeptosInput(page, "#login-password", password);

    const submit = page.getByRole("button", { name: "Entrar" });
    await expect(submit).toBeEnabled({ timeout: e2eTimeout });

    const loginResponse = page.waitForResponse(
      (res) =>
        res.url().includes("/api/auth/login") &&
        res.request().method() === "POST",
      { timeout: e2eTimeout },
    );

    await submit.click();

    const loginRes = await loginResponse;
    if (!loginRes.ok()) {
      const body = await loginRes.text().catch(() => "");
      throw new Error(
        `login API ${loginRes.status()}: ${body || "(empty body)"}`,
      );
    }

    await Promise.race([
      page.waitForURL(/\/workspace/, { timeout: e2eTimeout }),
      page
        .getByRole("alert")
        .waitFor({ state: "visible", timeout: e2eTimeout })
        .then(async () => {
          const msg =
            (await page.getByRole("alert").textContent())?.trim() ?? "auth failed";
          throw new Error(`Auth did not reach /workspace: ${msg}`);
        }),
    ]);

    await expect(page).toHaveURL(/\/workspace/);
    await expect(page.getByRole("heading", { name: "Workspace" })).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator(".session-bar__email")).toContainText(accountEmail, {
      timeout: e2eTimeout,
    });
  });

  test("landing exposes login and register CTAs", async ({ page }) => {
    await gotoApp(page, "/");
    await expect(page.getByRole("heading", { name: "IngenierIA" })).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.getByRole("link", { name: "Iniciar sesión" }).first()).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.getByRole("link", { name: "Crear cuenta" }).first()).toBeVisible({
      timeout: e2eTimeout,
    });
  });
});
