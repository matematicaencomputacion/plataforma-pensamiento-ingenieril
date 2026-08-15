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
    micro: 643,
    id: "py-643-coin-change-ii",
    title: "DSA DP Knapsack · Coin Change II",
    solution: "def change(amount, coins):\n    dp = [0] * (amount + 1)\n    dp[0] = 1\n    for coin in coins:\n        for x in range(coin, amount + 1):\n            dp[x] += dp[x - coin]\n    return dp[amount]\n\nprint(change(5, [1, 2, 5]))\n",
    nextUrl: /\/learn\/py-644-target-sum/,
    cursorAfter: "644",
  },
  {
    micro: 644,
    id: "py-644-target-sum",
    title: "DSA DP Knapsack · Target Sum",
    solution: "def find_target_sum_ways(nums, target):\n    total = sum(nums)\n    if (total + target) % 2 or abs(target) > total:\n        return 0\n    need = (total + target) // 2\n    dp = [0] * (need + 1)\n    dp[0] = 1\n    for x in nums:\n        for s in range(need, x - 1, -1):\n            dp[s] += dp[s - x]\n    return dp[need]\n\nprint(find_target_sum_ways([1, 1, 1, 1, 1], 3))\n",
    nextUrl: /\/learn\/py-645-last-stone-ii/,
    cursorAfter: "645",
  },
  {
    micro: 645,
    id: "py-645-last-stone-ii",
    title: "DSA DP Knapsack · Last Stone II",
    solution: "def last_stone_weight_ii(stones):\n    total = sum(stones)\n    need = total // 2\n    dp = [False] * (need + 1)\n    dp[0] = True\n    for x in stones:\n        for s in range(need, x - 1, -1):\n            dp[s] = dp[s] or dp[s - x]\n    for s in range(need, -1, -1):\n        if dp[s]:\n            return total - 2 * s\n    return total\n\nprint(last_stone_weight_ii([2, 7, 4, 1, 8, 1]))\n",
    nextUrl: /\/learn\/py-646-ones-and-zeroes/,
    cursorAfter: "646",
  },
  {
    micro: 646,
    id: "py-646-ones-and-zeroes",
    title: "DSA DP Knapsack · Ones and Zeroes",
    solution: "def find_max_form(strs, m, n):\n    dp = [[0] * (n + 1) for _ in range(m + 1)]\n    for s in strs:\n        zeros = s.count('0')\n        ones = len(s) - zeros\n        for j in range(m, zeros - 1, -1):\n            for k in range(n, ones - 1, -1):\n                dp[j][k] = max(dp[j][k], 1 + dp[j - zeros][k - ones])\n    return dp[m][n]\n\nprint(find_max_form(['10', '0001', '111001', '1', '0'], 5, 3))\n",
    nextUrl: /\/learn\/py-647-combination-sum-iv/,
    cursorAfter: "647",
  },
  {
    micro: 647,
    id: "py-647-combination-sum-iv",
    title: "DSA DP Knapsack · Combination Sum IV",
    solution: "def combination_sum4(nums, target):\n    dp = [0] * (target + 1)\n    dp[0] = 1\n    for x in range(1, target + 1):\n        for num in nums:\n            if num <= x:\n                dp[x] += dp[x - num]\n    return dp[target]\n\nprint(combination_sum4([1, 2, 3], 4))\n",
    nextUrl: /\/learn\/py-648-can-partition/,
    cursorAfter: "648",
  },
  {
    micro: 648,
    id: "py-648-can-partition",
    title: "DSA DP Knapsack · Equal Partition",
    solution: "def can_partition(nums):\n    total = sum(nums)\n    if total % 2:\n        return False\n    need = total // 2\n    dp = [False] * (need + 1)\n    dp[0] = True\n    for x in nums:\n        for s in range(need, x - 1, -1):\n            dp[s] = dp[s] or dp[s - x]\n    return dp[need]\n\nprint(can_partition([1, 5, 11, 5]))\n",
    nextUrl: /\/learn\/py-649-num-distinct/,
    cursorAfter: "649",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:643|644|645|646|647|648)-/);
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

test.describe("micro-steps 643–648 · dp knapsack", () => {
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
      if (nextMicro <= 1000) {
        await expect(
          page.locator(
            `#workspace-microsteps [data-microstep="${nextMicro}"]`,
          ),
        ).toHaveClass(/workspace__microstep--open|workspace__microstep--jumpable/);
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

      if (step.micro < 648) {
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
