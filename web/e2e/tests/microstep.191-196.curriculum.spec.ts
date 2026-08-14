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
  { micro: 191, id: "py-191-kth-largest", title: "DSA Kth Largest", solution: `import heapq

def find_kth_largest(nums, k):
    return heapq.nlargest(k, nums)[-1]
print(find_kth_largest([3, 2, 1, 5, 6, 4], 2))
`, nextUrl: /\/learn\/py-192-top-k-frequent/, cursorAfter: "192" },
  { micro: 192, id: "py-192-top-k-frequent", title: "DSA Top K Frequent", solution: `from collections import Counter

def top_k_frequent(nums, k):
    counts = Counter(nums)
    return sorted(n for n, _ in counts.most_common(k))
print(top_k_frequent([1, 1, 1, 2, 2, 3], 2))
`, nextUrl: /\/learn\/py-193-merge-k-lists/, cursorAfter: "193" },
  { micro: 193, id: "py-193-merge-k-lists", title: "DSA Merge K Lists", solution: `import heapq

def merge_k_lists(lists):
    heap = []
    for i, lst in enumerate(lists):
        if lst:
            heapq.heappush(heap, (lst[0], i, 0))
    result = []
    while heap:
        value, list_i, idx = heapq.heappop(heap)
        result.append(value)
        if idx + 1 < len(lists[list_i]):
            heapq.heappush(heap, (lists[list_i][idx + 1], list_i, idx + 1))
    return result
print(merge_k_lists([[1, 4, 5], [1, 3, 4], [2, 6]]))
`, nextUrl: /\/learn\/py-194-meeting-rooms/, cursorAfter: "194" },
  { micro: 194, id: "py-194-meeting-rooms", title: "DSA Meeting Rooms", solution: `def can_attend_meetings(intervals):
    intervals = sorted(intervals)
    for i in range(1, len(intervals)):
        if intervals[i][0] < intervals[i - 1][1]:
            return False
    return True
print(can_attend_meetings([[0, 30], [5, 10], [15, 20]]))
`, nextUrl: /\/learn\/py-195-ugly-number/, cursorAfter: "195" },
  { micro: 195, id: "py-195-ugly-number", title: "DSA Ugly Number", solution: `def nth_ugly_number(n):
    ugly = [1]
    i2 = i3 = i5 = 0
    while len(ugly) < n:
        n2, n3, n5 = ugly[i2] * 2, ugly[i3] * 3, ugly[i5] * 5
        nxt = min(n2, n3, n5)
        ugly.append(nxt)
        if nxt == n2:
            i2 += 1
        if nxt == n3:
            i3 += 1
        if nxt == n5:
            i5 += 1
    return ugly[-1]
print(nth_ugly_number(10))
`, nextUrl: /\/learn\/py-196-k-closest/, cursorAfter: "196" },
  { micro: 196, id: "py-196-k-closest", title: "DSA K Closest Points", solution: `def k_closest(points, k):
    chosen = sorted(points, key=lambda p: p[0] * p[0] + p[1] * p[1])[:k]
    return sorted(chosen)
print(k_closest([[1, 3], [-2, 2], [2, -2]], 2))
`, nextUrl: /\/learn\/py-197-coin-change-ii/, cursorAfter: "197" },
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
      if (nextMicro <= 522) {
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
