import { expect, test, type Page } from "@playwright/test";
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

const P2_STEP = {
  micro: 52,
  id: "py-52-functions",
  title: "Python Functions",
  solution: `def my_function():
    print("Hello from a function")
my_function()
`,
};

function uniqueCreds() {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
  return { email: `e2e-concepts-${stamp}@example.com`, password };
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

test.describe("journey conceptual · partición 2", () => {
  test.beforeEach(async ({ page }) => {
    if (!useRealPyodide) {
      await installPyodideMock(page);
    }
  });

  test("login → learn P2 → [2] → hub → drill → Validar → mastery", async ({
    page,
    request,
  }) => {
    const { email, password } = uniqueCreds();
    const reg = await request.post("/api/auth/register", {
      data: { email, password },
      timeout: e2eTimeout,
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();
    const regJson = (await reg.json()) as { token: string };

    await login(page, email, password);
    await expect(page.locator("#partition-nav-2")).toHaveAttribute(
      "data-mastery",
      "0",
      { timeout: e2eTimeout },
    );

    await unlockThroughMicroStep(request, regJson.token, P2_STEP.micro - 1);
    await page.reload();
    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });

    await page.goto(`/learn/${P2_STEP.id}`);
    await expect(page).toHaveURL(new RegExp(`/learn/${P2_STEP.id}`), {
      timeout: e2eTimeout,
    });
    await expect(
      page.getByRole("heading", { name: P2_STEP.title }),
    ).toBeVisible({ timeout: e2eTimeout });

    const engineTimeout = useRealPyodide ? 120_000 : e2eTimeout;
    await expect(page.locator("#learn-engine-status")).toHaveAttribute(
      "data-status",
      "ready",
      { timeout: engineTimeout },
    );

    await page.locator("#partition-nav-2").click();
    await expect(page).toHaveURL(/\/concepts\/2/, { timeout: e2eTimeout });
    await expect(
      page.getByRole("heading", { name: /Ámbitos y Nombres/i }),
    ).toBeVisible({ timeout: e2eTimeout });
    await expect(page.locator(`#concepts-drill-${P2_STEP.micro}`)).toBeVisible({
      timeout: e2eTimeout,
    });

    await page.locator(`#concepts-drill-${P2_STEP.micro}`).click();
    await expect(page).toHaveURL(new RegExp(`/learn/${P2_STEP.id}`), {
      timeout: e2eTimeout,
    });

    await fillLeptosTextarea(page, "#learn-editor", P2_STEP.solution);
    await page.getByRole("button", { name: "Validar solución" }).click();
    await expect(page.locator("#learn-progress-check")).toBeVisible({
      timeout: engineTimeout,
    });

    await expect(page.locator("#partition-nav-2")).not.toHaveAttribute(
      "data-mastery",
      "0",
      { timeout: e2eTimeout },
    );

    await page.locator("#partition-nav-2").click();
    await expect(page).toHaveURL(/\/concepts\/2/, { timeout: e2eTimeout });
    await expect(
      page.locator(`#concepts-drill-${P2_STEP.micro}`).locator(".."),
    ).toHaveClass(/concepts__drill--done/);
    await expect(page.locator("#partition-nav-2")).not.toHaveAttribute(
      "data-mastery",
      "0",
    );
  });
});
