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
    micro: 673,
    id: "py-673-search-bst",
    title: "DSA BST VI · Search",
    solution: "class TreeNode:\n    def __init__(self, val=0, left=None, right=None):\n        self.val = val\n        self.left = left\n        self.right = right\n\ndef search_bst(root, val):\n    if not root or root.val == val:\n        return root\n    if val < root.val:\n        return search_bst(root.left, val)\n    return search_bst(root.right, val)\n\nprint(search_bst(TreeNode(4, TreeNode(2, TreeNode(1), TreeNode(3)), TreeNode(7)), 2).val)\n",
    nextUrl: /\/learn\/py-674-insert-bst/,
    cursorAfter: "674",
  },
  {
    micro: 674,
    id: "py-674-insert-bst",
    title: "DSA BST VI · Insert",
    solution: "class TreeNode:\n    def __init__(self, val=0, left=None, right=None):\n        self.val = val\n        self.left = left\n        self.right = right\n\ndef insert_into_bst(root, val):\n    if not root:\n        return TreeNode(val)\n    if val < root.val:\n        root.left = insert_into_bst(root.left, val)\n    else:\n        root.right = insert_into_bst(root.right, val)\n    return root\n\ndef inorder(root):\n    return inorder(root.left) + [root.val] + inorder(root.right) if root else []\n\nroot = TreeNode(4, TreeNode(2, TreeNode(1), TreeNode(3)), TreeNode(7))\nprint(inorder(insert_into_bst(root, 5)))\n",
    nextUrl: /\/learn\/py-675-delete-bst/,
    cursorAfter: "675",
  },
  {
    micro: 675,
    id: "py-675-delete-bst",
    title: "DSA BST VI · Delete",
    solution: "class TreeNode:\n    def __init__(self, val=0, left=None, right=None):\n        self.val = val\n        self.left = left\n        self.right = right\n\ndef delete_node(root, key):\n    if not root:\n        return None\n    if key < root.val:\n        root.left = delete_node(root.left, key)\n    elif key > root.val:\n        root.right = delete_node(root.right, key)\n    else:\n        if not root.left:\n            return root.right\n        if not root.right:\n            return root.left\n        succ = root.right\n        while succ.left:\n            succ = succ.left\n        root.val = succ.val\n        root.right = delete_node(root.right, succ.val)\n    return root\n\ndef inorder(root):\n    return inorder(root.left) + [root.val] + inorder(root.right) if root else []\n\nroot = TreeNode(5, TreeNode(3, TreeNode(2), TreeNode(4)), TreeNode(6, None, TreeNode(7)))\nprint(inorder(delete_node(root, 3)))\n",
    nextUrl: /\/learn\/py-676-sorted-array-bst/,
    cursorAfter: "676",
  },
  {
    micro: 676,
    id: "py-676-sorted-array-bst",
    title: "DSA BST VI · Sorted Array",
    solution: "class TreeNode:\n    def __init__(self, val=0, left=None, right=None):\n        self.val = val\n        self.left = left\n        self.right = right\n\ndef sorted_array_to_bst(nums):\n    def build(lo, hi):\n        if lo > hi:\n            return None\n        mid = (lo + hi) // 2\n        return TreeNode(nums[mid], build(lo, mid - 1), build(mid + 1, hi))\n    return build(0, len(nums) - 1)\n\nroot = sorted_array_to_bst([-10, -3, 0, 5, 9])\nprint([root.val, root.left.val, root.right.val])\n",
    nextUrl: /\/learn\/py-677-lca-bst/,
    cursorAfter: "677",
  },
  {
    micro: 677,
    id: "py-677-lca-bst",
    title: "DSA BST VI · LCA BST",
    solution: "class TreeNode:\n    def __init__(self, val=0, left=None, right=None):\n        self.val = val\n        self.left = left\n        self.right = right\n\ndef lca_bst(root, p, q):\n    while root:\n        if p.val < root.val and q.val < root.val:\n            root = root.left\n        elif p.val > root.val and q.val > root.val:\n            root = root.right\n        else:\n            return root\n    return None\n\np = TreeNode(2, TreeNode(0), TreeNode(4, TreeNode(3), TreeNode(5)))\nq = TreeNode(8, TreeNode(7), TreeNode(9))\nroot = TreeNode(6, p, q)\nprint(lca_bst(root, p, q).val)\n",
    nextUrl: /\/learn\/py-678-bst-to-gst/,
    cursorAfter: "678",
  },
  {
    micro: 678,
    id: "py-678-bst-to-gst",
    title: "DSA BST VI · Greater Sum",
    solution: "class TreeNode:\n    def __init__(self, val=0, left=None, right=None):\n        self.val = val\n        self.left = left\n        self.right = right\n\ndef bst_to_gst(root):\n    acc = [0]\n    def dfs(node):\n        if not node:\n            return\n        dfs(node.right)\n        acc[0] += node.val\n        node.val = acc[0]\n        dfs(node.left)\n    dfs(root)\n    return root\n\ndef inorder(root):\n    return inorder(root.left) + [root.val] + inorder(root.right) if root else []\n\nroot = TreeNode(4, TreeNode(1, TreeNode(0), TreeNode(2, None, TreeNode(3))), TreeNode(6, TreeNode(5), TreeNode(7, None, TreeNode(8))))\nprint(inorder(bst_to_gst(root)))\n",
    nextUrl: /\/learn\/py-679-reverse-between/,
    cursorAfter: "679",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-(?:673|674|675|676|677|678)-/);
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

test.describe("micro-steps 673–678 · bst vi", () => {
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

      if (step.micro < 678) {
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
