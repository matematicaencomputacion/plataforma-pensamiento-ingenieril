import { expect, test, type APIRequestContext, type Page } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  fillLeptosTextarea,
  gotoApp,
  installPyodideMock,
  waitForAuthFormReady,
} from "./helpers";

/**
 * Rebanada 3: Validar solución (fail/pass) + banner + progreso a Go.
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

async function goToLearnReady(page: Page, request: APIRequestContext) {
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
  return { engineTimeout };
}

test.describe("exercise evaluation (rebanada 3)", () => {
  test.beforeEach(async ({ page }) => {
    if (!useRealPyodide) {
      await installPyodideMock(page);
    }
  });

  test("Ready then Ejecutar shows stdout Hola IngenierIA", async ({
    page,
    request,
  }) => {
    const { engineTimeout } = await goToLearnReady(page, request);

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

  test("incorrect solution shows failing test case", async ({ page, request }) => {
    const { engineTimeout } = await goToLearnReady(page, request);

    await fillLeptosTextarea(page, "#learn-editor", "x = 1\n");
    await page.getByRole("button", { name: "Validar solución" }).click();

    const failCase = page.locator("#learn-test-cases .learn__case--fail");
    await expect(failCase).toBeVisible({ timeout: engineTimeout });
    await expect(failCase).toHaveAttribute("data-status", "fail");
    await expect(page.locator("#learn-success-banner")).toHaveCount(0);
    await expect(page.locator("#learn-continue")).toBeDisabled();
  });

  test("correct solution shows green cases, success banner, and posts progress", async ({
    page,
    request,
  }) => {
    const { engineTimeout } = await goToLearnReady(page, request);

    const progressPromise = page.waitForRequest(
      (req) =>
        req.url().includes("/api/progress/complete") && req.method() === "POST",
      { timeout: engineTimeout },
    );

    await fillLeptosTextarea(
      page,
      "#learn-editor",
      'nombre = "Ana"\nedad = 25\nprint(nombre, edad)\n',
    );
    await page.getByRole("button", { name: "Validar solución" }).click();

    const progressReq = await progressPromise;
    const body = progressReq.postDataJSON() as {
      level_id?: number;
      step_id?: string;
      passed?: boolean;
      code?: string;
    };
    expect(body.passed).toBe(true);
    expect(body.step_id).toBe("py-02-variables");
    expect(body.code).toBeUndefined();

    const passCase = page.locator("#learn-test-cases .learn__case--pass");
    await expect(passCase).toBeVisible({ timeout: engineTimeout });
    await expect(passCase).toHaveAttribute("data-status", "pass");
    await expect(page.locator("#learn-success-banner")).toContainText(
      "¡Ejercicio completado con éxito!",
    );
    await expect(page.locator("#learn-continue")).toBeEnabled({
      timeout: e2eTimeout,
    });
  });
});
