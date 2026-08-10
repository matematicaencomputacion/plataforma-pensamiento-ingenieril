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
 * Paso 2 coding surface. Default: mock `window.ppiPyodide` for CI stability.
 * Set PPI_E2E_REAL_PYODIDE=1 to exercise the CDN engine locally.
 */

const useRealPyodide = process.env.PPI_E2E_REAL_PYODIDE === "1";

function uniqueCreds() {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
  const email = `e2e-learn-${stamp}@example.com`;
  if (password.length < 8) {
    test.skip(true, "PPI_E2E_PASSWORD must be at least 8 characters");
    throw new Error("unreachable");
  }
  return { email, password };
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

test.describe("learn coding (Paso 2)", () => {
  test.beforeEach(async ({ page }) => {
    if (!useRealPyodide) {
      await installPyodideMock(page);
    }
  });

  test("asset ppi-pyodide.js is served", async ({ request }) => {
    const res = await request.get("/ppi-pyodide.js");
    expect(res.ok(), await res.text()).toBeTruthy();
    const body = await res.text();
    expect(body).toContain("ppiPyodide");
    expect(body).toContain("0.27.7");
  });

  test("unauthenticated /learn redirects to login", async ({ page }) => {
    await gotoApp(page, "/learn");
    await expect(page).toHaveURL(/\/login/, { timeout: e2eTimeout });
  });

  test("validate unlocks continue then next coding step", async ({ page, request }) => {
    const { email, password } = uniqueCreds();
    const reg = await request.post("/api/auth/register", {
      data: { email, password },
      timeout: e2eTimeout,
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();

    await login(page, email, password);
    await page.getByRole("link", { name: "Paso 2 · Coding" }).click();
    await expect(page).toHaveURL(/\/learn/, { timeout: e2eTimeout });
    await expect(
      page.getByRole("heading", { name: /Variables/i }),
    ).toBeVisible({ timeout: e2eTimeout });

    const engineTimeout = useRealPyodide ? 120_000 : e2eTimeout;
    await expect(page.locator("#learn-engine-status")).toHaveAttribute(
      "data-status",
      "ready",
      { timeout: engineTimeout },
    );

    // Type chips rail under Enunciado → Variables
    await expect(page.locator("#learn-type-chips .type-chips__label")).toHaveText(
      "Variables",
    );
    await page.locator("#type-chip-str").click();
    await expect(page.locator("#type-chip-panel")).toContainText(/String|Cadena/i);
    await expect(page.locator('[data-ident="nombre"].learn__ident--flash')).toHaveCount(2);
    await page.locator("#type-chip-int").click();
    await expect(page.locator("#type-chip-panel")).toContainText(/Integer|entero/i);
    await expect(page.locator('[data-ident="edad"].learn__ident--flash')).toHaveCount(2);
    await page.locator("#type-chip-obj").click();
    await expect(page.locator("#type-chip-panel")).toContainText(/Object|Objeto/i);
    await page.locator("#type-chip-obj").click();
    await expect(page.locator("#type-chip-panel")).toHaveCount(0);

    const solution = 'nombre = "Ana"\nedad = 25\nprint(nombre, edad)\n';
    await fillLeptosTextarea(page, "#learn-editor", solution);

    await page.getByRole("button", { name: "Validar solución" }).click();
    await expect(page.locator("#learn-check-log")).toContainText(/Checks OK|PASSED|OK/i, {
      timeout: engineTimeout,
    });
    await expect(page.locator("#learn-success-banner")).toContainText(
      "¡Ejercicio completado con éxito!",
      { timeout: engineTimeout },
    );
    await expect(page.locator("#learn-continue")).toBeEnabled({
      timeout: e2eTimeout,
    });

    await page.locator("#learn-continue").click();
    await expect(page).toHaveURL(/\/learn\/py-02-intro/, { timeout: e2eTimeout });
    await expect(page.getByRole("heading", { name: "Python Intro" })).toBeVisible({
      timeout: e2eTimeout,
    });
  });
});
