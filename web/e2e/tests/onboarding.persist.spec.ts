import { expect, test } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  fillLeptosTextarea,
  gotoApp,
  waitForAuthFormReady,
} from "./helpers";

/**
 * Persist + hydrate coaching profile via GET|PUT /api/user/profile.
 */

function uniqueCreds(): { email: string; password: string } {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
  const email = `e2e-persist-${stamp}@example.com`;
  if (password.length < 8) {
    test.skip(true, "PPI_E2E_PASSWORD must be at least 8 characters (API rule)");
    throw new Error("unreachable: short PPI_E2E_PASSWORD");
  }
  return { email, password };
}

async function loginViaUi(
  page: import("@playwright/test").Page,
  email: string,
  password: string,
) {
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

test.describe("onboarding persist + hydrate", () => {
  test("save profile then reload hydrates saved state", async ({
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

    await loginViaUi(page, email, password);
    await page.getByRole("link", { name: "Empezar coaching" }).click();
    await expect(page).toHaveURL(/\/onboarding/, { timeout: e2eTimeout });

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
    expect((await synthesizeResponse).ok()).toBeTruthy();
    await expect(page.locator("#profile-purpose")).toBeVisible({
      timeout: e2eTimeout,
    });

    await fillLeptosTextarea(
      page,
      "#profile-purpose",
      "Propósito editado para persistencia E2E",
    );

    const putResponse = page.waitForResponse(
      (res) =>
        res.url().includes("/api/user/profile") &&
        res.request().method() === "PUT",
      { timeout: e2eTimeout },
    );
    await page.getByRole("button", { name: "Guardar perfil" }).click();
    const putRes = await putResponse;
    expect(
      putRes.ok(),
      `PUT profile failed: ${putRes.status()} ${await putRes.text()}`,
    ).toBeTruthy();

    await expect(page.locator("#coaching-saved-status")).toHaveText(
      /Perfil guardado/i,
      { timeout: e2eTimeout },
    );
    await expect(
      page.getByRole("link", { name: "Continuar al Paso 2" }),
    ).toBeVisible();

    // Re-enter /onboarding so hydrate runs GET /api/user/profile with Bearer.
    await page.getByRole("link", { name: "Volver al workspace" }).click();
    await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
    await page.getByRole("link", { name: "Empezar coaching" }).click();
    await expect(page).toHaveURL(/\/onboarding/, { timeout: e2eTimeout });
    await expect(page.locator("#coaching-saved-status")).toHaveText(
      /Perfil guardado/i,
      { timeout: e2eTimeout },
    );
    await expect(page.locator("#profile-purpose")).toHaveValue(
      "Propósito editado para persistencia E2E",
      { timeout: e2eTimeout },
    );
  });
});
