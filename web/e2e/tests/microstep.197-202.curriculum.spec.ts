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
  { micro: 197, id: "py-197-coin-change-ii", title: "DSA Coin Change II", solution: `def coin_change_ways(amount, coins):
    dp = [0] * (amount + 1)
    dp[0] = 1
    for coin in coins:
        for a in range(coin, amount + 1):
            dp[a] += dp[a - coin]
    return dp[amount]
print(coin_change_ways(5, [1, 2, 5]))
`, nextUrl: /\/learn\/py-198-house-robber-ii/, cursorAfter: "198" },
  { micro: 198, id: "py-198-house-robber-ii", title: "DSA House Robber II", solution: `def rob_circular(nums):
    def rob_line(arr):
        prev = cur = 0
        for value in arr:
            prev, cur = cur, max(cur, prev + value)
        return cur
    if not nums:
        return 0
    if len(nums) == 1:
        return nums[0]
    return max(rob_line(nums[:-1]), rob_line(nums[1:]))
print(rob_circular([2, 3, 2]))
`, nextUrl: /\/learn\/py-199-unique-paths-ii/, cursorAfter: "199" },
  { micro: 199, id: "py-199-unique-paths-ii", title: "DSA Unique Paths II", solution: `def unique_paths_with_obstacles(obstacle_grid):
    m, n = len(obstacle_grid), len(obstacle_grid[0])
    dp = [[0] * n for _ in range(m)]
    if obstacle_grid[0][0] == 1:
        return 0
    dp[0][0] = 1
    for i in range(m):
        for j in range(n):
            if obstacle_grid[i][j] == 1:
                dp[i][j] = 0
                continue
            if i == 0 and j == 0:
                continue
            from_up = dp[i - 1][j] if i else 0
            from_left = dp[i][j - 1] if j else 0
            dp[i][j] = from_up + from_left
    return dp[-1][-1]
print(unique_paths_with_obstacles([[0, 0, 0], [0, 1, 0], [0, 0, 0]]))
`, nextUrl: /\/learn\/py-200-max-product/, cursorAfter: "200" },
  { micro: 200, id: "py-200-max-product", title: "DSA Max Product Subarray", solution: `def max_product(nums):
    best = imax = imin = nums[0]
    for value in nums[1:]:
        candidates = (value, imax * value, imin * value)
        imax = max(candidates)
        imin = min(candidates)
        best = max(best, imax)
    return best
print(max_product([2, 3, -2, 4]))
`, nextUrl: /\/learn\/py-201-partition-subset/, cursorAfter: "201" },
  { micro: 201, id: "py-201-partition-subset", title: "DSA Partition Equal Subset", solution: `def can_partition(nums):
    total = sum(nums)
    if total % 2:
        return False
    target = total // 2
    reachable = 1
    for value in nums:
        reachable |= reachable << value
    return bool(reachable & (1 << target))
print(can_partition([1, 5, 11, 5]))
`, nextUrl: /\/learn\/py-202-perfect-squares/, cursorAfter: "202" },
  { micro: 202, id: "py-202-perfect-squares", title: "DSA Perfect Squares", solution: `def num_squares(n):
    dp = [0] + [float('inf')] * n
    for i in range(1, n + 1):
        j = 1
        while j * j <= i:
            dp[i] = min(dp[i], dp[i - j * j] + 1)
            j += 1
    return int(dp[n])
print(num_squares(12))
`, nextUrl: /\/learn\/py-203-num-islands/, cursorAfter: "203" },
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

test.describe("micro-steps 179–184 · stacks / deque", () => {
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
      if (nextMicro <= 594) {
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
