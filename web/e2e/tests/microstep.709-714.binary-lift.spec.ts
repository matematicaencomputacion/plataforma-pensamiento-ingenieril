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
    micro: 709,
    id: "py-709-tree-parents",
    title: "DSA Lift · Parents",
    solution: "def build_graph(n, edges):\n    g = [[] for _ in range(n)]\n    for u, v in edges:\n        g[u].append(v)\n        g[v].append(u)\n    return g\n\ndef build_parent_depth(n, edges, root=0):\n    g = build_graph(n, edges)\n    parent = [-1] * n\n    depth = [0] * n\n    stack = [root]\n    seen = {root}\n    while stack:\n        u = stack.pop()\n        for v in g[u]:\n            if v in seen:\n                continue\n            seen.add(v)\n            parent[v] = u\n            depth[v] = depth[u] + 1\n            stack.append(v)\n    return parent, depth\n\ndef build_up(parent):\n    n = len(parent)\n    log = max(1, (n - 1).bit_length())\n    up = [[-1] * n for _ in range(log)]\n    up[0] = parent[:]\n    for k in range(1, log):\n        for v in range(n):\n            p = up[k - 1][v]\n            up[k][v] = -1 if p < 0 else up[k - 1][p]\n    return up\n\ndef kth_ancestor(up, node, k):\n    bit = 0\n    while k and node >= 0:\n        if k & 1:\n            node = up[bit][node]\n            if node < 0:\n                return -1\n        k >>= 1\n        bit += 1\n    return node\n\nparent, depth = build_parent_depth(7, [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]])\nprint(parent)\n",
    nextUrl: /\/learn\/py-710-binary-lift-table/,
    cursorAfter: "710",
  },
  {
    micro: 710,
    id: "py-710-binary-lift-table",
    title: "DSA Lift · Jump Table",
    solution: "def build_graph(n, edges):\n    g = [[] for _ in range(n)]\n    for u, v in edges:\n        g[u].append(v)\n        g[v].append(u)\n    return g\n\ndef build_parent_depth(n, edges, root=0):\n    g = build_graph(n, edges)\n    parent = [-1] * n\n    depth = [0] * n\n    stack = [root]\n    seen = {root}\n    while stack:\n        u = stack.pop()\n        for v in g[u]:\n            if v in seen:\n                continue\n            seen.add(v)\n            parent[v] = u\n            depth[v] = depth[u] + 1\n            stack.append(v)\n    return parent, depth\n\ndef build_up(parent):\n    n = len(parent)\n    log = max(1, (n - 1).bit_length())\n    up = [[-1] * n for _ in range(log)]\n    up[0] = parent[:]\n    for k in range(1, log):\n        for v in range(n):\n            p = up[k - 1][v]\n            up[k][v] = -1 if p < 0 else up[k - 1][p]\n    return up\n\ndef kth_ancestor(up, node, k):\n    bit = 0\n    while k and node >= 0:\n        if k & 1:\n            node = up[bit][node]\n            if node < 0:\n                return -1\n        k >>= 1\n        bit += 1\n    return node\n\nparent, _ = build_parent_depth(7, [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]])\nup = build_up(parent)\nprint(up[1][3])\n",
    nextUrl: /\/learn\/py-711-kth-ancestor/,
    cursorAfter: "711",
  },
  {
    micro: 711,
    id: "py-711-kth-ancestor",
    title: "DSA Lift · Kth Ancestor",
    solution: "def build_graph(n, edges):\n    g = [[] for _ in range(n)]\n    for u, v in edges:\n        g[u].append(v)\n        g[v].append(u)\n    return g\n\ndef build_parent_depth(n, edges, root=0):\n    g = build_graph(n, edges)\n    parent = [-1] * n\n    depth = [0] * n\n    stack = [root]\n    seen = {root}\n    while stack:\n        u = stack.pop()\n        for v in g[u]:\n            if v in seen:\n                continue\n            seen.add(v)\n            parent[v] = u\n            depth[v] = depth[u] + 1\n            stack.append(v)\n    return parent, depth\n\ndef build_up(parent):\n    n = len(parent)\n    log = max(1, (n - 1).bit_length())\n    up = [[-1] * n for _ in range(log)]\n    up[0] = parent[:]\n    for k in range(1, log):\n        for v in range(n):\n            p = up[k - 1][v]\n            up[k][v] = -1 if p < 0 else up[k - 1][p]\n    return up\n\ndef kth_ancestor(up, node, k):\n    bit = 0\n    while k and node >= 0:\n        if k & 1:\n            node = up[bit][node]\n            if node < 0:\n                return -1\n        k >>= 1\n        bit += 1\n    return node\n\nparent, _ = build_parent_depth(7, [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]])\nup = build_up(parent)\nprint(kth_ancestor(up, 3, 2))\n",
    nextUrl: /\/learn\/py-712-lca-lift/,
    cursorAfter: "712",
  },
  {
    micro: 712,
    id: "py-712-lca-lift",
    title: "DSA Lift · LCA",
    solution: "def build_graph(n, edges):\n    g = [[] for _ in range(n)]\n    for u, v in edges:\n        g[u].append(v)\n        g[v].append(u)\n    return g\n\ndef build_parent_depth(n, edges, root=0):\n    g = build_graph(n, edges)\n    parent = [-1] * n\n    depth = [0] * n\n    stack = [root]\n    seen = {root}\n    while stack:\n        u = stack.pop()\n        for v in g[u]:\n            if v in seen:\n                continue\n            seen.add(v)\n            parent[v] = u\n            depth[v] = depth[u] + 1\n            stack.append(v)\n    return parent, depth\n\ndef build_up(parent):\n    n = len(parent)\n    log = max(1, (n - 1).bit_length())\n    up = [[-1] * n for _ in range(log)]\n    up[0] = parent[:]\n    for k in range(1, log):\n        for v in range(n):\n            p = up[k - 1][v]\n            up[k][v] = -1 if p < 0 else up[k - 1][p]\n    return up\n\ndef kth_ancestor(up, node, k):\n    bit = 0\n    while k and node >= 0:\n        if k & 1:\n            node = up[bit][node]\n            if node < 0:\n                return -1\n        k >>= 1\n        bit += 1\n    return node\n\ndef lca(up, depth, u, v):\n    if depth[u] < depth[v]:\n        u, v = v, u\n    u = kth_ancestor(up, u, depth[u] - depth[v])\n    if u == v:\n        return u\n    for k in range(len(up) - 1, -1, -1):\n        if up[k][u] != up[k][v]:\n            u, v = up[k][u], up[k][v]\n    return up[0][u]\n\nparent, depth = build_parent_depth(7, [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]])\nup = build_up(parent)\nprint(lca(up, depth, 3, 5))\n",
    nextUrl: /\/learn\/py-713-tree-dist/,
    cursorAfter: "713",
  },
  {
    micro: 713,
    id: "py-713-tree-dist",
    title: "DSA Lift · Tree Dist",
    solution: "def build_graph(n, edges):\n    g = [[] for _ in range(n)]\n    for u, v in edges:\n        g[u].append(v)\n        g[v].append(u)\n    return g\n\ndef build_parent_depth(n, edges, root=0):\n    g = build_graph(n, edges)\n    parent = [-1] * n\n    depth = [0] * n\n    stack = [root]\n    seen = {root}\n    while stack:\n        u = stack.pop()\n        for v in g[u]:\n            if v in seen:\n                continue\n            seen.add(v)\n            parent[v] = u\n            depth[v] = depth[u] + 1\n            stack.append(v)\n    return parent, depth\n\ndef build_up(parent):\n    n = len(parent)\n    log = max(1, (n - 1).bit_length())\n    up = [[-1] * n for _ in range(log)]\n    up[0] = parent[:]\n    for k in range(1, log):\n        for v in range(n):\n            p = up[k - 1][v]\n            up[k][v] = -1 if p < 0 else up[k - 1][p]\n    return up\n\ndef kth_ancestor(up, node, k):\n    bit = 0\n    while k and node >= 0:\n        if k & 1:\n            node = up[bit][node]\n            if node < 0:\n                return -1\n        k >>= 1\n        bit += 1\n    return node\n\ndef lca(up, depth, u, v):\n    if depth[u] < depth[v]:\n        u, v = v, u\n    u = kth_ancestor(up, u, depth[u] - depth[v])\n    if u == v:\n        return u\n    for k in range(len(up) - 1, -1, -1):\n        if up[k][u] != up[k][v]:\n            u, v = up[k][u], up[k][v]\n    return up[0][u]\n\ndef dist(up, depth, u, v):\n    w = lca(up, depth, u, v)\n    return depth[u] + depth[v] - 2 * depth[w]\n\nparent, depth = build_parent_depth(7, [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]])\nup = build_up(parent)\nprint(dist(up, depth, 3, 5))\n",
    nextUrl: /\/learn\/py-714-jump-depth/,
    cursorAfter: "714",
  },
  {
    micro: 714,
    id: "py-714-jump-depth",
    title: "DSA Lift · Jump Depth",
    solution: "def build_graph(n, edges):\n    g = [[] for _ in range(n)]\n    for u, v in edges:\n        g[u].append(v)\n        g[v].append(u)\n    return g\n\ndef build_parent_depth(n, edges, root=0):\n    g = build_graph(n, edges)\n    parent = [-1] * n\n    depth = [0] * n\n    stack = [root]\n    seen = {root}\n    while stack:\n        u = stack.pop()\n        for v in g[u]:\n            if v in seen:\n                continue\n            seen.add(v)\n            parent[v] = u\n            depth[v] = depth[u] + 1\n            stack.append(v)\n    return parent, depth\n\ndef build_up(parent):\n    n = len(parent)\n    log = max(1, (n - 1).bit_length())\n    up = [[-1] * n for _ in range(log)]\n    up[0] = parent[:]\n    for k in range(1, log):\n        for v in range(n):\n            p = up[k - 1][v]\n            up[k][v] = -1 if p < 0 else up[k - 1][p]\n    return up\n\ndef kth_ancestor(up, node, k):\n    bit = 0\n    while k and node >= 0:\n        if k & 1:\n            node = up[bit][node]\n            if node < 0:\n                return -1\n        k >>= 1\n        bit += 1\n    return node\n\ndef jump_to_depth(up, depth, node, d):\n    if d > depth[node]:\n        return -1\n    return kth_ancestor(up, node, depth[node] - d)\n\nparent, depth = build_parent_depth(7, [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]])\nup = build_up(parent)\nprint(jump_to_depth(up, depth, 3, 1))\n",
    nextUrl: /\/learn\/py-715-sieve-primes/,
    cursorAfter: "715",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:709|710|711|712|713|714)-/);
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

test.describe("micro-steps 709–714 · binary lift", () => {
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
