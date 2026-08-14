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
    micro: 263,
    id: "py-263-open-lock",
    title: "DSA Open the Lock",
    solution: `from collections import deque

def open_lock(deadends, target):
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
                nxt = cur[:i] + str((int(cur[i]) + d) % 10) + cur[i + 1 :]
                if nxt not in seen and nxt not in dead:
                    seen.add(nxt)
                    q.append((nxt, dist + 1))
    return -1

print(open_lock(["0201", "0101", "0102", "1212", "2002"], "0202"))
`,
    nextUrl: /\/learn\/py-264-shortest-binary/,
    cursorAfter: "264",
  },
  {
    micro: 264,
    id: "py-264-shortest-binary",
    title: "DSA Shortest Binary Path",
    solution: `from collections import deque

def shortest_path_binary(grid):
    n = len(grid)
    if grid[0][0] or grid[n - 1][n - 1]:
        return -1
    q = deque([(0, 0, 1)])
    grid[0][0] = 1
    dirs = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
    while q:
        r, c, dist = q.popleft()
        if r == n - 1 and c == n - 1:
            return dist
        for dr, dc in dirs:
            nr, nc = r + dr, c + dc
            if 0 <= nr < n and 0 <= nc < n and grid[nr][nc] == 0:
                grid[nr][nc] = 1
                q.append((nr, nc, dist + 1))
    return -1

print(shortest_path_binary([[0, 1], [1, 0]]))
`,
    nextUrl: /\/learn\/py-265-walls-gates/,
    cursorAfter: "265",
  },
  {
    micro: 265,
    id: "py-265-walls-gates",
    title: "DSA Walls and Gates",
    solution: `from collections import deque

INF = 2147483647

def walls_and_gates(rooms):
    if not rooms:
        return
    rows, cols = len(rooms), len(rooms[0])
    q = deque()
    for i in range(rows):
        for j in range(cols):
            if rooms[i][j] == 0:
                q.append((i, j))
    while q:
        r, c = q.popleft()
        for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            nr, nc = r + dr, c + dc
            if 0 <= nr < rows and 0 <= nc < cols and rooms[nr][nc] == INF:
                rooms[nr][nc] = rooms[r][c] + 1
                q.append((nr, nc))

rooms = [[INF, -1, 0, INF], [INF, INF, INF, -1], [INF, -1, INF, -1], [0, -1, INF, INF]]
walls_and_gates(rooms)
print(rooms)
`,
    nextUrl: /\/learn\/py-266-circular-queue/,
    cursorAfter: "266",
  },
  {
    micro: 266,
    id: "py-266-circular-queue",
    title: "DSA Circular Queue",
    solution: `class MyCircularQueue:
    def __init__(self, k):
        self.data = [0] * k
        self.k = k
        self.head = 0
        self.size = 0

    def en_queue(self, value):
        if self.is_full():
            return False
        self.data[(self.head + self.size) % self.k] = value
        self.size += 1
        return True

    def de_queue(self):
        if self.is_empty():
            return False
        self.head = (self.head + 1) % self.k
        self.size -= 1
        return True

    def front(self):
        return -1 if self.is_empty() else self.data[self.head]

    def rear(self):
        return -1 if self.is_empty() else self.data[(self.head + self.size - 1) % self.k]

    def is_empty(self):
        return self.size == 0

    def is_full(self):
        return self.size == self.k

q = MyCircularQueue(3)
print([q.en_queue(1), q.en_queue(2), q.en_queue(3), q.en_queue(4), q.rear(), q.is_full(), q.de_queue(), q.en_queue(4), q.rear()])
`,
    nextUrl: /\/learn\/py-267-recent-counter/,
    cursorAfter: "267",
  },
  {
    micro: 267,
    id: "py-267-recent-counter",
    title: "DSA Recent Counter",
    solution: `from collections import deque

class RecentCounter:
    def __init__(self):
        self.q = deque()

    def ping(self, t):
        self.q.append(t)
        while self.q[0] < t - 3000:
            self.q.popleft()
        return len(self.q)

c = RecentCounter()
print([c.ping(1), c.ping(100), c.ping(3001), c.ping(3002)])
`,
    nextUrl: /\/learn\/py-268-time-tickets/,
    cursorAfter: "268",
  },
  {
    micro: 268,
    id: "py-268-time-tickets",
    title: "DSA Time Needed Tickets",
    solution: `def time_required(tickets, k):
    time = 0
    for i, t in enumerate(tickets):
        if i <= k:
            time += min(t, tickets[k])
        else:
            time += min(t, tickets[k] - 1)
    return time

print(time_required([2, 3, 2], 2))
`,
    nextUrl: /\/learn\/py-269-last-stone/,
    cursorAfter: "269",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-26[3-8]-/);
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

test.describe("micro-steps 263–268 · queues & BFS II", () => {
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
      if (nextMicro <= 504) {
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
