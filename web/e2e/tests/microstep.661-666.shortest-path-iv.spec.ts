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
    micro: 661,
    id: "py-661-network-delay-k",
    title: "DSA Shortest Path IV · Delay Time",
    solution: "import heapq\nfrom collections import defaultdict\n\ndef network_delay_time(times, n, k):\n    graph = defaultdict(list)\n    for u, v, w in times:\n        graph[u].append((v, w))\n    dist = {k: 0}\n    heap = [(0, k)]\n    while heap:\n        d, u = heapq.heappop(heap)\n        if d > dist.get(u, 10**18):\n            continue\n        for v, w in graph[u]:\n            nd = d + w\n            if nd < dist.get(v, 10**18):\n                dist[v] = nd\n                heapq.heappush(heap, (nd, v))\n    return max(dist.values()) if len(dist) == n else -1\n\nprint(network_delay_time([[2,1,1],[2,3,1],[3,4,1]], 4, 2))\n",
    nextUrl: /\/learn\/py-662-cheapest-flights-k/,
    cursorAfter: "662",
  },
  {
    micro: 662,
    id: "py-662-cheapest-flights-k",
    title: "DSA Shortest Path IV · K Stops",
    solution: "def find_cheapest_price(n, flights, src, dst, k):\n    prices = [10**18] * n\n    prices[src] = 0\n    for _ in range(k + 1):\n        nxt = prices[:]\n        for u, v, w in flights:\n            if prices[u] + w < nxt[v]:\n                nxt[v] = prices[u] + w\n        prices = nxt\n    return -1 if prices[dst] >= 10**18 else prices[dst]\n\nprint(find_cheapest_price(4, [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], 0, 3, 1))\n",
    nextUrl: /\/learn\/py-663-path-with-min-effort/,
    cursorAfter: "663",
  },
  {
    micro: 663,
    id: "py-663-path-with-min-effort",
    title: "DSA Shortest Path IV · Min Effort",
    solution: "import heapq\n\ndef minimum_effort_path(heights):\n    rows, cols = len(heights), len(heights[0])\n    dist = [[10**18] * cols for _ in range(rows)]\n    dist[0][0] = 0\n    heap = [(0, 0, 0)]\n    while heap:\n        d, i, j = heapq.heappop(heap)\n        if (i, j) == (rows - 1, cols - 1):\n            return d\n        if d > dist[i][j]:\n            continue\n        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n            ni, nj = i + di, j + dj\n            if 0 <= ni < rows and 0 <= nj < cols:\n                nd = max(d, abs(heights[ni][nj] - heights[i][j]))\n                if nd < dist[ni][nj]:\n                    dist[ni][nj] = nd\n                    heapq.heappush(heap, (nd, ni, nj))\n    return 0\n\nprint(minimum_effort_path([[1,2,2],[3,8,2],[5,3,5]]))\n",
    nextUrl: /\/learn\/py-664-swim-in-water/,
    cursorAfter: "664",
  },
  {
    micro: 664,
    id: "py-664-swim-in-water",
    title: "DSA Shortest Path IV · Swim in Water",
    solution: "import heapq\n\ndef swim_in_water(grid):\n    n = len(grid)\n    dist = [[10**18] * n for _ in range(n)]\n    dist[0][0] = grid[0][0]\n    heap = [(grid[0][0], 0, 0)]\n    while heap:\n        t, i, j = heapq.heappop(heap)\n        if (i, j) == (n - 1, n - 1):\n            return t\n        if t > dist[i][j]:\n            continue\n        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n            ni, nj = i + di, j + dj\n            if 0 <= ni < n and 0 <= nj < n:\n                nt = max(t, grid[ni][nj])\n                if nt < dist[ni][nj]:\n                    dist[ni][nj] = nt\n                    heapq.heappush(heap, (nt, ni, nj))\n    return grid[0][0]\n\nprint(swim_in_water([[0,2],[1,3]]))\n",
    nextUrl: /\/learn\/py-665-cheapest-binary-maze/,
    cursorAfter: "665",
  },
  {
    micro: 665,
    id: "py-665-cheapest-binary-maze",
    title: "DSA Shortest Path IV · Shortest Path Obstacles",
    solution: "from collections import deque\n\ndef shortest_path(grid, k):\n    rows, cols = len(grid), len(grid[0])\n    q = deque([(0, 0, k, 0)])\n    seen = {(0, 0, k)}\n    while q:\n        i, j, left, d = q.popleft()\n        if (i, j) == (rows - 1, cols - 1):\n            return d\n        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n            ni, nj = i + di, j + dj\n            if 0 <= ni < rows and 0 <= nj < cols:\n                nleft = left - grid[ni][nj]\n                if nleft >= 0 and (ni, nj, nleft) not in seen:\n                    seen.add((ni, nj, nleft))\n                    q.append((ni, nj, nleft, d + 1))\n    return -1\n\nprint(shortest_path([[0,0,0],[1,1,0],[0,0,0],[0,1,1],[0,0,0]], 1))\n",
    nextUrl: /\/learn\/py-666-maze-nearest-exit/,
    cursorAfter: "666",
  },
  {
    micro: 666,
    id: "py-666-maze-nearest-exit",
    title: "DSA Shortest Path IV · Nearest Exit",
    solution: "from collections import deque\n\ndef nearest_exit(maze, entrance):\n    rows, cols = len(maze), len(maze[0])\n    sr, sc = entrance\n    q = deque([(sr, sc, 0)])\n    maze[sr][sc] = '+'\n    while q:\n        i, j, d = q.popleft()\n        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n            ni, nj = i + di, j + dj\n            if 0 <= ni < rows and 0 <= nj < cols and maze[ni][nj] == '.':\n                if ni in (0, rows - 1) or nj in (0, cols - 1):\n                    return d + 1\n                maze[ni][nj] = '+'\n                q.append((ni, nj, d + 1))\n    return -1\n\nprint(nearest_exit([['+','+','.','+'],['.','.','.','+'],['+','+','+','.']], [1, 2]))\n",
    nextUrl: /\/learn\/py-667-level-order/,
    cursorAfter: "667",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:661|662|663|664|665|666)-/);
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

test.describe("micro-steps 661–666 · shortest path iv", () => {
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

      if (step.micro < 666) {
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
