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
    micro: 667,
    id: "py-667-level-order",
    title: "DSA Trees V · Level Order",
    solution: "from collections import deque\n\nclass TreeNode:\n    def __init__(self, val=0, left=None, right=None):\n        self.val = val\n        self.left = left\n        self.right = right\n\ndef level_order(root):\n    if not root:\n        return []\n    out = []\n    q = deque([root])\n    while q:\n        level = []\n        for _ in range(len(q)):\n            node = q.popleft()\n            level.append(node.val)\n            if node.left:\n                q.append(node.left)\n            if node.right:\n                q.append(node.right)\n        out.append(level)\n    return out\n\nprint(level_order(TreeNode(3, TreeNode(9), TreeNode(20, TreeNode(15), TreeNode(7)))))\n",
    nextUrl: /\/learn\/py-668-right-side-view/,
    cursorAfter: "668",
  },
  {
    micro: 668,
    id: "py-668-right-side-view",
    title: "DSA Trees V · Right Side View",
    solution: "from collections import deque\n\nclass TreeNode:\n    def __init__(self, val=0, left=None, right=None):\n        self.val = val\n        self.left = left\n        self.right = right\n\ndef right_side_view(root):\n    if not root:\n        return []\n    out = []\n    q = deque([root])\n    while q:\n        last = None\n        for _ in range(len(q)):\n            last = q.popleft()\n            if last.left:\n                q.append(last.left)\n            if last.right:\n                q.append(last.right)\n        out.append(last.val)\n    return out\n\nprint(right_side_view(TreeNode(1, TreeNode(2, None, TreeNode(5)), TreeNode(3, None, TreeNode(4)))))\n",
    nextUrl: /\/learn\/py-669-lowest-common-ancestor/,
    cursorAfter: "669",
  },
  {
    micro: 669,
    id: "py-669-lowest-common-ancestor",
    title: "DSA Trees V · LCA Binary Tree",
    solution: "class TreeNode:\n    def __init__(self, val=0, left=None, right=None):\n        self.val = val\n        self.left = left\n        self.right = right\n\ndef lowest_common_ancestor(root, p, q):\n    if not root or root is p or root is q:\n        return root\n    left = lowest_common_ancestor(root.left, p, q)\n    right = lowest_common_ancestor(root.right, p, q)\n    if left and right:\n        return root\n    return left or right\n\np=TreeNode(5); q=TreeNode(1); root=TreeNode(3, p, q); print(lowest_common_ancestor(root, p, q).val)\n",
    nextUrl: /\/learn\/py-670-serialize-tree/,
    cursorAfter: "670",
  },
  {
    micro: 670,
    id: "py-670-serialize-tree",
    title: "DSA Trees V · Serialize",
    solution: "class TreeNode:\n    def __init__(self, val=0, left=None, right=None):\n        self.val = val\n        self.left = left\n        self.right = right\n\ndef serialize(root):\n    out = []\n    def dfs(node):\n        if not node:\n            out.append('#')\n            return\n        out.append(str(node.val))\n        dfs(node.left)\n        dfs(node.right)\n    dfs(root)\n    return ','.join(out)\n\ndef deserialize(data):\n    it = iter(data.split(','))\n    def dfs():\n        val = next(it)\n        if val == '#':\n            return None\n        node = TreeNode(int(val))\n        node.left = dfs()\n        node.right = dfs()\n        return node\n    return dfs()\n\nroot=TreeNode(1, TreeNode(2), TreeNode(3, TreeNode(4), TreeNode(5))); print(serialize(deserialize(serialize(root))))\n",
    nextUrl: /\/learn\/py-671-build-tree-pre-in/,
    cursorAfter: "671",
  },
  {
    micro: 671,
    id: "py-671-build-tree-pre-in",
    title: "DSA Trees V · Build Pre/In",
    solution: "class TreeNode:\n    def __init__(self, val=0, left=None, right=None):\n        self.val = val\n        self.left = left\n        self.right = right\n\ndef build_tree(preorder, inorder):\n    idx = {v: i for i, v in enumerate(inorder)}\n    pre_i = [0]\n    def dfs(lo, hi):\n        if lo > hi:\n            return None\n        val = preorder[pre_i[0]]\n        pre_i[0] += 1\n        mid = idx[val]\n        node = TreeNode(val)\n        node.left = dfs(lo, mid - 1)\n        node.right = dfs(mid + 1, hi)\n        return node\n    return dfs(0, len(inorder) - 1)\n\nroot=build_tree([3,9,20,15,7],[9,3,15,20,7]); print([root.val, root.left.val, root.right.val])\n",
    nextUrl: /\/learn\/py-672-max-path-sum/,
    cursorAfter: "672",
  },
  {
    micro: 672,
    id: "py-672-max-path-sum",
    title: "DSA Trees V · Max Path Sum",
    solution: "class TreeNode:\n    def __init__(self, val=0, left=None, right=None):\n        self.val = val\n        self.left = left\n        self.right = right\n\ndef max_path_sum(root):\n    best = [root.val]\n    def dfs(node):\n        if not node:\n            return 0\n        left = max(0, dfs(node.left))\n        right = max(0, dfs(node.right))\n        best[0] = max(best[0], node.val + left + right)\n        return node.val + max(left, right)\n    dfs(root)\n    return best[0]\n\nprint(max_path_sum(TreeNode(-10, TreeNode(9), TreeNode(20, TreeNode(15), TreeNode(7)))))\n",
    nextUrl: /\/learn\/py-673-search-bst/,
    cursorAfter: "673",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:667|668|669|670|671|672)-/);
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

test.describe("micro-steps 667–672 · trees v", () => {
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
      if (nextMicro <= 672) {
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

      if (step.micro < 672) {
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
