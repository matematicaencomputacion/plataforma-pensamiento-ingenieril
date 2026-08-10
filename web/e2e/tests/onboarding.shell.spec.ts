import { expect, test } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  fillLeptosTextarea,
  gotoApp,
  waitForAuthFormReady,
} from "./helpers";

/**
 * Smoke: authenticated learner reaches Leptos `/onboarding` shell (Paso 1).
 * Full analyze coverage lives in `onboarding.synthesize.spec.ts`.
 */

function uniqueCreds(): { email: string; password: string } {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
  const email =
    process.env.PPI_E2E_EMAIL?.trim() ||
    `e2e-onboard-${stamp}@example.com`;
  if (password.length < 8) {
    test.skip(true, "PPI_E2E_PASSWORD must be at least 8 characters (API rule)");
    throw new Error("unreachable: short PPI_E2E_PASSWORD");
  }
  return { email, password };
}

test.describe("onboarding shell smoke", () => {
  test("workspace CTA reaches drafting surface", async ({ page, request }) => {
    const { email, password } = uniqueCreds();
    const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
    const accountEmail = email.includes("@")
      ? email.replace(/@/, `+${stamp}@`)
      : `e2e-onboard-${stamp}@example.com`;

    const reg = await request.post("/api/auth/register", {
      data: { email: accountEmail, password },
      timeout: e2eTimeout,
    });
    expect(
      reg.ok(),
      `register seed failed: ${reg.status()} ${await reg.text()}`,
    ).toBeTruthy();

    await gotoApp(page, "/login");
    await waitForAuthFormReady(page, {
      emailSelector: "#login-email",
      passwordSelector: "#login-password",
      submitName: "Entrar",
    });
    await fillLeptosInput(page, "#login-email", accountEmail);
    await fillLeptosInput(page, "#login-password", password);

    const loginResponse = page.waitForResponse(
      (res) =>
        res.url().includes("/api/auth/login") &&
        res.request().method() === "POST",
      { timeout: e2eTimeout },
    );
    await page.getByRole("button", { name: "Entrar" }).click();
    expect((await loginResponse).ok()).toBeTruthy();
    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });

    await page.getByRole("link", { name: "Empezar coaching" }).click();
    await expect(page).toHaveURL(/\/onboarding/, { timeout: e2eTimeout });
    await expect(
      page.getByRole("heading", { name: "Hola, ¿cómo estás?" }),
    ).toBeVisible({ timeout: e2eTimeout });

    const draft = "Quiero automatizar reportes; urgencia alta; visión a 5 años en datos.";
    await fillLeptosTextarea(page, "#coaching-notes", draft);
    await expect(page.locator("#coaching-notes")).toHaveValue(draft);

    await expect(page.locator("#coaching-analyze")).toBeEnabled();
  });

  test("unauthenticated /onboarding redirects to login", async ({ page }) => {
    await gotoApp(page, "/onboarding");
    await expect(page).toHaveURL(/\/login/, { timeout: e2eTimeout });
  });
});
