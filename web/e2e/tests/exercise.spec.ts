import { expect, test, type Page } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  fillLeptosTextarea,
  gotoApp,
  installPyodideMock,
  waitForAuthFormReady,
} from "./helpers";

/**
 * Rebanada ejecución: Ready → Ejecutar código → stdout en consola.
 * Mock por defecto (ADR 002: nunca se envía Python a Go).
 */

const useRealPyodide = process.env.PPI_E2E_REAL_PYODIDE === "1";

function uniqueCreds() {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
  return { email: `e2e-ex-${stamp}@example.com`, password };
}

async function login(page: Page, email: string, password: string) {
  await gotoApp(page, "/login");
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
}

test.describe("exercise execution engine", () => {
  test.beforeEach(async ({ page }) => {
    if (!useRealPyodide) {
      await installPyodideMock(page);
    }
  });

  test("Ready then Ejecutar shows stdout Hola IngenierIA", async ({
    page,
    request,
  }) => {
    const { email, password } = uniqueCreds();
    const reg = await request.post("/api/auth/register", {
      data: { email, password },
      timeout: e2eTimeout,
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();

    await login(page, email, password);
    await page.getByRole("link", { name: "Paso 2 · Coding" }).click();
    await expect(page).toHaveURL(/\/learn/, { timeout: e2eTimeout });

    const engineTimeout = useRealPyodide ? 120_000 : e2eTimeout;
    await expect(page.locator("#learn-engine-status")).toHaveAttribute(
      "data-status",
      "ready",
      { timeout: engineTimeout },
    );

    // Level metadata from Go (statement only — code never leaves the browser).
    await expect(page.locator(".learn__level-statement")).toBeVisible({
      timeout: e2eTimeout,
    });

    await fillLeptosTextarea(
      page,
      "#learn-editor",
      'print("Hola IngenierIA")\n',
    );

    await page.getByRole("button", { name: "Ejecutar código" }).click();
    await expect(page.locator("#learn-stdout")).toContainText("Hola IngenierIA", {
      timeout: engineTimeout,
    });
    await expect(page.locator("#learn-stderr")).toHaveCount(0);
  });
});
