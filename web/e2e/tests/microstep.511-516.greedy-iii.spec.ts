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
    micro: 511,
    id: "py-511-jump-game",
    title: "DSA Jump Game",
    solution: `def can_jump(nums):
    reach = 0
    for i, x in enumerate(nums):
        if i > reach:
            return False
        reach = max(reach, i + x)
    return True

print(can_jump([2, 3, 1, 1, 4]))
`,
    nextUrl: /\/learn\/py-512-jump-game-ii/,
    cursorAfter: "512",
  },
  {
    micro: 512,
    id: "py-512-jump-game-ii",
    title: "DSA Jump Game II",
    solution: `def jump(nums):
    jumps = end = far = 0
    for i in range(len(nums) - 1):
        far = max(far, i + nums[i])
        if i == end:
            jumps += 1
            end = far
    return jumps

print(jump([2, 3, 1, 1, 4]))
`,
    nextUrl: /\/learn\/py-513-can-place-flowers/,
    cursorAfter: "513",
  },
  {
    micro: 513,
    id: "py-513-can-place-flowers",
    title: "DSA Place Flowers",
    solution: `def can_place_flowers(flowerbed, n):
    bed = [0] + flowerbed + [0]
    for i in range(1, len(bed) - 1):
        if bed[i - 1] == bed[i] == bed[i + 1] == 0:
            bed[i] = 1
            n -= 1
    return n <= 0

print(can_place_flowers([1, 0, 0, 0, 1], 1))
`,
    nextUrl: /\/learn\/py-514-lemonade/,
    cursorAfter: "514",
  },
  {
    micro: 514,
    id: "py-514-lemonade",
    title: "DSA Lemonade",
    solution: `def lemonade_change(bills):
    five = ten = 0
    for b in bills:
        if b == 5:
            five += 1
        elif b == 10:
            if not five:
                return False
            five -= 1; ten += 1
        else:
            if ten and five:
                ten -= 1; five -= 1
            elif five >= 3:
                five -= 3
            else:
                return False
    return True

print(lemonade_change([5, 5, 5, 10, 20]))
`,
    nextUrl: /\/learn\/py-515-best-time-stock/,
    cursorAfter: "515",
  },
  {
    micro: 515,
    id: "py-515-best-time-stock",
    title: "DSA Best Time Stock",
    solution: `def max_profit(prices):
    mn = prices[0]
    best = 0
    for p in prices:
        mn = min(mn, p)
        best = max(best, p - mn)
    return best

print(max_profit([7, 1, 5, 3, 6, 4]))
`,
    nextUrl: /\/learn\/py-516-best-time-stock-ii/,
    cursorAfter: "516",
  },
  {
    micro: 516,
    id: "py-516-best-time-stock-ii",
    title: "DSA Stock II",
    solution: `def max_profit(prices):
    return sum(max(prices[i] - prices[i - 1], 0) for i in range(1, len(prices)))

print(max_profit([7, 1, 5, 3, 6, 4]))
`,
    nextUrl: /\/workspace/,
    cursorAfter: "517",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:511|512|513|514|515|516)-/);
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

test.describe("micro-steps 511–516 · greedy III", () => {
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
      if (nextMicro <= 558) {
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
