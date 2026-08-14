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
    micro: 113,
    id: "py-113-heap",
    title: "DSA Heap (heapq)",
    solution: `import heapq
h = []
for x in [5, 3, 8, 1]:
    heapq.heappush(h, x)
while h:
    print(heapq.heappop(h))
`,
    nextUrl: /\/learn\/py-114-priority-queue/,
    cursorAfter: "114",
  },
  {
    micro: 114,
    id: "py-114-priority-queue",
    title: "DSA Priority Queue",
    solution: `import heapq
pq = []
heapq.heappush(pq, (2, 'code'))
heapq.heappush(pq, (1, 'eat'))
heapq.heappush(pq, (3, 'sleep'))
while pq:
    print(heapq.heappop(pq)[1])
`,
    nextUrl: /\/learn\/py-115-union-find/,
    cursorAfter: "115",
  },
  {
    micro: 115,
    id: "py-115-union-find",
    title: "DSA Union-Find",
    solution: `parent = [0, 1, 2, 3]
def find(x):
    while parent[x] != x:
        x = parent[x]
    return x
def union(a, b):
    ra, rb = find(a), find(b)
    if ra != rb:
        parent[rb] = ra
union(0, 1)
union(2, 3)
union(1, 2)
print(find(0) == find(3))
`,
    nextUrl: /\/learn\/py-116-kruskal/,
    cursorAfter: "116",
  },
  {
    micro: 116,
    id: "py-116-kruskal",
    title: "DSA Kruskal MST",
    solution: `edges = [(1, 'A', 'B'), (2, 'B', 'C'), (3, 'A', 'C'), (4, 'C', 'D')]
def kruskal(edges, nodes):
    parent = {n: n for n in nodes}
    def find(x):
        while parent[x] != x:
            x = parent[x]
        return x
    total = 0
    for w, u, v in sorted(edges, key=lambda e: e[0]):
        if find(u) != find(v):
            parent[find(v)] = find(u)
            total += w
    return total
print(kruskal(edges, ['A', 'B', 'C', 'D']))
`,
    nextUrl: /\/learn\/py-117-prim/,
    cursorAfter: "117",
  },
  {
    micro: 117,
    id: "py-117-prim",
    title: "DSA Prim MST",
    solution: `import heapq
graph = {'A': {'B': 1, 'C': 3}, 'B': {'A': 1, 'C': 2, 'D': 4}, 'C': {'A': 3, 'B': 2, 'D': 5}, 'D': {'B': 4, 'C': 5}}
def prim(graph, start='A'):
    visited = set()
    pq = [(0, start)]
    total = 0
    while pq and len(visited) < len(graph):
        w, u = heapq.heappop(pq)
        if u in visited:
            continue
        visited.add(u)
        total += w
        for v, vw in graph[u].items():
            if v not in visited:
                heapq.heappush(pq, (vw, v))
    return total
print(prim(graph))
`,
    nextUrl: /\/learn\/py-118-topo-sort/,
    cursorAfter: "118",
  },
  {
    micro: 118,
    id: "py-118-topo-sort",
    title: "DSA Topological Sort",
    solution: `from collections import deque
graph = {'A': ['B', 'C'], 'B': ['D'], 'C': ['D'], 'D': []}
def topo(graph):
    indeg = {n: 0 for n in graph}
    for u in graph:
        for v in graph[u]:
            indeg[v] += 1
    q = deque([n for n in graph if indeg[n] == 0])
    order = []
    while q:
        u = q.popleft()
        order.append(u)
        for v in graph[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    return order
print(topo(graph))
`,
    nextUrl: /\/learn\/py-119-bellman-ford/,
    cursorAfter: "119",
  },
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

test.describe("micro-steps 113–118 · Heap / PQ / Union-Find / MST / Topo", () => {
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
      if (nextMicro <= 564) {
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
