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
  { micro: 191, id: "py-191-kth-largest", title: "DSA Kth Largest Element", solution: `import heapq
def find_kth_largest(nums, k):
    return heapq.nlargest(k, nums)[-1]
print(find_kth_largest([3, 2, 1, 5, 6, 4], 2))
`, nextUrl: /\/learn\/py-192-top-k-frequent/, cursorAfter: "192" },
  { micro: 192, id: "py-192-top-k-frequent", title: "DSA Top K Frequent", solution: `from collections import Counter
import heapq
def top_k_frequent(nums, k):
    return [value for value, _ in heapq.nlargest(k, Counter(nums).items(), key=lambda item: item[1])]
print(sorted(top_k_frequent([1, 1, 1, 2, 2, 3], 2)))
`, nextUrl: /\/learn\/py-193-merge-k-lists/, cursorAfter: "193" },
  { micro: 193, id: "py-193-merge-k-lists", title: "DSA Merge K Sorted Lists", solution: `import heapq
def merge_k_lists(lists):
    heap = [(row[0], i, 0) for i, row in enumerate(lists) if row]
    heapq.heapify(heap)
    merged = []
    while heap:
        value, row, col = heapq.heappop(heap)
        merged.append(value)
        if col + 1 < len(lists[row]):
            heapq.heappush(heap, (lists[row][col + 1], row, col + 1))
    return merged
print(merge_k_lists([[1, 4, 5], [1, 3, 4], [2, 6]]))
`, nextUrl: /\/learn\/py-194-can-attend-meetings/, cursorAfter: "194" },
  { micro: 194, id: "py-194-can-attend-meetings", title: "DSA Can Attend Meetings", solution: `def can_attend_meetings(intervals):
    intervals = sorted(intervals)
    return all(intervals[i - 1][1] <= intervals[i][0] for i in range(1, len(intervals)))
print(can_attend_meetings([[0, 30], [5, 10], [15, 20]]))
`, nextUrl: /\/learn\/py-195-nth-ugly/, cursorAfter: "195" },
  { micro: 195, id: "py-195-nth-ugly", title: "DSA Nth Ugly Number", solution: `def nth_ugly_number(n):
    ugly = [1]
    i2 = i3 = i5 = 0
    while len(ugly) < n:
        candidate = min(ugly[i2] * 2, ugly[i3] * 3, ugly[i5] * 5)
        ugly.append(candidate)
        if candidate == ugly[i2] * 2: i2 += 1
        if candidate == ugly[i3] * 3: i3 += 1
        if candidate == ugly[i5] * 5: i5 += 1
    return ugly[-1]
print(nth_ugly_number(10))
`, nextUrl: /\/learn\/py-196-k-closest/, cursorAfter: "196" },
  { micro: 196, id: "py-196-k-closest", title: "DSA K Closest Points", solution: `import heapq
def k_closest(points, k):
    return heapq.nsmallest(k, points, key=lambda point: point[0] ** 2 + point[1] ** 2)
print(sorted(k_closest([[1, 3], [-2, 2], [5, 8]], 2)))
`, nextUrl: /\/learn\/py-197-coin-change-ii/, cursorAfter: "197" },
  { micro: 197, id: "py-197-coin-change-ii", title: "DP Coin Change II", solution: `def change(amount, coins):
    dp = [0] * (amount + 1)
    dp[0] = 1
    for coin in coins:
        for value in range(coin, amount + 1):
            dp[value] += dp[value - coin]
    return dp[amount]
print(change(5, [1, 2, 5]))
`, nextUrl: /\/learn\/py-198-rob-circular/, cursorAfter: "198" },
  { micro: 198, id: "py-198-rob-circular", title: "DP House Robber II", solution: `def rob_circular(nums):
    def rob_line(houses):
        prev2 = prev1 = 0
        for value in houses:
            prev2, prev1 = prev1, max(prev1, prev2 + value)
        return prev1
    if len(nums) <= 1:
        return nums[0] if nums else 0
    return max(rob_line(nums[:-1]), rob_line(nums[1:]))
print(rob_circular([2, 3, 2]))
`, nextUrl: /\/learn\/py-199-unique-paths-obstacles/, cursorAfter: "199" },
  { micro: 199, id: "py-199-unique-paths-obstacles", title: "DP Unique Paths Obstacles", solution: `def unique_paths_with_obstacles(grid):
    dp = [0] * len(grid[0])
    dp[0] = 1
    for row in grid:
        for col, cell in enumerate(row):
            if cell:
                dp[col] = 0
            elif col:
                dp[col] += dp[col - 1]
    return dp[-1]
print(unique_paths_with_obstacles([[0, 0, 0], [0, 1, 0], [0, 0, 0]]))
`, nextUrl: /\/learn\/py-200-lcs-length/, cursorAfter: "200" },
  { micro: 200, id: "py-200-lcs-length", title: "DP Longest Common Subsequence", solution: `def lcs_length(text1, text2):
    dp = [0] * (len(text2) + 1)
    for left in text1:
        previous = 0
        for j, right in enumerate(text2, 1):
            saved = dp[j]
            dp[j] = previous + 1 if left == right else max(dp[j], dp[j - 1])
            previous = saved
    return dp[-1]
print(lcs_length('abcde', 'ace'))
`, nextUrl: /\/learn\/py-201-can-partition/, cursorAfter: "201" },
  { micro: 201, id: "py-201-can-partition", title: "DP Partition Equal Subset", solution: `def can_partition(nums):
    total = sum(nums)
    if total % 2:
        return False
    target = total // 2
    possible = {0}
    for value in nums:
        possible |= {current + value for current in possible if current + value <= target}
    return target in possible
print(can_partition([1, 5, 11, 5]))
`, nextUrl: /\/learn\/py-202-num-squares/, cursorAfter: "202" },
  { micro: 202, id: "py-202-num-squares", title: "DP Perfect Squares", solution: `def num_squares(n):
    dp = [0] + [n] * n
    for value in range(1, n + 1):
        square = 1
        while square * square <= value:
            dp[value] = min(dp[value], dp[value - square * square] + 1)
            square += 1
    return dp[n]
print(num_squares(12))
`, nextUrl: /\/workspace/, cursorAfter: "203" },
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

