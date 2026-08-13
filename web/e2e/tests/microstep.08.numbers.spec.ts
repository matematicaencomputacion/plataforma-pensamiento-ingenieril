import { expect, test, type APIRequestContext, type Page } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  fillLeptosTextarea,
  gotoApp,
  installPyodideMock,
  waitForAuthFormReady,
} from "./helpers";
import { unlockThroughMicroStep } from "./microstepProgress";

const useRealPyodide = process.env.PPI_E2E_REAL_PYODIDE === "1";

const SOLUTION = `x = 5
y = 3.14
z = 2 + 3j
print(type(x))
print(type(y))
print(type(z))
`;

function uniqueCreds() {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
  return { email: `e2e-ms08-${stamp}@example.com`, password };
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

test.describe("micro-step 8 · Python Numbers", () => {
  test.beforeEach(async ({ page }) => {
    if (!useRealPyodide) {
      await installPyodideMock(page);
    }
  });

  test("rail opens numbers challenge; pass returns to workspace with badge", async ({
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
    await unlockThroughMicroStep(request, regJson.token, 7);
    await page.reload();
    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
    await expect(page.locator("#workspace-microsteps")).toHaveAttribute(
      "data-current-level",
      "8",
      { timeout: e2eTimeout },
    );

    await expect(page.locator("#workspace-microstep-link-8")).toBeVisible();
    await expect(page.locator(
      `#workspace-microsteps [data-microstep="9"]`,
    )).toHaveClass(/workspace__microstep--jumpable/);

    await page.locator("#workspace-microstep-link-8").click();
    await expect(page).toHaveURL(/\/learn\/py-08-numbers/, { timeout: e2eTimeout });
    await expect(
      page.getByRole("heading", { name: "Python Numbers" }),
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
    await expect(page).toHaveURL(/\/learn\/py-09-casting/, {
      timeout: e2eTimeout,
    });

    await page
      .getByLabel("Navegación del Paso 2")
      .getByRole("link", { name: "Workspace" })
      .click();
    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
    await expect(page.locator("#workspace-microsteps")).toHaveAttribute(
      "data-current-level",
      "9",
    );
    const step8 = page.locator('#workspace-microsteps [data-microstep="8"]');
    await expect(step8).toHaveClass(/workspace__microstep--done/);
    await expect(step8.locator(".workspace__microstep-badge")).toBeVisible();
  });
});
