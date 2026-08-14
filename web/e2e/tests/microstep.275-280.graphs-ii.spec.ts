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
    micro: 275,
    id: "py-275-course-order",
    title: "DSA Course Schedule II",
    solution: `from collections import deque, defaultdict

def find_order(num_courses, prerequisites):
    graph = defaultdict(list)
    indeg = [0] * num_courses
    for a, b in prerequisites:
        graph[b].append(a)
        indeg[a] += 1
    q = deque([i for i in range(num_courses) if indeg[i] == 0])
    order = []
    while q:
        u = q.popleft()
        order.append(u)
        for v in graph[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    return order if len(order) == num_courses else []

print(find_order(4, [[1, 0], [2, 0], [3, 1], [3, 2]]))
`,
    nextUrl: /\/learn\/py-276-cheapest-flights/,
    cursorAfter: "276",
  },
  {
    micro: 276,
    id: "py-276-cheapest-flights",
    title: "DSA Cheapest Flights",
    solution: `def find_cheapest_price(n, flights, src, dst, k):
    prices = [float("inf")] * n
    prices[src] = 0
    for _ in range(k + 1):
        nxt = prices[:]
        for u, v, w in flights:
            if prices[u] + w < nxt[v]:
                nxt[v] = prices[u] + w
        prices = nxt
    return -1 if prices[dst] == float("inf") else prices[dst]

print(find_cheapest_price(4, [[0, 1, 100], [1, 2, 100], [2, 0, 100], [1, 3, 600], [2, 3, 200]], 0, 3, 1))
`,
    nextUrl: /\/learn\/py-277-redundant-edge/,
    cursorAfter: "277",
  },
  {
    micro: 277,
    id: "py-277-redundant-edge",
    title: "DSA Redundant Connection",
    solution: `def find_redundant_connection(edges):
    parent = list(range(len(edges) + 1))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for a, b in edges:
        ra, rb = find(a), find(b)
        if ra == rb:
            return [a, b]
        parent[rb] = ra
    return []

print(find_redundant_connection([[1, 2], [1, 3], [2, 3]]))
`,
    nextUrl: /\/learn\/py-278-accounts-merge/,
    cursorAfter: "278",
  },
  {
    micro: 278,
    id: "py-278-accounts-merge",
    title: "DSA Accounts Merge",
    solution: `from collections import defaultdict

def accounts_merge(accounts):
    n = len(accounts)
    parent = list(range(n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    email_to_id = {}
    for i, acc in enumerate(accounts):
        for email in acc[1:]:
            if email in email_to_id:
                parent[find(i)] = find(email_to_id[email])
            else:
                email_to_id[email] = i
    groups = defaultdict(set)
    for email, i in email_to_id.items():
        groups[find(i)].add(email)
    out = [[accounts[i][0]] + sorted(emails) for i, emails in groups.items()]
    return sorted(out, key=lambda a: (a[0], a[1]))

accounts = [["John", "j1@mail.com", "j2@mail.com"], ["John", "j3@mail.com"], ["John", "j1@mail.com", "j4@mail.com"], ["Mary", "m@mail.com"]]
print(accounts_merge(accounts))
`,
    nextUrl: /\/learn\/py-279-alien-dict/,
    cursorAfter: "279",
  },
  {
    micro: 279,
    id: "py-279-alien-dict",
    title: "DSA Alien Dictionary",
    solution: `from collections import defaultdict, deque

def alien_order(words):
    graph = defaultdict(set)
    indeg = {c: 0 for w in words for c in w}
    for w1, w2 in zip(words, words[1:]):
        if len(w1) > len(w2) and w1.startswith(w2):
            return ""
        for a, b in zip(w1, w2):
            if a != b:
                if b not in graph[a]:
                    graph[a].add(b)
                    indeg[b] += 1
                break
    q = deque(sorted([c for c in indeg if indeg[c] == 0]))
    out = []
    while q:
        u = q.popleft()
        out.append(u)
        for v in sorted(graph[u]):
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    return "".join(out) if len(out) == len(indeg) else ""

print(alien_order(["wrt", "wrf", "er", "ett", "rftt"]))
`,
    nextUrl: /\/learn\/py-280-min-cost-points/,
    cursorAfter: "280",
  },
  {
    micro: 280,
    id: "py-280-min-cost-points",
    title: "DSA Min Cost Points",
    solution: `import heapq

def min_cost_connect(points):
    n = len(points)
    if n <= 1:
        return 0
    in_mst = [False] * n
    heap = [(0, 0)]
    cost = 0
    used = 0
    while heap and used < n:
        d, i = heapq.heappop(heap)
        if in_mst[i]:
            continue
        in_mst[i] = True
        cost += d
        used += 1
        xi, yi = points[i]
        for j in range(n):
            if not in_mst[j]:
                xj, yj = points[j]
                heapq.heappush(heap, (abs(xi - xj) + abs(yi - yj), j))
    return cost

print(min_cost_connect([[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]))
`,
    nextUrl: /\/learn\/py-281-jump-game-ii/,
    cursorAfter: "281",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-27[5-9]-|^py-280-/);
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

test.describe("micro-steps 275–280 · graphs II", () => {
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
