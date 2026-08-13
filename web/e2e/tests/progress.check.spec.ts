import { expect, test, type APIRequestContext, type Page } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  fillLeptosTextarea,
  gotoApp,
  installPyodideMock,
  waitForAuthFormReady,
} from "./helpers";

const useRealPyodide = process.env.PPI_E2E_REAL_PYODIDE === "1";

function uniqueCreds() {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
  return { email: `e2e-check-${stamp}@example.com`, password };
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

test.describe("persistent progress checks", () => {
  test.beforeEach(async ({ page }) => {
    if (!useRealPyodide) {
      await installPyodideMock(page);
    }
  });

  test("validate shows check; workspace mirrors it; reset clears", async ({
    page,
    request,
  }: {
    page: Page;
    request: APIRequestContext;
  }) => {
    const { email, password } = uniqueCreds();
    const reg = await request.post("/api/auth/register", {
      data: { email, password },
      timeout: e2eTimeout,
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();

    await login(page, email, password);
    const step1 = page.locator('#workspace-microsteps [data-microstep="1"]');
    await expect(step1.locator(".workspace__microstep-badge")).toHaveCount(0);

    await page.getByRole("link", { name: "Paso 2 · Coding" }).click();
    await expect(page).toHaveURL(/\/learn/, { timeout: e2eTimeout });

    const engineTimeout = useRealPyodide ? 120_000 : e2eTimeout;
    await expect(page.locator("#learn-engine-status")).toHaveAttribute(
      "data-status",
      "ready",
      { timeout: engineTimeout },
    );

    await fillLeptosTextarea(
      page,
      "#learn-editor",
      'nombre = "Ana"\nedad = 25\nprint(nombre, edad)\n',
    );
    await page.getByRole("button", { name: "Validar solución" }).click();

    await expect(page.locator("#learn-progress-check")).toBeVisible({
      timeout: engineTimeout,
    });
    await expect(page.locator("#learn-test-cases .learn__case--pass")).toBeVisible();

    await page.locator("#learn-continue").click();
    await expect(page).toHaveURL(/\/learn\/py-02-intro/, { timeout: e2eTimeout });

    await page.getByLabel("Navegación del Paso 2").getByRole("link", { name: "Workspace" }).click();
    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
    await expect(page.locator("#workspace-microsteps")).toBeVisible();
    await expect(page.locator("#workspace-microsteps")).toHaveAttribute(
      "data-current-level",
      "2",
    );
    await expect(step1).toHaveClass(/workspace__microstep--done/);
    await expect(step1.locator(".workspace__microstep-badge")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(step1.locator(".progress-check")).toBeVisible();
    await expect(page.locator("#workspace-microsteps li")).toHaveCount(430);
    await expect(page.locator("#workspace-level-check")).toHaveCount(0);

    const resetResponse = page.waitForResponse(
      (res) =>
        res.url().includes("/api/progress/reset") &&
        res.request().method() === "POST",
      { timeout: e2eTimeout },
    );
    await page.locator("#workspace-reset-progress").click();
    expect((await resetResponse).ok()).toBeTruthy();
    await expect(page.locator("#workspace-microsteps")).toHaveAttribute(
      "data-current-level",
      "1",
    );
    await expect(step1.locator(".workspace__microstep-badge")).toHaveCount(0, {
      timeout: e2eTimeout,
    });
    await expect(page.locator("#workspace-reset-note")).toContainText(/reiniciado/i);
  });

  test("jump-ahead complete marks only that micro-step", async ({
    page,
    request,
  }: {
    page: Page;
    request: APIRequestContext;
  }) => {
    const { email, password } = uniqueCreds();
    const reg = await request.post("/api/auth/register", {
      data: { email, password },
      timeout: e2eTimeout,
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();
    const regJson = (await reg.json()) as { token: string };
    const token = regJson.token;

    const complete = await request.post("/api/progress/complete", {
      headers: { Authorization: `Bearer ${token}` },
      data: { level_id: 157, step_id: "py-157-jump", passed: true },
      timeout: e2eTimeout,
    });
    expect(complete.ok(), await complete.text()).toBeTruthy();
    const body = (await complete.json()) as {
      current_level: number;
      advanced: boolean;
      completed_levels: number[];
    };
    expect(body.current_level).toBe(1);
    expect(body.advanced).toBe(false);
    expect(body.completed_levels).toEqual([157]);

    await login(page, email, password);
    await expect(page.locator("#workspace-microsteps")).toHaveAttribute(
      "data-current-level",
      "1",
    );

    const step157 = page.locator('#workspace-microsteps [data-microstep="157"]');
    const step1 = page.locator('#workspace-microsteps [data-microstep="1"]');
    const step156 = page.locator('#workspace-microsteps [data-microstep="156"]');

    await expect(step157).toHaveClass(/workspace__microstep--done/);
    await expect(step157.locator(".workspace__microstep-badge")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(step1.locator(".workspace__microstep-badge")).toHaveCount(0);
    await expect(step156.locator(".workspace__microstep-badge")).toHaveCount(0);
    await expect(page.locator("#workspace-microsteps .workspace__microstep-badge")).toHaveCount(1);
  });
});
