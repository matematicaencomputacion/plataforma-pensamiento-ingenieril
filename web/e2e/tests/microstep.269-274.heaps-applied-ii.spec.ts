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
    micro: 269,
    id: "py-269-last-stone",
    title: "DSA Last Stone Weight",
    solution: `import heapq

def last_stone_weight(stones):
    heap = [-s for s in stones]
    heapq.heapify(heap)
    while len(heap) > 1:
        a = -heapq.heappop(heap)
        b = -heapq.heappop(heap)
        if a != b:
            heapq.heappush(heap, -(a - b))
    return -heap[0] if heap else 0

print(last_stone_weight([2, 7, 4, 1, 8, 1]))
`,
    nextUrl: /\/learn\/py-270-task-scheduler/,
    cursorAfter: "270",
  },
  {
    micro: 270,
    id: "py-270-task-scheduler",
    title: "DSA Task Scheduler",
    solution: `from collections import Counter

def least_interval(tasks, n):
    freqs = list(Counter(tasks).values())
    max_f = max(freqs)
    count_max = freqs.count(max_f)
    return max(len(tasks), (max_f - 1) * (n + 1) + count_max)

print(least_interval(["A", "A", "A", "B", "B", "B"], 2))
`,
    nextUrl: /\/learn\/py-271-reorganize-string/,
    cursorAfter: "271",
  },
  {
    micro: 271,
    id: "py-271-reorganize-string",
    title: "DSA Reorganize String",
    solution: `import heapq
from collections import Counter

def reorganize_string(s):
    heap = [(-c, ch) for ch, c in Counter(s).items()]
    heapq.heapify(heap)
    out = []
    prev = (0, "")
    while heap:
        count, ch = heapq.heappop(heap)
        out.append(ch)
        if prev[0] < 0:
            heapq.heappush(heap, prev)
        prev = (count + 1, ch)
    ans = "".join(out)
    return ans if len(ans) == len(s) else ""

print(reorganize_string("aab"))
`,
    nextUrl: /\/learn\/py-272-find-median/,
    cursorAfter: "272",
  },
  {
    micro: 272,
    id: "py-272-find-median",
    title: "DSA Find Median Stream",
    solution: `import heapq

class MedianFinder:
    def __init__(self):
        self.lo = []
        self.hi = []

    def add_num(self, num):
        heapq.heappush(self.lo, -num)
        heapq.heappush(self.hi, -heapq.heappop(self.lo))
        if len(self.hi) > len(self.lo):
            heapq.heappush(self.lo, -heapq.heappop(self.hi))

    def find_median(self):
        if len(self.lo) > len(self.hi):
            return float(-self.lo[0])
        return (-self.lo[0] + self.hi[0]) / 2.0

mf = MedianFinder()
mf.add_num(1)
mf.add_num(2)
a = mf.find_median()
mf.add_num(3)
b = mf.find_median()
print([a, b])
`,
    nextUrl: /\/learn\/py-273-kth-matrix/,
    cursorAfter: "273",
  },
  {
    micro: 273,
    id: "py-273-kth-matrix",
    title: "DSA Kth Matrix Element",
    solution: `import heapq

def kth_smallest(matrix, k):
    n = len(matrix)
    heap = [(matrix[i][0], i, 0) for i in range(n)]
    heapq.heapify(heap)
    for _ in range(k):
        val, r, c = heapq.heappop(heap)
        if c + 1 < n:
            heapq.heappush(heap, (matrix[r][c + 1], r, c + 1))
    return val

print(kth_smallest([[1, 5, 9], [10, 11, 13], [12, 13, 15]], 8))
`,
    nextUrl: /\/learn\/py-274-network-delay/,
    cursorAfter: "274",
  },
  {
    micro: 274,
    id: "py-274-network-delay",
    title: "DSA Network Delay Time",
    solution: `import heapq
from collections import defaultdict

def network_delay_time(times, n, k):
    graph = defaultdict(list)
    for u, v, w in times:
        graph[u].append((v, w))
    dist = {k: 0}
    heap = [(0, k)]
    while heap:
        d, node = heapq.heappop(heap)
        if d > dist.get(node, float("inf")):
            continue
        for nei, w in graph[node]:
            nd = d + w
            if nd < dist.get(nei, float("inf")):
                dist[nei] = nd
                heapq.heappush(heap, (nd, nei))
    return max(dist.values()) if len(dist) == n else -1

print(network_delay_time([[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2))
`,
    nextUrl: /\/learn\/py-275-course-order/,
    cursorAfter: "275",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-26[9]-|^py-27[0-4]-/);
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

test.describe("micro-steps 269–274 · heaps aplicados II", () => {
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
      if (nextMicro <= 600) {
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
