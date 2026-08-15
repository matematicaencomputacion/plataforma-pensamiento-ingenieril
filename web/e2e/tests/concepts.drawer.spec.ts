import { expect, test, type Page } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  fillLeptosTextarea,
  gotoApp,
  installPyodideMock,
  waitForAuthFormReady,
} from "./helpers";

const MARKER = "WAVE_B_MARKER_42";
const STEP_ID = "py-02-variables";

function uniqueCreds() {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
  return { email: `e2e-drawer-${stamp}@example.com`, password };
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

test.describe("conceptual glossary drawer", () => {
  test.beforeEach(async ({ page }) => {
    await installPyodideMock(page);
  });

  test("FAB search peek dock keeps editor marker", async ({ page, request }) => {
    const { email, password } = uniqueCreds();
    const reg = await request.post("/api/auth/register", {
      data: { email, password },
      timeout: e2eTimeout,
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();

    await login(page, email, password);
    await gotoApp(page, `/learn/${STEP_ID}`);
    await expect(page).toHaveURL(new RegExp(`/learn/${STEP_ID}`), {
      timeout: e2eTimeout,
    });
    await expect(page.locator("#learn-editor")).toBeVisible({
      timeout: e2eTimeout,
    });

    await expect(page.locator("#concept-fab")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator("#concept-glossary-search")).toHaveCount(0);
    await expect(page.locator("#concept-drawer")).toHaveCount(0);

    await fillLeptosTextarea(page, "#learn-editor", MARKER);
    await expect(page.locator("#learn-editor")).toHaveValue(MARKER);

    await page.keyboard.press("Control+K");
    await expect(page.locator("#concept-glossary-search")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator("#concept-glossary-search")).toBeFocused();
    for (const n of [1, 2, 3, 4, 5]) {
      await expect(page.locator(`#concept-lens-${n}`)).toBeVisible();
      await expect(page.locator(`#concept-lens-${n}`)).toHaveAttribute(
        "data-lens",
        String(n),
      );
    }

    await fillLeptosInput(page, "#concept-glossary-search", "extend");
    await expect(page.locator("#concept-hit-python-lists")).toBeVisible({
      timeout: e2eTimeout,
    });
    await page.locator("#concept-hit-python-lists").click();

    await expect(page.locator("#concept-peek")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator("#concept-peek #concept-diagram")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator("#concept-peek .concept-card__tldr")).toBeVisible();
    await expect(page).toHaveURL(new RegExp(`/learn/${STEP_ID}`));
    await expect(page.locator("#learn-editor")).toHaveValue(MARKER);

    await page.locator("#concept-dock").click();
    await expect(page.locator("#concept-drawer")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator("#concept-drawer #concept-diagram")).toBeVisible();
    await expect(page.locator("#learn-editor")).toBeVisible();
    await expect(page.locator("#learn-editor")).toHaveValue(MARKER);
    await expect(page).toHaveURL(new RegExp(`/learn/${STEP_ID}`));

    await page.locator("#concept-undock").click();
    await expect(page.locator("#concept-fab")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator("#concept-drawer")).toHaveCount(0);
    await expect(page.locator("#learn-editor")).toHaveValue(MARKER);
  });
});
