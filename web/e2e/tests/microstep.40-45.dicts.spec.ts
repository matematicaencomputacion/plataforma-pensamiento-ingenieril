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

type FamilyStep = {
  micro: number;
  id: string;
  title: string;
  solution: string;
  nextUrl: RegExp;
  cursorAfter: string;
};

const FAMILY: FamilyStep[] = [
  {
    micro: 40,
    id: "py-40-dictionaries",
    title: "Python Dictionaries",
    solution: `thisdict = {"brand": "Ford", "model": "Mustang", "year": 1964}
print(len(thisdict))
`,
    nextUrl: /\/learn\/py-41-dict-access/,
    cursorAfter: "41",
  },
  {
    micro: 41,
    id: "py-41-dict-access",
    title: "Python Access Dictionary Items",
    solution: `thisdict = {"brand": "Ford", "model": "Mustang", "year": 1964}
print(thisdict["model"])
`,
    nextUrl: /\/learn\/py-42-dict-change/,
    cursorAfter: "42",
  },
  {
    micro: 42,
    id: "py-42-dict-change",
    title: "Python Change Dictionary Items",
    solution: `thisdict = {"brand": "Ford", "model": "Mustang", "year": 1964}
thisdict["year"] = 2018
print(thisdict["year"])
`,
    nextUrl: /\/learn\/py-43-dict-add/,
    cursorAfter: "43",
  },
  {
    micro: 43,
    id: "py-43-dict-add",
    title: "Python Add Dictionary Items",
    solution: `thisdict = {"brand": "Ford", "model": "Mustang", "year": 1964}
thisdict["color"] = "red"
print(thisdict["color"])
`,
    nextUrl: /\/learn\/py-44-dict-remove/,
    cursorAfter: "44",
  },
  {
    micro: 44,
    id: "py-44-dict-remove",
    title: "Python Remove Dictionary Items",
    solution: `thisdict = {"brand": "Ford", "model": "Mustang", "year": 1964}
thisdict.pop("model")
print("model" in thisdict)
`,
    nextUrl: /\/learn\/py-45-dict-loop/,
    cursorAfter: "45",
  },
  {
    micro: 45,
    id: "py-45-dict-loop",
    title: "Python Loop Dictionaries",
    solution: `thisdict = {"brand": "Ford", "model": "Mustang", "year": 1964}
for x in thisdict:
    print(x)
`,
    nextUrl: /\/learn\/py-46-dict-copy/,
    cursorAfter: "46",
  },
];

function uniqueCreds(micro: number) {
  const password = process.env.PPI_E2E_PASSWORD?.trim() || "secreto12ci";
  const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
  return { email: `e2e-ms${micro}-${stamp}@example.com`, password };
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

test.describe("micro-steps 40–45 · Dictionaries family", () => {
  test.beforeEach(async ({ page }) => {
    if (!useRealPyodide) {
      await installPyodideMock(page);
    }
  });

  for (const step of FAMILY) {
    test(`rail opens ${step.id}; pass advances chain`, async ({
      page,
      request,
    }: {
      page: Page;
      request: APIRequestContext;
    }) => {
      const { email, password } = uniqueCreds(step.micro);
      const reg = await request.post("/api/auth/register", {
        data: { email, password },
        timeout: e2eTimeout,
      });
      expect(reg.ok(), await reg.text()).toBeTruthy();
      const regJson = (await reg.json()) as { token: string };

      await login(page, email, password);
      await unlockThroughMicroStep(request, regJson.token, step.micro - 1);
      await page.reload();
      await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
      await expect(page.locator("#workspace-microsteps")).toHaveAttribute(
        "data-current-level",
        String(step.micro),
        { timeout: e2eTimeout },
      );

      await expect(
        page.locator(`#workspace-microstep-link-${step.micro}`),
      ).toBeVisible();
      const nextMicro = step.micro + 1;
      if (nextMicro <= 582) {
        await expect(
          page.locator(
            `#workspace-microsteps [data-microstep="${nextMicro}"]`,
          ),
        ).toHaveClass(/workspace__microstep--jumpable/);
      } else {
        await expect(
          page.locator(`#workspace-microstep-link-${nextMicro}`),
        ).toHaveCount(0);
      }

      await page.locator(`#workspace-microstep-link-${step.micro}`).click();
      await expect(page).toHaveURL(new RegExp(`/learn/${step.id}`), {
        timeout: e2eTimeout,
      });
      await expect(
        page.getByRole("heading", { name: step.title }),
      ).toBeVisible({ timeout: e2eTimeout });

      const engineTimeout = useRealPyodide ? 120_000 : e2eTimeout;
      await expect(page.locator("#learn-engine-status")).toHaveAttribute(
        "data-status",
        "ready",
        { timeout: engineTimeout },
      );

      await fillLeptosTextarea(page, "#learn-editor", step.solution);
      await page.getByRole("button", { name: "Validar solución" }).click();
      await expect(page.locator("#learn-progress-check")).toBeVisible({
        timeout: engineTimeout,
      });

      await page.locator("#learn-continue").click();
      await expect(page).toHaveURL(step.nextUrl, { timeout: e2eTimeout });

      if (!step.nextUrl.source.includes("workspace")) {
        await page
          .getByLabel("Navegación del Paso 2")
          .getByRole("link", { name: "Workspace" })
          .click();
        await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
      }

      await expect(page.locator("#workspace-microsteps")).toHaveAttribute(
        "data-current-level",
        step.cursorAfter,
      );
      const cell = page.locator(
        `#workspace-microsteps [data-microstep="${step.micro}"]`,
      );
      await expect(cell).toHaveClass(/workspace__microstep--done/);
      await expect(cell.locator(".workspace__microstep-badge")).toBeVisible();
    });
  }
});
