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
    micro: 529,
    id: "py-529-recent-counter",
    title: "DSA Recent Counter",
    solution: `class RecentCounter:
    def __init__(self):
        from collections import deque
        self.q = deque()
    def ping(self, t):
        self.q.append(t)
        while self.q[0] < t - 3000:
            self.q.popleft()
        return len(self.q)

c = RecentCounter()
print([c.ping(1), c.ping(100), c.ping(3001), c.ping(3002)])
`,
    nextUrl: /\/learn\/py-530-dota2-senate/,
    cursorAfter: "530",
  },
  {
    micro: 530,
    id: "py-530-dota2-senate",
    title: "DSA Dota Senate",
    solution: `def predict_party_victory(senate):
    from collections import deque
    r = deque(); d = deque()
    n = len(senate)
    for i, ch in enumerate(senate):
        (r if ch == "R" else d).append(i)
    while r and d:
        a, b = r.popleft(), d.popleft()
        if a < b:
            r.append(a + n)
        else:
            d.append(b + n)
    return "Radiant" if r else "Dire"

print(predict_party_victory("RD"))
`,
    nextUrl: /\/learn\/py-531-open-lock/,
    cursorAfter: "531",
  },
  {
    micro: 531,
    id: "py-531-open-lock",
    title: "DSA Open Lock",
    solution: `def open_lock(deadends, target):
    from collections import deque
    dead = set(deadends)
    if "0000" in dead:
        return -1
    q = deque([("0000", 0)])
    seen = {"0000"}
    while q:
        cur, dist = q.popleft()
        if cur == target:
            return dist
        for i in range(4):
            for d in (-1, 1):
                nxt = cur[:i] + str((int(cur[i]) + d) % 10) + cur[i + 1:]
                if nxt not in seen and nxt not in dead:
                    seen.add(nxt)
                    q.append((nxt, dist + 1))
    return -1

print(open_lock(["0201", "0101", "0102", "1212", "2002"], "0202"))
`,
    nextUrl: /\/learn\/py-532-oranges-rot/,
    cursorAfter: "532",
  },
  {
    micro: 532,
    id: "py-532-oranges-rot",
    title: "DSA Oranges Rot",
    solution: `def oranges_rotting(grid):
    from collections import deque
    q = deque()
    fresh = 0
    rows, cols = len(grid), len(grid[0])
    for i in range(rows):
        for j in range(cols):
            if grid[i][j] == 2:
                q.append((i, j, 0))
            elif grid[i][j] == 1:
                fresh += 1
    mins = 0
    while q:
        r, c, t = q.popleft()
        mins = t
        for dr, dc in ((1,0),(-1,0),(0,1),(0,-1)):
            nr, nc = r + dr, c + dc
            if 0 <= nr < rows and 0 <= nc < cols and grid[nr][nc] == 1:
                grid[nr][nc] = 2
                fresh -= 1
                q.append((nr, nc, t + 1))
    return mins if fresh == 0 else -1

print(oranges_rotting([[2, 1, 1], [1, 1, 0], [0, 1, 1]]))
`,
    nextUrl: /\/learn\/py-533-shortest-path-bin/,
    cursorAfter: "533",
  },
  {
    micro: 533,
    id: "py-533-shortest-path-bin",
    title: "DSA Shortest Path Bin",
    solution: `def shortest_path_binary_matrix(grid):
    from collections import deque
    n = len(grid)
    if grid[0][0] or grid[n - 1][n - 1]:
        return -1
    q = deque([(0, 0, 1)])
    grid[0][0] = 1
    while q:
        r, c, d = q.popleft()
        if r == n - 1 and c == n - 1:
            return d
        for dr in (-1, 0, 1):
            for dc in (-1, 0, 1):
                nr, nc = r + dr, c + dc
                if 0 <= nr < n and 0 <= nc < n and grid[nr][nc] == 0:
                    grid[nr][nc] = 1
                    q.append((nr, nc, d + 1))
    return -1

print(shortest_path_binary_matrix([[0, 1], [1, 0]]))
`,
    nextUrl: /\/learn\/py-534-snakes-ladders/,
    cursorAfter: "534",
  },
  {
    micro: 534,
    id: "py-534-snakes-ladders",
    title: "DSA Snakes Ladders",
    solution: `def snakes_and_ladders(board):
    from collections import deque
    n = len(board)
    def cell(pos):
        r, c = divmod(pos - 1, n)
        row = n - 1 - r
        col = c if r % 2 == 0 else n - 1 - c
        return row, col
    q = deque([(1, 0)])
    seen = {1}
    while q:
        pos, dist = q.popleft()
        if pos == n * n:
            return dist
        for nxt in range(pos + 1, min(pos + 6, n * n) + 1):
            r, c = cell(nxt)
            dest = board[r][c] if board[r][c] != -1 else nxt
            if dest not in seen:
                seen.add(dest)
                q.append((dest, dist + 1))
    return -1

print(snakes_and_ladders([[-1, -1, -1, -1, -1, -1], [-1, -1, -1, -1, -1, -1], [-1, -1, -1, -1, -1, -1], [-1, 35, -1, -1, 13, -1], [-1, -1, -1, -1, -1, -1], [-1, 15, -1, -1, -1, -1]]))
`,
    nextUrl: /\/workspace/,
    cursorAfter: "535",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:529|530|531|532|533|534)-/);
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

test.describe("micro-steps 529–534 · queues III", () => {
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
      if (nextMicro <= 558) {
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
