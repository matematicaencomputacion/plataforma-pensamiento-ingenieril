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
    micro: 655,
    id: "py-655-num-islands",
    title: "DSA Graphs BFS V · Number of Islands",
    solution: "from collections import deque\n\ndef num_islands(grid):\n    if not grid:\n        return 0\n    rows, cols = len(grid), len(grid[0])\n    seen = [[False] * cols for _ in range(rows)]\n    total = 0\n    for r in range(rows):\n        for c in range(cols):\n            if grid[r][c] != '1' or seen[r][c]:\n                continue\n            total += 1\n            q = deque([(r, c)])\n            seen[r][c] = True\n            while q:\n                i, j = q.popleft()\n                for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n                    ni, nj = i + di, j + dj\n                    if 0 <= ni < rows and 0 <= nj < cols and not seen[ni][nj] and grid[ni][nj] == '1':\n                        seen[ni][nj] = True\n                        q.append((ni, nj))\n    return total\n\nprint(num_islands([['1','1','0'],['1','0','0'],['0','0','1']]))\n",
    nextUrl: /\/learn\/py-656-max-area-island/,
    cursorAfter: "656",
  },
  {
    micro: 656,
    id: "py-656-max-area-island",
    title: "DSA Graphs BFS V · Max Area",
    solution: "from collections import deque\n\ndef max_area_of_island(grid):\n    rows, cols = len(grid), len(grid[0])\n    seen = [[False] * cols for _ in range(rows)]\n    best = 0\n    for r in range(rows):\n        for c in range(cols):\n            if grid[r][c] != 1 or seen[r][c]:\n                continue\n            area = 0\n            q = deque([(r, c)])\n            seen[r][c] = True\n            while q:\n                i, j = q.popleft()\n                area += 1\n                for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n                    ni, nj = i + di, j + dj\n                    if 0 <= ni < rows and 0 <= nj < cols and not seen[ni][nj] and grid[ni][nj] == 1:\n                        seen[ni][nj] = True\n                        q.append((ni, nj))\n            best = max(best, area)\n    return best\n\nprint(max_area_of_island([[0,0,1,0,0],[1,1,1,0,0],[0,1,0,0,1],[0,0,0,1,1]]))\n",
    nextUrl: /\/learn\/py-657-surrounded-regions/,
    cursorAfter: "657",
  },
  {
    micro: 657,
    id: "py-657-surrounded-regions",
    title: "DSA Graphs BFS V · Surrounded Regions",
    solution: "from collections import deque\n\ndef solve(board):\n    if not board:\n        return\n    rows, cols = len(board), len(board[0])\n    q = deque()\n    for i in range(rows):\n        for j in (0, cols - 1):\n            if board[i][j] == 'O':\n                q.append((i, j))\n    for j in range(cols):\n        for i in (0, rows - 1):\n            if board[i][j] == 'O':\n                q.append((i, j))\n    while q:\n        i, j = q.popleft()\n        if not (0 <= i < rows and 0 <= j < cols) or board[i][j] != 'O':\n            continue\n        board[i][j] = 'S'\n        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n            q.append((i + di, j + dj))\n    for i in range(rows):\n        for j in range(cols):\n            if board[i][j] == 'O':\n                board[i][j] = 'X'\n            elif board[i][j] == 'S':\n                board[i][j] = 'O'\n\nb=[['X','X','X','X'],['X','O','O','X'],['X','X','O','X'],['X','O','X','X']]; solve(b); print(b)\n",
    nextUrl: /\/learn\/py-658-update-matrix/,
    cursorAfter: "658",
  },
  {
    micro: 658,
    id: "py-658-update-matrix",
    title: "DSA Graphs BFS V · 01 Matrix",
    solution: "from collections import deque\n\ndef update_matrix(mat):\n    rows, cols = len(mat), len(mat[0])\n    dist = [[10**9] * cols for _ in range(rows)]\n    q = deque()\n    for i in range(rows):\n        for j in range(cols):\n            if mat[i][j] == 0:\n                dist[i][j] = 0\n                q.append((i, j))\n    while q:\n        i, j = q.popleft()\n        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n            ni, nj = i + di, j + dj\n            if 0 <= ni < rows and 0 <= nj < cols and dist[ni][nj] > dist[i][j] + 1:\n                dist[ni][nj] = dist[i][j] + 1\n                q.append((ni, nj))\n    return dist\n\nprint(update_matrix([[0,0,0],[0,1,0],[1,1,1]]))\n",
    nextUrl: /\/learn\/py-659-shortest-bridge/,
    cursorAfter: "659",
  },
  {
    micro: 659,
    id: "py-659-shortest-bridge",
    title: "DSA Graphs BFS V · Shortest Bridge",
    solution: "from collections import deque\n\ndef shortest_bridge(grid):\n    n = len(grid)\n    q = deque()\n    def paint(i, j):\n        if not (0 <= i < n and 0 <= j < n) or grid[i][j] != 1:\n            return\n        grid[i][j] = 2\n        q.append((i, j, 0))\n        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n            paint(i + di, j + dj)\n    found = False\n    for i in range(n):\n        for j in range(n):\n            if grid[i][j] == 1:\n                paint(i, j)\n                found = True\n                break\n        if found:\n            break\n    while q:\n        i, j, d = q.popleft()\n        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n            ni, nj = i + di, j + dj\n            if 0 <= ni < n and 0 <= nj < n:\n                if grid[ni][nj] == 1:\n                    return d\n                if grid[ni][nj] == 0:\n                    grid[ni][nj] = 2\n                    q.append((ni, nj, d + 1))\n    return 0\n\nprint(shortest_bridge([[0,1],[1,0]]))\n",
    nextUrl: /\/learn\/py-660-pacific-atlantic/,
    cursorAfter: "660",
  },
  {
    micro: 660,
    id: "py-660-pacific-atlantic",
    title: "DSA Graphs BFS V · Pacific Atlantic",
    solution: "from collections import deque\n\ndef pacific_atlantic(heights):\n    rows, cols = len(heights), len(heights[0])\n    def reach(starts):\n        seen = set(starts)\n        q = deque(starts)\n        while q:\n            i, j = q.popleft()\n            for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n                ni, nj = i + di, j + dj\n                if 0 <= ni < rows and 0 <= nj < cols and (ni, nj) not in seen and heights[ni][nj] >= heights[i][j]:\n                    seen.add((ni, nj))\n                    q.append((ni, nj))\n        return seen\n    pac = [(0, j) for j in range(cols)] + [(i, 0) for i in range(rows)]\n    atl = [(rows - 1, j) for j in range(cols)] + [(i, cols - 1) for i in range(rows)]\n    both = reach(pac) & reach(atl)\n    return [[i, j] for i, j in both]\n\nprint(sorted(pacific_atlantic([[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]])))\n",
    nextUrl: /\/learn\/py-661-network-delay-k/,
    cursorAfter: "661",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:655|656|657|658|659|660)-/);
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

test.describe("micro-steps 655–660 · graphs bfs v", () => {
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
      if (nextMicro <= 660) {
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

      if (step.micro < 660) {
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
