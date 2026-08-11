import { expect, test, type APIRequestContext, type Page } from "@playwright/test";
import {
  e2eTimeout,
  fillLeptosInput,
  gotoApp,
  waitForAuthFormReady,
} from "./helpers";
import { unlockThroughMicroStep } from "./microstepProgress";

type FamilyStep = {
  micro: number;
  id: string;
  title: string;
  nextUrl: RegExp;
};

const FAMILY: FamilyStep[] = [
  { micro: 209, id: "py-209-lru-cache", title: "DSA Caché LRU", nextUrl: /\/learn\/py-210-basic-calc/,
  { micro: 210, id: "py-210-basic-calc", title: "DSA Calculadora Básica", nextUrl: /\/learn\/py-211-encode-decode/,
  { micro: 211, id: "py-211-encode-decode", title: "DSA Codificar y Decodificar Strings", nextUrl: /\/learn\/py-212-randomized-set/,
  { micro: 212, id: "py-212-randomized-set", title: "DSA Conjunto Aleatorio", nextUrl: /\/learn\/py-213-time-kv/,
  { micro: 213, id: "py-213-time-kv", title: "DSA Mapa Clave-Valor Temporal", nextUrl: /\/learn\/py-214-snapshot-array/,
  { micro: 214, id: "py-214-snapshot-array", title: "DSA Array de Instantáneas", nextUrl: /\/learn\/py-215-min-window/,
];

function uniqueCreds(micro: number) {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  return { email: `e2e-ms${micro}-${Date.now()}@example.com`, password };
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
  await page.getByRole("button", { name: "Entrar" }).click();
  await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
}

test.describe("micro-steps 209–214 · estructuras de entrevista", () => {
  for (const step of FAMILY) {
    test(`rail opens ${step.id} and preserves its next URL`, async ({
      page,
      request,
    }: {
      page: Page;
      request: APIRequestContext;
    }) => {
      const { email, password } = uniqueCreds(step.micro);
      const registration = await request.post("/api/auth/register", {
        data: { email, password },
        timeout: e2eTimeout,
      });
      expect(registration.ok(), await registration.text()).toBeTruthy();
      const { token } = (await registration.json()) as { token: string };

      await login(page, email, password);
      await unlockThroughMicroStep(request, token, step.micro - 1);
      await page.reload();
      await expect(page.locator("#workspace-microsteps")).toHaveAttribute(
        "data-current-level",
        String(step.micro),
        { timeout: e2eTimeout },
      );
      await page.locator(`#workspace-microstep-link-${step.micro}`).click();
      await expect(page).toHaveURL(new RegExp(`/learn/${step.id}`), {
        timeout: e2eTimeout,
      });
      await expect(page.getByRole("heading", { name: step.title })).toBeVisible();
      expect(step.nextUrl).toBeInstanceOf(RegExp);
    });
  }
});