test.describe("micro-steps 191–202 · heaps / priority / DP", () => {
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
      await expect(page.locator("#workspace-microsteps")).toHaveAttribute(
        "data-current-level",
        String(step.micro),
        { timeout: e2eTimeout },
      );
      await expect(page.locator(`#workspace-microstep-link-${step.micro}`)).toBeVisible();
      await expect(page.locator(`#workspace-microstep-link-${step.micro + 1}`)).toHaveCount(0);

      await page.locator(`#workspace-microstep-link-${step.micro}`).click();
      await expect(page).toHaveURL(new RegExp(`/learn/${step.id}`), { timeout: e2eTimeout });
      await expect(page.getByRole("heading", { name: step.title })).toBeVisible({ timeout: e2eTimeout });

      const engineTimeout = useRealPyodide ? 120_000 : e2eTimeout;
      await expect(page.locator("#learn-engine-status")).toHaveAttribute(
        "data-status",
        "ready",
        { timeout: engineTimeout },
      );
      await fillLeptosTextarea(page, "#learn-editor", step.solution);
      await page.getByRole("button", { name: "Validar solución" }).click();
      await expect(page.locator("#learn-progress-check")).toBeVisible({ timeout: engineTimeout });
      await page.locator("#learn-continue").click();
      await expect(page).toHaveURL(step.nextUrl, { timeout: e2eTimeout });

      if (!step.nextUrl.source.includes("workspace")) {
        await page.getByLabel("Navegación del Paso 2").getByRole("link", { name: "Workspace" }).click();
        await expect(page).toHaveURL(/\/workspace/, { timeout: e2eTimeout });
      }

      await expect(page.locator("#workspace-microsteps")).toHaveAttribute(
        "data-current-level",
        step.cursorAfter,
      );
      const cell = page.locator(`#workspace-microsteps [data-microstep="${step.micro}"]`);
      await expect(cell).toHaveClass(/workspace__microstep--done/);
      await expect(cell.locator(".workspace__microstep-badge")).toBeVisible();
    });
  }
});
