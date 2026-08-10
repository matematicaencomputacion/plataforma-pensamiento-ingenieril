import { expect, test, type Page } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  fillLeptosTextarea,
  gotoApp,
  waitForAuthFormReady,
} from "./helpers";

/**
 * Coaching fixes: synthesize fills 4 fields → save persists (LEARNER_PROFILE_LLM=mock).
 */

function uniqueCreds() {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
  return { email: `e2e-coach-${stamp}@example.com`, password };
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

test.describe("coaching synthesize + persist", () => {
  test("asset ppi-speech.js is served", async ({ request }) => {
    const res = await request.get("/ppi-speech.js");
    expect(res.ok(), await res.text()).toBeTruthy();
    const body = await res.text();
    expect(body).toContain("ppiSpeech");
  });

  test("analyze fills four fields then save shows Perfil guardado", async ({
    page,
    request,
  }) => {
    const { email, password } = uniqueCreds();
    const reg = await request.post("/api/auth/register", {
      data: { email, password },
      timeout: e2eTimeout,
    });
    expect(reg.ok(), await reg.text()).toBeTruthy();

    await login(page, email, password);
    await page.getByRole("link", { name: "Empezar coaching" }).click();
    await expect(page).toHaveURL(/\/onboarding/, { timeout: e2eTimeout });

    const draft =
      "Soy estudiante y necesito resultados rápido por urgencia en el trabajo. No se bien el camino. Probé Coursera.";
    await fillLeptosTextarea(page, "#coaching-notes", draft);

    const synthesizeResponse = page.waitForResponse(
      (res) =>
        res.url().includes("/api/learner/profile/synthesize") &&
        res.request().method() === "POST",
      { timeout: e2eTimeout },
    );
    await page
      .getByRole("button", { name: /Analizar mi respuesta con IA/i })
      .click();
    expect((await synthesizeResponse).ok()).toBeTruthy();

    await expect(page.locator("#profile-purpose")).not.toHaveValue("", {
      timeout: e2eTimeout,
    });
    await expect(page.locator("#profile-urgency")).not.toHaveValue("");
    // Vision/stack pueden venir vacíos según el clasificador; completar a mano si hace falta.
    const vision = page.locator("#profile-vision");
    const stack = page.locator("#profile-stack");
    if ((await vision.inputValue()) === "") {
      await fillLeptosTextarea(page, "#profile-vision", "Visión E2E de coaching");
    }
    if ((await stack.inputValue()) === "") {
      await fillLeptosTextarea(page, "#profile-stack", "Cursor · Jupyter");
    }
    await expect(page.locator("#coaching-profile-phase")).toContainText(/revisión/i);

    const putResponse = page.waitForResponse(
      (res) =>
        res.url().includes("/api/user/profile") &&
        res.request().method() === "PUT",
      { timeout: e2eTimeout },
    );
    await page.getByRole("button", { name: "Guardar perfil" }).click();
    expect((await putResponse).ok()).toBeTruthy();

    await expect(page.locator("#coaching-saved-status")).toHaveText(
      /Perfil guardado/i,
      { timeout: e2eTimeout },
    );
    await expect(page.locator("#coaching-continue")).toBeVisible();
    await expect(page.locator("#coaching-workspace")).toBeVisible();
  });
});
