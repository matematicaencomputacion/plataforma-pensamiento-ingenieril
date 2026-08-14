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
    micro: 239,
    id: "py-239-tree-diameter",
    title: "DSA Tree Diameter",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def diameter_of_binary_tree(root):
    best = [0]
    def depth(node):
        if node is None:
            return 0
        left = depth(node.left)
        right = depth(node.right)
        best[0] = max(best[0], left + right)
        return 1 + max(left, right)
    depth(root)
    return best[0]

root = TreeNode(1)
root.left = TreeNode(2)
root.right = TreeNode(3)
root.left.left = TreeNode(4)
root.left.right = TreeNode(5)
print(diameter_of_binary_tree(root))
`,
    nextUrl: /\/learn\/py-240-lca/,
    cursorAfter: "240",
  },
  {
    micro: 240,
    id: "py-240-lca",
    title: "DSA Lowest Common Ancestor",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def lowest_common_ancestor(root, p, q):
    if root is None or root is p or root is q:
        return root
    left = lowest_common_ancestor(root.left, p, q)
    right = lowest_common_ancestor(root.right, p, q)
    if left and right:
        return root
    return left or right

root = TreeNode(3)
root.left = TreeNode(5)
root.right = TreeNode(1)
print(lowest_common_ancestor(root, root.left, root.right).data)
`,
    nextUrl: /\/learn\/py-241-path-sum/,
    cursorAfter: "241",
  },
  {
    micro: 241,
    id: "py-241-path-sum",
    title: "DSA Path Sum",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def has_path_sum(root, target):
    if root is None:
        return False
    if root.left is None and root.right is None:
        return root.data == target
    return (
        has_path_sum(root.left, target - root.data)
        or has_path_sum(root.right, target - root.data)
    )

root = TreeNode(5)
root.left = TreeNode(4)
root.right = TreeNode(8)
root.left.left = TreeNode(11)
root.left.left.left = TreeNode(7)
root.left.left.right = TreeNode(2)
root.right.left = TreeNode(13)
root.right.right = TreeNode(4)
root.right.right.right = TreeNode(1)
print(has_path_sum(root, 22))
`,
    nextUrl: /\/learn\/py-242-right-side/,
    cursorAfter: "242",
  },
  {
    micro: 242,
    id: "py-242-right-side",
    title: "DSA Right Side View",
    solution: `from collections import deque

class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def right_side_view(root):
    if root is None:
        return []
    out = []
    queue = deque([root])
    while queue:
        last = None
        for _ in range(len(queue)):
            node = queue.popleft()
            last = node.data
            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)
        out.append(last)
    return out

root = TreeNode(1)
root.left = TreeNode(2)
root.right = TreeNode(3)
root.left.right = TreeNode(5)
root.right.right = TreeNode(4)
print(right_side_view(root))
`,
    nextUrl: /\/learn\/py-243-flatten-tree/,
    cursorAfter: "243",
  },
  {
    micro: 243,
    id: "py-243-flatten-tree",
    title: "DSA Flatten Tree",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def flatten(root):
    while root:
        if root.left:
            pred = root.left
            while pred.right:
                pred = pred.right
            pred.right = root.right
            root.right = root.left
            root.left = None
        root = root.right

root = TreeNode(1)
root.left = TreeNode(2)
root.right = TreeNode(5)
root.left.left = TreeNode(3)
root.left.right = TreeNode(4)
root.right.right = TreeNode(6)
flatten(root)
values = []
cur = root
while cur:
    values.append(cur.data)
    cur = cur.right
print(values)
`,
    nextUrl: /\/learn\/py-244-validate-bst/,
    cursorAfter: "244",
  },
  {
    micro: 244,
    id: "py-244-validate-bst",
    title: "DSA Validate BST",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def is_valid_bst(root):
    def valid(node, lo, hi):
        if node is None:
            return True
        if not (lo < node.data < hi):
            return False
        return valid(node.left, lo, node.data) and valid(node.right, node.data, hi)
    return valid(root, float('-inf'), float('inf'))

root = TreeNode(2)
root.left = TreeNode(1)
root.right = TreeNode(3)
print(is_valid_bst(root))
`,
    nextUrl: /\/learn\/py-245-remove-nth/,
    cursorAfter: "245",
  },
];

test("declares the contiguous learn-route family", () => {
  for (const step of FAMILY) {
    expect(step.id).toMatch(/^py-24[0-4]-|^py-239-/);
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

test.describe("micro-steps 239–244 · trees avanzados", () => {
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
      if (nextMicro <= 594) {
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
