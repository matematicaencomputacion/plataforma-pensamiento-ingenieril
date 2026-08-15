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
    micro: 619,
    id: "py-619-equations-possible",
    title: "DSA Union-Find IV · Equations",
    solution: "def equations_possible(equations):\n    parent = list(range(26))\n    def find(x):\n        while parent[x] != x:\n            parent[x] = parent[parent[x]]\n            x = parent[x]\n        return x\n    for eq in equations:\n        if eq[1] == '=':\n            parent[find(ord(eq[0]) - 97)] = find(ord(eq[3]) - 97)\n    for eq in equations:\n        if eq[1] == '!' and find(ord(eq[0]) - 97) == find(ord(eq[3]) - 97):\n            return False\n    return True\n\nprint(equations_possible(['a==b', 'b!=a']))\n",
    nextUrl: /\/learn\/py-620-smallest-string-swaps/,
    cursorAfter: "620",
  },
  {
    micro: 620,
    id: "py-620-smallest-string-swaps",
    title: "DSA Union-Find IV · Swap Letters",
    solution: "from collections import defaultdict\n\ndef smallest_string_with_swaps(s, pairs):\n    n = len(s)\n    parent = list(range(n))\n    def find(x):\n        while parent[x] != x:\n            parent[x] = parent[parent[x]]\n            x = parent[x]\n        return x\n    for a, b in pairs:\n        parent[find(a)] = find(b)\n    groups = defaultdict(list)\n    for i in range(n):\n        groups[find(i)].append(i)\n    out = list(s)\n    for idxs in groups.values():\n        chars = sorted(out[i] for i in idxs)\n        for i, ch in zip(sorted(idxs), chars):\n            out[i] = ch\n    return ''.join(out)\n\nprint(smallest_string_with_swaps('dcab', [[0, 3], [1, 2]]))\n",
    nextUrl: /\/learn\/py-621-provinces/,
    cursorAfter: "621",
  },
  {
    micro: 621,
    id: "py-621-provinces",
    title: "DSA Union-Find IV · Provinces",
    solution: "def find_circle_num(is_connected):\n    n = len(is_connected)\n    parent = list(range(n))\n    def find(x):\n        while parent[x] != x:\n            parent[x] = parent[parent[x]]\n            x = parent[x]\n        return x\n    for i in range(n):\n        for j in range(i + 1, n):\n            if is_connected[i][j]:\n                parent[find(i)] = find(j)\n    return len({find(i) for i in range(n)})\n\nprint(find_circle_num([[1, 1, 0], [1, 1, 0], [0, 0, 1]]))\n",
    nextUrl: /\/learn\/py-622-stones-removed/,
    cursorAfter: "622",
  },
  {
    micro: 622,
    id: "py-622-stones-removed",
    title: "DSA Union-Find IV · Stones Removed",
    solution: "def remove_stones(stones):\n    parent = {}\n    def find(x):\n        parent.setdefault(x, x)\n        while parent[x] != x:\n            parent[x] = parent[parent[x]]\n            x = parent[x]\n        return x\n    def union(a, b):\n        parent[find(a)] = find(b)\n    for r, c in stones:\n        union(r, ~c)\n    return len(stones) - len({find(r) for r, c in stones})\n\nprint(remove_stones([[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]]))\n",
    nextUrl: /\/learn\/py-623-similar-string-groups/,
    cursorAfter: "623",
  },
  {
    micro: 623,
    id: "py-623-similar-string-groups",
    title: "DSA Union-Find IV · Similar Groups",
    solution: "def num_similar_groups(strs):\n    n = len(strs)\n    parent = list(range(n))\n    def find(x):\n        while parent[x] != x:\n            parent[x] = parent[parent[x]]\n            x = parent[x]\n        return x\n    def similar(a, b):\n        diff = 0\n        for x, y in zip(a, b):\n            if x != y:\n                diff += 1\n                if diff > 2:\n                    return False\n        return diff in (0, 2)\n    for i in range(n):\n        for j in range(i + 1, n):\n            if similar(strs[i], strs[j]):\n                parent[find(i)] = find(j)\n    return len({find(i) for i in range(n)})\n\nprint(num_similar_groups(['tars', 'rats', 'arts', 'star']))\n",
    nextUrl: /\/learn\/py-624-graph-valid-tree/,
    cursorAfter: "624",
  },
  {
    micro: 624,
    id: "py-624-graph-valid-tree",
    title: "DSA Union-Find IV · Valid Tree",
    solution: "def valid_tree(n, edges):\n    if len(edges) != n - 1:\n        return False\n    parent = list(range(n))\n    def find(x):\n        while parent[x] != x:\n            parent[x] = parent[parent[x]]\n            x = parent[x]\n        return x\n    for a, b in edges:\n        ra, rb = find(a), find(b)\n        if ra == rb:\n            return False\n        parent[rb] = ra\n    return True\n\nprint(valid_tree(5, [[0, 1], [0, 2], [0, 3], [1, 4]]))\n",
    nextUrl: /\/learn\/py-625-implement-trie/,
    cursorAfter: "625",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:619|620|621|622|623|624)-/);
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

test.describe("micro-steps 619–624 · union find iv", () => {
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

      if (step.micro < 624) {
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
