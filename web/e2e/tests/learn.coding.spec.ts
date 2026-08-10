import { expect, test, type Page } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  fillLeptosTextarea,
  gotoApp,
  waitForAuthFormReady,
} from "./helpers";

/**
 * Paso 2 coding surface. Default: mock `window.ppiPyodide` for CI stability.
 * Set PPI_E2E_REAL_PYODIDE=1 to exercise the CDN engine locally.
 */

const useRealPyodide = process.env.PPI_E2E_REAL_PYODIDE === "1";

async function installPyodideMock(page: Page) {
  await page.addInitScript(() => {
    const api = {
      version: "mock",
      ensure: async () => ({
        status: "ready",
        message: "Motor Python listo (mock E2E).",
      }),
      run: async (code: string) => {
        try {
          // Minimal echo of print(...) for mock console.
          const m = String(code).match(/print\((.*)\)/);
          return {
            ok: true,
            stdout: m ? `${m[1].replace(/['"]/g, "")}\n` : "",
            stderr: "",
          };
        } catch (err) {
          return {
            ok: false,
            stdout: "",
            stderr: String(err),
            error: String(err),
          };
        }
      },
      check: async (code: string, _testSource: string) => {
        const src = String(code);
        const hasNombre = /nombre\s*=\s*["'].*["']/.test(src);
        const hasEdad = /edad\s*=\s*\d+/.test(src);
        const hasPrint = /print\s*\(\s*nombre\s*,\s*edad\s*\)/.test(src);
        const passed = hasNombre && hasEdad && hasPrint;
        return {
          passed,
          stdout: passed ? "PASSED test_variables\nOK — 1 test(s) passed\n" : "FAILED\n",
          stderr: "",
          summary: passed
            ? "✓ Checks OK — podés Continuar"
            : "✗ Checks fallaron — revisá el enunciado y el código",
          details: passed ? "Todos los tests pasaron." : "assert failed",
        };
      },
      formatRunLog: (r: { ok: boolean; stdout?: string; stderr?: string; error?: string }) => {
        const parts = ["=== Run ==="];
        if (r.stdout?.trim()) parts.push(r.stdout.trimEnd());
        if (r.stderr?.trim()) {
          parts.push("--- stderr ---", r.stderr.trimEnd());
        }
        if (!r.ok && r.error) parts.push("--- error ---", r.error);
        if (r.ok) parts.push("", "✓ Ejecución finalizada");
        return parts.join("\n");
      },
      formatCheckLog: (r: { summary?: string; details?: string }) =>
        ["=== Validar ===", r.summary || "", "", r.details || ""].join("\n"),
      statusMessage: (s: string) => s,
      isReady: () => true,
      getLastError: () => null,
    };
    (window as unknown as { ppiPyodide: typeof api }).ppiPyodide = api;
  });
}

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
    if (!useRealPyodide) {
      await installPyodideMock(page);
    }
    await gotoApp(page, "/learn");
    await expect(page).toHaveURL(/\/login/, { timeout: e2eTimeout });
  });

  test("validate unlocks continue then workspace", async ({ page, request }) => {
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

    const solution = 'nombre = "Ana"\nedad = 25\nprint(nombre, edad)\n';
    await fillLeptosTextarea(page, "#learn-editor", solution);

    await page.getByRole("button", { name: "Validar" }).click();
    await expect(page.locator("#learn-console")).toContainText(/Checks OK|PASSED|OK/i, {
      timeout: engineTimeout,
    });
    await expect(page.locator("#learn-continue")).toBeEnabled({
      timeout: e2eTimeout,
    });

    await page.locator("#learn-continue").click();
    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
  });
});
