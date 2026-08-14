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
  { micro: 203, id: "py-203-num-islands", title: "DSA Number of Islands", solution: `def num_islands(grid):
    if not grid:
        return 0
    rows, cols = len(grid), len(grid[0])
    count = 0

    def dfs(i, j):
        if i < 0 or j < 0 or i >= rows or j >= cols or grid[i][j] != "1":
            return
        grid[i][j] = "0"
        dfs(i + 1, j)
        dfs(i - 1, j)
        dfs(i, j + 1)
        dfs(i, j - 1)

    for i in range(rows):
        for j in range(cols):
            if grid[i][j] == "1":
                count += 1
                dfs(i, j)
    return count

print(num_islands([
    ["1", "1", "0", "0", "0"],
    ["1", "1", "0", "0", "0"],
    ["0", "0", "1", "0", "0"],
    ["0", "0", "0", "1", "1"],
]))
`, nextUrl: /\/learn\/py-204-/, cursorAfter: "204" },
  { micro: 204, id: "py-204-clone-graph", title: "DSA Clone Graph", solution: `from collections import deque

class Node:
    def __init__(self, val):
        self.val = val
        self.neighbors = []

def clone_graph(node):
    if node is None:
        return None
    mapping = {node: Node(node.val)}
    queue = deque([node])
    while queue:
        current = queue.popleft()
        for neighbor in current.neighbors:
            if neighbor not in mapping:
                mapping[neighbor] = Node(neighbor.val)
                queue.append(neighbor)
            mapping[current].neighbors.append(mapping[neighbor])
    return mapping[node]

n1, n2, n3, n4 = Node(1), Node(2), Node(3), Node(4)
n1.neighbors = [n2, n4]
n2.neighbors = [n1, n3]
n3.neighbors = [n2, n4]
n4.neighbors = [n1, n3]
cloned = clone_graph(n1)
print(sorted(neighbor.val for neighbor in cloned.neighbors))
`, nextUrl: /\/learn\/py-205-/, cursorAfter: "205" },
  { micro: 205, id: "py-205-course-schedule", title: "DSA Course Schedule", solution: `from collections import defaultdict, deque

def can_finish(num_courses, prerequisites):
    adj = defaultdict(list)
    indeg = [0] * num_courses
    for course, prep in prerequisites:
        adj[prep].append(course)
        indeg[course] += 1
    queue = deque([i for i in range(num_courses) if indeg[i] == 0])
    seen = 0
    while queue:
        node = queue.popleft()
        seen += 1
        for nxt in adj[node]:
            indeg[nxt] -= 1
            if indeg[nxt] == 0:
                queue.append(nxt)
    return seen == num_courses
print(can_finish(2, [[1, 0]]))
`, nextUrl: /\/learn\/py-206-/, cursorAfter: "206" },
  { micro: 206, id: "py-206-pacific-atlantic", title: "DSA Pacific Atlantic", solution: `from collections import deque

def pacific_atlantic(heights):
    rows, cols = len(heights), len(heights[0])

    def bfs(starts):
        seen = set(starts)
        queue = deque(starts)
        while queue:
            i, j = queue.popleft()
            for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                ni, nj = i + di, j + dj
                if (
                    0 <= ni < rows
                    and 0 <= nj < cols
                    and (ni, nj) not in seen
                    and heights[ni][nj] >= heights[i][j]
                ):
                    seen.add((ni, nj))
                    queue.append((ni, nj))
        return seen

    pacific = [(i, 0) for i in range(rows)] + [(0, j) for j in range(cols)]
    atlantic = [(i, cols - 1) for i in range(rows)] + [(rows - 1, j) for j in range(cols)]
    both = sorted(bfs(pacific) & bfs(atlantic))
    return [[i, j] for i, j in both]

print(pacific_atlantic([
    [1, 2, 2, 3, 5],
    [3, 2, 3, 4, 4],
    [2, 4, 5, 3, 1],
    [6, 7, 1, 4, 5],
    [5, 1, 1, 2, 4],
]))
`, nextUrl: /\/learn\/py-207-/, cursorAfter: "207" },
  { micro: 207, id: "py-207-rot-oranges", title: "DSA Rotting Oranges", solution: `from collections import deque

def oranges_rotting(grid):
    rows, cols = len(grid), len(grid[0])
    queue = deque()
    fresh = 0
    for i in range(rows):
        for j in range(cols):
            if grid[i][j] == 2:
                queue.append((i, j, 0))
            elif grid[i][j] == 1:
                fresh += 1
    minutes = 0
    while queue:
        i, j, t = queue.popleft()
        minutes = t
        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            ni, nj = i + di, j + dj
            if 0 <= ni < rows and 0 <= nj < cols and grid[ni][nj] == 1:
                grid[ni][nj] = 2
                fresh -= 1
                queue.append((ni, nj, t + 1))
    return minutes if fresh == 0 else -1
print(oranges_rotting([[2, 1, 1], [1, 1, 0], [0, 1, 1]]))
`, nextUrl: /\/learn\/py-208-/, cursorAfter: "208" },
  { micro: 208, id: "py-208-word-ladder", title: "DSA Word Ladder Length", solution: `from collections import deque

def ladder_length(begin_word, end_word, word_list):
    words = set(word_list)
    if end_word not in words:
        return 0
    queue = deque([(begin_word, 1)])
    while queue:
        word, dist = queue.popleft()
        if word == end_word:
            return dist
        for i in range(len(word)):
            for ord_c in range(ord('a'), ord('z') + 1):
                nxt = word[:i] + chr(ord_c) + word[i + 1:]
                if nxt in words:
                    words.remove(nxt)
                    queue.append((nxt, dist + 1))
    return 0
print(ladder_length('hit', 'cog', ['hot', 'dot', 'dog', 'lot', 'log', 'cog']))
`, nextUrl: /\/learn\/py-209-lru-cache/, cursorAfter: "209" },
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

test.describe("micro-steps 203–208 · curriculum", () => {
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
      if (nextMicro <= 576) {
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
