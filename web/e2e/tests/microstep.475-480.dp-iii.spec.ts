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
    micro: 475,
    id: "py-475-min-cost-stairs",
    title: "DSA Min Cost Stairs",
    solution: `def min_cost_climbing_stairs(cost):
    a = b = 0
    for c in cost:
        a, b = b, c + min(a, b)
    return min(a, b)

print(min_cost_climbing_stairs([10, 15, 20]))
`,
    nextUrl: /\/learn\/py-476-unique-paths/,
    cursorAfter: "476",
  },
  {
    micro: 476,
    id: "py-476-unique-paths",
    title: "DSA Unique Paths",
    solution: `def unique_paths(m, n):
    row = [1] * n
    for _ in range(1, m):
        for j in range(1, n):
            row[j] += row[j - 1]
    return row[-1]

print(unique_paths(3, 7))
`,
    nextUrl: /\/learn\/py-477-min-path-sum/,
    cursorAfter: "477",
  },
  {
    micro: 477,
    id: "py-477-min-path-sum",
    title: "DSA Min Path Sum",
    solution: `def min_path_sum(grid):
    m, n = len(grid), len(grid[0])
    dp = [0] * n
    for i in range(m):
        for j in range(n):
            if i == 0 and j == 0:
                dp[j] = grid[i][j]
            elif i == 0:
                dp[j] = dp[j - 1] + grid[i][j]
            elif j == 0:
                dp[j] = dp[j] + grid[i][j]
            else:
                dp[j] = min(dp[j], dp[j - 1]) + grid[i][j]
    return dp[-1]

print(min_path_sum([[1,3,1],[1,5,1],[4,2,1]]))
`,
    nextUrl: /\/learn\/py-478-integer-break/,
    cursorAfter: "478",
  },
  {
    micro: 478,
    id: "py-478-integer-break",
    title: "DSA Integer Break",
    solution: `def integer_break(n):
    if n <= 3:
        return n - 1
    q, r = divmod(n, 3)
    if r == 0:
        return 3 ** q
    if r == 1:
        return 3 ** (q - 1) * 4
    return 3 ** q * 2

print(integer_break(10))
`,
    nextUrl: /\/learn\/py-479-decode-ways/,
    cursorAfter: "479",
  },
  {
    micro: 479,
    id: "py-479-decode-ways",
    title: "DSA Decode Ways",
    solution: `def num_decodings(s):
    if not s or s[0] == "0":
        return 0
    a, b = 1, 1
    for i in range(1, len(s)):
        cur = 0
        if s[i] != "0":
            cur += b
        two = int(s[i - 1:i + 1])
        if 10 <= two <= 26:
            cur += a
        a, b = b, cur
    return b

print(num_decodings("226"))
`,
    nextUrl: /\/learn\/py-480-rob-circle/,
    cursorAfter: "480",
  },
  {
    micro: 480,
    id: "py-480-rob-circle",
    title: "DSA Rob Circle",
    solution: `def rob(nums):
    def linear(arr):
        a = b = 0
        for x in arr:
            a, b = b, max(b, a + x)
        return b
    if len(nums) == 1:
        return nums[0]
    return max(linear(nums[:-1]), linear(nums[1:]))

print(rob([2, 3, 2]))
`,
    nextUrl: /\/learn\/py-481-last-stone/,
    cursorAfter: "481",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:475|476|477|478|479|480)-/);
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

test.describe("micro-steps 475–480 · DP III", () => {
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
