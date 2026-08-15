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
    micro: 613,
    id: "py-613-sliding-max",
    title: "DSA Mono Deque · Sliding Maximum",
    solution: "from collections import deque\n\ndef max_sliding_window(nums, k):\n    q = deque()\n    out = []\n    for i, value in enumerate(nums):\n        while q and nums[q[-1]] <= value:\n            q.pop()\n        q.append(i)\n        if q[0] <= i - k:\n            q.popleft()\n        if i >= k - 1:\n            out.append(nums[q[0]])\n    return out\n\nprint(max_sliding_window([1, 3, -1, -3, 5, 3, 6, 7], 3))\n",
    nextUrl: /\/learn\/py-614-sliding-min/,
    cursorAfter: "614",
  },
  {
    micro: 614,
    id: "py-614-sliding-min",
    title: "DSA Mono Deque · Sliding Minimum",
    solution: "from collections import deque\n\ndef min_sliding_window(nums, k):\n    q = deque()\n    out = []\n    for i, value in enumerate(nums):\n        while q and nums[q[-1]] >= value:\n            q.pop()\n        q.append(i)\n        if q[0] <= i - k:\n            q.popleft()\n        if i >= k - 1:\n            out.append(nums[q[0]])\n    return out\n\nprint(min_sliding_window([1, 3, -1, -3, 5, 3, 6, 7], 3))\n",
    nextUrl: /\/learn\/py-615-shortest-subarray/,
    cursorAfter: "615",
  },
  {
    micro: 615,
    id: "py-615-shortest-subarray",
    title: "DSA Mono Deque · Shortest Subarray",
    solution: "from collections import deque\n\ndef shortest_subarray(nums, k):\n    prefix = [0]\n    for value in nums:\n        prefix.append(prefix[-1] + value)\n    q = deque()\n    best = len(nums) + 1\n    for i, total in enumerate(prefix):\n        while q and total - prefix[q[0]] >= k:\n            best = min(best, i - q.popleft())\n        while q and prefix[q[-1]] >= total:\n            q.pop()\n        q.append(i)\n    return best if best <= len(nums) else -1\n\nprint(shortest_subarray([2, -1, 2], 3))\n",
    nextUrl: /\/learn\/py-616-constrained-subseq/,
    cursorAfter: "616",
  },
  {
    micro: 616,
    id: "py-616-constrained-subseq",
    title: "DSA Mono Deque · Constrained Subseq",
    solution: "from collections import deque\n\ndef constrained_subset_sum(nums, k):\n    n = len(nums)\n    dp = nums[:]\n    q = deque()\n    best = nums[0]\n    for i in range(n):\n        if q and q[0] < i - k:\n            q.popleft()\n        if q:\n            dp[i] = max(dp[i], nums[i] + dp[q[0]])\n        best = max(best, dp[i])\n        while q and dp[q[-1]] <= dp[i]:\n            q.pop()\n        q.append(i)\n    return best\n\nprint(constrained_subset_sum([10, 2, -10, 5, 20], 2))\n",
    nextUrl: /\/learn\/py-617-jump-game-vi/,
    cursorAfter: "617",
  },
  {
    micro: 617,
    id: "py-617-jump-game-vi",
    title: "DSA Mono Deque · Jump Game VI",
    solution: "from collections import deque\n\ndef max_result(nums, k):\n    n = len(nums)\n    dp = [0] * n\n    dp[0] = nums[0]\n    q = deque([0])\n    for i in range(1, n):\n        while q and q[0] < i - k:\n            q.popleft()\n        dp[i] = nums[i] + dp[q[0]]\n        while q and dp[q[-1]] <= dp[i]:\n            q.pop()\n        q.append(i)\n    return dp[-1]\n\nprint(max_result([1, -1, -2, 4, -7, 3], 2))\n",
    nextUrl: /\/learn\/py-618-longest-cont-subarray/,
    cursorAfter: "618",
  },
  {
    micro: 618,
    id: "py-618-longest-cont-subarray",
    title: "DSA Mono Deque · Limit Difference",
    solution: "from collections import deque\n\ndef longest_subarray(nums, limit):\n    maxq, minq = deque(), deque()\n    left = best = 0\n    for right, value in enumerate(nums):\n        while maxq and nums[maxq[-1]] < value:\n            maxq.pop()\n        while minq and nums[minq[-1]] > value:\n            minq.pop()\n        maxq.append(right)\n        minq.append(right)\n        while nums[maxq[0]] - nums[minq[0]] > limit:\n            if maxq[0] == left:\n                maxq.popleft()\n            if minq[0] == left:\n                minq.popleft()\n            left += 1\n        best = max(best, right - left + 1)\n    return best\n\nprint(longest_subarray([8, 2, 4, 7], 4))\n",
    nextUrl: /\/learn\/py-619-equations-possible/,
    cursorAfter: "619",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:613|614|615|616|617|618)-/);
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

test.describe("micro-steps 613–618 · mono deque", () => {
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

      if (step.micro < 618) {
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
