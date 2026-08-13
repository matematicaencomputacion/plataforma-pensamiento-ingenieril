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

const SOLUTION = `# This is a comment
# print("This should not run")
"""This is
a multiline
comment"""
`;

function uniqueCreds() {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
  return { email: `e2e-ms06-${stamp}@example.com`, password };
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

async function unlockThroughStep5(request: APIRequestContext, token: string) {
  const steps: Array<{ level_id: number; step_id: string }> = [
    { level_id: 1, step_id: "py-02-variables" },
    { level_id: 2, step_id: "py-02-intro" },
    { level_id: 3, step_id: "py-03-get-started" },
    { level_id: 4, step_id: "py-04-syntax" },
    { level_id: 5, step_id: "py-05-output" },
  ];
  for (const body of steps) {
    const res = await request.post("/api/progress/complete", {
      headers: { Authorization: `Bearer ${token}` },
      data: { ...body, passed: true },
      timeout: e2eTimeout,
    });
    expect(res.ok(), await res.text()).toBeTruthy();
  }
}

test.describe("micro-step 6 · Python Comments", () => {
  test.beforeEach(async ({ page }) => {
    if (!useRealPyodide) {
      await installPyodideMock(page);
    }
  });

  test("rail opens comments challenge; pass returns to workspace with badge", async ({
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

    await login(page, email, password);
    await unlockThroughStep5(request, regJson.token);
    await page.reload();
    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
    await expect(page.locator("#workspace-microsteps")).toHaveAttribute(
      "data-current-level",
      "6",
      { timeout: e2eTimeout },
    );

    await expect(page.locator("#workspace-microstep-link-6")).toBeVisible();
    await expect(page.locator(
      `#workspace-microsteps [data-microstep="7"]`,
    )).toHaveClass(/workspace__microstep--jumpable/);

    await page.locator("#workspace-microstep-link-6").click();
    await expect(page).toHaveURL(/\/learn\/py-06-comments/, { timeout: e2eTimeout });
    await expect(
      page.getByRole("heading", { name: "Python Comments" }),
    ).toBeVisible({ timeout: e2eTimeout });

    const engineTimeout = useRealPyodide ? 120_000 : e2eTimeout;
    await expect(page.locator("#learn-engine-status")).toHaveAttribute(
      "data-status",
      "ready",
      { timeout: engineTimeout },
    );

    await fillLeptosTextarea(page, "#learn-editor", SOLUTION);
    await page.getByRole("button", { name: "Validar solución" }).click();
    await expect(page.locator("#learn-progress-check")).toBeVisible({
      timeout: engineTimeout,
    });

    await page.locator("#learn-continue").click();
    await expect(page).toHaveURL(/\/learn\/py-07-data-types/, {
      timeout: e2eTimeout,
    });

    await page
      .getByLabel("Navegación del Paso 2")
      .getByRole("link", { name: "Workspace" })
      .click();
    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
    await expect(page.locator("#workspace-microsteps")).toHaveAttribute(
      "data-current-level",
      "7",
    );
    const step6 = page.locator('#workspace-microsteps [data-microstep="6"]');
    await expect(step6).toHaveClass(/workspace__microstep--done/);
    await expect(step6.locator(".workspace__microstep-badge")).toBeVisible();
  });
});
