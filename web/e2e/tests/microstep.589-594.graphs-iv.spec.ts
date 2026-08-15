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
    micro: 589,
    id: "py-589-clone-graph",
    title: "DSA Clone Graph",
    solution: `class Node:
    def __init__(self, val=0, neighbors=None):
        self.val = val
        self.neighbors = neighbors or []

def clone_graph(node):
    if not node:
        return None
    mp = {}
    def dfs(n):
        if n in mp:
            return mp[n]
        c = Node(n.val)
        mp[n] = c
        c.neighbors = [dfs(x) for x in n.neighbors]
        return c
    return dfs(node)

a = Node(1); b = Node(2)
a.neighbors = [b]; b.neighbors = [a]
c = clone_graph(a)
print(c.val, c.neighbors[0].val, c is not a)
`,
    nextUrl: /\/learn\/py-590-course-schedule/,
    cursorAfter: "590",
  },
  {
    micro: 590,
    id: "py-590-course-schedule",
    title: "DSA Course Schedule",
    solution: `def can_finish(num_courses, prereq):
    from collections import defaultdict
    g = defaultdict(list)
    indeg = [0] * num_courses
    for a, b in prereq:
        g[b].append(a); indeg[a] += 1
    q = [i for i in range(num_courses) if indeg[i] == 0]
    seen = 0
    while q:
        u = q.pop()
        seen += 1
        for v in g[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    return seen == num_courses

print(can_finish(2, [[1, 0]]))
`,
    nextUrl: /\/learn\/py-591-network-delay/,
    cursorAfter: "591",
  },
  {
    micro: 591,
    id: "py-591-network-delay",
    title: "DSA Network Delay",
    solution: `def network_delay_time(times, n, k):
    import heapq
    from collections import defaultdict
    g = defaultdict(list)
    for u, v, w in times:
        g[u].append((v, w))
    dist = {k: 0}
    h = [(0, k)]
    while h:
        d, u = heapq.heappop(h)
        if d > dist.get(u, 10**9):
            continue
        for v, w in g[u]:
            nd = d + w
            if nd < dist.get(v, 10**9):
                dist[v] = nd
                heapq.heappush(h, (nd, v))
    return max(dist.values()) if len(dist) == n else -1

print(network_delay_time([[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2))
`,
    nextUrl: /\/learn\/py-592-redundant-conn/,
    cursorAfter: "592",
  },
  {
    micro: 592,
    id: "py-592-redundant-conn",
    title: "DSA Redundant Conn",
    solution: `def find_redundant_connection(edges):
    p = list(range(len(edges) + 1))
    def find(x):
        while p[x] != x:
            p[x] = p[p[x]]
            x = p[x]
        return x
    for a, b in edges:
        pa, pb = find(a), find(b)
        if pa == pb:
            return [a, b]
        p[pa] = pb
    return []

print(find_redundant_connection([[1, 2], [1, 3], [2, 3]]))
`,
    nextUrl: /\/learn\/py-593-valid-path/,
    cursorAfter: "593",
  },
  {
    micro: 593,
    id: "py-593-valid-path",
    title: "DSA Valid Path",
    solution: `def valid_path(n, edges, source, dest):
    from collections import defaultdict, deque
    g = defaultdict(list)
    for a, b in edges:
        g[a].append(b); g[b].append(a)
    seen = {source}
    q = deque([source])
    while q:
        u = q.popleft()
        if u == dest:
            return True
        for v in g[u]:
            if v not in seen:
                seen.add(v); q.append(v)
    return False

print(valid_path(3, [[0, 1], [1, 2], [2, 0]], 0, 2))
`,
    nextUrl: /\/learn\/py-594-all-paths/,
    cursorAfter: "594",
  },
  {
    micro: 594,
    id: "py-594-all-paths",
    title: "DSA All Paths",
    solution: `def all_paths_source_target(graph):
    res = []
    def dfs(u, path):
        if u == len(graph) - 1:
            res.append(path[:]); return
        for v in graph[u]:
            path.append(v)
            dfs(v, path)
            path.pop()
    dfs(0, [0])
    return res

print(all_paths_source_target([[1, 2], [3], [3], []]))
`,
    nextUrl: /\/learn\/py-595-majority-n2/,
    cursorAfter: "595",
  }
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:589|590|591|592|593|594)-/);
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

test.describe("micro-steps 589–594 · graphs IV", () => {
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
