import { expect, test } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  gotoApp,
  waitForAuthFormReady,
} from "./helpers";

/**
 * Canonical auth + hub journey (ADR 003 / docs/testing/journeys.md).
 *
 * Pages oil-check:
 *   P1 `/` → P2a `/register` → P3 `/workspace`
 *   P3 ↔ P1 (authenticated hub)
 *   P2b `/login` after logout
 *   Recovery P2c → P2d → P3
 */

function stampEmail(prefix: string): string {
  return `${prefix}-${Date.now()}-${Math.floor(Math.random() * 1e6)}@example.com`;
}

test.describe("journey auth + hub (P1→P3)", () => {
  test("register landing workspace portada loop and login again", async ({
    page,
    request,
  }) => {
    const email = stampEmail("journey");
    const password = "secreto12journey";

    // --- P1 anonymous ---
    await gotoApp(page, "/");
    await expect(page.getByRole("heading", { name: "IngenierIA" })).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.getByRole("link", { name: "Crear cuenta" }).first()).toBeVisible();
    await page.getByRole("link", { name: "Crear cuenta" }).first().click();

    // --- P2a register ---
    await expect(page).toHaveURL(/\/register/);
    await waitForAuthFormReady(page, {
      emailSelector: "#register-email",
      passwordSelector: "#register-password",
      submitName: "Crear cuenta",
    });
    await fillLeptosInput(page, "#register-email", email);
    await fillLeptosInput(page, "#register-password", password);

    const regResponse = page.waitForResponse(
      (res) =>
        res.url().includes("/api/auth/register") &&
        res.request().method() === "POST",
      { timeout: e2eTimeout },
    );
    await page.getByRole("button", { name: "Crear cuenta" }).click();
    expect((await regResponse).ok()).toBeTruthy();

    // --- P3 workspace hub ---
    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
    await expect(page.getByRole("heading", { name: "Workspace" })).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator(".session-bar__email")).toContainText(email);
    await expect(
      page.getByRole("heading", { name: "Current level micro-step" }),
    ).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator("#workspace-microsteps li")).toHaveCount(480);
    await expect(page.getByRole("link", { name: "Portada" }).first()).toBeVisible();
    await expect(page.getByRole("link", { name: "Workspace" }).first()).toBeVisible();

    // --- Hub journey P3 → P1 authenticated ---
    await page.getByRole("link", { name: "Portada" }).first().click();
    await expect(page).toHaveURL((url) => new URL(url).pathname === "/");
    await expect(page.getByRole("heading", { name: "IngenierIA" })).toBeVisible();
    await expect(page.getByRole("link", { name: "Ir al workspace" })).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator(".session-bar__email")).toContainText(email);

    await page.getByRole("link", { name: "Ir al workspace" }).click();
    await expect(page).toHaveURL(/\/workspace/);
    await expect(page.getByRole("heading", { name: "Workspace" })).toBeVisible();

    // --- Logout → P1 anonymous ---
    await page.getByRole("button", { name: "Salir" }).click();
    await expect(page.getByRole("link", { name: "Iniciar sesión" }).first()).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator(".session-bar__email")).toHaveCount(0);

    // --- P2b login again ---
    await page.getByRole("link", { name: "Iniciar sesión" }).first().click();
    await expect(page).toHaveURL(/\/login/);
    await waitForAuthFormReady(page, {
      emailSelector: "#login-email",
      passwordSelector: "#login-password",
      submitName: "Entrar",
    });
    await fillLeptosInput(page, "#login-email", email);
    await fillLeptosInput(page, "#login-password", password);
    const loginResponse = page.waitForResponse(
      (res) =>
        res.url().includes("/api/auth/login") && res.request().method() === "POST",
      { timeout: e2eTimeout },
    );
    await page.getByRole("button", { name: "Entrar" }).click();
    expect((await loginResponse).ok()).toBeTruthy();
    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
    await expect(page.locator(".session-bar__email")).toContainText(email);

    // API still sees the account (sanity vs dual-DB traps).
    const meLogin = await request.post("/api/auth/login", {
      data: { email, password },
    });
    expect(meLogin.ok()).toBeTruthy();
  });

  test("recovery pages P2c→P2d land in workspace", async ({ page, request }) => {
    const email = stampEmail("journey-reset");
    const oldPassword = "secreto12oldxx";
    const newPassword = "secreto12newxx";

    const reg = await request.post("/api/auth/register", {
      data: { email, password: oldPassword },
      timeout: e2eTimeout,
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();

    await gotoApp(page, "/login");
    await page.getByRole("link", { name: "Olvidé mi contraseña" }).click();
    await expect(page).toHaveURL(/\/forgot-password/);
    await expect(page.getByRole("heading", { name: "Recuperar contraseña" })).toBeVisible();

    await fillLeptosInput(page, "#forgot-email", email);
    const forgotUi = page.waitForResponse(
      (res) =>
        res.url().includes("/api/auth/forgot-password") &&
        res.request().method() === "POST",
      { timeout: e2eTimeout },
    );
    await page.getByRole("button", { name: "Enviar instrucciones" }).click();
    const forgotRes = await forgotUi;
    expect(forgotRes.ok()).toBeTruthy();
    const body = (await forgotRes.json()) as { resetToken?: string };
    expect(body.resetToken, JSON.stringify(body)).toBeTruthy();

    // Dev path should land on reset with the real token (auto-nav or deep link).
    await expect(page).toHaveURL(new RegExp(`/reset-password\\?token=${body.resetToken}`), {
      timeout: e2eTimeout,
    });
    await expect(page.getByRole("heading", { name: "Nueva contraseña" })).toBeVisible({
      timeout: e2eTimeout,
    });
    await fillLeptosInput(page, "#reset-password", newPassword);
    await page.getByRole("button", { name: "Restablecer contraseña" }).click();
    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
    await expect(page.locator(".session-bar__email")).toContainText(email);

    const oldLogin = await request.post("/api/auth/login", {
      data: { email, password: oldPassword },
    });
    expect(oldLogin.status()).toBe(401);
    const newLogin = await request.post("/api/auth/login", {
      data: { email, password: newPassword },
    });
    expect(newLogin.ok()).toBeTruthy();
  });
});
