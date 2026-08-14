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
    micro: 383,
    id: "py-383-num-islands",
    title: "DSA Num Islands",
    solution: `def num_islands(grid):
    if not grid:
        return 0
    m, n = len(grid), len(grid[0])
    c = 0

    def dfs(i, j):
        if i < 0 or j < 0 or i >= m or j >= n or grid[i][j] != "1":
            return
        grid[i][j] = "0"
        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            dfs(i + di, j + dj)

    for i in range(m):
        for j in range(n):
            if grid[i][j] == "1":
                c += 1
                dfs(i, j)
    return c

print(num_islands([["1", "1", "1", "1", "0"], ["1", "1", "0", "1", "0"], ["1", "1", "0", "0", "0"], ["0", "0", "0", "0", "0"]]))
`,
    nextUrl: /\/learn\/py-384-clone-graph/,
    cursorAfter: "384",
  },
  {
    micro: 384,
    id: "py-384-clone-graph",
    title: "DSA Clone Graph",
    solution: `def clone_graph(adj):
    return {k: list(v) for k, v in adj.items()}

print(sorted((k, sorted(v)) for k, v in clone_graph({1: [2, 4], 2: [1, 3], 3: [2, 4], 4: [1, 3]}).items()))
`,
    nextUrl: /\/learn\/py-385-course-order/,
    cursorAfter: "385",
  },
  {
    micro: 385,
    id: "py-385-course-order",
    title: "DSA Course Order",
    solution: `from collections import defaultdict, deque

def find_order(num_courses, prerequisites):
    g = defaultdict(list)
    indeg = [0] * num_courses
    for a, b in prerequisites:
        g[b].append(a)
        indeg[a] += 1
    q = deque([i for i in range(num_courses) if indeg[i] == 0])
    out = []
    while q:
        u = q.popleft()
        out.append(u)
        for v in g[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    return out if len(out) == num_courses else []

print(find_order(4, [[1, 0], [2, 0], [3, 1], [3, 2]]))
`,
    nextUrl: /\/learn\/py-386-oranges-rotting/,
    cursorAfter: "386",
  },
  {
    micro: 386,
    id: "py-386-oranges-rotting",
    title: "DSA Oranges Rotting",
    solution: `from collections import deque

def oranges_rotting(grid):
    m, n = len(grid), len(grid[0])
    q = deque()
    fresh = 0
    for i in range(m):
        for j in range(n):
            if grid[i][j] == 2:
                q.append((i, j, 0))
            elif grid[i][j] == 1:
                fresh += 1
    mins = 0
    while q:
        i, j, t = q.popleft()
        mins = t
        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            ni, nj = i + di, j + dj
            if 0 <= ni < m and 0 <= nj < n and grid[ni][nj] == 1:
                grid[ni][nj] = 2
                fresh -= 1
                q.append((ni, nj, t + 1))
    return mins if fresh == 0 else -1

print(oranges_rotting([[2, 1, 1], [1, 1, 0], [0, 1, 1]]))
`,
    nextUrl: /\/learn\/py-387-network-delay/,
    cursorAfter: "387",
  },
  {
    micro: 387,
    id: "py-387-network-delay",
    title: "DSA Network Delay",
    solution: `import heapq
from collections import defaultdict

def network_delay_time(times, n, k):
    g = defaultdict(list)
    for u, v, w in times:
        g[u].append((v, w))
    dist = {}
    h = [(0, k)]
    while h:
        d, u = heapq.heappop(h)
        if u in dist:
            continue
        dist[u] = d
        for v, w in g[u]:
            if v not in dist:
                heapq.heappush(h, (d + w, v))
    return max(dist.values()) if len(dist) == n else -1

print(network_delay_time([[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2))
`,
    nextUrl: /\/learn\/py-388-shortest-path-bin/,
    cursorAfter: "388",
  },
  {
    micro: 388,
    id: "py-388-shortest-path-bin",
    title: "DSA Shortest Path Bin",
    solution: `from collections import deque

def shortest_path_binary_matrix(grid):
    n = len(grid)
    if grid[0][0] or grid[n - 1][n - 1]:
        return -1
    q = deque([(0, 0, 1)])
    grid[0][0] = 1
    while q:
        i, j, d = q.popleft()
        if i == n - 1 and j == n - 1:
            return d
        for di in (-1, 0, 1):
            for dj in (-1, 0, 1):
                ni, nj = i + di, j + dj
                if 0 <= ni < n and 0 <= nj < n and grid[ni][nj] == 0:
                    grid[ni][nj] = 1
                    q.append((ni, nj, d + 1))
    return -1

print(shortest_path_binary_matrix([[0, 1], [1, 0]]))
`,
    nextUrl: /\/learn\/py-389-climb-stairs/,
    cursorAfter: "389",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:383|384|385|386|387|388)-/);
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

test.describe("micro-steps 383–388 · graphs II", () => {
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
      if (nextMicro <= 540) {
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
