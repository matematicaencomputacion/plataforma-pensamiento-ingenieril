import { expect, test } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  fillLeptosTextarea,
  gotoApp,
  waitForAuthFormReady,
} from "./helpers";

/**
 * Synthesize smoke against Go with LEARNER_PROFILE_LLM=mock (harness default).
 */

function uniqueCreds(): { email: string; password: string } {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
  const email = `e2e-synth-${stamp}@example.com`;
  if (password.length < 8) {
    test.skip(true, "PPI_E2E_PASSWORD must be at least 8 characters (API rule)");
    throw new Error("unreachable: short PPI_E2E_PASSWORD");
  }
  return { email, password };
}

test.describe("onboarding synthesize", () => {
  test("analyze fills editable profile fields (mock LLM)", async ({
    page,
    request,
  }) => {
    const { email, password } = uniqueCreds();

    const reg = await request.post("/api/auth/register", {
      data: { email, password },
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
    await fillLeptosInput(page, "#login-email", email);
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

    // Keywords tip the mock classifier (estudiante + urgencia → purpose + urgency).
    const draft =
      "Soy estudiante y necesito resultados rápido por urgencia en el trabajo.";
    await fillLeptosTextarea(page, "#coaching-notes", draft);

    const synthesizeResponse = page.waitForResponse(
      (res) =>
        res.url().includes("/api/learner/profile/synthesize") &&
        res.request().method() === "POST",
      { timeout: e2eTimeout },
    );
    await page.getByRole("button", { name: "Enviar para análisis" }).click();
    const synthRes = await synthesizeResponse;
    expect(
      synthRes.ok(),
      `synthesize failed: ${synthRes.status()} ${await synthRes.text()}`,
    ).toBeTruthy();

    await expect(page.locator("#profile-purpose")).toBeVisible({
      timeout: e2eTimeout,
    });
    await expect(page.locator("#profile-purpose")).toHaveValue(
      /familia|autonomía|autonomia/i,
    );
    await expect(page.locator("#profile-urgency")).toHaveValue(/Extrema|inmediato/i);
    await expect(
      page.getByRole("button", { name: "Guardar perfil" }),
    ).toBeDisabled();
  });
});
