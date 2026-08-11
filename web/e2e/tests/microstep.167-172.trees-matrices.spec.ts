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
    micro: 167,
    id: "py-167-invert-tree",
    title: "DSA Invert Binary Tree",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def invert_tree(root):
    if root is None:
        return None
    root.left, root.right = invert_tree(root.right), invert_tree(root.left)
    return root

def level_order(root):
    if root is None:
        return []
    result = []
    queue = [root]
    while queue:
        node = queue.pop(0)
        result.append(node.data)
        if node.left:
            queue.append(node.left)
        if node.right:
            queue.append(node.right)
    return result

root = TreeNode(4)
root.left = TreeNode(2)
root.right = TreeNode(7)
root.left.left = TreeNode(1)
root.left.right = TreeNode(3)
root.right.left = TreeNode(6)
root.right.right = TreeNode(9)
invert_tree(root)
print(level_order(root))
`,
    nextUrl: /\/learn\/py-168-same-tree/,
    cursorAfter: "168",
  },
  {
    micro: 168,
    id: "py-168-same-tree",
    title: "DSA Same Tree",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def is_same_tree(p, q):
    if p is None and q is None:
        return True
    if p is None or q is None or p.data != q.data:
        return False
    return is_same_tree(p.left, q.left) and is_same_tree(p.right, q.right)

p = TreeNode(1)
p.left = TreeNode(2)
p.right = TreeNode(3)
q = TreeNode(1)
q.left = TreeNode(2)
q.right = TreeNode(3)
print(is_same_tree(p, q))
`,
    nextUrl: /\/learn\/py-169-max-depth/,
    cursorAfter: "169",
  },
  {
    micro: 169,
    id: "py-169-max-depth",
    title: "DSA Maximum Depth",
    solution: `class TreeNode:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def max_depth(root):
    if root is None:
        return 0
    return 1 + max(max_depth(root.left), max_depth(root.right))

root = TreeNode(3)
root.left = TreeNode(9)
root.right = TreeNode(20)
root.right.left = TreeNode(15)
root.right.right = TreeNode(7)
print(max_depth(root))
`,
    nextUrl: /\/learn\/py-170-spiral-matrix/,
    cursorAfter: "170",
  },
  {
    micro: 170,
    id: "py-170-spiral-matrix",
    title: "DSA Spiral Matrix",
    solution: `def spiral_order(matrix):
    if not matrix:
        return []
    result = []
    top, bottom = 0, len(matrix) - 1
    left, right = 0, len(matrix[0]) - 1
    while top <= bottom and left <= right:
        for j in range(left, right + 1):
            result.append(matrix[top][j])
        top += 1
        for i in range(top, bottom + 1):
            result.append(matrix[i][right])
        right -= 1
        if top <= bottom:
            for j in range(right, left - 1, -1):
                result.append(matrix[bottom][j])
            bottom -= 1
        if left <= right:
            for i in range(bottom, top - 1, -1):
                result.append(matrix[i][left])
            left += 1
    return result
print(spiral_order([[1, 2, 3], [4, 5, 6], [7, 8, 9]]))
`,
    nextUrl: /\/learn\/py-171-set-zeroes/,
    cursorAfter: "171",
  },
  {
    micro: 171,
    id: "py-171-set-zeroes",
    title: "DSA Set Matrix Zeroes",
    solution: `def set_zeroes(matrix):
    rows = set()
    cols = set()
    for i in range(len(matrix)):
        for j in range(len(matrix[0])):
            if matrix[i][j] == 0:
                rows.add(i)
                cols.add(j)
    for i in range(len(matrix)):
        for j in range(len(matrix[0])):
            if i in rows or j in cols:
                matrix[i][j] = 0
    return matrix
print(set_zeroes([[1, 1, 1], [1, 0, 1], [1, 1, 1]]))
`,
    nextUrl: /\/learn\/py-172-subsets/,
    cursorAfter: "172",
  },
  {
    micro: 172,
    id: "py-172-subsets",
    title: "DSA Subsets",
    solution: `def subsets(nums):
    result = [[]]
    for n in nums:
        result += [subset + [n] for subset in result]
    return result
print(sorted(subsets([1, 2])))
`,
    nextUrl: /\/learn\/py-173-jump-game/,
    cursorAfter: "173",
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

test.describe("micro-steps 167–172 · trees / matrices", () => {
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
