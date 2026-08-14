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
    micro: 457,
    id: "py-457-defang-ip",
    title: "DSA Defang IP",
    solution: `def defang_ipaddr(address):
    return address.replace(".", "[.]")

print(defang_ipaddr("1.1.1.1"))
`,
    nextUrl: /\/learn\/py-458-goal-parser/,
    cursorAfter: "458",
  },
  {
    micro: 458,
    id: "py-458-goal-parser",
    title: "DSA Goal Parser",
    solution: `def interpret(command):
    return command.replace("()", "o").replace("(al)", "al")

print(interpret("G()(al)"))
`,
    nextUrl: /\/learn\/py-459-shuffle-string/,
    cursorAfter: "459",
  },
  {
    micro: 459,
    id: "py-459-shuffle-string",
    title: "DSA Shuffle String",
    solution: `def restore_string(s, indices):
    out = [""] * len(s)
    for ch, i in zip(s, indices):
        out[i] = ch
    return "".join(out)

print(restore_string("codeleet", [4, 5, 6, 7, 0, 2, 1, 3]))
`,
    nextUrl: /\/learn\/py-460-count-matches/,
    cursorAfter: "460",
  },
  {
    micro: 460,
    id: "py-460-count-matches",
    title: "DSA Count Matches",
    solution: `def count_matches(items, rule_key, rule_value):
    idx = {"type": 0, "color": 1, "name": 2}[rule_key]
    return sum(1 for it in items if it[idx] == rule_value)

print(count_matches([["phone", "blue", "pixel"], ["computer", "silver", "lenovo"], ["phone", "gold", "iphone"]], "color", "silver"))
`,
    nextUrl: /\/learn\/py-461-split-balanced/,
    cursorAfter: "461",
  },
  {
    micro: 461,
    id: "py-461-split-balanced",
    title: "DSA Split Balanced",
    solution: `def balanced_string_split(s):
    bal = ans = 0
    for ch in s:
        bal += 1 if ch == "R" else -1
        if bal == 0:
            ans += 1
    return ans

print(balanced_string_split("RLRRLLRLRL"))
`,
    nextUrl: /\/learn\/py-462-max-words/,
    cursorAfter: "462",
  },
  {
    micro: 462,
    id: "py-462-max-words",
    title: "DSA Max Words",
    solution: `def most_words_found(sentences):
    return max(len(s.split()) for s in sentences)

print(most_words_found(["alice and bob love leetcode", "i think so too", "this is great thanks very much"]))
`,
    nextUrl: /\/learn\/py-463-max-depth-bt/,
    cursorAfter: "463",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:457|458|459|460|461|462)-/);
    expect(step.nextUrl).toBeInstanceOf(RegExp);
  }
});

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

test.describe("micro-steps 457–462 · strings III", () => {
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
