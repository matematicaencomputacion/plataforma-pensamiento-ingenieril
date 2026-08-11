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
    micro: 107,
    id: "py-107-tree-inorder",
    title: "DSA Tree Inorder",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None
def inorder(node):
    if node is None:
        return
    inorder(node.left)
    print(node.data)
    inorder(node.right)
root = TreeNode(1)
root.left = TreeNode(2)
root.right = TreeNode(3)
inorder(root)
`,
    nextUrl: /\/learn\/py-108-tree-postorder/,
    cursorAfter: "108",
  },
  {
    micro: 108,
    id: "py-108-tree-postorder",
    title: "DSA Tree Postorder",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None
def postorder(node):
    if node is None:
        return
    postorder(node.left)
    postorder(node.right)
    print(node.data)
root = TreeNode(1)
root.left = TreeNode(2)
root.right = TreeNode(3)
postorder(root)
`,
    nextUrl: /\/learn\/py-109-graph-dfs/,
    cursorAfter: "109",
  },
  {
    micro: 109,
    id: "py-109-graph-dfs",
    title: "DSA Graph DFS",
    solution: `graph = {'A': ['B', 'C'], 'B': ['D'], 'C': ['E'], 'D': [], 'E': []}
def dfs(graph, node, visited=None):
    if visited is None:
        visited = set()
    if node in visited:
        return
    visited.add(node)
    print(node)
    for neighbor in graph[node]:
        dfs(graph, neighbor, visited)
dfs(graph, 'A')
`,
    nextUrl: /\/learn\/py-110-graph-bfs/,
    cursorAfter: "110",
  },
  {
    micro: 110,
    id: "py-110-graph-bfs",
    title: "DSA Graph BFS",
    solution: `from collections import deque
graph = {'A': ['B', 'C'], 'B': ['D'], 'C': ['E'], 'D': [], 'E': []}
def bfs(graph, start):
    visited = set([start])
    q = deque([start])
    while q:
        node = q.popleft()
        print(node)
        for neighbor in graph[node]:
            if neighbor not in visited:
                visited.add(neighbor)
                q.append(neighbor)
bfs(graph, 'A')
`,
    nextUrl: /\/learn\/py-111-tree-height/,
    cursorAfter: "111",
  },
  {
    micro: 111,
    id: "py-111-tree-height",
    title: "DSA Tree Height",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None
def treeHeight(node):
    if node is None:
        return 0
    return 1 + max(treeHeight(node.left), treeHeight(node.right))
root = TreeNode(1)
root.left = TreeNode(2)
root.right = TreeNode(3)
print(treeHeight(root))
`,
    nextUrl: /\/learn\/py-112-dijkstra/,
    cursorAfter: "112",
  },
  {
    micro: 112,
    id: "py-112-dijkstra",
    title: "DSA Dijkstra Intro",
    solution: `graph = {'A': {'B': 4, 'C': 2}, 'B': {'C': 1, 'D': 5}, 'C': {'D': 8}, 'D': {}}
def dijkstra(graph, start):
    dist = {n: float('inf') for n in graph}
    dist[start] = 0
    unvisited = set(graph)
    while unvisited:
        u = min(unvisited, key=lambda n: dist[n])
        unvisited.remove(u)
        for v, w in graph[u].items():
            alt = dist[u] + w
            if alt < dist[v]:
                dist[v] = alt
    return dist
print(dijkstra(graph, 'A'))
`,
    nextUrl: /\/workspace/,
    cursorAfter: "113",
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

test.describe("micro-steps 107–112 · Tree walks / Graphs / Dijkstra", () => {
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
      await expect(
        page.locator(`#workspace-microstep-link-${step.micro + 1}`),
      ).toHaveCount(0);

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
